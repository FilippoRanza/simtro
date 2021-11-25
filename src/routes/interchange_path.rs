//!
//! This module compute the interchange path matrix for the given matrix.
//! interchange path matrix is inside a metro system the analog for the successor
//! matrix for a graph: item *ij*
//! tells which is the next interchange, or terminal station to reach the destination
//! from the current
//! starting point.
//!
//!

use super::metro_line_set;
use super::path_iterator::PathIterator;
use super::Mat;
use crate::utils::matrix_utils;
use ndarray::Array2;
use std::collections::HashSet;

/// Build the interchange path matrix starting
/// from the given successor matrix, the terminus list (to identify the lines)
/// and the interchanges list.
/// For the implementation the order of the interchanges, order of terminus and relative
/// order in station in the terminus couple is irrelevant.
pub fn build_interchange_path_matrix(next: &Mat, lines: &metro_line_set::MetroLinesSet) -> Mat {
    let interchanges = lines.find_interchanges();
    let mut output = matrix_utils::zeros_as(next);

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

/// Build the interchange path matrix starting
/// from the given successor matrix, the terminus list (to identify the lines)
/// and the interchanges list.
/// For the implementation the order of the interchanges, order of terminus and relative
/// order in station in the terminus couple is irrelevant.
pub fn fast_build_interchange_path_matrix(
    next: &Mat,
    lines: &metro_line_set::MetroLinesSet,
) -> Mat {
    let interchanges = lines.find_interchanges();
    let mut output = matrix_utils::zeros_as(next);
    let mut ipf = InterchangePathFinder::new(next, interchanges);

    for s in 0..next.nrows() {
        for e in 0..next.nrows() {
            if lines.is_same_line(s, e) {
                output[(s, e)] = e;
            } else if !ipf.is_set(s, e) {
                ipf.set_path_interchange(s, e, &mut output);
            }
        }
    }

    output
}

struct InterchangePathFinder<'a> {
    next: &'a Mat,
    interchanges: HashSet<usize>,
    memo: MemoEngine,
    path_cache: Vec<usize>,
    result: Option<usize>,
}

impl<'a> InterchangePathFinder<'a> {
    fn new(next: &'a Mat, interchanges: HashSet<usize>) -> Self {
        let n = next.ncols();
        let memo = MemoEngine::new(n);
        let path_cache = Vec::with_capacity(n);
        Self {
            next,
            interchanges,
            memo,
            path_cache,
            result: None,
        }
    }

    fn set_path_interchange(&mut self, start: usize, stop: usize, ipm: &mut Mat) {
        self.find_interchange(start, stop);
        self.set_interchange_value(ipm, stop);
    }

    fn find_interchange(&mut self, start: usize, stop: usize) {
        if let Some(value) = self.memo.get_value((start, stop)) {
            self.result = Some(value);
        }

        let path_iter = PathIterator::new(start, stop, self.next);
        self.path_cache.clear();
        for n in path_iter {
            if let Some(v) = self.handle_next_step(n, stop) {
                self.result = Some(v);
                break;
            } else {
                self.path_cache.push(n);
            }
        }
    }

    fn handle_next_step(&self, src: usize, stop: usize) -> Option<usize> {
        if let Some(value) = self.memo.get_value((src, stop)) {
            Some(value)
        }
        else if self.interchanges.contains(&src) {
            Some(src)
        } else {
            None
        }
    }

    fn set_interchange_value(&mut self, ipm: &mut Mat, dst: usize) {
        let inter = self.result.unwrap();
        for src in &self.path_cache {
            self.memo.set_value((*src, dst), inter);
            ipm[(*src, dst)] = inter;
        }
    }


    fn is_set(&self, s: usize, e: usize) -> bool {
        self.memo.get_value((s, e)).is_some()
    }
}

struct MemoEngine {
    memo: Array2<Option<usize>>,
}

impl MemoEngine {
    fn new(size: usize) -> Self {
        Self {
            memo: matrix_utils::zeros(size),
        }
    }

    fn get_value(&self, idx: (usize, usize)) -> Option<usize> {
        self.memo[idx]
    }

    fn set_value(&mut self, idx: (usize, usize), value: usize) {
        self.memo[idx] = Some(value)
    }
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

    use super::super::metro_lines;
    use super::super::test_definitions;
    use super::*;

    #[test]
    fn test_build_interchange_path_matrix() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let lines = metro_lines::MetroLines::from_successor_matrix(&next, &terminus);
        let lines = metro_line_set::MetroLinesSet::from(&lines);

        let ipm = build_interchange_path_matrix(&next, &lines);
        let correct = test_definitions::make_correct_interchange_path();
        assert_eq!(ipm, correct);
    }
    #[test]
    fn test_fast_build_interchange_path_matrix() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let lines = metro_lines::MetroLines::from_successor_matrix(&next, &terminus);
        let lines = metro_line_set::MetroLinesSet::from(&lines);

        let ipm = fast_build_interchange_path_matrix(&next, &lines);
        let correct = test_definitions::make_correct_interchange_path();
        assert_eq!(ipm, correct);
    }
}
