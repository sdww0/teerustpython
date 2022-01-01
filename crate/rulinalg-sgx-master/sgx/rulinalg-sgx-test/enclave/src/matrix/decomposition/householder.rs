use std::prelude::v1::*;
use rulinalg::vector::Vector;
use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::matrix::decomposition::HouseholderReflection;
use rulinalg::matrix::decomposition::householder::create_composition;

pub fn house_as_matrix(house: HouseholderReflection<f64>)
    -> Matrix<f64>
{
    let m = house.v.size();
    let v = Matrix::new(m, 1, house.v.into_vec());
    let v_t = v.transpose();
    Matrix::identity(m) - v * v_t * house.tau
}

fn verify_house(x: Vector<f64>, house: HouseholderReflection<f64>) {
    let m = x.size();
    assert!(m > 0);

    let house = house_as_matrix(house);
    let y = house.clone() * x.clone();

    // Check that y[1 ..] is approximately zero
    let z = Vector::new(y.data().iter().skip(1).cloned().collect::<Vec<_>>());
    assert_vector_eq!(z, Vector::zeros(m - 1), comp = float, eps = 1e-12);

    // Check that applying the Householder transformation again
    // recovers the original vector (since H = H^T = inv(H))
    let w = house * y;
    assert_vector_eq!(x, w, comp = float);
}

//#[test]
pub fn compute_empty_vector() {
    let x: Vector<f64> = vector![];
    let house = HouseholderReflection::compute(x.clone());
    assert_scalar_eq!(house.tau, 0.0);
    assert_vector_eq!(house.v, x.clone());
}

//#[test]
pub fn compute_single_element_vector() {
    let x = vector![2.0];
    let house = HouseholderReflection::compute(x.clone());
    assert_scalar_eq!(house.tau, 0.0);
}

//#[test]
pub fn compute_examples() {
    {
        let x = vector![1.0, 0.0, 0.0];
        let house = HouseholderReflection::compute(x.clone());
        verify_house(x, house);
    }

    {
        let x = vector![-1.0, 0.0, 0.0];
        let house = HouseholderReflection::compute(x.clone());
        verify_house(x, house);
    }

    {
        let x = vector![3.0, -2.0, 5.0];
        let house = HouseholderReflection::compute(x.clone());
        verify_house(x, house);
    }
}

//#[test]
pub fn householder_reflection_left_multiply() {
    let mut x = matrix![ 0.0,  1.0,  2.0,  3.0;
                         4.0,  5.0,  6.0,  7.0;
                         8.0,  9.0, 10.0, 11.0;
                        12.0, 13.0, 14.0, 15.0 ];

    // The provided data is rather rubbish, but
    // the result should still hold
    let h = HouseholderReflection {
        tau: 0.06666666666666667,
        v: vector![1.0, 2.0, 3.0, 4.0]
    };

    let mut buffer = vec![0.0; 4];

    h.buffered_left_multiply_into(&mut x, &mut buffer);

    let expected = matrix![ -5.3333,  -5.0000, -4.6667,  -4.3333;
                            -6.6667,  -7.0000, -7.3333,  -7.6667;
                            -8.0000,  -9.0000,-10.0000, -11.0000;
                            -9.3333, -11.0000,-12.6667, -14.3333];
    assert_matrix_eq!(x, expected, comp = abs, tol = 1e-3);
}

//#[test]
pub fn householder_composition_left_multiply() {
    let storage = matrix![ 5.0,  3.0,  2.0;
                           2.0,  1.0,  3.0;
                          -2.0,  3.0, -2.0];
    let tau = vec![2.0/9.0, 1.0 / 5.0, 2.0];

    // `q` is a manually computed matrix representation
    // of the Householder composition stored implicitly in
    // `storage` and `tau. We leave it here to make writing
    // further tests easier
    // let q = matrix![7.0/9.0, -28.0/45.0,   4.0/45.0;
    //                -4.0/9.0, - 4.0/ 9.0,   7.0/ 9.0;
    //                 4.0/9.0,  29.0/45.0,  28.0/45.0];
    let composition = create_composition(&storage, &tau);

    {
        // Square
        let mut x = matrix![4.0,  5.0, -3.0;
                            2.0, -1.0, -3.0;
                            1.0,  3.0,  5.0];
        composition.left_multiply_into(&mut x);

        let expected = matrix![ 88.0/45.0, 43.0/9.0, -1.0/45.0;
                               -17.0/ 9.0,  5.0/9.0, 59.0/ 9.0;
                               166.0/45.0, 31.0/9.0, -7.0/45.0];
        assert_matrix_eq!(x, expected, comp = float, eps = 1e-15);
    }

    {
        // Tall
        let mut x = matrix![ 4.0, 5.0;
                             3.0, 2.0;
                            -1.0,-2.0];
        composition.left_multiply_into(&mut x);
        let expected = matrix![52.0/45.0,  37.0/15.0;
                              -35.0/ 9.0, -14.0/ 3.0;
                              139.0/45.0,  34.0/15.0];
        assert_matrix_eq!(x, expected, comp = float, eps = 1e-15);
    }

    {
        // Short
        let mut x = matrix![ 4.0,  5.0,  2.0, -5.0;
                             3.0,  2.0,  1.0,  1.0;
                            -1.0, -2.0,  0.0, -5.0];
        composition.left_multiply_into(&mut x);
        let expected = matrix![52.0/45.0,  37.0/15.0, 14.0/15.0, -223.0/45.0;
                              -35.0/ 9.0, -14.0/ 3.0, -4.0/ 3.0,  -19.0/ 9.0;
                              139.0/45.0,  34.0/15.0, 23.0/15.0, -211.0/45.0];
        assert_matrix_eq!(x, expected, comp = float, eps = 1e-15);
    }
}

//#[test]
pub fn householder_composition_first_k_columns() {
    let storage = matrix![ 5.0,  3.0,  2.0;
                           2.0,  1.0,  3.0;
                          -2.0,  3.0, -2.0];
    let tau = vec![2.0/9.0, 1.0 / 5.0, 2.0];
    let composition = create_composition(&storage, &tau);

    // This corresponds to the following `Q` matrix
    let q = matrix![7.0/9.0, -28.0/45.0,   4.0/45.0;
                   -4.0/9.0, - 4.0/ 9.0,   7.0/ 9.0;
                    4.0/9.0,  29.0/45.0,  28.0/45.0];
    {
        // First 0 columns
        let q_k = composition.first_k_columns(0);
        assert_eq!(q_k.rows(), 3);
        assert_eq!(q_k.cols(), 0);
    }

    {
        // First column
        let q_k = composition.first_k_columns(1);
        assert_matrix_eq!(q_k, q.sub_slice([0, 0], 3, 1),
                          comp = float);
    }

    {
        // First 2 columns
        let q_k = composition.first_k_columns(2);
        assert_matrix_eq!(q_k, q.sub_slice([0, 0], 3, 2),
                          comp = float);
    }

    {
        // First 3 columns
        let q_k = composition.first_k_columns(3);
        assert_matrix_eq!(q_k, q.sub_slice([0, 0], 3, 3),
                          comp = float);
    }
}
