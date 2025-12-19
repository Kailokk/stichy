pub use crate::display::controller::DisplayController;

mod command;
mod controller;
mod driver;


const fn set_bit_if_true(value: bool, bit_to_set: u8, byte: &mut u8) {
    if value {
        *byte |= bit_to_set
    }
}
