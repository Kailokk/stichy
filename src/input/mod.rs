use embassy_executor::{SpawnError, Spawner};
use embassy_futures::{join, select};
use embassy_rp::{
    gpio::{AnyPin, Input, Pull},
    Peri,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;

#[derive(Clone, Copy, defmt::Format)]
pub enum InputIdentifier {
    ButtonAPress,
    ButtonALongPress,
    ButtonADoublePress,
    ButtonBPress,
    ButtonCPress,
}

pub static INPUT_CHANNEL: Channel<ThreadModeRawMutex, InputIdentifier, 1> = Channel::new();

//TODO: Improve Input Functionality
#[embassy_executor::task(pool_size = 1)]
async fn complex_input_task(
    pin: Peri<'static, AnyPin>,
    button_identifier: InputIdentifier,
    long_press_identifier: InputIdentifier,
    double_press_identifier: InputIdentifier,
) {
    let mut input = Input::new(pin, Pull::None);
    loop {
        input.wait_for_low().await;
        Timer::after_millis(100).await;
        match select::select(input.wait_for_high(), Timer::after_millis(800)).await {
            select::Either::First(_) => {
                match select::select(input.wait_for_low(), Timer::after_millis(200)).await {
                    select::Either::First(_) => {
                        input.wait_for_low().await;
                        INPUT_CHANNEL.send(double_press_identifier).await;
                        input.wait_for_high().await;
                    }
                    select::Either::Second(_) => INPUT_CHANNEL.send(button_identifier).await,
                }
            }
            select::Either::Second(_) => {
                join::join(
                    INPUT_CHANNEL.send(long_press_identifier),
                    input.wait_for_high(),
                )
                .await;
                ()
            }
        }
        Timer::after_millis(80).await;
    }
}

//TODO: Improve Input Functionality
#[embassy_executor::task(pool_size = 2)]
async fn input_task(pin: Peri<'static, AnyPin>, button_identifier: InputIdentifier) {
    let mut input = Input::new(pin, Pull::None);
    loop {
        input.wait_for_low().await;
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
    spawner.spawn(complex_input_task(
        button_a,
        InputIdentifier::ButtonAPress,
        InputIdentifier::ButtonALongPress,
        InputIdentifier::ButtonADoublePress,
    ))?;
    spawner.spawn(input_task(button_b, InputIdentifier::ButtonBPress))?;
    spawner.spawn(input_task(button_c, InputIdentifier::ButtonCPress))?;
    Ok(())
}
