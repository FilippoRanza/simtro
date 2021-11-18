//! General function to handle matrix in a specify way that is required by multiple 
//! portion of the code but is too specific to be defined in the ndarray matrix.
//! Most of this code is just used to keep the code sorter where the real stuff happens.


use ndarray::Array2;

/// Create a square matrix of size n  and initialize the value of 
/// each item to the default value of the output parameter.
pub fn zeros<D: Default + Clone>(n: usize) -> Array2<D> {
    Array2::from_elem((n, n), D::default())
}

/// Create a zero matrix with the same size and type of the input 
/// argument. 
pub fn zeros_as<D: Default + Clone>(m: &Array2<D>) -> Array2<D> {
    let shape = m.raw_dim();
    Array2::from_elem(shape, D::default())
}
