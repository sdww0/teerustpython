use std::prelude::v1::*;
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut};
use rulinalg::matrix::PermutationMatrix;

//#[test]
pub fn permutation_vector_mul() {
    let p = PermutationMatrix::from_array(vec![1, 2, 0]).unwrap();
    let x = vector![1, 2, 3];
    let expected = vector![3, 1, 2];

    {
        let y = p.clone() * x.clone();
        assert_eq!(y, expected);
    }

    {
        let y = p.clone() * &x;
        assert_eq!(y, expected);
    }

    {
        let y = &p * x.clone();
        assert_eq!(y, expected);
    }

    {
        let y = &p * &x;
        assert_eq!(y, expected);
    }
}

//#[test]
pub fn permutation_matrix_left_mul_for_matrix() {
    let p = PermutationMatrix::from_array(vec![1, 2, 0]).unwrap();
    let x = matrix![1, 2, 3;
                    4, 5, 6;
                    7, 8, 9];
    let expected = matrix![7, 8, 9;
                           1, 2, 3;
                           4, 5, 6];

    {
        // Consume p, consume rhs
        let y = p.clone() * x.clone();
        assert_eq!(y, expected);
    }

    {
        // Consume p, borrow rhs
        let y = p.clone() * &x;
        assert_eq!(y, expected);
    }

    {
        // Borrow p, consume rhs
        let y = &p * x.clone();
        assert_eq!(y, expected);
    }

    {
        // Borrow p, borrow rhs
        let y = &p * &x;
        assert_eq!(y, expected);
    }
}

//#[test]
pub fn permutation_matrix_left_mul_for_matrix_slice() {
    let p = PermutationMatrix::from_array(vec![1, 2, 0]).unwrap();
    let x_source = matrix![1, 2, 3;
                           4, 5, 6;
                           7, 8, 9];
    let expected = matrix![7, 8, 9;
                           1, 2, 3;
                           4, 5, 6];

    {
        // Consume immutable, consume p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = p.clone() * x;
        assert_eq!(y, expected);
    }

    {
        // Borrow immutable, consume p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = p.clone() * &x;
        assert_eq!(y, expected);
    }

    {
        // Consume immutable, borrow p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = &p * x;
        assert_eq!(y, expected);
    }

    {
        // Borrow immutable, borrow p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = &p * &x;
        assert_eq!(y, expected);
    }

    {
        // Consume mutable, consume p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = p.clone() * x;
        assert_eq!(y, expected);
    }

    {
        // Borrow mutable, consume p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = p.clone() * &x;
        assert_eq!(y, expected);
    }

    {
        // Consume mutable, borrow p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = &p * x;
        assert_eq!(y, expected);
    }

    {
        // Borrow mutable, borrow p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = &p * &x;
        assert_eq!(y, expected);
    }
}

//#[test]
pub fn permutation_matrix_right_mul_for_matrix() {
    let p = PermutationMatrix::from_array(vec![1, 2, 0]).unwrap();
    let x = matrix![1, 2, 3;
                    4, 5, 6;
                    7, 8, 9];
    let expected = matrix![3, 1, 2;
                           6, 4, 5;
                           9, 7, 8];

    {
        // Consume lhs, consume p
        let y = x.clone() * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Consume lhs, borrow p
        let y = x.clone() * &p;
        assert_eq!(y, expected);
    }

    {
        // Borrow lhs, consume p
        let y = &x * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Borrow lhs, borrow p
        let y = &x * &p;
        assert_eq!(y, expected);
    }
}

//#[test]
pub fn permutation_matrix_right_mul_for_matrix_slice() {
    let p = PermutationMatrix::from_array(vec![1, 2, 0]).unwrap();
    let x_source = matrix![1, 2, 3;
                           4, 5, 6;
                           7, 8, 9];
    let expected = matrix![3, 1, 2;
                           6, 4, 5;
                           9, 7, 8];

    {
        // Consume immutable lhs, consume p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = x * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Borrow immutable lhs, consume p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = &x * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Consume immutable lhs, consume p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = x * &p;
        assert_eq!(y, expected);
    }

    {
        // Borrow immutable lhs, borrow p
        let x = x_source.sub_slice([0, 0], 3, 3);
        let y = &x * &p;
        assert_eq!(y, expected);
    }

    {
        // Consume mutable lhs, consume p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = x * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Borrow mutable lhs, consume p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = &x * p.clone();
        assert_eq!(y, expected);
    }

    {
        // Consume mutable lhs, borrow p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = x * &p;
        assert_eq!(y, expected);
    }

    {
        // Borrow mutable lhs, borrow p
        let mut x_source = x_source.clone();
        let x = x_source.sub_slice_mut([0, 0], 3, 3);
        let y = &x * &p;
        assert_eq!(y, expected);
    }
}

//#[test]
pub fn permutation_matrix_self_multiply() {
    let p1 = PermutationMatrix::<u32>::from_array(vec![2, 0, 1, 3]).unwrap();
    let p2 = PermutationMatrix::<u32>::from_array(vec![0, 3, 2, 1]).unwrap();

    let p1p2 = PermutationMatrix::from_array(vec![2, 3, 1, 0]).unwrap();
    let p2p1 = PermutationMatrix::from_array(vec![2, 0, 3, 1]).unwrap();

    {
        // Consume p1, consume p2
        assert_eq!(p1p2, p1.clone() * p2.clone());
        assert_eq!(p2p1, p2.clone() * p1.clone());
    }

    {
        // Consume p1, borrow p2
        assert_eq!(p1p2, p1.clone() * &p2);
        assert_eq!(p2p1, &p2 * p1.clone());
    }

    {
        // Borrow p1, consume p2
        assert_eq!(p1p2, &p1 * p2.clone());
        assert_eq!(p2p1, p2.clone() * &p1);
    }

    {
        // Borrow p1, borrow p2
        assert_eq!(p1p2, &p1 * &p2);
        assert_eq!(p2p1, &p2 * &p1);
    }
}
