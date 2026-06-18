//! Progress tracking data structures for lessons
//!
//! This module provides core progress tracking functionality that can be used
//! across different platforms (desktop TUI, ESP32, web, etc.)
//!
//! Features:
//! - `no_std` compatible (works on embedded systems)
//! - Optional `serde` support for serialization
//! - Timestamp-based tracking (Unix epoch seconds as u64)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Progress tracking for a single lesson
///
/// Tracks the number of attempts, correct/incorrect answers, and timestamps
/// for when the lesson was first and last attempted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LessonProgress {
    /// The ID of the lesson this progress belongs to
    pub lesson_id: usize,

    /// Total number of times this lesson has been attempted
    pub completed_count: usize,

    /// Number of correct answers
    pub correct_count: usize,

    /// Number of incorrect answers
    pub incorrect_count: usize,

    /// Unix timestamp (seconds since epoch) of the last attempt
    pub last_attempted_timestamp: u64,

    /// Unix timestamp (seconds since epoch) of the first attempt
    pub first_attempted_timestamp: u64,
}

impl LessonProgress {
    /// Create a new LessonProgress with the current timestamp
    ///
    /// # Arguments
    /// * `lesson_id` - The ID of the lesson
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn new(lesson_id: usize, current_timestamp: u64) -> Self {
        Self {
            lesson_id,
            completed_count: 0,
            correct_count: 0,
            incorrect_count: 0,
            last_attempted_timestamp: current_timestamp,
            first_attempted_timestamp: current_timestamp,
        }
    }

    /// Record a new answer attempt
    ///
    /// # Arguments
    /// * `is_correct` - Whether the answer was correct
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn record_answer(&mut self, is_correct: bool, current_timestamp: u64) {
        self.completed_count += 1;
        if is_correct {
            self.correct_count += 1;
        } else {
            self.incorrect_count += 1;
        }
        self.last_attempted_timestamp = current_timestamp;
    }

    /// Calculate the accuracy percentage
    ///
    /// Returns a value between 0.0 and 100.0
    pub fn accuracy(&self) -> f64 {
        if self.completed_count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.completed_count as f64) * 100.0
        }
    }

    /// Get the number of days since the lesson was first attempted
    ///
    /// # Arguments
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn days_since_first_attempt(&self, current_timestamp: u64) -> u64 {
        if current_timestamp >= self.first_attempted_timestamp {
            (current_timestamp - self.first_attempted_timestamp) / 86400 // seconds in a day
        } else {
            0
        }
    }

    /// Get the number of days since the lesson was last attempted
    ///
    /// # Arguments
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn days_since_last_attempt(&self, current_timestamp: u64) -> u64 {
        if current_timestamp >= self.last_attempted_timestamp {
            (current_timestamp - self.last_attempted_timestamp) / 86400
        } else {
            0
        }
    }
}

/// Overall progress statistics across all lessons
///
/// Provides aggregate statistics for tracking overall learning progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProgressSummary {
    /// Total number of attempts across all lessons
    pub total_attempts: usize,

    /// Total number of correct answers across all lessons
    pub total_correct: usize,

    /// Unix timestamp of the last session
    pub last_session_timestamp: u64,

    /// Unix timestamp of the first session
    pub first_session_timestamp: u64,
}

impl ProgressSummary {
    /// Create a new ProgressSummary with the current timestamp
    ///
    /// # Arguments
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn new(current_timestamp: u64) -> Self {
        Self {
            total_attempts: 0,
            total_correct: 0,
            last_session_timestamp: current_timestamp,
            first_session_timestamp: current_timestamp,
        }
    }

    /// Record a new answer attempt
    ///
    /// # Arguments
    /// * `is_correct` - Whether the answer was correct
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn record_answer(&mut self, is_correct: bool, current_timestamp: u64) {
        self.total_attempts += 1;
        if is_correct {
            self.total_correct += 1;
        }
        self.last_session_timestamp = current_timestamp;
    }

    /// Calculate the overall accuracy percentage
    ///
    /// Returns a value between 0.0 and 100.0
    pub fn overall_accuracy(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.total_correct as f64 / self.total_attempts as f64) * 100.0
        }
    }

    /// Get the number of days of learning
    ///
    /// # Arguments
    /// * `current_timestamp` - Current Unix timestamp in seconds
    pub fn days_of_learning(&self, current_timestamp: u64) -> u64 {
        if current_timestamp >= self.first_session_timestamp {
            (current_timestamp - self.first_session_timestamp) / 86400
        } else {
            0
        }
    }

    /// Get the total number of incorrect answers
    pub fn total_incorrect(&self) -> usize {
        if self.total_attempts >= self.total_correct {
            self.total_attempts - self.total_correct
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesson_progress_new() {
        let progress = LessonProgress::new(1, 1000);
        assert_eq!(progress.lesson_id, 1);
        assert_eq!(progress.completed_count, 0);
        assert_eq!(progress.correct_count, 0);
        assert_eq!(progress.incorrect_count, 0);
        assert_eq!(progress.first_attempted_timestamp, 1000);
        assert_eq!(progress.last_attempted_timestamp, 1000);
    }

    #[test]
    fn test_record_answer() {
        let mut progress = LessonProgress::new(1, 1000);

        progress.record_answer(true, 1100);
        assert_eq!(progress.completed_count, 1);
        assert_eq!(progress.correct_count, 1);
        assert_eq!(progress.incorrect_count, 0);
        assert_eq!(progress.last_attempted_timestamp, 1100);

        progress.record_answer(false, 1200);
        assert_eq!(progress.completed_count, 2);
        assert_eq!(progress.correct_count, 1);
        assert_eq!(progress.incorrect_count, 1);
        assert_eq!(progress.last_attempted_timestamp, 1200);
    }

    #[test]
    fn test_accuracy() {
        let mut progress = LessonProgress::new(1, 1000);
        assert_eq!(progress.accuracy(), 0.0);

        progress.record_answer(true, 1100);
        assert_eq!(progress.accuracy(), 100.0);

        progress.record_answer(false, 1200);
        assert_eq!(progress.accuracy(), 50.0);

        progress.record_answer(true, 1300);
        assert!((progress.accuracy() - 66.666).abs() < 0.01);
    }

    #[test]
    fn test_progress_summary() {
        let mut summary = ProgressSummary::new(1000);
        assert_eq!(summary.total_attempts, 0);
        assert_eq!(summary.overall_accuracy(), 0.0);

        summary.record_answer(true, 1100);
        assert_eq!(summary.total_attempts, 1);
        assert_eq!(summary.total_correct, 1);
        assert_eq!(summary.overall_accuracy(), 100.0);

        summary.record_answer(false, 1200);
        assert_eq!(summary.total_attempts, 2);
        assert_eq!(summary.total_correct, 1);
        assert_eq!(summary.overall_accuracy(), 50.0);
    }

    #[test]
    fn test_days_calculations() {
        let progress = LessonProgress::new(1, 0);
        let one_day = 86400;

        assert_eq!(progress.days_since_first_attempt(one_day), 1);
        assert_eq!(progress.days_since_first_attempt(one_day * 7), 7);

        let summary = ProgressSummary::new(0);
        assert_eq!(summary.days_of_learning(one_day * 30), 30);
    }
}
