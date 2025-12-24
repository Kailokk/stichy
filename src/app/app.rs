use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    i2c::{Instance, InterruptHandler},
    peripherals::I2C0,
};
use embassy_time::Timer;

use crate::{
    app::{
        modes::{self, Counting, Initializing, Menu, ProjectList, Rename},
        peripheral_controller::PeripheralController,
        AppError,
    },
    data::UserData,
    memory_controller::MemoryIOError,
};

use crate::data::DEFAULT_NAME_CHARACTER;

#[derive(defmt::Format)]
pub enum AppState {
    Initializing(Initializing),
    Counting(Counting),
    Menu(Menu),
    Rename(Rename),
    ProjectList(ProjectList),
}

pub struct App<'a> {
    peripherals: PeripheralController<'a, I2C0>,
    app_state: AppState,
    user_data: UserData,
}

impl<'a> App<'a> {
    pub async fn new(mut peripherals: PeripheralController<'a, I2C0>) -> Self {
        let (_, user_data): ((), Result<UserData, MemoryIOError>) = join(
            Initializing::flash_display(&mut peripherals.display_controller),
            peripherals.memory_controller.read_state(),
        )
        .await;

        let mut initial_state = Initializing::MoveToCounting;
        let user_data = match user_data {
            Ok(data) => {
                if data
                    .all_pages()
                    .iter()
                    .all(|page| page.name.ends_with(&[DEFAULT_NAME_CHARACTER]))
                {
                    initial_state = Initializing::MoveToEnterName;
                }
                data
            }
            Err(error) => {
                defmt::error!("Error reading saved state data: {}", error);
                initial_state = Initializing::MoveToError(error.into());
                Default::default()
            }
        };
        defmt::debug!("Default char: {}", DEFAULT_NAME_CHARACTER);
        defmt::debug!("User Data: {}", user_data);

        Self {
            app_state: AppState::Initializing(initial_state),
            peripherals,
            user_data,
        }
    }

    pub async fn step(&mut self) {
        defmt::debug!("Activating state: {}", self.app_state);
        self.app_state = match &self.app_state {
            AppState::Initializing(state) => match state {
                Initializing::MoveToCounting => AppState::Counting(Counting::Continue),
                Initializing::MoveToError(app_error) => {
                    Self::handle_error(app_error, &mut self.peripherals).await
                }
                Initializing::MoveToEnterName => AppState::Rename(Rename::Continue),
            },
            AppState::Counting(counting_mode) => match counting_mode {
                Counting::Continue => AppState::Counting(
                    Counting::run(&mut self.peripherals, &mut self.user_data).await,
                ),
                Counting::MoveToMenu => AppState::Menu(Menu::Continue),
                Counting::MoveToError(app_error) => {
                    Self::handle_error(app_error, &mut self.peripherals).await
                }
            },
            AppState::Menu(menu) => match menu {
                Menu::Continue => AppState::Menu(Menu::run(&mut self.peripherals).await),
                Menu::MoveToCounting => AppState::Counting(Counting::Continue),
                Menu::MoveToRename => AppState::Rename(Rename::Continue),
                Menu::MoveToError(app_error) => {
                    Self::handle_error(app_error, &mut self.peripherals).await
                }
                Menu::MoveToProjectSelect => AppState::ProjectList(ProjectList::Continue),
            },
            AppState::Rename(rename) => match rename {
                Rename::Continue => {
                    AppState::Rename(Rename::run(&mut self.peripherals, &mut self.user_data).await)
                }
                Rename::MoveToCounting => AppState::Counting(Counting::Continue),
                Rename::MoveToError(app_error) => {
                    Self::handle_error(app_error, &mut self.peripherals).await
                }
            },
            AppState::ProjectList(project_list) => match project_list {
                ProjectList::Continue => AppState::ProjectList(
                    ProjectList::run(&mut self.peripherals, &mut self.user_data).await,
                ),
                ProjectList::MoveToCounting => AppState::Counting(Counting::Continue),
                ProjectList::MoveToError(app_error) => {
                    Self::handle_error(app_error, &mut self.peripherals).await
                }
                ProjectList::MoveToName => AppState::Rename(Rename::Continue),
            },
        }
    }

    async fn handle_error<T: Instance>(
        app_error: &AppError,
        peripherals: &mut PeripheralController<'a, T>,
    ) -> AppState {
        match app_error {
            crate::app::AppError::Memory(error) => {
                defmt::debug!("Memory error: {}", error);
                modes::Error::display_recoverable_error(app_error, peripherals).await;
                AppState::Rename(Rename::Continue)
            }
            AppError::CouldNotAddNew => {
                defmt::debug!("Could not add new option to project list");
                modes::Error::display_unrecoverable_error(app_error, peripherals).await;
                loop{
                    Timer::after_secs(3000).await
                }
            },
        }
    }
}
