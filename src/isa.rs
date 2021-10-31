use crate::reexports::*;

use std::{rc::Rc};

#[derive(Clone, Debug)]
pub enum Instruction {
    Push(ISAVariant),
    RefLocal(Id<Local>),
    Write,
    Read,

    Call,
    Ret,

    Print,
    Add2, Sub2, Mul2, Div2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Local(pub Id<Struct>);  // field ix

#[derive(Clone, Debug)]
pub enum ISAVariant {
    VInt(i64),
    VFloat(f64),
    VProc(Id<Procedure>)  
    // VString(String),
}