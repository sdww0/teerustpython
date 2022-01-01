use std::prelude::v1::*;
use rulinalg::matrix::{DiagOffset, Matrix, MatrixSlice, MatrixSliceMut,
             BaseMatrix, BaseMatrixMut};
use rulinalg::vector::Vector;

//#[test]
pub fn inner_product_as_matrix_multiplication() {
    let u: Vector<f32> = vector![1., 2., 3.];
    let v: Vector<f32> = vector![3., 4., 5.];
    let dot_product = u.dot(&v);

    let um: Matrix<f32> = u.into();
    let vm: Matrix<f32> = v.into();
    let matrix_product = um.transpose() * vm;

    assert_eq!(dot_product, matrix_product.data()[0]);
}

//#[test]
pub fn matrix_from_slice() {
    let mut a = Matrix::new(3, 3, vec![2.0; 9]);

    {
        let b = MatrixSlice::from_matrix(&a, [1, 1], 2, 2);
        let c = Matrix::from(b);
        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 2);
    }

    let d = MatrixSliceMut::from_matrix(&mut a, [1, 1], 2, 2);
    let e = Matrix::from(d);
    assert_eq!(e.rows(), 2);
    assert_eq!(e.cols(), 2);
}

//#[test]
pub fn diag_offset_from_int() {
    let a: DiagOffset = 3.into();
    assert_eq!(a, DiagOffset::Above(3));
    let a: DiagOffset = (-3).into();
    assert_eq!(a, DiagOffset::Below(3));
    let a: DiagOffset = 0.into();
    assert_eq!(a, DiagOffset::Main);
}

//#[test]
pub fn try_into_empty_matrix() {
    {
        let x: Matrix<f64> = matrix![];
        let y: Matrix<f32> = x.try_into().unwrap();
        assert_matrix_eq!(y, matrix![]);
    }

    {
        let x: Matrix<u64> = matrix![];
        let y: Matrix<u32> = x.try_into().unwrap();
        assert_matrix_eq!(y, matrix![]);
    }

    {
        let x: Matrix<f64> = matrix![];
        let y: Matrix<u64> = x.try_into().unwrap();
        assert_matrix_eq!(y, matrix![]);
    }

    {
        let x: Matrix<u8> = matrix![];
        let y: Matrix<u64> = x.try_into().unwrap();
        assert_matrix_eq!(y, matrix![]);
    }
}

//#[test]
pub fn try_into_f64_to_i64() {
    let x: Matrix<f64> = matrix![ 1.0, 2.0;
                                 -3.0, 4.0];
    let y: Matrix<i64> = x.try_into().unwrap();
    let expected = matrix![ 1, 2;
                           -3, 4];
    assert_matrix_eq!(y, expected);
}

//#[test]
pub fn try_into_f64_to_u64() {
    let x: Matrix<f64> = matrix![ 1.0, 2.0;
                                  3.0, 4.0];
    let y: Matrix<u64> = x.try_into().unwrap();
    let expected = matrix![ 1, 2;
                            3, 4];
    assert_matrix_eq!(y, expected);
}

//#[test]
pub fn try_into_i64_to_f64() {
    {
        let x: Matrix<i64> = matrix![ 1, 2;
                                     -3, 4];
        let y: Matrix<f64> = x.try_into().unwrap();

        let expected = matrix![ 1.0, 2.0;
                               -3.0, 4.0];
        assert_matrix_eq!(y, expected);
    }

    {
        // Recall that f64 cannot exactly represent integers of sufficiently
        // large absolute value. Yet, Rust will cast and round as necessary,
        // so we only check that the result is Ok.
        {
            let x: Matrix<i64> = matrix![1, 2, i64::max_value()];
            let y_result = x.try_into::<f64>();
            assert!(y_result.is_ok());
        }

        {
            let x: Matrix<i64> = matrix![1, 2, i64::min_value()];
            let y_result = x.try_into::<f64>();
            assert!(y_result.is_ok());
        }
    }
}

//#[test]
pub fn try_into_u64_to_f64() {
    {
        let x: Matrix<u64> = matrix![ 1, 2;
                                      3, 4];
        let y: Matrix<f64> = x.try_into().unwrap();

        let expected = matrix![ 1.0, 2.0;
                                3.0, 4.0];
        assert_matrix_eq!(y, expected);
    }

    {
        // Recall that f64 cannot exactly represent integers of sufficiently
        // large absolute value. Yet, Rust will cast and round as necessary,
        // so we only check that the result is Ok.
        {
            let x: Matrix<u64> = matrix![1, 2, u64::max_value()];
            let y_result = x.try_into::<f64>();
            assert!(y_result.is_ok());
        }
    }
}

//#[test]
pub fn try_into_signed_unsigned() {
    {
        let x: Matrix<u64> = matrix![ 1, 2;
                                      3, 4];
        let y: Matrix<i64> = x.try_into().unwrap();

        let expected = matrix![ 1, 2;
                                3, 4];
        assert_matrix_eq!(y, expected);
    }

    {
        let x: Matrix<i64> = matrix![ 1, 2;
                                      3, 4];
        let y: Matrix<u64> = x.try_into().unwrap();

        let expected = matrix![ 1, 2;
                                3, 4];
        assert_matrix_eq!(y, expected);
    }

    {
        // Cannot cast negative values into unsigned
        let x = matrix![ 1, -2;
                         3,  4];
        let y_result = x.try_into::<u64>();
        assert!(y_result.is_err());
    }
}

//#[test]
pub fn test_row_convert() {
    let a: Matrix<i64> = matrix![1, 2, 3, 4;
                                 5, 6, 7, 8;
                                 9, 10, 11, 12];
    let row = a.row(1);
    let v: Vector<i64> = row.into();
    assert_eq!(v, vector![5, 6, 7, 8]);

    let row = a.row(2);
    let v = Vector::from(row);
    assert_eq!(v, vector![9, 10, 11, 12]);
}

//#[test]
pub fn test_row_convert_mut() {
    let mut a: Matrix<i64> = matrix![1, 2, 3, 4;
                                     5, 6, 7, 8;
                                     9, 10, 11, 12];

    let row = a.row_mut(1);
    let v: Vector<i64> = row.into();
    assert_eq!(v, vector![5, 6, 7, 8]);

    let mut a: Matrix<i64> = matrix![1, 2, 3, 4;
                                     5, 6, 7, 8;
                                     9, 10, 11, 12];
    let row = a.row_mut(2);
    let v = Vector::from(row);
    assert_eq!(v, vector![9, 10, 11, 12]);
}

//#[test]
pub fn test_column_convert() {
    let a: Matrix<i64> = matrix![1, 2, 3, 4;
                                 5, 6, 7, 8;
                                 9, 10, 11, 12];
    let row = a.col(1);
    let v: Vector<i64> = row.into();
    assert_eq!(v, vector![2, 6, 10]);

    let row = a.col(2);
    let v = Vector::from(row);
    assert_eq!(v, vector![3, 7, 11]);
}

//#[test]
pub fn test_column_convert_mut() {
    let mut a: Matrix<i64> = matrix![1, 2, 3, 4;
                                     5, 6, 7, 8;
                                     9, 10, 11, 12];

    let row = a.col_mut(1);
    let v: Vector<i64> = row.into();
    assert_eq!(v, vector![2, 6, 10]);

    let mut a: Matrix<i64> = matrix![1, 2, 3, 4;
                                     5, 6, 7, 8;
                                     9, 10, 11, 12];
    let row = a.col_mut(2);
    let v = Vector::from(row);
    assert_eq!(v, vector![3, 7, 11]);
}
