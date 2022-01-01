use std::prelude::v1::*;
use rulinalg::matrix::Matrix;
use rulinalg::matrix::decomposition::Decomposition;
use rulinalg::vector::Vector;

use rulinalg::matrix::decomposition::Cholesky;
use rulinalg::matrix::decomposition::cholesky::transpose_back_substitution;

use quickcheck::TestResult;

//#[test]
//#[should_panic]
#[allow(deprecated)]
pub fn test_non_square_cholesky() {
    let a = Matrix::<f64>::ones(2, 3);

    let _ = a.cholesky();
}

//#[test]
pub fn cholesky_unpack_empty() {
    let x: Matrix<f64> = matrix![];
    let l = Cholesky::decompose(x.clone())
                        .unwrap()
                        .unpack();
    assert_matrix_eq!(l, x);
}

//#[test]
pub fn cholesky_unpack_1x1() {
    let x = matrix![ 4.0 ];
    let expected = matrix![ 2.0 ];
    let l = Cholesky::decompose(x)
                        .unwrap()
                        .unpack();
    assert_matrix_eq!(l, expected, comp = float);
}

//#[test]
pub fn cholesky_unpack_2x2() {
    {
        let x = matrix![ 9.0, -6.0;
                        -6.0, 20.0];
        let expected = matrix![ 3.0, 0.0;
                               -2.0, 4.0];

        let l = Cholesky::decompose(x)
                    .unwrap()
                    .unpack();
        assert_matrix_eq!(l, expected, comp = float);
    }
}

//#[test]
pub fn cholesky_singular_fails() {
    {
        let x = matrix![0.0];
        assert!(Cholesky::decompose(x).is_err());
    }

    {
        let x = matrix![0.0, 0.0;
                        0.0, 1.0];
        assert!(Cholesky::decompose(x).is_err());
    }

    {
        let x = matrix![1.0, 0.0;
                        0.0, 0.0];
        assert!(Cholesky::decompose(x).is_err());
    }

    {
        let x = matrix![1.0,   3.0,   5.0;
                        3.0,   9.0,  15.0;
                        5.0,  15.0,  65.0];
        assert!(Cholesky::decompose(x).is_err());
    }
}

//#[test]
pub fn cholesky_det_empty() {
    let x: Matrix<f64> = matrix![];
    let cholesky = Cholesky::decompose(x).unwrap();
    assert_eq!(cholesky.det(), 1.0);
}

//#[test]
pub fn cholesky_det() {
    {
        let x = matrix![1.0];
        let cholesky = Cholesky::decompose(x).unwrap();
        assert_scalar_eq!(cholesky.det(), 1.0, comp = float);
    }

    {
        let x = matrix![1.0,   3.0,   5.0;
                        3.0,  18.0,  33.0;
                        5.0,  33.0,  65.0];
        let cholesky = Cholesky::decompose(x).unwrap();
        assert_scalar_eq!(cholesky.det(), 36.0, comp = float);
    }
}

//#[test]
pub fn cholesky_solve_examples() {
    {
        let a: Matrix<f64> = matrix![];
        let b: Vector<f64> = vector![];
        let expected: Vector<f64> = vector![];
        let cholesky = Cholesky::decompose(a).unwrap();
        let x = cholesky.solve(b).unwrap();
        assert_eq!(x, expected);
    }

    {
        let a = matrix![ 1.0 ];
        let b = vector![ 4.0 ];
        let expected = vector![ 4.0 ];
        let cholesky = Cholesky::decompose(a).unwrap();
        let x = cholesky.solve(b).unwrap();
        assert_vector_eq!(x, expected, comp = float);
    }

    {
        let a = matrix![ 4.0,  6.0;
                         6.0, 25.0];
        let b = vector![ 2.0,  4.0];
        let expected = vector![ 0.40625,  0.0625 ];
        let cholesky = Cholesky::decompose(a).unwrap();
        let x = cholesky.solve(b).unwrap();
        assert_vector_eq!(x, expected, comp = float);
    }
}

//#[test]
pub fn cholesky_inverse_examples() {
    {
        let a: Matrix<f64> = matrix![];
        let expected: Matrix<f64> = matrix![];
        let cholesky = Cholesky::decompose(a).unwrap();
        assert_eq!(cholesky.inverse().unwrap(), expected);
    }

    {
        let a = matrix![ 2.0 ];
        let expected = matrix![ 0.5 ];
        let cholesky = Cholesky::decompose(a).unwrap();
        assert_matrix_eq!(cholesky.inverse().unwrap(), expected,
                          comp = float);
    }

    {
        let a = matrix![ 4.0,  6.0;
                         6.0, 25.0];
        let expected = matrix![  0.390625, -0.09375;
                                -0.093750 , 0.06250];
        let cholesky = Cholesky::decompose(a).unwrap();
        assert_matrix_eq!(cholesky.inverse().unwrap(), expected,
                          comp = float);
    }

    {
        let a = matrix![ 9.0,   6.0,   3.0;
                         6.0,  20.0,  10.0;
                         3.0,  10.0,  14.0];
        let expected = matrix![0.1388888888888889, -0.0416666666666667,  0.0               ;
                              -0.0416666666666667,  0.0902777777777778, -0.0555555555555556;
                                              0.0, -0.0555555555555556,  0.1111111111111111];
        let cholesky = Cholesky::decompose(a).unwrap();
        assert_matrix_eq!(cholesky.inverse().unwrap(), expected,
                          comp = float);
    }
}

quickcheck! {
    fn property_cholesky_of_identity_is_identity(n: usize) -> TestResult {
        if n > 30 {
            return TestResult::discard();
        }

        let x = Matrix::<f64>::identity(n);
        let l = Cholesky::decompose(x.clone()).map(|c| c.unpack());
        match l {
            Ok(l) => {
                assert_matrix_eq!(l, x, comp = float);
                TestResult::passed()
            },
            _ => TestResult::failed()
        }
    }
}

//#[test]
pub fn transpose_back_substitution_examples() {
    {
        let l: Matrix<f64> = matrix![];
        let b: Vector<f64> = vector![];
        let expected: Vector<f64> = vector![];
        let x = transpose_back_substitution(&l, b).unwrap();
        assert_vector_eq!(x, expected);
    }

    {
        let l = matrix![2.0];
        let b = vector![2.0];
        let expected = vector![1.0];
        let x = transpose_back_substitution(&l, b).unwrap();
        assert_vector_eq!(x, expected, comp = float);
    }

    {
        let l = matrix![2.0, 0.0;
                        3.0, 4.0];
        let b = vector![2.0, 1.0];
        let expected = vector![0.625, 0.25 ];
        let x = transpose_back_substitution(&l, b).unwrap();
        assert_vector_eq!(x, expected, comp = float);
    }

    {
        let l = matrix![ 2.0,  0.0,  0.0;
                         5.0, -1.0,  0.0;
                        -2.0,  0.0,  1.0];
        let b = vector![-1.0, 2.0, 3.0];
        let expected = vector![ 7.5, -2.0, 3.0 ];
        let x = transpose_back_substitution(&l, b).unwrap();
        assert_vector_eq!(x, expected, comp = float);
    }
}
