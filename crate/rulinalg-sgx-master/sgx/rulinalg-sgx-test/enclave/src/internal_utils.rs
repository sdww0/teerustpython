use std::prelude::v1::*;
use rulinalg::vector::Vector;

use rulinalg::internal_utils::nullify_lower_triangular_part;
use rulinalg::internal_utils::nullify_upper_triangular_part;
use rulinalg::internal_utils::transpose_gemv;
use rulinalg::internal_utils::ger;

//#[test]
pub fn nullify_lower_triangular_part_examples() {
    let mut x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0;
                        7.0, 8.0, 9.0];
    nullify_lower_triangular_part(&mut x);
    assert_matrix_eq!(x, matrix![
        1.0, 2.0, 3.0;
        0.0, 5.0, 6.0;
        0.0, 0.0, 9.0
    ]);
}

//#[test]
pub fn nullify_upper_triangular_part_examples() {
    let mut x = matrix![1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0;
                        7.0, 8.0, 9.0];
    nullify_upper_triangular_part(&mut x);
    assert_matrix_eq!(x, matrix![
        1.0, 0.0, 0.0;
        4.0, 5.0, 0.0;
        7.0, 8.0, 9.0
    ]);
}

//#[test]
pub fn transpose_gemv_examples() {
    {
        let a = matrix![3.0, 4.0, 5.0;
                        2.0, 3.0, 1.0];
        let x = vec![2.0, 3.0];
        let mut y = vec![0.0; 3];
        transpose_gemv(&a, &x, &mut y);

        let y = Vector::new(y);
        assert_vector_eq!(y, vector![12.0, 17.0, 13.0]);
    }
}

//#[test]
pub fn ger_examples() {
    {
        let mut a = matrix![3.0, 4.0, 5.0;
                        2.0, 3.0, 1.0];
        let x = vec![3.0, 4.0];
        let y = vec![2.0, 1.0, 3.0];
        let alpha = 3.0;

        ger(&mut a, alpha, &x, &y);

        let expected = matrix![21.0, 13.0, 32.0;
                               26.0, 15.0, 37.0];
        assert_matrix_eq!(a, expected, comp = float);
    }
}
