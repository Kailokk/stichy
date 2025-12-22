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
use embassy_rp::{
    bind_interrupts,
    flash::Flash,
    i2c::{self, Config, Instance, InterruptHandler},
    peripherals::I2C0,
    Peripherals,
};
use embassy_time::Timer;
use futures::{select_biased, FutureExt};

use crate::{
    display::DisplayController,
    input::{INPUT_CHANNEL, init_input},
    memory_controller::{MemoryController},
};



use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

//TODO: Understand this better and find out how i can move it out of here
bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

defmt::debug!("Initiating startup sequence");

        match init_input(
            &spawner,
            p.PIN_18.into(),
            p.PIN_17.into(),
            p.PIN_16.into(),
        ) {
            Ok(_) => (),
            //TODO: handle error
            Err(error) => defmt::error!("Error initializing input peripherals: {}", error),
        }

        let mut config = Config::default();

        config.frequency = 400_000; //400khz

        let i2c_peripheral = i2c::I2c::new_async(
            p.I2C0,
            p.PIN_5,
            p.PIN_4,
            Irqs,
            config,
        );

        Timer::after_millis(50).await;

        let display_controller = DisplayController::init(i2c_peripheral).await;

        let flash = Flash::new(p.FLASH, p.DMA_CH0);

        let memory_controller = MemoryController::init(flash);

    
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
