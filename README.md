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

### ceridwen-esp32

ESP32 firmware that uses the core library with ESP32 HAL and SSD1306 OLED display.

**Features:**

- 📟 SSD1306 128x64 OLED display support via I2C
- 🎓 Displays lessons from the core library
- 🔧 ESP32 hardware abstraction layer
- ✅ Testable display utilities without hardware

**Building:**

Prerequisites:
```bash
# Install ESP32 Rust toolchain
cargo install espup
espup install
. $HOME/export-esp.sh

# Install tools
cargo install ldproxy espflash
```

Build and flash:
```bash
cd ceridwen-esp32
cargo run --features esp32
```

Run tests (no hardware required):
```bash
cargo test --package ceridwen-esp32
```

**Hardware:**
- ESP32 development board
- SSD1306 OLED (I2C): SDA=GPIO21, SCL=GPIO22

See [ceridwen-esp32/README.md](ceridwen-esp32/README.md) for more details.

## License

MIT OR Apache-2.0
