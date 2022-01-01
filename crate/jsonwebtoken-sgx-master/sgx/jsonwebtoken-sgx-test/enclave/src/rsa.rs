use std::untrusted::time::SystemTimeEx;
use std::time::{Duration, SystemTime};
use std::prelude::v1::*;
use chrono::Utc;
use jsonwebtoken::{
    crypto::{sign, verify},
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

const RSA_ALGORITHMS: &[Algorithm] = &[
    Algorithm::RS256,
    Algorithm::RS384,
    Algorithm::RS512,
    Algorithm::PS256,
    Algorithm::PS384,
    Algorithm::PS512,
];

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
    let privkey_pem = include_bytes!("private_rsa_key_pkcs1.pem");
    let pubkey_pem = include_bytes!("public_rsa_key_pkcs1.pem");

    for &alg in RSA_ALGORITHMS {
        let encrypted =
            sign("hello world", &EncodingKey::from_rsa_pem(privkey_pem).unwrap(), alg).unwrap();
        let is_valid =
            verify(&encrypted, "hello world", &DecodingKey::from_rsa_pem(pubkey_pem).unwrap(), alg)
                .unwrap();
        assert!(is_valid);
    }
}

pub fn round_trip_claim() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string(),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 + 10000,
    };
    let privkey_pem = include_bytes!("private_rsa_key_pkcs1.pem");
    let pubkey_pem = include_bytes!("public_rsa_key_pkcs1.pem");

    for &alg in RSA_ALGORITHMS {
        let token =
            encode(&Header::new(alg), &my_claims, &EncodingKey::from_rsa_pem(privkey_pem).unwrap())
                .unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(pubkey_pem).unwrap(),
            &Validation::new(alg),
        )
        .unwrap();
        assert_eq!(my_claims, token_data.claims);
        assert!(token_data.header.kid.is_none());
    }
}
