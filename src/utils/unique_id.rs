pub struct UniqueId {
    curr: u32,
}

impl UniqueId {
    pub fn new() -> Self {
        Self { curr: 0 }
    }

    pub fn next(&mut self) -> u32 {
        let tmp = self.curr;
        self.curr += 1;
        tmp
    }

    pub fn set_id<T: SetId>(&mut self, t: T) -> T {
        t.set_id(self.next())
    }

    pub fn set_id_iter<'a, T: SetId, I: Iterator<Item = T> + 'a>(
        &'a mut self,
        iter: I,
    ) -> impl Iterator<Item = T> + 'a {
        iter.map(|t| t.set_id(self.next()))
    }
}

pub trait SetId {
    fn set_id(self, id: u32) -> Self;
}
