use crate::reexports::*;

use std::{borrow::Cow, rc::Rc};

#[derive(Debug)]
pub enum Instruction {
    Push(Variant),
    Save(Var),
    Load(Var),

    Call,
    Ret,

    Print,
    Add2, Sub2, Mul2, Div2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Var(pub(crate) Cow<'static, str>);

#[derive(Clone, Debug)]
pub enum Variant {
    VInt(i64),
    VFloat(f64),
    VAddress(Rc<Procedure>)  // TODO: Literally any other representation holy shit
    // VString(String),
}