use std::prelude::v1::*;
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut};

//#[test]
pub fn column_clone_into_slice() {
    let mat = matrix![1, 2;
                      3, 4];
    let mut v = vec![0, 0];
    mat.col(0).clone_into_slice(&mut v);
    assert_eq!(v, vec![1, 3]);
}

//#[test]
pub fn column_mut_clone_into_slice() {
    let mut mat = matrix![1, 2;
                      3, 4];
    let mut v = vec![0, 0];
    mat.col_mut(0).clone_into_slice(&mut v);
    assert_eq!(v, vec![1, 3]);
}

//#[test]
pub fn column_mut_clone_from_slice() {
    let mut mat = matrix![1, 2;
                          3, 4];
    let v = vec![5, 6];
    {
        let mut col = mat.col_mut(0);
        col.clone_from_slice(&v);
    }
    assert_matrix_eq!(mat, matrix![5, 2;
                                   6, 4]);
}

pub mod mat_mul;
pub mod impl_ops;
pub mod slice;
pub mod iter;
pub mod decomposition;
pub mod impl_permutation_mul;
pub mod base;
pub mod permutation_matrix;
pub mod impl_mat;
