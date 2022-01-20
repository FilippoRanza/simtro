//! Set of utility code that does not belong to a specific module
//! or that is required by multiple modules. It is never a too task-specific code.

pub mod counter;
pub mod cross_index_iterator;
pub mod index_list;
pub mod matrix_utils;
pub mod mixed_iterator;
pub mod unique_id;

use std::collections::HashSet;

/// Crate a vector to the given length initialized
/// to the default value of the return type.
#[must_use]
pub fn zeros<D: Default>(len: usize) -> Vec<D> {
    (0..len).map(|_| Default::default()).collect()
}

/// Collect given iterator into an Hash Set
#[must_use]
pub fn hash_set<T, I>(iter: I) -> HashSet<T>
where
    I: IntoIterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    iter.into_iter().collect()
}
