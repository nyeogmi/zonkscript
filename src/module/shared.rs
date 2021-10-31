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

pub type Identifier = Cow<'static, str>;

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
