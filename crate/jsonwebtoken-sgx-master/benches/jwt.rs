#![feature(test)]
extern crate test;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

#[bench]
fn bench_encode(b: &mut test::Bencher) {
    let claim = Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned() };
    let key = EncodingKey::from_secret("secret".as_ref());

    b.iter(|| encode(&Header::default(), &claim, &key));
}

#[bench]
fn bench_decode(b: &mut test::Bencher) {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ";
    let key = DecodingKey::from_secret("secret".as_ref());

    b.iter(|| decode::<Claims>(token, &key, &Validation::default()));
}
