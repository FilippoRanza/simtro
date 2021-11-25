//! This module implements both the initialization and the 
//! query of two direction matrix. 
//! The first is the Metro Direction Matrix that tells the 
//! passenger the direction of the train to take in order to 
//! reach the destination. If there is one or more interchanges 
//! in the middle this matrix shows the direction from the current 
//! station to the next interchange on the same line. 
//!
//! The second is the Interchange Path Matrix that tells the station 
//! that the passenger must visit on foot to reach the destination from 
//! the starting point. If the two stations are on the same line the matrix 
//! simply states that the next is the destination; if the stations are not 
//! on the same line then the matrix will return the next interchange on 
//! the path. 
//!
//! Those two matrix combined allow the passenger object to correctly 
//! reconstruct the metro path the it must follow to reach the destination. 
//!

use ndarray::Array2;

// Type alias defined for brevity.
type Mat = Array2<usize>;

mod all_shortest_path;
mod build_directions;
mod build_matrices;
mod interchange_path;
mod matrix_wrapper;
mod metro_direction;
mod metro_line_set;
mod metro_lines;
mod path_iterator;

pub use build_directions::{build_directions, build_directions_from_lines};
pub use matrix_wrapper::{MetroDirection, MetroInterchange};
pub use metro_lines::MetroLines;


#[cfg(test)]
pub mod test_exports {

    use super::*;

    pub fn fast_ipm_build(next: &Mat, lines: &metro_line_set::MetroLinesSet) {
        interchange_path::fast_build_interchange_path_matrix(next, lines);
    }

}


#[cfg(test)]
mod test_definitions {

    /*
     In this module there are the difinition
     for all the correct results for test
     in this module.
    */

    use super::Mat;
    use ndarray::{arr2, Array2};

    /*
        The graph, by shown just the arcs couple, is:
            (0, 1),
            (1, 2),
            (2, 3),
            (2, 5),
            (2, 7),
            (3, 4),
            (7, 8),
            (5, 6),
    */

    pub fn make_correct_direction_matrix() -> Mat {
        arr2(&[
            [0, 6, 6, 6, 6, 6, 6, 6, 6],
            [0, 1, 6, 6, 6, 6, 6, 6, 6],
            [0, 0, 2, 4, 4, 6, 6, 8, 8],
            [8, 8, 8, 3, 4, 8, 8, 8, 8],
            [8, 8, 8, 8, 4, 8, 8, 8, 8],
            [0, 0, 0, 0, 0, 5, 6, 0, 0],
            [0, 0, 0, 0, 0, 0, 6, 0, 0],
            [4, 4, 4, 4, 4, 4, 4, 7, 8],
            [4, 4, 4, 4, 4, 4, 4, 4, 8],
        ])
    }

    pub fn make_correct_interchange_path() -> Mat {
        arr2(&[
            [0, 1, 2, 2, 2, 5, 6, 2, 2],
            [0, 1, 2, 2, 2, 5, 6, 2, 2],
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [2, 2, 2, 3, 4, 2, 2, 7, 8],
            [2, 2, 2, 3, 4, 2, 2, 7, 8],
            [0, 1, 2, 2, 2, 5, 6, 2, 2],
            [0, 1, 2, 2, 2, 5, 6, 2, 2],
            [2, 2, 2, 3, 4, 2, 2, 7, 8],
            [2, 2, 2, 3, 4, 2, 2, 7, 8],
        ])
    }

    pub fn make_next_matrix() -> Mat {
        arr2(&[
            [0, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 1, 2, 2, 2, 2, 2, 2, 2],
            [1, 1, 2, 3, 3, 5, 5, 7, 7],
            [2, 2, 2, 3, 4, 2, 2, 2, 2],
            [3, 3, 3, 3, 4, 3, 3, 3, 3],
            [2, 2, 2, 2, 2, 5, 6, 2, 2],
            [5, 5, 5, 5, 5, 5, 6, 5, 5],
            [2, 2, 2, 2, 2, 2, 2, 7, 8],
            [7, 7, 7, 7, 7, 7, 7, 7, 8],
        ])
    }

    pub fn make_dist_matrix() -> Array2<u32> {
        arr2(&[
            [0, 1, 2, 3, 4, 3, 4, 3, 4],
            [1, 0, 1, 2, 3, 2, 3, 2, 3],
            [2, 1, 0, 1, 2, 1, 2, 1, 2],
            [3, 2, 1, 0, 1, 2, 3, 2, 3],
            [4, 3, 2, 1, 0, 3, 4, 3, 4],
            [3, 2, 1, 2, 3, 0, 1, 2, 3],
            [4, 3, 2, 3, 4, 1, 0, 3, 4],
            [3, 2, 1, 2, 3, 2, 3, 0, 1],
            [4, 3, 2, 3, 4, 3, 4, 1, 0],
        ])
    }

    pub fn make_terminus() -> Vec<(usize, usize)> {
        vec![(0, 6), (4, 8)]
    }

    pub fn make_interchanges() -> Vec<usize> {
        vec![2]
    }
}
