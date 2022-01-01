use std::prelude::v1::*;
use rulinalg::vector::Vector;

// *****************************************************
// Index
// *****************************************************

//#[test]
pub fn vector_index_mut() {
    let our_vec = vec![1., 2., 3., 4.];
    let mut our_vector = Vector::new(our_vec.clone());

    for i in 0..4 {
        our_vector[i] += 1.;
    }

    assert_eq!(our_vector, vector![2., 3., 4., 5.]);
}

// *****************************************************
// Arithmetic Ops
// *****************************************************

//#[test]
pub fn vector_mul_f32_broadcast() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = 3.0;

    let exp = vector![3.0, 6.0, 9.0, 12.0, 15.0, 18.0];

    // Allocating new memory
    let c = &a * &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a * b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() * &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a * b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_mul_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![2, 4, 6, 8, 10];

    // Allocating new memory
    let c = &a * &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a * b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() * &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a * b;
    assert_eq!(c, exp);
}

// mul_xxx_elemwise is tested in impl_vec

//#[test]
pub fn vector_div_f32_broadcast() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = 3.0;

    let exp = vector![1. / 3., 2. / 3., 3. / 3., 4. / 3., 5. / 3., 6. / 3.];

    // Allocating new memory
    let c = &a / &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a / b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() / &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a / b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_div_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![0, 1, 1, 2, 2];

    // Allocating new memory
    let c = &a / &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a / b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() / &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a / b;
    assert_eq!(c, exp);
}

// div_xxx_elemwise is tested in impl_vec

//#[test]
pub fn vector_add_f32_broadcast() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = 2.0;

    let exp = vector![3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    // Allocating new memory
    let c = &a + &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a + b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a + b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_add_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![3, 4, 5, 6, 7];

    // Allocating new memory
    let c = &a + &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a + b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a + b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_add_f32_elemwise() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

    let exp = vector![3.0, 5.0, 7.0, 9.0, 11.0, 13.0];

    // Allocating new memory
    let c = &a + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a + b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a + b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_add_int_elemwise() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let b = vector![2, 3, 4, 5, 6, 7];

    let exp = vector![3, 5, 7, 9, 11, 13];

    // Allocating new memory
    let c = &a + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a + b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() + &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a + b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_sub_f32_broadcast() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = 2.0;

    let exp = vector![-1.0, 0.0, 1.0, 2.0, 3.0, 4.0];

    // Allocating new memory
    let c = &a - &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a - b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a - b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_sub_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![-1, 0, 1, 2, 3];

    // Allocating new memory
    let c = &a - &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a - b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a - b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_sub_f32_elemwise() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

    let exp = vector![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];

    // Allocating new memory
    let c = &a - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a - b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a - b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_sub_int_elemwise() {
    let a = vector![10, 11, 12, 13, 14];
    let b = vector![2, 4, 6, 8, 10];

    let exp = vector![8, 7, 6, 5, 4];

    // Allocating new memory
    let c = &a - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a - b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() - &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a - b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_rem_f32_broadcast() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = 2.0;

    let exp = vector![1.0, 0.0, 1.0, 0.0, 1.0, 0.0];

    // Allocating new memory
    let c = &a % &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a % b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a % b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_rem_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 3;

    let exp = vector![1, 2, 0, 1, 2];

    // Allocating new memory
    let c = &a % &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a % b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a % b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_rem_f32_elemwise() {
    let a = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vector![3.0, 3.0, 3.0, 4.0, 4.0, 4.0];

    let exp = vector![1.0, 2.0, 0.0, 0.0, 1.0, 2.0];

    // Allocating new memory
    let c = &a % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a % b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a % b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_rem_int_elemwise() {
    let a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![1, 0, 1, 1, 2];

    // Allocating new memory
    let c = &a % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a % b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() % &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a % b;
    assert_eq!(c, exp);
}

// *****************************************************
// Arithmetic Assignments
// *****************************************************

//#[test]
pub fn vector_add_assign_int_broadcast() {
    let mut a = (0..9).collect::<Vector<_>>();

    let exp = (2..11).collect::<Vector<_>>();

    a += &2;
    assert_eq!(a, exp);

    let mut a = (0..9).collect::<Vector<_>>();

    a += 2;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_add_assign_int_elemwise() {
    let mut a = (0..9).collect::<Vector<_>>();
    let b = (0..9).collect::<Vector<_>>();

    let exp = (0..9).map(|x| 2 * x).collect::<Vector<_>>();

    a += &b;
    assert_eq!(a, exp);

    let mut a = (0..9).collect::<Vector<_>>();

    a += b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_sub_assign_int_broadcast() {
    let mut a = (0..9).collect::<Vector<_>>();

    let exp = (-2..7).collect::<Vector<_>>();

    a -= &2;
    assert_eq!(a, exp);

    let mut a = (0..9).collect::<Vector<i32>>();
    a -= 2;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_sub_assign_int_elemwise() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![-1, 0, 1, 1, 2];

    a -= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a -= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_div_assign_f32_broadcast() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let exp = vector![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];

    let mut a = Vector::new(a_data.clone());

    a /= &2f32;
    assert_eq!(a, exp);

    let mut a = Vector::new(a_data.clone());
    a /= 2f32;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_mul_assign_f32_broadcast() {
    let a_data = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let exp = vector![2f32, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0];
    let mut a = Vector::new(a_data.clone());

    a *= &2f32;
    assert_eq!(a, exp);

    let mut a = Vector::new(a_data.clone());
    a *= 2f32;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_rem_assign_int_broadcast() {
    let mut a = vector![1, 2, 3];

    let exp = vector![1, 2, 0];

    a %= &3;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3];
    a %= 3;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_rem_assign_int_elemwise() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![1, 0, 1, 1, 2];

    a %= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a %= b;
    assert_eq!(a, exp);
}

// *****************************************************
// Bitwise Ops
// *****************************************************

//#[test]
pub fn vector_bitand_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 & 2, 2 & 2, 3 & 2, 4 & 2, 5 & 2];

    // Allocating new memory
    let c = &a & &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a & b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a & b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitand_bool_broadcast() {
    let a = vector![true, false, true];
    let b = true;

    let exp = vector![true, false, true];

    // Allocating new memory
    let c = &a & &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a & b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a & b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitand_int_elemwise() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let b = vector![2, 3, 4, 5, 6, 7];

    let exp = vector![1 & 2, 2 & 3, 3 & 4, 4 & 5, 5 & 6, 6 & 7];

    // Allocating new memory
    let c = &a & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a & b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a & b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitand_bool_elemwise() {
    let a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![true, false, false, false];

    // Allocating new memory
    let c = &a & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a & b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() & &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a & b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitor_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 | 2, 2 | 2, 3 | 2, 4 | 2, 5 | 2];

    // Allocating new memory
    let c = &a | &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a | b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a | b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitor_bool_broadcast() {
    let a = vector![true, false, true];
    let b = true;

    let exp = vector![true, true, true];

    // Allocating new memory
    let c = &a | &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a | b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a | b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitor_int_elemwise() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let b = vector![2, 3, 4, 5, 6, 7];

    let exp = vector![1 | 2, 2 | 3, 3 | 4, 4 | 5, 5 | 6, 6 | 7];

    // Allocating new memory
    let c = &a | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a | b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a | b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitor_bool_elemwise() {
    let a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![true, true, true, false];

    // Allocating new memory
    let c = &a | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a | b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() | &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a | b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitxor_int_broadcast() {
    let a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 ^ 2, 2 ^ 2, 3 ^ 2, 4 ^ 2, 5 ^ 2];

    // Allocating new memory
    let c = &a ^ &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a ^ b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a ^ b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitxor_bool_broadcast() {
    let a = vector![true, false, true];
    let b = true;

    let exp = vector![false, true, false];

    // Allocating new memory
    let c = &a ^ &b;
    assert_eq!(c, exp);

    // Allocating new memory
    let c = &a ^ b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a ^ b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitxor_int_elemwise() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let b = vector![2, 3, 4, 5, 6, 7];

    let exp = vector![1 ^ 2, 2 ^ 3, 3 ^ 4, 4 ^ 5, 5 ^ 6, 6 ^ 7];

    // Allocating new memory
    let c = &a ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a ^ b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a ^ b;
    assert_eq!(c, exp);
}

//#[test]
pub fn vector_bitxor_bool_elemwise() {
    let a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![false, true, true, false];

    // Allocating new memory
    let c = &a ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = &a ^ b.clone();
    assert_eq!(c, exp);

    // Reusing memory
    let c = a.clone() ^ &b;
    assert_eq!(c, exp);

    // Reusing memory
    let c = a ^ b;
    assert_eq!(c, exp);
}

// *****************************************************
// Bitwise Assignments
// *****************************************************

//#[test]
pub fn vector_bitand_assign_int_broadcast() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 & 2, 2 & 2, 3 & 2, 4 & 2, 5 & 2];

    a &= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a &= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitand_assign_bool_broadcast() {
    let mut a = vector![true, true, false, false];
    let b = true;

    let exp = vector![true, true, false, false];

    a &= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a &= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitand_assign_int_elemwise() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![1 & 2, 2 & 2, 3 & 2, 4 & 3, 5 & 3];

    a &= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a &= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitand_assign_bool_elemwise() {
    let mut a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![true, false, false, false];

    a &= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a &= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitor_assign_int_broadcast() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 | 2, 2 | 2, 3 | 2, 4 | 2, 5 | 2];

    a |= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a |= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitor_assign_bool_broadcast() {
    let mut a = vector![true, true, false, false];
    let b = true;

    let exp = vector![true, true, true, true];

    a |= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a |= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitor_assign_int_elemwise() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![1 | 2, 2 | 2, 3 | 2, 4 | 3, 5 | 3];

    a |= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a |= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitor_assign_bool_elemwise() {
    let mut a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![true, true, true, false];

    a |= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a |= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitxor_assign_int_broadcast() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = 2;

    let exp = vector![1 ^ 2, 2 ^ 2, 3 ^ 2, 4 ^ 2, 5 ^ 2];

    a ^= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a ^= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitxor_assign_bool_broadcast() {
    let mut a = vector![true, true, false, false];
    let b = true;

    let exp = vector![false, false, true, true];

    a ^= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a ^= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitxor_assign_int_elemwise() {
    let mut a = vector![1, 2, 3, 4, 5];
    let b = vector![2, 2, 2, 3, 3];

    let exp = vector![1 ^ 2, 2 ^ 2, 3 ^ 2, 4 ^ 3, 5 ^ 3];

    a ^= &b;
    assert_eq!(a, exp);

    let mut a = vector![1, 2, 3, 4, 5];

    a ^= b;
    assert_eq!(a, exp);
}

//#[test]
pub fn vector_bitxor_assign_bool_elemwise() {
    let mut a = vector![true, true, false, false];
    let b = vector![true, false, true, false];

    let exp = vector![false, true, true, false];

    a ^= &b;
    assert_eq!(a, exp);

    let mut a = vector![true, true, false, false];

    a ^= b;
    assert_eq!(a, exp);
}

// *****************************************************
// Unary Ops
// *****************************************************

//#[test]
pub fn vector_neg_f32() {
    let a = vector![1., 2., 3., 4., 5., 6.];
    let exp = vector![-1., -2., -3., -4., -5., -6.];

    assert_eq!(- &a, exp);
    assert_eq!(- a, exp);
}

//#[test]
pub fn vector_neg_int() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let exp = vector![-1, -2, -3, -4, -5, -6];

    assert_eq!(- &a, exp);
    assert_eq!(- a, exp);
}

//#[test]
pub fn vector_not_int() {
    let a = vector![1, 2, 3, 4, 5, 6];
    let exp = vector![!1, !2, !3, !4, !5, !6];

    assert_eq!(!&a, exp);
    assert_eq!(!a, exp);
}

//#[test]
pub fn vector_not_bool() {
    let a = vector![true, false, true];
    let exp = vector![false, true, false];

    assert_eq!(!&a, exp);
    assert_eq!(!a, exp);
}
