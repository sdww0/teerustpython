use std::prelude::v1::*;
use hex::{encode, decode, FromHex, FromHexError};

//#[test]
pub fn test_encode() {
    assert_eq!(encode("foobar"), "666f6f626172");
}

//#[test]
pub fn test_decode() {
    assert_eq!(decode("666f6f626172"), Ok("foobar".to_owned().into_bytes()));
}

//#[test]
pub fn test_from_hex_okay_str() {
    assert_eq!(
        Vec::from_hex("666f6f626172").unwrap(),
        b"foobar"
    );
    assert_eq!(
        Vec::from_hex("666F6F626172").unwrap(),
        b"foobar"
    );
}

//[test]
pub fn test_from_hex_okay_bytes() {
    assert_eq!(
        Vec::from_hex(b"666f6f626172").unwrap(),
        b"foobar"
    );
    assert_eq!(
        Vec::from_hex(b"666F6F626172").unwrap(),
        b"foobar"
    );
}

//#[test]
pub fn test_invalid_length() {
    assert_eq!(
        Vec::from_hex("1").unwrap_err(),
        FromHexError::OddLength
    );
    assert_eq!(
        Vec::from_hex("666f6f6261721").unwrap_err(),
        FromHexError::OddLength
    );
}

//#[test]
pub fn test_invalid_char() {
    assert_eq!(
        Vec::from_hex("66ag").unwrap_err(),
        FromHexError::InvalidHexCharacter {
            c: 'g',
            index: 3
        }
    );
}

//#[test]
pub fn test_empty() {
    assert_eq!(Vec::from_hex("").unwrap(), b"");
}

//#[test]
pub fn test_from_hex_whitespace() {
    assert_eq!(
        Vec::from_hex("666f 6f62617").unwrap_err(),
        FromHexError::InvalidHexCharacter {
            c: ' ',
            index: 4
        }
    );
}

//#[test]
pub fn test_from_hex_array() {
    assert_eq!(
        <[u8; 6] as FromHex>::from_hex("666f6f626172"),
        Ok([0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72])
    );

    assert_eq!(
        <[u8; 5] as FromHex>::from_hex("666f6f626172"),
        Err(FromHexError::InvalidStringLength)
    );
}
