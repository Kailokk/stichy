use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, PartialEq, Default, defmt::Format)]
pub struct UserData{
    pages:[Page; 10],
    current_page:usize,
    background_color:(u8,u8,u8)
}

impl UserData{
    pub fn current_page(&self)->&Page{
        &self.pages[self.current_page]
    }

    pub fn current_page_mut(&mut self)->&mut Page{
        &mut self.pages[self.current_page]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, defmt::Format)]
pub struct Page{
    pub name:[u8;8],
    pub row_count:u16,
}