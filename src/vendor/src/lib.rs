mod packet;
use std::time::Instant;
use packet::Packet;
use field::{ Field, Mode as FieldMode };
use warehouse::{ Warehouse, Mode as WarehouseMode, Inventory, Token, SessionMode, SessionItem };
use scalar::{Scalar, Integer};
// use catalogue::Catalogue;
use fumble::Fumble;

type Pair = (String, Scalar);
const ID_LITERAL: &str = "_id";

pub struct Vendor {
    field: Field,
    warehouse: Warehouse<u32, Packet>,
    identifier: Inventory<u32, Token<u32>>,
    // catalogue: Catalogue<u8, u32>
}

impl Vendor {
    pub fn initialize(&mut self) {
        println!("Loading data...");
        let time = Instant::now();

        self.warehouse.start_session(SessionMode::Initialize);

        loop {
            let session_items = self.warehouse.session_items();

            if session_items.len() == 0 {
                break;
            }else {
                let numeral_id = *self.field.get_numeral(ID_LITERAL).unwrap();

                for session_item in session_items {
                    match session_item {
                        SessionItem::WithToken(token, packet) => {
                            let id = packet.get(numeral_id).unwrap().unwrap_integer() as u32;
                            
                            if self.identifier.get(id).is_some() {
                                // In case of crash this may happen
                                self.warehouse.remove(token)
                            }else {
                                self.identifier.reserve(Some(id));
                                self.identifier.replace(id, token);

                                // update indexes if any
                            }
                        },
                        _ => panic!("Not getting any token in initialize mode")
                    }
                }
            }
        }
        
        self.warehouse.stop_session();

        println!("Data Loaded in: {:?}", time.elapsed());
    }

    pub fn new() -> Self {
        let warehouse = Warehouse::new(WarehouseMode::Godown("data".to_owned()));
        let field = Field::new(FieldMode::WithInventory);

        let mut vendor = Self {
            field,
            warehouse,
            identifier: Inventory::new(),
            // catalogue: Catalogue::new()
        };

        vendor.initialize();
        
        vendor
    }

    fn get_by_id(&mut self, id: u32) -> Option<Packet> {
        let o_token = self.identifier.get(id);
        
        if let Some(token) = o_token {
            Some(self.warehouse.get(token).unwrap())
        }else {
            None
        }  
    }

    fn delete_by_id(&mut self, id: u32) -> bool {
        let o_token = self.identifier.take(id);

        if let Some(token) = o_token {
            self.warehouse.remove(token);

            true
        }else {
            false
        }   
    }

    pub fn search(&mut self, condition: (String, Scalar), o_fetch_fields: Option<Vec<String>>) -> Vec<Vec<Pair>>{
        let numeral_id = *self.field.get_numeral(ID_LITERAL).unwrap();
        let literal_condition = condition.0;
        let scalar_condition = condition.1;
        let o_numeral_condition = self.field.get_numeral(&literal_condition);
        let mut numeral_fetch_fields: Vec<u8> = Vec::new();
        let mut result: Vec<Vec<Pair>> = Vec::new();
        let mut packets: Vec<Packet> = Vec::new();

        if o_numeral_condition.is_none() {
            return result;
        }

        let numeral_condition = *o_numeral_condition.unwrap();

        // convert all literal fetch fields to numeral fields
        if let Some(fields) = o_fetch_fields {
            for literal in &fields {
                if let Some(numeral) = self.field.get_numeral(literal) {
                    numeral_fetch_fields.push(*numeral);    
                }
            }
        }

        if numeral_condition == numeral_id {
            let o_packet = self.get_by_id(scalar_condition.unwrap_integer() as u32);

            if let Some(packet) = o_packet {
                packets.push(packet)
            }
        }else {
            self.warehouse.start_session(SessionMode::Uninitialize);

            loop {
                let session_items = self.warehouse.session_items();

                if session_items.len() == 0 {
                    break;
                }

                for session_item in session_items {
                    match session_item {
                        SessionItem::WithoutToken(packet) => {
                            let o_scalar = packet.get(numeral_condition);

                            if let Some(s) = o_scalar {
                                if s == &scalar_condition {
                                    packets.push(packet);
                                }
                            }
                        },
                        _ => panic!()
                    }
                }

            }
            
            self.warehouse.stop_session();
        }

        for mut packet in packets {
            let numeral_pairs = packet.collect(None);
            let mut local_result: Vec<(String, Scalar)> = Vec::new();

            for numeral_pair in numeral_pairs {
                local_result.push((self.field.get_literal(numeral_pair.0).unwrap(), numeral_pair.1));    
            }

            result.push(local_result)
        }

        result
    }

    pub fn create(&mut self, pairs: Vec<(String, Scalar)>) -> Result<(String, Scalar), Fumble>{
        let mut packet = Packet::new();
        let id = self.identifier.reserve(None);
        let id_numeral = self.field.get_numeral(ID_LITERAL).unwrap();

        packet.add(*id_numeral, Scalar::Integer(Integer::new(id as isize)));

        for pair in pairs {
            let field = pair.0;
            let scalar = pair.1;
            let numeral = self.field.add(&field);
            
            packet.add(numeral, scalar);
        }
        
        let add_result = self.warehouse.add(packet);

        match add_result {
            Ok(token) => {
                self.identifier.replace(id, token);

                Ok((String::from(ID_LITERAL), Scalar::Text(id.to_string())))
            },
            Err(err) => Err(err)
        }
    }

    fn update_by_id(&mut self, id: u32, o_set: Option<&Vec<(String, Scalar)>>) -> Result<usize, Fumble> {
        // get token
        let o_token = self.identifier.get(id);

        match o_token {
            None => {
                return Ok(0)
            },
            Some(token) => {
                let mut packet = self.warehouse.get(&token).unwrap();
            
                if let Some(pairs) = o_set {
                    for pair in pairs {
                        let numeral = self.field.add(&pair.0);
        
                        packet.add(numeral, pair.1.clone());
                    }
                }
                
                let updated_result = self.warehouse.update(&token, packet);

                match updated_result {
                    Err(err) => {
                        return Err(err);
                    },
                    Ok(updated_token) => {
                        self.identifier.replace(id, updated_token);

                        Ok(1)
                    }
                }
            }
        }
    }

    pub fn update(&mut self, condition: (String, Scalar), o_set: Option<Vec<(String, Scalar)>>) -> Result<(String, Scalar), Fumble> {
        let condition_literal = condition.0;
        let condition_scalar = condition.1;
        let o_condition_numeral_field = self.field.get_numeral(&condition_literal);

        if o_condition_numeral_field.is_none() {
            return Ok(("updated".to_owned(), Scalar::Integer(Integer::new(0))));
        }else {
            let condition_numeral_field = *o_condition_numeral_field.unwrap();

            // get numeral id
            let numeral_id: u8 = *self.field.get_numeral(ID_LITERAL).unwrap();

            if condition_numeral_field == numeral_id {
                // get id value
                let id = condition_scalar.unwrap_integer() as u32;

                let result = self.update_by_id(id, o_set.as_ref());

                match result {
                    Err(err) => {
                        return Err(err)
                    },
                    Ok(count) => {
                        return Ok(("updated".to_owned(), Scalar::Integer(Integer::new(count as isize))));
                    }
                }
            }else {
                let mut total_updated = 0;

                self.warehouse.start_session(SessionMode::Uninitialize);

                loop {
                    let session_items = self.warehouse.session_items();
                    if session_items.len() == 0 {
                        break;
                    }

                    for session_item in session_items {
                        match session_item {
                            SessionItem::WithoutToken(packet) => {
                                let o_scalar = packet.get(condition_numeral_field);

                                if let Some(s) = o_scalar {
                                    if s == &condition_scalar {
                                        let id = packet.get(numeral_id).unwrap().unwrap_integer() as u32;
                                        let result = self.update_by_id(id, o_set.as_ref());

                                        match result {
                                            Err(err) => {
                                                return Err(err)
                                            },
                                            Ok(count) => {
                                                total_updated += count;    
                                            }
                                        }
                                    }
                                }
                            },
                            _ => panic!()
                        }
                    }
                }
                self.warehouse.stop_session();

                return Ok(("updated".to_owned(), Scalar::Integer(Integer::new(total_updated as isize))));
            }
        }   
    }

    pub fn delete(&mut self, condition: (String, Scalar)) -> (String, Scalar){
        let numeral_id = *self.field.get_numeral(ID_LITERAL).unwrap();
        let literal_condition = condition.0;
        let scalar_condition = condition.1;
        let o_numeral_condition = self.field.get_numeral(&literal_condition);
        let mut result: usize = 0;
        
        if o_numeral_condition.is_none() {
            return (String::from("deleted"), Scalar::Text(result.to_string()));
        }

        let numeral_condition = *o_numeral_condition.unwrap();

        if numeral_condition == numeral_id {
            let has_deleted = self.delete_by_id(scalar_condition.unwrap_integer() as u32);

            if has_deleted {
                result += 1;
            }
        }else {
            self.warehouse.start_session(SessionMode::Uninitialize);

            loop {
                let session_items = self.warehouse.session_items();

                if session_items.len() == 0 {
                    break;
                }

                for session_item in session_items {
                    match session_item {
                        SessionItem::WithoutToken(packet) => {
                            let o_scalar = packet.get(numeral_condition);

                            if let Some(s) = o_scalar {
                                if s == &scalar_condition {
                                    let has_deleted = self.delete_by_id(packet.get(numeral_id).unwrap().unwrap_integer() as u32);
                                    
                                    if has_deleted {
                                        result += 1;
                                    }

                                }
                            }
                        },
                        _ => panic!()
                    }
                }
            }
            self.warehouse.stop_session();
        }

        (String::from("deleted"), Scalar::Text(result.to_string()))
    }
}