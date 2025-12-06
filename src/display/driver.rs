use bbqueue::Consumer;
use embassy_rp::gpio::{AnyPin, Level, Output};

use crate::display::DISPLAY_COMMAND_BUFFER;


struct FourBitDriver<'a> {
    command_queue: Consumer<'a, 32>,
    register_select_pin: Output<'a>,
    rw_pin: Output<'a>,
    enable_pin: Output<'a>,
    db4: Output<'a>,
    db5: Output<'a>,
    db6: Output<'a>,
    db7: Output<'a>,
}

//commands are written, commands are read,

impl<'a> FourBitDriver<'a> {
    pub fn new(
        command_consumer: Consumer<'a, 32>,
        register_select_pin: AnyPin,
        rw_pin: AnyPin,
        enable_pin: AnyPin,
        data_pins: [AnyPin; 4],
    ) -> Self {

        let rw_pin = Output::new(rw_pin, Level::Low);
        let enable_pin = Output::new(enable_pin, Level::Low);
        let register_select_pin = Output::new(register_select_pin, Level::Low);
        let [db4, db5, db6, db7] = data_pins;
        let db4 = Output::new(db4, Level::Low);
        let db5 = Output::new(db5, Level::Low);
        let db6 = Output::new(db6, Level::Low);
        let db7 = Output::new(db7, Level::Low);

        Self {
            command_queue: command_consumer,
            register_select_pin,
            rw_pin,
            enable_pin,
            db4,
            db5,
            db6,
            db7,
        }
    }

    pub async fn write(&mut self) {
        let DISPLAY_COMMAND_BUFFER.receive().await
        let mut commands = match self.command_queue.read() {
            Ok(commands) => commands,
            Err(error) => return, //return as all error simply mean nothing can be read at this time
        };

        for command in commands.buf() {
            self.execute_command(*command).await
        }
        commands.to_release(commands.len());
    }

    async fn execute_command(&mut self, command: u8) {}
}
