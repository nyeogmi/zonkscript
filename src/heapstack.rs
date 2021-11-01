use bumpalo::Bump;

use crate::module::DataType;

pub struct HeapStack {
    // heap: Vec<u8>,
    stack: Bump,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeapStackRef {
    // Heap(u64), 
    Stack(*mut u8),  
}

impl HeapStack {
    pub fn new() -> HeapStack {
        HeapStack { 
            // heap: vec![], 
            stack: Bump::new(),
        }
    }

    pub fn stack_alloc(&mut self, datatype: &DataType) -> HeapStackRef {
        let ptr = self.stack.alloc_layout(datatype.layout).as_ptr();

        // zero the memory
        // ... for now!
        // in the future, let the struct provide an initializer function or else simply refuse to do this 
        unsafe { std::ptr::write_bytes(ptr, 0, datatype.layout.size()) }; 

        let reference = HeapStackRef::Stack(ptr);
        reference
    }

    pub unsafe fn stack_access_field(&mut self, hsr: HeapStackRef, datatype: &DataType, field: usize) -> HeapStackRef {
        match hsr {
            HeapStackRef::Stack(u) => HeapStackRef::Stack(
                u.offset(datatype.fields[field].0 as isize)
            )
        }
    }

    pub unsafe fn stack_access_primitive<'a>(&'a mut self, hsr: HeapStackRef, _datatype: &DataType) -> *mut u8 {
        match hsr {
            HeapStackRef::Stack(u) => u
        }
    }

    pub fn stack_unalloc(&mut self, _hsr: HeapStackRef) {
        // TODO: Call this at the appropriate time
    }
}