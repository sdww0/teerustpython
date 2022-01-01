use std::prelude::v1::*;
use quickcheck::{QuickCheck, StdGen};
use rand::{self, rngs::OsRng};

//#[test]
pub fn shrinking_regression_issue_126() {
    fn thetest(vals: Vec<bool>) -> bool {
        vals.iter().filter(|&v| *v).count() < 2
    }
    let failing_case =
        QuickCheck::new()
        .quicktest(thetest as fn(vals: Vec<bool>) -> bool)
        .unwrap_err();
    let expected_argument = format!("{:?}", [true, true]);
    assert_eq!(failing_case.arguments, vec![expected_argument]);
}

//#[test]
pub fn size_for_small_types_issue_143() {
    fn t(_: i8) -> bool { true }
    QuickCheck::new()
        .gen(StdGen::new(rand::thread_rng(), 129))
        .quickcheck(t as fn(i8) -> bool);
}

//#[test]
pub fn different_generator() {
    fn prop(_: i32) -> bool { true }
    QuickCheck::with_gen(StdGen::new(OsRng, 129))
        .quickcheck(prop as fn(i32) -> bool);
    QuickCheck::new()
        .gen(StdGen::new(OsRng, 129))
        .quickcheck(prop as fn(i32) -> bool);
}
