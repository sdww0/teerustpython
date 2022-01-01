use std::prelude::v1::*;

//#[test]
pub fn round_trip() {
    use lzw::{LsbWriter, LsbReader, Encoder, Decoder};

    let size = 8;
    let data = b"TOBEORNOTTOBEORTOBEORNOT";
    let mut compressed = vec![];
    {
        let mut enc = Encoder::new(LsbWriter::new(&mut compressed), size).unwrap();
        enc.encode_bytes(data).unwrap();
    }
    println!("{:?}", compressed);
    let mut dec = Decoder::new(LsbReader::new(), size);
    let mut compressed = &compressed[..];
    let mut data2 = vec![];
    while compressed.len() > 0 {
        let (start, bytes) = dec.decode_bytes(&compressed).unwrap();
        compressed = &compressed[start..];
        data2.extend(bytes.iter().map(|&i| i));
    }
    assert_eq!(data2, data)
}

use lzw::{BitReader, BitWriter, Bits};

//#[test]
pub fn reader_writer() {
    let data = [255, 20, 40, 120, 128];
    let mut offset = 0;
    let mut expanded_data = Vec::new();
    let mut reader = lzw::LsbReader::new();
    while let Bits::Some(consumed, b) = reader.read_bits(&data[offset..], 10) {
        offset += consumed;
        expanded_data.push(b)
    }
    let mut compressed_data = Vec::new();
    {
        let mut writer = lzw::LsbWriter::new(&mut compressed_data);
        for &datum in expanded_data.iter() {
            let _  = writer.write_bits(datum, 10);
        }
    }
    assert_eq!(&data[..], &compressed_data[..])
}
