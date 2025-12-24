use embassy_rp::i2c::Instance;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};
use heapless::String;

use crate::{
    app::{AppError, peripheral_controller::PeripheralController},
    data::UserData,
    display::DisplayController,
    input::INPUT_CHANNEL,
    utility::{string_as_bytes, wrapped_deincrement, wrapped_increment},
};

static MENU_ORDER: [MenuOption; 4] = [
    MenuOption::RowCounting,
    MenuOption::StitchCounting,
    MenuOption::ProjectSelect,
    MenuOption::NameCurrentProject,

];
enum MenuOption {
    ProjectSelect,
    NameCurrentProject,
    RowCounting,
    StitchCounting,
}

#[derive(PartialEq, Eq, defmt::Format)]
pub enum Menu {
    Continue,
    MoveToRename,
    MoveToCounting,
    MoveToProjectSelect,
    MoveToStitchCounting,
    MoveToError(AppError),
}

impl<'a> Menu {
    pub async fn run<T: Instance>(
        controllers: &mut PeripheralController<'a, T>,
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
        let mut current_menu_option = 0;
        let mut next_state = Self::Continue;

        defmt::debug!("Entering Menu loop");
        while next_state == Self::Continue {
            Self::write_menu_option(current_menu_option, &mut controllers.display_controller).await;

            select_biased! {
                button = INPUT_CHANNEL.receive().fuse() => {
                    match button{
                        crate::input::InputIdentifier::ButtonAPress| crate::input::InputIdentifier::ButtonALongPress | crate::input::InputIdentifier::ButtonADoublePress => next_state = Self::enter_state(current_menu_option),
                        crate::input::InputIdentifier::ButtonBPress => current_menu_option = wrapped_deincrement(current_menu_option, 0, MENU_ORDER.len() - 1) ,//wrap around
                        crate::input::InputIdentifier::ButtonCPress => current_menu_option = wrapped_increment(current_menu_option, 0, MENU_ORDER.len() - 1),
                    }
                }
                //TODO: Find better wait condition
                _ = Timer::after_secs(500).fuse() => {}
            }
        }

        next_state
    }

    const NAME_PROJECT: [u8; 12] = string_as_bytes("Name Project");
    const PROJECTS: [u8; 11] = string_as_bytes("Project Sel");
    const COUNT_ROWS: [u8; 10] = string_as_bytes("Count Rows");
    const COUNT_STITCHES: [u8; 11] = string_as_bytes("Count Stchs");

    async fn write_menu_option<T: Instance>(
        current_menu_option: usize,
        display_controller: &mut DisplayController<'a, T>,
    ) {
        Self::clear_previous_menu_text(display_controller);
        display_controller.move_cursor_to_coord((1, 0));
        Self::write_option_text(&MENU_ORDER[current_menu_option], display_controller).await;
        display_controller.move_cursor_to_coord((0, 1));
        Self::write_option_text(
            &MENU_ORDER[wrapped_increment(current_menu_option, 0, MENU_ORDER.len() - 1)],
            display_controller,
        )
        .await;
    }

    async fn write_option_text<T: Instance>(
        option: &MenuOption,
        display_controller: &mut DisplayController<'a, T>,
    ) {
        match option {
            MenuOption::ProjectSelect => {
                display_controller
                    .write_multiple_characters(&Self::PROJECTS)
                    .await;
            }
            MenuOption::NameCurrentProject => {
                display_controller
                    .write_multiple_characters(&Self::NAME_PROJECT)
                    .await;
            }
            MenuOption::RowCounting => {
                display_controller
                    .write_multiple_characters(&Self::COUNT_ROWS)
                    .await
            }
            MenuOption::StitchCounting => display_controller
                    .write_multiple_characters(&Self::COUNT_STITCHES)
                    .await,
        }
    }

    fn clear_previous_menu_text<T: Instance>(display_controller: &mut DisplayController<'a, T>) {
        display_controller.clear_display_section(1, 13, 0);
        display_controller.clear_display_section(0, 13, 1);
    }

    fn enter_state(current_menu_option: usize) -> Self {
        match MENU_ORDER[current_menu_option] {
            MenuOption::ProjectSelect => Self::MoveToProjectSelect,
            MenuOption::NameCurrentProject => Self::MoveToRename,
            MenuOption::RowCounting => Self::MoveToCounting,
            MenuOption::StitchCounting => Self::MoveToStitchCounting,
        }
    }
}
