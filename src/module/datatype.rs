use std::{alloc::Layout};

use super::*;


#[derive(Debug, PartialEq, Eq)]
pub struct DataType {
    pub name: Identifier,
    pub fields: Vec<(usize, ZId<DataType>)>,
    pub primitive_fields: Vec<(usize, ZId<Primitive>)>,
    pub layout: Layout,
}

pub struct DataTypeBuilder {
    pub name: Identifier,
    fields: Vec<ZId<DataType>>,
}

impl DataTypeBuilder {
    pub fn new(name: &Identifier) -> DataTypeBuilder {
        DataTypeBuilder {
            name: name.clone(),
            fields: vec![],
        }
    }

    pub fn push(&mut self, ty: ZId<DataType>) {
        self.fields.push(ty)
    }
}

impl DataType {
    pub(super) fn wrap(name: Identifier, id: ZId<Primitive>, prim: Primitive) -> DataType {
        DataType {
            name: name,
            fields: vec![], // no visible fields
            primitive_fields: vec![(0, id)],
            layout: prim.layout,
        }
    }

}impl Builds<DataType> for DataTypeBuilder {
    fn build<'a>(self, resolve: &mut impl FnMut(ZId<DataType>, &mut dyn FnMut(&DataType))) -> DataType {
        let mut datatype = DataType {
            name: self.name,
            fields: vec![],
            primitive_fields: vec![],
            layout: Layout::new::<()>(),
        };

        for type_id in self.fields.iter() {
            resolve(*type_id, &mut |ty: &DataType| {
                let (new_layout, offset) = datatype.layout.extend(ty.layout).unwrap();

                datatype.fields.push((offset, *type_id));
                for (field_offset, field) in ty.primitive_fields.iter() {
                    datatype.primitive_fields.push((offset + field_offset, *field))
                }
                datatype.layout = new_layout;
            })
        }

        datatype.layout = datatype.layout.pad_to_align();
        datatype
    }
}