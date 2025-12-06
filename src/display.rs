


enum CommunicationProtocol{
    I2C,
    ByteController,
    Terminal
}


impl CommunicationProtocol{
    pub fn new_terminal()-> Self{

    }
}


pub trait Display {
    fn write_from();

    fn update_character();

    fn clear_character();
}