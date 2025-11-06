use serde::{Deserialize, Serialize};

/// Tracks completion status for individual lessons
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LessonCompletion {
    /// Lesson ID
    pub lesson_id: usize,
    /// Whether the lesson has been completed
    pub completed: bool,
    /// Number of attempts (optional, for future analytics)
    pub attempts: u32,
}

impl LessonCompletion {
    /// Create a new lesson completion record
    pub fn new(lesson_id: usize) -> Self {
        Self {
            lesson_id,
            completed: false,
            attempts: 0,
        }
    }

    /// Mark the lesson as completed
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.attempts += 1;
    }

    /// Record an attempt (whether successful or not)
    pub fn record_attempt(&mut self) {
        self.attempts += 1;
    }
}

/// Manages completion state for all lessons
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CompletionState {
    /// Map of lesson ID to completion status
    pub completions: Vec<LessonCompletion>,
}

impl CompletionState {
    /// Create a new empty completion state
    pub fn new() -> Self {
        Self {
            completions: Vec::new(),
        }
    }

    /// Get completion record for a lesson, creating one if it doesn't exist
    fn get_or_create_completion(&mut self, lesson_id: usize) -> &mut LessonCompletion {
        if let Some(pos) = self.completions.iter().position(|c| c.lesson_id == lesson_id) {
            &mut self.completions[pos]
        } else {
            self.completions.push(LessonCompletion::new(lesson_id));
            self.completions
                .last_mut()
                .expect("Vec should have at least one element after push")
        }
    }

    /// Mark a lesson as completed
    pub fn mark_completed(&mut self, lesson_id: usize) {
        self.get_or_create_completion(lesson_id).mark_completed();
    }

    /// Record an attempt for a lesson
    pub fn record_attempt(&mut self, lesson_id: usize) {
        self.get_or_create_completion(lesson_id).record_attempt();
    }

    /// Check if a lesson is completed
    pub fn is_completed(&self, lesson_id: usize) -> bool {
        self.completions
            .iter()
            .find(|c| c.lesson_id == lesson_id)
            .map(|c| c.completed)
            .unwrap_or(false)
    }

    /// Get the number of attempts for a lesson
    pub fn get_attempts(&self, lesson_id: usize) -> u32 {
        self.completions
            .iter()
            .find(|c| c.lesson_id == lesson_id)
            .map(|c| c.attempts)
            .unwrap_or(0)
    }

    /// Get total number of completed lessons
    pub fn total_completed(&self) -> usize {
        self.completions.iter().filter(|c| c.completed).count()
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesson_completion_new() {
        let completion = LessonCompletion::new(1);
        assert_eq!(completion.lesson_id, 1);
        assert!(!completion.completed);
        assert_eq!(completion.attempts, 0);
    }

    #[test]
    fn test_mark_completed() {
        let mut completion = LessonCompletion::new(1);
        completion.mark_completed();
        assert!(completion.completed);
        assert_eq!(completion.attempts, 1);
    }

    #[test]
    fn test_record_attempt() {
        let mut completion = LessonCompletion::new(1);
        completion.record_attempt();
        assert!(!completion.completed);
        assert_eq!(completion.attempts, 1);
    }

    #[test]
    fn test_completion_state_new() {
        let state = CompletionState::new();
        assert!(state.completions.is_empty());
    }

    #[test]
    fn test_completion_state_mark_completed() {
        let mut state = CompletionState::new();
        state.mark_completed(1);
        assert!(state.is_completed(1));
        assert_eq!(state.get_attempts(1), 1);
    }

    #[test]
    fn test_completion_state_record_attempt() {
        let mut state = CompletionState::new();
        state.record_attempt(1);
        assert!(!state.is_completed(1));
        assert_eq!(state.get_attempts(1), 1);

        state.record_attempt(1);
        assert_eq!(state.get_attempts(1), 2);
    }

    #[test]
    fn test_completion_state_multiple_lessons() {
        let mut state = CompletionState::new();
        state.mark_completed(1);
        state.record_attempt(2);
        state.mark_completed(3);

        assert!(state.is_completed(1));
        assert!(!state.is_completed(2));
        assert!(state.is_completed(3));
        assert_eq!(state.total_completed(), 2);
    }

    #[test]
    fn test_json_serialization() {
        let mut state = CompletionState::new();
        state.mark_completed(1);
        state.record_attempt(2);

        let json = state.to_json().unwrap();
        assert!(json.contains("lesson_id"));
        assert!(json.contains("completed"));
        assert!(json.contains("attempts"));

        let deserialized = CompletionState::from_json(&json).unwrap();
        assert_eq!(state, deserialized);
        assert!(deserialized.is_completed(1));
        assert!(!deserialized.is_completed(2));
    }

    #[test]
    fn test_json_roundtrip() {
        let mut state = CompletionState::new();
        state.mark_completed(1);
        state.mark_completed(2);
        state.record_attempt(3);

        let json = state.to_json().unwrap();
        let restored = CompletionState::from_json(&json).unwrap();

        assert_eq!(state.completions.len(), restored.completions.len());
        assert_eq!(state.total_completed(), restored.total_completed());
        assert!(restored.is_completed(1));
        assert!(restored.is_completed(2));
        assert!(!restored.is_completed(3));
    }
}
