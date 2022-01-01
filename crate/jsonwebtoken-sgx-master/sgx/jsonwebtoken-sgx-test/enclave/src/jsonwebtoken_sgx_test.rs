use std::untrusted::time::SystemTimeEx;
use std::time::{Duration, SystemTime};
use std::prelude::v1::*;
use chrono::Utc;
use std::str::FromStr;
use jsonwebtoken::{
    *,
    crypto::{sign, verify},
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


pub fn sign_hs256() {
    let result =
        sign("hello world", &EncodingKey::from_secret(b"secret"), Algorithm::HS256).unwrap();
    let expected = "c0zGLzKEFWj0VxWuufTXiRMk5tlI5MbGDAYhzaxIYjo";
    assert_eq!(result, expected);
}

pub fn verify_hs256() {
    let sig = "c0zGLzKEFWj0VxWuufTXiRMk5tlI5MbGDAYhzaxIYjo";
    let valid =
        verify(sig, "hello world", &DecodingKey::from_secret(b"secret"), Algorithm::HS256).unwrap();
    assert!(valid);
}

pub fn encode_with_custom_header() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string(),
        exp:  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 + 10000,
    };
    let mut header = Header::default();
    header.kid = Some("kid".to_string());
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(b"secret")).unwrap();
    let token_data =
        decode::<Claims>(&token, &DecodingKey::from_secret(b"secret"), &Validation::default())
            .unwrap();
    assert_eq!(my_claims, token_data.claims);
    assert_eq!("kid", token_data.header.kid.unwrap());
}

pub fn round_trip_claim() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string(),
        exp:  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 + 10000,
    };
    let token =
        encode(&Header::default(), &my_claims, &EncodingKey::from_secret(b"secret")).unwrap();
    let token_data =
        decode::<Claims>(&token, &DecodingKey::from_secret(b"secret"), &Validation::default())
            .unwrap();
    assert_eq!(my_claims, token_data.claims);
    assert!(token_data.header.kid.is_none());
}

pub fn decode_token() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjI1MzI1MjQ4OTF9.9r56oF7ZliOBlOAyiOFperTGxBtPykRQiWNFxhDCW98";
    let claims =
        decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &Validation::default());
    println!("{:?}", claims);
    claims.unwrap();
}

#[should_panic(expected = "InvalidToken")]
pub fn decode_token_missing_parts() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
    let claims =
        decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &Validation::default());
    claims.unwrap();
}


#[should_panic(expected = "InvalidSignature")]
pub fn decode_token_invalid_signature() {
    let token =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUifQ.wrong";
    let claims =
        decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &Validation::default());
    claims.unwrap();
}


#[should_panic(expected = "InvalidAlgorithm")]
pub fn decode_token_wrong_algorithm() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUifQ.I1BvFoHe94AFf09O6tDbcSB8-jp8w6xZqmyHIwPeSdY";
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"secret"),
        &Validation::new(Algorithm::RS512),
    );
    claims.unwrap();
}


pub fn decode_token_with_bytes_secret() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjI1MzI1MjQ4OTF9.Hm0yvKH25TavFPz7J_coST9lZFYH1hQo0tvhvImmaks";
    let claims =
        decode::<Claims>(token, &DecodingKey::from_secret(b"\x01\x02\x03"), &Validation::default());
    assert!(claims.is_ok());
}


pub fn decode_header_only() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb21wYW55IjoiMTIzNDU2Nzg5MCIsInN1YiI6IkpvaG4gRG9lIn0.S";
    let header = decode_header(token).unwrap();
    assert_eq!(header.alg, Algorithm::HS256);
    assert_eq!(header.typ, Some("JWT".to_string()));
}


pub fn dangerous_unsafe_decode_token() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjI1MzI1MjQ4OTF9.9r56oF7ZliOBlOAyiOFperTGxBtPykRQiWNFxhDCW98";
    let claims = dangerous_unsafe_decode::<Claims>(token);
    claims.unwrap();
}


#[should_panic(expected = "InvalidToken")]
pub fn dangerous_unsafe_decode_token_missing_parts() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
    let claims = dangerous_unsafe_decode::<Claims>(token);
    claims.unwrap();
}


pub fn dangerous_unsafe_decode_token_invalid_signature() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjI1MzI1MjQ4OTF9.wrong";
    let claims = dangerous_unsafe_decode::<Claims>(token);
    claims.unwrap();
}


pub fn dangerous_unsafe_decode_token_wrong_algorithm() {
    let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjI1MzI1MjQ4OTF9.fLxey-hxAKX5rNHHIx1_Ch0KmrbiuoakDVbsJjLWrx8fbjKjrPuWMYEJzTU3SBnYgnZokC-wqSdqckXUOunC-g";
    let claims = dangerous_unsafe_decode::<Claims>(token);
    claims.unwrap();
}

// https://github.com/Keats/jsonwebtoken/issues/51

pub fn does_validation_in_right_order() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string(),
        exp: utc_now_timestamp() + 10000,
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(b"secret")).unwrap();
    let v = Validation {
        leeway: 5,
        validate_exp: true,
        iss: Some("iss no check".to_string()),
        sub: Some("sub no check".to_string()),
        ..Validation::default()
    };
    let res = decode::<Claims>(&token, &DecodingKey::from_secret(b"secret"), &v);
    assert!(res.is_err());
    println!("{:?}", res);
    //assert!(res.is_ok());
}


pub fn generate_algorithm_enum_from_str() {
    assert!(Algorithm::from_str("HS256").is_ok());
    assert!(Algorithm::from_str("HS384").is_ok());
    assert!(Algorithm::from_str("HS512").is_ok());
    assert!(Algorithm::from_str("RS256").is_ok());
    assert!(Algorithm::from_str("RS384").is_ok());
    assert!(Algorithm::from_str("RS512").is_ok());
    assert!(Algorithm::from_str("").is_err());
}
