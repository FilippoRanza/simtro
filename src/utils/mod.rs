//! Set of utility code that does not belong to a specific module 
//! or that is required by multiple modules. It is never a too task-specific code. 

pub mod cross_index_iterator;
pub mod matrix_utils;
pub mod unique_id;

use std::collections::HashSet;

/// Crate a vector to the given length initialized 
/// to the default value of the return type. 
pub fn zeros<D: Default>(len: usize) -> Vec<D> {
    (0..len).map(|_| Default::default()).collect()
}

pub fn hash_set<T, I>(iter: I) -> HashSet<T>
where
    I: IntoIterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    iter.into_iter().collect()
}
