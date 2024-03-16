
use scalar::Scalar;
use fumble::Fumble;

const SMALL_A_TO_Z: [u8; 2] = [97, 122];
const CAPTIAL_A_TO_Z: [u8; 2] = [65, 90];
const NUMERIC_0_TO_9: [u8; 2] = [48, 57];
const UNDERSCORE: u8 = 95;
// const SPACE: u8 = 32;
// const NON_ASCII: u8 = 240;
const EQUAL: u8 = 61;
// const SINGLE_QUOTE: u8 = 39;
// const BACKSLASH: u8 = 92;
const QUERY_CREATE: [u8; 6] = [99, 114, 101, 97, 116, 101];
const QUERY_MATCH: [u8; 5] = [109, 97, 116, 99, 104];
const QUERY_RETURN: [u8; 6] = [114, 101, 116, 117, 114, 110];
const QUERY_DELETE: [u8; 6] = [100, 101, 108, 101, 116, 101];
const QUERY_SET: [u8; 3] = [115, 101, 116];

type Field = String;
type Pair = (Field, Scalar);
type Pairs = Vec<Pair>;
type Fields = Vec<Field>;
type Operator = String;
type Condition = (Pair, Operator);

#[derive(Debug)]
pub enum QueryType {
    Create(Pairs),
    Match(Condition),
    Return(Fields),
    Set(Pairs),
    Delete
}

pub struct Parser;

impl Parser {
    fn parse_field(bytes: &[u8], pos: usize) -> Result<(Field, usize), Fumble> {
        let len = bytes.len();
        let mut counter = pos;
        let mut field: String = String::new();
        let mut has_fumbled: bool = false;

        while counter < len {
            let byte = bytes[counter];

            if (byte >= SMALL_A_TO_Z[0] && byte <= SMALL_A_TO_Z[1]) 
            || (byte >= NUMERIC_0_TO_9[0] && byte <= NUMERIC_0_TO_9[1])
            || (counter != pos && byte >= CAPTIAL_A_TO_Z[0] && byte <= CAPTIAL_A_TO_Z[1])
            || byte == UNDERSCORE {
                field.push(byte as char)
            }else if byte == 0 {
                break;
            }else {
                has_fumbled = true;
                break;
            }
        
            counter += 1;
        }

        if has_fumbled {
            Err(Fumble::FieldName)
        }else {
            Ok((field, counter + 1))
        }
    }

    fn parse_value(bytes: &[u8], pos: usize) -> Result<(Scalar, usize), Fumble> {
        let len = bytes.len();
        let mut counter = pos;
        
        while counter < len {
            if bytes[counter] == 0 {
                break;
            }

            counter += 1;
        }

        let result = Scalar::from_type_string(&bytes[pos..counter]);

        match result {
            Ok(parsed) => Ok((parsed, counter + 1)),
            Err(err) => Err(err)
        }
    }

    fn parse_operator(bytes: &[u8], pos: usize) -> Result<(String, usize), Fumble> {
        let len = bytes.len();
        let mut counter = pos;

        while counter < len {
            if bytes[counter] == 0 {
                break
            }

            counter += 1;
        }
        
        if counter - pos > 1 {
            Err(Fumble::Operator)
        }else if bytes[counter - 1] == EQUAL {
            Ok((String::from("="), counter + 1))
        }else {
            Err(Fumble::Operator)
        }
    }

    fn parse_condition(bytes: &[u8], pos: usize) -> Result<(QueryType, usize), Fumble> {
        let field_result = Self::parse_field(bytes, pos);

        match field_result {
            Err(err) => Err(err),
            Ok((field, pos)) => {
                let operator_result = Self::parse_operator(bytes, pos);

                match operator_result {
                    Err(err) => Err(err),
                    Ok((operator, pos)) => {
                        let scalar_result = Self::parse_value(bytes, pos);

                        match scalar_result {
                            Err(err) => Err(err),
                            Ok((scalar, pos)) => {
                                return Ok((QueryType::Match(((field, scalar), operator)), pos));
                            }
                        }
                    }
                }
            }
        }
    }

    fn parse_fields(bytes: &[u8], pos: usize) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        let mut counter = pos;
        let len = bytes.len();
        let mut field: String = String::new();

        while counter < len {
            let byte = bytes[counter];

            if byte == 0 {
                result.push(field);
                field = String::new();
            }else {
                field.push(byte as char);
            }

            counter += 1;
        }

        result
    }

    fn parse_pairs(bytes: &[u8], pos: usize) -> Result<(Pairs, usize), Fumble> {
        let len = bytes.len();
        let mut counter = pos;
        let mut result: Vec<Pair> = Vec::new();

        while counter < len {
            let field_result = Self::parse_field(bytes, counter);

            match field_result {
                Err(err) => {
                    return Err(err);
                },
                Ok((field, f_pos)) => {
                    let scalar_result = Self::parse_value(bytes, f_pos);
    
                    match scalar_result {
                        Err(err) => {
                            return Err(err)
                        },
                        Ok((scalar, s_pos)) => {
                            result.push((field, scalar));

                            counter = s_pos;
                        }
                    }
                }
            }
        }

        Ok((result, counter + 1))
    }

    fn parse_action(bytes: &[u8], pos: usize) -> Result<QueryType, Fumble> {
        if bytes[pos] == QUERY_DELETE[0] 
        && bytes[pos + 1] == QUERY_DELETE[1]
        && bytes[pos + 2] == QUERY_DELETE[2]
        && bytes[pos + 3] == QUERY_DELETE[3]
        && bytes[pos + 4] == QUERY_DELETE[4]
        && bytes[pos + 5] == QUERY_DELETE[5]
        && bytes[pos + 6] == 0 {
            Ok(QueryType::Delete)
        }else if bytes[pos] == QUERY_RETURN[0] 
        && bytes[pos + 1] == QUERY_RETURN[1]
        && bytes[pos + 2] == QUERY_RETURN[2]
        && bytes[pos + 3] == QUERY_RETURN[3]
        && bytes[pos + 4] == QUERY_RETURN[4]
        && bytes[pos + 5] == QUERY_RETURN[5]
        && bytes[pos + 6] == 0 {
            Ok(QueryType::Return(Self::parse_fields(bytes, pos + 7)))
        }else if bytes[pos] == QUERY_SET[0] 
        && bytes[pos + 1] == QUERY_SET[1]
        && bytes[pos + 2] == QUERY_SET[2]
        && bytes[pos + 3] == 0 {
            let pairs_result = Self::parse_pairs(bytes, pos + 4);

            match pairs_result {
                Err(err) => Err(err),
                Ok((pairs, _)) => {
                    Ok(QueryType::Set(pairs))
                }
            }
        }else {
            Err(Fumble::Action)
        }
    }

    pub fn parse(bytes: &[u8]) -> Result<Vec<QueryType>, Fumble> {
        let mut result: Vec<QueryType> = Vec::new();
        
        if bytes[0] == QUERY_MATCH[0] 
        && bytes[1] == QUERY_MATCH[1]
        && bytes[2] == QUERY_MATCH[2]
        && bytes[3] == QUERY_MATCH[3]
        && bytes[4] == QUERY_MATCH[4]
        && bytes[5] == 0 {
            let condition_result = Self::parse_condition(bytes, 6);

            match condition_result {
                Err(err) => {
                    return Err(err);
                },
                Ok((match_query_type, pos)) => {
                    let action_result = Self::parse_action(bytes, pos);        

                    match action_result {
                        Err(err) => {
                            return Err(err);
                        },
                        Ok(action_query_type) => {
                            result.push(action_query_type);
                            result.push(match_query_type);
                        }
                    }   
                }
            }
        }else if bytes[0] == QUERY_CREATE[0] 
        && bytes[1] == QUERY_CREATE[1]
        && bytes[2] == QUERY_CREATE[2]
        && bytes[3] == QUERY_CREATE[3]
        && bytes[4] == QUERY_CREATE[4] 
        && bytes[5] == QUERY_CREATE[5]
        && bytes[6] == 0 {
            let pairs_result = Self::parse_pairs(bytes, 7);

            match pairs_result {
                Err(err) => {
                    return Err(err)
                },
                Ok((pairs, _)) => {
                    result.push(QueryType::Create(pairs));
                }
            }
        }else {
            return Err(Fumble::Start)
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use  super::*;

    #[test]
    fn parse_field(){
        let query = b"name\0#hello\0";
        let field_result = Parser::parse_field(query, 0);
        
        match field_result {
            Err(err) => panic!("{:?}", err),
            Ok((field, pos)) => {
                assert_eq!(field.as_str(), "name");
            }
        }
    }

    #[test]
    fn parse_value(){
        let query = b"name\0#hello\0";
        let value_result = Parser::parse_value(query, 5);
        
        match value_result {
            Err(err) => panic!("{:?}", err),
            Ok((scalar, pos)) => {
                assert_eq!(scalar, Scalar::Text("hello".to_owned()));
            }
        }
    }

    #[test]
    fn parse_operator(){
        let query = b"name\0=\0#hello\0";
        let operator_result = Parser::parse_operator(query, 5);
        
        match operator_result {
            Err(err) => panic!("{:?}", err),
            Ok((operator, pos)) => {
                assert_eq!(operator, "=".to_owned());
            }
        }
    }

    #[test]
    fn parse_pairs(){
        let query = b"field1\0#string\0field2\0!true\0field3\0%0.1\0field4\0&1\0";
        let pairs_result = Parser::parse_pairs(query, 0);
        
        match pairs_result {
            Err(err) => panic!("{:?}", err),
            Ok((pairs, pos)) => {
                for pair in pairs {
                    // let field = pair.0;
                    // let scalar = pair.1;
                    println!("Pair: {:?}", pair)
                    // assert_eq!(field, "field1".to_owned())
                }
                
            }
        }
    }

    #[test]
    fn parse() {
        let query = b"create\0field1\0#name\0field2\0!true\0";
        let query_type_result = Parser::parse(query);

        match query_type_result {
            Err(err) => panic!("{:?}", err),
            Ok(queryType) => println!("{:?}", queryType)
        }
    }
}