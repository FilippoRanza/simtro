//! This sub-module join all the code in the module
//! to build the Metro Direction and the Metro Interchanges.

use ndarray::Array2;
use num_traits::PrimInt;

use super::all_shortest_path;
use super::build_matrices;
use super::matrix_wrapper;

/// Construct the [`super::matrix_wrapper::MetroDirection`]
/// and [`super::matrix_wrapper::MetroInterchange`]
/// object knowing the adjacent matrix of the network and the list
/// of terminus. The lines are automatically determined as the
/// shortest path between the terminal station. Interchanges stations
/// are computed are the intersection between lines.
pub fn build_directions<T: PrimInt + Default>(
    adj_mat: Array2<T>,
    terminus: &[(usize, usize)],
) -> (
    matrix_wrapper::MetroDirection,
    matrix_wrapper::MetroInterchange,
) {
    let (dist, next) = all_shortest_path::all_shortest_path(adj_mat);
    let path_mat = build_matrices::PathMatrix::init_matrices(&next, &dist, terminus);
    (
        matrix_wrapper::new_metro_direction(path_mat.mdm),
        matrix_wrapper::new_metro_interchange(path_mat.ipm),
    )
}