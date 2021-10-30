use crate::isa::Instruction;

#[derive(Debug)]
pub struct Procedure {
    pub(crate) instructions: Vec<Instruction>,
}