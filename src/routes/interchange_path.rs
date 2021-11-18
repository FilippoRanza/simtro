//!
//! This module compute the interchange path matrix for the given matrix. 
//! interchange path matrix is inside a metro system the analog for the successor 
//! metrix for a graph: item *ij* 
//! tells which is the next interchange, or terminal station to reach the destionation 
//! from the current 
//! starting point. 
//!
//!

use super::lines;
use super::path_iterator::PathIterator;
use super::Mat;
use crate::utils::matrix_utils;
use std::collections::HashSet;

/// Build the interchange path matrix starting 
/// from the given successor matrix, the terminus list (to identify the lines) 
/// and the interchanges list. 
/// For the implementation the order of the interchanges, order of terminus and relative 
/// order in station in the terminus couple is irrelevant. 
pub fn build_interchange_path_matrix(
    next: &Mat,
    terminus: &[(usize, usize)],
    interchanges: &[usize],
) -> Mat {
    let interchanges = interchanges.iter().map(|i| *i).collect();
    let mut output = matrix_utils::zeros_as(next);
    let lines = lines::MetroLines::new(next, terminus);

    for s in 0..next.nrows() {
        for e in 0..next.nrows() {
            let n = if lines.is_same_line(s, e) {
                e
            } else {
                take_next(s, e, next, &interchanges)
            };
            output[(s, e)] = n;
        }
    }

    output
}

/// Identify the fist interchange node in path from node start to node end. 
fn take_next(start: usize, end: usize, next: &Mat, interchanges: &HashSet<usize>) -> usize {
    PathIterator::new(start, end, next)
        .filter(|n| interchanges.contains(n))
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {

    use super::super::test_definitions;
    use super::*;

    #[test]
    fn test_build_interchange_path_matrix() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let interchanges = test_definitions::make_interchanges();

        let ipm = build_interchange_path_matrix(&next, &terminus, &interchanges);
        let correct = test_definitions::make_correct_interchange_path();
        assert_eq!(ipm, correct);
    }
}
