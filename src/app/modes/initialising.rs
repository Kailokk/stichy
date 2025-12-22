use embassy_futures::join;
use embassy_futures::join::join;
use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::{
    app::{app_dependencies::PeripheralControllers, AppError},
    data::UserData,
    display::DisplayController,
    memory_controller::{self, MemoryIOError},
};

pub enum Initializing {
    MoveToCounting,
    MoveToError(AppError),
}

impl<'a> Initializing {
    pub async fn flash_display<T: Instance>(display_controller: &mut DisplayController<'a, T>) {
        display_controller.set_backlight_color(0, 0, 255);
        Timer::after_secs(1).await;
        display_controller.set_backlight_color(0, 255, 0);
        Timer::after_secs(1).await;
        display_controller.set_backlight_color(255, 0, 0);
        Timer::after_secs(1).await;
    }
}
