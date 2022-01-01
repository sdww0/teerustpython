use std::prelude::v1::*;
use rulinalg::matrix::{Matrix, BaseMatrix};

//#[test]
pub fn matrix_macro() {
    {
        // An arbitrary rectangular matrix
        let mat = matrix![1, 2, 3;
                          4, 5, 6];
        assert_eq!(2, mat.rows());
        assert_eq!(3, mat.cols());
        assert_eq!(&vec![1, 2, 3, 4, 5, 6], mat.data());
    }

    {
        // A single row
        let mat = matrix![1, 2, 3];
        assert_eq!(1, mat.rows());
        assert_eq!(3, mat.cols());
        assert_eq!(&vec![1, 2, 3], mat.data());
    }

    {
        // A single element
        let mat = matrix![1];
        assert_eq!(1, mat.rows());
        assert_eq!(1, mat.cols());
        assert_eq!(&vec![1], mat.data());
    }

    {
        // A floating point matrix
        let mat = matrix![1.0, 2.0, 3.0;
                          4.0, 5.0, 6.0;
                          7.0, 8.0, 9.0];
        let ref expected_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(3, mat.rows());
        assert_eq!(3, mat.cols());
        assert_eq!(expected_data, mat.data());
    }
}

//#[test]
pub fn matrix_macro_empty_mat() {
    let mat: Matrix<f64> = matrix![];

    assert_eq!(0, mat.rows());
    assert_eq!(0, mat.cols());
}
