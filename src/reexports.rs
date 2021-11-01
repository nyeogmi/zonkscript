pub use super::heapstack::{HeapStack, HeapStackRef};
pub use super::isa::{Instruction, Local, ISAVariant};
pub use super::module::{
    Module, ModuleBuilder,
    Primitive, 
    Procedure, ProcedureBuilder,
    DataType, DataTypeBuilder,
};
pub use super::vm::Thread;