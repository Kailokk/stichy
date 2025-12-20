use embassy_rp::{
    bind_interrupts,
    i2c::{self, Async, Config, Error, I2c, Instance, InterruptHandler, SclPin, SdaPin},
    peripherals::{I2C0, PIN_4, PIN_5},
    Peri, Peripherals,
};
use embassy_time::Timer;

use crate::display::{command::DisplayCommand, driver::DisplayDriver, set_bit_if_true};

pub struct DisplayController<'a, T: Instance> {
    driver: DisplayDriver<'a, T>,
        cursor_position:u8,
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
            cursor_position:0
        }
    }

    pub fn set_backlight_color(&mut self, red: u8, green: u8, blue: u8) {
        self.driver.set_backlight_color(red, green, blue);
    }

    
    //TODO: separate this from coordinate mode
    pub fn write_character(&mut self, byte: u8) {
        self.driver.write_character(byte);
        self.cursor_position +=1;
    }

    pub fn write_character_at_coord(&mut self, byte: u8,coord:(u8,u8)) {
        let mut ram_address = 0b10000000 + coord.0;
        if coord.1 > 0{
            ram_address |= 0b01000000;
        }
        self.driver.set_ram_address(ram_address);
        self.driver.write_character(byte);
        self.cursor_position = (coord.0 * coord.1) + 1;
    }

    pub fn clear_display(&mut self) {
        self.driver
            .write_command(DisplayCommand::ClearDisplay.to_byte());
        self.cursor_position = 0;
    }

    pub fn set_writing_shift_mode(&mut self){
        self.driver.write_command(DisplayCommand::SetEntryMode { cursor_move_rtl: false, can_shift: true }.to_byte());
        
    }

    pub fn set_writing_coordinate_mode(&mut self){
        self.driver.write_command(DisplayCommand::SetEntryMode { cursor_move_rtl: false, can_shift: false }.to_byte());
    }

    pub fn return_home(&mut self) {
        self.driver
            .write_command(DisplayCommand::ReturnHome.to_byte());
    }

}
