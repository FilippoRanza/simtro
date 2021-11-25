//! This module defines a cross index iterator that will return all the unique couple
//! from the Cartesian product (0..n) X (0..n).
//! For example, (a, b) is considered as a duplicate of (b, a). Indexes like (a, a)
//! are also excluded.

/// Define the unique couple iterator. For coherence
/// with the language's rules the iterator will return
/// indexes in the half open interval [0, n[.
/// This struct can be used to define more complex iterators that
/// need to iterate twice on the same vector, like when working
/// with matrices.
pub struct CrossIndexIterator {
    count: usize,
    i: usize,
    j: usize,
}

impl CrossIndexIterator {
    /// Initialize the struct. Requires the maximal value
    /// of the index.
    pub fn new(count: usize) -> Self {
        Self { count, i: 0, j: 0 }
    }

    /// Return the current indexes. Helper function for the
    /// iterator implementation.
    fn get_current(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    /// Set the parameter to the next couple value. Helper function for the
    /// iterator implementation.
    fn update(&mut self) {
        self.j += 1;
        if self.j == self.count {
            self.i += 1;
            self.j = self.i;
        }
    }
}

impl Iterator for CrossIndexIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.j {
            self.update();
        }
        if self.i < self.count && self.j < self.count {
            let output = self.get_current();
            self.update();
            Some(output)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_cross_index_iterator() {
        let cii = CrossIndexIterator::new(3);
        let res: Vec<(usize, usize)> = cii.collect();
        let correct = vec![(0, 1), (0, 2), (1, 2)];
        assert_eq!(correct, res);
    }
}
