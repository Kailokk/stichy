use cortex_m::register::control;
use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::String;

use crate::{
    app::{peripheral_controller::PeripheralController, AppError},
    data::UserData,
    display::DisplayController,
    input::INPUT_CHANNEL,
};

static ROWS_WORD: [u8; 5] = [0b01010010, 0b01101111, 0b01110111, 0b01110011, 0b00111010];

pub enum Counting {
    Continue,
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
            .set_backlight_color(0, 255, 0);

        Timer::after_millis(3).await;
        controllers.display_controller.move_cursor_to_coord((0,1));
        for character in ROWS_WORD {
            controllers.display_controller.write_character(character);
        }

        Self::update_display(&mut controllers.display_controller, user_data).await;

        let mut exit_loop = false;

       while !exit_loop{
            select_biased! {
                button = INPUT_CHANNEL.receive().fuse() => {
                    match button{
                        crate::input::InputIdentifier::ButtonA => exit_loop = true,
                        crate::input::InputIdentifier::ButtonB => Self::decrement_count(user_data),
                        crate::input::InputIdentifier::ButtonC => Self::increment_count(user_data),
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

        Self::Continue
    }

    async fn update_display<T: Instance>(
        display_controller: &mut DisplayController<'a, T>,
        user_data: &mut UserData,
    ) {
        let row_count = user_data.current_page().row_count;
        let test:String<4> = String::from(row_count);

        display_controller.move_cursor_to_coord((5,1));
        for digit in test.as_bytes(){
            display_controller.write_character(*digit);
        }
    }

    fn increment_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count += 1;
    }

    fn decrement_count(user_data: &mut UserData) {
        user_data.current_page_mut().row_count -= 1;
    }
}
