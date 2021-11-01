use std::borrow::Cow;

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
    // TODO: Don't panic here
    pub(crate) fn resolve_procedure(&self, procedure: ZId<Procedure>) -> &Procedure {
        &self.procedures.data[procedure.0]
    }

    pub(crate) fn resolve_struct(&self, struct_: ZId<Struct>) -> &Struct {
        &self.structs.data[struct_.0]
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
    pub fn procedure(&mut self, identifier: &Identifier) -> ZId<Procedure> {
        self.procedures.reference(identifier)
    }

    fn mut_procedure(&mut self, id: ZId<Procedure>) -> Option<&mut ProcedureBuilder> {
        self.procedures.mutate(id, ProcedureBuilder::new)
    }

    pub fn seal_procedure(&mut self, id: ZId<Procedure>) {
        // TODO: Panic on double-finalize? Probably just ignore.
        self.procedures.seal(id);
    }

    pub fn local(&mut self, id: ZId<Procedure>, name: &Identifier, ty: ZId<Struct>) -> ZId<Local> {
        if let Some(mp) = self.mut_procedure(id) {
            mp.push_local(name, ty)
        } else {
            panic!("can't edit procedure")
        }
    }

    pub fn push_instruction(&mut self, id: ZId<Procedure>, instruction: Instruction) {
        if let Some(mp) = self.mut_procedure(id) {
            mp.push_instruction(instruction)
        } else {
            panic!("can't edit procedure")
        }
    }

    // == structures ==
    pub fn structure(&mut self, identifier: &Identifier) -> ZId<Struct> {
        self.structs.reference(identifier)
    }

    fn mut_struct(&mut self, id: ZId<Struct>) -> Option<&mut StructBuilder> {
        self.structs.mutate(id, StructBuilder::new)
    }

    pub fn seal_structure(&mut self, id: ZId<Struct>) {
        // TODO: Panic on double-finalize? Probably just ignore.
        self.structs.seal(id);
    }

    pub fn push_field(&mut self, id: ZId<Struct>, field: ZId<Struct>) {
        if let Some(ms) = self.mut_struct(id) {
            ms.push(field)
        } else {
            panic!("can't edit struct")
        }
    }

    // == primitives ==
    pub fn primitive(&mut self, identifier: &Identifier, primitive: impl Fn() -> Primitive) -> ZId<Struct> {
        // NYEO NOTE: This function breaks encapsulation in a few ways and should be changed somehow
        // more args?
        let id = self.structs.reference(identifier);
        if self.structs.is_populated(id) {
            // TODO: Assert that the struct _is_ a primitive for this primitive type?
            return id;
        }

        assert!(!self.primitives.names.contains_key(identifier));
        let prim = primitive();
        let prim_id = self.primitives.get_or_insert(identifier, || prim);
        let struct_id = self.structs.reference(identifier);
        self.structs.inject(struct_id, Struct::wrap(identifier.clone(), prim_id, prim), |x, y| x == y);
        struct_id
    }
}
