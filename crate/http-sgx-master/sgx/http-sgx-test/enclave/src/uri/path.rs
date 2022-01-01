//use http::uri::path::*;
use http::uri::*;
use std::prelude::v1::*;

//#[test]
pub fn equal_to_self_of_same_path() {
    let p1: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    let p2: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    assert_eq!(p1, p2);
    assert_eq!(p2, p1);
}

//#[test]
pub fn not_equal_to_self_of_different_path() {
    let p1: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    let p2: PathAndQuery = "/world&foo=bar".parse().unwrap();
    assert_ne!(p1, p2);
    assert_ne!(p2, p1);
}

//#[test]
pub fn equates_with_a_str() {
    let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    assert_eq!(&path_and_query, "/hello/world&foo=bar");
    assert_eq!("/hello/world&foo=bar", &path_and_query);
    assert_eq!(path_and_query, "/hello/world&foo=bar");
    assert_eq!("/hello/world&foo=bar", path_and_query);
}

//#[test]
pub fn not_equal_with_a_str_of_a_different_path() {
    let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    // as a reference
    assert_ne!(&path_and_query, "/hello&foo=bar");
    assert_ne!("/hello&foo=bar", &path_and_query);
    // without reference
    assert_ne!(path_and_query, "/hello&foo=bar");
    assert_ne!("/hello&foo=bar", path_and_query);
}

//#[test]
pub fn equates_with_a_string() {
    let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    assert_eq!(path_and_query, "/hello/world&foo=bar".to_string());
    assert_eq!("/hello/world&foo=bar".to_string(), path_and_query);
}

//#[test]
pub fn not_equal_with_a_string_of_a_different_path() {
    let path_and_query: PathAndQuery = "/hello/world&foo=bar".parse().unwrap();
    assert_ne!(path_and_query, "/hello&foo=bar".to_string());
    assert_ne!("/hello&foo=bar".to_string(), path_and_query);
}

//#[test]
pub fn compares_to_self() {
    let p1: PathAndQuery = "/a/world&foo=bar".parse().unwrap();
    let p2: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
    assert!(p1 < p2);
    assert!(p2 > p1);
}

//#[test]
pub fn compares_with_a_str() {
    let path_and_query: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
    // by ref
    assert!(&path_and_query < "/c/world&foo=bar");
    assert!("/c/world&foo=bar" > &path_and_query);
    assert!(&path_and_query > "/a/world&foo=bar");
    assert!("/a/world&foo=bar" < &path_and_query);

    // by val
    assert!(path_and_query < "/c/world&foo=bar");
    assert!("/c/world&foo=bar" > path_and_query);
    assert!(path_and_query > "/a/world&foo=bar");
    assert!("/a/world&foo=bar" < path_and_query);
}

//#[test]
pub fn compares_with_a_string() {
    let path_and_query: PathAndQuery = "/b/world&foo=bar".parse().unwrap();
    assert!(path_and_query < "/c/world&foo=bar".to_string());
    assert!("/c/world&foo=bar".to_string() > path_and_query);
    assert!(path_and_query > "/a/world&foo=bar".to_string());
    assert!("/a/world&foo=bar".to_string() < path_and_query);
}

//#[test]
pub fn ignores_valid_percent_encodings() {
    assert_eq!("/a%20b", pq("/a%20b?r=1").path());
    assert_eq!("qr=%31", pq("/a/b?qr=%31").query().unwrap());
}

//#[test]
pub fn ignores_invalid_percent_encodings() {
    assert_eq!("/a%%b", pq("/a%%b?r=1").path());
    assert_eq!("/aaa%", pq("/aaa%").path());
    assert_eq!("/aaa%", pq("/aaa%?r=1").path());
    assert_eq!("/aa%2", pq("/aa%2").path());
    assert_eq!("/aa%2", pq("/aa%2?r=1").path());
    assert_eq!("qr=%3", pq("/a/b?qr=%3").query().unwrap());
}

fn pq(s: &str) -> PathAndQuery {
    s.parse().expect(&format!("parsing {}", s))
}
