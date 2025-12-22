#![no_main]
#![no_std]

mod app;
mod display;
mod input;
mod memory_controller;
mod data;

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::{self as hal, clocks};

use crate::app::App;

use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut app = App::init(&spawner, p).await;

    // p.FLASH.

    app.run().await;
}

// // This isn't needed, but it's recomended to have these minimal entries.
// #[link_section = ".bi_entries"]
// #[used]
// pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
//     embassy_rp::binary_info::rp_program_name!(c"Lcd Example"),
//     embassy_rp::binary_info::rp_program_description!(c"This example tests an lcd"),
//     embassy_rp::binary_info::rp_cargo_version!(),
//     embassy_rp::binary_info::rp_program_build_attribute!(),
// ];
