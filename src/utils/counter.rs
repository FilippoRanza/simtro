//! This module implement a simple counter and cyclic counter (counter that restarts
//! at the end)

/// One time counter implementation.
#[derive(Debug, PartialEq)]
pub struct Counter {
    start: usize,
    current: usize,
}

/// For easy struct initialization
impl From<usize> for Counter {
    fn from(i: usize) -> Self {
        Self::new(i)
    }
}

impl Counter {
    /// Initialize struct specifing the target value
    #[must_use]
    pub fn new(start: usize) -> Self {
        Self { start, current: 0 }
    }

    /// Count one. This method will return true after start
    /// call. Once this method returns true it will continue until
    /// reset method is called
    /// ```
    /// use simtro::utils::counter::Counter;
    ///
    /// let mut counter = Counter::new(3);
    ///
    /// assert!(!counter.step());
    /// assert!(!counter.step());
    /// assert!(!counter.step());
    /// assert!(counter.step());
    /// assert!(counter.step());
    ///
    /// ```
    pub fn step(&mut self) -> bool {
        if self.is_done() {
            true
        } else {
            self.current += 1;
            false
        }
    }

    /// Return true if the counter is done. This method
    /// does not modify the counter
    #[must_use]
    pub fn is_done(&self) -> bool {
        self.current >= self.start
    }

    /// Reset counter to inittial state
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

/// Cyclic Counter implementation.
#[derive(Debug, PartialEq)]
pub struct CyclicCounter {
    counter: Counter,
}

/// For easy struct initialization
impl From<usize> for CyclicCounter {
    fn from(i: usize) -> Self {
        Self::new(i)
    }
}

impl CyclicCounter {
    /// Initialize struct specifing the target value
    #[must_use]
    pub fn new(target: usize) -> Self {
        Self {
            counter: Counter::new(target),
        }
    }

    /// Count one. This method will return true once after target
    /// call. The subsequent call will return false until
    /// target value is reached again.
    /// ```
    /// use simtro::utils::counter::CyclicCounter;
    ///
    /// let mut cyclic_counter = CyclicCounter::new(3);
    /// assert!(!cyclic_counter.count());
    /// assert!(!cyclic_counter.count());
    /// assert!(!cyclic_counter.count());
    /// assert!(cyclic_counter.count());
    /// assert!(!cyclic_counter.count());
    /// assert!(!cyclic_counter.count());
    /// assert!(!cyclic_counter.count());
    /// assert!(cyclic_counter.count());
    ///
    /// ```
    pub fn count(&mut self) -> bool {
        let stat = self.counter.step();
        if stat {
            self.counter.reset();
        }
        stat
    }
    #[must_use]
    pub fn is_done(&self) -> bool {
        self.counter.is_done()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new(3);
        assert!(!counter.step());
        assert!(!counter.step());
        assert!(!counter.step());

        assert!(counter.step());
        assert_eq!(counter.current, 3);

        assert!(counter.step());
        assert_eq!(counter.current, 3);

        assert!(counter.step());
        assert_eq!(counter.current, 3);

        counter.reset();
        assert!(!counter.step());
        assert!(!counter.step());
        assert!(!counter.step());
        assert!(counter.step());
        assert_eq!(counter.current, 3);
    }

    #[test]
    fn test_cyclic_counter() {
        let mut counter = CyclicCounter::new(5);
        // Count one
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(counter.count());

        // Count two
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(counter.count());

        // Count three
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(!counter.count());
        assert!(counter.count());
    }
}
