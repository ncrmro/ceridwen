use crate::AppMode;
use ceridwen_core::{Lesson, LessonType};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Left,
    Right,
    Both,
}

/// Hardware-independent two-button interaction, shared by firmware and simulator.
#[derive(Debug, Clone)]
pub struct Session {
    pub lesson_index: usize,
    pub mode: AppMode,
    pub selection: usize,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            lesson_index: 0,
            mode: AppMode::Browsing,
            selection: 0,
        }
    }
}

impl Session {
    pub fn apply(&mut self, action: Action, lessons: &[&Lesson]) {
        if lessons.is_empty() {
            return;
        }
        self.lesson_index %= lessons.len();
        let lesson = lessons[self.lesson_index];
        match (self.mode, action) {
            (AppMode::Browsing, Action::Left) => {
                self.lesson_index = (self.lesson_index + lessons.len() - 1) % lessons.len()
            }
            (AppMode::Browsing, Action::Right) => {
                self.lesson_index = (self.lesson_index + 1) % lessons.len()
            }
            (AppMode::Browsing, Action::Both) => {
                self.mode = AppMode::Interactive;
                self.selection = 0;
            }
            (AppMode::Interactive, Action::Both) => {
                let answer = if lesson.lesson_type == LessonType::Subitizing {
                    lesson
                        .dice_options
                        .get(self.selection)
                        .copied()
                        .filter(|_| self.selection < lesson.dice_options_count as usize)
                } else {
                    u8::try_from(self.selection).ok()
                };
                self.mode =
                    AppMode::Feedback(answer.is_some_and(|answer| lesson.check_answer(answer)));
            }
            (AppMode::Interactive, action) => {
                let count = if lesson.lesson_type == LessonType::Subitizing {
                    lesson.dice_options_count as usize
                } else {
                    256
                };
                if count == 0 {
                    return;
                }
                self.selection = match action {
                    Action::Left => (self.selection + count - 1) % count,
                    Action::Right => (self.selection + 1) % count,
                    Action::Both => unreachable!(),
                };
            }
            (AppMode::Feedback(false), _) => self.mode = AppMode::Interactive,
            (AppMode::Feedback(true), Action::Both) => {
                self.lesson_index = (self.lesson_index + 1) % lessons.len();
                self.mode = AppMode::Browsing;
                self.selection = 0;
            }
            (AppMode::Feedback(true), _) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrong_answer_retries_same_lesson_and_correct_answer_advances() {
        let first = Lesson::new_subitizing(1, &[1, 2], 2, "Select 2");
        let next = Lesson::new_addition(2, 1, 1);
        let lessons = [&first, &next];
        let mut s = Session::default();
        s.apply(Action::Both, &lessons);
        s.apply(Action::Both, &lessons);
        assert_eq!(s.mode, AppMode::Feedback(false));
        s.apply(Action::Both, &lessons);
        assert_eq!(s.lesson_index, 0);
        assert_eq!(s.mode, AppMode::Interactive);
        s.apply(Action::Right, &lessons);
        s.apply(Action::Both, &lessons);
        assert_eq!(s.mode, AppMode::Feedback(true));
        s.apply(Action::Both, &lessons);
        assert_eq!(s.lesson_index, 1);
    }
    #[test]
    fn arithmetic_requires_the_selected_answer() {
        let lesson = Lesson::new_addition(1, 1, 1);
        let lessons = [&lesson];
        let mut s = Session::default();
        s.apply(Action::Both, &lessons);
        s.apply(Action::Both, &lessons);
        assert_eq!(s.mode, AppMode::Feedback(false));
        s.apply(Action::Both, &lessons);
        s.apply(Action::Right, &lessons);
        s.apply(Action::Right, &lessons);
        s.apply(Action::Both, &lessons);
        assert_eq!(s.mode, AppMode::Feedback(true));
    }
    #[test]
    fn empty_lessons_do_not_panic() {
        Session::default().apply(Action::Both, &[]);
    }
}
