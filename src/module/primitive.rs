use std::{alloc::Layout, any::TypeId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Primitive {    
    pub type_id: TypeId,  
    pub layout: Layout,
    /*
    pub clone_callback: Option<fn(RefToUnknown<'_>, RefToUnknown<'_>)>,
    pub debug_callback: fn(RefToUnknown<'_>, &mut fmt::Formatter<'_>),
    pub drop_callback: Option<fn(RefToUnknown<'_>)>,pe {
    */
}

impl Primitive {
    pub const fn of<T: 'static>() -> Primitive {
        Primitive {
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
        }
    }
}