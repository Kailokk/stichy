mod peripheral_controller;
mod app;
pub mod modes;
pub use app::App;
pub use peripheral_controller::PeripheralController;
use crate::memory_controller::MemoryIOError;


#[derive(Debug,thiserror::Error, defmt::Format, PartialEq, Eq)]
#[error("App Error: {0}")]
pub enum AppError{
    Memory(#[from] MemoryIOError),
    #[error("Could not add new value to project list")]
    CouldNotAddNew,
}