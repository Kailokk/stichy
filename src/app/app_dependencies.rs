use embassy_rp::i2c::Instance;

use crate::{display::DisplayController, memory_controller::MemoryController};



pub struct PeripheralControllers<'a, T: Instance>{
    pub display_controller: DisplayController<'a, T>,
    pub memory_controller:MemoryController<'a>,
}

impl<'a, T:Instance> PeripheralControllers<'a,T>{
    pub fn new(display_controller:DisplayController<'a,T>, memory_controller:MemoryController<'a>)->Self{
        Self{
            display_controller,
            memory_controller
        }
    }
}