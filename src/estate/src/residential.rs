use std::collections::HashSet;
use ds::Stack;
use crate::def::WholeNumber;

pub struct Residential<Address> {
    size: usize,
    pointer: Address,
    retained: Stack<Address>,
    retained_verify: HashSet<Address>
}

impl<Address: WholeNumber> Residential<Address> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Residentail can't be intialize with size zero")
        }

        Self {
            size,
            pointer: Default::default(),
            retained: Stack::new(),
            retained_verify: HashSet::new()
        }
    }

    pub fn allot(&mut self) -> Address {
        if self.retained.is_empty() {
            let pointer = self.pointer;
            let location = self.pointer.to_usize() * self.size;

            if location + self.size > Address::get_max().to_usize() {
                panic!("Allot overflow")
            }

            self.pointer = self.pointer.add_int(1);

            return pointer
        }else {
            let pointer = self.retained.take().unwrap();
            self.retained_verify.remove(&pointer);

            return pointer
        }
    }

    pub fn retain(&mut self, address: Address) {
        if address >= self.pointer {
            panic!("Retained address is wrong")
        }else if self.retained_verify.contains(&address) {
            panic!("The address has already been retained")
        }
        // else if address.add_int(1) == self.pointer {
        //     self.pointer = self.pointer.add_int(-1);
        // }
        else {
            self.retained.add(address);
            self.retained_verify.insert(address);
        }
    }

    pub fn get_location(&self, address: Address) -> Option<u64> {
        if address >= self.pointer || self.retained_verify.contains(&address) {
            None
        }else {
            Some((address.to_usize() * self.size) as u64)
        }
    }

    pub fn register(&mut self, location: usize) -> Address {
        let pointer_location_as_usize = self.pointer.to_usize();
        
        if location == pointer_location_as_usize {
            let address = self.pointer;
            self.pointer = self.pointer.add_int(self.size as i32);

            address
        }else {
            panic!("Location should be equal to last location and size")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random() -> usize{
        rand::thread_rng().gen_range(1..=30) as usize
    }

    mod allot {
        use super::*;

        #[test]
        #[should_panic]
        fn size_0(){
            Residential::<u8>::new(0);
        }

        #[test]
        #[should_panic]
        fn overflow(){
            let mut residential: Residential<u8> = Residential::new(256);

            residential.allot();
            residential.allot();
        }

        #[test]
        fn alloted_normally(){
            let mut residential: Residential<u32> = Residential::new(random());
            let max = 100;
            
            for index in 0..=max {
                let address = residential.allot();
                assert_eq!(address, index)
            }
        }
    }

    mod retain {
        use  super::*;

        #[test]
        #[should_panic]
        fn wrong_address(){
            let mut residential: Residential<u8> = Residential::new(random());

            let address = residential.allot();
            residential.retain(address + 1);
        }

        #[test]
        #[should_panic]
        fn address_twice(){
            let mut residential: Residential<u8> = Residential::new(random());

            let address = residential.allot();
            residential.retain(address);
            residential.retain(address);
        }

        // #[test]
        // fn last_retain(){
        //     // So when a last alloted is retained
        //     // It will not keep that in retained
        //     // It will just reomve from the alloted
            
        //     let mut residential: Residential<u8> = Residential::new(random());
            
        //     residential.allot();

        //     let address1 = residential.allot();

        //     residential.allot();

        //     // Allot with samw size and this will be the last
        //     let address2 = residential.allot();

        //     // Retain both size
        //     residential.retain(address1);
        //     residential.retain(address2);

        //     // Allot again with same size
        //     let address3 = residential.allot();

        //     // Address1 and address3 should be same
        //     assert_eq!(address1, address3)
        // }
    }
    
    mod location {
        use super::*;

        #[test]
        fn wrong_address(){
            let mut residential: Residential<u8> = Residential::new(random());

            let address = residential.allot();

            let location = residential.get_location(address + 1);

            assert_eq!(location, None)
        }

        #[test]
        fn retained_address(){
            let mut residential: Residential<u8> = Residential::new(random());
            
            residential.allot();
            let address = residential.allot();
            residential.allot();

            residential.retain(address);

            let location = residential.get_location(address);

            assert_eq!(location, None)
        }
    }

    mod register {
        use super::*;

        #[test]
        #[should_panic]
        fn register_wrong_address(){
            let mut residential: Residential<u8> = Residential::new(random());

            residential.allot();
            residential.register(0);
        }

        #[test]
        #[should_panic]
        fn location_out_of_bound(){
            let mut residential: Residential<u8> = Residential::new(random());

            residential.register(300);
        }

        #[test]
        fn successful(){
            let random = random();
            let mut residential: Residential<u8> = Residential::new(random);

            residential.register(0);
            residential.retain(0);
            residential.register(random);
        }
    }
}