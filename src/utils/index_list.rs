use super::zeros;

pub trait Indexer {
    fn index(&self) -> usize;
}

pub struct IndexList<T> {
    list: Vec<Vec<T>>,
}

impl<T> IndexList<T>
where
    T: Indexer,
{
    pub fn new(capacity: usize) -> Self {
        let list = zeros(capacity);
        Self { list }
    }

    pub fn push(&mut self, t: T) {
        let i = t.index();
        self.list[i].push(t);
    }

    pub fn get_list_mut(&mut self, i: usize) -> &'_ mut Vec<T> {
        &mut self.list[i]
    }

    pub fn get_list(&self, i: usize) -> &'_ Vec<T> {
        &self.list[i]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_index_list() {
        let mut index_list = IndexList::new(10);

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

    impl Indexer for usize {
        fn index(&self) -> usize {
            *self / 10
        }
    }
}
