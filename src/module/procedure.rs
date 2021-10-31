use std::{borrow::Cow};

use crate::reexports::*;

use super::*;

#[derive(Debug)]
pub struct Procedure {
    pub name: Cow<'static, str>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) frame: Id<Struct>,
}

#[derive(Clone)]
pub struct ProcedureBuilder {
    pub name: Cow<'static, str>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) locals: Named<Local>,
}

impl Procedure {
    pub(crate) fn placeholder(name: &Identifier) -> Procedure {
        Procedure { name: name.clone(), instructions: vec![], frame: Id::id_min_value(), }
    }
}

impl ProcedureBuilder {
    pub(crate) fn new(name: &Identifier) -> ProcedureBuilder {
        ProcedureBuilder {
            name: name.clone(),
            instructions: vec![],
            locals: Named::new(),
        }
    }
    
    pub(super) fn build(self, frame: Id<Struct>) -> Procedure {
        Procedure {
            name: self.name,
            instructions: self.instructions,
            frame,
        }
    }

    pub(crate) fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub(crate) fn push_local(&mut self, name: &Identifier, ty: Id<Struct>) -> Id<Local> {
        self.locals.get_or_insert(name, || Local(ty))
    }
}