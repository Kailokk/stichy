use embassy_rp::{
    flash::{Async, Error, Flash, ERASE_SIZE},
    peripherals::FLASH,
};
use postcard::{from_bytes, to_slice};

use crate::data::UserData;

static FLASH_DATA_OFFSET: u32 = 0x100000;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

pub struct MemoryController<'a> {
    flash_peripheral: Flash<'a, FLASH, Async, FLASH_SIZE>,
}

impl<'a> MemoryController<'a> {
    pub fn init(flash_peripheral: Flash<'a, FLASH, Async, FLASH_SIZE>) -> Self {
        MemoryController { flash_peripheral }
    }
    pub fn save_state(&mut self, state: &UserData) -> Result<(), MemoryIOError> {
        defmt::debug!("Erasing data");
        let mut write_buffer = [0u8; ERASE_SIZE];
        let data_to_write = to_slice(state, &mut write_buffer)?;
        if data_to_write.len() > ERASE_SIZE {
            return Err(MemoryIOError::Unaligned);
        }
        self.flash_peripheral
            .blocking_erase(FLASH_DATA_OFFSET, FLASH_DATA_OFFSET + (ERASE_SIZE as u32))
            .inspect(|_| defmt::debug!("Data erased successfully"))
            .inspect_err(|err| defmt::error!("Failed to erase data: {}", err))?;

        defmt::debug!("Saving state data");
        self.flash_peripheral
            .blocking_write(FLASH_DATA_OFFSET, data_to_write)
            .map_err(MemoryIOError::from)
            .inspect(|_| defmt::debug!("Save succesful"))
            .inspect_err(|err| defmt::error!("Failed to save state data: {}", err))
    }

    pub async fn read_state(&mut self) -> Result<UserData, MemoryIOError> {
        let mut data: [u8; 104] = [0; 104];

        self.flash_peripheral
            .read(FLASH_DATA_OFFSET, data.as_mut_slice())
            .await
            .inspect(|_| defmt::debug!("Read succesful"))
            .inspect_err(|err| defmt::error!("Failed to read state data: {}", err))?;
        Ok(from_bytes(&data)?)
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
