//! Keep track of the metro lines. If it is possible
//! to determine the metro lines from the shortest path
//! this module can be used to automatically construct them.

use super::path_iterator::PathIterator;
use super::Mat;

/// Struct used to hold information about
/// the metro lines. This struct can be automatically
/// generated of given as input to the tools.
/// This struct keeps the information about
pub struct MetroLines<'a> {
    lines: Vec<Vec<usize>>,
    terminus: &'a [(usize, usize)],
}

impl<'a> MetroLines<'a> {
    /// Automatically build lines from the successor matrix
    /// and the terminus couples. A metro line is automatically
    /// considered the shortest path between the to terminus stations.
    /// The path for the line defined from terminus (T1, T2) is
    /// always the path from T1 to T2.
    #[must_use]
    pub fn from_successor_matrix(next: &Mat, terminus: &'a [(usize, usize)]) -> Self {
        let lines = terminus
            .iter()
            .map(|(t1, t2)| PathIterator::new(*t1, *t2, next).into_vector())
            .collect();
        Self { lines, terminus }
    }

    /// Build object from pre-build lines. Useful if the lines cannot be generated
    /// automatically from the shortest path.
    #[must_use]
    pub fn from_given_lines(lines: Vec<Vec<usize>>, terminus: &'a [(usize, usize)]) -> Self {
        Self { lines, terminus }
    }

    /// Return an iterator implementation over
    /// the available lines. The return order is always the same
    /// of the given terminus line on object construction.
    /// The path for the line defined from terminus (T1, T2) is
    /// always the path from T1 to T2.
    pub fn line_iter(&'a self) -> impl Iterator<Item = &[usize]> + 'a {
        self.lines.iter().map(Vec::as_slice)
    }

    /// Return a reference to the given terminus list on object
    /// construction.
    #[must_use]
    pub fn get_terminus(&'a self) -> &'a [(usize, usize)] {
        self.terminus
    }
}

#[cfg(test)]
mod test {

    use super::super::test_definitions;
    use super::*;

    #[test]
    fn test_line_construction() {
        let next = test_definitions::make_next_matrix();
        let terminus = test_definitions::make_terminus();

        let metro_lines = MetroLines::from_successor_matrix(&next, &terminus);
        let correct_lines = vec![vec![0, 1, 2, 5, 6], vec![4, 3, 2, 7, 8]];
        assert_eq!(metro_lines.lines, correct_lines);
    }
}
