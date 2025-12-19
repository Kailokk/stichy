use embassy_rp::peripherals::{I2C0, PIN_4, PIN_5};
use embassy_time::Timer;

use crate::display::{command::DisplayCommand, driver::DisplayDriver};

pub struct DisplayController<'a> {
    driver: DisplayDriver<'a>,
}

impl<'a> DisplayController<'a> {
    pub async fn init(peripheral: I2C0, sda_pin: PIN_4, scl_pin: PIN_5) -> Self {
        let mut display_driver = DisplayDriver::connect(peripheral, sda_pin, scl_pin);
        display_driver.write_command(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true,
                display_font_is_short: true,
            }
            .to_byte(),
        );
        Timer::after_millis(10).await;
        display_driver.write_command(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true,
                display_font_is_short: true,
            }
            .to_byte(),
        );
        Timer::after_millis(10).await;
        display_driver.write_command(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true,
                display_font_is_short: true,
            }
            .to_byte(),
        );
        Timer::after_millis(10).await;
        display_driver.write_command(
            DisplayCommand::DisplayOnOffControl {
                display_on: true,
                cursor_on: true,
                cursor_blinking: true,
            }
            .to_byte(),
        );
        display_driver.write_command(DisplayCommand::ReturnHome.to_byte());

        display_driver.init_backlight();

        Self {
            driver: display_driver,
        }
    }

    pub fn set_backlight_color(&mut self, red:u8,green:u8,blue:u8) {
        self.driver.set_backlight_color(red, green, blue);
    }

    pub fn clear_display() {}

    //this could maybe be abstracted
    pub fn return_home() {}

    pub fn configure_display() {}
}
