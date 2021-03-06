//! This sub-module join all the code in the module
//! to build the Metro Direction and the Metro Interchanges.

use ndarray::Array2;
use num_traits::PrimInt;

use super::build_matrices;
use super::matrix_wrapper;
use super::metro_lines;
use all_shortest_path;

/// Construct the [`super::matrix_wrapper::MetroDirection`]
/// and [`super::matrix_wrapper::MetroInterchange`]
/// object knowing the adjacent matrix of the network and the list
/// of terminus. The lines are automatically determined as the
/// shortest path between the terminal station. Interchanges stations
/// are computed are the intersection between lines.
#[must_use]
pub fn build_directions<T: PrimInt + Default>(
    adj_mat: Array2<T>,
    terminus: &'_ [(usize, usize)],
) -> (
    metro_lines::MetroLines<'_>,
    matrix_wrapper::MetroDirection,
    matrix_wrapper::MetroInterchange,
) {
    let (dist, next) = all_shortest_path::all_shortest_path(adj_mat);
    let metro_lines = metro_lines::MetroLines::from_successor_matrix(&next, terminus);
    let path_mat = build_matrices::PathMatrix::init_matrices(&next, &dist, &metro_lines);
    (
        metro_lines,
        matrix_wrapper::new_metro_direction(path_mat.mdm),
        matrix_wrapper::new_metro_interchange(path_mat.ipm),
    )
}

/// Construct the [`super::matrix_wrapper::MetroDirection`]
/// and [`super::matrix_wrapper::MetroInterchange`]
/// object knowing the adjacent matrix of the network and the metro
/// lines. To use only if the lines cannot be constructed automatically
/// from the terminus list following the shortest path between the terminus stations.
#[must_use]
pub fn build_directions_from_lines<T: PrimInt + Default>(
    adj_mat: Array2<T>,
    metro_lines: &metro_lines::MetroLines<'_>,
) -> (
    matrix_wrapper::MetroDirection,
    matrix_wrapper::MetroInterchange,
) {
    let (dist, next) = all_shortest_path::all_shortest_path(adj_mat);
    let path_mat = build_matrices::PathMatrix::init_matrices(&next, &dist, metro_lines);
    (
        matrix_wrapper::new_metro_direction(path_mat.mdm),
        matrix_wrapper::new_metro_interchange(path_mat.ipm),
    )
}
