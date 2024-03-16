mod def;
mod boolean;
mod integer;
mod text;
mod group;
use group::Group;
use scalar::Scalar;
use estate::WholeNumber;
use warehouse::Inventory;
use def::CatalogueTrait;

pub struct Catalogue<Field: WholeNumber, Identifier>(Inventory<Field, Group<Identifier>>);

impl<Field: WholeNumber, Identifier: Eq + Clone + Copy> Catalogue<Field, Identifier> {
    pub fn new() -> Self{
        Self(Inventory::new())
    }

    pub fn setup(&mut self, field: Field) {
        if self.0.get(field).is_some() {
            panic!("Field is already indexed")
        }

        self.0.reserve(Some(field));
        self.0.replace(field, Group::new())
    }

    pub fn has_index(&self, field: Field) -> bool {
        let o_catalogue = self.0.get(field);

        if o_catalogue.is_none() {
            false
        }else {
            true
        }
    }

    pub fn add(&mut self, field: Field, scalar: Scalar, meta: Identifier){
        let o_group = self.0.take(field);

        if let Some(mut group) = o_group {
            self.0.reserve(Some(field));
            group.add(scalar, meta);
            self.0.replace(field, group);
        }
    }

    pub fn remove(&mut self, field: Field, scalar: Scalar, meta: Identifier){
        let o_group = self.0.take(field);

        if let Some(mut group) = o_group {
            self.0.reserve(Some(field));
            group.remove(scalar, meta);
            self.0.replace(field, group);
        }
    }

    pub fn get(&self, field: Field, scalar: Scalar) -> Vec<Identifier> {
        let o_group = self.0.get(field);

        if let Some(group) = o_group {
            group.read(scalar)
        }else {
            vec![]
        }
    }
}