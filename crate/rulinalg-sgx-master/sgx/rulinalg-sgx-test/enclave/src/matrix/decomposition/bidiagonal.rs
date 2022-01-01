use std::prelude::v1::*;
use rulinalg::matrix::{BaseMatrix, Matrix};

fn validate_bidiag(mat: &Matrix<f64>,
                   b: &Matrix<f64>,
                   u: &Matrix<f64>,
                   v: &Matrix<f64>,
                   upper: bool) {
    for (idx, row) in b.row_iter().enumerate() {
        let pair_start = if upper { idx } else { idx.saturating_sub(1) };
        assert!(!row.iter().take(pair_start).any(|&x| x > 1e-10));
        assert!(!row.iter().skip(pair_start + 2).any(|&x| x > 1e-10));
    }

    let recovered = u * b * v.transpose();

    assert_eq!(recovered.rows(), mat.rows());
    assert_eq!(recovered.cols(), mat.cols());

    assert!(!mat.data()
        .iter()
        .zip(recovered.data().iter())
        .any(|(&x, &y)| (x - y).abs() > 1e-10));
}

//#[test]
pub fn test_bidiagonal_square() {
    let mat = matrix![1f64, 2.0, 3.0, 4.0, 5.0;
                      2.0, 4.0, 1.0, 2.0, 1.0;
                      3.0, 1.0, 7.0, 1.0, 1.0;
                      4.0, 2.0, 1.0, -1.0, 3.0;
                      5.0, 1.0, 1.0, 3.0, 2.0];
    let (b, u, v) = mat.clone().bidiagonal_decomp().unwrap();
    validate_bidiag(&mat, &b, &u, &v, true);
}

//#[test]
pub fn test_bidiagonal_non_square() {
    let mat = matrix![1f64, 2.0, 3.0;
                      4.0, 5.0, 2.0;
                      4.0, 1.0, 2.0;
                      1.0, 3.0, 1.0;
                      7.0, 1.0, 1.0];
    let (b, u, v) = mat.clone().bidiagonal_decomp().unwrap();
    validate_bidiag(&mat, &b, &u, &v, true);

    let mat = matrix![1f64, 2.0, 3.0, 4.0, 5.0;
                      2.0, 4.0, 1.0, 2.0, 1.0;
                      3.0, 1.0, 7.0, 1.0, 1.0];
    let (b, u, v) = mat.clone().bidiagonal_decomp().unwrap();
    validate_bidiag(&mat, &b, &u, &v, false);
}
