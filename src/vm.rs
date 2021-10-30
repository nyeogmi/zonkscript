use std::{mem, rc::Rc};

use crate::reexports::*;

pub struct Thread {
    heap_stack: HeapStack,
    stack: Vec<StackFrame>,
}

struct StackFrame {
    // TODO: Variables shouldn't be memcpyed
    // We should just keep a stack pointer and a stack layout pointer
    stack: Vec<StackVariant>,
    code: Rc<Procedure>,

    sp: HeapStackRef,
    ip: usize,
}

#[derive(Clone, Debug)]
enum StackVariant {
    VRef(Rc<RuntimeStruct>, HeapStackRef),  // NOTE: This is GONZO INEFFICIENT
    VInt(i64), VFloat(f64),
    VProc(Rc<Procedure>),
}

impl Thread {
    pub(crate) fn spawn(entry_point: Rc<Procedure>) -> Thread {
        // TODO: Check type of entry point and make sure it can run with no args
        let mut thr = Thread {
            heap_stack: HeapStack::new(),
            stack: vec![], 
        };
        thr.stack.push(StackFrame::new_on(&mut thr.heap_stack, entry_point));
        thr
    }

    pub(crate) fn step(&mut self) -> bool {
        if let Some(active) = self.stack.pop() {
            match active.step(&mut self.heap_stack) {
                // TODO: Return value
                StackFrameSuccessor::Return => {}
                StackFrameSuccessor::Continue(x) => { 
                    self.stack.push(x) 
                }
                StackFrameSuccessor::Descend(x, y) => { 
                    self.stack.push(x);
                    self.stack.push(y) 
                }
            }
            return true
        } else {
            return false
        }
    }
}

enum StackFrameSuccessor {
    Return,  // TODO: Variant
    Continue(StackFrame),
    Descend(StackFrame, StackFrame),
}

impl StackFrame {
    fn new_on(hs: &mut HeapStack, code: Rc<Procedure>) -> StackFrame {
        let sp = hs.stack_alloc(&code.frame);
        StackFrame {
            stack: Vec::new(),
            code,

            sp,
            ip: 0,
        }
    }

    fn step(mut self, hs: &mut HeapStack) -> StackFrameSuccessor {
        let instructions = &self.code.instructions;
        assert!((0..instructions.len()).contains(&self.ip));
        let old_ip = self.ip;
        self.ip += 1;

        match &instructions[old_ip] {
            Instruction::Push(v) => { 
                let v2 = match v {
                    ISAVariant::VProc(va) => StackVariant::VProc(va.clone()),
                    ISAVariant::VFloat(vf) => StackVariant::VFloat(*vf),
                    ISAVariant::VInt(vi) => StackVariant::VInt(*vi),
                };
                self.stack.push(v2);
            }
            Instruction::RefLocal(x) => {
                let field_type = &self.code.frame.fields[x.0].1;
                let field = unsafe {
                    hs.stack_access_field(self.sp, &self.code.frame, x.0)
                };
                
                self.stack.push(StackVariant::VRef(field_type.clone(), field));
            }
            Instruction::Write => {
                let reference = self.stack.pop().unwrap();
                let stack_top = self.stack.pop().unwrap();
                let (ty, reference) = if let StackVariant::VRef(ty, reference) = reference {
                    (ty, reference)
                } else {
                    panic!("can't write a non-reference");
                };

                // TODO: Consider type of `ty`
                match (stack_top, ty) {
                    (StackVariant::VFloat(vf), ty) => {
                        unsafe {
                            let ptr: *mut u8 = hs.stack_access_primitive(reference, &ty);
                            let float: *mut f64 = mem::transmute(ptr);
                            *float = vf;
                        }
                    }
                    (StackVariant::VInt(vi), ty) => {
                        unsafe {
                            let ptr: *mut u8 = hs.stack_access_primitive(reference, &ty);
                            let float: *mut i64 = mem::transmute(ptr);
                            *float = vi;
                        }
                    }
                    (StackVariant::VProc(_), _) | (StackVariant::VRef(_, _), _) => {
                        // TODO: Better message
                        panic!("can't write that!!!");
                    }
                }

            }
            Instruction::Read => {
                let reference = self.stack.pop().unwrap();
                let (ty, reference) = if let StackVariant::VRef(ty, reference) = reference {
                    (ty, reference)
                } else {
                    panic!("can't read a non-reference");
                };

                // TODO: Consider type of `ty`
                if ty.fields.len() != 0 {
                    panic!("can't read a non-primitive (got: {:?})", ty);
                }

                let field = ty.single_fields[0];

                let variant = if field.type_data == RuntimeType::VINT {
                    unsafe {
                        let ptr: *mut u8 = hs.stack_access_primitive(reference, &ty);
                        let int: *mut i64 = mem::transmute(ptr);
                        StackVariant::VInt(*int)
                    }
                } 
                else if field.type_data == RuntimeType::VFLOAT {
                    unsafe {
                        let ptr: *mut u8 = hs.stack_access_primitive(reference, &ty);
                        let float: *mut f64 = mem::transmute(ptr);
                        StackVariant::VFloat(*float)
                    }
                }
                else {
                    panic!("can't read primitive of type: {:?}", field)
                };

                self.stack.push(variant)
            }

            Instruction::Ret => {
                return StackFrameSuccessor::Return
            }
            Instruction::Call => {
                let v = self.stack.pop().unwrap();
                match v {
                    StackVariant::VProc(addr) => { return StackFrameSuccessor::Descend(self, StackFrame::new_on(hs, addr)) }
                    v => { 
                        panic!("cannot call: {:?}", v)
                    }
                }
            }
            
            Instruction::Print => {
                println!("{:?}", self.stack.pop().unwrap());
            }

            Instruction::Add2 => {
                let (v1, v0) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                self.stack.push(arith(v0, v1, "add", |x, y| x + y, |x, y| x + y))
            }
            Instruction::Sub2 => {
                let (v1, v0) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                self.stack.push(arith(v0, v1, "subtract", |x, y| x - y, |x, y| x - y))
            }
            Instruction::Mul2 => {
                let (v1, v0) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                self.stack.push(arith(v0, v1, "multiply", |x, y| x * y, |x, y| x * y))
            }
            Instruction::Div2 => {
                let (v1, v0) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                self.stack.push(arith(v0, v1, "divide", |x, y| x / y, |x, y| x / y))
            }
        }
        StackFrameSuccessor::Continue(self)
    }
}

fn arith(
    v0: StackVariant, v1: StackVariant, 
    name: &'static str, 
    i: impl Fn(i64, i64) -> i64, 
    f: impl Fn(f64, f64) -> f64,
) -> StackVariant {
    match (v0, v1) {
        (StackVariant::VInt(i0), StackVariant::VInt(i1)) => StackVariant::VInt(i(i0, i1)),
        (StackVariant::VFloat(f0), StackVariant::VFloat(f1)) => StackVariant::VFloat(f(f0, f1)),
        // TODO: Coerce an int into a float if needed?

        (v0, v1) => panic!("can't {}: {:?} and {:?}", name, v0, v1),
    }
}