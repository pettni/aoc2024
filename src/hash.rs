pub use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::hash::Hash;

pub trait FxHashMapBuilder<K, V> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [(K, V); N]) -> Self;
}

impl<K: Eq + Hash, V> FxHashMapBuilder<K, V> for FxHashMap<K, V> {
    fn new() -> Self {
        Self::with_hasher(FxBuildHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }

    fn build<const N: usize>(array: [(K, V); N]) -> Self {
        let mut map = Self::new();
        map.extend(array);
        map
    }
}

pub trait FxHashSetBuilder<K> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [K; N]) -> Self;
}

impl<K: Eq + Hash> FxHashSetBuilder<K> for FxHashSet<K> {
    fn new() -> Self {
        Self::with_hasher(FxBuildHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }

    fn build<const N: usize>(array: [K; N]) -> Self {
        let mut map = Self::new();
        map.extend(array);
        map
    }
}
