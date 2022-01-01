use std::prelude::v1::*;
use inflate::{DeflateDecoder};
use std::io::Read;

//#[test]
pub fn deflate_reader() {
    const TEST_STRING: &'static str = "Hello, world";
    let encoded = vec![243, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 1, 0];
    let mut decoder = DeflateDecoder::new(&encoded[..]);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), TEST_STRING);
    assert_eq!(decoder.total_in(), encoded.len() as u64);
    assert_eq!(decoder.total_out(), TEST_STRING.len() as u64);
}

//#[test]
pub fn zlib_reader() {
    const TEST_STRING: &'static str = "Hello, zlib!";
    let encoded = vec![120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201,
                   76, 82, 4, 0, 27, 101, 4, 19];
    let mut decoder = DeflateDecoder::from_zlib(&encoded[..]);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), TEST_STRING);
    assert_eq!(decoder.total_in(), encoded.len() as u64);
    assert_eq!(decoder.total_out(), TEST_STRING.len() as u64);
}
