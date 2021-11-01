use std::{alloc::Layout};

use super::*;


#[derive(Debug, PartialEq, Eq)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Vec<(usize, ZId<Struct>)>,
    pub primitive_fields: Vec<(usize, ZId<Primitive>)>,
    pub layout: Layout,
}

pub struct StructBuilder {
    pub name: Identifier,
    fields: Vec<ZId<Struct>>,
}

impl StructBuilder {
    pub fn new(name: &Identifier) -> StructBuilder {
        StructBuilder {
            name: name.clone(),
            fields: vec![],
        }
    }

    pub fn push(&mut self, ty: ZId<Struct>) {
        self.fields.push(ty)
    }
}

impl Struct {
    pub(super) fn wrap(name: Identifier, id: ZId<Primitive>, prim: Primitive) -> Struct {
        Struct {
            name: name,
            fields: vec![], // no visible fields
            primitive_fields: vec![(0, id)],
            layout: prim.layout,
        }
    }

}impl Builds<Struct> for StructBuilder {
    fn build<'a>(self, resolve: &mut impl FnMut(ZId<Struct>, &mut dyn FnMut(&Struct))) -> Struct {
        let mut struct_ = Struct {
            name: self.name,
            fields: vec![],
            primitive_fields: vec![],
            layout: Layout::new::<()>(),
        };

        for type_id in self.fields.iter() {
            resolve(*type_id, &mut |ty: &Struct| {
                let (new_layout, offset) = struct_.layout.extend(ty.layout).unwrap();

                struct_.fields.push((offset, *type_id));
                for (field_offset, field) in ty.primitive_fields.iter() {
                    struct_.primitive_fields.push((offset + field_offset, *field))
                }
                struct_.layout = new_layout;
            })
        }

        struct_.layout = struct_.layout.pad_to_align();
        struct_
    }
}