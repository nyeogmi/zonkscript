use crate::reexports::*;
use crate::module::*;

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Named<T> {
    pub(in crate::module) names: BTreeMap<Identifier, Id<T>>,
    pub(in crate::module) data: RawPom<T>,
}

impl<T> Named<T> {
    pub fn new() -> Named<T> {
        Named {
            names: BTreeMap::new(),
            data: RawPom::new(),
        }
    }

    pub fn get_or_insert(&mut self, identifier: &Identifier, factory: impl Fn() -> T) -> Id<T> {
        if let Some(id) = self.names.get(identifier) {
            return *id;
        }
        let id = self.data.insert(factory());
        self.names.insert(identifier.clone(), id);
        id
    }
}