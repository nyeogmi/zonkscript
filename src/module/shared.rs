use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, btree_map::Entry}};

use moogle::{Id, RawPom};

use crate::reexports::*;

use super::*;

pub struct ModuleBuilder {
    pub(super) procedures: Prototyped<Procedure, ProcedureBuilder>, 
    pub(super) structs: Prototyped<Struct, StructBuilder>,
    pub(super) primitives: Named<Primitive>,
}

#[derive(Debug)]
pub struct Module {
    pub(super) procedures: Finalized<Procedure>,
    pub(super) structs: Finalized<Struct>,
    pub(super) primitives: Finalized<Primitive>,
    pub std_primitives: StdPrimitives,
}

#[derive(Clone)]
pub struct Prototyped<T, Builder> {
    pub(super) names: BTreeMap<Identifier, Id<T>>,
    pub(super) rev_names: BTreeMap<Id<T>, Identifier>,
    pub(super) in_progress: BTreeMap<Id<T>, Builder>,
    pub(super) sealed: BTreeSet<Id<T>>,
    pub(super) finalized: BTreeSet<Id<T>>,
    pub(super) data: RawPom<T>,
}

#[derive(Clone)]
pub struct Named<T> {
    pub(super) names: BTreeMap<Identifier, Id<T>>,
    pub(super) data: RawPom<T>,
}

pub type Identifier = Cow<'static, str>;

#[derive(Debug)]
pub struct Finalized<T> {
    pub(super) names: BTreeMap<Identifier, Id<T>>,
    pub(super) data: RawPom<T>,
}

impl Module {
    pub(crate) fn resolve_procedure(&self, procedure: Id<Procedure>) -> &Procedure {
        self.procedures.data.get(procedure).unwrap()
    }

    pub(crate) fn resolve_struct(&self, struct_: Id<Struct>) -> &Struct {
        self.structs.data.get(struct_).unwrap()
    }
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        let mut builder = ModuleBuilder {
            procedures: Prototyped::new(),
            structs: Prototyped::new(),
            primitives: Named::new(),
        };
        builder.add_std_primitives();

        builder
    }

    // == procedures ==
    pub fn procedure(&mut self, identifier: &Identifier) -> Id<Procedure> {
        self.procedures.reference(identifier, Procedure::placeholder)
    }

    fn mut_procedure(&mut self, id: Id<Procedure>) -> Option<&mut ProcedureBuilder> {
        self.procedures.mutate(id, ProcedureBuilder::new)
    }

    pub fn seal_procedure(&mut self, id: Id<Procedure>) {
        // TODO: Panic on double-finalize? Probably just ignore.
        self.procedures.seal(id);
    }

    pub fn local(&mut self, id: Id<Procedure>, name: &Identifier, ty: Id<Struct>) -> Id<Local> {
        if let Some(mp) = self.mut_procedure(id) {
            mp.push_local(name, ty)
        } else {
            panic!("can't edit procedure")
        }
    }

    pub fn push_instruction(&mut self, id: Id<Procedure>, instruction: Instruction) {
        if let Some(mp) = self.mut_procedure(id) {
            mp.push_instruction(instruction)
        } else {
            panic!("can't edit procedure")
        }
    }

    // == structures ==
    pub fn structure(&mut self, identifier: &Identifier) -> Id<Struct> {
        self.structs.reference(identifier, Struct::placeholder)
    }

    fn mut_struct(&mut self, id: Id<Struct>) -> Option<&mut StructBuilder> {
        self.structs.mutate(id, StructBuilder::new)
    }

    pub fn seal_structure(&mut self, id: Id<Struct>) {
        // TODO: Panic on double-finalize? Probably just ignore.
        self.structs.seal(id);
    }

    pub fn push_field(&mut self, id: Id<Struct>, field: Id<Struct>) {
        if let Some(ms) = self.mut_struct(id) {
            ms.push(field)
        } else {
            panic!("can't edit struct")
        }
    }

    // == primitives ==
    pub fn primitive(&mut self, identifier: &Identifier, primitive: impl Fn() -> Primitive) -> Id<Struct> {
        // NYEO NOTE: This function breaks encapsulation in a few ways and should be changed somehow
        // more args?
        if let Some(id) = self.structs.names.get(identifier) { 
            if self.structs.in_progress.contains_key(id) {
                panic!("trying to create primitive for _partway_-populated struct. bad!")
            } else if self.structs.finalized.contains(id) {
                // no need to do more work, this was already generated
                // TODO: Assert that the struct _is_ a primitive for this primitive type?
                return *id;
            }
        } 

        assert!(!self.primitives.names.contains_key(identifier));
        let prim = primitive();
        let prim_id = self.primitives.get_or_insert(identifier, || prim);
        let struct_id = self.structs.reference(identifier, |name| Struct::wrap(name.clone(), prim_id, prim));
        self.structs.finalize(struct_id);
        struct_id
    }
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