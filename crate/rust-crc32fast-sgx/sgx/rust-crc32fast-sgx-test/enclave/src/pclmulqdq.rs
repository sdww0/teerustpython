use std::prelude::v1::*;

quickcheck! {
    fn check_against_baseline(init: u32, chunks: Vec<(Vec<u8>, usize)>) -> bool {
        let mut baseline = crc32fast::baseline::State::new(init);
        let mut pclmulqdq = crc32fast::specialized::State::new(init).expect("not supported");
        for (chunk, mut offset) in chunks {
            // simulate random alignments by offsetting the slice by up to 15 bytes
            offset &= 0xF;
            if chunk.len() <= offset {
                baseline.update(&chunk);
                pclmulqdq.update(&chunk);
            } else {
                baseline.update(&chunk[offset..]);
                pclmulqdq.update(&chunk[offset..]);
            }
        }
        pclmulqdq.finalize() == baseline.finalize()
    }
}
