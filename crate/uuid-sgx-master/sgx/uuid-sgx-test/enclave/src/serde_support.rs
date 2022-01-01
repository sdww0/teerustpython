use serde_test;

use std::prelude::v1::*;
use uuid::prelude::*;

//#[test]
pub fn test_serialize_readable() {
    use serde_test::Configure;

    let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
    let u = Uuid::parse_str(uuid_str).unwrap();
    serde_test::assert_tokens(
        &u.readable(),
        &[serde_test::Token::Str(uuid_str)],
    );
}

//#[test]
pub fn test_serialize_compact() {
    use serde_test::Configure;

    let uuid_bytes = b"F9168C5E-CEB2-4F";
    let u = Uuid::from_slice(uuid_bytes).unwrap();
    serde_test::assert_tokens(
        &u.compact(),
        &[serde_test::Token::Bytes(uuid_bytes)],
    );
}
