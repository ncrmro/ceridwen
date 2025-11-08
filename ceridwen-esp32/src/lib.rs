/// Display module for rendering lessons on SSD1306
pub mod display {
    use ceridwen_core::{Lesson, LessonType};

    /// Format a lesson for display on a small screen (128x64 OLED)
    /// Returns a tuple of (title, question, additional_info)
    pub fn format_lesson_for_display(lesson: &Lesson) -> (String, String, String) {
        let title = match lesson.lesson_type {
            LessonType::Subitizing => "Subitizing",
            LessonType::Addition => "Addition",
            LessonType::Subtraction => "Subtraction",
            LessonType::Multiplication => "Multiplication",
        };

        let question = truncate_string(&lesson.question, 20);
        
        let additional_info = match lesson.lesson_type {
            LessonType::Subitizing => lesson.get_dice_pattern().to_string(),
            _ => format!("= {}", lesson.answer),
        };

        (title.to_string(), question, additional_info)
    }

    /// Truncate a string to a maximum length
    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }

    /// Get display coordinates for centering text
    pub fn center_text(text_width: i32, screen_width: i32) -> i32 {
        (screen_width - text_width) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::display::*;
    use ceridwen_core::Lesson;

    #[test]
    fn test_format_subitizing_lesson() {
        let lesson = Lesson::new_subitizing(
            1,
            vec![1, 2, 3],
            2,
            "Select the die showing 2".to_string(),
        );

        let (title, question, info) = format_lesson_for_display(&lesson);
        
        assert_eq!(title, "Subitizing");
        assert_eq!(question, "Select the die sh...");
        assert_eq!(info, "⚁");
    }

    #[test]
    fn test_format_addition_lesson() {
        let lesson = Lesson::new_addition(1, 2, 3);

        let (title, question, info) = format_lesson_for_display(&lesson);
        
        assert_eq!(title, "Addition");
        assert_eq!(question, "2 + 3 = ?");
        assert_eq!(info, "= 5");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello", 10), "Hello");
        assert_eq!(truncate_string("Hello World!", 8), "Hello...");
        assert_eq!(truncate_string("Test", 4), "Test");
    }

    #[test]
    fn test_center_text() {
        assert_eq!(center_text(50, 128), 39);
        assert_eq!(center_text(64, 128), 32);
        assert_eq!(center_text(0, 128), 64);
    }
}
