use crate::def::Error;

const BOOLEAN: u8 = 33; // !
const STRING: u8 = 35; // #
const DECIMAL: u8 = 37; // %
const INTEGER: u8 = 38; // &

#[derive(Debug, PartialEq)]
pub enum Type {
    TEXT,
    BOOLEAN,
    DECIMAL,
    INTEGER
}

pub struct TypeCode;

impl TypeCode {
    pub fn get_code(t: Type) -> u8 {
        match t {
            Type::TEXT => STRING,
            Type::BOOLEAN => BOOLEAN,
            Type::DECIMAL => DECIMAL,
            Type::INTEGER => INTEGER
        }
    }

    pub fn get_type(code: u8) -> Result<Type, Error> {
        if code == STRING {
            Ok(Type::TEXT)
        }else if code == BOOLEAN {
            Ok(Type::BOOLEAN)
        }else if code == DECIMAL {
            Ok(Type::DECIMAL)
        }else if code == INTEGER {
            Ok(Type::INTEGER)
        }else {
            Err(Error::WrongCode)
        }
    }
}