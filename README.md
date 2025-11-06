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
- **CompletionState**: Track completion status for lessons, with JSON persistence support

### ceridwen-tui

A terminal user interface (TUI) application built with Ratatui for browsing and viewing lessons.

**Features:**

- 📚 Browse all available lessons
- 🔍 Filter lessons by type (Subitizing, Addition, Subtraction, Multiplication)
- 📖 View detailed information about each lesson
- ⌨️ Keyboard-driven navigation
- 🎨 Beautiful terminal UI with emoji icons
- ✅ Track lesson completion with persistent storage
- 📊 View progress statistics

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
  - `Enter` - View lesson details or start interactive subitizing
  - `C` - Filter by Subitizing lessons
  - `A` - Filter by Addition lessons
  - `S` - Filter by Subtraction lessons
  - `M` - Filter by Multiplication lessons
  - `X` - Clear filter (show all)
  - `Esc` - Return to home
  - `Q` - Quit

- **Interactive Subitizing:**
  - `←/→` - Navigate between dice
  - `Enter` - Select the highlighted die
  - `Esc` - Return to lesson list
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
│ → 🎲 1 - Select the die showing 1 (⚀)                         │
│   🎲 2 - Select the die showing 2 (⚁)                         │
│   ➕ 5 - 1 + 1 = ?                                             │
│   ➕ 6 - 2 + 3 = ?                                             │
│   ➖ 9 - 5 - 2 = ?                                             │
│   ✖️ 12 - 2 × 2 = ?                                            │
└────────────────────────────────────────────────────────────────┘
```

### Interactive Subitizing
```
┌──────────────────────────────────────────────────────────────┐
│              🎲 Subitizing Exercise                           │
└──────────────────────────────────────────────────────────────┘
┌Task───────────────────────────────────────────────────────────┐
│                                                                │
│              Select the die showing 1                          │
│                                                                │
└────────────────────────────────────────────────────────────────┘
┌Dice───────────────────────────────────────────────────────────┐
│                                                                │
│         ┌─────┐          ┌─────┐                              │
│         │     │          │ ●   │                              │
│         │  ●  │          │     │                              │
│         │     │          │   ● │                              │
│         └─────┘          └─────┘                              │
│                                                                │
│          ↑ ↑ ↑                                                │
│                                                                │
└────────────────────────────────────────────────────────────────┘
┌Result─────────────────────────────────────────────────────────┐
│                                                                │
│                      ✅ Correct!                              │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

## Completion Tracking

The TUI application now tracks lesson completion and persists progress to disk:

- **Automatic Persistence**: Progress is saved to `~/.local/share/ceridwen/ceridwen_completion.json` (or platform-specific location)
- **Visual Indicators**: Completed lessons are marked with ✅ in the lesson list
- **Progress Display**: Home screen shows completion statistics (e.g., "3/14 lessons completed")
- **ESP32 Compatible**: Uses simple JSON format designed to work with embedded systems

### Example Completion Data

```json
{
  "completions": [
    {
      "lesson_id": 1,
      "completed": true,
      "attempts": 1
    }
  ]
}
```

For more details, see [COMPLETION_TRACKING.md](COMPLETION_TRACKING.md).

## Future Components

- **ESP32 Firmware**: Will use the core library with ESP32 HAL, OLED display (SSD1306), and serial communication

## License

MIT OR Apache-2.0
