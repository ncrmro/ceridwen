pub mod display;
pub mod renderer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Browsing,
    Interactive,
    Feedback(bool), // true = correct, false = incorrect
}

#[cfg(test)]
mod tests {
    use super::display::*;
    use ceridwen_core::Lesson;

    #[test]
    fn test_format_subitizing_lesson() {
        let lesson = Lesson::new_subitizing(
            1,
            &[1, 2, 3],
            2,
            "Select the die showing 2",
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