use std::prelude::v1::*;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use rulinalg::matrix::permutation_matrix::{PermutationMatrix, Parity};
use rulinalg::matrix::permutation_matrix::{permute_by_swap};
//use rulinalg::matrix::permutation_matrix::{permute_by_swap, validate_permutation};

//use itertools::Itertools;

//#[test]
pub fn swap_rows() {
    let mut p = PermutationMatrix::<u64>::identity(4);
    p.swap_rows(0, 3);
    p.swap_rows(1, 3);

    let expected_permutation = PermutationMatrix::from_array(vec![3, 0, 2, 1]).unwrap();
    assert_eq!(p, expected_permutation);
}

//#[test]
pub fn as_matrix() {
    let p = PermutationMatrix::from_array(vec![2, 1, 0, 3]).unwrap();
    let expected_matrix: Matrix<u32> = matrix![0, 0, 1, 0;
                                               0, 1, 0, 0;
                                               1, 0, 0, 0;
                                               0, 0, 0, 1];

    assert_matrix_eq!(expected_matrix, p.as_matrix());
}

//#[test]
pub fn from_array() {
    let array = vec![1, 0, 3, 2];
    let p = PermutationMatrix::<u32>::from_array(array.clone()).unwrap();
    let p_as_array: Vec<usize> = p.into();
    assert_eq!(p_as_array, array);
}

//#[test]
pub fn from_array_unchecked() {
    let array = vec![1, 0, 3, 2];
    let p = unsafe { PermutationMatrix::<u32>::from_array_unchecked(array.clone()) };
    let p_as_array: Vec<usize> = p.into();
    assert_eq!(p_as_array, array);
}

//#[test]
pub fn from_array_invalid() {
    assert!(PermutationMatrix::<u32>::from_array(vec![0, 1, 3]).is_err());
    assert!(PermutationMatrix::<u32>::from_array(vec![0, 0]).is_err());
    assert!(PermutationMatrix::<u32>::from_array(vec![3, 0, 1]).is_err());
}

//#[test]
pub fn vec_from_permutation() {
    let source_vec = vec![0, 2, 1];
    let p = PermutationMatrix::<u32>::from_array(source_vec.clone()).unwrap();
    let vec = Vec::from(p);
    assert_eq!(&source_vec, &vec);
}

//#[test]
pub fn into_slice_ref() {
    let source_vec = vec![0, 2, 1];
    let ref p = PermutationMatrix::<u32>::from_array(source_vec.clone()).unwrap();
    let p_as_slice_ref: &[usize] = p.into();
    assert_eq!(source_vec.as_slice(), p_as_slice_ref);
}

//#[test]
pub fn map_row() {
    let p = PermutationMatrix::<u32>::from_array(vec![0, 2, 1]).unwrap();
    assert_eq!(p.map_row(0), 0);
    assert_eq!(p.map_row(1), 2);
    assert_eq!(p.map_row(2), 1);
}

//#[test]
pub fn inverse() {
    let p = PermutationMatrix::<u32>::from_array(vec![1, 2, 0]).unwrap();
    let expected_inverse = PermutationMatrix::<u32>::from_array(vec![2, 0, 1]).unwrap();
    assert_eq!(p.inverse(), expected_inverse);
}

//#[test]
pub fn parity() {
    {
        let p = PermutationMatrix::<u32>::from_array(vec![1, 0, 3, 2]).unwrap();
        assert_eq!(p.parity(), Parity::Even);
    }

    {
        let p = PermutationMatrix::<u32>::from_array(vec![4, 2, 3, 1, 0, 5]).unwrap();
        assert_eq!(p.parity(), Parity::Odd);
    }
}

//#[test]
pub fn det() {
    {
        let p = PermutationMatrix::<i32>::from_array(vec![1, 0, 3, 2]).unwrap();
        assert_eq!(p.det(), 1);
    }

    {
        let p = PermutationMatrix::<i32>::from_array(vec![4, 2, 3, 1, 0, 5]).unwrap();
        assert_eq!(p.det(), -1);
    }
}

//#[test]
pub fn permute_by_swap_on_empty_array() {
    let mut x = Vec::<char>::new();
    let mut permutation_array = Vec::new();
    permute_by_swap(&mut permutation_array, |i, j| x.swap(i, j));
}

//#[test]
pub fn permute_by_swap_on_arbitrary_array() {
    let mut x = vec!['a', 'b', 'c', 'd'];
    let mut permutation_array = vec![0, 2, 3, 1];

    permute_by_swap(&mut permutation_array, |i, j| x.swap(i, j));
    assert_eq!(x, vec!['a', 'd', 'b', 'c']);
}

//#[test]
pub fn permute_by_swap_identity_on_arbitrary_array() {
    let mut x = vec!['a', 'b', 'c', 'd'];
    let mut permutation_array = vec![0, 1, 2, 3];
    permute_by_swap(&mut permutation_array, |i, j| x.swap(i, j));
    assert_eq!(x, vec!['a', 'b', 'c', 'd']);
}

//#[test]
pub fn compose_into_buffer() {
    let p = PermutationMatrix::<u32>::from_array(vec![2, 1, 0]).unwrap();
    let q = PermutationMatrix::<u32>::from_array(vec![1, 0, 2]).unwrap();
    let pq_expected = PermutationMatrix::<u32>::from_array(vec![1, 2, 0]).unwrap();
    let qp_expected = PermutationMatrix::<u32>::from_array(vec![2, 0, 1]).unwrap();

    {
        let mut pq = PermutationMatrix::identity(3);
        p.compose_into_buffer(&q, &mut pq);
        assert_eq!(pq, pq_expected);
    }

    {
        let mut qp = PermutationMatrix::identity(3);
        q.compose_into_buffer(&p, &mut qp);
        assert_eq!(qp, qp_expected);
    }
}

//#[test]
pub fn compose_regression() {
    // At some point during development, this example failed to
    // give the expected results
    let p = PermutationMatrix::<u32>::from_array(vec![1, 2, 0]).unwrap();
    let q = PermutationMatrix::<u32>::from_array(vec![2, 0, 1]).unwrap();
    let pq_expected = PermutationMatrix::<u32>::from_array(vec![0, 1, 2]).unwrap();

    let mut pq = PermutationMatrix::identity(3);
    p.compose_into_buffer(&q, &mut pq);
    assert_eq!(pq, pq_expected);
}

//#[test]
pub fn permute_rows_into_buffer() {
    let x = matrix![ 0;
                     1;
                     2;
                     3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    let mut output = Matrix::zeros(4, 1);
    p.permute_rows_into_buffer(&x, &mut output);
    assert_matrix_eq!(output, matrix![ 3; 1; 0; 2]);
}

//#[test]
pub fn permute_rows_in_place() {
    let mut x = matrix![ 0;
                     1;
                     2;
                     3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    p.permute_rows_in_place(&mut x);
    assert_matrix_eq!(x, matrix![ 3; 1; 0; 2]);
}

//#[test]
pub fn permute_cols_into_buffer() {
    let x = matrix![ 0, 1, 2, 3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    let mut output = Matrix::zeros(1, 4);
    p.permute_cols_into_buffer(&x, &mut output);
    assert_matrix_eq!(output, matrix![ 3, 1, 0, 2]);
}

//#[test]
pub fn permute_cols_in_place() {
    let mut x = matrix![ 0, 1, 2, 3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    p.permute_cols_in_place(&mut x);
    assert_matrix_eq!(x, matrix![ 3, 1, 0, 2]);
}

//#[test]
pub fn permute_vector_into_buffer() {
    let x = vector![ 0, 1, 2, 3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    let mut output = Vector::zeros(4);
    p.permute_vector_into_buffer(&x, &mut output);
    assert_vector_eq!(output, vector![ 3, 1, 0, 2]);
}

//#[test]
pub fn permute_vector_in_place() {
    let mut x = vector![ 0, 1, 2, 3];
    let p = PermutationMatrix::from_array(vec![2, 1, 3, 0]).unwrap();
    p.permute_vector_in_place(&mut x);
    assert_vector_eq!(x, vector![ 3, 1, 0, 2]);
}

//use quickcheck::{Arbitrary, Gen};
//
//// In order to write property tests for the validation of a permutation,
//// we need to be able to generate arbitrary (valid) permutations.
//#[derive(Debug, Clone, PartialEq, Eq)]
//struct ValidPermutationArray(pub Vec<usize>);
//
//impl Arbitrary for ValidPermutationArray {
//    fn arbitrary<G: Gen>(g: &mut G) -> Self {
//        let upper_bound = g.size();
//        let mut array = (0 .. upper_bound).collect::<Vec<usize>>();
//        g.shuffle(&mut array);
//        ValidPermutationArray(array)
//    }
//}
//
//// We also want to be able to generate invalid permutations for
//// the same reasons
//#[derive(Debug, Clone, PartialEq, Eq)]
//struct InvalidPermutationArray(pub Vec<usize>);
//
//impl Arbitrary for InvalidPermutationArray {
//    fn arbitrary<G: Gen>(g: &mut G) -> Self {
//        // Take an arbitrary valid permutation and mutate it so that
//        // it is invalid
//        let mut permutation_array = ValidPermutationArray::arbitrary(g).0;
//        let n = permutation_array.len();
//
//        // There are two essential sources of invalidity:
//        // 1. Duplicate elements
//        // 2. Indices out of bounds
//        // We want to have either or both
//
//        let should_have_duplicates = g.gen::<bool>();
//        let should_have_out_of_bounds = !should_have_duplicates || g.gen::<bool>();
//        assert!(should_have_duplicates || should_have_out_of_bounds);
//
//        if should_have_out_of_bounds {
//            let num_out_of_bounds_rounds = g.gen_range::<usize>(1, n);
//            for _ in 0 .. num_out_of_bounds_rounds {
//                let interior_index = g.gen_range::<usize>(0, n);
//                let exterior_index = n + g.gen::<usize>();
//                permutation_array[interior_index] = exterior_index;
//            }
//        }
//
//        if should_have_duplicates {
//            let num_duplicates = g.gen_range::<usize>(1, n);
//            for _ in 0 .. num_duplicates {
//                let interior_index = g.gen_range::<usize>(0, n);
//                let duplicate_value = permutation_array[interior_index];
//                permutation_array.push(duplicate_value);
//            }
//        }
//
//        // The duplicates are placed at the end, so we perform
//        // an additional shuffle to end up with a more or less
//        // arbitrary invalid permutation
//        g.shuffle(&mut permutation_array);
//        InvalidPermutationArray(permutation_array)
//    }
//}
//
//#[derive(Debug, PartialEq, Eq, Clone)]
//struct TestPermutationMatrix<T> {
//    // A permutation matrix of dimensions NxN is represented as a permutation of the rows
//    // of an NxM matrix for any M.
//    perm: Vec<usize>,
//
//    // Currently, we need to let PermutationMatrix be generic over T,
//    // because BaseMatrixMut is.
//    marker: std::marker::PhantomData<T>
//}
//
//impl<T: Send + Clone + 'static> Arbitrary for TestPermutationMatrix<T> {
//    fn arbitrary<G: Gen>(g: &mut G) -> Self {
//        let ValidPermutationArray(array) = ValidPermutationArray::arbitrary(g);
//        TestPermutationMatrix::from_array(array)
//            .expect("The generated permutation array should always be valid.")
//    }
//}
//
//quickcheck! {
//    fn property_validate_permutation_is_ok_for_valid_input(array: ValidPermutationArray) -> bool {
//        validate_permutation(&array.0).is_ok()
//    }
//}
//
//quickcheck! {
//    fn property_validate_permutation_is_err_for_invalid_input(array: InvalidPermutationArray) -> bool {
//        validate_permutation(&array.0).is_err()
//    }
//}
//
//quickcheck! {
//    fn property_identity_has_identity_array(size: usize) -> bool {
//        let p = TestPermutationMatrix::<u64>::identity(size);
//        let p_as_array: Vec<usize> = p.into();
//        let expected = (0 .. size).collect::<Vec<usize>>();
//        p_as_array == expected
//    }
//}
//
//quickcheck! {
//    fn property_dim_is_equal_to_array_dimensions(array: ValidPermutationArray) -> bool {
//        let ValidPermutationArray(array) = array;
//        let n = array.len();
//        let p = TestPermutationMatrix::<u32>::from_array(array).unwrap();
//        p.size() == n
//    }
//}
//
//quickcheck! {
//    fn property_inverse_of_inverse_is_original(p: TestPermutationMatrix<u32>) -> bool {
//        p == p.inverse().inverse()
//    }
//}
//
//quickcheck! {
//    fn property_inverse_composes_to_identity(p: TestPermutationMatrix<u32>) -> bool {
//        // Recall that P * P_inv = I and P_inv * P = I
//        let n = p.size();
//        let pinv = p.inverse();
//        let mut p_pinv_composition = TestPermutationMatrix::identity(n);
//        let mut pinv_p_composition = TestPermutationMatrix::identity(n);
//        p.compose_into_buffer(&pinv, &mut p_pinv_composition);
//        pinv.compose_into_buffer(&p, &mut pinv_p_composition);
//        assert_eq!(p_pinv_composition, TestPermutationMatrix::identity(n));
//        assert_eq!(pinv_p_composition, TestPermutationMatrix::identity(n));
//        true
//    }
//}
//
//quickcheck! {
//    fn property_identity_parity_is_even(n: usize) -> bool {
//        let p = TestPermutationMatrix::<u32>::identity(n);
//        p.parity() ==  Parity::Even
//    }
//}
//
//quickcheck! {
//    fn property_parity_agrees_with_parity_of_inversions(p: TestPermutationMatrix<u32>) -> bool {
//        let array: &[usize] = (&p).into();
//        let num_inversions = array.iter().cloned().enumerate()
//                                  .cartesian_product(array.iter().cloned().enumerate())
//                                  .filter(|&((i, permuted_i), (j, permuted_j))|
//                                    // This is simply the definition of an inversion
//                                    i < j && permuted_i > permuted_j
//                                  )
//                                  .count();
//        // Recall that the parity of the number of inversions in the
//        // permutation is equal to the parity of the permutation
//        let parity = if num_inversions % 2 == 0 {
//            Parity::Even
//        } else {
//            Parity::Odd
//        };
//
//        parity == p.clone().parity()
//    }
//}
