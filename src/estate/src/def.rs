/// Wholenumber trait is defined for estate
/// Any estate will implement this trait

use std::{fmt::Debug, hash::Hash, ops::{Add, Sub}};

pub trait WholeNumber: Default
+ Debug
+ Copy
+ Eq 
+ Hash 
+ Add<Self, Output = Self> 
+ Sub<Output = Self> 
+ PartialOrd 
{
    fn add_int(self, value: i32) -> Self;
    // fn add_with<T: WholeNumber>(self, value: T) -> Self;
    fn to_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
    fn get_max() -> Self;
}

impl WholeNumber for u8 {
    fn add_int(self, value: i32) -> Self {
        let result: Result<Self, _> = ((self as i32) + value).try_into();

        match result {
            Ok(output) => output,
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    // fn add_with<T: WholeNumber>(self, value: T) -> Self {
    //     let add: Result<Self, _> = ((self as usize) + value.to_usize()).try_into();

    //     match add {
    //         Ok(result) => result,
    //         Err(err) => {
    //             panic!("{:?}", err)
    //         }
    //     }
    // }

    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }

    fn get_max() -> Self {
        Self::MAX
    }
}

impl WholeNumber for u16 {
    fn add_int(self, value: i32) -> Self {
        let result: Result<Self, _> = ((self as i32) + value).try_into();

        match result {
            Ok(output) => output,
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    // fn add_with<T: WholeNumber>(self, value: T) -> Self {
    //     let add: Result<Self, _> = ((self as usize) + value.to_usize()).try_into();

    //     match add {
    //         Ok(result) => result,
    //         Err(err) => {
    //             panic!("{:?}", err)
    //         }
    //     }
    // }
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }

    fn get_max() -> Self {
        Self::MAX
    }
}

impl WholeNumber for u32 {
    fn add_int(self, value: i32) -> Self {
        let result: Result<Self, _> = ((self as isize) + (value as isize)).try_into();

        match result {
            Ok(output) => output,
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    // fn add_with<T: WholeNumber>(self, value: T) -> Self {
    //     let add: Result<Self, _> = ((self as usize) + value.to_usize()).try_into();

    //     match add {
    //         Ok(result) => result,
    //         Err(err) => {
    //             panic!("{:?}", err)
    //         }
    //     }
    // }
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }

    fn get_max() -> Self {
        Self::MAX
    }
}

impl WholeNumber for u64 {
    fn add_int(self, value: i32) -> Self {
        let result: Result<Self, _> = ((self as isize) + (value as isize)).try_into();

        match result {
            Ok(output) => output,
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    // fn add_with<T: WholeNumber>(self, value: T) -> Self {
    //     let add: Result<Self, _> = ((self as usize) + value.to_usize()).try_into();

    //     match add {
    //         Ok(result) => result,
    //         Err(err) => {
    //             panic!("{:?}", err)
    //         }
    //     }
    // }
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }

    fn get_max() -> Self {
        Self::MAX
    }
}