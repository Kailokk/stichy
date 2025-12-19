#![no_std]
#![no_main]

mod counter;
mod display;

use defmt::debug;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::{self as hal};
use embassy_time::Timer;

use crate::display::DisplayController;
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
    
    let mut display_controller:DisplayController = DisplayController::init(p.I2C0, p.PIN_4, p.PIN_5).await;

    loop {
        for r in 0..=255{
            display_controller.set_backlight_color(r,0,0);
            Timer::after_millis(10).await;                    
        }

        Timer::after_secs(1).await;
        for r in (0..=255).rev(){
            display_controller.set_backlight_color(r,0,0);
            Timer::after_millis(10).await;                    
        }

        Timer::after_secs(1).await;
        for g in 0..=255{
            display_controller.set_backlight_color(0,g,0);
            Timer::after_millis(10).await;        
        }


        Timer::after_secs(1).await;
        for g in (0..=255).rev(){
            display_controller.set_backlight_color(0,g,0);
            Timer::after_millis(10).await;        
        }

        Timer::after_secs(1).await;
        for b in 0..255{
            display_controller.set_backlight_color(0,0,b);
            Timer::after_millis(10).await;
        }

        Timer::after_secs(1).await;
        for b in (0..=255).rev(){
            display_controller.set_backlight_color(0,0,b);
            Timer::after_millis(10).await;
        }
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
