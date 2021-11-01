use std::borrow::Cow;

use super::*;

impl ModuleBuilder {
    pub fn add_std_primitives(&mut self) {
        self.primitive(&Cow::Borrowed("i64"), || Primitive::of::<i64>());
        self.primitive(&Cow::Borrowed("f64"), || Primitive::of::<f64>());
    }

    pub(crate) fn get_std_primitives_record(&mut self) -> StdPrimitives {
        StdPrimitives {
            v_int: self.primitive(&Cow::Borrowed("i64"), || unreachable!()),
            v_float: self.primitive(&Cow::Borrowed("f64"), || unreachable!()),
        }
    }
}

#[derive(Debug)]
pub struct StdPrimitives {
    pub v_int: ZId<DataType>,
    pub v_float: ZId<DataType>,
}