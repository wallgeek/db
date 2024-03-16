
/// Farming estate is not constant in
/// size. So a token will be returned
/// whenever it is alloted and a
/// token is required to retain

use std::collections::HashMap;
use ds::Stack;
use crate::def::WholeNumber;

// Note: Obviousuly Location >= Size in terms of size
// Otherwise it may result in overflow error

type Location = usize;
type Size = usize;

#[derive(Clone, Copy)]
enum State {
    Occupied(Location),
    Retained(Location)
}

pub struct Farming <Address> {
    max: usize,
    locations: Vec<State>,
    retained: HashMap<Size, Stack<Address>>
}

impl<Address: WholeNumber> Farming<Address> {
    pub fn new() -> Self {
        Self {
            max: Address::get_max().to_usize(),
            locations: vec![State::Retained(Address::default().to_usize())],
            retained: HashMap::new()
        }
    }

    pub fn allot(&mut self, size: Size) -> Address {
        let len = self.locations.len();

        if size == 0 {
            panic!("Estate allot size can never be zero")
        }else if let State::Retained(last) = self.locations[len - 1] {
            if last + size > self.max {
                panic!("Farming overflow")
            }
        }

        let some_retained = self.retained.get_mut(&size);

        if let Some(retained) = some_retained {
            let address = retained.take().unwrap();
            let address_as_usize = address.to_usize();

            // delete if size is empty otherwise above unwrap will result in panic
            if retained.is_empty() {
                self.retained.remove(&size);
            }

            let state = self.locations[address_as_usize];

            match state {
                State::Retained(location) => {
                    self.locations[address_as_usize] = State::Occupied(location)
                },
                _ => panic!("Location showing occupied but it should be retained")
            }
            
            address
        }else {
            let s_location = self.locations[len - 1];
            
            if let State::Retained(location) = s_location {
                self.locations[len - 1] = State::Occupied(location);
                self.locations.push(State::Retained(location + size));
                
                Address::from_usize(len - 1)
            }else {
                panic!("If this happen, then stop being a coder please!")
            }
        }
    }

    pub fn retain(&mut self, address: Address) {
        let len = self.locations.len();
        let address_as_usize = address.to_usize();

        if address_as_usize >= len - 1 {
            panic!("Address is wrong")
        }
        // Note: The below code is removed as it was interfering
        // with register function. Very sad to comment the
        // below code
        // else if address_as_usize == len - 2 {
        //     self.locations.pop();
        // }
        else {   
            let state = self.locations[address_as_usize];

            match state {
                State::Retained(_) => panic!("Address is already retained"),
                State::Occupied(location) => {
                    let size = self.get_size(address).unwrap();
                    let has_size = self.retained.contains_key(&size);

                    if has_size {
                        self.retained.get_mut(&size).unwrap().add(address);
                    }else {
                        let mut stack = Stack::new();
                        stack.add(address);
                        self.retained.insert(size, stack);
                    }

                    self.locations[address_as_usize] = State::Retained(location);
                }
            }
        }
    }

    pub fn get_location(&self, address: Address) -> Option<Location> {
        let len = self.locations.len();
        let address_as_usize = address.to_usize();

        if address_as_usize < len - 1 {
            let state = self.locations[address_as_usize];

            match state {
                State::Occupied(value) => Some(value),
                State::Retained(_) => None
            }
        }else {
            None
        }
    }

    pub fn get_size(&self, address: Address) -> Option<Size> {
        let len = self.locations.len();
        let address_as_usize = address.to_usize();

        if address_as_usize < len - 1 {
            let state = self.locations[address_as_usize];
            let next_state = &self.locations[address_as_usize + 1];

            match state {
                State::Occupied(value) => {
                    match next_state {
                        State::Occupied(next_value) | State::Retained(next_value) => {
                            Some(next_value - value)
                        }
                    }
                },
                State::Retained(_) => {
                    None
                }
            }
        }else {
            None
        }    
    }

    pub fn register(&mut self, location: Location, size: Size) -> Address {
        let len = self.locations.len();
        let last_state = self.locations[len - 1];

        match last_state {
            State::Occupied(last) | State::Retained(last) => {
                if location == last {
                    self.locations[len - 1] = State::Occupied(last);
                    self.locations.push(State::Retained(location + size));
                    Address::from_usize(len - 1)
                }else {
                    panic!("Location should match the last estate location and size")
                }        
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random() -> usize{
        rand::thread_rng().gen_range(1..=50) as usize
    }

    mod allot {
        use super::*;

        #[test]
        #[should_panic]
        fn size_0(){
            let mut farming: Farming<u8> = Farming::new();

            farming.allot(0);
        }

        #[test]
        #[should_panic]
        fn overflow(){
            let mut farming: Farming<u8> = Farming::new();

            farming.allot(256);
        }

        #[test]
        fn alloted_normally(){
            let mut farming: Farming<u32> = Farming::new();
            let max = 100;
            
            for index in 0..=max {
                let address = farming.allot(random());
                assert_eq!(address, index)
            }
        }
    }

    mod retain {
        use super::*;

        #[test]
        #[should_panic]
        fn wrong_address(){
            let mut farming: Farming<u8> = Farming::new();

            farming.retain(1);
        }

        #[test]
        #[should_panic]
        fn address_twice(){
            let mut farming: Farming<u8> = Farming::new();

            let address = farming.allot(random());
            farming.retain(address);
            farming.retain(address);
        }

        // #[test]
        // fn last_retain(){
        //     // So when a last alloted is retained
        //     // It will not keep that in retained
        //     // It will just reomve from the alloted

        //     // So will create a allot 2 with same size
        //     // where one is last
        //     // We will retain both and when ask for 
        //     // allotement of samw size it will not return
        //     // the last.

        //     let mut farming: Farming<u8> = Farming::new();
        //     let size = random();
            
        //     // Allot random
        //     farming.allot(random());

        //     // Allot with given size
        //     let address1 = farming.allot(size);

        //     // Allot with different than size
        //     farming.allot(size + 1);

        //     // Allot with samw size and this will be the last
        //     let address2 = farming.allot(size);

        //     // Retain both size
        //     farming.retain(address1);
        //     farming.retain(address2);

        //     // Allot again with same size
        //     let address3 = farming.allot(size);

        //     // Address1 and address3 should be same
        //     assert_eq!(address1, address3)
        // }
    }

    mod location {
        use super::*;

        #[test]
        fn wrong_address(){
            let mut farming: Farming<u8> = Farming::new();

            let address = farming.allot(1);

            let location = farming.get_location(address + 1);

            assert_eq!(location, None)
        }

        #[test]
        fn retained_address(){
            let mut farming: Farming<u8> = Farming::new();

            let size1 = random();
            let size2 = random();
            let size3 = random();
            
            farming.allot(size1);
            let address = farming.allot(size2);
            farming.allot(size3);

            farming.retain(address);

            let location = farming.get_location(address);

            assert_eq!(location, None)
        }
    }

    mod size {
        use super::*;

        #[test]
        fn wrong_address(){
            let mut farming: Farming<u8> = Farming::new();

            let address = farming.allot(1);

            let size = farming.get_size(address + 1);

            assert_eq!(size, None)
        }
        
        #[test]
        fn retained_address(){
            let mut farming: Farming<u8> = Farming::new();

            let size1 = random();
            let size2 = random();
            let size3 = random();
            
            farming.allot(size1);
            let address = farming.allot(size2);
            farming.allot(size3);

            farming.retain(address);

            let size = farming.get_size(address);

            assert_eq!(size, None)
        }
    }

    mod register {
        use super::*;

        #[test]
        #[should_panic]
        fn wrong_address(){
            let mut farming: Farming<u8> = Farming::new();

            farming.allot(random());
            farming.register(0, 1);
        }

        #[test]
        #[should_panic]
        fn location_out_of_bound(){
            let mut farming: Farming<u8> = Farming::new();

            farming.register(300, 1);
        }

        #[test]
        fn successful(){
            let mut farming: Farming<u8> = Farming::new();

            farming.register(0, 3);
            farming.retain(0);
            farming.register(3, 2);
        }
    }
}