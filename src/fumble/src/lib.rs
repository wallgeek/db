#[derive(Debug)]
pub enum Fumble {
    FieldName,
    FieldSize,
    ScalarType,
    ScalarString,
    ScalarBoolean,
    ScalarDecimal,
    ScalarInteger,
    ScalarsCombinedSize,
    Operator,
    Start,
    Action,
    Identifier
}

impl Fumble {
    pub fn unwrap(&self) -> String {
        let result: &str;

        match self {
            Fumble::FieldName => {
                result = "Field name should only contain alphanumeric and underscore. First character shouldn't be capital"
            },

            Fumble::FieldSize => {
                result = "Field name is too long"
            },

            Fumble::ScalarType => {
                result = "Wrong. Only String, integer, decimal and boolean data types expected"
            },

            Fumble::ScalarString => {
                result = "Cannot parse string value"
            },

            Fumble::ScalarBoolean => {
                result = "Cannot parse boolean value"
            },
            
            Fumble::ScalarDecimal => {
                result = "Cannot parse decimal value"
            },

            Fumble::ScalarInteger => {
                result = "Cannot parse integer value"
            },

            Fumble::ScalarsCombinedSize => {
                result = "Data size limit exceeds"
            },

            Fumble::Operator => {
                result = "Unrecognized operator"
            },

            Fumble::Start => {
                result = "Query should either start with 'create' or 'match'"
            },

            Fumble::Action => {
                result = "'match' should be followed by 'return', 'set' or 'delete'"
            },

            Fumble::Identifier => {
                result = "Cannot set _id"
            }
        }

        let mut fumble = String::new();
        fumble.push('-');
        fumble.push_str(result);
        
        fumble
    }
}

