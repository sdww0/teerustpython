use rand;
//extern crate openssl;

use std::prelude::v1::*;

use sha1::Sha1;

//#[test]
pub fn test_simple() {
    let mut m = Sha1::new();

    let tests = [
        ("The quick brown fox jumps over the lazy dog",
         "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"),
        ("The quick brown fox jumps over the lazy cog",
         "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3"),
        ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        ("testing\n", "9801739daae44ec5293d4e1f53d3f4d2d426d91c"),
        ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
         "025ecbd5d70f8fb3c5457cd96bab13fda305dc59"),
        ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
         "4300320394f7ee239bcdce7d3b8bcee173a0cd5c"),
        ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
         "cef734ba81a024479e09eb5a75b6ddae62e6abf1"),
    ];

    for &(s, ref h) in tests.iter() {
        let data = s.as_bytes();

        m.reset();
        m.update(data);
        let hh = m.digest().to_string();

        assert_eq!(hh.len(), h.len());
        assert_eq!(hh, *h);
    }
}

//#[test]
pub fn test_shortcuts() {
    let s = Sha1::from("The quick brown fox jumps over the lazy dog");
    assert_eq!(s.digest().to_string(), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");

    let s = Sha1::from(&b"The quick brown fox jumps over the lazy dog"[..]);
    assert_eq!(s.digest().to_string(), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");

    #[cfg(feature="std")] {
        let s = Sha1::from("The quick brown fox jumps over the lazy dog");
        assert_eq!(s.hexdigest(), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }
}

//#[test]
pub fn test_multiple_updates() {
    let mut m = Sha1::new();

    m.reset();
    m.update("The quick brown ".as_bytes());
    m.update("fox jumps over ".as_bytes());
    m.update("the lazy dog".as_bytes());
    let hh = m.digest().to_string();


    let h = "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12";
    assert_eq!(hh.len(), h.len());
    assert_eq!(hh, &*h);
}

//#[test]
pub fn test_sha1_loop() {
    let mut m = Sha1::new();
    let s = "The quick brown fox jumps over the lazy dog.";
    let n = 1000u64;

    for _ in 0..3 {
        m.reset();
        for _ in 0..n {
            m.update(s.as_bytes());
        }
        assert_eq!(m.digest().to_string(),
                   "7ca27655f67fceaa78ed2e645a81c7f1d6e249d2");
    }
}

//#[test]
pub fn spray_and_pray() {
    use rand::Rng;
    use rand::RngCore;
    use sgx_tcrypto::SgxSha1Handle;

    let mut rng = rand::thread_rng();
    let mut m = Sha1::new();
    let mut bytes = [0; 512];

    for _i in 0..20 {
//        let ty = openssl::hash::MessageDigest::sha1();
//        let mut r = openssl::hash::Hasher::new(ty).unwrap();
        let sgx_sha1 = SgxSha1Handle::new();
        sgx_sha1.init().unwrap();
        m.reset();
        for _ in 0..50 {
            let len = rng.gen::<usize>() % (bytes.len() - 1) + 1;
            rng.fill_bytes(&mut bytes[..len]);
            m.update(&bytes[..len]);
            sgx_sha1.update_slice(&bytes[..len]).unwrap();
 //           r.update(&bytes[..len]).unwrap();
        }
//        assert_eq!(r.finish().unwrap().as_ref(), &m.digest().bytes());
        assert_eq!(sgx_sha1.get_hash().unwrap().as_ref(), &m.digest().bytes());
    }
}

//#[test]
//#[cfg(feature="std")]
#[allow(deprecated)]
pub fn test_parse() {
    use sha1::Digest;
    use std::error::Error;
    let y: Digest = "2ef7bde608ce5404e97d5f042f95f89f1c232871".parse().unwrap();
    assert_eq!(y.to_string(), "2ef7bde608ce5404e97d5f042f95f89f1c232871");
    assert!("asdfasdf".parse::<Digest>().is_err());
    assert_eq!("asdfasdf".parse::<Digest>()
        .map_err(|x| x.description().to_string()).unwrap_err(), "not a valid sha1 hash");
}

pub mod serde_tests {
    use serde_json;
    use std::prelude::v1::*;

    use sha1::{Sha1, Digest};

    //#[test]
    pub fn test_to_json() {
        let mut s = Sha1::new();
        s.update(b"Hello World!");
        let x = s.digest();
        let y = serde_json::to_vec(&x).unwrap();
        assert_eq!(y, &b"\"2ef7bde608ce5404e97d5f042f95f89f1c232871\""[..]);
    }

    //#[test]
    pub fn test_from_json() {
        let y: Digest = serde_json::from_str("\"2ef7bde608ce5404e97d5f042f95f89f1c232871\"").unwrap();
        assert_eq!(y.to_string(), "2ef7bde608ce5404e97d5f042f95f89f1c232871");
    }
}
