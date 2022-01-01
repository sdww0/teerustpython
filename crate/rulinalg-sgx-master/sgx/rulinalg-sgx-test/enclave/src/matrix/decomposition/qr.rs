use std::prelude::v1::*;
use rulinalg::matrix::decomposition::qr::HouseholderQr;
use rulinalg::matrix::decomposition::qr::{QR, ThinQR};

use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::matrix::decomposition::Decomposition;

use crate::testsupport::is_upper_triangular;

fn verify_qr(x: Matrix<f64>, qr: QR<f64>) {
    let m = x.rows();
    let QR { ref q, ref r } = qr;

    assert_matrix_eq!(q * r, x, comp = float, ulp = 100);
    assert!(is_upper_triangular(r));

    // check orthogonality
    assert_matrix_eq!(q.transpose() * q, Matrix::identity(m),
        comp = float, ulp = 100);
    assert_matrix_eq!(q * q.transpose(), Matrix::identity(m),
        comp = float, ulp = 100);
}

fn verify_thin_qr(x: Matrix<f64>, qr: ThinQR<f64>) {
    use std::cmp::min;

    let m = x.rows();
    let n = x.cols();
    let ThinQR { ref q1, ref r1 } = qr;

    assert_matrix_eq!(q1 * r1, x, comp = float, ulp = 100);
    assert!(is_upper_triangular(r1));

    // Check that q1 has orthogonal columns
    assert_matrix_eq!(q1.transpose() * q1, Matrix::identity(min(m, n)),
        comp = float, ulp = 100);
}

//#[test]
pub fn householder_qr_unpack_reconstruction() {
    {
        // 1x1
        let x = matrix![1.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 1x2
        let x = matrix![1.0, 2.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 2x1
        let x = matrix![1.0;
                        2.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 2x2
        let x = matrix![1.0, 2.0;
                        3.0, 4.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 3x2
        let x = matrix![1.0, 2.0;
                        3.0, 4.0;
                        4.0, 5.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 2x3
        let x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        // 3x3
        let x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0;
                        7.0, 8.0, 9.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }
}

//#[test]
pub fn householder_qr_unpack_square_reconstruction_corner_cases() {
    {
        let x = matrix![-1.0, 0.0;
                         0.0, 1.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        let x = matrix![1.0,  0.0,  0.0;
                        0.0,  1.0,  0.0;
                        0.0,  0.0, -1.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }

    {
        let x = matrix![1.0,   0.0,  0.0;
                        0.0,  -1.0,  0.0;
                        0.0,   0.0, -1.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack();
        verify_qr(x, qr);
    }
}

//#[test]
pub fn householder_qr_unpack_thin_reconstruction() {
    {
        // 1x1
        let x = matrix![1.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 1x2
        let x = matrix![1.0, 2.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 2x1
        let x = matrix![1.0;
                        2.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 2x2
        let x = matrix![1.0, 2.0;
                        3.0, 4.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 3x2
        let x = matrix![1.0, 2.0;
                        3.0, 4.0;
                        4.0, 5.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 2x3
        let x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }

    {
        // 3x3
        let x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0;
                        7.0, 8.0, 9.0];
        let qr = HouseholderQr::decompose(x.clone()).unpack_thin();
        verify_thin_qr(x, qr);
    }
}
