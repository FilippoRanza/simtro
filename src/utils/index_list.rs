//! This module implements an indexed collection. Items 
//! inside the collection are segregated into different sub collections
//! based on ther value of the index function.

use super::zeros;

/// Trait defining the index function that maps
/// an object into the proper sublist.
pub trait Indexer<T> {
    /// specify the sub group that this item 
    /// belongs to. Value from this function must 
    /// be in range [0 - IndexList.capacity]; code 
    /// panics otherwise.
    fn index(&self, t: &T) -> usize;
}

/// Collect give objects into sub collection based 
/// on thier value of the index function. The
pub struct IndexList<T, I> {
    list: Vec<Vec<T>>,
    index: I,
}

impl<T, I> IndexList<T, I>
where
    I: Indexer<T> + Default,
{
    /// Create a new IndexList with given capacity and 
    /// prebuild Indexer
    pub fn new(capacity: usize, index: I) -> Self {
        let list = zeros(capacity);
        Self { list, index }
    }

    /// Create a new IndexList with given capacity and 
    /// default Indexer    
    pub fn new_with_default_index(capacity: usize) -> Self {
        Self::new(capacity, Default::default())
    }

    /// Add element to collection. Element is 
    /// inserted info proper sub collection.
    pub fn push(&mut self, t: T) {
        let i = self.index.index(&t);
        self.list[i].push(t);
    }

    /// Return a mutable reference to the i-th sub collection
    pub fn get_list_mut(&mut self, i: usize) -> &'_ mut Vec<T> {
        &mut self.list[i]
    }

    /// Return a reference to the i-th sub collection
    pub fn get_list(&self, i: usize) -> &'_ Vec<T> {
        &self.list[i]
    }

    /// Add all items from other into the collection,
    /// other is left empty (but with untouched capacity )
    pub fn append(&mut self, other: &mut Vec<T>) {
        while let Some(i) = other.pop() {
            self.push(i)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_index_list() {
        let mut index_list = IndexList::new(10, 10);

        index_list.push(6);
        index_list.push(16);
        index_list.push(26);
        index_list.push(27);

        let correct = vec![
            vec![6],
            vec![16],
            vec![26, 27],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        assert_eq!(correct, index_list.list);
    }

    impl Indexer<usize> for usize {
        fn index(&self, i: &usize) -> usize {
            *i / *self
        }
    }
}
