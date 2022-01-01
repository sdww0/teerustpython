use gzip_header::Crc;
//#[test]
pub fn checksum_correct() {
    let mut c = Crc::new();
    c.update(b"abcdefg12345689\n");
    assert_eq!(c.sum(), 0x141ddb83);
    assert_eq!(c.amt_as_u32(), 16);
}
