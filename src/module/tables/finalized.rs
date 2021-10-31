use crate::reexports::*;
use crate::module::*;

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Finalized<T> {
    pub(in crate::module) names: BTreeMap<Identifier, Id<T>>,
    pub(in crate::module) data: RawPom<T>,
}