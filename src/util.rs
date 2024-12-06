use std::hash::BuildHasherDefault;

pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<rustc_hash::FxHasher>>;
