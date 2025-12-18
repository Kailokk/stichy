use embassy_rp::i2c_slave::Command;



type DisplayCoordinate = (u8,u8);





pub struct DisplayController{
    driver:(),
    command_buffer:[Command; 32],
    
}

impl DisplayController{
    pub fn clear_display(){

    }

    //this could maybe be abstracted
    pub fn return_home(){

    }

    pub fn configure_display(){

    }
}


