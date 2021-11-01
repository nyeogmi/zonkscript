use crate::module::*;

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Named<T> {
    pub(in crate::module) names: BTreeMap<Identifier, ZId<T>>,
    pub(in crate::module) data: Vec<T>,
}

impl<T> Named<T> {
    pub fn new() -> Named<T> {
        Named {
            names: BTreeMap::new(),
            data: Vec::new(),
        }
    }

    pub fn get_or_insert(&mut self, identifier: &Identifier, factory: impl Fn() -> T) -> ZId<T> {
        if let Some(id) = self.names.get(identifier) {
            return *id;
        }
        let id = ZId::new(self.data.len());
        self.data.push(factory());
        id
    }

    pub(crate) fn link(self) -> Finalized<T> {
        Finalized { names: self.names, data: self.data }
    }
}