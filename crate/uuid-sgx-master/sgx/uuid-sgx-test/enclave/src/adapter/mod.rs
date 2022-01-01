use std::prelude::v1::*;
use uuid::*;

pub mod compact;

//#[test]
pub fn hyphenated_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_hyphenated().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::Hyphenated::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
pub fn hyphenated_ref_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_hyphenated().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::HyphenatedRef::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
pub fn simple_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_simple().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::Simple::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
pub fn simple_ref_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_simple().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::SimpleRef::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
pub fn urn_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_urn().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::Urn::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
pub fn urn_ref_trailing() {
    let mut buf = [b'x'; 100];
    let len = Uuid::nil().to_urn().encode_lower(&mut buf).len();
    assert_eq!(len, uuid::adapter::UrnRef::LENGTH);
    assert!(buf[len..].iter().all(|x| *x == b'x'));
}

//#[test]
//#[should_panic]
pub fn hyphenated_too_small() {
    Uuid::nil().to_hyphenated().encode_lower(&mut [0; 35]);
}

//#[test]
//#[should_panic]
pub fn hyphenated_ref_too_small() {
    Uuid::nil().to_hyphenated_ref().encode_lower(&mut [0; 35]);
}

//#[test]
//#[should_panic]
pub fn simple_too_small() {
    Uuid::nil().to_simple().encode_lower(&mut [0; 31]);
}
//#[test]
//#[should_panic]
pub fn simple_ref_too_small() {
    Uuid::nil().to_simple_ref().encode_lower(&mut [0; 31]);
}
//#[test]
//#[should_panic]
pub fn urn_too_small() {
    Uuid::nil().to_urn().encode_lower(&mut [0; 44]);
}
//#[test]
//#[should_panic]
pub fn urn_ref_too_small() {
    Uuid::nil().to_urn_ref().encode_lower(&mut [0; 44]);
}
