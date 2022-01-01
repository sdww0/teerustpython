use http::*;
use std::str::FromStr;
//#[test]
pub fn test_method_eq() {
    assert_eq!(Method::GET, Method::GET);
    assert_eq!(Method::GET, "GET");
    assert_eq!(&Method::GET, "GET");

    assert_eq!("GET", Method::GET);
    assert_eq!("GET", &Method::GET);

    assert_eq!(&Method::GET, Method::GET);
    assert_eq!(Method::GET, &Method::GET);
}

//#[test]
pub fn test_invalid_method() {
    assert!(Method::from_str("").is_err());
    assert!(Method::from_bytes(b"").is_err());
}
