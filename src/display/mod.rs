use bbqueue::BBBuffer;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

mod controller;
mod driver;
mod command;

pub type CommandByte = u8;


static DISPLAY_COMMAND_BUFFER: Channel<ThreadModeRawMutex, CommandByte, 32> = Channel::new();