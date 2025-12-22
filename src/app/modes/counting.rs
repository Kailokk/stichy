use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::{app::{AppError, app_dependencies::PeripheralControllers}, data::UserData, display::DisplayController};

static ROWS_WORD: [u8; 5] = [0b01010010, 0b01101111, 0b01110111, 0b01110011, 0b00111010];

pub enum Counting {
    Continue,
    MoveToError(AppError)
}


impl<'a> Counting {
    pub async fn run<T:Instance>( controllers:&mut PeripheralControllers<'a,T>, user_data:&mut UserData) -> Self {
        controllers.display_controller.clear_display();
        controllers.display_controller.return_home();
        controllers.display_controller.set_backlight_color(0, 255, 0);

        Timer::after_millis(3).await;
        for character in ROWS_WORD {
            controllers.display_controller.write_character(character);
        }

        controllers.display_controller
            .write_character_at_coord(0b00110000, (5, 0));


        let mut exit_loop = false;
        while !exit_loop{
            //TODO: Counting main loop
            exit_loop = true;
        }

        if let Err(error) = controllers.memory_controller.save_state(user_data){
            return Self::MoveToError(error.into());
        }

        Self::Continue

    }

    async fn update_display<T:Instance>(display_controller:&mut DisplayController<'a,T>, user_data:&mut UserData) {

        // let mut digits:[u8;4] = [0,0,0,0];
        // digits[3] = self.controllers.user_data()rows % 10 ;
        // let digit_count = (self.rows /10) + 1;
        

        display_controller
            .write_character_at_coord(0b00110000 + user_data.current_page().row_count, (5, 0));
    }

    async fn increment_count(user_data:&mut UserData) {
        user_data.current_page_mut().row_count += 1;
    }

    async fn decrement_count(user_data:&mut UserData) {
        user_data.current_page_mut().row_count -= 1;
    }

    async fn reset_count(user_data:&mut UserData) {
        user_data.current_page_mut().row_count = 0;
    }
}
