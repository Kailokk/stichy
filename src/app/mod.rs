mod peripheral_controller;
mod app_runner;
pub mod modes;
pub use app_runner::AppRunner;
pub use peripheral_controller::PeripheralController;
use crate::{app::modes::{Counting, Initializing, Menu, ProjectList, Rename, StitchCounting}, memory_controller::MemoryIOError};



#[derive(defmt::Format)]
pub enum AppState {
    Initializing(Initializing),
    Counting(Counting),
    Menu(Menu),
    Rename(Rename),
    ProjectList(ProjectList),
    StitchCounting(StitchCounting)
}

#[derive(Debug,thiserror::Error, defmt::Format, PartialEq, Eq)]
#[error("App Error: {0}")]
pub enum AppError{
    Memory(#[from] MemoryIOError),
    #[error("Could not add new value to project list")]
    CouldNotAddNew,
}