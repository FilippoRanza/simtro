//!
//! Manage the metro system line. This auxiliary module is designed to 
//! help the construction of the interchange path matrix and the direction matrix. 

use std::collections::HashSet;

use crate::utils::cross_index_iterator;

use super::path_iterator::PathIterator;
use super::Mat;

/// Just for brevity 
type Line = HashSet<usize>;

/// Just for brevity 
type Set<'a> = &'a Line;

/// Helper struct used to determine if 
/// two stations are on the same line or not. 
/// From this struct is also possible to get two iterators. The first 
/// one returns all the couples terminus, line. The second return all 
/// the unique couples line A, line B without repetitions. 
pub struct MetroLines<'a> {
    terminus: &'a [(usize, usize)],
    lines: Vec<Line>,
}

impl<'a> MetroLines<'a> {
/// Iniitialize Line data structure. Start from the successor matrix 
/// and the terminus list. Order inside the list and order between station is 
/// irrelevant. 
    pub fn new(next: &Mat, terminus: &'a [(usize, usize)]) -> Self {
        let lines = terminus
            .iter()
            .map(|(t1, t2)| PathIterator::new(*t1, *t2, next).to_set())
            .collect();
        MetroLines { terminus, lines }
    }

/// Tell if station s1 and station s2 are on the same metro line or not. 
    pub fn is_same_line(&self, s1: usize, s2: usize) -> bool {
        for line in &self.lines {
            if line.contains(&s1) && line.contains(&s2) {
                return true;
            }
        }
        false
    }

/// Return an IntoIterator that iterates through the couple 
/// (start, end) with the associated line station. start and end 
/// are inside the object. 
    pub fn line_iterator(&'a self) -> impl IntoIterator<Item = LineItem<'a>> {
        self.terminus
            .iter()
            .zip(self.lines.iter())
            .map(|t| LineItem::from_tuple(t))
    }

/// Iterate though all the unique couple of lines. Unique means that 
/// if couple (a, b) is returend the iterator will never generate couple (b, a). 
    pub fn cross_line_iter(&'a self) -> impl IntoIterator<Item = (Set<'a>, Set<'a>)> {
        CrossLineIterator::new(&self.lines)
    }
}

/// struct used to hold the output couple from the line iterator. Here just to give 
/// more 
/// order then a simple tuple. 
pub struct LineItem<'a> {
    pub terminus: (usize, usize),
    pub stations: Set<'a>,
}

impl<'a> LineItem<'a> {
/// Initialize data structure. The input is data from a zip iterator. Only for in module 
/// usage. 
    fn from_tuple(t: (&(usize, usize), &'a HashSet<usize>)) -> Self {
        let ((t1, t2), stations) = t;
        let terminus = (*t1, *t2);
        Self { terminus, stations }
    }
}

/// Implemente the unique couple line iterator. 
/// The actual index iterator is not implemented here. See [`crate::utils::cross_index_iterator::CrossIndexIterator`] 
/// for that. 
pub struct CrossLineIterator<'a> {
    lines: &'a Vec<HashSet<usize>>,
    iterator: cross_index_iterator::CrossIndexIterator,
}

impl<'a> CrossLineIterator<'a> {
/// Initialize struct. Takes as input the lines list. 
    fn new(lines: &'a Vec<HashSet<usize>>) -> Self {
        let iterator = cross_index_iterator::CrossIndexIterator::new(lines.len());
        Self { lines, iterator }
    }
}

impl<'a> Iterator for CrossLineIterator<'a> {
    type Item = (Set<'a>, Set<'a>);
/// Iterate though all unique list couples. 
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
        /*
         Ensure that the HashSet and the vectors are properly initialized
        */
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let metro_lines = MetroLines::new(&next, &terminus);
        let lines = metro_lines.lines;
        let correct_line_1 = HashSet::from([0, 1, 2, 5, 6]);
        let correct_line_2 = HashSet::from([4, 3, 2, 7, 8]);
        let expected = vec![correct_line_1, correct_line_2];
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_same_line_check() {
        /*
            Ensure that station on same lines and stations
            on different lines are correctly recognized.
        */
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();
        let metro_lines = MetroLines::new(&next, &terminus);
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
