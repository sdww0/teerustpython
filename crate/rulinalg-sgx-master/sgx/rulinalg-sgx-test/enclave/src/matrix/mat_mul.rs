use std::prelude::v1::*;
use rulinalg::matrix::{BaseMatrix, Matrix, MatrixSlice, MatrixSliceMut};

//#[test]
pub fn matrix_mul_f32() {
    let a = matrix![1f32, 2.;
                    3., 4.;
                    5., 6.];
    let b = matrix![1f32, 2., 3.;
                    4., 5., 6.];

    // Allocating new memory
    let c = &a * &b;

    assert_eq!(c.rows(), 3);
    assert_eq!(c.cols(), 3);

    assert_eq!(c[[0, 0]], 9.0);
    assert_eq!(c[[0, 1]], 12.0);
    assert_eq!(c[[0, 2]], 15.0);
    assert_eq!(c[[1, 0]], 19.0);
    assert_eq!(c[[1, 1]], 26.0);
    assert_eq!(c[[1, 2]], 33.0);
    assert_eq!(c[[2, 0]], 29.0);
    assert_eq!(c[[2, 1]], 40.0);
    assert_eq!(c[[2, 2]], 51.0);
}

//#[test]
pub fn matrix_mul_f64() {
    let a = matrix![1f64, 2.;
                    3., 4.;
                    5., 6.];
    let b = matrix![1f64, 2., 3.;
                    4., 5., 6.];

    // Allocating new memory
    let c = &a * &b;

    assert_eq!(c.rows(), 3);
    assert_eq!(c.cols(), 3);

    assert_eq!(c[[0, 0]], 9.0);
    assert_eq!(c[[0, 1]], 12.0);
    assert_eq!(c[[0, 2]], 15.0);
    assert_eq!(c[[1, 0]], 19.0);
    assert_eq!(c[[1, 1]], 26.0);
    assert_eq!(c[[1, 2]], 33.0);
    assert_eq!(c[[2, 0]], 29.0);
    assert_eq!(c[[2, 1]], 40.0);
    assert_eq!(c[[2, 2]], 51.0);
}

//#[test]
pub fn matrix_mul_usize() {
    let a = matrix![1usize, 2;
                    3, 4;
                    5, 6];
    let b = matrix![1usize, 2, 3;
                    4, 5, 6];

    // Allocating new memory
    let c = &a * &b;

    assert_eq!(c.rows(), 3);
    assert_eq!(c.cols(), 3);

    assert_eq!(c[[0, 0]], 9);
    assert_eq!(c[[0, 1]], 12);
    assert_eq!(c[[0, 2]], 15);
    assert_eq!(c[[1, 0]], 19);
    assert_eq!(c[[1, 1]], 26);
    assert_eq!(c[[1, 2]], 33);
    assert_eq!(c[[2, 0]], 29);
    assert_eq!(c[[2, 1]], 40);
    assert_eq!(c[[2, 2]], 51);
}

//#[test]
pub fn mul_slice_basic() {
    let a = 3.0;
    let b = Matrix::ones(2, 2);
    let mut c = Matrix::ones(3, 3) * 2.;
    {
        let d = MatrixSlice::from_matrix(&c, [1, 1], 2, 2);

        let m_1 = &d * a.clone();
        assert_eq!(m_1.into_vec(), vec![6.0; 4]);

        let m_2 = &d * b.clone();
        assert_eq!(m_2.into_vec(), vec![4.0; 4]);

        let m_3 = &d * &d;
        assert_eq!(m_3.into_vec(), vec![8.0; 4]);
    }

    let e = MatrixSliceMut::from_matrix(&mut c, [1, 1], 2, 2);

    let m_1 = &e * a;
    assert_eq!(m_1.into_vec(), vec![6.0; 4]);

    let m_2 = &e * b;
    assert_eq!(m_2.into_vec(), vec![4.0; 4]);

    let m_3 = &e * &e;
    assert_eq!(m_3.into_vec(), vec![8.0; 4]);
}

//#[test]
pub fn mul_slice_uneven_data() {
    let a = matrix![1.0, 2.0; 3.0, 4.0];

    let c = matrix![1.0, 2.0, 3.0;
                    4.0, 5.0, 6.0];
    let d = MatrixSlice::from_matrix(&c, [0, 0], 2, 2);

    let e = d * a;

    assert_eq!(e[[0, 0]], 7.0);
    assert_eq!(e[[0, 1]], 10.0);
    assert_eq!(e[[1, 0]], 19.0);
    assert_eq!(e[[1, 1]], 28.0);
}

//#[test]
pub fn mul_slice_uneven_data_usize() {
    let a = matrix![1usize, 2; 3, 4];

    let c = matrix![1usize, 2, 3; 4, 5, 6];
    let d = MatrixSlice::from_matrix(&c, [0, 0], 2, 2);

    let e = d * a;

    assert_eq!(e[[0, 0]], 7);
    assert_eq!(e[[0, 1]], 10);
    assert_eq!(e[[1, 0]], 19);
    assert_eq!(e[[1, 1]], 28);
}
