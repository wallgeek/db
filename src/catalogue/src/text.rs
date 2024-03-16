use ds::LinkedList;
use std::collections::HashMap;
use crate::CatalogueTrait;

pub struct Text<T>(HashMap<String, LinkedList<T>>);

impl<T: Eq + Copy> Text<T> {
    pub fn new() -> Self {
        Self (HashMap::new())
    }
}

impl<T: Eq + Copy> CatalogueTrait<String, T> for Text<T> {
    fn add(&mut self, key: String, value: T) {
        let o_ll = self.0.get_mut(&key);

        if o_ll.is_none() {
            let mut ll = LinkedList::new();
            ll.add(value);
            self.0.insert(key.clone(), ll);
        }else if let Some(ll) = o_ll {
            ll.add(value);
        }
    }

    fn remove(&mut self, key: String, value: T) {
        let o_ll = self.0.get_mut(&key);

        if let Some(ll) = o_ll {
            ll.remove(value);

            if ll.is_empty() {
                self.0.remove(&key);
            }
        }
    }

    fn read(&self, key: String) -> Vec<T> {
        let o_ll = self.0.get(&key);

        if let Some(ll) = o_ll {
            ll.collect()
        }else {
            vec![]
        }
    }
}