use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    i2c::{Instance, InterruptHandler},
    peripherals::I2C0,
};

use crate::{
    app::{
        AppError, modes::{self, Counting, Initializing, Menu, Rename}, peripheral_controller::PeripheralController
    },
    data::UserData,
    memory_controller::MemoryIOError,
};

#[derive(defmt::Format)]
pub enum AppState {
    Initializing(Initializing),
    Counting(Counting),
    Menu(Menu),
    Rename(Rename)
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
            Ok(data) => data,
            Err(error) => {
                defmt::error!("Error reading saved state data: {}", error);
                initial_state = Initializing::MoveToError(error.into());
                Default::default()
            }
        };

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
                Initializing::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
            AppState::Counting(counting_mode) => match counting_mode {
                Counting::Continue => AppState::Counting(Counting::run(&mut self.peripherals,&mut self.user_data).await),
                Counting::MoveToMenu => AppState::Menu(Menu::Continue),
                Counting::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
            AppState::Menu(menu) => match menu{
                Menu::Continue => AppState::Menu(Menu::run(&mut self.peripherals).await),
                Menu::MoveToCounting => AppState::Counting(Counting::Continue),
                Menu::MoveToRename => AppState::Rename(Rename::Continue),
                Menu::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
            AppState::Rename(rename) => match rename{
                Rename::Continue => AppState::Rename(Rename::run(&mut self.peripherals,&mut self.user_data).await),
                Rename::MoveToCounting => AppState::Counting(Counting::Continue),
                Rename::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
        }
    }

    async fn handle_error<T:Instance>(app_error: &AppError, peripherals:&mut PeripheralController<'a,T>) -> AppState {
        match app_error {
            crate::app::AppError::Memory(_) => {
                modes::Error::display_recoverable_error(app_error, peripherals).await;
                AppState::Counting(Counting::Continue)
            }
        }
    }
}
