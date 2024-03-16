use primitive::Merchandise;
type Good = Vec<u8>;

#[derive(Clone)]
pub struct Finfo {
    numeral: u8,
    literal: String
}

impl Finfo {
    pub fn new<S: AsRef<str>>(numeral: u8, literal: S) -> Self {
        Self {
            numeral,
            literal: literal.as_ref().to_owned()
        }
    }

    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }

    pub fn get_numeral(&self) -> u8 {
        self.numeral
    }
}

impl Merchandise for Finfo {
    fn to_good(&self) -> Vec<u8> {
        let mut good: Good = Good::new();
        let literal_as_bytes = self.literal.as_bytes();
        let numeral = self.numeral;
        
        good.push(numeral);
        good.extend(literal_as_bytes);

        good
    }

    fn from_good(good: Vec<u8>) -> Self {
        let len = good.len();
        let numeral: u8 = good[0];
        let literal: String = String::from_utf8(good[1..len].to_vec()).unwrap();

        Self {
            numeral,
            literal
        }
    }
}