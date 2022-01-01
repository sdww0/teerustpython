use std::prelude::v1::*;
use rulinalg::matrix::{Axes, BaseMatrix, Matrix};

//#[test]
pub fn test_new_mat() {
    let a = vec![2.0; 9];
    let b = Matrix::new(3, 3, a);

    assert_eq!(b.rows(), 3);
    assert_eq!(b.cols(), 3);
    assert_eq!(b.into_vec(), vec![2.0; 9]);
}

//#[test]
//#[should_panic]
pub fn test_new_mat_bad_data() {
    let a = vec![2.0; 7];
    let _ = Matrix::new(3, 3, a);
}

//#[test]
pub fn test_new_mat_from_fn() {
    let mut counter = 0;
    let m: Matrix<usize> = Matrix::from_fn(3, 2, |_, _| {
        let value = counter;
        counter += 1;
        value
    });
    assert!(m.rows() == 3);
    assert!(m.cols() == 2);
    assert!(m.data == vec![0, 1, 2, 3, 4, 5]);
}

//#[test]
pub fn test_equality() {
    // well, "PartialEq", at least
    let a = matrix![1., 2., 3.;
                    4., 5., 6.];
    let a_redux = a.clone();
    assert_eq!(a, a_redux);
}

//#[test]
pub fn test_new_from_slice() {
    let data_vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let data_slice: &[u32] = &data_vec[..];
    let from_vec = Matrix::new(3, 2, data_vec.clone());
    let from_slice = Matrix::new(3, 2, data_slice);
    assert_eq!(from_vec, from_slice);
}

//#[test]
pub fn test_display_formatting() {
    let first_matrix = matrix![1, 2, 3;
                               4, 5, 6];
    let first_expectation = "⎡1 2 3⎤\n⎣4 5 6⎦";
    assert_eq!(first_expectation, format!("{}", first_matrix));

    let second_matrix = matrix![3.14, 2.718, 1.414;
                                2.503, 4.669, 1.202;
                                1.618, 0.5772, 1.3;
                                2.68545, 1.282, 10000.];
    let second_exp = "⎡   3.14   2.718   1.414⎤\n⎢  2.503   4.669   1.202⎥\n⎢  \
                    1.618  0.5772     1.3⎥\n⎣2.68545   1.282   10000⎦";
    assert_eq!(second_exp, format!("{}", second_matrix));
}

//#[test]
pub fn test_single_row_display_formatting() {
    let one_row_matrix = matrix![1, 2, 3, 4];
    assert_eq!("[1 2 3 4]", format!("{}", one_row_matrix));
}

//#[test]
pub fn test_display_formatting_precision() {
    let our_matrix = matrix![1.2, 1.23, 1.234;
                             1.2345, 1.23456, 1.234567];
    let expectations = vec!["⎡1.2 1.2 1.2⎤\n⎣1.2 1.2 1.2⎦",

                            "⎡1.20 1.23 1.23⎤\n⎣1.23 1.23 1.23⎦",

                            "⎡1.200 1.230 1.234⎤\n⎣1.234 1.235 1.235⎦",

                            "⎡1.2000 1.2300 1.2340⎤\n⎣1.2345 1.2346 1.2346⎦"];

    for (places, &expectation) in (1..5).zip(expectations.iter()) {
        assert_eq!(expectation, format!("{:.1$}", our_matrix, places));
    }
}

//#[test]
pub fn test_matrix_index_mut() {
    let mut a = Matrix::ones(3, 3) * 2.0;

    a[[0, 0]] = 13.0;

    for i in 1..9 {
        assert_eq!(a.data()[i], 2.0);
    }

    assert_eq!(a[[0, 0]], 13.0);
}

//#[test]
pub fn test_matrix_select_rows() {
    let a = Matrix::new(4, 2, (0..8).collect::<Vec<usize>>());

    let b = a.select_rows(&[0, 2, 3]);

    assert_eq!(b.into_vec(), vec![0, 1, 4, 5, 6, 7]);
}

//#[test]
pub fn test_matrix_select_cols() {
    let a = Matrix::new(4, 2, (0..8).collect::<Vec<usize>>());

    let b = a.select_cols(&[1]);

    assert_eq!(b.into_vec(), vec![1, 3, 5, 7]);
}

//#[test]
pub fn test_matrix_select() {
    let a = Matrix::new(4, 2, (0..8).collect::<Vec<usize>>());

    let b = a.select(&[0, 2], &[1]);

    assert_eq!(b.into_vec(), vec![1, 5]);
}

//#[test]
pub fn matrix_diag() {
    let a = matrix![1., 3., 5.;
                    2., 4., 7.;
                    1., 1., 0.];

    let b = a.is_diag();

    assert!(!b);

    let c = matrix![1., 0., 0.;
                    0., 2., 0.;
                    0., 0., 3.];
    let d = c.is_diag();

    assert!(d);
}

//#[test]
pub fn matrix_det() {
    let a = matrix![2., 3.;
                    1., 2.];
    let b = a.det();

    assert_eq!(b, 1.);

    let c = matrix![1., 2., 3.;
                    4., 5., 6.;
                    7., 8., 9.];
    let d = c.det();

    assert_eq!(d, 0.);

    let e: Matrix<f64> = matrix![1., 2., 3., 4., 5.;
                                 3., 0., 4., 5., 6.;
                                 2., 1., 2., 3., 4.;
                                 0., 0., 0., 6., 5.;
                                 0., 0., 0., 5., 6.];

    let f = e.det();

    assert_scalar_eq!(f, 99.0, comp = float);

    let g: Matrix<f64> = matrix![1., 2., 3., 4.;
                                 0., 0., 0., 0.;
                                 0., 0., 0., 0.;
                                 0., 0., 0., 0.];
    let h = g.det();
    assert_eq!(h, 0.);
}

//#[test]
pub fn matrix_solve() {
    let a = matrix![2., 3.;
                    1., 2.];

    let y = vector![8., 5.];

    let x = a.solve(y).unwrap();

    assert_eq!(x.size(), 2);

    assert_eq!(x[0], 1.);
    assert_eq!(x[1], 2.);
}

//#[test]
pub fn create_mat_zeros() {
    let a = Matrix::<f32>::zeros(10, 10);

    assert_eq!(a.rows(), 10);
    assert_eq!(a.cols(), 10);

    for i in 0..10 {
        for j in 0..10 {
            assert_eq!(a[[i, j]], 0.0);
        }
    }
}

//#[test]
pub fn create_mat_identity() {
    let a = Matrix::<f32>::identity(4);

    assert_eq!(a.rows(), 4);
    assert_eq!(a.cols(), 4);

    assert_eq!(a[[0, 0]], 1.0);
    assert_eq!(a[[1, 1]], 1.0);
    assert_eq!(a[[2, 2]], 1.0);
    assert_eq!(a[[3, 3]], 1.0);

    assert_eq!(a[[0, 1]], 0.0);
    assert_eq!(a[[2, 1]], 0.0);
    assert_eq!(a[[3, 0]], 0.0);
}

//#[test]
pub fn create_mat_diag() {
    let a = Matrix::from_diag(&[1.0, 2.0, 3.0, 4.0]);

    assert_eq!(a.rows(), 4);
    assert_eq!(a.cols(), 4);

    assert_eq!(a[[0, 0]], 1.0);
    assert_eq!(a[[1, 1]], 2.0);
    assert_eq!(a[[2, 2]], 3.0);
    assert_eq!(a[[3, 3]], 4.0);

    assert_eq!(a[[0, 1]], 0.0);
    assert_eq!(a[[2, 1]], 0.0);
    assert_eq!(a[[3, 0]], 0.0);
}

//#[test]
pub fn test_empty_mean() {
    let a: Matrix<f64> = matrix![];

    let c = a.mean(Axes::Row);
    assert_eq!(*c.data(), vec![]);

    let d = a.mean(Axes::Col);
    assert_eq!(*d.data(), vec![]);
}

//#[test]
pub fn test_invalid_variance() {
    // Only one row
    let a: Matrix<f32> = matrix![1.0, 2.0];

    let a_row = a.variance(Axes::Row);
    assert!(a_row.is_err());

    let a_col = a.variance(Axes::Col).unwrap();
    assert_eq!(*a_col.data(), vec![0.5]);

    // Only one column
    let b: Matrix<f32> = matrix![1.0; 2.0];

    let b_row = b.variance(Axes::Row).unwrap();
    assert_eq!(*b_row.data(), vec![0.5]);

    let b_col = b.variance(Axes::Col);
    assert!(b_col.is_err());

    // Empty matrix
    let d: Matrix<f32> = matrix![];

    let d_row = d.variance(Axes::Row);
    assert!(d_row.is_err());

    let d_col = d.variance(Axes::Col);
    assert!(d_col.is_err());
}
