use std::untrusted::time::SystemTimeEx;
use std::time::{Duration, SystemTime};
use std::prelude::v1::*;

use chrono::Utc;
use jsonwebtoken::{
    crypto::{sign, verify},
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

pub fn utc_now_timestamp() -> i64 {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    use std::untrusted::time::SystemTimeEx;
    use chrono::naive::NaiveDateTime;
    use chrono::DateTime;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now = now.as_secs() as i64;

    now
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

pub fn round_trip_sign_verification() {
    let privkey = include_bytes!("private_ecdsa_key.pk8");
    let pubkey = include_bytes!("public_ecdsa_key.pk8");

    let encrypted =
        sign("hello world", &EncodingKey::from_ec_der(privkey), Algorithm::ES256).unwrap();
    let is_valid =
        verify(&encrypted, "hello world", &DecodingKey::from_ec_der(pubkey), Algorithm::ES256)
            .unwrap();
    assert!(is_valid);
}

pub fn round_trip_claim() {
    let privkey_pem = include_bytes!("private_ecdsa_key.pem");
    let pubkey_pem = include_bytes!("public_ecdsa_key.pem");
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string(),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 + 10000
    };
    let token = encode(
        &Header::new(Algorithm::ES256),
        &my_claims,
        &EncodingKey::from_ec_pem(privkey_pem).unwrap(),
    )
    .unwrap();
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_ec_pem(pubkey_pem).unwrap(),
        &Validation::new(Algorithm::ES256),
    )
    .unwrap();
    assert_eq!(my_claims, token_data.claims);
}
