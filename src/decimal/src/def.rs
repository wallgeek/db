pub const MAX_SCALE: usize = 10;

#[derive(Debug)]
pub enum ParseError {
    NotValidDecimal,
    OutOfBound,
    NotValidNumerics
}

pub trait DecimalTrait<T, const N: usize> 
where Self: Sized {
    fn to_string(&self) -> String;
    fn from_string(s: String) -> Result<Self, ParseError>; 
    fn as_bytes(&self) -> [u8; N];
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_isize(&self) -> isize;
    fn unwrap(&self) -> T;
    fn to_integer(&self) -> T;
}

