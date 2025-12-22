
pub mod modes;
mod app_state;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, Config, InterruptHandler},
    peripherals::I2C0,
    Peripherals,
};
use embassy_time::Timer;
use futures::{select_biased, FutureExt};

use crate::{app::{app_state::AppState, modes::CountingMode}, display::DisplayController, input::{INPUT_CHANNEL, init_input}};


//TODO: Understand this better and find out how i can move it out of here
bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

pub struct App<'a> {
    state: AppState<'a, I2C0>,
}

impl<'a> App<'a> {
    pub async fn init(spawner: &Spawner, peripherals: Peripherals) -> Self {
        defmt::debug!("Initiating startup sequence");

        match init_input(
            spawner,
            peripherals.PIN_18.into(),
            peripherals.PIN_17.into(),
            peripherals.PIN_16.into(),
        ) {
            Ok(_) => (),
            //TODO: handle error
            Err(error) => defmt::error!("Error initializing input peripherals: {}", error),
        }

        let mut config = Config::default();

        config.frequency = 400_000; //400khz

        let i2c_peripheral = i2c::I2c::new_async(
            peripherals.I2C0,
            peripherals.PIN_5,
            peripherals.PIN_4,
            Irqs,
            config,
        );

        Timer::after_millis(50).await;

        let display_controller = DisplayController::init(i2c_peripheral).await;

        Self {
            state: AppState::CountingMode(CountingMode::enter(display_controller).await),
        }
    }

    pub async fn run(&mut self) {
        defmt::debug!("App initialization complete, running program");

        loop {
            match &mut self.state {
                AppState::CountingMode(app) => {
                    select_biased! {
                    input = INPUT_CHANNEL.receive().fuse() => {
                        match input{
                            crate::input::InputIdentifier::ButtonA => {
                                app.reset_count().await;
                            },
                            crate::input::InputIdentifier::ButtonB => {
                                app.decrement_count().await;
                            },
                            crate::input::InputIdentifier::ButtonC => {
                                app.increment_count().await;
                            },
                        }
                    }
                    }
                }
            }
        }
    }
}
