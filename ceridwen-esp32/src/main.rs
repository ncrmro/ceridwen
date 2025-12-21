use ceridwen_core::{LessonManager, LessonType};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{PinDriver, Pull},
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn draw_dice<D>(target: &mut D, top_left: Point, value: u8) -> Result<(), D::Error>
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

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF services
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting Ceridwen ESP32...");

    // Create peripherals
    let peripherals = Peripherals::take()?;

    // Hardware Configuration
    // I2C Pins (Moved to avoid LED conflict on GPIO8):
    // - SDA: GPIO4
    // - SCL: GPIO5
    log::info!("Configuring I2C on GPIO4 (SDA) and GPIO5 (SCL)");
    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;

    // Onboard LED Control
    // Most ESP32-C3 boards have an LED on GPIO8.
    // Retry setting it LOW (Active High logic) now that I2C is moved.
    log::info!("Disabling onboard LED on GPIO8");
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;
    led.set_low()?;
    
        // Button Configuration
    
        // Button 1 (Left/Prev): GPIO0
    
        // Button 2 (Right/Next): GPIO1
    
        // Action (Both): Show Answer
    
        log::info!("Configuring buttons on GPIO0 (Left) and GPIO1 (Right)");
    
        let mut btn_left = PinDriver::input(peripherals.pins.gpio0)?;
    
        btn_left.set_pull(Pull::Up)?;
    
        
    
        let mut btn_right = PinDriver::input(peripherals.pins.gpio1)?;
    
        btn_right.set_pull(Pull::Up)?;
    
    
    
        let mut i2c = I2cDriver::new(
    
            peripherals.i2c0,
    
            sda,
    
            scl,
    
            &I2cConfig::new().baudrate(100.kHz().into()),
    
        )?;
    
        
    
        log::info!("I2C driver initialized");
    
    
    
        // Scan for I2C devices to verify connection
    
        log::info!("Scanning I2C bus...");
    
        let mut found_devices = false;
    
        for addr in 1..127 {
    
            // Try to write 0 bytes to the address to check for ACK
    
            match i2c.write(addr, &[], 10) {
    
                Ok(_) => {
    
                    log::info!("Found I2C device at address 0x{:02x}", addr);
    
                    found_devices = true;
    
                }
    
                Err(_) => {}
    
            }
    
        }
    
        if !found_devices {
    
            log::warn!("No I2C devices found! Check wiring and pull-up resistors.");
    
        }
    
    
    
        // Create the display interface
    
        let interface = I2CDisplayInterface::new(i2c);
    
    
    
        // Create the display driver
    
        log::info!("Creating SSD1306 display driver");
    
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    
            .into_buffered_graphics_mode();
    
    
    
        // Initialize the display
    
        log::info!("Initializing display...");
    
        display.init().map_err(|e| anyhow::anyhow!("Display init error: {:?}", e))?;
    
    
    
        log::info!("Display initialized");
    
    
    
        // Create lesson manager with default lessons
    
        let lesson_manager = LessonManager::with_defaults();
    
        let lessons = lesson_manager.get_all_lessons_vec();
    
        
    
        log::info!("Lesson manager created with {} lessons", lessons.len());
    
    
    
        // Display welcome message
    
        display.clear(BinaryColor::Off).map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;
    
    
    
        let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    
    
        Text::new("Ceridwen", Point::new(10, 20), text_style)
    
            .draw(&mut display)
    
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        Text::new("ESP32 Ready!", Point::new(10, 35), text_style)
    
            .draw(&mut display)
    
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
            
    
        Text::new("< Prev | Next >", Point::new(10, 50), text_style)
    
            .draw(&mut display)
    
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
        
    
        Text::new("Both: Show/Select", Point::new(10, 60), text_style)
    
            .draw(&mut display)
    
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        display.flush().map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;
    
    
    
        log::info!("Welcome message displayed");
    
        FreeRtos::delay_ms(2000);
    
    
    
            // App State
    
    
    
            #[derive(PartialEq)]
    
    
    
            enum AppMode {
    
    
    
                Browsing,
    
    
    
                Interactive,
    
    
    
                Feedback(bool), // true = correct, false = incorrect
    
    
    
            }
    
    
    
        
    
    
    
            let mut current_lesson_index = 0;
    
    
    
            let mut app_mode = AppMode::Browsing;
    
    
    
            let mut selected_option_index = 0;
    
    
    
            let mut needs_update = true;
    
    
    
        
    
    
    
            // Main loop
    
    
    
            loop {
    
    
    
                let left_pressed = btn_left.is_low();
    
    
    
                let right_pressed = btn_right.is_low();
    
    
    
        
    
    
    
                if left_pressed || right_pressed {
    
    
    
                    // Debounce delay
    
    
    
                    FreeRtos::delay_ms(50);
    
    
    
                    
    
    
    
                    // Re-read after debounce
    
    
    
                    let left_stable = btn_left.is_low();
    
    
    
                    let right_stable = btn_right.is_low();
    
    
    
                    
    
    
    
                    if left_stable && right_stable {
    
    
    
                        // Action: Both Pressed
    
    
    
                        match app_mode {
    
    
    
                            AppMode::Browsing => {
    
    
    
                                // Enter Interactive Mode
    
    
    
                                app_mode = AppMode::Interactive;
    
    
    
                                selected_option_index = 0;
    
    
    
                                log::info!("Mode: Browsing -> Interactive");
    
    
    
                            }
    
    
    
                            AppMode::Interactive => {
    
    
    
                                // Check Answer
    
    
    
                                if let Some(lesson) = lessons.get(current_lesson_index) {
    
    
    
                                    let is_correct = if lesson.lesson_type == LessonType::Subitizing {
    
    
    
                                        let selected_val = lesson.dice_options[selected_option_index];
    
    
    
                                        selected_val == lesson.answer
    
    
    
                                    } else {
    
    
    
                                        // For non-subitizing, just showing answer was the old behavior
    
    
    
                                        // For now, treat "Both" as showing answer/pass
    
    
    
                                        true
    
    
    
                                    };
    
    
    
                                    
    
    
    
                                    app_mode = AppMode::Feedback(is_correct);
    
    
    
                                    log::info!("Mode: Interactive -> Feedback({})", is_correct);
    
    
    
                                }
    
    
    
                            }
    
    
    
                            AppMode::Feedback(_) => {
    
    
    
                                // Return to Browsing (Next Lesson)
    
    
    
                                app_mode = AppMode::Browsing;
    
    
    
                                current_lesson_index = (current_lesson_index + 1) % lessons.len();
    
    
    
                                log::info!("Mode: Feedback -> Browsing (Next)");
    
    
    
                            }
    
    
    
                        }
    
    
    
                        needs_update = true;
    
    
    
                    } else if left_stable {
    
    
    
                        // Action: Left Pressed
    
    
    
                        match app_mode {
    
    
    
                            AppMode::Browsing => {
    
    
    
                                if current_lesson_index > 0 {
    
    
    
                                    current_lesson_index -= 1;
    
    
    
                                } else {
    
    
    
                                    current_lesson_index = lessons.len() - 1;
    
    
    
                                }
    
    
    
                                log::info!("Browsing: Prev Lesson -> {}", current_lesson_index);
    
    
    
                            }
    
    
    
                            AppMode::Interactive => {
    
    
    
                                 if let Some(lesson) = lessons.get(current_lesson_index) {
    
    
    
                                    if lesson.dice_options_count > 0 {
    
    
    
                                        if selected_option_index > 0 {
    
    
    
                                            selected_option_index -= 1;
    
    
    
                                        } else {
    
    
    
                                            selected_option_index = (lesson.dice_options_count - 1) as usize;
    
    
    
                                        }
    
    
    
                                        log::info!("Interactive: Selection -> {}", selected_option_index);
    
    
    
                                    }
    
    
    
                                 }
    
    
    
                            }
    
    
    
                            AppMode::Feedback(_) => {
    
    
    
                                // Optional: Allow navigating back/retry? For now do nothing
    
    
    
                            }
    
    
    
                        }
    
    
    
                        needs_update = true;
    
    
    
                    } else if right_stable {
    
    
    
                        // Action: Right Pressed
    
    
    
                        match app_mode {
    
    
    
                            AppMode::Browsing => {
    
    
    
                                current_lesson_index = (current_lesson_index + 1) % lessons.len();
    
    
    
                                log::info!("Browsing: Next Lesson -> {}", current_lesson_index);
    
    
    
                            }
    
    
    
                            AppMode::Interactive => {
    
    
    
                                if let Some(lesson) = lessons.get(current_lesson_index) {
    
    
    
                                    if lesson.dice_options_count > 0 {
    
    
    
                                        selected_option_index = (selected_option_index + 1) % (lesson.dice_options_count as usize);
    
    
    
                                        log::info!("Interactive: Selection -> {}", selected_option_index);
    
    
    
                                    }
    
    
    
                                }
    
    
    
                            }
    
    
    
                            AppMode::Feedback(_) => {
    
    
    
                                 // Do nothing
    
    
    
                            }
    
    
    
                        }
    
    
    
                        needs_update = true;
    
    
    
                    }
    
    
    
                    
    
    
    
                    // Wait for release
    
    
    
                    while btn_left.is_low() || btn_right.is_low() {
    
    
    
                        FreeRtos::delay_ms(20);
    
    
    
                    }
    
    
    
                }
    
    
    
        
    
    
    
                        if needs_update {
    
    
    
        
    
    
    
                            let res = (|| -> anyhow::Result<()> {
    
    
    
        
    
    
    
                                display.clear(BinaryColor::Off).map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;
    
    
    
        
    
    
    
                                
    
    
    
        
    
    
    
                                if let Some(lesson) = lessons.get(current_lesson_index) {
    
    
    
        
    
    
    
                                    // Header
    
    
    
        
    
    
    
                                    let mode_str = match app_mode {
    
    
    
        
    
    
    
                                        AppMode::Browsing => "Browse",
    
    
    
        
    
    
    
                                        AppMode::Interactive => "Solve",
    
    
    
        
    
    
    
                                        AppMode::Feedback(_) => "Result",
    
    
    
        
    
    
    
                                    };
    
    
    
        
    
    
    
                                    let header = format!("{} {}/{}", mode_str, current_lesson_index + 1, lessons.len());
    
    
    
        
    
    
    
                                    Text::new(&header, Point::new(0, 10), text_style)
    
    
    
        
    
    
    
                                        .draw(&mut display)
    
    
    
        
    
    
    
                                        .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                    
    
    
    
        
    
    
    
                                    match app_mode {
    
    
    
        
    
    
    
                                        AppMode::Browsing => {
    
    
    
        
    
    
    
                                             // Show Question Preview
    
    
    
        
    
    
    
                                            Text::new("Question:", Point::new(0, 25), text_style)
    
    
    
        
    
    
    
                                                .draw(&mut display)
    
    
    
        
    
    
    
                                                .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            
    
    
    
        
    
    
    
                                            let q = &lesson.question;
    
    
    
        
    
    
    
                                            let max_chars = 21;
    
    
    
        
    
    
    
                                            if q.len() > max_chars {
    
    
    
        
    
    
    
                                                 Text::new(&q[..max_chars], Point::new(0, 38), text_style)
    
    
    
        
    
    
    
                                                    .draw(&mut display)
    
    
    
        
    
    
    
                                                    .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                                 let end = q.len().min(max_chars * 2);
    
    
    
        
    
    
    
                                                 Text::new(&q[max_chars..end], Point::new(0, 48), text_style)
    
    
    
        
    
    
    
                                                    .draw(&mut display)
    
    
    
        
    
    
    
                                                    .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            } else {
    
    
    
        
    
    
    
                                                Text::new(q, Point::new(0, 38), text_style)
    
    
    
        
    
    
    
                                                    .draw(&mut display)
    
    
    
        
    
    
    
                                                    .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            }
    
    
    
        
    
    
    
                                            
    
    
    
        
    
    
    
                                            Text::new("Both -> Start", Point::new(10, 60), text_style)
    
    
    
        
    
    
    
                                                .draw(&mut display)
    
    
    
        
    
    
    
                                                .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                        }
    
    
    
        
    
    
    
                                        AppMode::Interactive => {
    
    
    
        
    
    
    
                                            // Render Options
    
    
    
        
    
    
    
                                             if lesson.lesson_type == LessonType::Subitizing && lesson.dice_options_count > 0 {
    
    
    
        
    
    
    
                                                let start_x = 10;
    
    
    
        
    
    
    
                                                let y = 30;
    
    
    
        
    
    
    
                                                let spacing = 25;
    
    
    
        
    
    
    
                                                
    
    
    
        
    
    
    
                                                for i in 0..lesson.dice_options_count as usize {
    
    
    
        
    
    
    
                                                    let val = lesson.dice_options[i];
    
    
    
        
    
    
    
                                                    let x = start_x + (i as i32 * spacing);
    
    
    
        
    
    
    
                                                    
    
    
    
        
    
    
    
                                                    draw_dice(&mut display, Point::new(x, y), val)
    
    
    
        
    
    
    
                                                        .map_err(|e| anyhow::anyhow!("Draw dice error: {:?}", e))?;
    
    
    
        
    
    
    
                                                    
    
    
    
        
    
    
    
                                                    // Draw cursor if selected
    
    
    
        
    
    
    
                                                    if i == selected_option_index {
    
    
    
        
    
    
    
                                                        // Underline
    
    
    
        
    
    
    
                                                        Rectangle::new(Point::new(x, y + 22), Size::new(20, 2))
    
    
    
        
    
    
    
                                                            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    
    
    
        
    
    
    
                                                            .draw(&mut display)
    
    
    
        
    
    
    
                                                            .map_err(|e| anyhow::anyhow!("Draw cursor error: {:?}", e))?;
    
    
    
        
    
    
    
                                                    }
    
    
    
        
    
    
    
                                                }
    
    
    
        
    
    
    
                                                
    
    
    
        
    
    
    
                                                // Show Target hint?
    
    
    
        
    
    
    
                                                // The question usually says "Select 5", so just showing the question is good
    
    
    
        
    
    
    
                                                let q = &lesson.question;
    
    
    
        
    
    
    
                                                 Text::new(q, Point::new(0, 20), text_style)
    
    
    
        
    
    
    
                                                    .draw(&mut display)
    
    
    
        
    
    
    
                                                    .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            } else {
    
    
    
        
    
    
    
                                                // Fallback for non-subitizing
    
    
    
        
    
    
    
                                                 Text::new("No interactive view", Point::new(0, 30), text_style)
    
    
    
        
    
    
    
                                                    .draw(&mut display)
    
    
    
        
    
    
    
                                                    .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            }
    
    
    
        
    
    
    
                                        }
    
    
    
        
    
    
    
                                        AppMode::Feedback(correct) => {
    
    
    
        
    
    
    
                                            let msg = if correct { "Correct! :)" } else { "Try Again :(" };
    
    
    
        
    
    
    
                                            Text::new(msg, Point::new(30, 35), text_style)
    
    
    
        
    
    
    
                                                 .draw(&mut display)
    
    
    
        
    
    
    
                                                 .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                            
    
    
    
        
    
    
    
                                            Text::new("Both -> Next", Point::new(20, 55), text_style)
    
    
    
        
    
    
    
                                                .draw(&mut display)
    
    
    
        
    
    
    
                                                .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
    
    
    
        
    
    
    
                                        }
    
    
    
        
    
    
    
                                    }
    
    
    
        
    
    
    
                                }
    
    
    
        
    
    
    
                                
    
    
    
        
    
    
    
                                display.flush().map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;
    
    
    
        
    
    
    
                                Ok(())
    
    
    
        
    
    
    
                            })();
    
    
    
        
    
    
    
                
    
    
    
        
    
    
    
                            if let Err(e) = res {
    
    
    
        
    
    
    
                                log::error!("Display update failed: {:?}", e);
    
    
    
        
    
    
    
                            } else {
    
    
    
        
    
    
    
                                // Only reset flag if successful? Or always reset to avoid stuck loop?
    
    
    
        
    
    
    
                                // Better to reset and retry next interaction
    
    
    
        
    
    
    
                                needs_update = false;
    
    
    
        
    
    
    
                            }
    
    
    
        
    
    
    
                        }
    
    
    
        
    
    
    
                FreeRtos::delay_ms(50); // Idle poll rate
    
    
    
            }}
