# ceridwen

A Rust-based educational system for teaching subitizing and arithmetic. The system is designed to work with ESP32 devices and terminal user interfaces (TUI).

## Architecture

The project consists of a shared core library that contains business logic and types, and a TUI application for interacting with lessons.

### ceridwen-core

The core library containing shared types and business logic for both ESP32 firmware and TUI applications.

**Key Components:**

- **Lesson Types**: Subitizing (dice patterns 1-6), Addition, Subtraction, Multiplication
- **LessonManager**: In-memory lesson storage and querying
- **LessonQuery**: Flexible query system for filtering lessons by type, difficulty, and more

### ceridwen-tui

A terminal user interface (TUI) application built with Ratatui for browsing and viewing lessons.

**Features:**

- 📚 Browse all available lessons
- 🔍 Filter lessons by type (Subitizing, Addition, Subtraction, Multiplication)
- 📖 View detailed information about each lesson
- ⌨️ Keyboard-driven navigation
- 🎨 Beautiful terminal UI with emoji icons

## Usage

### Running the TUI Application

```bash
cargo run --package ceridwen-tui
```

**Keyboard Controls:**

- **Home Screen:**
  - `1` - View Lessons
  - `Q` - Quit

- **Lesson List:**
  - `↑/↓` - Navigate through lessons
  - `Enter` - View lesson details
  - `C` - Filter by Subitizing lessons
  - `A` - Filter by Addition lessons
  - `S` - Filter by Subtraction lessons
  - `M` - Filter by Multiplication lessons
  - `X` - Clear filter (show all)
  - `Esc` - Return to home
  - `Q` - Quit

- **Lesson Detail:**
  - `Enter` or `Esc` - Return to lesson list
  - `Q` - Quit

### Running Tests

```bash
cargo test --package ceridwen-core
```

### Using the Core Library

```rust
use ceridwen_core::{LessonManager, LessonQuery, LessonType};

// Create a lesson manager with default lessons
let manager = LessonManager::with_defaults();

// Query lessons by type
let query = LessonQuery {
    lesson_type: Some(LessonType::Addition),
    ..Default::default()
};
let results = manager.query_lessons(&query);

// Check answers
for lesson in results {
    println!("{}", lesson.question);
    let is_correct = lesson.check_answer(user_answer);
}
```

## Screenshots

### Home Screen
```
┌──────────────────────────────────────────────────────────────┐
│              🎓 Ceridwen - Educational System                │
└──────────────────────────────────────────────────────────────┘
┌Welcome────────────────────────────────────────────────────────┐
│                    Welcome to Ceridwen!                        │
│        An educational system for teaching subitizing           │
│                   and arithmetic.                              │
│                       Main Menu:                               │
│                   1. View Lessons                              │
└────────────────────────────────────────────────────────────────┘
```

### Lessons List
```
┌──────────────────────────────────────────────────────────────┐
│                     📚 Lesson Library                         │
└──────────────────────────────────────────────────────────────┘
┌Lessons────────────────────────────────────────────────────────┐
│ → 🎲 1 - What number do you see? (⚀)                          │
│   🎲 2 - What number do you see? (⚁)                          │
│   ➕ 7 - 1 + 1 = ?                                             │
│   ➕ 8 - 2 + 3 = ?                                             │
│   ➖ 11 - 5 - 2 = ?                                            │
│   ✖️ 14 - 2 × 2 = ?                                            │
└────────────────────────────────────────────────────────────────┘
```

## Future Components

- **ESP32 Firmware**: Will use the core library with ESP32 HAL, OLED display (SSD1306), and serial communication

## License

MIT OR Apache-2.0
