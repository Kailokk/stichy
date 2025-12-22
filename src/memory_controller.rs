use core::ops::Deref;
use embassy_rp::{
    dma::Channel,
    flash::{Async, Error, Flash},
    peripherals::FLASH,
    Peri,
};
use heapless::Vec;
use postcard::{from_bytes, to_vec};
use serde::{Deserialize, Serialize};

use crate::data::UserData;

static FLASH_DATA_OFFSET: u32 = 0x100000;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

pub struct MemoryController<'a> {
    flash_peripheral: Flash<'a, FLASH, Async, FLASH_SIZE>,
}

impl<'a> MemoryController<'a> {
    pub fn init(flash_peripheral: Flash<'a, FLASH, Async, FLASH_SIZE>) -> Self {
        MemoryController {
            flash_peripheral: flash_peripheral,
        }
    }
    pub fn save_state(&mut self, state: &UserData) -> Result<(), MemoryIOError> {
        let data: heapless::Vec<u8, 256> = postcard::to_vec(state)?;

        self.flash_peripheral
            .blocking_write(FLASH_DATA_OFFSET, data.as_slice())
            .map_err(MemoryIOError::from)
    }

    pub async fn read_state(&mut self) -> Result<UserData, MemoryIOError> {
        let mut data: [u8; 256] = [0; 256];

        self.flash_peripheral
            .read(FLASH_DATA_OFFSET, data.as_mut_slice())
            .await?;
        return Ok(from_bytes(&data)?);
    }
}

#[derive(thiserror::Error, Debug, defmt::Format)]
#[error("Memory IO Error: {0}")]
pub enum MemoryIOError {
    SerializationError(#[from] postcard::Error),
    #[error("Memory address does not exist in memory")]
    OutOfBounds,
    #[error("Operation or buffer was unaligned")]
    Unaligned,
    #[error("Memory accessed from incorrect core")]
    InvalidCore,
    #[error("Unexplained error occurred")]
    Other,
}

impl From<embassy_rp::flash::Error> for MemoryIOError {
    fn from(value: embassy_rp::flash::Error) -> Self {
        match value {
            Error::OutOfBounds => Self::OutOfBounds,
            Error::Unaligned => Self::Unaligned,
            Error::InvalidCore => Self::InvalidCore,
            Error::Other => Self::Other,
        }
    }
}
