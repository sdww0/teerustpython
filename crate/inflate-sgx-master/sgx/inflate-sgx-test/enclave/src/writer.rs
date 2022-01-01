use std::prelude::v1::*;
use inflate::InflateWriter;
use std::io::Write;

//#[test]
pub fn inflate_writer() {
   let encoded = [243, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 1, 0];
   let mut decoder = InflateWriter::new(Vec::new());
   decoder.write(&encoded).unwrap();
   let decoded = decoder.finish().unwrap();
   assert!(String::from_utf8(decoded).unwrap() == "Hello, world");
}

//#[test]
pub fn inflate_writer_from_zlib() {
   let encoded = [120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201, 76, 82, 4, 0, 27, 101, 4, 19];
   let mut decoder = InflateWriter::from_zlib(Vec::new());
   decoder.write(&encoded).unwrap();
   let decoded = decoder.finish().unwrap();
   assert!(String::from_utf8(decoded).unwrap() == "Hello, zlib!");
}
