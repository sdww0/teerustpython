use std::prelude::v1::*;
//#[test]
pub fn slow() {
    assert_eq!(crc32fast::baseline::update_slow(0, b""), 0);

    // test vectors from the iPXE project (input and output are bitwise negated)
    assert_eq!(crc32fast::baseline::update_slow(!0x12345678, b""), !0x12345678);
    assert_eq!(crc32fast::baseline::update_slow(!0xffffffff, b"hello world"), !0xf2b5ee7a);
    assert_eq!(crc32fast::baseline::update_slow(!0xffffffff, b"hello"), !0xc9ef5979);
    assert_eq!(crc32fast::baseline::update_slow(!0xc9ef5979, b" world"), !0xf2b5ee7a);

    // Some vectors found on Rosetta code
    assert_eq!(crc32fast::baseline::update_slow(0, b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"), 0x190A55AD);
    assert_eq!(crc32fast::baseline::update_slow(0, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"), 0xFF6CAB0B);
    assert_eq!(crc32fast::baseline::update_slow(0, b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F"), 0x91267E8A);
}

quickcheck! {
    fn fast_16_is_the_same_as_slow(crc: u32, bytes: Vec<u8>) -> bool {
        crc32fast::baseline::update_fast_16(crc, &bytes) == crc32fast::baseline::update_slow(crc, &bytes)
    }
}
