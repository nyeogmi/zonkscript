#![feature(const_type_id)]

use std::{borrow::Cow, rc::Rc};

#[allow(unused_imports)]
use crate::reexports::*;

mod isa;
mod heapstack;
mod module;
mod reexports;
mod vm;

fn main() {
    let mut module = ModuleBuilder::new();
    let main = module.procedure(&Cow::Borrowed("main"));

    let loc_type = module.datatype(&Cow::Borrowed("i64"));
    let loc0 = module.local(main, &Cow::Borrowed("a"), loc_type);

    for inst in [
        Instruction::Push(ISAVariant::VInt(48)),
        Instruction::RefLocal(loc0),
        Instruction::Write,
        Instruction::RefLocal(loc0),
        Instruction::Read,
        Instruction::Push(ISAVariant::VInt(4)),
        Instruction::Div2,
        Instruction::Print,
        Instruction::Ret,
    ] {
        module.push_instruction(main, inst);
    }

    module.seal_procedure(main);

    let module = Rc::new(module.build());

    let mut main_thread = Thread::spawn(module, main); 
    
    loop {
        if !main_thread.step() {
            break;
        }
    }
    /*
    let mut frame = StructBuilder::new();
    frame.push(Rc::new(Struct::wrap(RuntimeType::VINT)));

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
                frame: Rc::new(StructBuilder::new().build()),
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
    */
}
