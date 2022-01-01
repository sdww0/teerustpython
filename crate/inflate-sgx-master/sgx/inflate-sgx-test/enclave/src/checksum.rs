use inflate::adler32_from_bytes;

pub fn adler32() {
    let bytes = [0x00, 0x00, 0x01, 0x0b];
    assert_eq!(adler32_from_bytes(&bytes), 267);
}
