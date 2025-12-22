mod app_dependencies;
pub mod modes;
mod app;
pub use app::App;

use crate::memory_controller::MemoryIOError;


#[derive(Debug,thiserror::Error, defmt::Format)]
#[error("App Error: {0}")]
enum AppError{
    Memory(#[from] MemoryIOError)
}