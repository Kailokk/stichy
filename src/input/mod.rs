
use embassy_executor::{SpawnError, Spawner};
use embassy_rp::{
    gpio::{AnyPin, Input, Pull},
    Peri,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;

#[derive(Clone, Copy, defmt::Format)]
pub enum InputIdentifier {
    ButtonA,
    ButtonB,
    ButtonC,
}

pub static INPUT_CHANNEL: Channel<ThreadModeRawMutex, InputIdentifier, 1> = Channel::new();

//TODO: Improve Input Functionality
#[embassy_executor::task(pool_size = 3)]
async fn input_task(pin: Peri<'static, AnyPin>, button_identifier: InputIdentifier) {
    let mut input = Input::new(pin, Pull::None);
    loop {
        input.wait_for_low().await;
        defmt::debug!("Button Press Registered: {}", button_identifier);
        INPUT_CHANNEL.send(button_identifier).await;
        Timer::after_millis(100).await;
        input.wait_for_high().await;
    }
}

pub fn init_input(
    spawner: &Spawner,
    button_a: Peri<'static, AnyPin>,
    button_b: Peri<'static, AnyPin>,
    button_c: Peri<'static, AnyPin>,
) -> Result<(), SpawnError> {
    spawner.spawn(input_task(button_a, InputIdentifier::ButtonA))?;
    spawner.spawn(input_task(button_b, InputIdentifier::ButtonB))?;
    spawner.spawn(input_task(button_c, InputIdentifier::ButtonC))?;
    Ok(())
}
