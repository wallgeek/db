use crate::def::{ DecimalTrait, ParseError };
use crate::decimal::Decimal;

const LEFT_COUNT: usize = 4; // It also represent the byte size to be stored in memory
const RIGHT_COUNT: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct D32(i32);

impl DecimalTrait<i32, LEFT_COUNT> for D32 {
    fn to_string(&self) -> String {
        Decimal::new(LEFT_COUNT, RIGHT_COUNT).to_string(self.0 as isize)
    }

    fn from_string(s: String) -> Result<Self, ParseError> {
        let decimal = Decimal::new(LEFT_COUNT, RIGHT_COUNT);
        let parsed = decimal.from_string(s);

        match parsed {
            Ok(value) => Result::Ok(Self(value as i32)),
            Err(err) => Result::Err(err)
        }
    }

    fn as_bytes(&self) -> [u8; LEFT_COUNT] {
        self.0.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != LEFT_COUNT {
            panic!("Should be of {:?} length", LEFT_COUNT)
        }

        Self(i32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3]
        ]))
    }

    fn to_isize(&self) -> isize {
        Decimal::new(LEFT_COUNT, RIGHT_COUNT).to_isize(self.0 as isize)
    }

    fn unwrap(&self) -> i32 {
        self.0
    }

    fn to_integer(&self) -> i32 {
        let base: usize = 10;
        let multiplier = base.pow(RIGHT_COUNT as u32);
        self.0 / (multiplier as i32)
    }
}