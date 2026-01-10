use embassy_futures::select;
use embassy_rp::gpio::{Input, Level};
use embassy_time::{Duration, Timer};

use crate::input::PressKind;

pub struct Button<'a> {
    input: Input<'a>,
    debounce_time: Duration,
    long_press_min_length: Duration,
    double_press_max_interval: Duration,
}

impl<'a> Button<'a> {
    pub fn new(input: Input<'a>, debounce_time: Duration, long_press_min_length:Duration, double_press_max_interval:Duration) -> Self {
        Self {
            input,
            debounce_time,
            long_press_min_length,
            double_press_max_interval,
        }
    }

    pub async fn wait_for_press(&mut self) -> PressKind {
        self.wait_for_depress().await;

        // Check for long press
        match select::select(
            Timer::after(self.long_press_min_length),
            self.wait_for_release(),
        )
        .await
        {
            select::Either::First(_) => {
                self.wait_for_release().await;
                return PressKind::LongPress;
            }
            select::Either::Second(_) => {}
        }

        // Check for double press
        return match select::select(
            Timer::after(self.double_press_max_interval),
            self.wait_for_depress(),
        )
        .await
        {
            select::Either::First(_) => PressKind::Press,
            select::Either::Second(_) => {
                self.wait_for_release().await;
                PressKind::DoublePress
            }
        };
    }

    async fn wait_for_depress(&mut self) {
        loop {
            self.input.wait_for_falling_edge().await;

            Timer::after(self.debounce_time).await;

            if self.input.get_level() != Level::High {
                break;
            }
        }
    }

    async fn wait_for_release(&mut self) {
        loop {
            self.input.wait_for_rising_edge().await;

            Timer::after(self.debounce_time).await;

            if self.input.get_level() != Level::Low {
                break;
            }
        }
    }
}
