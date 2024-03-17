use scalar::Scalar;
use warehouse::Merchandise;

type Good = Vec<u8>;

pub enum Result {
    Added,
    Updated
}

#[derive(Debug, Clone)]
pub struct Packet(Vec<Option<Scalar>>);

impl Packet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, field: u8, scalar: Scalar) -> Result {
        let len = self.0.len();
        let field_as_usize = field as usize;
        let mut result = Result::Added;

        if field_as_usize >= len {
            self.0.resize_with(field_as_usize + 1, || None);
        }else {
            let o_scalar = &self.0[field_as_usize];

            if o_scalar.is_some() {
                result = Result::Updated
            }
        }

        self.0[field_as_usize] = Some(scalar);

        result
    }

    // pub fn remove(&mut self, field: u8) {
    //     self.0[field as usize] = None;
    // }

    pub fn get(&self, field: u8) -> Option<&Scalar> {
        if field as usize >= self.0.len() {
            None
        }else {
            self.0[field as usize].as_ref()
        }
    }
    
    pub fn collect(&mut self, o_numerals: Option<&[u8]>) -> Vec<(u8, Scalar)> {
        let mut result: Vec<(u8, Scalar)> = Vec::new();
        let mut default: Vec<Option<Scalar>> = Vec::new();

        std::mem::swap(&mut default, &mut self.0);

        if let Some(numerals) = o_numerals {
            for numeral in numerals {
                let numeral_as_usize = *numeral as usize;
                let o_scalar = default[numeral_as_usize].take();

                if let Some(scalar) = o_scalar {
                    result.push((*numeral, scalar))
                }
            }
        }else {
            for (numeral, o_scalar) in default.into_iter().enumerate() {
                if let Some(scalar) = o_scalar {
                    result.push((numeral as u8, scalar))
                }
            }
        }

        result
    }

    // pub fn to_pairs(&mut self) -> Vec<Pair>{
    //     let mut result: Vec<Pair> = Vec::new();

    //     let len = self.0.len();

    //     for index in 0..len {
    //         let o_scalar = self.space[index].take();

    //         if let Some(scalar) = o_scalar {
    //             let tup: Pair = (index as u8, scalar);

    //             result.push(tup)
    //         }
    //     }

    //     result
    // }

    // pub fn from_pairs(pairs: Vec<Pair>) -> Self {
    //     let len = pairs.len();
    //     let mut packet = Self::new();

    //     for index in 0..len {
    //         let (field, scalar) = pairs[index].clone();

    //         packet.add(field, scalar)
    //     }

    //     packet
    // }
}

impl Merchandise for Packet {
    fn to_good(&self) -> Good {
        let mut good: Good = Good::new();

        for index in 0..self.0.len() {
            let o_scalar = &self.0[index as usize];

            if let Some(scalar) = o_scalar {
                let scalar_as_bytes = scalar.as_bytes();
                let field = index as u8;
                let cumulative_length_as_u16: u16 = (scalar_as_bytes.len() + 1) as u16;
                let cumulative_length_as_bytes = cumulative_length_as_u16.to_le_bytes();

                good
                .extend(vec![
                    cumulative_length_as_bytes[0],
                    cumulative_length_as_bytes[1],
                    field
                ]);
                
                good.extend(scalar_as_bytes);
            }
        }

        good
    }

    fn from_good(good: Good) -> Packet {
        let mut packet: Packet = Packet::new();
        let mut pointer = 0;
        
        while pointer < good.len() {
            let len = u16::from_le_bytes([
                good[pointer],
                good[pointer + 1]
            ]);
            let field = good[pointer + 2];
            let scalar_start = pointer + 3;
            let scalar_end = pointer + 1 + (len as usize);
            let scalar = Scalar::from_bytes(good.get(scalar_start..=scalar_end).unwrap());
            packet.add(field, scalar);
            
            pointer = scalar_end + 1;
        }
        
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec() {
        let mut packet = Packet::new();

        packet.add(0, Scalar::Boolean(true));
        packet.add(1, Scalar::Text("Hello".to_owned()));
        packet.add(2, Scalar::Text("World".to_owned()));

        let good = packet.to_good();
        let decoded_packet = Packet::from_good(good.clone());
        
        assert_eq!(decoded_packet.get(0).unwrap(), packet.get(0).unwrap());
        assert_eq!(decoded_packet.get(1).unwrap(), packet.get(1).unwrap());
        assert_eq!(decoded_packet.get(2).unwrap(), packet.get(2).unwrap());
    }
}

