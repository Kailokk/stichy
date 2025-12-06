// use core::ops::Add;

// pub struct Counter {
//     row_count: usize,
//     stich_count: usize,
// }

// impl Counter {
//     pub fn new() -> Self {
//         Self {
//             row_count: 0,
//             stich_count: 0,
//         }
//     }

//     pub fn increment_row(&mut self) -> Result<(), ()> {
//         match self.row_count.checked_add(1) {
//             Some(new_value) => {
//                 self.row_count = new_value;
//                 self.stich_count = 0
//             }
//             None => return Err(()),
//         }
//         Ok(())
//     }

//     pub fn decrement_row(&mut self) -> Result<(), ()> {
//         match self.row_count.checked_sub(1) {
//             Some(new_value) => {
//                 self.row_count = new_value;
//                 self.stich_count = 0
//             }
//             None => return Err(()),
//         }
//         Ok(())
//     }

//         pub fn increment_stitch(&mut self) -> Result<(), ()> {
//         match self.stich_count.checked_add(1) {
//             Some(new_value) => {
//                 self.stich_count = new_value;
//                 self.stich_count = 0
//             }
//             None => return Err(()),
//         }
//         Ok(())
//     }

//     pub fn decrement_stitch(&mut self) {
//         if let Some(new_value) = self.stich_count.checked_sub(1){
//             self.stich_count = new_value
//         }
//     }
// }
