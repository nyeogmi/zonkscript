use std::{hash::Hash, marker::PhantomData};

use moogle::IdLike;

// The normal moogle ID type is assumed to be 1-indexed.
// Also, it assumes a normal moogle pom.
// So, let's use this instead.

pub struct ZId<T>(pub usize, std::marker::PhantomData<*const T>);

impl<T> ZId<T> {
    pub(crate) fn new(ix: usize) -> ZId<T> { ZId(ix, PhantomData) }
}

impl<T> std::fmt::Debug for ZId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Include T
        f.debug_tuple("ZId").field(&self.0).finish()
    }
}

impl<T> Clone for ZId<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T> Copy for ZId<T> {

}

impl<T> PartialEq for ZId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 
    }
}

impl<T> Eq for ZId<T> {

}

impl<T> PartialOrd for ZId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for ZId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for ZId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: 'static> IdLike for ZId<T> {
    fn id_min_value() -> Self { Self(usize::id_min_value(), PhantomData) }
    fn id_max_value() -> Self { Self(usize::id_max_value(), PhantomData) }
}