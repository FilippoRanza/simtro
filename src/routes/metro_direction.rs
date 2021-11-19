//! This module implements the code used by the passenger
//! to determine the direction (i.e. it must find the correct train
//! by terminus) knowing the current passenger's position and the passenger's station.
//!

use ndarray::Array2;
use num_traits::PrimInt;

use super::lines::MetroLines;
use super::Mat;

use crate::utils::matrix_utils;

/// Inform passenger objects about the direction
/// to take.
pub struct MetroDirection {}

impl MetroDirection {
    /// Given a passenger that starts at station start and must
    /// go at station dest this function returns the terminus (or the metro direction)
    /// of the train that the passenger must take. If there is one (or more) interchange
    /// in the middle the function returns the direction that
    /// the passenger must take in order to reach the next interchange station.
    pub fn get_direction(start: usize, dest: usize) -> usize {
        todo! {}
    }
}

/// Build the metro direction matrix. Suppose that there is a passenger that need to
/// go
/// from station A to station B, station A is on line 1 that has its terminus as
/// stations T1 and T2.
/// The Metro Direction Matrix tells in which direction (from T1 to
/// T2 or from T2 to T1) goes the train
/// that will bring the passenger to B if
/// B is on line 1, or to the next interchange if B is on another line. Entry
/// *AB* is in this case either T1 or T2.
/// Take in input the successor matrix, the distance matrix both from
/// [`super::all_shortest_path::all_shortest_path`],
/// terminus list and the interchange path matrix from
/// [`super::interchange_path::build_interchange_path_matrix`]
pub fn build_metro_direction<T: PrimInt>(
    next: &Mat,
    dist: &Array2<T>,
    terminus: &[(usize, usize)],
    interchange_path_matrix: &Mat,
) -> Mat {
    let output = matrix_utils::zeros_as(next);
    let lines = MetroLines::new(next, terminus);

    let output = set_in_line_directions(&lines, dist, output);

    set_cross_line_directions(&lines, interchange_path_matrix, output)
}

/// Set direction for station on the same line
fn set_in_line_directions<T: PrimInt>(lines: &MetroLines, dist: &Array2<T>, mut dir_mat: Mat) -> Mat {
    for line in lines.line_iterator() {
        for s1 in line.stations {
            for s2 in line.stations {
                let dir = find_closer(dist, line.terminus.0, line.terminus.1, *s1, *s2);
                dir_mat[(*s1, *s2)] = dir;
            }
        }
    }
    dir_mat
}

/// Set direction for stations on different lines
fn set_cross_line_directions(lines: &MetroLines, ipm: &Mat, mut dir_mat: Mat) -> Mat {
    for (line_a, line_b) in lines.cross_line_iter() {
        for a in line_a {
            for b in line_b {
                get_interchange_direction(*a, *b, ipm, &mut dir_mat);
                get_interchange_direction(*b, *a, ipm, &mut dir_mat);
            }
        }
    }
    dir_mat
}


/// This function knowing the distance matrix (from
/// [`super::all_shortest_path::all_shortest_path`]),
/// the
/// terminus couple (irrelevant order) the start end the destination
/// returns the direction that passengers needs to follow to go from station start
/// to station dest. It assumes
/// that start and dest are on the same line.
fn find_closer<T: PrimInt>(
    dist: &Array2<T>,
    t1: usize,
    t2: usize,
    start: usize,
    dest: usize,
) -> usize {
    if start == dest {
        start
    } else {
        let d1 = dist[(start, t1)];
        let d2 = dist[(dest, t1)];
        if d2 >= d1 {
            t2
        } else {
            t1
        }
    }
}

/// This function identifies the direction the passenger must
/// follow to reach from station start (that is on one line) station 
/// dst that is on another line. The direction set by this function is 
/// the direction required to reach the next interchange to reach the destination. 
/// Takes in input the start station, the destination station, the interchange
/// path matrix and the current direction matrix.
fn get_interchange_direction(
    start: usize,
    dst: usize,
    interchange_path_matrix: &Mat,
    direction_matrix: &mut Mat,
) {
    let interchange = interchange_path_matrix[(start, dst)];
    let dir = direction_matrix[(start, interchange)];
    direction_matrix[(start, dst)] = dir;
}

#[cfg(test)]
mod test {

    use super::*;

    use super::super::test_definitions;

    #[test]
    fn test_metro_direction_matrix() {
        /*
            Ensure a correct contruction of the metro direction matrix.
        */
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let dist = test_definitions::make_dist_matrix();

        let interchange_path = test_definitions::make_correct_interchange_path();

        let expected_direction = test_definitions::make_correct_direction_matrix();

        let direction = build_metro_direction(&next, &dist, &terminus, &interchange_path);
        assert_eq!(direction, expected_direction);
    }
}
