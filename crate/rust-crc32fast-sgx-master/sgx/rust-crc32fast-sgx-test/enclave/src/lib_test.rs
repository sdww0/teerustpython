use crc32fast::Hasher;
use std::prelude::v1::*;

quickcheck! {
    fn combine(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> bool {
        let mut hash_a = Hasher::new();
        hash_a.update(&bytes_1);
        hash_a.update(&bytes_2);
        let mut hash_b = Hasher::new();
        hash_b.update(&bytes_2);
        let mut hash_c = Hasher::new();
        hash_c.update(&bytes_1);
        hash_c.combine(&hash_b);

        hash_a.finalize() == hash_c.finalize()
    }
}
