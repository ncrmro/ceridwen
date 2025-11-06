/// Types of lessons available in the educational system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LessonType {
    /// Subitizing exercises - recognizing quantities on dice (1-6)
    Subitizing,
    /// Addition exercises (e.g., 2 + 3 = ?)
    Addition,
    /// Subtraction exercises (e.g., 5 - 2 = ?)
    Subtraction,
    /// Multiplication exercises (e.g., 3 × 4 = ?)
    Multiplication,
}

/// Represents a single lesson with question and answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lesson {
    /// Unique identifier for the lesson
    pub id: usize,
    /// Type of lesson
    pub lesson_type: LessonType,
    /// First operand or value
    pub value1: u8,
    /// Second operand (0 for subitizing exercises)
    pub value2: u8,
    /// Correct answer
    pub answer: u8,
    /// Human-readable question text
    pub question: String,
    /// For subitizing: dice faces to display (e.g., [1, 2] shows dice with 1 and 2 dots)
    pub dice_options: Vec<u8>,
    /// For subitizing: which target number to select
    pub target_number: Option<u8>,
}

impl Lesson {
    /// Get a larger ASCII art dice pattern for a subitizing lesson (3x3 grid)
    pub fn get_dice_art(value: u8) -> Vec<String> {
        match value {
            1 => vec![
                "┌─────┐".to_string(),
                "│     │".to_string(),
                "│  ●  │".to_string(),
                "│     │".to_string(),
                "└─────┘".to_string(),
            ],
            2 => vec![
                "┌─────┐".to_string(),
                "│ ●   │".to_string(),
                "│     │".to_string(),
                "│   ● │".to_string(),
                "└─────┘".to_string(),
            ],
            3 => vec![
                "┌─────┐".to_string(),
                "│ ●   │".to_string(),
                "│  ●  │".to_string(),
                "│   ● │".to_string(),
                "└─────┘".to_string(),
            ],
            4 => vec![
                "┌─────┐".to_string(),
                "│ ● ● │".to_string(),
                "│     │".to_string(),
                "│ ● ● │".to_string(),
                "└─────┘".to_string(),
            ],
            5 => vec![
                "┌─────┐".to_string(),
                "│ ● ● │".to_string(),
                "│  ●  │".to_string(),
                "│ ● ● │".to_string(),
                "└─────┘".to_string(),
            ],
            6 => vec![
                "┌─────┐".to_string(),
                "│ ● ● │".to_string(),
                "│ ● ● │".to_string(),
                "│ ● ● │".to_string(),
                "└─────┘".to_string(),
            ],
            _ => vec![
                "┌─────┐".to_string(),
                "│  ?  │".to_string(),
                "│  ?  │".to_string(),
                "│  ?  │".to_string(),
                "└─────┘".to_string(),
            ],
        }
    }

    /// Get the dice pattern for a subitizing lesson (single Unicode character)
    pub fn get_dice_pattern(&self) -> String {
        if self.lesson_type != LessonType::Subitizing {
            return String::new();
        }

        match self.value1 {
            1 => "⚀".to_string(),
            2 => "⚁".to_string(),
            3 => "⚂".to_string(),
            4 => "⚃".to_string(),
            5 => "⚄".to_string(),
            6 => "⚅".to_string(),
            _ => "?".to_string(),
        }
    }

    /// Create a new subitizing lesson with multiple dice options
    pub fn new_subitizing(
        id: usize,
        dice_options: Vec<u8>,
        target_number: u8,
        question: String,
    ) -> Self {
        Self {
            id,
            lesson_type: LessonType::Subitizing,
            value1: target_number,
            value2: 0,
            answer: target_number,
            question,
            dice_options,
            target_number: Some(target_number),
        }
    }

    /// Create a new addition lesson
    pub fn new_addition(id: usize, a: u8, b: u8) -> Self {
        Self {
            id,
            lesson_type: LessonType::Addition,
            value1: a,
            value2: b,
            answer: a.saturating_add(b),
            question: format!("{} + {} = ?", a, b),
            dice_options: vec![],
            target_number: None,
        }
    }

    /// Create a new subtraction lesson
    pub fn new_subtraction(id: usize, a: u8, b: u8) -> Self {
        Self {
            id,
            lesson_type: LessonType::Subtraction,
            value1: a,
            value2: b,
            answer: a.saturating_sub(b),
            question: format!("{} - {} = ?", a, b),
            dice_options: vec![],
            target_number: None,
        }
    }

    /// Create a new multiplication lesson
    pub fn new_multiplication(id: usize, a: u8, b: u8) -> Self {
        Self {
            id,
            lesson_type: LessonType::Multiplication,
            value1: a,
            value2: b,
            answer: a.saturating_mul(b),
            question: format!("{} × {} = ?", a, b),
            dice_options: vec![],
            target_number: None,
        }
    }

    /// Check if a given answer is correct
    pub fn check_answer(&self, user_answer: u8) -> bool {
        self.answer == user_answer
    }
}

/// Query parameters for filtering lessons
#[derive(Debug, Clone, Default)]
pub struct LessonQuery {
    /// Filter by lesson type
    pub lesson_type: Option<LessonType>,
    /// Filter by minimum difficulty (based on operand values)
    pub min_difficulty: Option<u8>,
    /// Filter by maximum difficulty
    pub max_difficulty: Option<u8>,
    /// Limit number of results
    pub limit: Option<usize>,
}

/// Manages a collection of lessons in memory
#[derive(Debug, Clone)]
pub struct LessonManager {
    lessons: Vec<Lesson>,
    next_id: usize,
}

impl LessonManager {
    /// Create a new empty lesson manager
    pub fn new() -> Self {
        Self {
            lessons: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a lesson manager with default lessons
    pub fn with_defaults() -> Self {
        let mut manager = Self::new();

        // Lesson 1 - Introduce 1 and 2 (two parts)
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subitizing(
            id,
            vec![1, 2],
            1,
            "Select the die showing 1".to_string(),
        ));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subitizing(
            id,
            vec![1, 2],
            2,
            "Select the die showing 2".to_string(),
        ));

        // Lesson 2 - Introduce 3
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subitizing(
            id,
            vec![1, 2, 3],
            3,
            "Select the die showing 3".to_string(),
        ));

        // Lesson 3 - Introduce 1-6
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subitizing(
            id,
            vec![1, 2, 3, 4, 5, 6],
            5,
            "Select the die showing 5".to_string(),
        ));

        // Add addition lessons
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_addition(id, 1, 1));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_addition(id, 2, 3));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_addition(id, 5, 4));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_addition(id, 7, 8));

        // Add subtraction lessons
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subtraction(id, 5, 2));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subtraction(id, 10, 3));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_subtraction(id, 8, 5));

        // Add multiplication lessons
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_multiplication(id, 2, 2));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_multiplication(id, 2, 3));
        let id = manager.next_id();
        manager.add_lesson(Lesson::new_multiplication(id, 3, 4));

        manager
    }

    /// Get the next available ID
    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a lesson to the manager
    pub fn add_lesson(&mut self, lesson: Lesson) {
        self.lessons.push(lesson);
    }

    /// Get a lesson by ID
    pub fn get_lesson(&self, id: usize) -> Option<&Lesson> {
        self.lessons.iter().find(|l| l.id == id)
    }

    /// Get all lessons
    pub fn get_all_lessons(&self) -> &[Lesson] {
        &self.lessons
    }

    /// Query lessons based on criteria
    pub fn query_lessons(&self, query: &LessonQuery) -> Vec<&Lesson> {
        let mut results: Vec<&Lesson> = self
            .lessons
            .iter()
            .filter(|lesson| {
                // Filter by lesson type
                if let Some(lesson_type) = query.lesson_type {
                    if lesson.lesson_type != lesson_type {
                        return false;
                    }
                }

                // Filter by difficulty (max of value1 and value2)
                let difficulty = lesson.value1.max(lesson.value2);

                if let Some(min) = query.min_difficulty {
                    if difficulty < min {
                        return false;
                    }
                }

                if let Some(max) = query.max_difficulty {
                    if difficulty > max {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        results
    }

    /// Get total number of lessons
    pub fn count(&self) -> usize {
        self.lessons.len()
    }

    /// Get lessons by type
    pub fn get_by_type(&self, lesson_type: LessonType) -> Vec<&Lesson> {
        self.query_lessons(&LessonQuery {
            lesson_type: Some(lesson_type),
            ..Default::default()
        })
    }
}

impl Default for LessonManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesson_creation() {
        let lesson = Lesson::new_addition(1, 2, 3);
        assert_eq!(lesson.id, 1);
        assert_eq!(lesson.lesson_type, LessonType::Addition);
        assert_eq!(lesson.value1, 2);
        assert_eq!(lesson.value2, 3);
        assert_eq!(lesson.answer, 5);
        assert_eq!(lesson.question, "2 + 3 = ?");
    }

    #[test]
    fn test_check_answer() {
        let lesson = Lesson::new_subitizing(
            1,
            vec![1, 2, 3, 4, 5],
            5,
            "Select the die showing 5".to_string(),
        );
        assert!(lesson.check_answer(5));
        assert!(!lesson.check_answer(4));
    }

    #[test]
    fn test_lesson_manager_add_and_get() {
        let mut manager = LessonManager::new();
        let lesson = Lesson::new_subitizing(
            1,
            vec![1, 2, 3, 4, 5],
            5,
            "Select the die showing 5".to_string(),
        );
        manager.add_lesson(lesson.clone());

        assert_eq!(manager.count(), 1);
        assert_eq!(manager.get_lesson(1), Some(&lesson));
        assert_eq!(manager.get_lesson(999), None);
    }

    #[test]
    fn test_lesson_manager_with_defaults() {
        let manager = LessonManager::with_defaults();
        assert!(manager.count() > 0);

        // Check we have different types of lessons
        assert!(!manager.get_by_type(LessonType::Subitizing).is_empty());
        assert!(!manager.get_by_type(LessonType::Addition).is_empty());
        assert!(!manager.get_by_type(LessonType::Subtraction).is_empty());
        assert!(!manager.get_by_type(LessonType::Multiplication).is_empty());
    }

    #[test]
    fn test_query_by_type() {
        let manager = LessonManager::with_defaults();

        let query = LessonQuery {
            lesson_type: Some(LessonType::Addition),
            ..Default::default()
        };

        let results = manager.query_lessons(&query);
        assert!(!results.is_empty());

        for lesson in results {
            assert_eq!(lesson.lesson_type, LessonType::Addition);
        }
    }

    #[test]
    fn test_query_by_difficulty() {
        let manager = LessonManager::with_defaults();

        let query = LessonQuery {
            min_difficulty: Some(5),
            max_difficulty: Some(10),
            ..Default::default()
        };

        let results = manager.query_lessons(&query);

        for lesson in results {
            let difficulty = lesson.value1.max(lesson.value2);
            assert!((5..=10).contains(&difficulty));
        }
    }

    #[test]
    fn test_query_with_limit() {
        let manager = LessonManager::with_defaults();

        let query = LessonQuery {
            limit: Some(3),
            ..Default::default()
        };

        let results = manager.query_lessons(&query);
        assert!(results.len() <= 3);
    }

    #[test]
    fn test_query_combined_filters() {
        let manager = LessonManager::with_defaults();

        let query = LessonQuery {
            lesson_type: Some(LessonType::Addition),
            max_difficulty: Some(5),
            limit: Some(2),
            ..Default::default()
        };

        let results = manager.query_lessons(&query);

        assert!(results.len() <= 2);
        for lesson in results {
            assert_eq!(lesson.lesson_type, LessonType::Addition);
            let difficulty = lesson.value1.max(lesson.value2);
            assert!(difficulty <= 5);
        }
    }

    #[test]
    fn test_get_all_lessons() {
        let manager = LessonManager::with_defaults();
        let all_lessons = manager.get_all_lessons();
        assert_eq!(all_lessons.len(), manager.count());
    }

    #[test]
    fn test_dice_pattern() {
        let lesson1 = Lesson::new_subitizing(1, vec![1], 1, "Select the die showing 1".to_string());
        assert_eq!(lesson1.get_dice_pattern(), "⚀");

        let lesson2 = Lesson::new_subitizing(2, vec![2], 2, "Select the die showing 2".to_string());
        assert_eq!(lesson2.get_dice_pattern(), "⚁");

        let lesson6 = Lesson::new_subitizing(6, vec![6], 6, "Select the die showing 6".to_string());
        assert_eq!(lesson6.get_dice_pattern(), "⚅");

        // Non-subitizing lesson should return empty string
        let addition_lesson = Lesson::new_addition(7, 1, 1);
        assert_eq!(addition_lesson.get_dice_pattern(), "");
    }

    #[test]
    fn test_dice_art() {
        let art = Lesson::get_dice_art(1);
        assert_eq!(art.len(), 5); // 5 lines
        assert!(art[2].contains("●")); // center dot

        let art3 = Lesson::get_dice_art(3);
        assert_eq!(art3.len(), 5);
        assert!(art3[1].contains("●")); // top corner
        assert!(art3[2].contains("●")); // center
        assert!(art3[3].contains("●")); // bottom corner
    }
}
