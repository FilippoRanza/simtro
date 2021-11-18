use ndarray::Array2;

pub fn zeros<D: Default + Clone>(n: usize) -> Array2<D> {
    Array2::from_elem((n, n), D::default())
}

pub fn zeros_as<D: Default + Clone>(m: &Array2<D>) -> Array2<D> {
    let shape = m.raw_dim();
    Array2::from_elem(shape, D::default())
}
