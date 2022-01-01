use std::prelude::v1::*;
use rulinalg::matrix::Matrix;

//#[test]
pub fn test_1_by_1_matrix_eigenvalues() {
    let a = Matrix::ones(1, 1) * 3.;
    assert_eq!(vec![3.], a.eigenvalues().unwrap());
}

//#[test]
pub fn test_2_by_2_matrix_eigenvalues() {
    let a = matrix![1., 2.; 3., 4.];
    // characteristic polynomial is λ² − 5λ − 2 = 0
    assert_eq!(vec![(5. - (33.0f32).sqrt()) / 2., (5. + (33.0f32).sqrt()) / 2.],
               a.eigenvalues().unwrap());
}

//#[test]
pub fn test_2_by_2_matrix_zeros_eigenvalues() {
    let a = Matrix::zeros(2, 2);
    // characteristic polynomial is λ² = 0
    assert_eq!(vec![0.0, 0.0], a.eigenvalues().unwrap());
}

//#[test]
pub fn test_2_by_2_matrix_complex_eigenvalues() {
    // This test currently fails - complex eigenvalues would be nice though!
    let a = matrix![1., -3.; 1., 1.];
    // characteristic polynomial is λ² − λ + 4 = 0

    // Decomposition will fail
    assert!(a.eigenvalues().is_err());
}

//#[test]
pub fn test_2_by_2_matrix_eigendecomp() {
    let a = matrix![20., 4.; 20., 16.];
    let (eigenvals, eigenvecs) = a.clone().eigendecomp().unwrap();

    let lambda_1 = eigenvals[0];
    let lambda_2 = eigenvals[1];

    let v1 = vector![eigenvecs[[0, 0]], eigenvecs[[1, 0]]];
    let v2 = vector![eigenvecs[[0, 1]], eigenvecs[[1, 1]]];

    assert_vector_eq!(&a * &v1, &v1 * lambda_1, comp = float);
    assert_vector_eq!(&a * &v2, &v2 * lambda_2, comp = float);
}

//#[test]
pub fn test_3_by_3_eigenvals() {
    let a = matrix![17f64, 22., 27.;
                    22., 29., 36.;
                    27., 36., 45.];

    let eigs = a.eigenvalues().unwrap();

    let eig_1 = 90.4026;
    let eig_2 = 0.5973;
    let eig_3 = 0.0;

    assert!(eigs.iter().any(|x| (x - eig_1).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_2).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_3).abs() < 1e-4));
}

//#[test]
pub fn test_5_by_5_eigenvals() {
    let a = matrix![1f64, 2.0, 3.0, 4.0, 5.0;
                    2.0, 4.0, 1.0, 2.0, 1.0;
                    3.0, 1.0, 7.0, 1.0, 1.0;
                    4.0, 2.0, 1.0, -1.0, 3.0;
                    5.0, 1.0, 1.0, 3.0, 2.0];

    let eigs = a.eigenvalues().unwrap();

    let eig_1 = 12.174;
    let eig_2 = 5.2681;
    let eig_3 = -4.4942;
    let eig_4 = 2.9279;
    let eig_5 = -2.8758;

    assert!(eigs.iter().any(|x| (x - eig_1).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_2).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_3).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_4).abs() < 1e-4));
    assert!(eigs.iter().any(|x| (x - eig_5).abs() < 1e-4));
}

//#[test]
//#[should_panic]
pub fn test_non_square_eigenvalues() {
    let a: Matrix<f64> = Matrix::ones(2, 3);

    let _ = a.eigenvalues();
}

//#[test]
//#[should_panic]
pub fn test_non_square_eigendecomp() {
    let a: Matrix<f64> = Matrix::ones(2, 3);

    let _ = a.eigendecomp();
}
