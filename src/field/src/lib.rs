mod finfo;
use finfo::Finfo;
use warehouse::{Warehouse, Token, Inventory, Mode as WarehouseMode, SessionMode, SessionItem};
use std::collections::HashMap;

const ID_LITERAL: &str = "_id";
const FIELD_DATA_FILE_NAME: &str = "field";

#[derive(Debug, PartialEq)]
pub enum Mode {
    OnlyInventory,
    WithInventory
}

pub struct Field {
    warehouse: Warehouse<u8, Finfo>,
    token_book: Inventory<u8, Token<u8>>,
    literal_book: HashMap<String, u8>
}

impl Field {
    fn create_id(&mut self){
        // Start with creating id field if there is no data present.
        // Note: If there is any data then id field must be there.
        let has_id = self.literal_book.contains_key(ID_LITERAL);

        if has_id == false {
            let numeral = self.token_book.reserve(None);
            let finfo: Finfo = Finfo::new(numeral, ID_LITERAL);
            let token = self.warehouse.add(finfo).ok().unwrap();
    
            self.token_book.replace(numeral, token);
            self.literal_book.insert(ID_LITERAL.to_string(), numeral);
        }
    }

    fn initialize(&mut self) {
        self.warehouse.start_session(SessionMode::Initialize);
        
        let mut has_data: bool = true;
        
        while has_data {
            let session_items = self.warehouse.session_items();
            
            if session_items.len() == 0 {
                has_data = false
            }
            
            for session_item in session_items {
                match session_item {
                    SessionItem::WithToken(token, finfo) => {
                        let numeral = finfo.get_numeral();
                        
                        self.token_book.reserve(Some(numeral));
                        self.token_book.replace(numeral, token);
                        self.literal_book.insert(finfo.get_literal(), numeral);
                    },
                    _ => panic!()
                }
            }
        }
        
        self.warehouse.stop_session();
    }

    pub fn new(mode: Mode) -> Self{
        let mut warehouse: Warehouse<u8, Finfo> = Warehouse::new(WarehouseMode::Inventory);
        let token_book: Inventory<u8, Token<u8>> = Inventory::new();

        match mode {
            Mode::WithInventory => {
                warehouse = Warehouse::new(WarehouseMode::Both(FIELD_DATA_FILE_NAME.to_string()));
            },
            _ => {}
        }

        let mut s = Self {
            warehouse,
            token_book,
            literal_book: HashMap::new()
        };   

        if mode == Mode::WithInventory {
            s.initialize();
        }
        
        s.create_id();
        
        s    
    }

    pub fn get_literal(&mut self, numeral: u8) -> Option<String> {
        let token = self.token_book.get(numeral).unwrap();
        let finfo = self.warehouse.get(token).unwrap();
            
        Some(finfo.get_literal())
    }

    pub fn get_numeral(&self, literal: &str) -> Option<&u8> {
        self.literal_book.get(literal)
    }

    pub fn add(&mut self, literal: &str) -> u8{
        // check if literal already exists
        if let Some(numeral) = self.literal_book.get(literal) {
            *numeral  
        }else {
            let fid = self.token_book.reserve(None);
            let finfo: Finfo = Finfo::new(fid, literal);
            let token = self.warehouse.add(finfo).ok().unwrap();

            self.token_book.replace(fid, token);
            self.literal_book.insert(literal.to_string(), fid);

            fid
        }
    }

    pub fn update(&mut self, old_literal: &str, new_literal: &str){
        if old_literal == new_literal {
            panic!("Liteals are same")
        }

        let o_numeral = self.literal_book.remove(&old_literal.to_string());
        
        if let Some(numeral) = o_numeral {
            let updated_finfo: Finfo = Finfo::new(numeral, new_literal);
            let token = self.token_book.get(numeral).unwrap();
            
            let updated_token = self.warehouse.update(token, updated_finfo).ok().unwrap();
            self.literal_book.insert(new_literal.to_owned(), numeral);
            self.token_book.replace(numeral, updated_token);
        }else {
            panic!("Literal not found")
        }
    }

    // fn remove(&mut self, numeral: u8){
    //     let literal = self.get_literal(numeral).unwrap();
    //     let token = self.token_book.take(numeral).unwrap();
        
    //     self.literal_book.remove(&literal);
    //     self.warehouse.remove(token);
        
    // }

    pub fn get_total(&self) -> u8 {
        self.literal_book.len() as u8
    }
}