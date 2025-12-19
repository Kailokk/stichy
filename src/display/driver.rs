use core::error;

use embassy_rp::{Peripheral, PeripheralRef, bind_interrupts, gpio::{AnyPin, Level, Output}, i2c::{self, Async, Blocking, Config, Error, I2c, Instance, InterruptHandler, SclPin, SdaPin}, peripherals::{I2C0, PIN_0, PIN_1, PIN_2, PIN_4, PIN_5}, pio::Irq};

use crate::display::set_bit_if_true;


enum DisplayChip{
    Character,
    Backlight
}

impl DisplayChip{
    const fn as_bytes(self)->u16{
        match self{
            DisplayChip::Character => 0x7cu16 >> 1,
            DisplayChip::Backlight => 0xC0u16 >> 1,
        }
    }
}

pub struct DisplayDriver<'a> {
    i2c_connection:I2c<'a, I2C0, Async>
}

//commands are written, commands are read,

impl<'a> DisplayDriver<'a> {
    pub fn connect(
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

    //TODO: Improve design of writing mechanism to allow buffering of instructions
    fn write(&mut self, chip:DisplayChip, word: &[u8]) -> Option<Error> {
        if let Err(error)  = self.i2c_connection.blocking_write(chip.as_bytes(), word) {
            //TODO: improve error handling
            defmt::error!("Fatal i2c error occurred: {}", error);
        }

        None
    }

    pub fn write_command(&mut self, command:u8){
        let word = [0b00000000, command];
        self.write(DisplayChip::Character, &word);
    }

    pub fn write_character(&mut self, character:u8){
        let word = [0b01000000, character];
        self.write(DisplayChip::Character, &word);
    }

    //TODO: Gain A Better Understanding, and Extract from this structure
    pub fn init_backlight(&mut self){
        let word = [0b00000000,0];
        self.write(DisplayChip::Backlight, &word);

        let word = [0x08u8,0xFF];
        self.write(DisplayChip::Backlight, &word);

        let word = [0x01u8,0x20];
        self.write(DisplayChip::Backlight, &word);

        let word = [0x02u8,255];
        self.write(DisplayChip::Backlight, &word);
    }  

        pub fn set_backlight_color(&mut self,red:u8,green:u8,blue:u8){

        let word = [0x04u8,red];
        self.write(DisplayChip::Backlight, &word);
        
        let word = [0x03u8,green];
        self.write(DisplayChip::Backlight, &word);

        let word = [0x02u8,blue];
        self.write(DisplayChip::Backlight, &word);


    }  
}
