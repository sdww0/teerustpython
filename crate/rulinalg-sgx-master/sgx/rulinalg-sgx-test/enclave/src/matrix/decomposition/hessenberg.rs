use rulinalg::matrix::Matrix;

//#[test]
//#[should_panic]
pub fn test_non_square_upper_hessenberg() {
    let a: Matrix<f64> = Matrix::ones(2, 3);

    let _ = a.upper_hessenberg();
}

//#[test]
//#[should_panic]
pub fn test_non_square_upper_hess_decomp() {
    let a: Matrix<f64> = Matrix::ones(2, 3);

    let _ = a.upper_hess_decomp();
}
