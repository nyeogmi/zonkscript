use std::{borrow::Cow};

use crate::reexports::*;

use super::*;

#[derive(Debug)]
pub struct Procedure {
    pub name: Cow<'static, str>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) frame: ZId<DataType>,
}

#[derive(Clone)]
pub struct ProcedureBuilder {
    pub name: Cow<'static, str>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) locals: Named<Local>,
    pub frame_hint: Option<ZId<DataType>>,
}

impl ProcedureBuilder {
    pub(crate) fn new(name: &Identifier) -> ProcedureBuilder {
        ProcedureBuilder {
            name: name.clone(),
            instructions: vec![],
            locals: Named::new(),
            frame_hint: None,
        }
    }

    pub(crate) fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub(crate) fn push_local(&mut self, name: &Identifier, ty: ZId<DataType>) -> ZId<Local> {
        self.locals.get_or_insert(name, || Local(ty))
    }
}

impl Builds<Procedure> for ProcedureBuilder {
    fn build<'a>(self, _resolve: &mut impl FnMut(ZId<Procedure>, &mut dyn FnMut(&Procedure))) -> Procedure {
        // TODO: Don't panic here
        Procedure {
            name: self.name,
            instructions: self.instructions,
            frame: self.frame_hint.unwrap(),
        }
    }
}