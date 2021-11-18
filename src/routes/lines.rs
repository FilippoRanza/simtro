//!
//! This module builds the set of all the lines
//!
//!

use std::collections::HashSet;

use crate::utils::cross_index_iterator;

use super::path_iterator::PathIterator;
use super::Mat;

type Set<'a> = &'a HashSet<usize>;

///
/// Helper struct used to determine if
/// two stations are on the same line or not.
pub struct MetroLines<'a> {
    terminus: &'a [(usize, usize)],
    lines: Vec<HashSet<usize>>,
}

impl<'a> MetroLines<'a> {
    /// Tell if station s1 and station s2 are on the same metro line or not.
    pub fn is_same_line(&self, s1: usize, s2: usize) -> bool {
        for line in &self.lines {
            if line.contains(&s1) && line.contains(&s2) {
                return true;
            }
        }
        false
    }

    pub fn line_iterator(&'a self) -> impl IntoIterator<Item = LineItem<'a>> {
        self.terminus
            .iter()
            .zip(self.lines.iter())
            .map(|t| LineItem::from_tuple(t))
    }

    pub fn cross_line_iter(&'a self) -> impl IntoIterator<Item = (Set<'a>, Set<'a>)> {
        CrossLineIterator::new(&self.lines)
    }
}

impl<'a> Iterator for MetroLines<'a> {
    type Item = LineItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct LineItem<'a> {
    pub terminus: (usize, usize),
    pub stations: &'a HashSet<usize>,
}

impl<'a> LineItem<'a> {
    fn from_tuple(t: (&(usize, usize), &'a HashSet<usize>)) -> Self {
        let ((t1, t2), stations) = t;
        let terminus = (*t1, *t2);
        Self { terminus, stations }
    }
}

pub fn build_metro_lines<'a>(next: &Mat, terminus: &'a [(usize, usize)]) -> MetroLines<'a> {
    let lines = terminus
        .iter()
        .map(|(t1, t2)| PathIterator::new(*t1, *t2, next).to_set())
        .collect();
    MetroLines { terminus, lines }
}

pub struct CrossLineIterator<'a> {
    lines: &'a Vec<HashSet<usize>>,
    iterator: cross_index_iterator::CrossIndexIterator,
}

impl<'a> CrossLineIterator<'a> {
    fn new(lines: &'a Vec<HashSet<usize>>) -> Self {
        let iterator = cross_index_iterator::CrossIndexIterator::new(lines.len());
        Self { lines, iterator }
    }
}

impl<'a> Iterator for CrossLineIterator<'a> {
    type Item = (Set<'a>, Set<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.iterator.next()?;
        let i = &self.lines[i];
        let j = &self.lines[j];
        Some((i, j))
    }
}

#[cfg(test)]
mod test {

    use super::super::test_definitions;
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_line_generation() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let metro_lines = build_metro_lines(&next, &terminus);
        let lines = metro_lines.lines;
        let correct_line_1 = HashSet::from([0, 1, 2, 5, 6]);
        let correct_line_2 = HashSet::from([4, 3, 2, 7, 8]);
        let expected = vec![correct_line_1, correct_line_2];
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_same_line_check() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let metro_lines = build_metro_lines(&next, &terminus);
        let line_one = [0, 1, 2, 5, 6];
        let line_two = [4, 3, 2, 7, 8];
        for i in &line_one {
            for j in &line_one {
                assert!(metro_lines.is_same_line(*i, *j))
            }
        }

        for i in &line_two {
            for j in &line_two {
                assert!(metro_lines.is_same_line(*i, *j))
            }
        }
        for i in &line_one {
            for j in &line_two {
                // 2 is the interchange staion: is on both lines.arg
                if *i != 2 && *j != 2 {
                    assert!(!metro_lines.is_same_line(*i, *j), "{} {}", i, j)
                }
            }
        }
    }
}
