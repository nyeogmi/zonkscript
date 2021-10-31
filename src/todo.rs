// TODO: Things below.
// - Don't use Moogle (Use something ad hoc instead.)
// - Get rid of hacks for 1-indexing.
// - Actually check for whether functions and structs haven't been sealed -- error if so.
// - Fail gracefully; don't panic.
// - std_primitives.rs and linker.rs are _real ugly
// - Variant should be Copy.
// - Arguments for Call instruction.
// - Return values for Ret instruction.
// - Separate typechecking pass
// - Instruction to ref an individual field
// - Instructions to make assertions about the stack.
// - Primitives on the stack should have an "allocated?" flag
//
// - Implement free as a non-noop for stack frames on the bump allocator.