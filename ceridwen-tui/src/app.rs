use ceridwen_core::{Lesson, LessonManager, LessonType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    LessonList,
    LessonDetail,
}

pub struct App {
    pub current_page: Page,
    pub lesson_manager: LessonManager,
    pub selected_index: usize,
    pub filtered_lessons: Vec<usize>, // Indices of lessons in the manager
    pub current_filter: Option<LessonType>,
    pub current_lesson_id: Option<usize>,
}

impl App {
    pub fn new(lesson_manager: LessonManager) -> Self {
        let total_lessons = lesson_manager.count();
        let filtered_lessons: Vec<usize> = (0..total_lessons).collect();
        
        Self {
            current_page: Page::Home,
            lesson_manager,
            selected_index: 0,
            filtered_lessons,
            current_filter: None,
            current_lesson_id: None,
        }
    }

    pub fn view_lessons(&mut self) {
        self.current_page = Page::LessonList;
        self.selected_index = 0;
    }

    pub fn go_home(&mut self) {
        self.current_page = Page::Home;
        self.selected_index = 0;
    }

    pub fn next_item(&mut self) {
        match self.current_page {
            Page::Home => {
                // No scrolling on home page
            }
            Page::LessonList => {
                if !self.filtered_lessons.is_empty() {
                    self.selected_index = (self.selected_index + 1) % self.filtered_lessons.len();
                }
            }
            Page::LessonDetail => {
                // No scrolling on detail page
            }
        }
    }

    pub fn previous_item(&mut self) {
        match self.current_page {
            Page::Home => {
                // No scrolling on home page
            }
            Page::LessonList => {
                if !self.filtered_lessons.is_empty() {
                    if self.selected_index == 0 {
                        self.selected_index = self.filtered_lessons.len() - 1;
                    } else {
                        self.selected_index -= 1;
                    }
                }
            }
            Page::LessonDetail => {
                // No scrolling on detail page
            }
        }
    }

    pub fn select_item(&mut self) {
        match self.current_page {
            Page::Home => {
                // On home page, Enter goes to lessons
                self.view_lessons();
            }
            Page::LessonList => {
                // On lesson list, Enter shows lesson detail
                if !self.filtered_lessons.is_empty() && self.selected_index < self.filtered_lessons.len() {
                    let lesson_index = self.filtered_lessons[self.selected_index];
                    let all_lessons = self.lesson_manager.get_all_lessons();
                    if lesson_index < all_lessons.len() {
                        self.current_lesson_id = Some(all_lessons[lesson_index].id);
                        self.current_page = Page::LessonDetail;
                    }
                }
            }
            Page::LessonDetail => {
                // On detail page, Enter goes back to list
                self.current_page = Page::LessonList;
            }
        }
    }

    pub fn filter_by_type(&mut self, lesson_type: LessonType) {
        self.current_filter = Some(lesson_type);
        self.apply_filter();
        self.selected_index = 0;
    }

    pub fn clear_filter(&mut self) {
        self.current_filter = None;
        self.apply_filter();
        self.selected_index = 0;
    }

    fn apply_filter(&mut self) {
        let all_lessons = self.lesson_manager.get_all_lessons();
        
        if let Some(filter_type) = self.current_filter {
            self.filtered_lessons = all_lessons
                .iter()
                .enumerate()
                .filter(|(_, lesson)| lesson.lesson_type == filter_type)
                .map(|(idx, _)| idx)
                .collect();
        } else {
            self.filtered_lessons = (0..all_lessons.len()).collect();
        }
    }

    pub fn get_filtered_lessons(&self) -> Vec<&Lesson> {
        let all_lessons = self.lesson_manager.get_all_lessons();
        self.filtered_lessons
            .iter()
            .filter_map(|&idx| all_lessons.get(idx))
            .collect()
    }

    pub fn get_current_lesson(&self) -> Option<&Lesson> {
        self.current_lesson_id.and_then(|id| self.lesson_manager.get_lesson(id))
    }
}
