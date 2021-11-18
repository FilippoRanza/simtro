use super::Mat;

use std::collections::HashSet;

pub struct PathIterator<'a> {
    curr: usize,
    end: usize,
    last: Option<usize>,
    next_mat: &'a Mat,
}

impl<'a> PathIterator<'a> {
    pub fn new(curr: usize, end: usize, next_mat: &'a Mat) -> Self {
        Self {
            curr,
            end,
            next_mat,
            last: None,
        }
    }

    pub fn to_vector(self) -> Vec<usize> {
        self.collect()
    }

    pub fn to_set(self) -> HashSet<usize> {
        self.collect()
    }
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr != self.end {
            let tmp = self.curr;
            self.curr = self.next_mat[(self.curr, self.end)];
            self.last = Some(self.curr);
            Some(tmp)
        } else {
            self.last.take()
        }
    }
}

#[cfg(test)]
mod test {

    use super::super::test_definitions::make_next_matrix;
    use super::*;
    #[test]
    fn test_path_iterator() {
        let next_mat = make_next_matrix();
        let mut path_iter = PathIterator::new(0, 6, &next_mat);
        assert_eq!(path_iter.next(), Some(0));
        assert_eq!(path_iter.next(), Some(1));
        assert_eq!(path_iter.next(), Some(2));
        assert_eq!(path_iter.next(), Some(5));
        assert_eq!(path_iter.next(), Some(6));
        assert_eq!(path_iter.next(), None);
    }
}
