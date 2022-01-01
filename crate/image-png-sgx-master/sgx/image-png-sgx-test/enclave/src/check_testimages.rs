use std::prelude::v1::*;
use std::collections::HashMap;
use std::untrusted::fs::File;
use std::path::{PathBuf};
use std::io::BufReader;
use std::io::prelude::*;

use crc32fast::Hasher as Crc32;

const BASE_PATH: [&'static str; 2] = [".", "tests"];



fn process_images<F>(func: F)
where F: Fn(PathBuf) -> Result<u32, png::DecodingError> {
let tests = &[
"tests/pngsuite/basi0g16.png",
"tests/pngsuite/g07n3p04.png",
"tests/pngsuite/s07n3p02.png",
"tests/pngsuite/ct0n0g04.png",
"tests/pngsuite/tp0n2c08.png",
"tests/pngsuite/basi4a16.png",
"tests/pngsuite/cm7n0g04.png",
"tests/pngsuite/g03n2c08.png",
"tests/pngsuite/tbgn3p08.png",
"tests/pngsuite/oi4n2c16.png",
"tests/pngsuite/cdfn2c08.png",
"tests/pngsuite/cs3n2c16.png",
"tests/pngsuite/tbbn3p08.png",
"tests/pngsuite/xhdn0g08.png",
"tests/pngsuite/s07i3p02.png",
"tests/pngsuite/ps1n2c16.png",
"tests/pngsuite/basn0g04.png",
"tests/pngsuite/basi3p04.png",
"tests/pngsuite/basn3p04.png",
"tests/pngsuite/xs1n0g01.png",
"tests/pngsuite/ccwn2c08.png",
"tests/pngsuite/g04n2c08.png",
"tests/pngsuite/s34i3p04.png",
"tests/pngsuite/g25n2c08.png",
"tests/pngsuite/basi2c16.png",
"tests/pngsuite/pp0n2c16.png",
"tests/pngsuite/bggn4a16.png",
"tests/pngsuite/xs4n0g01.png",
"tests/pngsuite/cs5n2c08.png",
"tests/pngsuite/basn2c08.png",
"tests/pngsuite/cs8n3p08.png",
"tests/pngsuite/ch1n3p04.png",
"tests/pngsuite/tbbn0g04.png",
"tests/pngsuite/g04n0g16.png",
"tests/pngsuite/g03n0g16.png",
"tests/pngsuite/oi4n0g16.png",
"tests/pngsuite/z03n2c08.png",
"tests/pngsuite/f01n2c08.png",
"tests/pngsuite/basi0g02.png",
"tests/pngsuite/basi6a08.png",
"tests/pngsuite/xd3n2c08.png",
"tests/pngsuite/PngSuite.png",
"tests/pngsuite/tm3n3p02.png",
"tests/pngsuite/basn4a16.png",
"tests/pngsuite/xcsn0g01.png",
"tests/pngsuite/basn3p08.png",
"tests/pngsuite/s39i3p04.png",
"tests/pngsuite/ctgn0g04.png",
"tests/pngsuite/ps1n0g08.png",
"tests/pngsuite/tbwn3p08.png",
"tests/pngsuite/f02n2c08.png",
"tests/pngsuite/g25n3p04.png",
"tests/pngsuite/cdsn2c08.png",
"tests/pngsuite/pp0n6a08.png",
"tests/pngsuite/cten0g04.png",
"tests/pngsuite/z06n2c08.png",
"tests/pngsuite/basi3p02.png",
"tests/pngsuite/ps2n2c16.png",
"tests/pngsuite/s35i3p04.png",
"tests/pngsuite/cm9n0g04.png",
"tests/pngsuite/s01n3p01.png",
"tests/pngsuite/s09i3p02.png",
"tests/pngsuite/g05n2c08.png",
"tests/pngsuite/s04n3p01.png",
"tests/pngsuite/xc9n2c08.png",
"tests/pngsuite/cdun2c08.png",
"tests/pngsuite/ct1n0g04.png",
"tests/pngsuite/g05n0g16.png",
"tests/pngsuite/cs5n3p08.png",
"tests/pngsuite/s03n3p01.png",
"tests/pngsuite/s01i3p01.png",
"tests/pngsuite/oi9n2c16.png",
"tests/pngsuite/basn3p01.png",
"tests/pngsuite/f03n2c08.png",
"tests/pngsuite/xd9n2c08.png",
"tests/pngsuite/basi3p01.png",
"tests/pngsuite/basi0g04.png",
"tests/pngsuite/f04n0g08.png",
"tests/pngsuite/oi9n0g16.png",
"tests/pngsuite/bgai4a08.png",
"tests/pngsuite/s02n3p01.png",
"tests/pngsuite/s33n3p04.png",
"tests/pngsuite/g25n0g16.png",
"tests/pngsuite/basn6a16.png",
"tests/pngsuite/s38n3p04.png",
"tests/pngsuite/tbwn0g16.png",
"tests/pngsuite/g10n0g16.png",
"tests/pngsuite/f01n0g08.png",
"tests/pngsuite/tp0n3p08.png",
"tests/pngsuite/s06n3p02.png",
"tests/pngsuite/basi2c08.png",
"tests/pngsuite/ctjn0g04.png",
"tests/pngsuite/f03n0g08.png",
"tests/pngsuite/bgan6a08.png",
"tests/pngsuite/s38i3p04.png",
"tests/pngsuite/s34n3p04.png",
"tests/pngsuite/bgan6a16.png",
"tests/pngsuite/ch2n3p08.png",
"tests/pngsuite/xdtn0g01.png",
"tests/pngsuite/s06i3p02.png",
"tests/pngsuite/oi2n0g16.png",
"tests/pngsuite/basn0g16.png",
"tests/pngsuite/ctfn0g04.png",
"tests/pngsuite/basi4a08.png",
"tests/pngsuite/f00n2c08.png",
"tests/pngsuite/ps2n0g08.png",
"tests/pngsuite/tbgn2c16.png",
"tests/pngsuite/s08n3p02.png",
"tests/pngsuite/s02i3p01.png",
"tests/pngsuite/z09n2c08.png",
"tests/pngsuite/g07n2c08.png",
"tests/pngsuite/basi0g08.png",
"tests/pngsuite/g10n2c08.png",
"tests/pngsuite/s32n3p04.png",
"tests/pngsuite/xs2n0g01.png",
"tests/pngsuite/s35n3p04.png",
"tests/pngsuite/xd0n2c08.png",
"tests/pngsuite/g04n3p04.png",
"tests/pngsuite/xcrn0g04.png",
"tests/pngsuite/f99n0g04.png",
"tests/pngsuite/s05i3p02.png",
"tests/pngsuite/s36i3p04.png",
"tests/pngsuite/basn3p02.png",
"tests/pngsuite/basn2c16.png",
"tests/pngsuite/oi1n0g16.png",
"tests/pngsuite/g05n3p04.png",
"tests/pngsuite/ctzn0g04.png",
"tests/pngsuite/s37n3p04.png",
"tests/pngsuite/f02n0g08.png",
"tests/pngsuite/basn6a08.png",
"tests/pngsuite/tbbn2c16.png",
"tests/pngsuite/f04n2c08.png",
"tests/pngsuite/cs3n3p08.png",
"tests/pngsuite/s37i3p04.png",
"tests/pngsuite/g10n3p04.png",
"tests/pngsuite/cdhn2c08.png",
"tests/pngsuite/g07n0g16.png",
"tests/pngsuite/s40n3p04.png",
"tests/pngsuite/ccwn3p08.png",
"tests/pngsuite/s08i3p02.png",
"tests/pngsuite/xc1n0g08.png",
"tests/pngsuite/cs8n2c08.png",
"tests/pngsuite/s03i3p01.png",
"tests/pngsuite/basn0g02.png",
"tests/pngsuite/s09n3p02.png",
"tests/pngsuite/z00n2c08.png",
"tests/pngsuite/basn4a08.png",
"tests/pngsuite/tp1n3p08.png",
"tests/pngsuite/cm0n0g04.png",
"tests/pngsuite/s05n3p02.png",
"tests/pngsuite/basn0g01.png",
"tests/pngsuite/bgwn6a08.png",
"tests/pngsuite/cthn0g04.png",
"tests/pngsuite/bgyn6a16.png",
"tests/pngsuite/s39n3p04.png",
"tests/pngsuite/basi6a16.png",
"tests/pngsuite/bgbn4a08.png",
"tests/pngsuite/s33i3p04.png",
"tests/pngsuite/basn0g08.png",
"tests/pngsuite/tbyn3p08.png",
"tests/pngsuite/g03n3p04.png",
"tests/pngsuite/oi1n2c16.png",
"tests/pngsuite/tp0n0g08.png",
"tests/pngsuite/xs7n0g01.png",
"tests/pngsuite/xlfn0g04.png",
"tests/pngsuite/oi2n2c16.png",
"tests/pngsuite/s40i3p04.png",
"tests/pngsuite/s04i3p01.png",
"tests/pngsuite/basi0g01.png",
"tests/pngsuite/s32i3p04.png",
"tests/pngsuite/s36n3p04.png",
"tests/pngsuite/bgai4a16.png",
"tests/pngsuite/basi3p08.png",
"tests/pngsuite/tbrn2c08.png",
"tests/pngsuite/f00n0g08.png",
"tests/pngsuite-extra/basi3p01_2.png",
"tests/pngsuite-extra/basi3p02_2.png",
"tests/bugfixes/x_unexpected_eof.png",
"tests/bugfixes/issue#403.png",
"tests/bugfixes/x_ihdr_missing.png",
"tests/bugfixes/acid2.png",
"tests/bugfixes/invalid_palette_index.png",
];
    //let tests = &[
    //    //"tests/sample_big.gif",
    //    "tests/samples/2x2.gif",
    //    "tests/samples/alpha_gif_a.gif",
    //    "tests/samples/anim-gr.gif",
    //    "tests/samples/beacon.gif",
    //    "tests/samples/interlaced.gif",
    //    "tests/samples/moon_impact.gif",
    //    "tests/samples/sample_1.gif",
    //];
    let base: PathBuf = BASE_PATH.iter().collect();
    let mut results = HashMap::new();
    let mut expected_failures = 0;
    //for suite in test_suites {
    //    let mut path = base.clone();
    //    path.push(suite);
    //    path.push("*.png");

    //    let pattern = &*format!("{}", path.display());
        for path in tests.iter() {
            print!("{:?}: ", path.clone());
            let path:std::path::PathBuf = path.into();
            match func(path.clone()) {
                Ok(crc) => {
                    results.insert(format!("{}", path.to_str().unwrap()), format!("{}", crc));
                    println!("{}", crc)
                },
                Err(_) if path.file_name().unwrap().to_str().unwrap().starts_with("x") => {
                    expected_failures += 1;
                    println!("Expected failure")
                },
                err => panic!("{:?}", err)
            }
        }
    //}
    let mut path = base.clone();
    path.push("results.txt");
    let mut ref_results: HashMap<String, String> = HashMap::new();
    let mut failures = 0;
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(": ").collect();
        if parts[1] == "Expected failure" {
            failures += 1;
        } else {
            ref_results.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    assert_eq!(expected_failures, failures);
    for (path, crc) in results.iter() {
        assert_eq!(
            ref_results.get(path).expect(&format!("reference for {} is missing", path)), 
            crc
        )
    }
}

//#[test]
pub fn render_images() {
    process_images(|path| {
        let decoder = png::Decoder::new(File::open(path)?);
        let (info, mut reader) = decoder.read_info()?;
        let mut img_data = vec![0; info.buffer_size()];
        reader.next_frame(&mut img_data)?;
        // First sanity check:
        assert_eq!(
            img_data.len(), 
            info.width as usize
            * info.height as usize
            * info.color_type.samples()
            * info.bit_depth as usize/8
        );
        let mut crc = Crc32::new();
        crc.update(&img_data);
        Ok(crc.finalize())
    })
}

