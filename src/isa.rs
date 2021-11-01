use crate::{module::ZId, reexports::*};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Instruction {
    Push(ISAVariant),
    RefLocal(ZId<Local>),
    Write,
    Read,

    Call,
    Ret,

    Print,
    Add2, Sub2, Mul2, Div2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Local(pub ZId<Struct>);  // field ix

#[derive(Clone, Debug)]
pub enum ISAVariant {
    VInt(i64),
    VFloat(f64),
    VProc(ZId<Procedure>)  
    // VString(String),
}