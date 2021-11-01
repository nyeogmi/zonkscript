use crate::module::*;

use std::collections::BTreeMap;

use super::ZId;

#[derive(Debug)]
pub struct Finalized<T> {
    pub(in crate::module) names: BTreeMap<Identifier, ZId<T>>,
    pub(in crate::module) data: Vec<T>,
}