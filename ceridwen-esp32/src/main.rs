use ceridwen_core::{LessonManager, LessonType};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF services
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting Ceridwen ESP32...");

    // Create peripherals
    let peripherals = Peripherals::take()?;

    // Configure I2C for the SSD1306 display
    // ESP32-C3 pins: SDA=GPIO6, SCL=GPIO7
    log::info!("Configuring I2C on GPIO6 (SDA) and GPIO7 (SCL)");
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &I2cConfig::new().baudrate(100.kHz().into()),
    )?;
    
    log::info!("I2C driver initialized");

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

    log::info!("Lesson manager created with {} lessons", lesson_manager.count());

    // Display welcome message
    display.clear(BinaryColor::Off).map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    Text::new("Ceridwen", Point::new(10, 20), text_style)
        .draw(&mut display)
        .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;

    Text::new("ESP32 Ready!", Point::new(10, 35), text_style)
        .draw(&mut display)
        .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;

    display.flush().map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

    log::info!("Welcome message displayed");

    FreeRtos::delay_ms(2000);

    // Display first lesson
    let lessons = lesson_manager.get_by_type(LessonType::Subitizing);
    if let Some(lesson) = lessons.first() {
        display.clear(BinaryColor::Off).map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;

        Text::new("Lesson:", Point::new(5, 10), text_style)
            .draw(&mut display)
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;

        // Display the lesson question (truncated to fit)
        let question = if lesson.question.len() > 20 {
            &lesson.question[..20]
        } else {
            &lesson.question
        };

        Text::new(question, Point::new(5, 25), text_style)
            .draw(&mut display)
            .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;

        // Display dice pattern
        let dice_pattern = lesson.get_dice_pattern();
        if !dice_pattern.is_empty() {
            Text::new(&dice_pattern, Point::new(5, 40), text_style)
                .draw(&mut display)
                .map_err(|e| anyhow::anyhow!("Draw error: {:?}", e))?;
        }

        display.flush().map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

        log::info!("First lesson displayed");
    }

    // Main loop
    loop {
        FreeRtos::delay_ms(1000);
    }
}
