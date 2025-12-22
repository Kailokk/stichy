use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppData{
    //160 bytes
    pub pages:[Page; 10],
    // 4 bytes
    pub last_page:usize,
    // 3 bytes
    pub background_color:(u8,u8,u8)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Page{
    pub name:[u8;8],
    pub row_count:u32,
    pub stitch_count:u32,
}