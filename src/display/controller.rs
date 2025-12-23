use embassy_rp::i2c::{Async, I2c, Instance};
use embassy_time::Timer;

use crate::display::{command::DisplayCommand, driver::DisplayDriver};

pub struct DisplayController<'a, T: Instance> {
    driver: DisplayDriver<'a, T>,
}

impl<'a, T: Instance> DisplayController<'a, T> {
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
                cursor_on: false,
                cursor_blinking: false,
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

    //TODO: separate this from coordinate mode
    pub fn write_character(&mut self, byte: u8) {
        self.driver.write_character(byte);
    }

    pub async fn write_multiple_characters<const SIZE: usize>(&mut self, bytes: &[u8; SIZE]) {
        self.driver.write_characters(bytes).await;
        Timer::after_millis(3).await
    }

    pub fn move_cursor_to_coord(&mut self, coord: (u8, u8)) {
        let mut ram_address = 0b10000000 + coord.0;
        if coord.1 > 0 {
            ram_address |= 0b01000000;
        }
        self.driver.set_ram_address(ram_address);
    }

    pub fn write_character_at_coord(&mut self, byte: u8, coord: (u8, u8)) {
        let mut ram_address = 0b10000000 + coord.0;
        if coord.1 > 0 {
            ram_address |= 0b01000000;
        }
        self.driver.set_ram_address(ram_address);
        self.driver.write_character(byte);
    }

    pub fn clear_display(&mut self) {
        self.driver
            .write_command(DisplayCommand::ClearDisplay.to_byte());
        self.set_cursor(false, false);
    }

    pub fn clear_display_section(&mut self, from_x: u8, to_x: u8, y: u8) {
        self.move_cursor_to_coord((from_x, y));
        for _ in from_x..=to_x {
            self.driver.write_character(0b11111110);
        }
    }

    pub fn set_writing_shift_mode(&mut self) {
        self.driver.write_command(
            DisplayCommand::SetEntryMode {
                cursor_move_rtl: false,
                can_shift: true,
            }
            .to_byte(),
        );
    }

    pub fn set_writing_coordinate_mode(&mut self) {
        self.driver.write_command(
            DisplayCommand::SetEntryMode {
                cursor_move_rtl: false,
                can_shift: false,
            }
            .to_byte(),
        );
    }

    pub fn return_home(&mut self) {
        self.move_cursor_to_coord((0, 0));
    }

    pub fn set_cursor(&mut self, active: bool, blinking: bool) {
        self.driver.write_command(
            DisplayCommand::DisplayOnOffControl {
                display_on: true,
                cursor_on: active,
                cursor_blinking: blinking,
            }
            .to_byte(),
        );
    }
}
