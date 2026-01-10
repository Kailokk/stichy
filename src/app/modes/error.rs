use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::{
    app::{peripheral_controller::PeripheralController, AppError},
    utility::string_as_bytes,
};

pub async fn display_recoverable_error<'a, T: Instance>(
    error: &AppError,
    peripheral_controller: &mut PeripheralController<'a, T>,
) {
    peripheral_controller
        .display_controller
        .set_backlight_color(255, 0, 0);
    //TODO: Display error message for a time
    defmt::error!("Recoverable error hit: {}", error);
}

pub async fn display_unrecoverable_error<'a, T: Instance>(
    error: &AppError,
    peripheral_controller: &mut PeripheralController<'a, T>,
) {
    peripheral_controller
        .display_controller
        .set_backlight_color(255, 0, 0);
    peripheral_controller.display_controller.clear_display();
    Timer::after_millis(30).await;
    peripheral_controller.display_controller.return_home();
    Timer::after_millis(30).await;
    peripheral_controller
        .display_controller
        .write_multiple_characters(&string_as_bytes::<11>("FATAL ERROR"))
        .await;
    Timer::after_millis(30).await;
    peripheral_controller
        .display_controller
        .move_cursor_to_coord((0, 1));
    Timer::after_millis(30).await;
    peripheral_controller
        .display_controller
        .write_multiple_characters(&string_as_bytes::<13>("PLEASE REBOOT"))
        .await;
    Timer::after_millis(30).await;
    defmt::error!("Unrecoverable error hit: {}", error);
    loop {
        peripheral_controller
            .display_controller
            .set_backlight_color(255, 0, 0);
        Timer::after_secs(1).await;
        peripheral_controller
            .display_controller
            .set_backlight_color(155, 100, 0);
        Timer::after_secs(1).await;
    }
}
