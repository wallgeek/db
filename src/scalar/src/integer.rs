use crate::def::Error;
use decimal::MAX_SCALE;

#[derive(Debug, Clone, PartialEq)]
pub enum Integer {
    Tiny(i8),
    Small(i16),
    Medium(i32),
    Large(i64)
}

impl Integer {
    pub fn new(value: isize) -> Self {
        if value >= i8::MIN as isize && value <= i8::MAX as isize {
            Self::Tiny(value as i8)
        }else if value >= i16::MIN as isize && value <= i16::MAX as isize {
            Self::Small(value as i16)
        }else if value >= i32::MIN as isize && value <= i32::MAX as isize {
            Self::Medium(value as i32)
        }else {
            Self::Large(value as i64)
        }
    }

    pub fn unwrap(&self) -> isize {
        match self {
            Self::Tiny(v) => *v as isize,
            Self::Small(v) => *v as isize,
            Self::Medium(v) => *v as isize,
            Self::Large(v) => *v as isize
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Tiny(v) => v.to_le_bytes().to_vec(),
            Self::Small(v) => v.to_le_bytes().to_vec(),
            Self::Medium(v) => v.to_le_bytes().to_vec(),
            Self::Large(v) => v.to_le_bytes().to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() == 1 {
            Result::Ok(Self::Tiny(i8::from_le_bytes([bytes[0]]))) 
        }else if bytes.len() == 2 {
            Result::Ok(Self::Small(i16::from_le_bytes([bytes[0], bytes[1]])))
        }else if bytes.len() == 4 {
            Result::Ok(Self::Medium(i32::from_le_bytes([
                bytes[0], bytes[1], 
                bytes[2], bytes[3]])
            ))
        }else if bytes.len() == 8 {
            Result::Ok(Self::Large(i64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7]
            ])))
        }else {
            Result::Err(Error::IncorrectByteLength)
        }
    }

    pub fn from_string(s: String) -> Result<Self, Error> {
        let parse = s.parse::<isize>();

        if parse.is_err() {
            Result::Err(Error::CannotParse)
        }else {
            Result::Ok(Self::new(parse.unwrap()))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Tiny(v) => v.to_string(),
            Self::Small(v) => v.to_string(),
            Self::Medium(v) => v.to_string(),
            Self::Large(v) => v.to_string(),
        }
    }

    pub fn to_decimal(&self) -> isize {
        // Note: This conversion may result in overflow
        // Because decimal is limitied to certain digits
        // But this will never happen in inside the program
        // because any overflow decimal had already been
        // rejected in the very first encounter.
        // So assuming this is a successfult conversion
        let base: usize = 10;
        let multiplier = base.pow(MAX_SCALE as u32);
        self.unwrap() * (multiplier as isize)
    }
}