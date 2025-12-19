#![no_std]
#![no_main]

mod counter;
mod display;

use defmt::debug;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::{self as hal};
use embassy_time::Timer;

use crate::display::command::DisplayCommand;
use crate::display::driver::DisplayDriver;

use {defmt_rtt as _, panic_probe as _};

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    debug!("Initiating startup sequence");
    Timer::after_millis(50).await;
    let mut display_driver = DisplayDriver::new_i2c_driver(p.I2C0, p.PIN_4, p.PIN_5);

    debug!("Display driver initiated");
    // display_driver.write(0x7C).await;
    display_driver
        .write(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true,
                display_font_is_short: true,
            }
            .to_byte()
            .byte,
        )
        .await;
    Timer::after_millis(10).await;
    debug!("First function set");
    display_driver
        .write(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true ,
                display_font_is_short: true,
            }
            .to_byte()
            .byte,
        )
        .await;
    Timer::after_millis(10).await;
    debug!("Second function set");
    display_driver
        .write(
            DisplayCommand::FunctionSet {
                display_is_two_lines: true,
                display_font_is_short: true,
            }
            .to_byte()
            .byte,
        )
        .await;
    Timer::after_millis(10).await;
    debug!("Third, settings display");
    display_driver
        .write(
            DisplayCommand::DisplayOnOffControl {
                display_on: true,
                cursor_on: true,
                cursor_blinking: true,
            }
            .to_byte()
            .byte,
        )
        .await;
    debug!("Third, settings display");
    display_driver
        .write(DisplayCommand::ReturnHome.to_byte().byte)
        .await;
    debug!("startup complete");

    loop {
        display_driver.write_char(0b01000001).await;
        Timer::after_millis(5).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Lcd Example"),
    embassy_rp::binary_info::rp_program_description!(c"This example tests an lcd"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
