// TODO: Things below.
// - Allocate whole stack inline.
// - Variant should be Copy.
// - Composite data types.
// - Arguments for Call instruction.
// - Return values for Ret instruction.
// - Procedure ids for Instruction set (instead of just using Rc<Procedure> everywhere)
// - Instructions to make assertions about the stack.
// - Stack values should be ref-like.
//   Effectively: things in the stack (as opposed to variable slots) are &Xs, variable slots are non-fixed-size. Whoa!! 
//   This might be cool or might be terrible. It is very JavaScript Engine!
use std::{borrow::Cow, rc::Rc};

use crate::reexports::*;

mod isa;
mod module;
mod reexports;
mod vm;

fn main() {
    let main: Rc<Procedure> = Rc::new(Procedure {
        instructions: vec![
            Instruction::Push(Variant::VInt(48)),
            Instruction::Save(Var(Cow::Borrowed("a"))),
            Instruction::Load(Var(Cow::Borrowed("a"))),
            Instruction::Push(Variant::VInt(4)),
            Instruction::Div2,
            Instruction::Print,
            Instruction::Push(Variant::VAddress(Rc::new(Procedure {
                instructions: vec![
                    Instruction::Push(Variant::VInt(5)),
                    Instruction::Print,
                    Instruction::Ret,
                ]
            }))),
            Instruction::Call,
            Instruction::Ret,
        ]
    });

    let mut main_thread = Thread::spawn(main); 
    
    /*
    {
        stack: vec![StackFrame::new(main.clone())]
    };
    */

    loop {
        if !main_thread.step() {
            break;
        }
    }
}
