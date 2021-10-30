use std::rc::Rc;

use crate::reexports::*;

#[derive(Debug)]
pub struct Procedure {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) frame: Rc<RuntimeStruct>,
}