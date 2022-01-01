//#[test]
pub fn inflate_bytes_with_zlib() {
    use inflate::inflate_bytes_zlib;
    use std::str::from_utf8;

    let encoded = [120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201,
                   76, 82, 4, 0, 27, 101, 4, 19];
    let decoded = inflate_bytes_zlib(&encoded).unwrap();
    assert!(from_utf8(&decoded).unwrap() == "Hello, zlib!");
}

//#[test]
pub fn inflate_bytes_with_zlib_checksum_fail() {
    use inflate::inflate_bytes_zlib;

    // The last 4 bytes are the checksum, we set them to 0 here to check that decoding fails
    // if the checksum is wrong.
    let encoded = [120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201,
                   76, 82, 4, 0, 0, 0, 0, 0];
    inflate_bytes_zlib(&encoded).unwrap_err();
}

//#[test]
pub fn inflate_bytes_with_zlib_trailing() {
    use inflate::inflate_bytes_zlib;
    use std::str::from_utf8;

    // The additional 4 bytes should be ignored.
    let encoded = [120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201,
                   76, 82, 4, 0, 27, 101, 4, 19, 0, 0, 0, 0];
    let decoded = inflate_bytes_zlib(&encoded).unwrap();
    assert!(from_utf8(&decoded).unwrap() == "Hello, zlib!");
}

