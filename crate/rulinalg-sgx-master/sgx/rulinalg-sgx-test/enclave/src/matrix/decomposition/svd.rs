use std::prelude::v1::*;
use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::vector::Vector;
use rulinalg::matrix::decomposition::svd::sort_svd;

fn validate_svd(mat: &Matrix<f64>, b: &Matrix<f64>, u: &Matrix<f64>, v: &Matrix<f64>) {
    // b is diagonal (the singular values)
    for (idx, row) in b.row_iter().enumerate() {
        assert!(!row.iter().take(idx).any(|&x| x > 1e-10));
        assert!(!row.iter().skip(idx + 1).any(|&x| x > 1e-10));
        // Assert non-negativity of diagonal elements
        assert!(row[idx] >= 0.0);
    }

    let recovered = u * b * v.transpose();

    assert_eq!(recovered.rows(), mat.rows());
    assert_eq!(recovered.cols(), mat.cols());

    assert!(!mat.data()
        .iter()
        .zip(recovered.data().iter())
        .any(|(&x, &y)| (x - y).abs() > 1e-10));

    // The transposition is due to the fact that there does not exist
    // any column iterators at the moment, and we need to simultaneously iterate
    // over the columns. Once they do exist, we should rewrite
    // the below iterators to use iter_cols() or whatever instead.
    let ref u_transposed = u.transpose();
    let ref v_transposed = v.transpose();
    let ref mat_transposed = mat.transpose();

    let mut singular_triplets = u_transposed.row_iter().zip(b.diag()).zip(v_transposed.row_iter())
        // chained zipping results in nested tuple. Flatten it.
        .map(|((u_col, singular_value), v_col)| (Vector::new(u_col.raw_slice()), singular_value, Vector::new(v_col.raw_slice())));

    assert!(singular_triplets.by_ref()
        // For a matrix M, each singular value σ and left and right singular vectors u and v respectively
        // satisfy M v = σ u, so we take the difference
        .map(|(ref u, sigma, ref v)| mat * v - u * sigma)
        .flat_map(|v| v.into_vec().into_iter())
        .all(|x| x.abs() < 1e-10));

    assert!(singular_triplets.by_ref()
        // For a matrix M, each singular value σ and left and right singular vectors u and v respectively
        // satisfy M_transposed u = σ v, so we take the difference
        .map(|(ref u, sigma, ref v)| mat_transposed * u - v * sigma)
        .flat_map(|v| v.into_vec().into_iter())
        .all(|x| x.abs() < 1e-10));
}

//#[test]
pub fn test_sort_svd() {
    let u = matrix![1.0, 2.0, 3.0;
                    4.0, 5.0, 6.0];
    let b = matrix![4.0, 0.0, 0.0;
                    0.0, 8.0, 0.0;
                    0.0, 0.0, 2.0];
    let v = matrix![21.0, 22.0, 23.0;
                    24.0, 25.0, 26.0;
                    27.0, 28.0, 29.0];

    let (b, u, v) = sort_svd(b, u, v);

    assert_eq!(b.data(), &vec![8.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 2.0]);
    assert_eq!(u.data(), &vec![2.0, 1.0, 3.0, 5.0, 4.0, 6.0]);
    assert_eq!(v.data(),
               &vec![22.0, 21.0, 23.0, 25.0, 24.0, 26.0, 28.0, 27.0, 29.0]);

}

//#[test]
pub fn test_svd_tall_matrix() {
    // Note: This matrix is not arbitrary. It has been constructed specifically so that
    // the "natural" order of the singular values it not sorted by default.
    let mat = matrix![3.61833700244349288, -3.28382346228211697,  1.97968027781346501, -0.41869628192662156;
                      3.96046289599926427,  0.70730060716580723, -2.80552479438772817, -1.45283286109873933;
                      1.44435028724617442,  1.27749196276785826, -1.09858397535426366, -0.03159619816434689;
                      1.13455445826500667,  0.81521390274755756,  3.99123446373437263, -2.83025703359666192;
                      -3.30895752093770579, -0.04979044289857298,  3.03248594516832792,  3.85962479743330977];
    let (b, u, v) = mat.clone().svd().unwrap();

    let expected_values = vec![8.0, 6.0, 4.0, 2.0];

    validate_svd(&mat, &b, &u, &v);

    // Assert the singular values are what we expect
    assert!(expected_values.iter()
        .zip(b.diag())
        .all(|(expected, actual)| (expected - actual).abs() < 1e-14));
}

//#[test]
pub fn test_svd_short_matrix() {
    // Note: This matrix is not arbitrary. It has been constructed specifically so that
    // the "natural" order of the singular values it not sorted by default.
    let mat = matrix![3.61833700244349288,  3.96046289599926427,  1.44435028724617442,  1.13455445826500645, -3.30895752093770579;
                     -3.28382346228211697,  0.70730060716580723,  1.27749196276785826,  0.81521390274755756, -0.04979044289857298;
                      1.97968027781346545, -2.80552479438772817, -1.09858397535426366,  3.99123446373437263,  3.03248594516832792;
                     -0.41869628192662156, -1.45283286109873933, -0.03159619816434689, -2.83025703359666192,  3.85962479743330977];
    let (b, u, v) = mat.clone().svd().unwrap();

    let expected_values = vec![8.0, 6.0, 4.0, 2.0];

    validate_svd(&mat, &b, &u, &v);

    // Assert the singular values are what we expect
    assert!(expected_values.iter()
        .zip(b.diag())
        .all(|(expected, actual)| (expected - actual).abs() < 1e-14));
}

//#[test]
pub fn test_svd_square_matrix() {
    let mat = matrix![1.0,  2.0,  3.0,  4.0,  5.0;
                      2.0,  4.0,  1.0,  2.0,  1.0;
                      3.0,  1.0,  7.0,  1.0,  1.0;
                      4.0,  2.0,  1.0, -1.0,  3.0;
                      5.0,  1.0,  1.0,  3.0,  2.0];

    let expected_values = vec![12.1739747429271112,
                               5.2681047320525831,
                               4.4942269799769843,
                               2.9279675877385123,
                               2.8758200827412224];

    let (b, u, v) = mat.clone().svd().unwrap();
    validate_svd(&mat, &b, &u, &v);

    // Assert the singular values are what we expect
    assert!(expected_values.iter()
        .zip(b.diag())
        .all(|(expected, actual)| (expected - actual).abs() < 1e-12));
}
