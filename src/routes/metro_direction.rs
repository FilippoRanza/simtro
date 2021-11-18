//! This module implements the code used by the passenger
//! to determine the direction (i.e. it must find the  correct train
//! by terminus) knowing the current passenger's position and the passenger's station

use ndarray::Array2;
use num_traits::PrimInt;

use super::lines::build_metro_lines;
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

pub fn build_metro_direction<T: PrimInt + std::fmt::Debug>(
    next: &Mat,
    dist: &Array2<T>,
    terminus: &[(usize, usize)],
    interchange_path_matrix: &Mat,
) -> Mat {
    let mut output = matrix_utils::zeros_as(next);
    let lines = build_metro_lines(next, terminus);

    for line in lines.line_iterator() {
        for s1 in line.stations {
            for s2 in line.stations {
                let dir = find_closer(dist, line.terminus.0, line.terminus.1, *s1, *s2);
                output[(*s1, *s2)] = dir;
            }
        }
    }

    for (line_a, line_b) in lines.cross_line_iter() {
        for a in line_a {
            for b in line_b {
                get_interchange_direction(*a, *b, interchange_path_matrix, &mut output);
                get_interchange_direction(*b, *a, interchange_path_matrix, &mut output);
            }
        }
    }

    output
}

fn find_closer<T: PrimInt + std::fmt::Debug>(
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

fn get_interchange_direction(
    start: usize,
    dst: usize,
    next_station: &Mat,
    direction_matrix: &mut Mat,
) {
    let interchange = next_station[(start, dst)];
    let dir = direction_matrix[(start, interchange)];
    direction_matrix[(start, dst)] = dir;
}

#[cfg(test)]
mod test {

    use super::*;

    use super::super::test_definitions;

    #[test]
    fn test_metro_direction_matrix() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let dist = test_definitions::make_dist_matrix();

        let interchange_path = test_definitions::make_correct_interchange_path();

        let expected_direction = test_definitions::make_correct_direction_matrix();

        let direction = build_metro_direction(&next, &dist, &terminus, &interchange_path);
        let mut result = vec![];
        let nodes = next.ncols();
        for r in 0..nodes {
            let res = expected_direction.row(r) == direction.row(r);
            result.push(res);
        }
        let ok: Vec<bool> = result.iter().map(|_| true).collect();
        assert_eq!(direction, expected_direction);
        assert_eq!(result, ok);
    }
}
