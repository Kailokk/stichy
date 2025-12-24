use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::String;

use crate::{
    app::{peripheral_controller::PeripheralController, AppError},
    data::UserData,
    display::DisplayController,
    input::INPUT_CHANNEL,
    utility::string_as_bytes,
};

static STITCHES_WORD: [u8; 9] = string_as_bytes("Stitches:");

#[derive(PartialEq, Eq, defmt::Format)]
pub enum StitchCounting {
    Continue,
    MoveToMenu,
    MoveToError(AppError),
}

impl<'a> StitchCounting {
    pub async fn run<T: Instance>(controllers: &mut PeripheralController<'a, T>) -> Self {
        defmt::debug!("Entering Counting State");
        controllers.display_controller.clear_display();
        controllers.display_controller.return_home();
        controllers
            .display_controller
            .set_backlight_color(0, 0, 255);

        Timer::after_millis(3).await;
        controllers.display_controller.move_cursor_to_coord((0, 0));
        controllers
            .display_controller
            .write_multiple_characters(&STITCHES_WORD)
            .await;

        let mut stitch_count: u16 = 0;
        let mut colors = (0, 0, 150);

        Self::update_display(&mut controllers.display_controller, stitch_count, colors).await;
        let mut next_state = Self::Continue;
        while next_state == Self::Continue {
            select_biased! {
                button = INPUT_CHANNEL.receive().fuse() => {
                    match button{
                        crate::input::InputIdentifier::ButtonAPress => next_state = Self::MoveToMenu,
                        crate::input::InputIdentifier::ButtonBPress => stitch_count = stitch_count.checked_sub(1).unwrap_or(0),
                        crate::input::InputIdentifier::ButtonCPress => stitch_count = stitch_count.checked_add(1).unwrap_or(stitch_count),
                        _ => ()
                    }
                    colors = Self::change_colors(colors, stitch_count);
                    Self::update_display(&mut controllers.display_controller, stitch_count, colors).await;
                }
                //TODO: Find better wait condition
                _ = Timer::after_secs(500).fuse() => {}
            }
        }

        next_state
    }

    async fn update_display<T: Instance>(
        display_controller: &mut DisplayController<'a, T>,
        stitch_count: u16,
        colors: (u8, u8, u8),
    ) {
        let stitch_count_string: String<4> = String::from(stitch_count);

        display_controller.move_cursor_to_coord((0, 1));
        for digit in stitch_count_string.as_bytes() {
            display_controller.write_character(*digit);
        }

        display_controller.set_backlight_color(colors.0, colors.1, colors.2);
    }

    fn change_colors(mut colors: (u8, u8, u8), stitch_count: u16) -> (u8, u8, u8) {
        if stitch_count < 50 {
            colors.2 = colors.2.checked_add(255 / 50).unwrap_or(255);
        } else if stitch_count > 50 && stitch_count < 100 {
            colors.1 = colors.1.checked_add(255 / 50).unwrap_or(255);
            colors.2 = colors.2.checked_sub(255 / 50).unwrap_or(0);
        } else if stitch_count > 100 && stitch_count < 150 {
            colors.0 = colors.0.checked_add(255 / 50).unwrap_or(255);
            colors.1 = colors.1.checked_sub(255 / 50).unwrap_or(0);
        } else {
            colors.0 = colors.0.checked_add(255 / 50).unwrap_or(255);
            colors.1 = colors.1.checked_add(255 / 50).unwrap_or(255);
            colors.2 = colors.2.checked_add(255 / 50).unwrap_or(255);
        }

        colors
    }

    fn increment_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count += 1;
    }

    fn decrement_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count = user_data
            .current_page_mut()
            .row_count
            .checked_sub(1)
            .unwrap_or(0);
    }
}
