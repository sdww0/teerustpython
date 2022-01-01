use std::prelude::v1::*;
use uuid::*;
use crate::test_util;

macro_rules! check {
    ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
        $buf.clear();
        write!($buf, $format, $target).unwrap();
        assert!($buf.len() == $len);
        assert!($buf.chars().all($cond), "{}", $buf);
    };
}

//#[test]
pub fn test_uuid_compare() {
    let uuid1 = test_util::new();
    let uuid2 = test_util::new2();

    assert_eq!(uuid1, uuid1);
    assert_eq!(uuid2, uuid2);

    assert_ne!(uuid1, uuid2);
    assert_ne!(uuid2, uuid1);
}

//#[test]
pub fn test_uuid_default() {
    let default_uuid = Uuid::default();
    let nil_uuid = Uuid::nil();

    assert_eq!(default_uuid, nil_uuid);
}

//#[test]
pub fn test_uuid_display() {
    use core::fmt::Write;

    let uuid = test_util::new();
    let s = uuid.to_string();
    let mut buffer = String::new();

    assert_eq!(s, uuid.to_hyphenated().to_string());

    check!(buffer, "{}", uuid, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

//#[test]
pub fn test_uuid_lowerhex() {
    use core::fmt::Write;

    let mut buffer = String::new();
    let uuid = test_util::new();

    check!(buffer, "{:x}", uuid, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

// noinspection RsAssertEqual
//#[test]
pub fn test_uuid_operator_eq() {
    let uuid1 = test_util::new();
    let uuid1_dup = uuid1.clone();
    let uuid2 = test_util::new2();

    assert!(uuid1 == uuid1);
    assert!(uuid1 == uuid1_dup);
    assert!(uuid1_dup == uuid1);

    assert!(uuid1 != uuid2);
    assert!(uuid2 != uuid1);
    assert!(uuid1_dup != uuid2);
    assert!(uuid2 != uuid1_dup);
}

//#[test]
pub fn test_uuid_to_string() {
    use core::fmt::Write;

    let uuid = test_util::new();
    let s = uuid.to_string();
    let mut buffer = String::new();

    assert_eq!(s.len(), 36);

    check!(buffer, "{}", s, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

//#[test]
pub fn test_uuid_upperhex() {
    use core::fmt::Write;

    let mut buffer = String::new();
    let uuid = test_util::new();

    check!(buffer, "{:X}", uuid, 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
}
