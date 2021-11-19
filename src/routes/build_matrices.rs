//! This module perform Metro Destination Matrix and 
//! Interchange Path Matrix initialization 

use super::interchange_path;
use super::lines;
use super::metro_direction;
use super::Mat;
use ndarray::Array2;
use num_traits::PrimInt;

/// Store the just generated Metro Destination Matrix and 
/// the Interchange Path Matrix 
pub struct PathMatrix {
    pub mdm: Mat,
    pub ipm: Mat,
}

impl PathMatrix {
    /// Build the Matrices using the code in [`super::metro_direction`] and 
    /// [`super::interchange_path`] 
    pub fn init_matrices<T: PrimInt>(
        next_mat: &Mat,
        dist_mat: &Array2<T>,
        terminus: &[(usize, usize)],
    ) -> Self {
        let metro_lines = lines::MetroLines::new(next_mat, terminus);
        let ipm = interchange_path::build_interchange_path_matrix(next_mat, &metro_lines);
        let mdm = metro_direction::build_metro_direction(next_mat, dist_mat, &metro_lines, &ipm);
        Self { ipm, mdm }
    }
}

#[cfg(test)]
mod test {

    use super::super::test_definitions;
    use super::*;

    #[test]
    fn test_path_matrix_build() {
        let next = test_definitions::make_next_matrix();
        let dist = test_definitions::make_dist_matrix();
        let term = test_definitions::make_terminus();
        let path_matrix = PathMatrix::init_matrices(&next, &dist, &term);

        assert_eq!(
            path_matrix.ipm,
            test_definitions::make_correct_interchange_path()
        );
        assert_eq!(
            path_matrix.mdm,
            test_definitions::make_correct_direction_matrix()
        );
    }
}
