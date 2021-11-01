use std::{mem, rc::Rc};

use crate::{module::ZId, reexports::*};

pub struct Thread {
    globals: Globals,
    stack: Vec<StackFrame>,
}

struct Globals {
    module: Rc<Module>,
    heap_stack: HeapStack,
}

struct StackFrame {
    // TODO: Variables shouldn't be memcpyed
    // We should just keep a stack pointer and a stack layout pointer
    stack: Vec<StackVariant>,
    code: ZId<Procedure>,

    sp: HeapStackRef,
    ip: usize,
}

#[derive(Clone, Debug)]
enum StackVariant {
    VRef(ZId<DataType>, HeapStackRef),  // NOTE: This is GONZO INEFFICIENT
    VInt(i64), VFloat(f64),
    VProc(ZId<Procedure>),
}

impl Thread {
    pub(crate) fn spawn(module: Rc<Module>, entry_point: ZId<Procedure>) -> Thread {
        // TODO: Check type of entry point and make sure it can run with no args
        let mut thr = Thread {
            globals: Globals { module, heap_stack: HeapStack::new() },
            stack: vec![], 
        };
        thr.stack.push(StackFrame::new_on(&mut thr.globals, entry_point));
        thr
    }

    pub(crate) fn step(&mut self) -> bool {
        if let Some(active) = self.stack.pop() {
            match active.step(&mut self.globals) {
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
    fn new_on(globals: &mut Globals, code: ZId<Procedure>) -> StackFrame {
        let proc = globals.module.procedure(code);
        let sp = globals.heap_stack.stack_alloc((*globals.module).datatype(proc.frame));

        StackFrame {
            stack: Vec::new(),
            code,

            sp,
            ip: 0,
        }
    }

    fn step(mut self, globals: &mut Globals) -> StackFrameSuccessor {
        let code = globals.module.procedure(self.code);
        let frame = globals.module.datatype(code.frame);

        let instructions = &code.instructions;
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
                // NYEO NOTE: This -1 is to compensate for the 1-indexing in moogle
                // ... Objectively, it is terrible. Don't use moogle for this, probably!
                let field_type = frame.fields[x.0].1;
                let field = unsafe {
                    globals.heap_stack.stack_access_field(self.sp, frame, x.0)
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
                            let ptr: *mut u8 = globals.heap_stack.stack_access_primitive(reference, globals.module.datatype(ty));
                            let float: *mut f64 = mem::transmute(ptr);
                            *float = vf;
                        }
                    }
                    (StackVariant::VInt(vi), ty) => {
                        unsafe {
                            let ptr: *mut u8 = globals.heap_stack.stack_access_primitive(reference, globals.module.datatype(ty));
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
                let datatype = globals.module.datatype(ty);

                let variant = if ty == globals.module.std_primitives.v_int {
                    unsafe {
                        let ptr: *mut u8 = globals.heap_stack.stack_access_primitive(reference, datatype);
                        let int: *mut i64 = mem::transmute(ptr);
                        StackVariant::VInt(*int)
                    }
                } 
                else if ty == globals.module.std_primitives.v_float {
                    unsafe {
                        let ptr: *mut u8 = globals.heap_stack.stack_access_primitive(reference, datatype);
                        let float: *mut f64 = mem::transmute(ptr);
                        StackVariant::VFloat(*float)
                    }
                }
                else {
                    panic!("can't read primitive of type: {:?}", ty)
                };

                self.stack.push(variant)
            }

            Instruction::Ret => {
                return StackFrameSuccessor::Return
            }
            Instruction::Call => {
                let v = self.stack.pop().unwrap();
                match v {
                    StackVariant::VProc(addr) => { return StackFrameSuccessor::Descend(self, StackFrame::new_on(globals, addr)) }
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