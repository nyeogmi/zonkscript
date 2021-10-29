// TODO: Things below.
// - Stack indices.
// - Allocate whole stack inline.
// - Variant should be Copy.
// - Composite data types.
// - Call instruction.
// - Return values for Ret instruction.
// - Procedure ids for Instruction set (instead of just using Rc<Procedure> everywhere)
// - Instructions to make assertions about the stack.
// - Effectively: things in the stack (as opposed to variable slots) are &Xs, variable slots are non-fixed-size. Whoa!! 
//   This might be cool or might be terrible. It is very JavaScript Engine!
use std::{borrow::Cow, collections::HashMap, rc::Rc};

struct Thread {
    stack: Vec<StackFrame>,
}

#[derive(Debug)]
struct Procedure {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Push(Variant),
    Save(Var),
    Load(Var),

    Call,
    Ret,

    Print,
    Add2, Sub2, Mul2, Div2,
}

struct StackFrame {
    // TODO: Variables by _index_ instead!
    values: HashMap<Var, Variant>,
    stack: Vec<Variant>,
    code: Rc<Procedure>,
    ip: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Var(Cow<'static, str>);

#[derive(Clone, Debug)]
enum Variant {
    VInt(i64),
    VFloat(f64),
    VAddress(Rc<Procedure>)  // TODO: Literally any other representation holy shit
    // VString(String),
}

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

    let mut main_thread = Thread {
        stack: vec![StackFrame::new(main.clone())]
    };

    loop {
        if !main_thread.step() {
            break;
        }
    }
}

impl Thread {
    fn step(&mut self) -> bool {
        if let Some(active) = self.stack.pop() {
            match active.step() {
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
    fn new(code: Rc<Procedure>) -> StackFrame {
        StackFrame {
            values: HashMap::new(),
            stack: Vec::new(),
            code,
            ip: 0,
        }
    }

    fn step(mut self) -> StackFrameSuccessor {
        let instructions = &self.code.instructions;
        assert!((0..instructions.len()).contains(&self.ip));
        let old_ip = self.ip;
        self.ip += 1;

        match &instructions[old_ip] {
            Instruction::Push(v) => { self.stack.push(v.clone()); }
            Instruction::Save(x) => {
                self.values.insert(x.clone(), self.stack.pop().unwrap());
            }
            Instruction::Load(x) => {
                self.stack.push(self.values.get(&x).unwrap().clone());
            }
            Instruction::Ret => {
                return StackFrameSuccessor::Return
            }
            Instruction::Call => {
                let v = self.stack.pop().unwrap();
                match v {
                    Variant::VAddress(addr) => { return StackFrameSuccessor::Descend(self, StackFrame::new(addr)) }
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
    v0: Variant, v1: Variant, 
    name: &'static str, 
    i: impl Fn(i64, i64) -> i64, 
    f: impl Fn(f64, f64) -> f64,
) -> Variant {
    match (v0, v1) {
        (Variant::VInt(i0), Variant::VInt(i1)) => Variant::VInt(i(i0, i1)),
        (Variant::VFloat(f0), Variant::VFloat(f1)) => Variant::VFloat(f(f0, f1)),
        // TODO: Coerce an int into a float if needed?

        (v0, v1) => panic!("can't {}: {:?} and {:?}", name, v0, v1),
    }
}