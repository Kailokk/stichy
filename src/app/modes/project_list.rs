use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::{String, Vec};

use crate::{
    app::{peripheral_controller::PeripheralController, AppError},
    data::{Page, UserData, DEFAULT_NAME, DEFAULT_NAME_CHARACTER, PAGE_COUNT},
    display::DisplayController,
    input::INPUT_CHANNEL,
    utility::{string_as_bytes, wrapped_deincrement, wrapped_increment},
};

// static MENU_ORDER: [MenuOption; 3] = [
//     MenuOption::RowCounting,
//     MenuOption::ProjectSelect,
//     MenuOption::NameCurrentProject,
// ];
// enum MenuOption {
//     ProjectSelect,
//     NameCurrentProject,
//     RowCounting,
// }

#[derive(PartialEq, Eq, defmt::Format)]
pub enum ProjectList {
    Continue,
    MoveToCounting,
    MoveToName,
    MoveToError(AppError),
}

impl<'a> ProjectList {
    pub async fn run<T: Instance>(
        controllers: &mut PeripheralController<'a, T>,
        user_data: &mut UserData,
    ) -> Self {
        defmt::debug!("Entering Menu State");
        controllers.display_controller.clear_display();
        controllers.display_controller.return_home();
        controllers
            .display_controller
            .set_backlight_color(116, 241, 9);

        Timer::after_millis(3).await;
        controllers
            .display_controller
            .write_character_at_coord(0b00111100, (14, 1));
        controllers
            .display_controller
            .write_character_at_coord(0b00111110, (15, 1));
        controllers
            .display_controller
            .write_character_at_coord(0b01111110, (0, 0)); //cursor

        let mut pages: Vec<Page, PAGE_COUNT> = user_data
            .all_pages()
            .iter()
            .copied()
            .filter(|page| !(page.name == DEFAULT_NAME))
            .collect();
        //TODO: Fix this idiotic piece of code
        if let Err(_) = pages.push(Page {
            name: (string_as_bytes("New       ")),
            row_count: 0,
        }) {
            defmt::debug!("Contents:");
            for page in pages.iter().enumerate() {
                defmt::debug!("{}: {}", page.0, page.1);
            }
            return Self::MoveToError(AppError::CouldNotAddNew);
        }
        let mut current_menu_option = 0;
        let mut next_state = Self::Continue;

        defmt::debug!("Entering Menu loop");
        while next_state == Self::Continue {
            Self::write_menu_option(
                current_menu_option,
                pages.as_slice(),
                &mut controllers.display_controller,
            )
            .await;

            select_biased! {
                button = INPUT_CHANNEL.receive().fuse() => {
                    match button{
                        crate::input::InputIdentifier::ButtonAPress | crate::input::InputIdentifier::ButtonALongPress | crate::input::InputIdentifier::ButtonADoublePress => {
                            if pages[current_menu_option].name == string_as_bytes("New       "){
                                next_state = Self::MoveToName
                            }else{
                            next_state = Self::MoveToCounting}
                        },
                        crate::input::InputIdentifier::ButtonBPress => current_menu_option = wrapped_deincrement(current_menu_option, 0, pages.len() -1) ,//wrap around
                        crate::input::InputIdentifier::ButtonCPress => current_menu_option = wrapped_increment(current_menu_option, 0, pages.len() -1),
                    }
                }
                //TODO: Find better wait condition
                _ = Timer::after_secs(500).fuse() => {}
            }
        }
        user_data.set_current_page(current_menu_option);
        if let Err(error) = controllers.memory_controller.save_state(user_data) {
            return Self::MoveToError(error.into());
        }
        next_state
    }

    async fn write_menu_option<T: Instance>(
        current_menu_option: usize,
        options: &[Page],
        display_controller: &mut DisplayController<'a, T>,
    ) {
        Self::clear_previous_menu_text(display_controller);
        display_controller.move_cursor_to_coord((1, 0));
        Self::write_option_text(&options[current_menu_option], display_controller).await;
        display_controller.move_cursor_to_coord((0, 1));
        Self::write_option_text(
            &options[wrapped_increment(current_menu_option, 0, options.len() - 1)],
            display_controller,
        )
        .await;
    }

    async fn write_option_text<T: Instance>(
        option: &Page,
        display_controller: &mut DisplayController<'a, T>,
    ) {
        display_controller
            .write_multiple_characters(&option.name)
            .await;
    }

    fn clear_previous_menu_text<T: Instance>(display_controller: &mut DisplayController<'a, T>) {
        display_controller.clear_display_section(1, 13, 0);
        display_controller.clear_display_section(0, 13, 1);
    }
}
