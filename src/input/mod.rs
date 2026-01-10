use embassy_executor::{SpawnError, Spawner};
use embassy_rp::{
    gpio::{AnyPin, Input, Pull},
    Peri,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Duration;

use crate::input::button::Button;

mod button;

const DEBOUNCE_DURATION: Duration = Duration::from_millis(20);
const LONG_PRESS_MIN_LENGTH: Duration = Duration::from_millis(800);
const SHORT_PRESS_MAX_INTERVAL: Duration = Duration::from_millis(200);

//TODO: Pick a number for size of this channel
pub static INPUT_CHANNEL: Channel<ThreadModeRawMutex, ButtonPress, 10> = Channel::new();

#[derive(Clone, Copy, defmt::Format)]
pub enum ButtonIdentifier {
    ButtonA,
    ButtonB,
    ButtonC,
}

#[derive(Clone, Copy, defmt::Format)]
pub enum PressKind {
    Press,
    LongPress,
    DoublePress,
}

pub struct ButtonPress {
    pub button_identifier: ButtonIdentifier,
    pub press_kind: PressKind,
}

//TODO: Improve Input Functionality
#[embassy_executor::task(pool_size = 3)]
async fn observe_button_task(pin: Peri<'static, AnyPin>, button_identifier: ButtonIdentifier) {
    let mut button = Button::new(
        Input::new(pin, Pull::Up),
        DEBOUNCE_DURATION,
        LONG_PRESS_MIN_LENGTH,
        SHORT_PRESS_MAX_INTERVAL,
    );
    loop {
        let press_kind = button.wait_for_press().await;

        INPUT_CHANNEL.send(ButtonPress {
            button_identifier,
            press_kind,
        }).await;
    }
}

pub fn init_input(
    spawner: &Spawner,
    button_a: Peri<'static, AnyPin>,
    button_b: Peri<'static, AnyPin>,
    button_c: Peri<'static, AnyPin>,
) -> Result<(), SpawnError> {
    spawner.spawn(observe_button_task(button_b, ButtonIdentifier::ButtonA))?;
    spawner.spawn(observe_button_task(button_a, ButtonIdentifier::ButtonB))?;
    spawner.spawn(observe_button_task(button_c, ButtonIdentifier::ButtonC))?;
    Ok(())
}
