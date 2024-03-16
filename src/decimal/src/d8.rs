use crate::def::{ DecimalTrait, ParseError };
use crate::decimal::Decimal;

const LEFT_COUNT: usize = 1; // It also represent the byte size to be stored in memory
const RIGHT_COUNT: usize = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct D8(i8);

impl DecimalTrait<i8, LEFT_COUNT> for D8 {
    fn to_string(&self) -> String {
        Decimal::new(LEFT_COUNT, RIGHT_COUNT).to_string(self.0 as isize)
    }

    fn from_string(s: String) -> Result<Self, ParseError> {
        let decimal = Decimal::new(LEFT_COUNT, RIGHT_COUNT);
        let parsed = decimal.from_string(s);

        match parsed {
            Ok(value) => Result::Ok(Self(value as i8)),
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

        Self(i8::from_le_bytes([bytes[0]]))
    }

    fn to_isize(&self) -> isize {
        Decimal::new(LEFT_COUNT, RIGHT_COUNT).to_isize(self.0 as isize)
    }

    fn unwrap(&self) -> i8 {
        self.0
    }

    fn to_integer(&self) -> i8 {
        let base: usize = 10;
        let multiplier = base.pow(RIGHT_COUNT as u32);
        self.0 / (multiplier as i8)
    }
}