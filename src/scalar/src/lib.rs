mod def;
mod decimal;
mod integer;
mod type_code;
use core::panic;
pub use decimal::Decimal;
use fumble::Fumble;
use type_code::{ Type, TypeCode };
pub use integer::Integer;

#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    Boolean(bool),
    Text(String),
    Decimal(Decimal),
    Integer(Integer)
}

impl Scalar {
    pub fn unwrap_boolean(&self) -> bool {
        match self {
            Self::Boolean(v) => *v,
            _ => panic!("{:?}", "Incorrect unwrap")
        }
    }

    pub fn unwrap_integer(&self) -> isize {
        match self {
            Self::Integer(v) => v.unwrap(),
            _ => panic!("{:?}", "Incorrect unwrap")
        }
    }

    pub fn unwrap_decimal(&self) -> isize {
        match self {
            Self::Decimal(v) => v.unwrap(),
            _ => panic!("{:?}", "Incorrect unwrap")
        }
    }

    pub fn unwrap_string(&self) -> String {
        match self {
            Self::Text(v) => v.to_owned(),
            _ => panic!("{:?}", "Incorrect unwrap")
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Integer(integer) => {
                let tp = TypeCode::get_code(Type::INTEGER);
                let mut result: Vec<u8> = vec![tp];
                
                result.extend(integer.as_bytes());

                return result
            },
            Self::Decimal(v) => {
                let tp = TypeCode::get_code(Type::DECIMAL);
                let mut result: Vec<u8> = vec![tp];
                result.extend(v.as_bytes());

                return result
                
            },
            Self::Boolean(v) => {
                let tp = TypeCode::get_code(Type::BOOLEAN);
                let mut result: Vec<u8> = vec![tp];
                if v == &true {
                    result.push(1);
                }else {
                    result.push(1);
                }
                
                return result
            },
            Self::Text(v) => {
                let tp = TypeCode::get_code(Type::TEXT);
                let mut result: Vec<u8> = vec![tp];
                result.extend(v.as_bytes().to_vec());

                return result
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        // read first byte for type
        let code = bytes[0];
        let tp = TypeCode::get_type(code).ok().unwrap();

        match tp {
            Type::BOOLEAN => {
                if bytes[1] == 1 {
                    return Self::Boolean(true)
                }else {
                    return Self::Boolean(false)
                }
            },
            Type::TEXT => Self::Text(String::from_utf8(bytes[1..bytes.len()].to_vec()).unwrap()),
            Type::DECIMAL => Self::Decimal(Decimal::from_bytes(&bytes[1..bytes.len()]).unwrap()),
            Type::INTEGER => Self::Integer(Integer::from_bytes(&bytes[1..bytes.len()]).unwrap())
        }
    }

    pub fn from_type_string(bytes: &[u8]) -> Result<Self, Fumble> {
        let code = bytes[0];
        let tp_result = TypeCode::get_type(code);
        
        match tp_result {
            Err(_err) => Err(Fumble::ScalarType),
            Ok(tp) => {
                match tp {
                    Type::BOOLEAN => {
                        if bytes[1] as char == 't' 
                        && bytes[2] as char == 'r' 
                        && bytes[3] as char == 'u' 
                        && bytes[4] as char == 'e' {
                            return Ok(Self::Boolean(true))
                        }else if bytes[1] as char == 'f' 
                        && bytes[1] as char == 'a' 
                        && bytes[2] as char == 'l' 
                        && bytes[3] as char == 's' 
                        && bytes[4] as char == 'e' {
                            return Ok(Self::Boolean(false))
                        }else {
                            return Err(Fumble::ScalarBoolean)
                        }
                    },
                    Type::TEXT => {
                        let parse_result = String::from_utf8(bytes[1..bytes.len()].to_vec());

                        match parse_result {
                            Err(_err) => Err(Fumble::ScalarString),
                            Ok(parsed) => Ok(Self::Text(parsed))
                        }
                    },
                    Type::DECIMAL => {
                        let parse_string = String::from_utf8(bytes[1..bytes.len()].to_vec());

                        match parse_string {
                            Err(_err) => Err(Fumble::ScalarDecimal),
                            Ok(parsed_string) => {
                                let decimal_parse_result = Decimal::from_string(parsed_string);

                                match decimal_parse_result {
                                    Err(_err) => Err(Fumble::ScalarDecimal),
                                    Ok(parsed) => Ok(Self::Decimal(parsed))
                                }
                            }
                        }
                    },
                    Type::INTEGER => {
                        let parse_string = String::from_utf8(bytes[1..bytes.len()].to_vec());

                        match parse_string {
                            Err(_err) => Err(Fumble::ScalarInteger),
                            Ok(parsed_string) => {
                                let integer_parse_result = Integer::from_string(parsed_string);

                                match integer_parse_result {
                                    Err(_err) => Err(Fumble::ScalarInteger),
                                    Ok(parsed) => Ok(Self::Integer(parsed))
                                }
                            }
                        }
                    }
                }
            }
        }

        
    }

    pub fn from_string(s: &str) -> Self {
        if s.len() == 0 {
            panic!("String received cannot be empty")
        }else if s.starts_with("\"") == true && s.ends_with("\"") == true {
            let text = s.trim_end_matches("\"").trim_start_matches("\"");
            Self::Text(text.to_owned())
        }else if s.starts_with("'") == true && s.ends_with("'") == true {
            let text = s.trim_end_matches("'").trim_start_matches("'");
            Self::Text(text.to_owned())
        }else if s == "true" {
            Self::Boolean(true)
        }else if s == "false" {
            Self::Boolean(false)
        }else if let Ok(int) = Integer::from_string(s.to_string()) {
            Self::Integer(int)
        }else if let Ok(dec) = Decimal::from_string(s.to_string()) {
            Self::Decimal(dec)
        }else {
            panic!("Cannot parse");
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Self::Boolean(v) => v.to_string(),
            Self::Text(v) => v.clone(),
            Self::Decimal(v) => v.to_string(),
            Self::Integer(v) => v.to_string()
        }
    }
}
