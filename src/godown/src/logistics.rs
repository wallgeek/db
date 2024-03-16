use crate::def::Item;
type Poiner = usize;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Initialize,
    Uninitialize
}

pub struct Logistics {
    mode: Mode,
    pointer: Poiner,
    block_number: usize,
    items: Vec<Item>,
    has_done: bool
}

impl Logistics {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            pointer: Poiner::default(),
            block_number: 0,
            items: Vec::new(),
            has_done: false
        }
    }

    pub fn set_has_done(&mut self) {
        self.has_done = true;
    }

    pub fn get_has_done(&self) -> bool {
        self.has_done
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn set_block_number(&mut self, block_number: usize) {
        self.block_number = block_number;
    }

    pub fn get_block_number(&self) -> usize {
        self.block_number
    }

    pub fn get_pointer(&self) -> Poiner {
        self.pointer
    }

    pub fn set_pointer(&mut self, pointer: Poiner){
        self.pointer = pointer;
    }

    pub fn load_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn unload(&mut self) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();

        std::mem::swap(&mut items, &mut self.items);

        items
    }
}