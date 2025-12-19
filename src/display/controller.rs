use embassy_rp::peripherals::{I2C0, PIN_4, PIN_5};

use crate::display::driver::DisplayDriver;

type DisplayCoordinate = (u8, u8);

pub struct DisplayController<'a> {
    driver: DisplayDriver<'a>,
    command_buffer: [Command; 32],
}

impl DisplayController {
    pub fn new_display(peripheral: I2C0, sda_pin: PIN_4, scl_pin: PIN_5) {
        let driver = DisplayDriver::new_i2c_driver(peripheral, sda_pin, scl_pin);

        Self{

        }
    }

    pub fn clear_display() {}

    //this could maybe be abstracted
    pub fn return_home() {}

    pub fn configure_display() {}
}
