use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::{
    app::AppError, display::DisplayController
};

#[derive(defmt::Format)]
pub enum Initializing {
    MoveToCounting,
    MoveToEnterName,
    MoveToError(AppError),
}

impl<'a> Initializing {
    pub async fn flash_display<T: Instance>(display_controller: &mut DisplayController<'a, T>) {
        display_controller.set_backlight_color(0, 0, 255);
        Timer::after_millis(500).await;
        display_controller.set_backlight_color(0, 255, 0);
        Timer::after_millis(500).await;
        display_controller.set_backlight_color(255, 0, 0);
        Timer::after_millis(500).await;

    }
}
