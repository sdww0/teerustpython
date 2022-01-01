use std::prelude::v1::*;
use rulinalg::vector::Vector;
use rulinalg::norm::Euclidean;

//#[test]
pub fn test_display() {
    let v = vector![1, 2, 3, 4];
    assert_eq!(format!("{}", v), "[ 1, 2, 3, 4]");

    let v2 = vector![3.3, 4.0, 5.0, 6.0];
    assert_eq!(format!("{}", v2), "[ 3.3, 4, 5, 6]");
    assert_eq!(format!("{:.1}", v2), "[ 3.3, 4.0, 5.0, 6.0]");
}

//#[test]
pub fn test_equality() {
    let v = vector![1, 2, 3, 4];
    let v_redux = v.clone();
    assert_eq!(v, v_redux);
}

//#[test]
pub fn create_vector_new() {
    let a = vector![1.0; 12];

    assert_eq!(a.size(), 12);

    for i in 0..12 {
        assert_eq!(a[i], 1.0);
    }
}

//#[test]
pub fn create_vector_new_from_slice() {
    let data_vec: Vec<u32> = vec![1, 2, 3];
    let data_slice: &[u32] = &data_vec[..];
    let from_vec = Vector::new(data_vec.clone());
    let from_slice = Vector::new(data_slice);
    assert_eq!(from_vec, from_slice);
}

//#[test]
pub fn create_vector_from_fn() {
    let v1 = Vector::from_fn(3, |x| x + 1);
    assert_eq!(v1, vector![1, 2, 3]);

    let v2 = Vector::from_fn(3, |x| x as f64);
    assert_eq!(v2, vector![0., 1., 2.]);

    let mut z = 0;
    let v3 = Vector::from_fn(3, |x| { z += 1; x + z });
    assert_eq!(v3, vector![0 + 1, 1 + 2, 2 + 3]);

    let v4 = Vector::from_fn(3, move |x| x + 1);
    assert_eq!(v4, vector![1, 2, 3]);

    let v5 = Vector::from_fn(0, |x| x);
    assert_eq!(v5, Vector::new(vec![]));
}

//#[test]
pub fn create_vector_zeros() {
    let a = Vector::<f32>::zeros(7);

    assert_eq!(a.size(), 7);

    for i in 0..7 {
        assert_eq!(a[i], 0.0);
    }
}

//#[test]
pub fn vector_dot_product() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![3.0; 6];

    let c = a.dot(&b);

    assert_eq!(c, 63.0);
}

//#[test]
pub fn vector_euclidean_norm() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = a.norm(Euclidean);

    assert_eq!(b, (1. + 4. + 9. + 16. + 25. + 36. as f32).sqrt());
}

//#[test]
pub fn vector_iteration() {
    let our_vec = vec![2i32, 7, 1, 8, 2, 8];
    let our_vector = Vector::new(our_vec.clone());
    let our_vector_again = our_vector.clone();

    // over Vector (consuming)
    let mut our_recovered_vec = Vec::new();
    for i in our_vector {
        our_recovered_vec.push(i);
    }
    assert_eq!(our_recovered_vec, our_vec);

    // over &Vector
    let mut our_refcovered_vec = Vec::new();
    for i in &our_vector_again {
        our_refcovered_vec.push(*i);
    }
    assert_eq!(our_refcovered_vec, our_vec);
}

//#[test]
pub fn vector_from_iter() {
    let v1: Vector<usize> = (2..5).collect();
    let exp1 = vector![2, 3, 4];
    assert_eq!(v1, exp1);

    let orig: Vec<f64> = vec![2., 3., 4.];
    let v2: Vector<f64> = orig.iter().map(|x| x + 1.).collect();
    let exp2 = vector![3., 4., 5.];
    assert_eq!(v2, exp2);
}

//#[test]
pub fn vector_get_unchecked() {
    let v1 = vector![1, 2, 3];
    unsafe {
        assert_eq!(v1.get_unchecked(1), &2);
    }

    let mut v2 = vector![1, 2, 3];

    unsafe {
        let elem = v2.get_unchecked_mut(1);
        *elem = 4;
    }
    assert_eq!(v2, vector![1, 4, 3]);
}

//#[test]
pub fn vector_mul_f32_elemwise() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

    let exp = vector![2.0, 6.0, 12.0, 20.0, 30.0, 42.0];

    // Allocating new memory
    let c = &a.elemul(&b);
    assert_eq!(c, &exp);

    // Allocating new memory
    let c = a.elemul(&b);
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_mul_int_elemwise() {
    let a = vector![1, 2, 3, 4];
    let b = vector![2, 4, 6, 8];

    let exp = vector![2, 8, 18, 32];

    // Allocating new memory
    let c = &a.elemul(&b);
    assert_eq!(c, &exp);

    // Allocating new memory
    let c = a.elemul(&b);
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_div_f32_elemwise() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

    let exp = vector![1. / 2., 2. / 3., 3. / 4., 4. / 5., 5. / 6., 6. / 7.];

    // Allocating new memory
    let c = &a.elediv(&b);
    assert_eq!(c, &exp);

    // Allocating new memory
    let c = a.elediv(&b);
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_div_int_elemwise() {
    let a = vector![2, 4, 6, 8];
    let b = vector![2, 2, 3, 3];

    let exp = vector![1, 2, 2, 2];

    // Allocating new memory
    let c = &a.elediv(&b);
    assert_eq!(c, &exp);

    // Allocating new memory
    let c = a.elediv(&b);
    assert_eq!(c, exp);
}
