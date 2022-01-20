use std::vec::IntoIter;
pub struct MixingIterator<T, K> {
    t: IntoIter<T>,
    k: IntoIter<K>,
    first: bool,
}

impl<T, K> MixingIterator<T, K> {
    pub fn new(t: IntoIter<T>, k: IntoIter<K>) -> Self {
        let first = true;
        Self { t, k, first }
    }
}

impl<T, K> Iterator for MixingIterator<T, K> {
    type Item = MixingIteratorItem<T, K>;

    fn next(&mut self) -> Option<Self::Item> {
        let output = if self.first {
            let t = self.t.next()?;
            MixingIteratorItem::T(t)
        } else {
            let k = self.k.next()?;
            MixingIteratorItem::K(k)
        };
        self.first = !self.first;
        Some(output)
    }
}

#[derive(PartialEq, Debug)]
pub enum MixingIteratorItem<T, K> {
    T(T),
    K(K),
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_mixing_itertor() {
        let a = vec![4, 5, 6];
        let b = vec![2., 3.];

        let mut iterator = MixingIterator::new(a.into_iter(), b.into_iter());
        assert_eq!(iterator.next(), Some(MixingIteratorItem::T(4)));
        assert_eq!(iterator.next(), Some(MixingIteratorItem::K(2.)));
        assert_eq!(iterator.next(), Some(MixingIteratorItem::T(5)));
        assert_eq!(iterator.next(), Some(MixingIteratorItem::K(3.)));
        assert_eq!(iterator.next(), Some(MixingIteratorItem::T(6)));
        assert_eq!(iterator.next(), None);
    }
}