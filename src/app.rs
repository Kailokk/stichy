use embassy_executor::Spawner;
use embassy_rp::{
    Peripherals, bind_interrupts, i2c::{self, Config, InterruptHandler}, peripherals::I2C0
};
use embassy_time::Timer;
use futures::{select_biased, FutureExt};

use crate::{display::DisplayController, input::{INPUT_CHANNEL, init_input}};

//TODO: Understand this better and find out how i can move it out of here
bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

enum AppState {
    Initialized,
    Counter,
}

pub struct App<'a> {
    state: AppState,
    display_controller: DisplayController<'a, I2C0>,
}

impl<'a> App<'a> {
    pub async fn init(spawner: &Spawner, peripherals: Peripherals) -> Self {
        defmt::debug!("Initiating startup sequence");

      match  init_input(
            spawner,
            peripherals.PIN_18.into(),
            peripherals.PIN_17.into(),
            peripherals.PIN_16.into(),
        ){
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
            state: AppState::Initialized,
            display_controller,
        }
    }


    pub async fn run(&mut self){
        loop{
            select_biased!{
                input = INPUT_CHANNEL.receive().fuse() => {
                    let color:(u8,u8,u8) = match input{
                        crate::input::InputIdentifier::ButtonA => (255,0,0),
                        crate::input::InputIdentifier::ButtonB => (0,255,0),
                        crate::input::InputIdentifier::ButtonC => (0,0,255),
                    };

                    self.display_controller.set_backlight_color(color.0,color.1, color.2);
                }
            }
        }
    }
}
