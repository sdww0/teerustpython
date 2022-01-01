use std::prelude::v1::*;
use libnum::Float;
use std::f64;

use rulinalg::norm::*;
use rulinalg::vector::Vector;
use rulinalg::matrix::{Matrix, MatrixSlice};

//#[test]
pub fn test_euclidean_vector_norm() {
    let v = vector![3.0, 4.0];
    assert_scalar_eq!(VectorNorm::norm(&Euclidean, &v), 5.0, comp = float);
}

//#[test]
pub fn test_euclidean_matrix_norm() {
    let m = matrix![3.0, 4.0;
                    1.0, 3.0];
    assert_scalar_eq!(MatrixNorm::norm(&Euclidean, &m), 35.0.sqrt(), comp = float);
}

//#[test]
pub fn test_euclidean_matrix_slice_norm() {
    let m = matrix![3.0, 4.0;
                    1.0, 3.0];

    let slice = MatrixSlice::from_matrix(&m, [0,0], 1, 2);
    assert_scalar_eq!(MatrixNorm::norm(&Euclidean, &slice), 5.0, comp = float);
}

//#[test]
pub fn test_euclidean_vector_metric() {
    let v = vector![3.0, 4.0];
    assert_scalar_eq!(VectorMetric::metric(&Euclidean, &v, &v), 0.0, comp = float);

    let v1 = vector![0.0, 0.0];
    assert_scalar_eq!(VectorMetric::metric(&Euclidean, &v, &v1), 5.0, comp = float);

    let v2 = vector![4.0, 3.0];
    assert_scalar_eq!(VectorMetric::metric(&Euclidean, &v, &v2), 2.0.sqrt(), comp = float);
}

//#[test]
//#[should_panic]
pub fn test_euclidean_vector_metric_bad_dim() {
    let v = vector![3.0, 4.0];
    let v2 = vector![1.0, 2.0, 3.0];

    VectorMetric::metric(&Euclidean, &v, &v2);
}

//#[test]
pub fn test_euclidean_matrix_metric() {
    let m = matrix![3.0, 4.0;
                    1.0, 3.0];
    assert_scalar_eq!(MatrixMetric::metric(&Euclidean, &m, &m), 0.0, comp = float);

    let m1 = Matrix::zeros(2, 2);
    assert_scalar_eq!(MatrixMetric::metric(&Euclidean, &m, &m1), 35.0.sqrt(), comp = float);

    let m2 = matrix![2.0, 3.0;
                     2.0, 4.0];
    assert_scalar_eq!(MatrixMetric::metric(&Euclidean, &m, &m2), 2.0, comp = float);
}

//#[test]
//#[should_panic]
pub fn test_euclidean_matrix_metric_bad_dim() {
    let m = matrix![3.0, 4.0];
    let m2 = matrix![1.0, 2.0, 3.0];

    MatrixMetric::metric(&Euclidean, &m, &m2);
}

//#[test]
pub fn test_euclidean_matrix_slice_metric() {
    let m = matrix![
        1.0, 1.0, 1.0;
        1.0, 1.0, 1.0;
        1.0, 1.0, 1.0
    ];

    let m2 = matrix![
        0.0, 0.0, 0.0;
        0.0, 0.0, 0.0;
        0.0, 0.0, 0.0
    ];

    let m_slice = MatrixSlice::from_matrix(
        &m, [0; 2], 1, 2
    );

    let m2_slice = MatrixSlice::from_matrix(
        &m2, [0; 2], 1, 2
    );

    assert_scalar_eq!(MatrixMetric::metric(&Euclidean, &m_slice, &m2_slice), 2.0.sqrt(), comp = exact);
}

//#[test]
//#[should_panic]
pub fn test_euclidean_matrix_slice_metric_bad_dim() {
    let m = matrix![3.0, 4.0];
    let m2 = matrix![1.0, 2.0, 3.0];

    let m_slice = MatrixSlice::from_matrix(
        &m, [0; 2], 1, 1
    );

    let m2_slice = MatrixSlice::from_matrix(
        &m2, [0; 2], 1, 2
    );

    MatrixMetric::metric(&Euclidean, &m_slice, &m2_slice);
}

//#[test]
pub fn test_lp_vector_supremum() {
    let v = vector![-5.0, 3.0];

    let sup = VectorNorm::norm(&Lp::Infinity, &v);
    assert_eq!(sup, 5.0);
}

//#[test]
pub fn test_lp_matrix_supremum() {
    let m = matrix![0.0, -2.0;
                    3.5, 1.0];

    let sup = MatrixNorm::norm(&Lp::Infinity, &m);
    assert_eq!(sup, 3.5);
}

//#[test]
pub fn test_lp_vector_one() {
    let v = vector![1.0, 2.0, -2.0];
    assert_eq!(VectorNorm::norm(&Lp::Integer(1), &v), 5.0);
}

//#[test]
pub fn test_lp_matrix_one() {
    let m = matrix![1.0, -2.0;
                    0.5, 1.0];
    assert_eq!(MatrixNorm::norm(&Lp::Integer(1), &m), 4.5);
}

//#[test]
pub fn test_lp_vector_float() {
    let v = vector![1.0, 2.0, -2.0];
    assert_eq!(VectorNorm::norm(&Lp::Float(1.0), &v), 5.0);
}

//#[test]
pub fn test_lp_matrix_float() {
    let m = matrix![1.0, -2.0;
                    0.5, 1.0];
    assert_eq!(MatrixNorm::norm(&Lp::Float(1.0), &m), 4.5);
}

//#[test]
//#[should_panic]
pub fn test_lp_vector_bad_p() {
    let v = Vector::new(vec![]);
    VectorNorm::norm(&Lp::Float(0.5), &v);
}

//#[test]
//#[should_panic]
pub fn test_lp_matrix_bad_p() {
    let m = matrix![];
    MatrixNorm::norm(&Lp::Float(0.5), &m);
}

//#[test]
//#[should_panic]
pub fn test_lp_vector_bad_int_p() {
    let v: Vector<f64> = Vector::new(vec![]);
    VectorNorm::norm(&Lp::Integer(0), &v);
}

//#[test]
//#[should_panic]
pub fn test_lp_matrix_bad_int_p() {
    let m: Matrix<f64> = matrix![];
    MatrixNorm::norm(&Lp::Integer(0), &m);
}
