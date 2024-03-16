use crate::def::{ ParseError, MAX_SCALE };

pub struct Decimal{
    pub left_count: usize,
    pub right_count: usize
}

impl Decimal {
    pub fn new(left_count: usize, right_count: usize) -> Self {
        Self {
            left_count,
            right_count
        }
    }
    pub fn to_string(&self, mut value: isize) -> String {
        // All the operation will be done on +ve value
        // Any negative value will be conversted to +ve
        // Keep track if the number is negative
        let mut is_negative = false;
        
        if value < 0 {
            is_negative = true;
            value = value * -1;
        }
    
        // calculate leading zeros and reduce value
        let mut leading_zeros = 0;
        
        while leading_zeros < self.right_count && value > 0 && value % 10 == 0 {
            leading_zeros += 1;
            value /= 10;
        }
    
        if value == 0 {
            leading_zeros = self.right_count;
        }
    
        // Calculate effective right count keeping leading zeros in account
        let effective_right_count = self.right_count - leading_zeros;
    
        // get the right max. Will be later used to compare
        let mut right_max: isize = 0;
    
        for _ in 0..effective_right_count {
            right_max = 9 + (right_max * 10);
        }
    
        let mut s = value.to_string();
    
        if effective_right_count == 0 {
            s.push_str(".0");
        }else if value > right_max {
            s.insert(s.len() - (effective_right_count as usize), '.');
        }else {
            while value * 10 < right_max {
                s.insert(0, '0');
                value *= 10;
            }
    
            s.insert_str(0, "0.")
        }
        
        if is_negative {
            s.insert(0, '-');
        }
    
        s
    }

    pub fn from_string(&self, s: String) -> Result<isize, ParseError> {
        // unimplemented!()
        let split: Vec<&str> = s.split('.').collect();
        
        if split.len() != 2 {
            return Result::Err(ParseError::NotValidDecimal);
        }

        let left = split[0];
        let right = split[1];
        let right_len = right.len();

        if left.len() <= self.left_count && right_len <= self.right_count {
            let ok_left_num = left.parse::<isize>();
            let ok_right_num = right.parse::<isize>();

            if ok_left_num.is_ok() && ok_right_num.is_ok() {
                let left_num = ok_left_num.unwrap();
                let right_num = ok_right_num.unwrap();
                let base: usize = 10;
                let mutliplier = base.pow(self.right_count as u32) as isize;
                let right_multiplier = base.pow((self.right_count - right_len) as u32) as isize;
                
                Result::Ok((left_num * mutliplier) + (right_num * right_multiplier))
            }else {
                Result::Err(ParseError::NotValidNumerics)
            }
        }else {
            Result::Err(ParseError::OutOfBound)
        }
    }

    pub fn to_isize(&self, value: isize) -> isize {
        let base: usize = 10;
        let multiplier = base.pow((MAX_SCALE - self.right_count) as u32); 
        value * (multiplier as isize)
    }
}

