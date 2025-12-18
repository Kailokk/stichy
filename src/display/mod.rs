use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

mod controller;
mod driver;
mod command;

pub type CommandByte = u8;
