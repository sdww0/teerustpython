use std::prelude::v1::*;
use gzip_header::*;
use std::io::Cursor;

fn roundtrip_inner(use_crc: bool) {
    const COMMENT: &'static [u8] = b"Comment";
    const FILENAME: &'static [u8] = b"Filename";
    const MTIME: u32 = 12345;
    const OS: FileSystemType = FileSystemType::NTFS;
    const XFL: ExtraFlags = ExtraFlags::FastestCompression;

    let header = GzBuilder::new()
        .comment(COMMENT)
        .filename(FILENAME)
        .mtime(MTIME)
        .os(OS)
        .xfl(ExtraFlags::FastestCompression)
        .into_header_inner(use_crc);

    let mut reader = Cursor::new(header.clone());

    let header_read = read_gz_header(&mut reader).unwrap();

    assert_eq!(header_read.comment().unwrap(), COMMENT);
    assert_eq!(header_read.filename().unwrap(), FILENAME);
    assert_eq!(header_read.mtime(), MTIME);
    assert_eq!(header_read.os(), OS.as_u8());
    assert_eq!(header_read.xfl(), XFL.as_u8());
}

//#[test]
pub fn roundtrip() {
    roundtrip_inner(false);

}

//#[test]
pub fn roundtrip_with_crc() {
    roundtrip_inner(true);
}

//#[test]
pub fn filesystem_enum() {
    for n in 0..20 {
        assert_eq!(n, FileSystemType::from_u8(n).as_u8());
    }

    for n in 20..(u8::max_value() as u16) + 1 {
        assert_eq!(FileSystemType::from_u8(n as u8), FileSystemType::Unknown);
    }
}
