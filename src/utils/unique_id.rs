//! Define a simple counter useful to define unique IDs for objects.

/// Counter that starts from zero and continues up to
/// ``u32::MAX``. No checks are performed at that point,
/// it is considered unlikely that the program will need that
/// many IDs. This struct does not implements a global state,
/// so IDs returned by different instances will be duplicated.
pub struct UniqueId {
    curr: u32,
}

impl UniqueId {
    /// Initialize the struct.
    #[must_use]
    pub fn new() -> Self {
        Self { curr: 0 }
    }

    /// Return the next unique id for this instance.
    pub fn next_uid(&mut self) -> u32 {
        let tmp = self.curr;
        self.curr += 1;
        tmp
    }

    /// Set to the given object's id value to the next
    /// id value for this instance.
    pub fn set_id<T: SetId>(&mut self, t: T) -> T {
        t.set_id(self.next_uid())
    }

    /// Set to each element of the given iterator the value
    /// of the id. The iterator is not modified in place but
    /// returned as output.
    pub fn set_id_iter<'a, T: SetId, I: Iterator<Item = T> + 'a>(
        &'a mut self,
        iter: I,
    ) -> impl Iterator<Item = T> + 'a {
        iter.map(|t| t.set_id(self.next_uid()))
    }
}

impl Default for UniqueId {
    fn default() -> Self {
        Self::new()
    }
}

/// Define the ``SetId`` trait used to set the ID to the given object.
/// For simplicity the object is consumed by this function and returned as
/// output.
pub trait SetId {
    fn set_id(self, id: u32) -> Self;
}
