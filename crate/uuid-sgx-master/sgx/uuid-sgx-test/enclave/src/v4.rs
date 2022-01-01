use uuid::*;
use std::prelude::v1::*;

//#[test]
pub fn test_new() {
    let uuid = Uuid::new_v4();

    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
}

//#[test]
pub fn test_get_version() {
    let uuid = Uuid::new_v4();

    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_version_num(), 4)
}
