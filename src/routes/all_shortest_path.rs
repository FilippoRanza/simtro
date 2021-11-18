//! Floyd–Warshall Algorithm implementation.
//! See: <https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm>
//!
use ndarray::Array2;
use num_traits::PrimInt;

use crate::utils::matrix_utils::zeros;

/// Simple type alias. Useful for shorter function signature.
/// The first item is the distance matrix, the second is the
/// successor matrix.
type FWResult<T> = (Array2<T>, Array2<usize>);

/// Floyd-Warshall Algorithm implementation. The input matrix
/// is expected to be the square adjacent matrix of the graph.
/// When there is no direct connection between two nodes,
/// the value associated is T::max_value().
/// Function assumes that g is a square matrix. Panics otherwise.
pub fn all_shortest_path<T: PrimInt + Default>(g: Array2<T>) -> FWResult<T> {
    let (mut dist, mut next) = init_matrixes(g);
    let n = dist.nrows();
    for h in 0..n {
        for i in 0..n {
            for j in 0..n {
                if update_condition(dist[(i, j)], dist[(i, h)], dist[(h, j)]) {
                    dist[(i, j)] = dist[(i, h)] + dist[(h, j)];
                    next[(i, j)] = next[(i, h)];
                }
            }
        }
    }
    (dist, next)
}

/// The update condition of the Algorithm. Required to avoid
/// an overflow sum if adj[i, h] or adj[h, j] is T::max_value()
fn update_condition<T: PrimInt + Default>(ij: T, ih: T, hj: T) -> bool {
    // avoid overflow sum with infinity value.
    if ih == T::max_value() || hj == T::max_value() {
        false
    } else {
        ij > ih + hj
    }
}

/// Setup the successor matrix with the graph's arcs.
fn init_matrixes<T: PrimInt + Default>(g: Array2<T>) -> FWResult<T> {
    let mut next = zeros(g.nrows());
    for ((i, j), w) in g.indexed_iter() {
        if *w < T::max_value() {
            next[(i, j)] = j;
        }
    }
    for i in 0..g.nrows() {
        next[(i, i)] = i;
    }
    (g, next)
}

#[cfg(test)]
mod test {

    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_all_shortest_path() {
        let adj_mat = get_graph();

        let (dist, next) = all_shortest_path(adj_mat);
        // Results from https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm#Example
        let expected_dist = arr2(&[[0, -1, -2, 0], [4, 0, 2, 4], [5, 1, 0, 2], [3, -1, 1, 0]]);
        let expected_next = arr2(&[[0, 2, 2, 2], [0, 1, 0, 0], [3, 3, 2, 3], [1, 1, 1, 3]]);
        assert_eq!(dist, expected_dist);
        assert_eq!(next, expected_next);
    }

    #[test]
    fn test_next_init() {
        let adj_mat = get_graph();
        let (dist, next) = init_matrixes(adj_mat);
        let adj_mat = get_graph();
        assert_eq!(dist, adj_mat); // ensure that dist is not changed
        let expected_next = arr2(&[[0, 0, 2, 0], [0, 1, 2, 0], [0, 0, 2, 3], [0, 1, 0, 3]]);
        assert_eq!(expected_next, next);
    }

    fn get_graph() -> Array2<i32> {
        // Example from https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm#Example
        arr2(&[
            [0, i32::MAX, -2, i32::MAX],
            [4, 0, 3, i32::MAX],
            [i32::MAX, i32::MAX, 0, 2],
            [i32::MAX, -1, i32::MAX, 0],
        ])
    }
}
