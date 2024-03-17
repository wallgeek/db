use estate::{Residential, WholeNumber};

#[derive(PartialEq)]
enum State<Item> {
    Taken(Item),
    Reserved,
    Empty
}

pub struct Inventory<Address, Item> where Address: WholeNumber {
    estate: Residential<Address>,
    storage: Vec<State<Item>>
}

impl<Address: WholeNumber, Item> Inventory<Address, Item> {
    pub fn new() -> Self {
        Self {
            estate: Residential::new(1),
            storage: Vec::new()
        }
    }

    pub fn place(&mut self, item: Item) -> Address {
        let address = self.estate.allot();
        let len = self.storage.len();
        let address_as_usize = address.to_usize();

        if address_as_usize > len {
            panic!("Address overflow")
        }else if address_as_usize == len {
            self.storage.push(State::Taken(item));
        }else if let State::Taken(_) = &self.storage[address_as_usize] {
            panic!("Storage at address is not empty");
        }else {
            self.storage[address_as_usize] = State::Taken(item);
        }
        
        address
    }

    pub fn replace(&mut self, address: Address, item: Item) {
        let len = self.storage.len();
        let address_as_usize = address.to_usize();

        if address_as_usize >= len {
            panic!("Wrong address")
        }else if let State::Empty = self.storage[address_as_usize] {
            panic!("Storage at address is empty");
        }else {
            self.storage[address_as_usize] = State::Taken(item);
        }
    }

    pub fn remove(&mut self, address: Address) {
        let len = self.storage.len();
        let address_as_usize = address.to_usize();

        if address_as_usize >= len {
            panic!{"Wrong address"}
        }

        self.estate.retain(address);
        self.storage[address_as_usize] = State::Empty;
    }

    pub fn get(&self, address: Address) -> Option<&Item> {
        let len = self.storage.len();
        let address_as_usize = address.to_usize();
        
        if address_as_usize >= len {
            return None;
        }

        match &self.storage[address_as_usize] {
            State::Taken(item) => Some(&item),
            _ => None
        }
    }

    pub fn take(&mut self, address: Address) -> Option<Item> {
        let len = self.storage.len();
        let address_as_usize = address.to_usize();

        if address_as_usize >= len {
            panic!{"Wrong address"}
        }

        let mut replace_state = State::Empty;
        
        std::mem::swap(&mut self.storage[address_as_usize], &mut replace_state);
        
        match replace_state {
            State::Taken(item) => Some(item),
            _ => None
        }
    }

    pub fn reserve(&mut self, o_specific_address: Option<Address>) -> Address {
        let len = self.storage.len();
        
        if let Some(specific_address) = o_specific_address {
            let specific_address_as_usize = specific_address.to_usize();

            if specific_address_as_usize < len { 
                let state = &self.storage[specific_address_as_usize];
                
                match state {
                    State::Empty => {
                        self.storage[specific_address_as_usize] = State::Reserved;

                        specific_address
                    },
                    _ => panic!("")
                }
            }else {
                for _ in len..=specific_address_as_usize {
                    self.estate.allot();
                    self.storage.push(State::Empty)
                }

                self.storage[specific_address_as_usize] = State::Reserved;
                
                specific_address
            }
        }else {
            let address = self.estate.allot();
            let address_as_usize = address.to_usize();
            
            if address_as_usize > len {
                panic!("Address overflow")
            }else if address_as_usize == len {
                self.storage.push(State::Reserved);
            }else if let State::Taken(_) = &self.storage[address_as_usize] {
                panic!("Storage at address is not empty");
            }else {
                self.storage[address_as_usize] = State::Reserved;
            }
            
            address
        }
    }
}