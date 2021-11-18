pub struct CrossIndexIterator {
    count: usize,
    i: usize,
    j: usize,
}

impl CrossIndexIterator {
    pub fn new(count: usize) -> Self {
        Self { count, i: 0, j: 0 }
    }

    fn get_current(&self) -> (usize, usize) {
        (self.i, self.j)
    }

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
        let correct = vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (2, 2)];
        assert_eq!(correct, res);
    }
}
