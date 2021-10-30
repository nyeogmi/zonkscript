use crate::reexports::*;

use std::{rc::Rc};

#[derive(Debug)]
pub enum Instruction {
    Push(ISAVariant),
    RefLocal(Local),
    Write,
    Read,

    Call,
    Ret,

    Print,
    Add2, Sub2, Mul2, Div2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Local(pub usize);

#[derive(Clone, Debug)]
pub enum ISAVariant {
    VInt(i64),
    VFloat(f64),
    VProc(Rc<Procedure>)  // TODO: Literally any other representation holy shit
    // VString(String),
}