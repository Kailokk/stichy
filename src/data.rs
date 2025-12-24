use serde::{Deserialize, Serialize};


pub static PAGE_COUNT:usize = 10;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, defmt::Format)]
pub struct UserData{
    pages:[Page; PAGE_COUNT],
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

    pub fn set_current_page(&mut self, new_page:usize){
        self.current_page = new_page;
    }

    pub fn all_pages(&self)->&[Page; 10]{
        &self.pages
    }

}

#[derive(Serialize, Deserialize,  Debug, PartialEq, defmt::Format, Clone, Copy)]
pub struct Page{
    pub name:[u8;10],
    pub row_count:u16,
}

pub static DEFAULT_NAME_CHARACTER:u8 = 0b00100000;

pub static DEFAULT_NAME:[u8;10] = [DEFAULT_NAME_CHARACTER;10];

impl Default for Page{
    fn default() -> Self {
        Self { name: DEFAULT_NAME, row_count: Default::default() }
    }
}
