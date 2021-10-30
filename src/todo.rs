// TODO: Things below.
// - Allocate whole stack inline.
// - Variant should be Copy.
// - Composite data types.
// - Arguments for Call instruction.
// - Return values for Ret instruction.
// - Procedure ids for Instruction set (instead of just using Rc<Procedure> everywhere)
// - Type ids for Instruction set
// - Separate typechecking pass
// - Instruction to ref an individual field
// - Instructions to make assertions about the stack.
// - Primitives on the stack should have an "allocated?" flag
// - Stack values should be ref-like.
//   Effectively: things in the stack (as opposed to variable slots) are &Xs, variable slots are non-fixed-size. Whoa!! 
//   This might be cool or might be terrible. It is very JavaScript Engine!
//
// - Implement free as a non-noop for stack frames on the bump allocator.