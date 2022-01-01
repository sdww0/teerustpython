use rand;
use self::rand::Rng;

use wasmi::nan_preserving_float::{F32, F64};

use core::{
    fmt::Debug,
    iter,
    ops::{Add, Div, Mul, Neg, Sub},
};

fn test_ops<T, F, I>(iter: I)
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Copy
        + Debug
        + PartialEq,
    F: Into<T>
        + Add<Output = F>
        + Div<Output = F>
        + Mul<Output = F>
        + Sub<Output = F>
        + Neg<Output = F>
        + Copy
        + Debug,
    I: IntoIterator<Item = (F, F)>,
{
    for (a, b) in iter {
        assert_eq!((a + b).into(), a.into() + b.into());
        assert_eq!((a - b).into(), a.into() - b.into());
        assert_eq!((a * b).into(), a.into() * b.into());
        assert_eq!((a / b).into(), a.into() / b.into());
        assert_eq!((-a).into(), -a.into());
        assert_eq!((-b).into(), -b.into());
    }
}

//#[test]
pub fn test_ops_f32() {
    let mut rng = rand::thread_rng();
    let iter = iter::repeat(()).map(|_| rng.gen());

    test_ops::<F32, f32, _>(iter.take(1000));
}

//#[test]
pub fn test_ops_f64() {
    let mut rng = rand::thread_rng();
    let iter = iter::repeat(()).map(|_| rng.gen());

    test_ops::<F64, f64, _>(iter.take(1000));
}

//#[test]
pub fn test_neg_nan_f32() {
    assert_eq!((-F32(0xff80_3210)).0, 0x7f80_3210);
}

//#[test]
pub fn test_neg_nan_f64() {
    assert_eq!((-F64(0xff80_3210_0000_0000)).0, 0x7f80_3210_0000_0000);
}
