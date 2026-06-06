// Demo example to show the subitizing lessons
use ceridwen_core::{Lesson, LessonManager, LessonType};

fn main() {
    let manager = LessonManager::with_defaults();

    println!("=== Ceridwen Subitizing Lessons Demo ===\n");

    // Get all subitizing lessons
    let subitizing_lessons = manager.get_by_type(LessonType::Subitizing);

    println!("Interactive Subitizing Lessons:\n");
    for lesson in subitizing_lessons {
        println!("Lesson #{}", lesson.id);
        println!("  Question: {}", lesson.question);
        println!("  Dice Options: {:?}", lesson.dice_options);
        println!("  Target Number: {:?}", lesson.target_number);
        println!("  Answer: {}", lesson.answer);

        // Show dice art for each option
        println!("\n  Dice Art:");
        for &dice_val in &lesson.dice_options {
            let art = Lesson::get_dice_art(dice_val);
            for line in art {
                println!("    {}", line);
            }
            println!();
        }
        println!();
    }

    println!(
        "Total subitizing lessons: {}",
        manager.get_by_type(LessonType::Subitizing).len()
    );
    println!("Total all lessons: {}", manager.count());
}
