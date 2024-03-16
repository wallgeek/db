use std::collections::BTreeMap;
use crate::LinkedList;

pub struct BTree<K, V> {
    space: BTreeMap<K, LinkedList<V>>
}

impl<K: Ord, V: Eq + Copy> BTree<K, V>{
    pub fn new() -> Self {
        Self {
            space: BTreeMap::new()
        }
    }

    pub fn add(&mut self, key: K, value: V) {
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

    pub fn remove(&mut self, key: K, value: V) {
        let o_ll = self.space.get_mut(&key);

        if let Some(ll) = o_ll {
            ll.remove(value)
        }
    }

    pub fn collect(&self, key: K) -> Vec<V> {
        let o_ll = self.space.get(&key);

        if let Some(ll) = o_ll {
            ll.collect()
        }else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add(){
        let mut btree: BTree<u64, u32> = BTree::new();
        let key: u64 = 1;
        let values: Vec<u32> = vec![10, 20, 2, 3];

        for value in values.clone() {
            btree.add(key, value);
        }

        let get_values = btree.collect(key);

        assert_eq!(get_values.len(), values.len());

        btree.remove(key, 2);

        let get_values = btree.collect(key);

        assert_eq!(get_values.len(), values.len() - 1);
    }
}