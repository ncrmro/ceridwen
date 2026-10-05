use ceridwen_core::LessonManager;
use ceridwen_esp32::{
    renderer,
    session::{Action, Session},
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{PinDriver, Pull},
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
    display
        .init()
        .map_err(|e| anyhow::anyhow!("Display init error: {:?}", e))?;

    log::info!("Display initialized");

    // Create lesson manager with default lessons
    let lesson_manager = LessonManager::with_defaults();
    let lessons = lesson_manager.get_all_lessons_vec();

    log::info!("Lesson manager created with {} lessons", lessons.len());

    // Display welcome message
    display
        .clear(BinaryColor::Off)
        .map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;

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

    display
        .flush()
        .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

    log::info!("Welcome message displayed");

    FreeRtos::delay_ms(2000);

    // App State
    let mut session = Session::default();
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

            let action = match (left_stable, right_stable) {
                (true, true) => Some(Action::Both),
                (true, false) => Some(Action::Left),
                (false, true) => Some(Action::Right),
                _ => None,
            };
            if let Some(action) = action {
                session.apply(action, &lessons);
                needs_update = true;
            }

            // Wait for release
            while btn_left.is_low() || btn_right.is_low() {
                FreeRtos::delay_ms(20);
            }
        }

        if needs_update {
            let res = (|| -> anyhow::Result<()> {
                display
                    .clear(BinaryColor::Off)
                    .map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;

                if let Some(lesson) = lessons.get(session.lesson_index) {
                    renderer::draw_screen(
                        &mut display,
                        session.mode,
                        lesson,
                        session.lesson_index,
                        lessons.len(),
                        session.selection,
                    )
                    .map_err(|e| anyhow::anyhow!("Render error: {:?}", e))?;
                }

                display
                    .flush()
                    .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;
                Ok(())
            })();

            if let Err(e) = res {
                log::error!("Display update failed: {:?}", e);
            } else {
                needs_update = false;
            }
        }

        FreeRtos::delay_ms(50); // Idle poll rate
    }
}
