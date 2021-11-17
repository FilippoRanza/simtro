use ndarray::Array2;

pub fn zeros<D: Default + Clone>(n: usize) -> Array2<D> {
    Array2::from_elem((n, n), D::default())
}
