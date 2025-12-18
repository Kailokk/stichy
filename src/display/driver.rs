use embassy_rp::gpio::{AnyPin, Level, Output};

use crate::display::command::Command;

pub enum DisplayDriver<'a> {
    FourBit {
        register_select_pin: Output<'a>,
        rw_pin: Output<'a>,
        enable_pin: Output<'a>,
        db4: Output<'a>,
        db5: Output<'a>,
        db6: Output<'a>,
        db7: Output<'a>,
    },
}


//commands are written, commands are read,

impl<'a> DisplayDriver<'a> {
    pub fn new_four_bit(
        register_select_pin: AnyPin,
        rw_pin: AnyPin,
        enable_pin: AnyPin,
        data_pins: [AnyPin; 4],
    ) -> Self {
        let rw_pin = Output::new(rw_pin, Level::Low);
        let enable_pin = Output::new(enable_pin, Level::Low);
        let register_select_pin = Output::new(register_select_pin, Level::Low);
        let [db4, db5, db6, db7] = dat a_pins;
        let db4 = Output::new(db4, Level::Low);
        let db5 = Output::new(db5, Level::Low);
        let db6 = Output::new(db6, Level::Low);
        let db7 = Output::new(db7, Level::Low);

        Self::FourBit {
            register_select_pin,
            rw_pin,
            enable_pin,
            db4,
            db5,
            db6,
            db7,
        }
    }

    pub async fn write(&mut self, commands: &[Command]) {
        for command in commands {
            self.execute_command(command).await
        }
    }

    async fn execute_command(&mut self, command: &Command) {
        match self {
            DisplayDriver::FourBit { register_select_pin, rw_pin, enable_pin, db4, db5, db6, db7 } => {
                register_select_pin.set_high();   
                rw_pin.set_low();
                for bit in 0..3{

                }
            },
        }
        
    }

    async fn set_data_pins()
}


fn byte_to_bool_array<const ARR_LEN:usize>(byte:u8){
    let mut bits:[bool; ARR_LEN] = [
        
    ];
}