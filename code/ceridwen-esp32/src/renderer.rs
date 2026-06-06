use crate::AppMode;
use ceridwen_core::{Lesson, LessonType};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

pub fn draw_dice<D>(target: &mut D, top_left: Point, value: u8) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let fill_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build();

    // Draw box 20x20
    Rectangle::new(top_left, Size::new(20, 20))
        .into_styled(style)
        .draw(target)?;

    let dot_radius = 2;
    let center = top_left + Point::new(10, 10);
    let tl = top_left + Point::new(5, 5);
    let tr = top_left + Point::new(15, 5);
    let ml = top_left + Point::new(5, 10);
    let mr = top_left + Point::new(15, 10);
    let bl = top_left + Point::new(5, 15);
    let br = top_left + Point::new(15, 15);

    let draw_dot = |pos: Point, t: &mut D| -> Result<(), D::Error> {
        Circle::with_center(pos, dot_radius)
            .into_styled(fill_style)
            .draw(t)
    };

    match value {
        1 => {
            draw_dot(center, target)?;
        }
        2 => {
            draw_dot(tl, target)?;
            draw_dot(br, target)?;
        }
        3 => {
            draw_dot(tl, target)?;
            draw_dot(center, target)?;
            draw_dot(br, target)?;
        }
        4 => {
            draw_dot(tl, target)?;
            draw_dot(tr, target)?;
            draw_dot(bl, target)?;
            draw_dot(br, target)?;
        }
        5 => {
            draw_dot(tl, target)?;
            draw_dot(tr, target)?;
            draw_dot(center, target)?;
            draw_dot(bl, target)?;
            draw_dot(br, target)?;
        }
        6 => {
            draw_dot(tl, target)?;
            draw_dot(tr, target)?;
            draw_dot(ml, target)?;
            draw_dot(mr, target)?;
            draw_dot(bl, target)?;
            draw_dot(br, target)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn draw_screen<D>(
    target: &mut D,
    app_mode: AppMode,
    lesson: &Lesson,
    lesson_index: usize,
    total_lessons: usize,
    selected_option_index: usize,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    match app_mode {
        AppMode::Browsing => {
            // Show Question Preview
            // Moved down to y=15 for better margin
            Text::new("Question:", Point::new(0, 15), text_style).draw(target)?;

            let q_string = crate::display::get_lesson_question(lesson);
            let lines = crate::display::wrap_text(&q_string, 21);
            
            // Draw up to 3 lines of question
            for (i, line) in lines.iter().take(3).enumerate() {
                let y = 30 + (i as i32 * 10);
                Text::new(line, Point::new(0, y), text_style).draw(target)?;
            }
        }
        AppMode::Interactive => {
            // Render Options
            if lesson.lesson_type == LessonType::Subitizing && lesson.dice_options_count > 0 {
                let start_x = 10;
                let y = 40; // Moved down to y=40
                let spacing = 25;

                // Show Target hint (Question)
                let q_string = crate::display::get_lesson_question(lesson);
                let lines = crate::display::wrap_text(&q_string, 21);
                for (i, line) in lines.iter().take(2).enumerate() {
                    let text_y = 15 + (i as i32 * 10); // Start at y=15
                    Text::new(line, Point::new(0, text_y), text_style).draw(target)?;
                }

                for i in 0..lesson.dice_options_count as usize {
                    let val = lesson.dice_options[i];
                    let x = start_x + (i as i32 * spacing);

                    draw_dice(target, Point::new(x, y), val)?;

                    // Draw cursor if selected
                    if i == selected_option_index {
                        // Underline
                        Rectangle::new(Point::new(x, y + 22), Size::new(20, 2))
                            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                            .draw(target)?;
                    }
                }
            } else {
                // Fallback for non-subitizing
                Text::new("No interactive view", Point::new(0, 30), text_style).draw(target)?;
            }
        }
        AppMode::Feedback(correct) => {
            let msg = if correct {
                "Correct! :)"
            } else {
                "Try Again :("
            };
            // Centered vertically-ish
            Text::new(msg, Point::new(30, 35), text_style).draw(target)?;

            Text::new("Both -> Next", Point::new(20, 55), text_style).draw(target)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ceridwen_core::Lesson;
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        prelude::*,
    };

    /// A wrapper around a DrawTarget that asserts all pixels are within bounds
    struct BoundsCheckDisplay<D> {
        inner: D,
        width: i32,
        height: i32,
    }

    impl<D> BoundsCheckDisplay<D> {
        fn new(inner: D, width: i32, height: i32) -> Self {
            Self { inner, width, height }
        }
    }

    impl<D: DrawTarget> Dimensions for BoundsCheckDisplay<D> {
        fn bounding_box(&self) -> Rectangle {
            // Return a huge bounding box so standard drawables don't pre-clip.
            // We want to receive the OOB pixels in draw_iter to detect the error.
            Rectangle::new(Point::new(0, 0), Size::new(1000, 1000))
        }
    }

    impl<D: DrawTarget> DrawTarget for BoundsCheckDisplay<D> {
        type Color = D::Color;
        type Error = D::Error;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            for Pixel(pt, color) in pixels {
                if pt.x < 0 || pt.x >= self.width || pt.y < 0 || pt.y >= self.height {
                    // Panic immediately for easy debugging
                    panic!("Pixel out of bounds: {:?} (Limits: {}x{})", pt, self.width, self.height);
                }
                self.inner.draw_iter(core::iter::once(Pixel(pt, color)))?;
            }
            Ok(())
        }
    }

    struct DummyDisplay;
    impl DrawTarget for DummyDisplay {
        type Color = BinaryColor;
        type Error = core::convert::Infallible;
        
        fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            Ok(())
        }
    }

    impl Dimensions for DummyDisplay {
        fn bounding_box(&self) -> Rectangle {
            Rectangle::new(Point::zero(), Size::new(128, 64))
        }
    }

    #[test]
    fn test_draw_browsing_fits_screen() {
        let mut display = BoundsCheckDisplay::new(DummyDisplay, 128, 64);
        
        let lesson = Lesson::new_subitizing(1, &[1, 2], 1, "Select the die showing 1");
        
        draw_screen(
            &mut display,
            AppMode::Browsing,
            &lesson,
            0,
            10,
            0
        ).unwrap();
    }
    
    #[test]
    fn test_long_question_truncation() {
        let mut display = BoundsCheckDisplay::new(DummyDisplay, 128, 64);
        
        let long_q = "This is a very long question that might not fit on the screen properly";
        let lesson = Lesson::new_subitizing(1, &[1], 1, long_q);
        
        // This should NOT panic if truncation logic works
        draw_screen(
            &mut display,
            AppMode::Browsing,
            &lesson,
            0,
            10,
            0
        ).unwrap();
    }
}
