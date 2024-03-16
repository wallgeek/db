
mod inventory;
use estate::WholeNumber;
use fumble::Fumble;
use godown::{ Godown, Logistics, Address as GodownAddress, Item as LogisticsItem };
use primitive::Merchandise;
pub use godown::LogisticsMode as SessionMode;
pub use inventory::Inventory;

#[derive(PartialEq)]
pub enum Mode {
    Godown(String),
    Both(String),
    Inventory
}

pub enum SessionItem<Token, Item> {
    WithToken(Token, Item),
    WithoutToken(Item)
}

pub enum Token<Size> {
    Godown(GodownAddress),
    Inventory(Size),
    Both(GodownAddress, Size)
}


pub struct Warehouse<Size, Item>
where Size: WholeNumber, Item: Merchandise 
{
    o_godown: Option<Godown>,
    o_inventory: Option<Inventory<Size, Item>>,
    o_logistics: Option<Logistics>
}

impl<Size: WholeNumber, Item: Merchandise> Warehouse<Size, Item> {
    pub fn new(mode: Mode) -> Self {
        let mut o_godown: Option<Godown> = None;
        let mut o_inventory: Option<Inventory<Size, Item>> = None;
        
        match mode {
            Mode::Godown(file_path) => {
                println!("Starting with Godown only");
                o_godown = Some(Godown::new(file_path.as_str()));
            },
            Mode::Both(file_path) => {
                println!("Starting with Both");
                o_godown = Some(Godown::new(file_path.as_str()));
                o_inventory = Some(Inventory::new());
            },
            Mode::Inventory => {
                println!("Starting witn inventory only");
                o_inventory = Some(Inventory::new());
            }
        }

        Self {
            o_godown,
            o_inventory,
            o_logistics: None
        }
    }

    pub fn add(&mut self, item: Item) -> Result<Token<Size>, Fumble> {
        let mut o_godown_address: Option<GodownAddress> = None;
        let mut o_inventory_address: Option<Size> = None;

        if let Some(godown) = self.o_godown.as_mut() {
            let place_result = godown.place(item.to_good());

            if place_result.is_err() {
                return Err(place_result.err().unwrap());
            }else {
                o_godown_address = Some(place_result.ok().unwrap()); 

            }
        }
        
        if let Some(inventory) = self.o_inventory.as_mut() {
            o_inventory_address = Some(inventory.place(item));
        }

        if o_godown_address.is_some() && o_inventory_address.is_some() {
            Ok(Token::Both(o_godown_address.unwrap(), o_inventory_address.unwrap()))
        }else if o_godown_address.is_some() {
            Ok(Token::Godown(o_godown_address.unwrap()))
        }else {
            Ok(Token::Inventory(o_inventory_address.unwrap()))
        }
    }

    pub fn update(&mut self, token: &Token<Size>, item: Item) -> Result<Token<Size>, Fumble>{
        match token {
            Token::Both(old_godown_address, old_inventory_address) => {
                let mut o_godown_address: Option<GodownAddress> = None;
                let inventory_address = *old_inventory_address;

                if let Some(godown) = self.o_godown.as_mut() {
                    let replace_result = godown.replace(*old_godown_address, item.to_good());

                    if replace_result.is_err() {
                        return Err(replace_result.err().unwrap());
                    }else {
                        o_godown_address = Some(replace_result.ok().unwrap());   
                    }
                }
                
                if let Some(inventory) = self.o_inventory.as_mut() {
                    inventory.replace(inventory_address, item);
                }

                Ok(Token::Both(o_godown_address.unwrap(), inventory_address))
            },
            Token::Godown(old_godown_address) => {
                if let Some(godown) = self.o_godown.as_mut() {
                    let replace_result = godown.replace(*old_godown_address, item.to_good());

                    if replace_result.is_err() {
                        Err(replace_result.err().unwrap())
                    }else {
                        Ok(Token::Godown(replace_result.ok().unwrap()))
                    }
                }else {
                    panic!("Missing godown")
                }                
            },
            Token::Inventory(inventory_address) => {
                if let Some(inventory) = self.o_inventory.as_mut() {
                    inventory.replace(*inventory_address, item);
                    Ok(Token::Inventory(*inventory_address))
                }else {
                    panic!("Missing inventory")
                }
            }
        } 
    }

    pub fn remove(&mut self, token: Token<Size>){
        match  token {
            Token::Both(godown_address, inventory_address) => {
                if let Some(godown) = self.o_godown.as_mut() {
                    godown.remove(godown_address)   
                }
                
                if let Some(inventory) = self.o_inventory.as_mut() {
                    inventory.remove(inventory_address);
                }
            },
            Token::Godown(godown_address) => {
                if let Some(godown) = self.o_godown.as_mut() {
                    godown.remove(godown_address);     
                }
            },

            Token::Inventory(inventory_address) => {
                if let Some(inventory) = self.o_inventory.as_mut() {
                    inventory.remove(inventory_address);
                }
            }
        } 
    }

    pub fn get(&mut self, token: &Token<Size>) -> Option<Item>{
        match  token {
            Token::Both(_, inventory_address) => {
                if let Some(inventory) = self.o_inventory.as_mut() {
                    Some(inventory.get(*inventory_address).unwrap().clone())
                }else {
                    None
                }
            },
            Token::Godown(godown_address) => {
                if let Some(godown) = self.o_godown.as_mut() {
                    Some(Item::from_good(godown.get(*godown_address)))
                }else {
                    None
                }
            },

            Token::Inventory(inventory_address) => {
                if let Some(inventory) = self.o_inventory.as_mut() {
                    Some(inventory.get(*inventory_address).unwrap().clone())
                }else {
                    None
                }
            }
        }
    }

    pub fn start_session(&mut self, mode: SessionMode) {
        if self.o_godown.is_none() {
            panic!("Session cannot run in this Warehouse mode");
        }else if self.o_logistics.is_some() {
            panic!("Session is already running")
        }else{
            self.o_logistics = Some(Logistics::new(mode));
        }
    }

    pub fn session_items(&mut self) -> Vec<SessionItem<Token<Size>, Item>> {
        let mut session_items: Vec<SessionItem<Token<Size>, Item>> = Vec::new();
        
        if let Some(godown) = self.o_godown.as_mut() {
            if let Some(mut logistics) = self.o_logistics.as_mut() {
                godown.transfer_chunk(&mut logistics);
                // println!("Recursion is happening: {:?}", logistics.get_has_done());
                let logistics_items = logistics.unload();
                
                // It might be possible that no items received
                // May be due to all deleted item in this session
                // We may need to call again until has done is true
                if logistics_items.len() == 0 && logistics.get_has_done() {
                    return session_items;
                }else if logistics_items.len() == 0 {
                    return self.session_items();
                }else {
                    // deal with if both inventory and godown is present
                    // deal if only godown is present
                    match logistics.get_mode() {
                        SessionMode::Uninitialize => {
                            for logistics_item in logistics_items {
                                match logistics_item {
                                    LogisticsItem::WithoutAddress(good) => {
                                        let item = Item::from_good(good);
                                        session_items.push(SessionItem::WithoutToken(item));
                                    },
                                    _ => {}
                                }
                            }        
                        },
                        SessionMode::Initialize => {
                            for logistics_item in logistics_items {
                                match logistics_item {
                                    LogisticsItem::WithAddress(address, good) => {
                                        let item = Item::from_good(good);
                                        
                                        if let Some(inventory) = self.o_inventory.as_mut() {
                                            let inventory_item = item.clone();
                                            let inventory_address = inventory.place(inventory_item);

                                            session_items.push(SessionItem::WithToken(Token::Both(address, inventory_address), item))
                                        }else {
                                            session_items.push(SessionItem::WithToken(Token::Godown(address), item))
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                
            }else {
                panic!("Cannot find logistics")
            }
        }else {
            panic!("Can't use session without godown")
        }

        session_items
    }

    pub fn stop_session(&mut self) {
        if self.o_logistics.is_none() {
            panic!("Session is not running")
        }else{
            self.o_logistics = None;
        }
    }
}