use std::prelude::v1::*;
use rulinalg::matrix::{DiagOffset, Matrix, MatrixSlice, MatrixSliceMut};
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut};

//#[test]
pub fn test_diag_offset_equivalence() {
    // This test will check that `Main`,
    // `Below(0)`, and `Above(0)` are all equivalent.
    let a = matrix![0.0, 1.0, 2.0;
                    3.0, 4.0, 5.0;
                    6.0, 7.0, 8.0];

    // Collect each diagonal and compare them
    let d1 = a.diag_iter(DiagOffset::Main).collect::<Vec<_>>();
    let d2 = a.diag_iter(DiagOffset::Above(0)).collect::<Vec<_>>();
    let d3 = a.diag_iter(DiagOffset::Below(0)).collect::<Vec<_>>();
    assert_eq!(d1, d2);
    assert_eq!(d2, d3);

    let b = MatrixSlice::from_matrix(&a, [0, 0], 2, 3);
    let d1 = b.diag_iter(DiagOffset::Main).collect::<Vec<_>>();
    let d2 = b.diag_iter(DiagOffset::Above(0)).collect::<Vec<_>>();
    let d3 = b.diag_iter(DiagOffset::Below(0)).collect::<Vec<_>>();
    assert_eq!(d1, d2);
    assert_eq!(d2, d3);
}

//#[test]
pub fn test_matrix_diag() {
    let mut a = matrix![0.0, 1.0, 2.0;
                        3.0, 4.0, 5.0;
                        6.0, 7.0, 8.0];

    let diags = vec![0.0, 4.0, 8.0];
    assert_eq!(a.diag_iter(DiagOffset::Main).cloned().collect::<Vec<_>>(), diags);
    let diags = vec![1.0, 5.0];
    assert_eq!(a.diag_iter(DiagOffset::Above(1)).cloned().collect::<Vec<_>>(), diags);
    let diags = vec![3.0, 7.0];
    assert_eq!(a.diag_iter(DiagOffset::Below(1)).cloned().collect::<Vec<_>>(), diags);
    let diags = vec![2.0];
    assert_eq!(a.diag_iter(DiagOffset::Above(2)).cloned().collect::<Vec<_>>(), diags);
    let diags = vec![6.0];
    assert_eq!(a.diag_iter(DiagOffset::Below(2)).cloned().collect::<Vec<_>>(), diags);

    {
        let diags_iter_mut = a.diag_iter_mut(DiagOffset::Main);
        for d in diags_iter_mut {
            *d = 1.0;
        }
    }

    for i in 0..3 {
        assert_eq!(a[[i,i]], 1.0);
    }
}

//#[test]
pub fn test_empty_matrix_diag() {
    let a: Matrix<f32> = matrix![];

    assert_eq!(None, a.diag_iter(DiagOffset::Main).next());
}

//#[test]
pub fn test_matrix_slice_diag() {
    let mut a = matrix![0.0, 1.0, 2.0, 3.0;
                        4.0, 5.0, 6.0, 7.0;
                        8.0, 9.0, 10.0, 11.0];
    {
        let b = MatrixSlice::from_matrix(&a, [0, 0], 2, 4);

        let diags = vec![0.0, 5.0];
        assert_eq!(b.diag_iter(DiagOffset::Main).cloned().collect::<Vec<_>>(), diags);
        let diags = vec![1.0, 6.0];
        assert_eq!(b.diag_iter(DiagOffset::Above(1)).cloned().collect::<Vec<_>>(), diags);
        let diags = vec![2.0, 7.0];
        assert_eq!(b.diag_iter(DiagOffset::Above(2)).cloned().collect::<Vec<_>>(), diags);
        let diags = vec![3.0];
        assert_eq!(b.diag_iter(DiagOffset::Above(3)).cloned().collect::<Vec<_>>(), diags);
        let diags = vec![4.0];
        assert_eq!(b.diag_iter(DiagOffset::Below(1)).cloned().collect::<Vec<_>>(), diags);
    }

    {
        let diags_iter_mut = a.diag_iter_mut(DiagOffset::Main);
        for d in diags_iter_mut {
            *d = 1.0;
        }
    }

    for i in 0..3 {
        assert_eq!(a[[i,i]], 1.0);
    }
}

//#[test]
pub fn test_matrix_diag_nth() {
    let a = matrix![0.0, 1.0, 2.0, 3.0;
                    4.0, 5.0, 6.0, 7.0;
                    8.0, 9.0, 10.0, 11.0];

    let mut diags_iter = a.diag_iter(DiagOffset::Main);
    assert_eq!(0.0, *diags_iter.nth(0).unwrap());
    assert_eq!(10.0, *diags_iter.nth(1).unwrap());
    assert_eq!(None, diags_iter.next());

    let mut diags_iter = a.diag_iter(DiagOffset::Above(1));
    assert_eq!(6.0, *diags_iter.nth(1).unwrap());
    assert_eq!(11.0, *diags_iter.next().unwrap());
    assert_eq!(None, diags_iter.next());

    let mut diags_iter = a.diag_iter(DiagOffset::Below(1));
    assert_eq!(9.0, *diags_iter.nth(1).unwrap());
    assert_eq!(None, diags_iter.next());
}

//#[test]
pub fn test_matrix_slice_diag_nth() {
    let a = matrix![0.0, 1.0, 2.0, 3.0;
                    4.0, 5.0, 6.0, 7.0;
                    8.0, 9.0, 10.0, 11.0];
    let b = MatrixSlice::from_matrix(&a, [0, 0], 2, 4);

    let mut diags_iter = b.diag_iter(DiagOffset::Main);
    assert_eq!(5.0, *diags_iter.nth(1).unwrap());;
    assert_eq!(None, diags_iter.next());

    let mut diags_iter = b.diag_iter(DiagOffset::Above(1));
    assert_eq!(6.0, *diags_iter.nth(1).unwrap());
    assert_eq!(None, diags_iter.next());

    let mut diags_iter = b.diag_iter(DiagOffset::Below(1));
    assert_eq!(4.0, *diags_iter.nth(0).unwrap());
    assert_eq!(None, diags_iter.next());
}

//#[test]
pub fn test_matrix_diag_last() {
    let a = matrix![0.0, 1.0, 2.0;
                    3.0, 4.0, 5.0;
                    6.0, 7.0, 8.0];

    let diags_iter = a.diag_iter(DiagOffset::Main);
    assert_eq!(8.0, *diags_iter.last().unwrap());

    let diags_iter = a.diag_iter(DiagOffset::Above(2));
    assert_eq!(2.0, *diags_iter.last().unwrap());

    let diags_iter = a.diag_iter(DiagOffset::Below(2));
    assert_eq!(6.0, *diags_iter.last().unwrap());
}

//#[test]
pub fn test_matrix_slice_diag_last() {
    let a = matrix![0.0, 1.0, 2.0;
                    3.0, 4.0, 5.0;
                    6.0, 7.0, 8.0];
    let b = MatrixSlice::from_matrix(&a, [0, 0], 3, 2);

    {
        let diags_iter = b.diag_iter(DiagOffset::Main);
        assert_eq!(4.0, *diags_iter.last().unwrap());
    }

    {
        let diags_iter = b.diag_iter(DiagOffset::Above(1));
        assert_eq!(1.0, *diags_iter.last().unwrap());
    }

    {
        let diags_iter = b.diag_iter(DiagOffset::Below(2));
        assert_eq!(6.0, *diags_iter.last().unwrap());
    }
}

//#[test]
pub fn test_matrix_diag_count() {
    let a = matrix![0.0, 1.0, 2.0;
                    3.0, 4.0, 5.0;
                    6.0, 7.0, 8.0];

    assert_eq!(3, a.diag_iter(DiagOffset::Main).count());
    assert_eq!(2, a.diag_iter(DiagOffset::Above(1)).count());
    assert_eq!(1, a.diag_iter(DiagOffset::Above(2)).count());
    assert_eq!(2, a.diag_iter(DiagOffset::Below(1)).count());
    assert_eq!(1, a.diag_iter(DiagOffset::Below(2)).count());

    let mut diags_iter = a.diag_iter(DiagOffset::Main);
    diags_iter.next();
    assert_eq!(2, diags_iter.count());
}

//#[test]
pub fn test_matrix_diag_size_hint() {
    let a = matrix![0.0, 1.0, 2.0;
                    3.0, 4.0, 5.0;
                    6.0, 7.0, 8.0];

    let mut diags_iter = a.diag_iter(DiagOffset::Main);
    assert_eq!((3, Some(3)), diags_iter.size_hint());
    diags_iter.next();

    assert_eq!((2, Some(2)), diags_iter.size_hint());
    diags_iter.next();
    diags_iter.next();

    assert_eq!((0, Some(0)), diags_iter.size_hint());
    assert_eq!(None, diags_iter.next());
    assert_eq!((0, Some(0)), diags_iter.size_hint());
}

//#[test]
pub fn test_matrix_cols() {
    let mut a = matrix![0, 1, 2, 3;
                        4, 5, 6, 7;
                        8, 9, 10, 11];
    let data = [[0, 4, 8], [1, 5, 9], [2, 6, 10], [3, 7, 11]];

    for (i, col) in a.col_iter().enumerate() {
        for (j, value) in col.iter().enumerate() {
            assert_eq!(data[i][j], *value);
        }
    }

    for (i, mut col) in a.col_iter_mut().enumerate() {
        for (j, value) in col.iter_mut().enumerate() {
            assert_eq!(data[i][j], *value);
        }
    }

    for mut col in a.col_iter_mut() {
        for r in col.iter_mut() {
            *r = 0;
        }
    }

    assert_eq!(a.into_vec(), vec![0; 12]);
}

//#[test]
pub fn test_matrix_slice_cols() {
    let a = matrix![0, 1, 2, 3;
                    4, 5, 6, 7;
                    8, 9, 10, 11];

    let b = MatrixSlice::from_matrix(&a, [0, 0], 3, 2);

    let data = [[0, 4, 8], [1, 5, 9]];

    for (i, col) in b.col_iter().enumerate() {
        for (j, value) in col.iter().enumerate() {
            assert_eq!(data[i][j], *value);
        }
    }
}

//#[test]
pub fn test_matrix_slice_mut_cols() {
    let mut a = matrix![0, 1, 2, 3;
                        4, 5, 6, 7;
                        8, 9, 10, 11];

    {
        let mut b = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 2);

        let data = [[0, 4, 8], [1, 5, 9]];

        for (i, col) in b.col_iter().enumerate() {
            for (j, value) in col.iter().enumerate() {
                assert_eq!(data[i][j], *value);
            }
        }

        for (i, mut col) in b.col_iter_mut().enumerate() {
            for (j, value) in col.iter_mut().enumerate() {
                assert_eq!(data[i][j], *value);
            }
        }

        for mut col in b.col_iter_mut() {
            for r in col.iter_mut() {
                *r = 0;
            }
        }
    }

    assert_eq!(a.into_vec(), vec![0, 0, 2, 3, 0, 0, 6, 7, 0, 0, 10, 11]);
}

//#[test]
pub fn test_matrix_cols_nth() {
    let a = matrix![0, 1, 2, 3;
                    4, 5, 6, 7;
                    8, 9, 10, 11];

    let mut col_iter = a.col_iter();

    let mut nth0 = col_iter.nth(0).unwrap().into_iter();

    assert_eq!(0, *nth0.next().unwrap());
    assert_eq!(4, *nth0.next().unwrap());
    assert_eq!(8, *nth0.next().unwrap());

    let mut nth1 = col_iter.nth(2).unwrap().into_iter();

    assert_eq!(3, *nth1.next().unwrap());
    assert_eq!(7, *nth1.next().unwrap());
    assert_eq!(11, *nth1.next().unwrap());

    assert!(col_iter.next().is_none());
}

//#[test]
pub fn test_matrix_cols_last() {
    let a = matrix![0, 1, 2, 3;
                    4, 5, 6, 7;
                    8, 9, 10, 11];

    let mut col_iter = a.col_iter().last().unwrap().into_iter();

    assert_eq!(3, *col_iter.next().unwrap());
    assert_eq!(7, *col_iter.next().unwrap());
    assert_eq!(11, *col_iter.next().unwrap());

    let mut col_iter = a.col_iter();

    col_iter.next();

    let mut last_col_iter = col_iter.last().unwrap().into_iter();

    assert_eq!(3, *last_col_iter.next().unwrap());
    assert_eq!(7, *last_col_iter.next().unwrap());
    assert_eq!(11, *last_col_iter.next().unwrap());

    let mut col_iter = a.col_iter();

    col_iter.next();
    col_iter.next();
    col_iter.next();
    col_iter.next();

    assert!(col_iter.last().is_none());
}

//#[test]
pub fn test_matrix_cols_count() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let col_iter = a.col_iter();

    assert_eq!(3, col_iter.count());

    let mut col_iter_2 = a.col_iter();
    col_iter_2.next();
    assert_eq!(2, col_iter_2.count());
}

//#[test]
pub fn test_matrix_cols_size_hint() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let mut col_iter = a.col_iter();

    assert_eq!((3, Some(3)), col_iter.size_hint());

    col_iter.next();

    assert_eq!((2, Some(2)), col_iter.size_hint());
    col_iter.next();
    col_iter.next();

    assert_eq!((0, Some(0)), col_iter.size_hint());

    assert!(col_iter.next().is_none());
    assert_eq!((0, Some(0)), col_iter.size_hint());
}

//#[test]
pub fn test_matrix_rows() {
    let mut a = matrix![0, 1, 2;
                        3, 4, 5;
                        6, 7, 8];

    let data = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    for (i, row) in a.row_iter().enumerate() {
        assert_eq!(data[i], *row.raw_slice());
    }

    for (i, row) in a.row_iter_mut().enumerate() {
        assert_eq!(data[i], *row.raw_slice());
    }

    for mut row in a.row_iter_mut() {
        for r in row.raw_slice_mut() {
            *r = 0;
        }
    }

    assert_eq!(a.into_vec(), vec![0; 9]);
}

//#[test]
pub fn test_matrix_slice_rows() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let b = MatrixSlice::from_matrix(&a, [0, 0], 2, 2);

    let data = [[0, 1], [3, 4]];

    for (i, row) in b.row_iter().enumerate() {
        assert_eq!(data[i], *row.raw_slice());
    }
}

//#[test]
pub fn test_matrix_slice_mut_rows() {
    let mut a = matrix![0, 1, 2;
                        3, 4, 5;
                        6, 7, 8];

    {
        let mut b = MatrixSliceMut::from_matrix(&mut a, [0, 0], 2, 2);

        let data = [[0, 1], [3, 4]];

        for (i, row) in b.row_iter().enumerate() {
            assert_eq!(data[i], *row.raw_slice());
        }

        for (i, row) in b.row_iter_mut().enumerate() {
            assert_eq!(data[i], *row.raw_slice());
        }

        for mut row in b.row_iter_mut() {
            for r in row.raw_slice_mut() {
                *r = 0;
            }
        }
    }

    assert_eq!(a.into_vec(), vec![0, 0, 2, 0, 0, 5, 6, 7, 8]);
}

//#[test]
pub fn test_matrix_rows_nth() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let mut row_iter = a.row_iter();

    assert_eq!([0, 1, 2], *row_iter.nth(0).unwrap().raw_slice());
    assert_eq!([6, 7, 8], *row_iter.nth(1).unwrap().raw_slice());

    assert!(row_iter.next().is_none());
}

//#[test]
pub fn test_matrix_rows_last() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let row_iter = a.row_iter();

    assert_eq!([6, 7, 8], *row_iter.last().unwrap().raw_slice());

    let mut row_iter = a.row_iter();

    row_iter.next();
    assert_eq!([6, 7, 8], *row_iter.last().unwrap().raw_slice());

    let mut row_iter = a.row_iter();

    row_iter.next();
    row_iter.next();
    row_iter.next();
    row_iter.next();

    assert!(row_iter.last().is_none());
}

//#[test]
pub fn test_matrix_rows_count() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let row_iter = a.row_iter();

    assert_eq!(3, row_iter.count());

    let mut row_iter_2 = a.row_iter();
    row_iter_2.next();
    assert_eq!(2, row_iter_2.count());
}

//#[test]
pub fn test_matrix_rows_size_hint() {
    let a = matrix![0, 1, 2;
                    3, 4, 5;
                    6, 7, 8];

    let mut row_iter = a.row_iter();

    assert_eq!((3, Some(3)), row_iter.size_hint());

    row_iter.next();

    assert_eq!((2, Some(2)), row_iter.size_hint());
    row_iter.next();
    row_iter.next();

    assert_eq!((0, Some(0)), row_iter.size_hint());

    assert!(row_iter.next().is_none());
    assert_eq!((0, Some(0)), row_iter.size_hint());
}

//#[test]
pub fn into_iter_compile() {
    let a = Matrix::ones(3, 3) * 2.;
    let mut b = MatrixSlice::from_matrix(&a, [1, 1], 2, 2);

    for _ in b {
    }

    for _ in &b {
    }

    for _ in &mut b {
    }
}

//#[test]
pub fn into_iter_mut_compile() {
    let mut a = Matrix::<f32>::ones(3, 3) * 2.;

    {
        let b = MatrixSliceMut::from_matrix(&mut a, [1, 1], 2, 2);

        for v in b {
            *v = 1.0;
        }
    }

    {
        let b = MatrixSliceMut::from_matrix(&mut a, [1, 1], 2, 2);

        for _ in &b {
        }
    }

    {
        let mut b = MatrixSliceMut::from_matrix(&mut a, [1, 1], 2, 2);

        for v in &mut b {
            *v = 1.0;
        }
    }
}

//#[test]
pub fn iter_matrix_small_matrices() {
    {
        let x = matrix![ 1 ];
        let mut i = x.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
    }

    {
        let x = matrix![ 1, 2 ];
        let mut i = x.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), None);
    }

    {
        let x = matrix![ 1; 2 ];
        let mut i = x.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), None);
    }

    {
        let x = matrix![ 1, 2;
                         3, 4 ];
        let mut i = x.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&4));
        assert_eq!(i.next(), None);
    }
}

//#[test]
pub fn iter_matrix_slice() {
    let x = matrix![1, 2, 3;
                    4, 5, 6;
                    7, 8, 9];

    // Helper to simplify writing the below tests.
    // Note that .collect() is an implicit test of .next(),
    // including checking that None is returned when there
    // are no more elements.
    let collect_slice = |(i, j), rows, cols| {
        x.sub_slice([i, j], rows, cols)
         .iter()
         .cloned()
         .collect::<Vec<_>>()
    };

    {
        // Zero elements
        for i in 0 .. 2 {
            for j in 0 .. 2 {
                let y = x.sub_slice([i, j], 0, 0);
                assert!(y.iter().next().is_none());
            }
        }

    }

    {
        // One element
        for i in 0 .. 2 {
            for j in 0 .. 2 {
                let y = x.sub_slice([i, j], 1, 1);
                assert_eq!(y.iter().next(), Some(&x[[i, j]]));
            }
        }
    }

    {
        // 1x2 sub slices
        assert_eq!(collect_slice((0, 0), 1, 2), vec![1, 2]);
        assert_eq!(collect_slice((0, 1), 1, 2), vec![2, 3]);
        assert_eq!(collect_slice((1, 0), 1, 2), vec![4, 5]);
        assert_eq!(collect_slice((1, 1), 1, 2), vec![5, 6]);
        assert_eq!(collect_slice((2, 0), 1, 2), vec![7, 8]);
        assert_eq!(collect_slice((2, 1), 1, 2), vec![8, 9]);
    }

    {
        // 2x1 sub slices
        assert_eq!(collect_slice((0, 0), 2, 1), vec![1, 4]);
        assert_eq!(collect_slice((1, 0), 2, 1), vec![4, 7]);
        assert_eq!(collect_slice((0, 1), 2, 1), vec![2, 5]);
        assert_eq!(collect_slice((1, 1), 2, 1), vec![5, 8]);
        assert_eq!(collect_slice((0, 2), 2, 1), vec![3, 6]);
        assert_eq!(collect_slice((1, 2), 2, 1), vec![6, 9]);
    }

    {
        // 2x2 sub slices
        assert_eq!(collect_slice((0, 0), 2, 2), vec![1, 2, 4, 5]);
        assert_eq!(collect_slice((0, 1), 2, 2), vec![2, 3, 5, 6]);
        assert_eq!(collect_slice((1, 0), 2, 2), vec![4, 5, 7, 8]);
        assert_eq!(collect_slice((1, 1), 2, 2), vec![5, 6, 8, 9]);
    }
}

//#[test]
pub fn iter_empty_matrix() {
    {
        let x = Matrix::<u32>::zeros(0, 0);
        assert!(x.iter().next().is_none());
    }

    {
        let x = Matrix::<u32>::zeros(1, 0);
        assert!(x.iter().next().is_none());
    }

    {
        let x = Matrix::<u32>::zeros(0, 1);
        assert!(x.iter().next().is_none());
    }
}
