use embassy_rp::i2c::Instance;

use crate::app::modes::CountingMode;


pub enum AppState<'a, T: Instance> {
    CountingMode(CountingMode<'a, T>),
}
