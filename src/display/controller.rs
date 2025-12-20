use embassy_rp::{
    bind_interrupts,
    i2c::{self, Async, Config, Error, I2c, Instance, InterruptHandler, SclPin, SdaPin},
    peripherals::{I2C0, PIN_4, PIN_5},
    Peri, Peripherals,
};
use embassy_time::Timer;

use crate::display::{command::DisplayCommand, driver::DisplayDriver};

pub struct DisplayController<'a, T:Instance> {
    driver: DisplayDriver<'a,T>,
}

impl<'a, T:Instance> DisplayController<'a, T> {
    pub async fn init(i2c_peripheral: I2c<'a, T, Async>) -> Self {
        let mut display_driver = DisplayDriver::init(i2c_peripheral);
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

        display_driver.write_command(DisplayCommand::ClearDisplay.to_byte());
        display_driver.write_command(DisplayCommand::ReturnHome.to_byte());

        display_driver.init_backlight();

        Self {
            driver: display_driver,
        }
    }

    pub fn set_backlight_color(&mut self, red: u8, green: u8, blue: u8) {
        self.driver.set_backlight_color(red, green, blue);
    }

    pub fn write_character(&mut self, byte: u8) {
        self.driver.write_character(byte);
    }

    pub fn clear_display(&mut self) {
        self.driver
            .write_command(DisplayCommand::ClearDisplay.to_byte());
    }

    //this could maybe be abstracted
    pub fn return_home() {}

    pub fn configure_display() {}
}
