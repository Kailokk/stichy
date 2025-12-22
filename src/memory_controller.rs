use embassy_rp::{
    flash::{Async, Error, Flash},
    peripherals::FLASH,
};
use core::ops::Deref;
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};
use heapless::Vec;

use crate::data::AppData;

static FLASH_DATA_OFFSET: u32 = 0x100000;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[derive(thiserror)]
pub enum MemoryError{
    SerializationError(#[from] postcard::Error),
    IOError(#[from] embassy_rp::flash::Error)

}

pub struct MemoryController<'a> {
    flash_peripheral: Flash<'a, FLASH, Async, FLASH_SIZE>,
}

impl<'a> MemoryController<'a> {
    pub fn save_state(&mut self, state: &AppData)->Result<(),MemoryError> {
        let data: Vec<u8,256> = to_vec::<u8,256>(&state)?;

       self.flash_peripheral
            .blocking_write(FLASH_DATA_OFFSET, data.as_slice())
    }

    pub async fn read_state(&mut self, state: &AppData)->Result<(),MemoryError> {
        let mut data:[u8;256] = [0;256];

         self.flash_peripheral
            .read(FLASH_DATA_OFFSET, data.as_slice()).await;
        return Ok( from_bytes(&data)?)
    }
}
