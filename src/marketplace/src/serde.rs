use scalar::Scalar;
pub struct Serde;

impl Serde {
    pub fn pairs(pairs: Vec<(String, Scalar)>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for pair in pairs {
            let field = pair.0;
            let scalar = pair.1;

            bytes.extend(field.as_bytes());
            bytes.push(0);
            let scalar_type = scalar.as_bytes()[0];
            bytes.push(scalar_type);
            bytes.extend(scalar.to_string().as_bytes());
            bytes.push(0);
        }

        bytes
    }

    pub fn response(data: Vec<Vec<(String, Scalar)>>) -> String {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push('+' as u8);

        for pairs in data {
            bytes.extend(Self::pairs(pairs));
            bytes.push(10);
        }

        String::from_utf8(bytes).unwrap()
    }

    // pub fn serialize(pairs: Vec<(String, Scalar)>) -> String {
    //     let mut json: String = String::new();
    //     let mut counter = 0;
    //     let len = pairs.len();
    //     json.push('{');
        
    //     for (field, scalar) in pairs {
    //         json.push('\"');
    //         json.push_str(field.as_ref());
    //         json.push_str("\":");

    //         match scalar {
    //             Scalar::Boolean(v) => {
    //                 json = json + &v.to_string();
    //             },
    //             Scalar::Integer(v) => {
    //                 json = json + &v.to_string();
    //             },
    //             Scalar::Decimal(v) => {
    //                 json = json + &v.to_string();
    //             },
    //             Scalar::Text(v) => {
    //                 json.push('\"');
    //                 json.push_str(v.as_str());
    //                 json.push('\"');
    //             }
    //         }

    //         if counter != len - 1 {
    //             json.push(',')
    //         }

    //         counter += 1;
    //     }

    //     json.push('}');

    //     json
    // }

    // pub fn deserialize(s: &str) -> Vec<(String, Scalar)> {
    //     let mut result: Vec<(String, Scalar)> = Vec::new();
    //     let mut is_key = true;
    //     let mut key: String = String::new();
    //     let mut value: String = String::new();

    //     for ch in s.chars() {
    //         if ch == ',' {
    //             result.push((key.trim().to_owned(), Scalar::from_string(value.trim())));
    //             key = String::new();
    //             value = String::new();

    //             is_key = true
    //         }else if ch == '=' {
    //             is_key = false;
    //         }else if is_key {
    //             key.push(ch);
    //         }else {
    //             value.push(ch);
    //         }
    //     }

    //     result.push((key.trim().to_owned(), Scalar::from_string(value.trim())));
        
    //     result
    // }
}
