use ds::LinkedList;
use crate::CatalogueTrait;

pub struct Boolean<T>{
    t: LinkedList<T>,
    f: LinkedList<T>
}

impl<T: Eq + Copy> Boolean<T> {
    pub fn new() -> Self {
        Self {
            t: LinkedList::new(),
            f: LinkedList::new()
        }
    }
}

impl<T: Eq + Copy> CatalogueTrait<bool, T> for Boolean<T> {
    fn add(&mut self, key: bool, value: T) {
        if key == true {
            self.t.add(value)
        }else {
            self.f.add(value)
        }
    }

    fn remove(&mut self, key: bool, value: T) {
        if key == true {
            self.t.remove(value)
        }else {
            self.f.remove(value)
        }
    }

    fn read(&self, key: bool) -> Vec<T> {
        if key == true {
            self.t.collect()
        }else {
            self.f.collect()
        }
    }
}

