use decimal::{ D8, D16, D32, D64, DecimalTrait };
use crate::def::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Decimal {
    Tiny(D8),
    Small(D16),
    Medium(D32),
    Large(D64)
}

impl Decimal {
    pub fn unwrap(&self) -> isize {
        match self {
            Self::Tiny(v) => v.unwrap() as isize,
            Self::Small(v) => v.unwrap() as isize,
            Self::Medium(v) => v.unwrap() as isize,
            Self::Large(v) => v.unwrap() as isize
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Tiny(v) => v.as_bytes().to_vec(),
            Self::Small(v) => v.as_bytes().to_vec(),
            Self::Medium(v) => v.as_bytes().to_vec(),
            Self::Large(v) => v.as_bytes().to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() == 1 {
            Result::Ok(Self::Tiny(D8::from_bytes(bytes))) 
        }else if bytes.len() == 2 {
            Result::Ok(Self::Small(D16::from_bytes(bytes)))
        }else if bytes.len() == 4 {
            Result::Ok(Self::Medium(D32::from_bytes(bytes)))
        }else if bytes.len() == 8 {
            Result::Ok(Self::Large(D64::from_bytes(bytes)))
        }else {
            Result::Err(Error::IncorrectByteLength)
        }
    }

    pub fn from_string(s: String) -> Result<Self, Error> {
        if let Ok(tiny) = D8::from_string(s.clone()) {
            return Result::Ok(Decimal::Tiny(tiny));
        }else if let Ok(small) = D16::from_string(s.clone()) {
            return Result::Ok(Decimal::Small(small));
        }else if let Ok(medium) = D32::from_string(s.clone()) {
            return Result::Ok(Decimal::Medium(medium));
        }else if let Ok(large) = D64::from_string(s.clone()) {
            return Result::Ok(Decimal::Large(large));
        }else {
            return Result::Err(Error::CannotParse)
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

    pub fn to_integer(&self) -> isize {
        match self {
            Self::Tiny(v) => v.to_integer() as isize,
            Self::Small(v) => v.to_integer() as isize,
            Self::Medium(v) => v.to_integer() as isize,
            Self::Large(v) => v.to_integer() as isize,
        }
    }
}