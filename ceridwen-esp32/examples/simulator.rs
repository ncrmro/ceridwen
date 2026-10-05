//! Render the real firmware UI without a board: simulator rrrbrr screen.svg
//! l = left, r = right, b = both. Commands start from a fresh session.
use ceridwen_core::LessonManager;
use ceridwen_esp32::{
    renderer,
    session::{Action, Session},
};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::Rectangle};
use std::{convert::Infallible, fmt::Write};

struct Screen {
    pixels: [[bool; 128]; 64],
}
impl Dimensions for Screen {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(128, 64))
    }
}
impl DrawTarget for Screen {
    type Color = BinaryColor;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<BinaryColor>>,
    {
        for Pixel(p, c) in pixels {
            if (0..128).contains(&p.x) && (0..64).contains(&p.y) {
                self.pixels[p.y as usize][p.x as usize] = c == BinaryColor::On;
            }
        }
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let commands = args.get(1).map(String::as_str).unwrap_or("");
    let output = args.get(2).map(String::as_str).unwrap_or("screen.svg");
    let manager = LessonManager::with_defaults();
    let lessons = manager.get_all_lessons_vec();
    let mut session = Session::default();
    for c in commands.chars() {
        let action = match c {
            'l' => Action::Left,
            'r' => Action::Right,
            'b' => Action::Both,
            _ => return Err(format!("Unknown command {c:?}; use l, r, b").into()),
        };
        session.apply(action, &lessons);
    }
    let mut screen = Screen {
        pixels: [[false; 128]; 64],
    };
    renderer::draw_screen(
        &mut screen,
        session.mode,
        lessons[session.lesson_index],
        session.lesson_index,
        lessons.len(),
        session.selection,
    )?;
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 64\" width=\"768\" height=\"384\" shape-rendering=\"crispEdges\"><rect width=\"128\" height=\"64\" fill=\"#080e12\"/><g fill=\"white\">");
    for (y, row) in screen.pixels.iter().enumerate() {
        for (x, on) in row.iter().enumerate() {
            if *on {
                write!(svg, "<rect x=\"{x}\" y=\"{y}\" width=\"1\" height=\"1\"/>")?;
            }
        }
    }
    svg.push_str("</g></svg>\n");
    std::fs::write(output, svg)?;
    println!(
        "Lesson {}: {:?}, selection {} -> {}",
        session.lesson_index + 1,
        session.mode,
        session.selection,
        output
    );
    Ok(())
}
