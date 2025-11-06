use ceridwen_core::{LessonManager, LessonType};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod app;
mod ui;

use app::App;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let lesson_manager = LessonManager::with_defaults();
    let mut app = App::new(lesson_manager);

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return Ok(());
                }
                KeyCode::Esc => {
                    app.go_home();
                }
                KeyCode::Char('1') => {
                    app.view_lessons();
                }
                KeyCode::Up => {
                    app.previous_item();
                }
                KeyCode::Down => {
                    app.next_item();
                }
                KeyCode::Enter => {
                    app.select_item();
                }
                KeyCode::Char('c') if app.current_page == app::Page::LessonList => {
                    app.filter_by_type(LessonType::Subitizing);
                }
                KeyCode::Char('a') if app.current_page == app::Page::LessonList => {
                    app.filter_by_type(LessonType::Addition);
                }
                KeyCode::Char('s') if app.current_page == app::Page::LessonList => {
                    app.filter_by_type(LessonType::Subtraction);
                }
                KeyCode::Char('m') if app.current_page == app::Page::LessonList => {
                    app.filter_by_type(LessonType::Multiplication);
                }
                KeyCode::Char('x') if app.current_page == app::Page::LessonList => {
                    app.clear_filter();
                }
                _ => {}
            }
        }
    }
}
