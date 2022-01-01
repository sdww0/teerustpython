use std::prelude::v1::*;
use rulinalg::macros::comparison::{
    AbsoluteElementwiseComparator, AbsoluteError, ElementwiseComparator,
    ExactElementwiseComparator, ExactError,
    UlpElementwiseComparator, UlpError,
    FloatElementwiseComparator,
};
use rulinalg::ulp::{Ulp, UlpComparisonResult};
use quickcheck::TestResult;
use std::f64;

/// Returns the next adjacent floating point number (in the direction of positive infinity)
fn next_f64(x: f64) -> f64 {
    use std::mem;
    let as_int = unsafe { mem::transmute::<f64, i64>(x) };
    unsafe { mem::transmute::<i64, f64>(as_int + 1) }
}

//#[test]
pub fn absolute_comparator_integer() {
    let comp = AbsoluteElementwiseComparator { tol: 1 };

    assert_eq!(comp.compare(0, 0), Ok(()));
    assert_eq!(comp.compare(1, 0), Ok(()));
    assert_eq!(comp.compare(-1, 0), Ok(()));
    assert_eq!(comp.compare(2, 0), Err(AbsoluteError(2)));
    assert_eq!(comp.compare(-2, 0), Err(AbsoluteError(2)));
}

//#[test]
pub fn absolute_comparator_floating_point() {
    let comp = AbsoluteElementwiseComparator { tol: 1.0 };

    // Note: floating point math is not generally exact, but
    // here we only compare with 0.0, so we can expect exact results.
    assert_eq!(comp.compare(0.0, 0.0), Ok(()));
    assert_eq!(comp.compare(1.0, 0.0), Ok(()));
    assert_eq!(comp.compare(-1.0, 0.0), Ok(()));
    assert_eq!(comp.compare(2.0, 0.0), Err(AbsoluteError(2.0)));
    assert_eq!(comp.compare(-2.0, 0.0), Err(AbsoluteError(2.0)));
}

quickcheck! {
    fn property_absolute_comparator_is_symmetric_i64(a: i64, b: i64, tol: i64) -> TestResult {
        if tol <= 0 {
            return TestResult::discard()
        }

        let comp = AbsoluteElementwiseComparator { tol: tol };
        TestResult::from_bool(comp.compare(a, b) == comp.compare(b, a))
    }
}

quickcheck! {
    fn property_absolute_comparator_is_symmetric_f64(a: f64, b: f64, tol: f64) -> TestResult {
        if tol <= 0.0 {
            return TestResult::discard()
        }

        // Floating point math is not exact, but the AbsoluteElementwiseComparator is designed
        // so that it gives exactly the same result when the argument positions are reversed
        let comp = AbsoluteElementwiseComparator { tol: tol };
        TestResult::from_bool(comp.compare(a, b) == comp.compare(b, a))
    }
}

quickcheck! {
    fn property_absolute_comparator_tolerance_is_not_strict_f64(tol: f64) -> TestResult {
        if tol <= 0.0 || !tol.is_finite() {
            return TestResult::discard()
        }

        // The comparator is defined by <=, not <
        let comp = AbsoluteElementwiseComparator { tol: tol };
        let includes_tol = comp.compare(tol, 0.0).is_ok();
        let excludes_next_after_tol = comp.compare(next_f64(tol), 0.0).is_err();
        TestResult::from_bool(includes_tol && excludes_next_after_tol)
    }
}

//#[test]
pub fn exact_comparator_integer() {
    let comp = ExactElementwiseComparator;

    assert_eq!(comp.compare(0, 0), Ok(()));
    assert_eq!(comp.compare(1, 0), Err(ExactError));
    assert_eq!(comp.compare(-1, 0), Err(ExactError));
    assert_eq!(comp.compare(1, -1), Err(ExactError));
}

//#[test]
pub fn exact_comparator_floating_point() {
    let comp = ExactElementwiseComparator;

    assert_eq!(comp.compare(0.0, 0.0), Ok(()));
    assert_eq!(comp.compare(-0.0, -0.0), Ok(()));
    assert_eq!(comp.compare(-0.0, 0.0), Ok(()));
    assert_eq!(comp.compare(1.0, 0.0), Err(ExactError));
    assert_eq!(comp.compare(-1.0, 0.0), Err(ExactError));
    assert_eq!(comp.compare(f64::NAN, 5.0), Err(ExactError));
}

quickcheck! {
    fn property_exact_comparator_is_symmetric_i64(a: i64, b: i64) -> bool {
        let comp = ExactElementwiseComparator;
        comp.compare(a, b) == comp.compare(b, a)
    }
}

quickcheck! {
    fn property_exact_comparator_is_symmetric_f64(a: f64, b: f64) -> bool {
        let comp = ExactElementwiseComparator;
        comp.compare(a, b) == comp.compare(b, a)
    }
}

quickcheck! {
    fn property_exact_comparator_matches_equality_operator_i64(a: i64, b: i64) -> bool {
        let comp = ExactElementwiseComparator;
        let result = comp.compare(a, b);

        match a == b {
            true =>  result == Ok(()),
            false => result == Err(ExactError)
        }
    }
}

quickcheck! {
    fn property_exact_comparator_matches_equality_operator_f64(a: f64, b: f64) -> bool {
        let comp = ExactElementwiseComparator;
        let result = comp.compare(a, b);

        match a == b {
            true =>  result == Ok(()),
            false => result == Err(ExactError)
        }
    }
}

//#[test]
pub fn ulp_comparator_f64() {
    // The Ulp implementation has its own set of tests, so we just want
    // to make a sample here
    let comp = UlpElementwiseComparator { tol: 1 };

    assert_eq!(comp.compare(0.0, 0.0), Ok(()));
    assert_eq!(comp.compare(0.0, -0.0), Ok(()));
    assert_eq!(comp.compare(-1.0, 1.0), Err(UlpError(UlpComparisonResult::IncompatibleSigns)));
    assert_eq!(comp.compare(1.0, 0.0), Err(UlpError(f64::ulp_diff(&1.0, &0.0))));
    assert_eq!(comp.compare(f64::NAN, 0.0), Err(UlpError(UlpComparisonResult::Nan)));;
}

quickcheck! {
    fn property_ulp_comparator_is_symmetric(a: f64, b: f64, tol: u64) -> TestResult {
        if tol == 0 {
            return TestResult::discard()
        }

        let comp = UlpElementwiseComparator { tol: tol };
        TestResult::from_bool(comp.compare(a, b) == comp.compare(b, a))
    }
}

quickcheck! {
    fn property_ulp_comparator_matches_ulp_trait(a: f64, b: f64, tol: u64) -> bool {
        let comp = UlpElementwiseComparator { tol: tol };
        let result = comp.compare(a, b);

        use rulinalg::ulp::UlpComparisonResult::{ExactMatch, Difference};

        match f64::ulp_diff(&a, &b) {
            ExactMatch =>                      result.is_ok(),
            Difference(diff) if diff <= tol => result.is_ok(),
            otherwise =>                       result == Err(UlpError(otherwise))
        }
    }
}

quickcheck! {
    fn property_ulp_comparator_next_f64_is_ok_when_inside_tolerance(x: f64) -> TestResult {
        let y = next_f64(x);

        if !(x.is_finite() && y.is_finite() && x.signum() == y.signum()) {
            return TestResult::discard()
        }

        let comp0 = UlpElementwiseComparator { tol: 0 };
        let comp1 = UlpElementwiseComparator { tol: 1 };

        let tol_0_fails = comp0.compare(x, y) == Err(UlpError(UlpComparisonResult::Difference(1)));
        let tol_1_succeeds = comp1.compare(x, y) == Ok(());

        TestResult::from_bool(tol_0_fails && tol_1_succeeds)
    }
}

quickcheck! {
    fn property_float_comparator_matches_abs_with_zero_ulp_tol(a: f64, b: f64, abstol: f64) -> TestResult {
        if abstol <= 0.0 {
            return TestResult::discard()
        }

        let abstol = abstol.abs();
        let comp = FloatElementwiseComparator::default().eps(abstol).ulp(0);
        let abscomp = AbsoluteElementwiseComparator { tol: abstol };
        let result = comp.compare(a, b);

        // Recall that the float comparator returns UlpError, so we cannot compare the results
        // of abscomp directly
        TestResult::from_bool(match abscomp.compare(a, b) {
            Err(AbsoluteError(_)) =>   result.is_err(),
            Ok(_) =>                   result.is_ok()
        })
    }
}

quickcheck! {
    fn property_float_comparator_matches_ulp_with_zero_eps_tol(a: f64, b: f64, max_ulp: u64) -> bool {
        let comp = FloatElementwiseComparator::default().eps(0.0).ulp(max_ulp);
        let ulpcomp = UlpElementwiseComparator { tol: max_ulp };

        comp.compare(a, b) == ulpcomp.compare(a, b)
    }
}
