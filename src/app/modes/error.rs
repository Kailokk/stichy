use embassy_futures::join;
use embassy_futures::join::join;
use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::{
    app::{app::AppState, app_dependencies::PeripheralControllers, AppError},
    data::UserData,
    display::DisplayController,
    memory_controller::{self, MemoryIOError},
};

pub enum Error {
    Recoverable,
    Unrecoverable,
}

impl<'a> Error {
    pub async fn display_recoverable_error<T:Instance>(
        error: &AppError,
        peripheral_controller: &mut PeripheralControllers<'a, T>,
    ) {
        peripheral_controller
            .display_controller
            .set_backlight_color(255, 0, 0);
        //TODO: Display error message for a time
        defmt::error!("Recoverable error hit: {}", error);
    }

    pub async fn display_unrecoverable_error<T:Instance>(
        error: &AppError,
        peripheral_controller: &mut PeripheralControllers<'a, T>,
    ) {
        peripheral_controller
            .display_controller
            .set_backlight_color(255, 0, 0);
        defmt::error!("Unrecoverable error hit: {}", error);
        loop {
            Timer::after_secs(30).await;
            // restart device
        }
    }
}
