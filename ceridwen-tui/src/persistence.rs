use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Progress tracking for a single lesson
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonProgress {
    pub lesson_id: usize,
    pub completed_count: usize,
    pub correct_count: usize,
    pub incorrect_count: usize,
    pub last_attempted: DateTime<Utc>,
    pub first_attempted: DateTime<Utc>,
}

impl LessonProgress {
    pub fn new(lesson_id: usize) -> Self {
        let now = Utc::now();
        Self {
            lesson_id,
            completed_count: 0,
            correct_count: 0,
            incorrect_count: 0,
            last_attempted: now,
            first_attempted: now,
        }
    }

    pub fn record_answer(&mut self, is_correct: bool) {
        self.completed_count += 1;
        if is_correct {
            self.correct_count += 1;
        } else {
            self.incorrect_count += 1;
        }
        self.last_attempted = Utc::now();
    }

    pub fn accuracy(&self) -> f64 {
        if self.completed_count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.completed_count as f64) * 100.0
        }
    }
}

/// Overall progress tracking for all lessons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressData {
    pub lessons: HashMap<usize, LessonProgress>,
    pub total_attempts: usize,
    pub total_correct: usize,
    pub last_session: DateTime<Utc>,
    pub first_session: DateTime<Utc>,
}

impl Default for ProgressData {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            lessons: HashMap::new(),
            total_attempts: 0,
            total_correct: 0,
            last_session: now,
            first_session: now,
        }
    }
}

/// Manages persistence of progress data
pub struct ProgressManager {
    data: ProgressData,
    file_path: PathBuf,
}

impl ProgressManager {
    /// Create a new ProgressManager with the default file path
    pub fn new() -> io::Result<Self> {
        let file_path = Self::get_default_path()?;
        let data = Self::load_from_path(&file_path).unwrap_or_default();

        Ok(Self { data, file_path })
    }

    /// Get the default storage path for progress data
    fn get_default_path() -> io::Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find data directory"))?;

        let app_dir = data_dir.join("ceridwen");

        // Create directory if it doesn't exist
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir.join("progress.json"))
    }

    /// Load progress data from a file
    fn load_from_path(path: &PathBuf) -> io::Result<ProgressData> {
        if !path.exists() {
            return Ok(ProgressData::default());
        }

        let contents = fs::read_to_string(path)?;
        serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Save progress data to file
    pub fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// Record an answer for a lesson
    pub fn record_answer(&mut self, lesson_id: usize, is_correct: bool) {
        let lesson_progress = self.data.lessons
            .entry(lesson_id)
            .or_insert_with(|| LessonProgress::new(lesson_id));

        lesson_progress.record_answer(is_correct);

        self.data.total_attempts += 1;
        if is_correct {
            self.data.total_correct += 1;
        }
        self.data.last_session = Utc::now();
    }

    /// Get progress for a specific lesson
    pub fn get_lesson_progress(&self, lesson_id: usize) -> Option<&LessonProgress> {
        self.data.lessons.get(&lesson_id)
    }

    /// Get overall accuracy percentage
    pub fn overall_accuracy(&self) -> f64 {
        if self.data.total_attempts == 0 {
            0.0
        } else {
            (self.data.total_correct as f64 / self.data.total_attempts as f64) * 100.0
        }
    }

    /// Get total number of attempts across all lessons
    pub fn total_attempts(&self) -> usize {
        self.data.total_attempts
    }

    /// Get total number of correct answers across all lessons
    pub fn total_correct(&self) -> usize {
        self.data.total_correct
    }

    /// Get the number of unique lessons attempted
    pub fn lessons_attempted(&self) -> usize {
        self.data.lessons.len()
    }

    /// Get all lesson progress data
    pub fn all_progress(&self) -> &HashMap<usize, LessonProgress> {
        &self.data.lessons
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback if we can't create proper path
            Self {
                data: ProgressData::default(),
                file_path: PathBuf::from("progress.json"),
            }
        })
    }
}
