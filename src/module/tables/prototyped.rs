use crate::reexports::*;
use crate::module::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map::Entry;

#[derive(Clone)]
pub struct Prototyped<T, Builder> {
    pub(in crate::module) names: BTreeMap<Identifier, Id<T>>,
    pub(in crate::module) rev_names: BTreeMap<Id<T>, Identifier>,
    pub(in crate::module) in_progress: BTreeMap<Id<T>, Builder>,
    pub(in crate::module) sealed: BTreeSet<Id<T>>,
    pub(in crate::module) finalized: BTreeSet<Id<T>>,
    pub(in crate::module) data: RawPom<T>,
}

impl<T, Builder> Prototyped<T, Builder> {
    pub fn new() -> Prototyped<T, Builder> {
        Prototyped {
            names: BTreeMap::new(),
            rev_names: BTreeMap::new(),
            in_progress: BTreeMap::new(),
            sealed: BTreeSet::new(),
            finalized: BTreeSet::new(),
            data: RawPom::new(),
        }
    }

    pub fn reference(&mut self, identifier: &Identifier, default: impl Fn(&Identifier) -> T) -> Id<T> {
        if let Some(id) = self.names.get(identifier) {
            return *id;
        }
        let id = self.data.insert(default(identifier));
        self.names.insert(identifier.clone(), id);
        self.rev_names.insert(id, identifier.clone());
        id
    }

    pub fn mutate(&mut self, id: Id<T>, default_builder: impl Fn(&Identifier) -> Builder) -> Option<&mut Builder> {
        if self.sealed.contains(&id) { return None; }

        match self.in_progress.entry(id) {
            Entry::Vacant(v) => {
                let name = self.rev_names.get(&id).unwrap();
                v.insert(default_builder(name));
            }
            Entry::Occupied(_) => { }
        };
        return self.in_progress.get_mut(&id)
    }

    pub fn seal(&mut self, id: Id<T>) {
        self.sealed.insert(id);
    }

    pub fn finalize(&mut self, id: Id<T>) {
        self.sealed.insert(id);
        self.finalized.insert(id);
    }
}