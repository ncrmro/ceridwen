use ceridwen_core::CompletionState;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Default filename for storing completion state
const COMPLETION_FILE: &str = "ceridwen_completion.json";

/// Get the path to the completion state file
fn get_completion_path() -> PathBuf {
    // Try to use user's data directory, fall back to current directory
    if let Some(data_dir) = dirs::data_local_dir() {
        let app_dir = data_dir.join("ceridwen");
        // Create directory if it doesn't exist
        let _ = fs::create_dir_all(&app_dir);
        app_dir.join(COMPLETION_FILE)
    } else {
        PathBuf::from(COMPLETION_FILE)
    }
}

/// Load completion state from JSON file
pub fn load_completion_state() -> io::Result<CompletionState> {
    let path = get_completion_path();
    
    if !path.exists() {
        // Return empty state if file doesn't exist
        return Ok(CompletionState::new());
    }

    let json = fs::read_to_string(&path)?;
    CompletionState::from_json(&json).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse completion state: {}", e),
        )
    })
}

/// Save completion state to JSON file
pub fn save_completion_state(state: &CompletionState) -> io::Result<()> {
    let path = get_completion_path();
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = state.to_json().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize completion state: {}", e),
        )
    })?;

    fs::write(&path, json)?;
    Ok(())
}

/// Get the path where completion state is stored (for debugging/info)
#[allow(dead_code)]
pub fn get_storage_path() -> PathBuf {
    get_completion_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load() {
        let test_file = PathBuf::from("/tmp/test_ceridwen_completion.json");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_file);
        
        let mut state = CompletionState::new();
        state.mark_completed(1);
        state.mark_completed(2);
        state.record_attempt(3);

        // Save to a test location
        let json = state.to_json().unwrap();
        fs::write(&test_file, json).unwrap();

        // Load it back
        let loaded_json = fs::read_to_string(&test_file).unwrap();
        let loaded_state = CompletionState::from_json(&loaded_json).unwrap();

        assert_eq!(state.total_completed(), loaded_state.total_completed());
        assert!(loaded_state.is_completed(1));
        assert!(loaded_state.is_completed(2));
        assert!(!loaded_state.is_completed(3));

        // Clean up
        let _ = fs::remove_file(&test_file);
    }
}
