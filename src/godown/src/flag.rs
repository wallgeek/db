const DELETE: u8 = 0;
const INSERT: u8 = 1;
const UPDATE: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlagType {
    Insert,
    Delete,
    Update
}

pub struct Flag;

impl Flag {
    pub fn get_value(tp: FlagType) -> u8 {
        match tp {
            FlagType::Delete => DELETE,
            FlagType::Insert => INSERT,
            FlagType::Update => UPDATE,
        }
    }
    
    pub fn get_type(value: u8) -> FlagType {
        if value == UPDATE {
            FlagType::Update
        }else if value == DELETE {
            FlagType::Delete
        }else {
            FlagType::Insert
        }
    }
}