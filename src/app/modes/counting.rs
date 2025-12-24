use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::String;

use crate::{
    app::{AppError, peripheral_controller::PeripheralController},
    data::UserData,
    display::DisplayController,
    input::INPUT_CHANNEL, utility::string_as_bytes,
};

static ROWS_WORD: [u8; 5] = string_as_bytes("Rows:");

#[derive(PartialEq, Eq, defmt::Format)]
pub enum Counting {
    Continue,
    MoveToMenu,
    MoveToError(AppError),
}

impl<'a> Counting {
    pub async fn run<T: Instance>(
        controllers: &mut PeripheralController<'a, T>,
        user_data: &mut UserData,
    ) -> Self {
        defmt::debug!("Entering Counting State");
        controllers.display_controller.clear_display();
        controllers.display_controller.return_home();
        controllers
            .display_controller
            .set_backlight_color(241, 77, 9);

        Timer::after_millis(3).await;
        controllers.display_controller.move_cursor_to_coord((0, 1));
        controllers.display_controller.write_multiple_characters(&ROWS_WORD).await;
        
        controllers.display_controller.move_cursor_to_coord((0, 0));
        controllers.display_controller.write_multiple_characters(&user_data.current_page().name).await;

        Self::update_display(&mut controllers.display_controller, user_data).await;

        let mut next_state = Self::Continue;
        while next_state == Self::Continue {
            select_biased! {
                button = INPUT_CHANNEL.receive().fuse() => {
                    match button{
                        crate::input::InputIdentifier::ButtonAPress | crate::input::InputIdentifier::ButtonALongPress | crate::input::InputIdentifier::ButtonADoublePress => next_state = Self::MoveToMenu,
                        crate::input::InputIdentifier::ButtonBPress => Self::decrement_count(user_data),
                        crate::input::InputIdentifier::ButtonCPress => Self::increment_count(user_data),
                    }
                    Self::update_display(&mut controllers.display_controller, user_data).await;
                }
                //TODO: Find better wait condition
                _ = Timer::after_secs(500).fuse() => {}
            }
        }

        if let Err(error) = controllers.memory_controller.save_state(user_data) {
            return Self::MoveToError(error.into());
        }

        next_state
    }

    async fn update_display<T: Instance>(
        display_controller: &mut DisplayController<'a, T>,
        user_data: &mut UserData,
    ) {
        let row_count = user_data.current_page().row_count;
        let row_count_string: String<4> = String::from(row_count);

        display_controller.move_cursor_to_coord((5, 1));
        for digit in row_count_string.as_bytes() {
            display_controller.write_character(*digit);
        }
    }

    fn increment_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count += 1;
    }

    fn decrement_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count = user_data.current_page_mut().row_count.checked_sub(1).unwrap_or(0);
    }
}
