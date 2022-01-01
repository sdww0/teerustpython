use std::prelude::v1::*;
use rulinalg::matrix::Matrix;
use rulinalg::matrix::MatrixSlice;
use rulinalg::matrix::MatrixSliceMut;

//#[test]
pub fn indexing_mat() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];

    assert_eq!(a[[0, 0]], 1.0);
    assert_eq!(a[[0, 1]], 2.0);
    assert_eq!(a[[1, 0]], 3.0);
    assert_eq!(a[[1, 1]], 4.0);
    assert_eq!(a[[2, 0]], 5.0);
    assert_eq!(a[[2, 1]], 6.0);
}

//#[test]
pub fn matrix_vec_mul() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = vector![4., 7.];

    let c = a * b;

    assert_eq!(c.size(), 3);

    assert_eq!(c[0], 18.0);
    assert_eq!(c[1], 40.0);
    assert_eq!(c[2], 62.0);
}

//#[test]
pub fn matrix_f32_mul() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];

    // Allocating new memory
    let c = &a * &2.0;

    assert_eq!(c[[0, 0]], 2.0);
    assert_eq!(c[[0, 1]], 4.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 8.0);
    assert_eq!(c[[2, 0]], 10.0);
    assert_eq!(c[[2, 1]], 12.0);

    // Allocating new memory
    let c = &a * 2.0;

    assert_eq!(c[[0, 0]], 2.0);
    assert_eq!(c[[0, 1]], 4.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 8.0);
    assert_eq!(c[[2, 0]], 10.0);
    assert_eq!(c[[2, 1]], 12.0);

    // Reusing memory
    let c = a.clone() * &2.0;

    assert_eq!(c[[0, 0]], 2.0);
    assert_eq!(c[[0, 1]], 4.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 8.0);
    assert_eq!(c[[2, 0]], 10.0);
    assert_eq!(c[[2, 1]], 12.0);

    // Reusing memory
    let c = a * 2.0;

    assert_eq!(c[[0, 0]], 2.0);
    assert_eq!(c[[0, 1]], 4.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 8.0);
    assert_eq!(c[[2, 0]], 10.0);
    assert_eq!(c[[2, 1]], 12.0);
}

//#[test]
pub fn matrix_add() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = matrix![2., 3.;
                    4., 5.;
                    6., 7.];

    // Allocating new memory
    let c = &a + &b;

    assert_eq!(c[[0, 0]], 3.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 7.0);
    assert_eq!(c[[1, 1]], 9.0);
    assert_eq!(c[[2, 0]], 11.0);
    assert_eq!(c[[2, 1]], 13.0);

    // Reusing memory
    let c = a.clone() + &b;

    assert_eq!(c[[0, 0]], 3.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 7.0);
    assert_eq!(c[[1, 1]], 9.0);
    assert_eq!(c[[2, 0]], 11.0);
    assert_eq!(c[[2, 1]], 13.0);

    // Reusing memory
    let c = &a + b.clone();

    assert_eq!(c[[0, 0]], 3.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 7.0);
    assert_eq!(c[[1, 1]], 9.0);
    assert_eq!(c[[2, 0]], 11.0);
    assert_eq!(c[[2, 1]], 13.0);

    // Reusing memory
    let c = a + b;

    assert_eq!(c[[0, 0]], 3.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 7.0);
    assert_eq!(c[[1, 1]], 9.0);
    assert_eq!(c[[2, 0]], 11.0);
    assert_eq!(c[[2, 1]], 13.0);
}

//#[test]
pub fn matrix_f32_add() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = 3.0;

    // Allocating new memory
    let c = &a + &b;

    assert_eq!(c[[0, 0]], 4.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 7.0);
    assert_eq!(c[[2, 0]], 8.0);
    assert_eq!(c[[2, 1]], 9.0);

    // Allocating new memory
    let c = &a + b;

    assert_eq!(c[[0, 0]], 4.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 7.0);
    assert_eq!(c[[2, 0]], 8.0);
    assert_eq!(c[[2, 1]], 9.0);

    // Reusing memory
    let c = a.clone() + &b;

    assert_eq!(c[[0, 0]], 4.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 7.0);
    assert_eq!(c[[2, 0]], 8.0);
    assert_eq!(c[[2, 1]], 9.0);

    // Reusing memory
    let c = a + b;

    assert_eq!(c[[0, 0]], 4.0);
    assert_eq!(c[[0, 1]], 5.0);
    assert_eq!(c[[1, 0]], 6.0);
    assert_eq!(c[[1, 1]], 7.0);
    assert_eq!(c[[2, 0]], 8.0);
    assert_eq!(c[[2, 1]], 9.0);
}

//#[test]
pub fn matrix_sub() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = matrix![2., 3.;
                    4., 5.;
                    6., 7.];

    // Allocate new memory
    let c = &a - &b;

    assert_eq!(c[[0, 0]], -1.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], -1.0);
    assert_eq!(c[[1, 1]], -1.0);
    assert_eq!(c[[2, 0]], -1.0);
    assert_eq!(c[[2, 1]], -1.0);

    // Reusing memory
    let c = a.clone() - &b;

    assert_eq!(c[[0, 0]], -1.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], -1.0);
    assert_eq!(c[[1, 1]], -1.0);
    assert_eq!(c[[2, 0]], -1.0);
    assert_eq!(c[[2, 1]], -1.0);

    // Reusing memory
    let c = &a - b.clone();

    assert_eq!(c[[0, 0]], -1.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], -1.0);
    assert_eq!(c[[1, 1]], -1.0);
    assert_eq!(c[[2, 0]], -1.0);
    assert_eq!(c[[2, 1]], -1.0);

    // Reusing memory
    let c = &a - b;

    assert_eq!(c[[0, 0]], -1.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], -1.0);
    assert_eq!(c[[1, 1]], -1.0);
    assert_eq!(c[[2, 0]], -1.0);
    assert_eq!(c[[2, 1]], -1.0);
}

//#[test]
pub fn matrix_f32_sub() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = 3.0;

    // Allocating new memory
    let c = &a - &b;

    assert_eq!(c[[0, 0]], -2.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], 0.0);
    assert_eq!(c[[1, 1]], 1.0);
    assert_eq!(c[[2, 0]], 2.0);
    assert_eq!(c[[2, 1]], 3.0);

    // Allocating new memory
    let c = &a - b;

    assert_eq!(c[[0, 0]], -2.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], 0.0);
    assert_eq!(c[[1, 1]], 1.0);
    assert_eq!(c[[2, 0]], 2.0);
    assert_eq!(c[[2, 1]], 3.0);

    // Reusing memory
    let c = a.clone() - &b;

    assert_eq!(c[[0, 0]], -2.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], 0.0);
    assert_eq!(c[[1, 1]], 1.0);
    assert_eq!(c[[2, 0]], 2.0);
    assert_eq!(c[[2, 1]], 3.0);

    // Reusing memory
    let c = a - b;

    assert_eq!(c[[0, 0]], -2.0);
    assert_eq!(c[[0, 1]], -1.0);
    assert_eq!(c[[1, 0]], 0.0);
    assert_eq!(c[[1, 1]], 1.0);
    assert_eq!(c[[2, 0]], 2.0);
    assert_eq!(c[[2, 1]], 3.0);
}

//#[test]
pub fn matrix_f32_div() {
    let a = matrix![1., 2.;
                    3., 4.;
                    5., 6.];
    let b = 3.0;

    // Allocating new memory
    let c = &a / &b;

    assert_eq!(c[[0, 0]], 1.0 / 3.0);
    assert_eq!(c[[0, 1]], 2.0 / 3.0);
    assert_eq!(c[[1, 0]], 1.0);
    assert_eq!(c[[1, 1]], 4.0 / 3.0);
    assert_eq!(c[[2, 0]], 5.0 / 3.0);
    assert_eq!(c[[2, 1]], 2.0);

    // Allocating new memory
    let c = &a / b;

    assert_eq!(c[[0, 0]], 1.0 / 3.0);
    assert_eq!(c[[0, 1]], 2.0 / 3.0);
    assert_eq!(c[[1, 0]], 1.0);
    assert_eq!(c[[1, 1]], 4.0 / 3.0);
    assert_eq!(c[[2, 0]], 5.0 / 3.0);
    assert_eq!(c[[2, 1]], 2.0);

    // Reusing memory
    let c = a.clone() / &b;

    assert_eq!(c[[0, 0]], 1.0 / 3.0);
    assert_eq!(c[[0, 1]], 2.0 / 3.0);
    assert_eq!(c[[1, 0]], 1.0);
    assert_eq!(c[[1, 1]], 4.0 / 3.0);
    assert_eq!(c[[2, 0]], 5.0 / 3.0);
    assert_eq!(c[[2, 1]], 2.0);

    // Reusing memory
    let c = a / b;

    assert_eq!(c[[0, 0]], 1.0 / 3.0);
    assert_eq!(c[[0, 1]], 2.0 / 3.0);
    assert_eq!(c[[1, 0]], 1.0);
    assert_eq!(c[[1, 1]], 4.0 / 3.0);
    assert_eq!(c[[2, 0]], 5.0 / 3.0);
    assert_eq!(c[[2, 1]], 2.0);
}

//#[test]
pub fn add_slice() {
    let a = 3.0;
    let mut b = Matrix::ones(3, 3) * 2.;
    let c = Matrix::ones(2, 2);

    {
        let d = MatrixSlice::from_matrix(&b, [1, 1], 2, 2);

        let m_1 = &d + a.clone();
        assert_eq!(m_1.into_vec(), vec![5.0; 4]);

        let m_2 = c.clone() + &d;
        assert_eq!(m_2.into_vec(), vec![3.0; 4]);

        let m_3 = &d + c.clone();
        assert_eq!(m_3.into_vec(), vec![3.0; 4]);

        let m_4 = &d + &d;
        assert_eq!(m_4.into_vec(), vec![4.0; 4]);
    }

    let e = MatrixSliceMut::from_matrix(&mut b, [1, 1], 2, 2);

    let m_1 = &e + a.clone();
    assert_eq!(m_1.into_vec(), vec![5.0; 4]);

    let m_2 = c.clone() + &e;
    assert_eq!(m_2.into_vec(), vec![3.0; 4]);

    let m_3 = &e + c;
    assert_eq!(m_3.into_vec(), vec![3.0; 4]);

    let m_4 = &e + &e;
    assert_eq!(m_4.into_vec(), vec![4.0; 4]);
}

//#[test]
pub fn sub_slice() {
    let a = 3.0;
    let b = Matrix::ones(2, 2);
    let mut c = Matrix::ones(3, 3) * 2.;

    {
        let d = MatrixSlice::from_matrix(&c, [1, 1], 2, 2);

        let m_1 = &d - a.clone();
        assert_eq!(m_1.into_vec(), vec![-1.0; 4]);

        let m_2 = b.clone() - &d;
        assert_eq!(m_2.into_vec(), vec![-1.0; 4]);

        let m_3 = &d - b.clone();
        assert_eq!(m_3.into_vec(), vec![1.0; 4]);

        let m_4 = &d - &d;
        assert_eq!(m_4.into_vec(), vec![0.0; 4]);
    }

    let e = MatrixSliceMut::from_matrix(&mut c, [1, 1], 2, 2);

    let m_1 = &e - a;
    assert_eq!(m_1.into_vec(), vec![-1.0; 4]);

    let m_2 = b.clone() - &e;
    assert_eq!(m_2.into_vec(), vec![-1.0; 4]);

    let m_3 = &e - b;
    assert_eq!(m_3.into_vec(), vec![1.0; 4]);

    let m_4 = &e - &e;
    assert_eq!(m_4.into_vec(), vec![0.0; 4]);
}

//#[test]
pub fn div_slice() {
    let a = 3.0;

    let mut b = Matrix::ones(3, 3) * 2.;

    {
        let c = MatrixSlice::from_matrix(&b, [1, 1], 2, 2);

        let m = c / a;
        assert_eq!(m.into_vec(), vec![2.0/3.0 ;4]);
    }

    let d = MatrixSliceMut::from_matrix(&mut b, [1, 1], 2, 2);

    let m = d / a;
    assert_eq!(m.into_vec(), vec![2.0/3.0 ;4]);
}

//#[test]
pub fn neg_slice() {
    let b = Matrix::ones(3, 3) * 2.;

    let c = MatrixSlice::from_matrix(&b, [1, 1], 2, 2);

    let m = -c;
    assert_eq!(m.into_vec(), vec![-2.0;4]);

    let mut b = Matrix::ones(3, 3) * 2.;

    let c = MatrixSliceMut::from_matrix(&mut b, [1, 1], 2, 2);

    let m = -c;
    assert_eq!(m.into_vec(), vec![-2.0;4]);
}

//#[test]
pub fn index_slice() {
    let mut b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    {
        let c = MatrixSlice::from_matrix(&b, [1, 1], 2, 2);

        assert_eq!(c[[0, 0]], 4);
        assert_eq!(c[[0, 1]], 5);
        assert_eq!(c[[1, 0]], 7);
        assert_eq!(c[[1, 1]], 8);
    }


    let mut c = MatrixSliceMut::from_matrix(&mut b, [1, 1], 2, 2);

    assert_eq!(c[[0, 0]], 4);
    assert_eq!(c[[0, 1]], 5);
    assert_eq!(c[[1, 0]], 7);
    assert_eq!(c[[1, 1]], 8);

    c[[0, 0]] = 9;

    assert_eq!(c[[0, 0]], 9);
    assert_eq!(c[[0, 1]], 5);
    assert_eq!(c[[1, 0]], 7);
    assert_eq!(c[[1, 1]], 8);
}

//#[test]
pub fn matrix_add_assign() {
    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a += &2;
    assert_eq!(a.into_vec(), (2..11).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a += 2;
    assert_eq!(a.into_vec(), (2..11).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a += &b;
    assert_eq!(a.into_vec(), (0..9).map(|x| 2 * x).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a += b;
    assert_eq!(a.into_vec(), (0..9).map(|x| 2 * x).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let mut b = Matrix::new(4, 4, (0..16).collect::<Vec<_>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);

        a += &c;
        assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

        let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
        a += c;
        assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);
    }

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let c = MatrixSliceMut::from_matrix(&mut b, [0, 0], 3, 3);
    a += &c;
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    a += c;
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

}

//#[test]
pub fn matrix_sub_assign() {
    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());

    a -= &2;
    assert_eq!(a.into_vec(), (-2..7).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    a -= 2;
    assert_eq!(a.into_vec(), (-2..7).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a -= &b;
    assert_eq!(a.into_vec(), vec![0; 9]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    a -= b;
    assert_eq!(a.into_vec(), vec![0; 9]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let mut b = Matrix::new(4, 4, (0..16).collect::<Vec<_>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        a -= &c;
        assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);

        let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
        a -= c;
        assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);
    }

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    let c = MatrixSliceMut::from_matrix(&mut b, [0, 0], 3, 3);
    a -= &c;
    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
    a -= c;
    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);
}

//#[test]
pub fn matrix_div_assign() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let res_data = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
    let mut a = Matrix::new(3, 3, a_data.clone());

    a /= &2f32;
    assert_eq!(a.into_vec(), res_data.clone());

    let mut a = Matrix::new(3, 3, a_data.clone());
    a /= 2f32;
    assert_eq!(a.into_vec(), res_data.clone());
}

//#[test]
pub fn matrix_mul_assign() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let res_data = vec![2f32, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0];
    let mut a = Matrix::new(3, 3, a_data.clone());

    a *= &2f32;
    assert_eq!(a.into_vec(), res_data.clone());

    let mut a = Matrix::new(3, 3, a_data.clone());
    a *= 2f32;
    assert_eq!(a.into_vec(), res_data.clone());
}

//#[test]
#[allow(unused_assignments, unused_variables)]
pub fn slice_add_assign() {
    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += &2;
    }
    assert_eq!(a.into_vec(), (2..11).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());

    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += 2;
    }
    assert_eq!(a.into_vec(), (2..11).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    let b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());

    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += &b;
    }
    assert_eq!(a.into_vec(), (0..9).map(|x| 2 * x).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += b;
    }
    assert_eq!(a.into_vec(), (0..9).map(|x| 2 * x).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    let mut b = Matrix::new(4, 4, (0..16).collect::<Vec<_>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += &c;
    }
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += c;
    }
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += &c;
    }
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSliceMut::from_matrix(&mut b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice += c;
    }
    assert_eq!(a.into_vec(), vec![0, 2, 4, 7, 9, 11, 14, 16, 18]);

}

//#[test]
#[allow(unused_assignments, unused_variables)]
pub fn slice_sub_assign() {
    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= &2;
    }
    assert_eq!(a.into_vec(), (-2..7).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= 2;
    }
    assert_eq!(a.into_vec(), (-2..7).collect::<Vec<_>>());

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        let b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
        a_slice -= &b;
    }
    assert_eq!(a.into_vec(), vec![0; 9]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        let b = Matrix::new(3, 3, (0..9).collect::<Vec<_>>());
        a_slice -= b;
    }
    assert_eq!(a.into_vec(), vec![0; 9]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    let mut b = Matrix::new(4, 4, (0..16).collect::<Vec<_>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= &c;
    }

    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSlice::from_matrix(&b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= c;
    }
    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSliceMut::from_matrix(&mut b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= &c;
    }
    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);

    let mut a = Matrix::new(3, 3, (0..9).collect::<Vec<i32>>());
    {
        let c = MatrixSliceMut::from_matrix(&mut b, [0, 0], 3, 3);
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice -= c;
    }
    assert_eq!(a.into_vec(), vec![0, 0, 0, -1, -1, -1, -2, -2, -2]);
}

//#[test]
#[allow(unused_assignments, unused_variables)]
pub fn slice_div_assign() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let res_data = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
    let mut a = Matrix::new(3, 3, a_data.clone());

    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice /= &2f32;
    }
    assert_eq!(a.into_vec(), res_data.clone());

    let mut a = Matrix::new(3, 3, a_data.clone());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice /= 2f32;
    }
    assert_eq!(a.into_vec(), res_data.clone());
}

//#[test]
#[allow(unused_assignments, unused_variables)]
pub fn slice_mul_assign() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let res_data = vec![2f32, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0];
    let mut a = Matrix::new(3, 3, a_data.clone());

    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice *= &2f32;
    }
    assert_eq!(a.into_vec(), res_data.clone());

    let mut a = Matrix::new(3, 3, a_data.clone());
    {
        let mut a_slice = MatrixSliceMut::from_matrix(&mut a, [0, 0], 3, 3);
        a_slice *= 2f32;
    }
    assert_eq!(a.into_vec(), res_data.clone());
}
