use std::prelude::v1::*;
use rulinalg::matrix::{Matrix, MatrixSlice, MatrixSliceMut, BaseMatrix, Axes};

//#[test]
//#[should_panic]
pub fn make_slice_bad_dim() {
    let a = Matrix::ones(3, 3) * 2.0;
    let _ = MatrixSlice::from_matrix(&a, [1, 1], 3, 2);
}

//#[test]
pub fn make_slice() {
    let a = Matrix::ones(3, 3) * 2.0;
    let b = MatrixSlice::from_matrix(&a, [1, 1], 2, 2);

    assert_eq!(b.rows(), 2);
    assert_eq!(b.cols(), 2);
}

//#[test]
pub fn make_slice_mut() {
    let mut a = Matrix::ones(3, 3) * 2.0;
    {
        let mut b = MatrixSliceMut::from_matrix(&mut a, [1, 1], 2, 2);
        assert_eq!(b.rows(), 2);
        assert_eq!(b.cols(), 2);
        b += 2.0;
    }
    let exp = matrix![2.0, 2.0, 2.0;
                      2.0, 4.0, 4.0;
                      2.0, 4.0, 4.0];
    assert_matrix_eq!(a, exp);

}

//#[test]
pub fn matrix_min_max() {
    let a = matrix![1., 3., 5., 4.;
                    2., 4., 7., 1.;
                    1., 1., 0., 0.];
    assert_eq!(a.min(Axes::Col), vector![1., 1., 0.]);
    assert_eq!(a.min(Axes::Row), vector![1., 1., 0., 0.]);

    assert_eq!(a.max(Axes::Col), vector![5., 7., 1.]);
    assert_eq!(a.max(Axes::Row), vector![2., 4., 7., 4.]);

    let r = matrix![1., 3., 5., 4.];
    assert_eq!(r.min(Axes::Col), vector![1.]);
    assert_eq!(r.min(Axes::Row), vector![1., 3., 5., 4.]);

    assert_eq!(r.max(Axes::Col), vector![5.]);
    assert_eq!(r.max(Axes::Row), vector![1., 3., 5., 4.]);

    let c = matrix![1.; 2.; 3.];
    assert_eq!(c.min(Axes::Col), vector![1., 2., 3.]);
    assert_eq!(c.min(Axes::Row), vector![1.]);

    assert_eq!(c.max(Axes::Col), vector![1., 2., 3.]);
    assert_eq!(c.max(Axes::Row), vector![3.]);

    let t = matrix![1., 2.; 0., 1.];
    assert_eq!(t.min(Axes::Col), vector![1., 0.]);
    assert_eq!(t.min(Axes::Row), vector![0., 1.]);

    assert_eq!(t.max(Axes::Col), vector![2., 1.]);
    assert_eq!(t.max(Axes::Row), vector![1., 2.]);
}
