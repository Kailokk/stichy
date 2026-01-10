use embassy_futures::select;
use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::String;

use crate::{
    app::{peripheral_controller::PeripheralController, AppError},
    data::{UserData, DEFAULT_NAME_CHARACTER},
    display::DisplayController,
    input::{ButtonIdentifier, PressKind, INPUT_CHANNEL},
    utility::{
        string_as_bytes, wrapped_deincrement, wrapped_deincrement_u8, wrapped_increment,
        wrapped_increment_u8,
    },
};

static MIN_CHARACTER: u8 = 0b00100000;
static MAX_CHARACTER: u8 = 0b01011010;
static START_CHARACTER: u8 = 0b01000001; // A

#[derive(PartialEq, Eq, defmt::Format)]
pub enum Rename {
    Continue,
    MoveToCounting,
    MoveToError(AppError),
}

impl<'a> Rename {
    pub async fn run<T: Instance>(
        controllers: &mut PeripheralController<'a, T>,
        user_data: &mut UserData,
    ) -> Self {
        defmt::debug!("Entering Menu State");
        controllers.display_controller.set_cursor(true, true);
        controllers.display_controller.clear_display();
        controllers.display_controller.return_home();
        //TODO: Find cause of weird timing error
        Timer::after_millis(33).await;
        controllers
            .display_controller
            .set_backlight_color(152, 9, 241);

        Timer::after_millis(33).await;
        controllers
            .display_controller
            .write_multiple_characters(&string_as_bytes::<16>("Enter Proj Name:"))
            .await;
        Timer::after_millis(33).await;
        controllers.display_controller.move_cursor_to_coord((10, 1));
        controllers
            .display_controller
            .write_multiple_characters(&[0b0111_1111; 6])
            .await;
        controllers.display_controller.move_cursor_to_coord((0, 1));

        let name_value: &mut [u8; 10] = &mut user_data.current_page_mut().name;
        let mut character: u8 = START_CHARACTER;
        let mut index = 0;
        Self::update_display(character, index as u8, &mut controllers.display_controller).await;

        let mut next_state = Self::Continue;

        defmt::debug!("Entering Menu loop");
        while next_state == Self::Continue {
            let press = match select::select(Timer::after_secs(5), INPUT_CHANNEL.receive()).await {
                select::Either::First(_) => {
                    //TODO: Implement auto save if changes have been made
                    continue;
                }
                select::Either::Second(press) => press,
            };
            match press.button_identifier {
                crate::input::ButtonIdentifier::ButtonA => {
                    match press.press_kind {
                        PressKind::Press => {
                            // TODO: Implement go back and save early features
                            if index >= 9 {
                                next_state = Self::MoveToCounting;
                                break;
                            }
                            name_value[index] = character;
                            index += 1;
                            Self::update_display(
                                character,
                                index as u8,
                                &mut controllers.display_controller,
                            )
                            .await;
                        }
                        PressKind::LongPress => {
                            next_state = Self::MoveToCounting;
                            break;
                        }
                        PressKind::DoublePress => {
                            name_value[index] = DEFAULT_NAME_CHARACTER;
                            Self::update_display(
                                0b00100000,
                                index as u8,
                                &mut controllers.display_controller,
                            )
                            .await;
                            index = index.checked_sub(1).unwrap_or(0);
                            character = name_value[index];
                            Self::update_display(
                                character,
                                index as u8,
                                &mut controllers.display_controller,
                            )
                            .await;
                        }
                    }
                }
                crate::input::ButtonIdentifier::ButtonB => {
                    character = wrapped_deincrement_u8(character, MIN_CHARACTER, MAX_CHARACTER);
                    Self::update_display(
                        character,
                        index as u8,
                        &mut controllers.display_controller,
                    )
                    .await;
                }
                crate::input::ButtonIdentifier::ButtonC => {
                    character = wrapped_increment_u8(character, MIN_CHARACTER, MAX_CHARACTER);
                    Self::update_display(
                        character,
                        index as u8,
                        &mut controllers.display_controller,
                    )
                    .await;
                }
            }
        }

        if let Err(error) = controllers.memory_controller.save_state(user_data) {
            return Self::MoveToError(error.into());
        }

        Timer::after_millis(33).await;
        controllers.display_controller.set_cursor(false, false);

        next_state
    }

    async fn update_display<T: Instance>(
        character: u8,
        index: u8,
        display_controller: &mut DisplayController<'a, T>,
    ) {
        display_controller.write_character_at_coord(character, (index, 1));
        display_controller.move_cursor_to_coord((index, 1));
    }
}
