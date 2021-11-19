//!
//!

use super::Mat;
use ndarray::Array2;

/// Inform passenger objects about the direction 
/// to take. 
pub struct MetroDirection {
    wrap: MatrixWrapper<usize>,
}

/// Initialize a new MetroDirection object. Function not defined 
/// int MetroDirection to avoid exporting it with the struct 
pub fn new_metro_direction(mat: Mat) -> MetroDirection {
    MetroDirection {
        wrap: MatrixWrapper::new(mat),
    }
}

impl MetroDirection {
    /// Given a passenger that starts at station start and must 
    /// go at station dest this function returns the terminus (or the metro direction) 
    /// of the train that the passenger must take. If there is one (or more) interchange 
    /// in the middle the function returns the direction that 
    /// the passenger must take in order to reach the next interchange station. 
    pub fn get_direction(&self, start: usize, dest: usize) -> usize {
        self.wrap.index(start, dest)
    }
}

/// Inform passenger about the next station to visit to reach 
/// the final destination. 
pub struct MetroInterchange {
    wrap: MatrixWrapper<usize>,
}

/// Initialize a new MetroInterchange object. Function not defined 
/// int MetroInterchange to avoid exporting it with the struct 
pub fn new_metro_interchange(mat: Mat) -> MetroInterchange {
    MetroInterchange {
        wrap: MatrixWrapper::new(mat),
    }
}

impl MetroInterchange {
    /// Given a passenger that starts at station start and must 
    /// go at station dest this function returns the next station 
    /// that the passenger must visit to reach the destination. 
    /// If start and dest are on the same line this function will 
    /// return dest, otherwise function will return the next interchange 
    /// station that the passenger must reach. 
    pub fn next_station(&self, start: usize, dest: usize) -> usize {
        self.wrap.index(start, dest)
    }
}

/// Simple Wrapper around a matrix, help 
/// to contain code duplication in this file. 
struct MatrixWrapper<T> {
    mat: Array2<T>,
}

impl<T> MatrixWrapper<T>
where
    T: Copy,
{
    /// Create a new object 
    fn new(mat: Array2<T>) -> Self {
        Self { mat }
    }

    /// Return item at index (i, j) 
    fn index(&self, i: usize, j: usize) -> T {
        self.mat[(i, j)]
    }
}
