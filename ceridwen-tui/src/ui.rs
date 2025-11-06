use crate::app::{App, Page};
use ceridwen_core::{Lesson, LessonType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_page {
        Page::Home => draw_home(f, app),
        Page::LessonList => draw_lesson_list(f, app),
        Page::LessonDetail => draw_lesson_detail(f, app),
        Page::SubitizingInteractive => draw_subitizing_interactive(f, app),
    }
}

fn draw_home(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🎓 Ceridwen - Educational System")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Content with completion stats
    let total_lessons = app.lesson_manager.count();
    let completed_lessons = app.completion_state.total_completed();
    
    let welcome_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "Welcome to Ceridwen!",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("An educational system for teaching subitizing and arithmetic."),
        Line::from(""),
        Line::from(vec![
            Span::styled("Progress: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}/{} lessons completed", completed_lessons, total_lessons),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("Main Menu:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1. ", Style::default().fg(Color::Green)),
            Span::raw("View Lessons"),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled(
                "1",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to view lessons or ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Q",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to quit", Style::default().fg(Color::Gray)),
        ]),
    ];

    let content = Paragraph::new(welcome_text)
        .block(Block::default().borders(Borders::ALL).title("Welcome"))
        .alignment(Alignment::Center);
    f.render_widget(content, chunks[1]);

    // Help
    let help = Paragraph::new("1: View Lessons | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn draw_lesson_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Filter info
            Constraint::Min(5),    // Lesson list
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📚 Lesson Library")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Filter info
    let filter_text = if let Some(filter_type) = app.current_filter {
        let type_name = match filter_type {
            LessonType::Subitizing => "Subitizing",
            LessonType::Addition => "Addition",
            LessonType::Subtraction => "Subtraction",
            LessonType::Multiplication => "Multiplication",
        };
        format!(
            "Filter: {} ({} lessons)",
            type_name,
            app.filtered_lessons.len()
        )
    } else {
        format!("Showing all lessons ({} total)", app.filtered_lessons.len())
    };

    let filter_info = Paragraph::new(filter_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Filter"));
    f.render_widget(filter_info, chunks[1]);

    // Lesson list
    let lessons = app.get_filtered_lessons();
    let items: Vec<ListItem> = lessons
        .iter()
        .enumerate()
        .map(|(idx, lesson)| {
            let type_icon = match lesson.lesson_type {
                LessonType::Subitizing => "🎲",
                LessonType::Addition => "➕",
                LessonType::Subtraction => "➖",
                LessonType::Multiplication => "✖️",
            };

            let is_completed = app.completion_state.is_completed(lesson.id);
            let completion_icon = if is_completed { "✅" } else { "  " };

            let style = if idx == app.selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if idx == app.selected_index {
                "→ "
            } else {
                "  "
            };

            // For subitizing lessons, show the dice pattern
            let display_text = if lesson.lesson_type == LessonType::Subitizing {
                format!(
                    "{}{} {} {} - {} ({})",
                    prefix,
                    completion_icon,
                    type_icon,
                    lesson.id,
                    lesson.question,
                    lesson.get_dice_pattern()
                )
            } else {
                format!(
                    "{}{} {} {} - {}",
                    prefix, completion_icon, type_icon, lesson.id, lesson.question
                )
            };

            ListItem::new(display_text).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Lessons"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(list, chunks[2]);

    // Help
    let help = Paragraph::new("↑/↓: Navigate | Enter: View Details | C/A/S/M: Filter by Type | X: Clear Filter | Esc: Home | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[3]);
}

fn draw_lesson_detail(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Lesson details
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📖 Lesson Details")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Lesson details
    if let Some(lesson) = app.get_current_lesson() {
        let type_name = match lesson.lesson_type {
            LessonType::Subitizing => "Subitizing",
            LessonType::Addition => "Addition",
            LessonType::Subtraction => "Subtraction",
            LessonType::Multiplication => "Multiplication",
        };

        let type_icon = match lesson.lesson_type {
            LessonType::Subitizing => "🎲",
            LessonType::Addition => "➕",
            LessonType::Subtraction => "➖",
            LessonType::Multiplication => "✖️",
        };

        let mut detail_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Lesson ID: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{}", lesson.id),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{} {}", type_icon, type_name)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                if app.completion_state.is_completed(lesson.id) {
                    Span::styled(
                        "✅ Completed",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled("⬜ Not completed", Style::default().fg(Color::Gray))
                },
            ]),
            Line::from(""),
        ];

        // For subitizing lessons, show the dice pattern prominently
        if lesson.lesson_type == LessonType::Subitizing {
            detail_text.extend(vec![
                Line::from(vec![Span::styled(
                    "Dice Pattern: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    format!("          {}", lesson.get_dice_pattern()),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(""),
            ]);
        }

        detail_text.extend(vec![
            Line::from(vec![Span::styled(
                "Question: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                format!("  {}", lesson.question),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Answer: ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", lesson.answer),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]);

        // Only show values for non-subitizing lessons
        if lesson.lesson_type != LessonType::Subitizing {
            detail_text.extend(vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("Values: ", Style::default().fg(Color::Gray)),
                    Span::raw(format!(
                        "value1={}, value2={}",
                        lesson.value1, lesson.value2
                    )),
                ]),
            ]);
        }

        let details = Paragraph::new(detail_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(details, chunks[1]);
    } else {
        let error = Paragraph::new("No lesson selected")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error, chunks[1]);
    }

    // Help
    let help = Paragraph::new("Enter/Esc: Back to List | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn draw_subitizing_interactive(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(5), // Question
            Constraint::Min(10),   // Dice display area
            Constraint::Length(4), // Feedback
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🎲 Subitizing Exercise")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(lesson) = app.get_current_lesson() {
        // Question
        let question_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                &lesson.question,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
        ];
        let question = Paragraph::new(question_text)
            .block(Block::default().borders(Borders::ALL).title("Task"))
            .alignment(Alignment::Center);
        f.render_widget(question, chunks[1]);

        // Dice display
        let mut dice_lines: Vec<Line> = vec![Line::from("")];

        if !lesson.dice_options.is_empty() {
            // Get all dice art
            let dice_arts: Vec<Vec<String>> = lesson
                .dice_options
                .iter()
                .map(|&val| Lesson::get_dice_art(val))
                .collect();

            // Render each line of all dice horizontally
            for line_idx in 0..5 {
                let mut line_parts = vec![];
                for (dice_idx, dice_art) in dice_arts.iter().enumerate() {
                    let is_selected = dice_idx == app.selected_dice_index;

                    if is_selected {
                        line_parts.push(Span::styled(
                            dice_art[line_idx].clone(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ));
                    } else {
                        line_parts.push(Span::raw(dice_art[line_idx].clone()));
                    }

                    // Add spacing between dice
                    if dice_idx < dice_arts.len() - 1 {
                        line_parts.push(Span::raw("  "));
                    }
                }
                dice_lines.push(Line::from(line_parts));
            }

            // Add indicator below selected die
            dice_lines.push(Line::from(""));
            let mut indicator_parts = vec![];
            const INDICATOR_TEXT: &str = "  ↑ ↑ ↑";
            let indicator_width = INDICATOR_TEXT.len();
            for (dice_idx, _) in dice_arts.iter().enumerate() {
                if dice_idx == app.selected_dice_index {
                    indicator_parts.push(Span::styled(
                        INDICATOR_TEXT,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    indicator_parts.push(Span::raw(" ".repeat(indicator_width)));
                }

                if dice_idx < dice_arts.len() - 1 {
                    indicator_parts.push(Span::raw("  "));
                }
            }
            dice_lines.push(Line::from(indicator_parts));
        }

        let dice_display = Paragraph::new(dice_lines)
            .block(Block::default().borders(Borders::ALL).title("Dice"))
            .alignment(Alignment::Center);
        f.render_widget(dice_display, chunks[2]);

        // Feedback
        let feedback_text = if app.show_feedback {
            if let Some(msg) = &app.feedback_message {
                vec![
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        msg,
                        Style::default()
                            .fg(if msg.contains("✅") {
                                Color::Green
                            } else {
                                Color::Red
                            })
                            .add_modifier(Modifier::BOLD),
                    )]),
                ]
            } else {
                vec![Line::from("")]
            }
        } else {
            vec![Line::from("")]
        };

        let feedback = Paragraph::new(feedback_text)
            .block(Block::default().borders(Borders::ALL).title("Result"))
            .alignment(Alignment::Center);
        f.render_widget(feedback, chunks[3]);
    }

    // Help
    let help = Paragraph::new("←/→: Navigate | Enter: Select | Esc: Back | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[4]);
}
