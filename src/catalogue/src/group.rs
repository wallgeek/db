use crate::def::CatalogueTrait;
use crate::boolean::Boolean;
use crate::text::Text;
use crate::integer::Integer;
use scalar::Scalar;

enum Individual<T> {
    Boolean(Boolean<T>),
    Text(Text<T>),
    Integer(Integer<isize, T>)
}

pub struct Group<T>([Individual<T>; 3]);

impl<T: Eq + Copy> Group<T> {
    pub fn new() -> Self {
        Self([
            Individual::Boolean(Boolean::new()),
            Individual::Text(Text::new()),
            Individual::Integer(Integer::new())
        ])
    }

    pub fn add(&mut self, key: Scalar, value: T){
        match key {
            Scalar::Boolean(k) => {
                let index_value = &mut self.0[0];
                
                match index_value {
                    Individual::Boolean(v) => {
                        v.add(k, value)
                    },
                    _ => {}
                }
            },
            Scalar::Text(k) => {
                let index_value = &mut self.0[1];
                
                match index_value {
                    Individual::Text(v) => {
                        v.add(k, value)
                    },
                    _ => {}
                }
            },
            Scalar::Integer(k) => {
                let index_value = &mut self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.add(k.unwrap(), value)
                    },
                    _ => {}
                }
            },
            Scalar::Decimal(k) => {
                let index_value = &mut self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.add(k.to_integer(), value)
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn remove(&mut self, key: Scalar, value: T){
        match key {
            Scalar::Boolean(k) => {
                let index_value = &mut self.0[0];
                
                match index_value {
                    Individual::Boolean(v) => {
                        v.remove(k, value)
                    },
                    _ => {}
                }
            },
            Scalar::Text(k) => {
                let index_value = &mut self.0[1];
                
                match index_value {
                    Individual::Text(v) => {
                        v.remove(k, value)
                    },
                    _ => {}
                }
            },
            Scalar::Integer(k) => {
                let index_value = &mut self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.remove(k.unwrap(), value)
                    },
                    _ => {}
                }
            },
            Scalar::Decimal(k) => {
                let index_value = &mut self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.remove(k.to_integer(), value)
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn read(&self, key: Scalar) -> Vec<T>{
        match key {
            Scalar::Boolean(k) => {
                let index_value = &self.0[0];
                
                match index_value {
                    Individual::Boolean(v) => {
                        v.read(k)
                    },
                    _ => {
                        vec![]
                    }
                }
            },
            Scalar::Text(k) => {
                let index_value = &self.0[1];
                
                match index_value {
                    Individual::Text(v) => {
                        v.read(k)
                    },
                    _ => {
                        vec![]
                    }
                }
            },
            Scalar::Integer(k) => {
                let index_value = &self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.read(k.unwrap())
                    },
                    _ => {
                        vec![]
                    }
                }
            },
            Scalar::Decimal(k) => {
                let index_value = &self.0[2];
                
                match index_value {
                    Individual::Integer(v) => {
                        v.read(k.to_integer())
                    },
                    _ => {
                        vec![]
                    }
                }
            }
        }
    }
}