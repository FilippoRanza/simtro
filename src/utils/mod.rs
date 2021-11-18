pub mod cross_index_iterator;
pub mod matrix_utils;
pub mod unique_id;

pub fn zeros<D: Default>(len: usize) -> Vec<D> {
    (0..len).map(|_| Default::default()).collect()
}
