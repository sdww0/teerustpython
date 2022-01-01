use rulinalg::ulp::Ulp;
use rulinalg::ulp::UlpComparisonResult;
use std::mem;
use std::{f32, f64};
use quickcheck::TestResult;

//#[test]
pub fn plus_minus_zero_is_exact_match_f32() {
    assert!(f32::ulp_diff(&0.0, &0.0) == UlpComparisonResult::ExactMatch);
    assert!(f32::ulp_diff(&-0.0, &-0.0) == UlpComparisonResult::ExactMatch);
    assert!(f32::ulp_diff(&0.0, &-0.0) == UlpComparisonResult::ExactMatch);
    assert!(f32::ulp_diff(&-0.0, &0.0) == UlpComparisonResult::ExactMatch);
}

//#[test]
pub fn plus_minus_zero_is_exact_match_f64() {
    assert!(f64::ulp_diff(&0.0, &0.0) == UlpComparisonResult::ExactMatch);
    assert!(f64::ulp_diff(&-0.0, &-0.0) == UlpComparisonResult::ExactMatch);
    assert!(f64::ulp_diff(&0.0, &-0.0) == UlpComparisonResult::ExactMatch);
    assert!(f64::ulp_diff(&-0.0, &0.0) == UlpComparisonResult::ExactMatch);
}

//#[test]
pub fn f32_double_nan() {
    assert!(f32::ulp_diff(&f32::NAN, &f32::NAN) == UlpComparisonResult::Nan);
}

//#[test]
pub fn f64_double_nan() {
    assert!(f64::ulp_diff(&f64::NAN, &f64::NAN) == UlpComparisonResult::Nan);
}

quickcheck! {
    fn property_exact_match_for_finite_f32_self_comparison(x: f32) -> TestResult {
        if x.is_finite() {
            TestResult::from_bool(f32::ulp_diff(&x, &x) == UlpComparisonResult::ExactMatch)
        } else {
            TestResult::discard()
        }
    }
}

quickcheck! {
    fn property_exact_match_for_finite_f64_self_comparison(x: f64) -> TestResult {
        if x.is_finite() {
            TestResult::from_bool(f64::ulp_diff(&x, &x) == UlpComparisonResult::ExactMatch)
        } else {
            TestResult::discard()
        }
    }
}

quickcheck! {
    fn property_recovers_ulp_diff_when_f32_constructed_from_i32(a: i32, b: i32) -> TestResult {
        if a == b {
            // Ignore self-comparisons, as it makes the below test have more complicated logic,
            // and moreover we test self-comparisons in another property.
            return TestResult::discard();
        }

        let x = unsafe { mem::transmute::<i32, f32>(a) };
        let y = unsafe { mem::transmute::<i32, f32>(b) };

        // Discard the input if it's non-finite or has different signs
        if x.is_finite() && y.is_finite() && x.signum() == y.signum() {
            TestResult::from_bool(f32::ulp_diff(&x, &y) == UlpComparisonResult::Difference((b - a).abs() as u64))
        } else {
            TestResult::discard()
        }
    }
}

quickcheck! {
    fn property_recovers_ulp_diff_when_f64_constructed_from_i64(a: i64, b: i64) -> TestResult {
        if a == b {
            // Ignore self-comparisons, as it makes the below test have more complicated logic,
            // and moreover we test self-comparisons in another property.
            return TestResult::discard();
        }

        let x = unsafe { mem::transmute::<i64, f64>(a) };
        let y = unsafe { mem::transmute::<i64, f64>(b) };

        // Discard the input if it's non-finite or has different signs
        if x.is_finite() && y.is_finite() && x.signum() == y.signum() {
            TestResult::from_bool(f64::ulp_diff(&x, &y) == UlpComparisonResult::Difference((b - a).abs() as u64))
        } else {
            TestResult::discard()
        }
    }
}

quickcheck! {
    fn property_f32_incompatible_signs_yield_corresponding_enum_value(x: f32, y: f32) -> TestResult {
        if x.signum() == y.signum() {
            TestResult::discard()
        } else if x.is_nan() || y.is_nan() {
            TestResult::discard()
        } else {
            TestResult::from_bool(f32::ulp_diff(&x, &y) == UlpComparisonResult::IncompatibleSigns)
        }
    }
}

quickcheck! {
    fn property_f64_incompatible_signs_yield_corresponding_enum_value(x: f64, y: f64) -> TestResult {
        if x.signum() == y.signum() {
            TestResult::discard()
        } else if x.is_nan() || y.is_nan() {
            TestResult::discard()
        } else {
            TestResult::from_bool(f64::ulp_diff(&x, &y) == UlpComparisonResult::IncompatibleSigns)
        }
    }
}

quickcheck! {
    fn property_f32_nan_gives_nan_enum_value(x: f32) -> bool {
        f32::ulp_diff(&f32::NAN, &x) == UlpComparisonResult::Nan
        && f32::ulp_diff(&x, &f32::NAN) == UlpComparisonResult::Nan
    }
}

quickcheck! {
    fn property_f64_nan_gives_nan_enum_value(x: f64) -> bool {
        f64::ulp_diff(&f64::NAN, &x) == UlpComparisonResult::Nan
        && f64::ulp_diff(&x, &f64::NAN) == UlpComparisonResult::Nan
    }
}
