#[derive(Debug)]
pub enum Error {
    CannotParse,
    IncorrectByteLength,
    WrongCode
}