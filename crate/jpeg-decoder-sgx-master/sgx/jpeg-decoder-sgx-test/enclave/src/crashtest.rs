use std::untrusted::fs::File;
use std::io::BufReader;
use std::path::Path;

use common;

//#[test]
pub fn crashtest() {
    let files = common::crash_test_files(&Path::new("tests").join("crashtest").join("images"));
    //println!("files = {:?}", files);

    for path in &files {
        let file = File::open(path).unwrap();
        let mut decoder = jpeg::Decoder::new(BufReader::new(file));
        let _ = decoder.decode();
    }
}
