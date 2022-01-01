use sgx_rand::Rng;
use std::io;
use std::prelude::v1::*;

use adler32::{adler32, RollingAdler32};

const BASE: u32 = 65521;

fn adler32_slow<R: io::Read>(reader: R) -> io::Result<u32> {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for byte in reader.bytes() {
        let byte = byte? as u32;
        a = (a + byte) % BASE;
        b = (b + a) % BASE;
    }

    Ok((b << 16) | a)
}

//#[test]
pub fn testvectors() {
    fn do_test(v: u32, bytes: &[u8]) {
        let mut hash = RollingAdler32::new();
        hash.update_buffer(&bytes);
        assert_eq!(hash.hash(), v);

        let r = io::Cursor::new(bytes);
        assert_eq!(adler32(r).unwrap(), v);
    }
    do_test(0x00000001, b"");
    do_test(0x00620062, b"a");
    do_test(0x024d0127, b"abc");
    do_test(0x29750586, b"message digest");
    do_test(0x90860b20, b"abcdefghijklmnopqrstuvwxyz");
    do_test(0x8adb150c, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789");
    do_test(0x97b61069, b"1234567890123456789012345678901234567890\
                          1234567890123456789012345678901234567890");
    do_test(0xD6251498, &[255; 64000]);
}

//#[test]
pub fn compare() {
    let mut rng = sgx_rand::thread_rng();
    let mut data = vec![0u8; 5589];
    for size in [0, 1, 3, 4, 5, 31, 32, 33, 67,
                 5550, 5552, 5553, 5568, 5584, 5589].iter().cloned() {
        rng.fill_bytes(&mut data[..size]);
        let r1 = io::Cursor::new(&data[..size]);
        let r2 = r1.clone();
        if adler32_slow(r1).unwrap() != adler32(r2).unwrap() {
            panic!("Comparison failed, size={}", size);
        }
    }
}

//#[test]
pub fn rolling() {
    assert_eq!(RollingAdler32::from_value(0x01020304).hash(), 0x01020304);

    fn do_test(a: &[u8], b: &[u8]) {
        let mut total = Vec::with_capacity(a.len() + b.len());
        total.extend(a);
        total.extend(b);
        let mut h = RollingAdler32::from_buffer(&total[..(b.len())]);
        for i in 0..(a.len()) {
            h.remove(b.len(), a[i]);
            h.update(total[b.len() + i]);
        }
        assert_eq!(h.hash(), adler32(b).unwrap());
    }
    do_test(b"a", b"b");
    do_test(b"", b"this a test");
    do_test(b"th", b"is a test");
    do_test(b"this a ", b"test");
}

//#[test]
pub fn long_window_remove() {
    let mut hash = RollingAdler32::new();
    let w = 65536;
    assert!(w as u32 > BASE);

    let mut bytes = vec![0; w*3];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = i as u8;
    }

    for (i, b) in bytes.iter().enumerate() {
        if i >= w {
            hash.remove(w, bytes[i - w]);
        }
        hash.update(*b);
        if i > 0 && i % w == 0 {
            assert_eq!(hash.hash(), 0x433a8772);
        }
    }
    assert_eq!(hash.hash(), 0xbbba8772);
}
