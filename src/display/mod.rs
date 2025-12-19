use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

pub use crate::display::controller::DisplayController;

pub mod command;
mod controller;
pub mod driver;

pub type CommandByte = u8;

const fn set_bit_if_true(value: bool, bit_to_set: u8, byte: &mut u8) {
    if value {
        *byte |= bit_to_set
    }
}