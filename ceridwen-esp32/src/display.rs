use ceridwen_core::{Lesson, LessonType};

/// Get the question text for a lesson, generating it if necessary
pub fn get_lesson_question(lesson: &Lesson) -> String {
    if !lesson.question.is_empty() {
        return lesson.question.to_string();
    }

    match lesson.lesson_type {
        LessonType::Addition => format!("{} + {} = ?", lesson.value1, lesson.value2),
        LessonType::Subtraction => format!("{} - {} = ?", lesson.value1, lesson.value2),
        LessonType::Multiplication => format!("{} x {} = ?", lesson.value1, lesson.value2),
        _ => "".to_string(),
    }
}

/// Format a lesson for display on a small screen (128x64 OLED)
/// Returns a tuple of (title, question, additional_info)
pub fn format_lesson_for_display(lesson: &Lesson) -> (String, String, String) {
    let title = match lesson.lesson_type {
        LessonType::Subitizing => "Subitizing",
        LessonType::Addition => "Addition",
        LessonType::Subtraction => "Subtraction",
        LessonType::Multiplication => "Multiplication",
    };

    let full_question = get_lesson_question(lesson);
    let question = truncate_string(&full_question, 20);
    
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

/// Wrap text into multiple lines ensuring no line exceeds max_chars
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_chars {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    // Fallback: If a single word is too long, we might need to force split, 
    // but for now simple word wrapping is sufficient for "Select the die..."
    lines
}

/// Get display coordinates for centering text
pub fn center_text(text_width: i32, screen_width: i32) -> i32 {
    (screen_width - text_width) / 2
}
