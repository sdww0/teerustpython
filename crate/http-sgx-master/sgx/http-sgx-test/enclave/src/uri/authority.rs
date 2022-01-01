use std::prelude::v1::*;
use http::uri::*;

//#[test]
pub fn parse_empty_string_is_error() {
    let err = Authority::parse_non_empty(b"").unwrap_err();
    assert_eq!(err.0, ErrorKind::Empty);
}

//#[test]
pub fn equal_to_self_of_same_authority() {
    let authority1: Authority = "example.com".parse().unwrap();
    let authority2: Authority = "EXAMPLE.COM".parse().unwrap();
    assert_eq!(authority1, authority2);
    assert_eq!(authority2, authority1);
}

//#[test]
pub fn not_equal_to_self_of_different_authority() {
    let authority1: Authority = "example.com".parse().unwrap();
    let authority2: Authority = "test.com".parse().unwrap();
    assert_ne!(authority1, authority2);
    assert_ne!(authority2, authority1);
}

//#[test]
pub fn equates_with_a_str() {
    let authority: Authority = "example.com".parse().unwrap();
    assert_eq!(&authority, "EXAMPLE.com");
    assert_eq!("EXAMPLE.com", &authority);
    assert_eq!(authority, "EXAMPLE.com");
    assert_eq!("EXAMPLE.com", authority);
}

//#[test]
pub fn not_equal_with_a_str_of_a_different_authority() {
    let authority: Authority = "example.com".parse().unwrap();
    assert_ne!(&authority, "test.com");
    assert_ne!("test.com", &authority);
    assert_ne!(authority, "test.com");
    assert_ne!("test.com", authority);
}

//#[test]
pub fn equates_with_a_string() {
    let authority: Authority = "example.com".parse().unwrap();
    assert_eq!(authority, "EXAMPLE.com".to_string());
    assert_eq!("EXAMPLE.com".to_string(), authority);
}

//#[test]
pub fn equates_with_a_string_of_a_different_authority() {
    let authority: Authority = "example.com".parse().unwrap();
    assert_ne!(authority, "test.com".to_string());
    assert_ne!("test.com".to_string(), authority);
}

//#[test]
pub fn compares_to_self() {
    let authority1: Authority = "abc.com".parse().unwrap();
    let authority2: Authority = "def.com".parse().unwrap();
    assert!(authority1 < authority2);
    assert!(authority2 > authority1);
}

//#[test]
pub fn compares_with_a_str() {
    let authority: Authority = "def.com".parse().unwrap();
    // with ref
    assert!(&authority < "ghi.com");
    assert!("ghi.com" > &authority);
    assert!(&authority > "abc.com");
    assert!("abc.com" < &authority);

    // no ref
    assert!(authority < "ghi.com");
    assert!("ghi.com" > authority);
    assert!(authority > "abc.com");
    assert!("abc.com" < authority);
}

//#[test]
pub fn compares_with_a_string() {
    let authority: Authority = "def.com".parse().unwrap();
    assert!(authority < "ghi.com".to_string());
    assert!("ghi.com".to_string() > authority);
    assert!(authority > "abc.com".to_string());
    assert!("abc.com".to_string() < authority);
}

//#[test]
pub fn allows_percent_in_userinfo() {
    let authority_str = "a%2f:b%2f@example.com";
    let authority: Authority = authority_str.parse().unwrap();
    assert_eq!(authority, authority_str);
}

//#[test]
pub fn rejects_percent_in_hostname() {
    let err = Authority::parse_non_empty(b"example%2f.com").unwrap_err();
    assert_eq!(err.0, ErrorKind::InvalidAuthority);

    let err = Authority::parse_non_empty(b"a%2f:b%2f@example%2f.com").unwrap_err();
    assert_eq!(err.0, ErrorKind::InvalidAuthority);
}
