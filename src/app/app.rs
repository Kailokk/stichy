use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    flash::Flash,
    i2c::{self, Config, Instance, InterruptHandler},
    peripherals::I2C0,
    Peripherals,
};
use embassy_time::Timer;
use futures::{select_biased, FutureExt};

use crate::{
    app::{
        app_dependencies::PeripheralControllers,
        modes::{self, Counting, Initializing},
        AppError,
    },
    data::UserData,
    display::DisplayController,
    input::{init_input, INPUT_CHANNEL},
    memory_controller::{self, MemoryController, MemoryIOError},
};

//TODO: Understand this better and find out how i can move it out of here
bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

pub enum AppState {
    Initializing(Initializing),
    Counting(Counting),
    Error(modes::Error),
}

pub struct App<'a> {
    peripherals: PeripheralControllers<'a, I2C0>,
    app_state: AppState,
    user_data: UserData,
}

impl<'a> App<'a> {
    pub async fn new(mut peripherals: PeripheralControllers<'a, I2C0>) -> Self {
        let (_, user_data): ((), Result<UserData, MemoryIOError>) = join(
            Initializing::flash_display(&mut peripherals.display_controller),
            peripherals.memory_controller.read_state(),
        )
        .await;

        let mut initial_state = Initializing::MoveToCounting;
        let user_data = match user_data {
            Ok(data) => data,
            Err(error) => {
                initial_state = Initializing::MoveToError(error.into());
                Default::default()
            }
        };

        Self {
            app_state: AppState::Initializing(initial_state),
            peripherals,
            user_data: user_data,
        }
    }

    pub async fn step(&mut self) {
        self.app_state = match &self.app_state {
            AppState::Initializing(state) => match state {
                Initializing::MoveToCounting => AppState::Counting(Counting::Continue),
                Initializing::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
            AppState::Counting(counting_mode) => match counting_mode {
                Counting::Continue => AppState::Counting(Counting::run(&mut self.peripherals,&mut self.user_data).await),
                Counting::MoveToError(app_error) => Self::handle_error(app_error, &mut self.peripherals).await,
            },
            AppState::Error(error) => todo!(),
        }
    }

    async fn handle_error<T:Instance>(app_error: &AppError, mut peripherals:&mut PeripheralControllers<'a,T>) -> AppState {
        match app_error {
            crate::app::AppError::Memory(_) => {
                modes::Error::display_recoverable_error(&app_error, &mut peripherals).await;
                AppState::Counting(Counting::Continue)
            }
        }
    }
}
