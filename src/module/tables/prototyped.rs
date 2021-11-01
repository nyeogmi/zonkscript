use crate::module::*;

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Prototyped<T, Builder: Builds<T>> {
    pub(in crate::module) names: BTreeMap<Identifier, ZId<T>>,
    pub(in crate::module) rev_names: BTreeMap<ZId<T>, Identifier>,
    pub(in self) data: Vec<Phased<T, Builder>>,
}

impl<T, Builder: Builds<T>> Prototyped<T, Builder> {
    pub fn new() -> Prototyped<T, Builder> {
        Prototyped {
            names: BTreeMap::new(),
            rev_names: BTreeMap::new(),
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn reference(&mut self, identifier: &Identifier) -> ZId<T> {
        if let Some(id) = self.names.get(identifier) {
            return *id;
        }

        let id = ZId::new(self.data.len());
        self.data.push(Phased::Mentioned);
        self.names.insert(identifier.clone(), id);
        self.rev_names.insert(id, identifier.clone());
        id
    }

    pub fn mutate(&mut self, id: ZId<T>, default_builder: impl for<'a> Fn(&'a Identifier) -> Builder) -> Option<&mut Builder> {
        let p = &mut self.data[id.0];
        if let Phased::Mentioned = p {
            let name = self.rev_names.get(&id).unwrap();
            *p = Phased::InProgress(default_builder(name));
        }

        match &mut self.data[id.0] {
            Phased::Mentioned => { unreachable!(); }
            Phased::InProgress(b) => { return Some(b); }
            Phased::Sealed(_) | Phased::Built(_) => { return None; }
        }
    }

    pub fn mutate_raw(&mut self, id: ZId<T>) -> &mut Phased<T, Builder> {
        &mut self.data[id.0]
    }

    pub(crate) fn is_populated(&self, id: ZId<DataType>) -> bool {
        match &self.data[id.0] {
            Phased::Mentioned => false,
            Phased::InProgress(_) => true,
            Phased::Sealed(_) => true,
            Phased::Built(_) => true,
        }
    }

    pub fn seal(&mut self, id: ZId<T>) {
        if let Phased::InProgress(_) = &self.data[id.0] {
            
        } else { panic!("wtf"); }

        let mut p: Phased<T, Builder> = Phased::Mentioned;
        std::mem::swap(&mut self.data[id.0], &mut p);

        let mut p = match p {
            Phased::Mentioned => unreachable!(),
            Phased::InProgress(x) => Phased::Sealed(x),
            Phased::Sealed(_) => unreachable!(),
            Phased::Built(_) => unreachable!(),
        };
        std::mem::swap(&mut self.data[id.0], &mut p);
    }

    pub fn inject(&mut self, id: ZId<T>, value: T, eq: impl Fn(&T, &T) -> bool) {  
        // crash if the value was already partway created via some other means
        let p = &mut self.data[id.0];
        match p {
            Phased::Mentioned => *p = Phased::Built(value),
            Phased::InProgress(_) => panic!("wtf"),
            Phased::Sealed(_) => panic!("wtf"),
            Phased::Built(t) => {
                if eq(t, &value) { return } else { panic!("wtf") }
            }
        }
    }

    pub fn link(mut self) -> Finalized<T> {
        fn link_item<T, Builder: Builds<T>>(
            ix: ZId<T>,
            data: &mut Vec<Phased<T, Builder>>,
        ) {
            match &data[ix.0] {
                Phased::Mentioned => panic!("wtf"),
                Phased::InProgress(_) => panic!("wtf"),  // must be sealed
                Phased::Sealed(b) => b,
                Phased::Built(_) => { return },
            };

            let mut val: Phased<T, Builder> = Phased::Mentioned;
            std::mem::swap(&mut val, &mut data[ix.0]);
            let builder = if let Phased::Sealed(b) = val {
                b
            } else { unreachable!(); };

            data[ix.0] = Phased::Built(builder.build(
                &mut |ix2, cb: &mut dyn FnMut(&T)| {
                    link_item(ix2, data);
                    cb(data[ix2.0].unwrap_ref())
                }
            ));
        }

        for ix in 0..self.data.len() {
            link_item(ZId::new(ix), &mut self.data);
        }

        Finalized {
            names: self.names,
            data: self.data.into_iter().map(|x| x.unwrap()).collect(),
        }
    }
}

pub trait Builds<T> {
    fn build<'a>(self, resolve: &mut impl FnMut(ZId<T>, &mut dyn FnMut(&T))) -> T;
}

#[derive(Copy, Clone)]
pub enum Phased<T, TBuilder> {
    Mentioned,
    InProgress(TBuilder),
    Sealed(TBuilder),
    Built(T),
}

impl<T, TBuilder> Phased<T, TBuilder> {
    pub(crate) fn unwrap_ref(&self) -> &T {
        match self {
            Self::Mentioned => panic!(),
            Self::InProgress(_) => panic!(),
            Self::Sealed(_) => panic!(),
            Self::Built(t) => &t,
        }
    }

    pub(crate) fn unwrap(self) -> T {
        match self {
            Self::Mentioned => panic!(),
            Self::InProgress(_) => panic!(),
            Self::Sealed(_) => panic!(),
            Self::Built(t) => t,
        }
    }
}