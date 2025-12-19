use core::error;

use embassy_rp::{Peripheral, PeripheralRef, bind_interrupts, gpio::{AnyPin, Level, Output}, i2c::{self, Async, Blocking, Config, Error, I2c, Instance, InterruptHandler, SclPin, SdaPin}, peripherals::{I2C0, PIN_0, PIN_1, PIN_2, PIN_4, PIN_5}, pio::Irq};


enum DisplayChip{
    Character,
    Backlight
}

impl DisplayChip{
    const fn as_bytes(self)->u16{
        match self{
            DisplayChip::Character => 0x7cu16 >> 1,
            DisplayChip::Backlight => todo!(),
        }
    }
}

pub struct DisplayDriver<'a> {
    i2c_connection:I2c<'a, I2C0, Async>
}

//commands are written, commands are read,

impl<'a> DisplayDriver<'a> {
    pub fn new_i2c_driver(
        //TODO: Genericise these types where possible?
        peripheral:I2C0,
        sda_pin: PIN_4,
        scl_pin: PIN_5,
        
    ) -> Self {

        bind_interrupts!(struct Irqs {
        I2C0_IRQ => InterruptHandler<I2C0>;
        });

        let mut config = Config::default();

        config.frequency = 400_000; //400khz

        Self { i2c_connection:i2c::I2c::new_async(peripheral, scl_pin, sda_pin, Irqs, config)  }
    }

    fn write(&mut self, chip:DisplayChip, bytes: &[u8]) -> Option<Error> {
        if let Err(error)  = self.i2c_connection.blocking_write(chip.as_bytes(), bytes) {
            //TODO: improve error handling
            defmt::error!("Fatal i2c error occurred: {}", error);
        }

        None
    }

        pub async fn write_char(&mut self, command: u8) -> Option<Error> {
        defmt::debug!("Writing command: {:08b}",command);
        if let Err(error)  = self.i2c_connection.write_async(0x7cu16 >> 1, [0b01000000,command]).await {
            //TODO: improve error handling
            defmt::error!("Fatal i2c error occurred: {}", error);
        }

        None
    }
}
