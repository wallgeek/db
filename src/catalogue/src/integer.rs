
use std::collections::BTreeMap;
use ds::LinkedList;
use crate::def::CatalogueTrait;

pub struct Integer<K, V> {
    space: BTreeMap<K, LinkedList<V>>
}

impl<K: Ord, V: Eq + Copy> Integer<K, V>{
    pub fn new() -> Self {
        Self {
            space: BTreeMap::new()
        }
    }
}

impl<K: Ord, V: Eq + Copy> CatalogueTrait<K, V> for Integer<K, V> {
    fn add(&mut self, key: K, value: V) {
        let has_key = self.space.contains_key(&key);

        if has_key == false {
            let mut ll: LinkedList<V> = LinkedList::new();
            ll.add(value);
            self.space.insert(key, ll);
        }else {
            let ll = self.space.get_mut(&key).unwrap();
            ll.add(value)
        }
    }

    fn remove(&mut self, key: K, value: V) {
        let o_ll = self.space.get_mut(&key);

        if let Some(ll) = o_ll {
            ll.remove(value)
        }
    }

    fn read(&self, key: K) -> Vec<V> {
        let o_ll = self.space.get(&key);

        if let Some(ll) = o_ll {
            ll.collect()
        }else {
            vec![]
        }
    }
}