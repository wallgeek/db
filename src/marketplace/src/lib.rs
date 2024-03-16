mod serde;
mod parser;
use serde::Serde;
use vendor::Vendor;
use parser::{Parser, QueryType};

pub struct Marketplace(Vendor);

impl Marketplace {
    pub fn new() -> Self {
        Self(Vendor::new())
    }

    pub fn query(&mut self, bytes: &[u8]) -> String {
        let query = Parser::parse(bytes);
        
        match query {
            Err(err) => {
                return err.unwrap();
            },
            Ok(query_type) => {
                match &query_type[0] {
                    QueryType::Delete => {
                        match &query_type[1] {
                            QueryType::Match(condition) => {
                                let result = self.0.delete(condition.0.clone());
                                Serde::response(vec![vec![result]])
                            },
                            _ => panic!()
                        }
                    },
                    QueryType::Set(pairs) => {
                        match &query_type[1] {
                            QueryType::Match(condition) => {
                                let result = self.0.update(condition.0.clone(), Some(pairs.clone()));
        
                                match result {
                                    Ok(pair) => Serde::response(vec![vec![pair]]),
                                    Err(fumble) => fumble.unwrap()
                                }
                                
                            },
                            _ => panic!()
                        }
                    },
                    QueryType::Return(fields) => {
                        match &query_type[1] {
                            QueryType::Match(condition) => {
                                if fields.len() == 0{
                                    let data = self.0.search(condition.0.clone(), None);
                                    Serde::response(data)
                                }else {
                                    let data = self.0.search(condition.0.clone(), Some(fields.clone()));
                                    Serde::response(data)
                                }
                            },
                            _ => panic!()
                        }
                    },
                    QueryType::Create(pairs) => {
                        let result = self.0.create(pairs.clone());
        
                        match result {
                            Ok(pair) => Serde::response(vec![vec![pair]]),
                            Err(fumble) => fumble.unwrap()
                        }
                    },
                    _ => panic!()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut marketplace = Marketplace::new();

        // let q = marketplace.query(b"match\0name\0=\0#ajkdf\0delete\0");

        // println!("{:?}", q);
    }
}
