#![feature(const_type_id)]
use std::{rc::Rc};

use crate::reexports::*;

mod isa;
mod heapstack;
mod layout;
mod module;
mod reexports;
mod vm;

fn main() {
    let mut frame = RuntimeStructBuilder::new();
    frame.push(Rc::new(RuntimeStruct::wrap(RuntimeType::VINT)));

    let frame = Rc::new(frame.build());
    let main: Rc<Procedure> = Rc::new(Procedure {
        frame,
        instructions: vec![
            Instruction::Push(ISAVariant::VInt(48)),
            Instruction::RefLocal(Local(0)),
            Instruction::Write,
            Instruction::RefLocal(Local(0)),
            Instruction::Read,
            Instruction::Push(ISAVariant::VInt(4)),
            Instruction::Div2,
            Instruction::Print,
            Instruction::Push(ISAVariant::VProc(Rc::new(Procedure {
                frame: Rc::new(RuntimeStructBuilder::new().build()),
                instructions: vec![
                    Instruction::Push(ISAVariant::VInt(5)),
                    Instruction::Print,
                    Instruction::Ret,
                ]
            }))),
            Instruction::Call,
            Instruction::Ret,
        ]
    });

    let mut main_thread = Thread::spawn(main); 
    
    loop {
        if !main_thread.step() {
            break;
        }
    }
}
