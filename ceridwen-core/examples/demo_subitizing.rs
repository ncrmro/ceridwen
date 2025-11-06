// Demo example to show the subitizing lessons
use ceridwen_core::{LessonManager, LessonType};

fn main() {
    let manager = LessonManager::with_defaults();

    println!("=== Ceridwen Subitizing Lessons Demo ===\n");

    // Get all subitizing lessons
    let subitizing_lessons = manager.get_by_type(LessonType::Subitizing);

    println!("Subitizing Lessons (Dice Recognition):\n");
    for lesson in subitizing_lessons {
        println!("Lesson #{}", lesson.id);
        println!("  Question: {}", lesson.question);
        println!("  Dice Pattern: {}", lesson.get_dice_pattern());
        println!("  Answer: {}", lesson.answer);
        println!();
    }

    println!(
        "\nTotal subitizing lessons: {}",
        manager.get_by_type(LessonType::Subitizing).len()
    );
    println!("Total all lessons: {}", manager.count());
}
