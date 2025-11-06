use ceridwen_core::{Lesson, LessonManager, LessonType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    LessonList,
    LessonDetail,
    SubitizingInteractive,
}

pub struct App {
    pub current_page: Page,
    pub lesson_manager: LessonManager,
    pub selected_index: usize,
    pub filtered_lessons: Vec<usize>, // Indices of lessons in the manager
    pub current_filter: Option<LessonType>,
    pub current_lesson_id: Option<usize>,
    // For interactive subitizing
    pub selected_dice_index: usize,
    pub feedback_message: Option<String>,
    pub show_feedback: bool,
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
            selected_dice_index: 0,
            feedback_message: None,
            show_feedback: false,
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
            Page::SubitizingInteractive => {
                // Navigate right through dice
                if let Some(lesson) = self.get_current_lesson() {
                    if !lesson.dice_options.is_empty() {
                        self.selected_dice_index =
                            (self.selected_dice_index + 1) % lesson.dice_options.len();
                    }
                }
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
            Page::SubitizingInteractive => {
                // Navigate left through dice
                if let Some(lesson) = self.get_current_lesson() {
                    if !lesson.dice_options.is_empty() {
                        if self.selected_dice_index == 0 {
                            self.selected_dice_index = lesson.dice_options.len() - 1;
                        } else {
                            self.selected_dice_index -= 1;
                        }
                    }
                }
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
                // On lesson list, Enter shows lesson detail or interactive mode
                if !self.filtered_lessons.is_empty()
                    && self.selected_index < self.filtered_lessons.len()
                {
                    let lesson_index = self.filtered_lessons[self.selected_index];
                    let all_lessons = self.lesson_manager.get_all_lessons();
                    if lesson_index < all_lessons.len() {
                        let lesson = &all_lessons[lesson_index];
                        self.current_lesson_id = Some(lesson.id);

                        // If it's a subitizing lesson with dice options, go to interactive mode
                        if lesson.lesson_type == LessonType::Subitizing
                            && !lesson.dice_options.is_empty()
                        {
                            self.current_page = Page::SubitizingInteractive;
                            self.selected_dice_index = 0;
                            self.feedback_message = None;
                            self.show_feedback = false;
                        } else {
                            self.current_page = Page::LessonDetail;
                        }
                    }
                }
            }
            Page::LessonDetail => {
                // On detail page, Enter goes back to list
                self.current_page = Page::LessonList;
            }
            Page::SubitizingInteractive => {
                // Check if the selected dice matches the target
                if let Some(lesson) = self.get_current_lesson() {
                    if self.selected_dice_index < lesson.dice_options.len() {
                        let selected_value = lesson.dice_options[self.selected_dice_index];
                        if lesson.check_answer(selected_value) {
                            self.feedback_message = Some("✅ Correct!".to_string());
                            self.show_feedback = true;
                        } else {
                            self.feedback_message = Some("❌ Incorrect. Try again!".to_string());
                            self.show_feedback = true;
                        }
                    }
                }
            }
        }
    }

    pub fn clear_feedback(&mut self) {
        self.show_feedback = false;
        self.feedback_message = None;
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
        self.current_lesson_id
            .and_then(|id| self.lesson_manager.get_lesson(id))
    }
}
