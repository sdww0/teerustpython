use std::prelude::v1::*;
use gif::*;

pub fn round_trip() {
    use std::io::prelude::*;
    use std::untrusted::fs::File;
    let mut data = vec![];
    File::open("tests/samples/sample_1.gif").unwrap().read_to_end(&mut data).unwrap();
    let mut decoder = Decoder::new(&*data).read_info().unwrap();
    let palette: Vec<u8> = decoder.palette().unwrap().into();
    let frame = decoder.read_next_frame().unwrap().unwrap();
    let mut data2 = vec![];
    {
        let mut encoder = Encoder::new(&mut data2, frame.width, frame.height, &palette).unwrap();
        encoder.write_frame(frame).unwrap();
    }
    assert_eq!(&data[..], &data2[..])
}
