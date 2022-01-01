//#[test]
pub fn compute() {
    let inputs = [
        "",
        "a",
        "abc",
        "message digest",
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "0123456789012345678901234567890123456789012345678901234567890123",
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    ];
    let outputs = [
        "d41d8cd98f00b204e9800998ecf8427e",
        "0cc175b9c0f1b6a831c399e269772661",
        "900150983cd24fb0d6963f7d28e17f72",
        "f96b697d7cb7938d525a2f31aaf161d0",
        "c3fcd3d76192e4007dfb496cca67e13b",
        "d174ab98d277d9f5a5611c2c9f419d9f",
        "7f7bfd348709deeaace19e3f535f8c54",
        "57edf4a22be3c955ac49da2e2107b67a",
    ];
    for (input, &output) in inputs.iter().zip(outputs.iter()) {
        assert_eq!(format!("{:x}", md5::compute(input)), output);
    }
}

//#[test]
pub fn index() {
    let mut digest = md5::compute(b"abc");
    assert_eq!(digest[0], 0x90);
    assert_eq!(&digest[0], &0x90);
    assert_eq!(&mut digest[0], &mut 0x90);
}

//#[test]
pub fn overflow_count() {
    use std::io::prelude::Write;
    let data = vec![0; 8 * 1024 * 1024];
    let mut context = md5::Context::new();
    for _ in 0..64 {
        context.write(&data).unwrap();
    }
    assert_eq!(
        format!("{:x}", context.compute()),
        "aa559b4e3523a6c931f08f4df52d58f2"
    );
}

//#[test]
//#[cfg(target_pointer_width = "64")]
pub fn overflow_length() {
    use std::io::prelude::Write;
    use std::u32::MAX;
    let data = vec![0; MAX as usize + 1];
    let mut context = md5::Context::new();
    context.write(&data).unwrap();
    assert_eq!(
        format!("{:x}", context.compute()),
        "c9a5a6878d97b48cc965c1e41859f034"
    );
}
