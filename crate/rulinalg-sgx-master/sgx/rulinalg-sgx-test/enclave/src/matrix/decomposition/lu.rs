use std::prelude::v1::*;
use rulinalg::matrix::{Matrix, PermutationMatrix};
use crate::testsupport::{is_lower_triangular, is_upper_triangular};

use rulinalg::matrix::decomposition::{PartialPivLu, LUP, FullPivLu, LUPQ};
use rulinalg::matrix::decomposition::Decomposition;

#[allow(deprecated)]
//#[test]
//#[should_panic]
pub fn test_non_square_lup_decomp() {
    let a: Matrix<f64> = Matrix::ones(2, 3);

    let _ = a.lup_decomp();
}

#[allow(deprecated)]
//#[test]
pub fn test_lup_decomp() {
    use rulinalg::error::ErrorKind;
    let a: Matrix<f64> = matrix![
        1., 2., 3., 4.;
        0., 0., 0., 0.;
        0., 0., 0., 0.;
        0., 0., 0., 0.
    ];

    match a.lup_decomp() {
        Err(e) => assert!(*e.kind() == ErrorKind::DivByZero),
        Ok(_) => panic!()
    }
}

//#[test]
pub fn partial_piv_lu_decompose_arbitrary() {
    // Since the LUP decomposition is not in general unique,
    // we can not test against factors directly, but
    // instead we must rely on the fact that the
    // matrices P, L and U together construct the
    // original matrix
    let x = matrix![ -3.0,   0.0,   4.0,   1.0;
                    -12.0,   5.0,  17.0,   1.0;
                     15.0,   0.0, -18.0,  -5.0;
                      6.0,  20.0, -10.0, -15.0 ];

    let LUP { l, u, p } = PartialPivLu::decompose(x.clone())
                                       .unwrap()
                                       .unpack();
    let y = p.inverse() * &l * &u;
    assert_matrix_eq!(x, y, comp = float);
    assert!(is_lower_triangular(&l));
    assert!(is_upper_triangular(&u));
}

//#[test]
pub fn partial_piv_lu_inverse_identity() {
    let lu = PartialPivLu::<f64> {
        lu: Matrix::identity(3),
        p: PermutationMatrix::identity(3)
    };

    let inv = lu.inverse().expect("Matrix is invertible.");

    assert_matrix_eq!(inv, Matrix::identity(3), comp = float);
}

//#[test]
pub fn partial_piv_lu_inverse_arbitrary_invertible_matrix() {
    let x = matrix![5.0, 0.0, 0.0, 1.0;
                    2.0, 2.0, 2.0, 1.0;
                    4.0, 5.0, 5.0, 5.0;
                    1.0, 6.0, 4.0, 5.0];

    let inv = matrix![1.85185185185185203e-01,   1.85185185185185175e-01, -7.40740740740740561e-02, -1.02798428206033007e-17;
                      1.66666666666666630e-01,   6.66666666666666519e-01, -6.66666666666666519e-01,  4.99999999999999833e-01;
                     -3.88888888888888840e-01,   1.11111111111111174e-01,  5.55555555555555358e-01, -4.99999999999999833e-01;
                      7.40740740740740838e-02,  -9.25925925925925819e-01,  3.70370370370370294e-01,  5.13992141030165006e-17];

    let lu = PartialPivLu::decompose(x).unwrap();

    assert_matrix_eq!(lu.inverse().unwrap(), inv, comp = float);
}

//#[test]
pub fn partial_piv_lu_det_identity() {
    let lu = PartialPivLu::<f64> {
        lu: Matrix::identity(3),
        p: PermutationMatrix::identity(3)
    };

    assert_eq!(lu.det(), 1.0);
}

//#[test]
pub fn partial_piv_lu_det_arbitrary_invertible_matrix() {
    let x = matrix![ 5.0,  0.0,  0.0,  1.0;
                     0.0,  2.0,  2.0,  1.0;
                    15.0,  4.0,  7.0, 10.0;
                     5.0,  2.0, 17.0, 32.0];

    let lu = PartialPivLu::decompose(x).unwrap();

    let expected_det = 149.99999999999997;
    assert_scalar_eq!(lu.det(), expected_det, comp = float);
}

//#[test]
pub fn partial_piv_lu_solve_arbitrary_matrix() {
    let x = matrix![ 5.0, 0.0, 0.0, 1.0;
                     2.0, 2.0, 2.0, 1.0;
                     4.0, 5.0, 5.0, 5.0;
                     1.0, 6.0, 4.0, 5.0 ];
    let b = vector![9.0, 16.0, 49.0, 45.0];
    let expected = vector![1.0, 2.0, 3.0, 4.0];

    let lu = PartialPivLu::decompose(x).unwrap();
    let y = lu.solve(b).unwrap();
    // Need to up the tolerance to take into account
    // numerical error. Ideally there'd be a more systematic
    // way to test this.
    assert_vector_eq!(y, expected, comp = ulp, tol = 100);
}

//#[test]
pub fn lu_forward_substitution() {
    use rulinalg::matrix::decomposition::lu::lu_forward_substitution;

    {
        let lu: Matrix<f64> = matrix![];
        let b = vector![];
        let x = lu_forward_substitution(&lu, b);
        assert!(x.size() == 0);
    }

    {
        let lu = matrix![3.0];
        let b = vector![1.0];
        let x = lu_forward_substitution(&lu, b);
        assert_eq!(x, vector![1.0]);
    }

    {
        let lu = matrix![3.0, 2.0;
                         2.0, 2.0];
        let b = vector![1.0, 2.0];
        let x = lu_forward_substitution(&lu, b);
        assert_eq!(x, vector![1.0, 0.0]);
    }
}

//#[test]
pub fn full_piv_lu_decompose_arbitrary() {
    // Since the LUP decomposition is not in general unique,
    // we can not test against factors directly, but
    // instead we must rely on the fact that the
    // matrices P, L and U together construct the
    // original matrix
    let x = matrix![ -3.0,   0.0,   4.0,   1.0;
                    -12.0,   5.0,  17.0,   1.0;
                     15.0,   0.0, -18.0,  -5.0;
                      6.0,  20.0, -10.0, -15.0 ];

    let LUPQ { l, u, p, q } = FullPivLu::decompose(x.clone())
                                       .unwrap()
                                       .unpack();

    let y = p.inverse() * &l * &u * q.inverse();

    assert_matrix_eq!(x, y, comp = float);
    assert!(is_lower_triangular(&l));
    assert!(is_upper_triangular(&u));
}

//#[test]
pub fn full_piv_lu_decompose_singular() {
    let x = matrix![ -3.0,   0.0,   4.0,   1.0;
                    -12.0,   5.0,  17.0,   1.0;
                     15.0,   0.0, -18.0,  -5.0;
                     -6.0,   0.0,   8.0,   2.0 ];

    let lu = FullPivLu::decompose(x.clone()).unwrap();

    assert_eq!(lu.rank(), 3);

    let LUPQ { l, u, p, q } = lu.unpack();

    let y = p.inverse() * &l * &u * q.inverse();

    assert_matrix_eq!(x, y, comp = float);
    assert!(is_lower_triangular(&l));
    assert!(is_upper_triangular(&u));
}

//#[test]
//#[should_panic]
pub fn full_piv_lu_decompose_rectangular() {
    let x = matrix![ -3.0,   0.0,   4.0;
                    -12.0,   5.0,  17.0;
                     15.0,   0.0, -18.0;
                     -6.0,   0.0,   20.0];
                     
    FullPivLu::decompose(x.clone()).unwrap();
}

//#[test]
pub fn full_piv_lu_solve_arbitrary_matrix() {
    let x = matrix![ 5.0, 0.0, 0.0, 1.0;
                     2.0, 2.0, 2.0, 1.0;
                     4.0, 5.0, 5.0, 5.0;
                     1.0, 6.0, 4.0, 5.0 ];
    let b = vector![9.0, 16.0, 49.0, 45.0];
    let expected = vector![1.0, 2.0, 3.0, 4.0];

    let lu = FullPivLu::decompose(x).unwrap();
    let y = lu.solve(b).unwrap();

    // Need to up the tolerance to take into account
    // numerical error. Ideally there'd be a more systematic
    // way to test this.
    assert_vector_eq!(y, expected, comp = ulp, tol = 100);
}

//#[test]
pub fn full_piv_lu_inverse_arbitrary_invertible_matrix() {
    let x = matrix![5.0, 0.0, 0.0, 1.0;
                    2.0, 2.0, 2.0, 1.0;
                    4.0, 5.0, 5.0, 5.0;
                    1.0, 6.0, 4.0, 5.0];

    let inv = matrix![1.85185185185185203e-01,   1.85185185185185175e-01, -7.40740740740740561e-02, -1.02798428206033007e-17;
                      1.66666666666666630e-01,   6.66666666666666519e-01, -6.66666666666666519e-01,  4.99999999999999833e-01;
                     -3.88888888888888840e-01,   1.11111111111111174e-01,  5.55555555555555358e-01, -4.99999999999999833e-01;
                      7.40740740740740838e-02,  -9.25925925925925819e-01,  3.70370370370370294e-01,  5.13992141030165006e-17];

    let lu = FullPivLu::decompose(x).unwrap();

    assert_matrix_eq!(lu.inverse().unwrap(), inv, comp = float);
}

//#[test]
pub fn full_piv_lu_inverse_noninvertible() {
    let x = matrix![5.0, 0.0, 1.0;
                    4.0, 5.0, 5.0;
                    9.0, 5.0, 6.0];

    let lu = FullPivLu::decompose(x).unwrap();

    assert!(lu.inverse().is_err());
}

//#[test]
pub fn full_piv_lu_empty_matrix() {
    use matrix::BaseMatrix;

    let x = Matrix::from_fn(0, 0, |_, _| 0.0);
    assert_eq!(x.rows(), 0);
    assert_eq!(x.cols(), 0);

    let lu = FullPivLu::decompose(x).unwrap();

    assert!(lu.is_invertible());
    assert_eq!(lu.rank(), 0);
    assert_eq!(lu.det(), 1.0);

    let inverse = lu.inverse().unwrap();
    assert_eq!(inverse.rows(), 0);
    assert_eq!(inverse.cols(), 0);

    let LUPQ { l, u, p, q } = lu.unpack();
    assert_eq!(l.rows(), 0);
    assert_eq!(l.cols(), 0);

    assert_eq!(u.rows(), 0);
    assert_eq!(u.cols(), 0);

    assert_eq!(p.size(), 0);
    assert_eq!(q.size(), 0);
}
