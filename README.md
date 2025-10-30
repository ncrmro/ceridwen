# ceridwen

A Rust-based educational system for teaching counting and arithmetic. The system is designed to work with ESP32 devices and terminal user interfaces (TUI).

## Architecture

The project consists of a shared core library that contains business logic and types:

### ceridwen-core

The core library containing shared types and business logic for both ESP32 firmware and TUI applications.

**Key Components:**

- **Lesson Types**: Counting, Addition, Subtraction, Multiplication
- **LessonManager**: In-memory lesson storage and querying
- **LessonQuery**: Flexible query system for filtering lessons by type, difficulty, and more

## Usage

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

## Future Components

- **ESP32 Firmware**: Will use the core library with ESP32 HAL, OLED display (SSD1306), and serial communication
- **TUI Host**: Will use the core library with Ratatui for terminal interface and serial communication

## License

MIT OR Apache-2.0
