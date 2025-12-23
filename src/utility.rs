use core::ops::{Add, Sub};

pub const fn string_as_bytes<const SIZE: usize>(string: &str) -> [u8; SIZE] {
    let mut array = [0; SIZE];
    array.copy_from_slice(string.as_bytes());
    array
}


    pub fn wrapped_increment<T: Add<Output=T> + PartialOrd + From<usize>>(index: T, min:T, max:T) -> T {
        if index >= max {
            min
        } else {
            Add::add(index, 1.into() ) 
        }
    }


    pub fn wrapped_deincrement<T: Sub<Output=T> + PartialOrd + From<usize>>(index: T, min:T, max:T) -> T {
        if index <= min {
            max
        } else {
            Sub::sub(index, 1.into() ) 
        }
    }


        pub fn wrapped_increment_u8<T: Add<Output=T> + PartialOrd + From<u8>>(index: T, min:T, max:T) -> T {
        if index >= max {
            min
        } else {
            Add::add(index, 1.into() ) 
        }
    }


    pub fn wrapped_deincrement_u8<T: Sub<Output=T> + PartialOrd + From<u8>>(index: T, min:T, max:T) -> T {
        if index <= min {
            max
        } else {
            Sub::sub(index, 1.into() ) 
        }
    }