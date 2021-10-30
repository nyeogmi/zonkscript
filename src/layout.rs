use std::{alloc::Layout, any::{Any, TypeId}, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeStruct {
    pub fields: Vec<(usize, Rc<RuntimeStruct>)>,
    pub single_fields: Vec<RuntimeSingle>,
    pub overall_layout: Layout,
}

pub struct RuntimeStructBuilder {
    // same
    pub fields: Vec<(usize, Rc<RuntimeStruct>)>,
    pub single_fields: Vec<RuntimeSingle>,
    pub overall_layout: Layout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeSingle {
    pub offset: usize,
    pub type_data: RuntimeType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeType {    
    pub type_id: TypeId,  
    pub layout: Layout,
    /*
    pub clone_callback: Option<fn(RefToUnknown<'_>, RefToUnknown<'_>)>,
    pub debug_callback: fn(RefToUnknown<'_>, &mut fmt::Formatter<'_>),
    pub drop_callback: Option<fn(RefToUnknown<'_>)>,pe {
    */
}

impl RuntimeType {
    pub const VINT: RuntimeType = RuntimeType::of::<i64>();
    pub const VFLOAT: RuntimeType = RuntimeType::of::<f64>();

    pub const fn of<T: 'static>() -> RuntimeType {
        RuntimeType {
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
        }
    }
}

impl RuntimeStructBuilder {
    pub fn new() -> RuntimeStructBuilder {
        RuntimeStructBuilder {
            fields: vec![],
            single_fields: vec![],
            overall_layout: Layout::new::<()>(),
        }
    }

    pub fn push(&mut self, ty: Rc<RuntimeStruct>) {
        let (new_overall_layout, offset) = 
            self.overall_layout.extend(ty.overall_layout).unwrap();

        for field in &ty.single_fields {
            self.single_fields.push(RuntimeSingle { 
                offset: offset + field.offset, 
                type_data: field.type_data,
            })
        }
        self.fields.push((offset, ty));
        self.overall_layout = new_overall_layout;
    }

    pub fn build(self) -> RuntimeStruct {
        let overall_layout = self.overall_layout.pad_to_align();
        RuntimeStruct { 
            fields: self.fields,
            single_fields: self.single_fields,
            overall_layout
        }
    }
}

impl RuntimeStruct {
    pub(super) fn wrap(single: RuntimeType) -> RuntimeStruct {
        RuntimeStruct {
            fields: vec![], // no visible fields
            single_fields: vec![RuntimeSingle { offset: 0, type_data: single }],
            overall_layout: single.layout
        }
    }
}