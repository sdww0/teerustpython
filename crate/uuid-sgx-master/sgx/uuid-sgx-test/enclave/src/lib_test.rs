use uuid::prelude::*;
use super::test_util;
use std::prelude::v1::*;

macro_rules! check {
    ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
        $buf.clear();
        write!($buf, $format, $target).unwrap();
        assert!($buf.len() == $len);
        assert!($buf.chars().all($cond), "{}", $buf);
    };
}

//#[test]
pub fn test_uuid_compare() {
    let uuid1 = test_util::new();
    let uuid2 = test_util::new2();

    assert_eq!(uuid1, uuid1);
    assert_eq!(uuid2, uuid2);

    assert_ne!(uuid1, uuid2);
    assert_ne!(uuid2, uuid1);
}

//#[test]
pub fn test_uuid_default() {
    let default_uuid = Uuid::default();
    let nil_uuid = Uuid::nil();

    assert_eq!(default_uuid, nil_uuid);
}

//#[test]
pub fn test_uuid_display() {
    use std::fmt::Write;

    let uuid = test_util::new();
    let s = uuid.to_string();
    let mut buffer = String::new();

    assert_eq!(s, uuid.to_hyphenated().to_string());

    check!(buffer, "{}", uuid, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

//#[test]
pub fn test_uuid_lowerhex() {
    use std::fmt::Write;

    let mut buffer = String::new();
    let uuid = test_util::new();

    check!(buffer, "{:x}", uuid, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

// noinspection RsAssertEqual
//#[test]
pub fn test_uuid_operator_eq() {
    let uuid1 = test_util::new();
    let uuid1_dup = uuid1.clone();
    let uuid2 = test_util::new2();

    assert!(uuid1 == uuid1);
    assert!(uuid1 == uuid1_dup);
    assert!(uuid1_dup == uuid1);

    assert!(uuid1 != uuid2);
    assert!(uuid2 != uuid1);
    assert!(uuid1_dup != uuid2);
    assert!(uuid2 != uuid1_dup);
}

//#[test]
pub fn test_uuid_to_string() {
    use std::fmt::Write;

    let uuid = test_util::new();
    let s = uuid.to_string();
    let mut buffer = String::new();

    assert_eq!(s.len(), 36);

    check!(buffer, "{}", s, 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
}

//#[test]
pub fn test_uuid_upperhex() {
    use std::fmt::Write;

    let mut buffer = String::new();
    let uuid = test_util::new();

    check!(buffer, "{:X}", uuid, 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
}

//#[test]
pub fn test_nil() {
    let nil = Uuid::nil();
    let not_nil = test_util::new();
    let from_bytes = Uuid::from_bytes([
        4, 54, 67, 12, 43, 2, 2, 76, 32, 50, 87, 5, 1, 33, 43, 87,
    ]);

    assert_eq!(from_bytes.get_version(), None);

    assert!(nil.is_nil());
    assert!(!not_nil.is_nil());

    assert_eq!(nil.get_version(), Some(Version::Nil));
    assert_eq!(not_nil.get_version(), Some(Version::Random))
}

//#[test]
pub fn test_predefined_namespaces() {
    assert_eq!(
        Uuid::NAMESPACE_DNS.to_hyphenated().to_string(),
        "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_URL.to_hyphenated().to_string(),
        "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_OID.to_hyphenated().to_string(),
        "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_X500.to_hyphenated().to_string(),
        "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
    );
}

//#[cfg(feature = "v3")]
//#[test]
pub fn test_get_version_v3() {
    let uuid =
        Uuid::new_v3(&Uuid::NAMESPACE_DNS, "rust-lang.org".as_bytes());

    assert_eq!(uuid.get_version().unwrap(), Version::Md5);
    assert_eq!(uuid.get_version_num(), 3);
}

//#[test]
pub fn test_get_variant() {
    let uuid1 = test_util::new();
    let uuid2 =
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid3 =
        Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
    let uuid4 =
        Uuid::parse_str("936DA01F9ABD4d9dC0C702AF85C822A8").unwrap();
    let uuid5 =
        Uuid::parse_str("F9168C5E-CEB2-4faa-D6BF-329BF39FA1E4").unwrap();
    let uuid6 =
        Uuid::parse_str("f81d4fae-7dec-11d0-7765-00a0c91e6bf6").unwrap();

    assert_eq!(uuid1.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid2.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid3.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid4.get_variant().unwrap(), Variant::Microsoft);
    assert_eq!(uuid5.get_variant().unwrap(), Variant::Microsoft);
    assert_eq!(uuid6.get_variant().unwrap(), Variant::NCS);
}

//#[test]
pub fn test_to_simple_string() {
    let uuid1 = test_util::new();
    let s = uuid1.to_simple().to_string();

    assert_eq!(s.len(), 32);
    assert!(s.chars().all(|c| c.is_digit(16)));
}

//#[test]
pub fn test_to_hyphenated_string() {
    let uuid1 = test_util::new();
    let s = uuid1.to_hyphenated().to_string();

    assert!(s.len() == 36);
    assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
}

//#[test]
pub fn test_upper_lower_hex() {
    use std::fmt::Write;

    let mut buf = String::new();
    let u = test_util::new();

    macro_rules! check {
        ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
            $buf.clear();
            write!($buf, $format, $target).unwrap();
            assert!(buf.len() == $len);
            assert!($buf.chars().all($cond), "{}", $buf);
        };
    }

    check!(buf, "{:X}", u, 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:X}", u.to_hyphenated(), 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:X}", u.to_simple(), 32, |c| c.is_uppercase()
        || c.is_digit(10));

    check!(buf, "{:x}", u.to_hyphenated(), 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:x}", u.to_simple(), 32, |c| c.is_lowercase()
        || c.is_digit(10));
}

//#[test]
pub fn test_to_urn_string() {
    let uuid1 = test_util::new();
    let ss = uuid1.to_urn().to_string();
    let s = &ss[9..];

    assert!(ss.starts_with("urn:uuid:"));
    assert_eq!(s.len(), 36);
    assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
}

//#[test]
pub fn test_to_simple_string_matching() {
    let uuid1 = test_util::new();

    let hs = uuid1.to_hyphenated().to_string();
    let ss = uuid1.to_simple().to_string();

    let hsn = hs.chars().filter(|&c| c != '-').collect::<String>();

    assert_eq!(hsn, ss);
}

//#[test]
pub fn test_string_roundtrip() {
    let uuid = test_util::new();

    let hs = uuid.to_hyphenated().to_string();
    let uuid_hs = Uuid::parse_str(&hs).unwrap();
    assert_eq!(uuid_hs, uuid);

    let ss = uuid.to_string();
    let uuid_ss = Uuid::parse_str(&ss).unwrap();
    assert_eq!(uuid_ss, uuid);
}

//#[test]
pub fn test_from_fields() {
    let d1: u32 = 0xa1a2a3a4;
    let d2: u16 = 0xb1b2;
    let d3: u16 = 0xc1c2;
    let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1, d2, d3, &d4).unwrap();

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_from_fields_le() {
    let d1: u32 = 0xa4a3a2a1;
    let d2: u16 = 0xb2b1;
    let d3: u16 = 0xc2c1;
    let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields_le(d1, d2, d3, &d4).unwrap();

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_as_fields() {
    let u = test_util::new();
    let (d1, d2, d3, d4) = u.as_fields();

    assert_ne!(d1, 0);
    assert_ne!(d2, 0);
    assert_ne!(d3, 0);
    assert_eq!(d4.len(), 8);
    assert!(!d4.iter().all(|&b| b == 0));
}

//#[test]
pub fn test_fields_roundtrip() {
    let d1_in: u32 = 0xa1a2a3a4;
    let d2_in: u16 = 0xb1b2;
    let d3_in: u16 = 0xc1c2;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.as_fields();

    assert_eq!(d1_in, d1_out);
    assert_eq!(d2_in, d2_out);
    assert_eq!(d3_in, d3_out);
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_fields_le_roundtrip() {
    let d1_in: u32 = 0xa4a3a2a1;
    let d2_in: u16 = 0xb2b1;
    let d3_in: u16 = 0xc2c1;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields_le(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.to_fields_le();

    assert_eq!(d1_in, d1_out);
    assert_eq!(d2_in, d2_out);
    assert_eq!(d3_in, d3_out);
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_fields_le_are_actually_le() {
    let d1_in: u32 = 0xa1a2a3a4;
    let d2_in: u16 = 0xb1b2;
    let d3_in: u16 = 0xc1c2;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.to_fields_le();

    assert_eq!(d1_in, d1_out.swap_bytes());
    assert_eq!(d2_in, d2_out.swap_bytes());
    assert_eq!(d3_in, d3_out.swap_bytes());
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_from_u128() {
    let v_in: u128 = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8;

    let u = Uuid::from_u128(v_in);

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_from_u128_le() {
    let v_in: u128 = 0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1;

    let u = Uuid::from_u128_le(v_in);

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_u128_roundtrip() {
    let v_in: u128 = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8;

    let u = Uuid::from_u128(v_in);
    let v_out = u.as_u128();

    assert_eq!(v_in, v_out);
}

//#[test]
pub fn test_u128_le_roundtrip() {
    let v_in: u128 = 0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1;

    let u = Uuid::from_u128_le(v_in);
    let v_out = u.to_u128_le();

    assert_eq!(v_in, v_out);
}

//#[test]
pub fn test_u128_le_is_actually_le() {
    let v_in: u128 = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8;

    let u = Uuid::from_u128(v_in);
    let v_out = u.to_u128_le();

    assert_eq!(v_in, v_out.swap_bytes());
}

//#[test]
pub fn test_from_slice() {
    let b = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_slice(&b).unwrap();
    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

    assert_eq!(u.to_simple().to_string(), expected);
}

//#[test]
pub fn test_from_bytes() {
    let b = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_bytes(b);
    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

    assert_eq!(u.to_simple().to_string(), expected);
}

//#[test]
pub fn test_as_bytes() {
    let u = test_util::new();
    let ub = u.as_bytes();

    assert_eq!(ub.len(), 16);
    assert!(!ub.iter().all(|&b| b == 0));
}

//#[test]
pub fn test_bytes_roundtrip() {
    let b_in: uuid::Bytes = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_slice(&b_in).unwrap();

    let b_out = u.as_bytes();

    assert_eq!(&b_in, b_out);
}

//#[test]
pub fn test_iterbytes_impl_for_uuid() {
    let mut set = std::collections::HashSet::new();
    let id1 = test_util::new();
    let id2 = test_util::new2();
    set.insert(id1.clone());

    assert!(set.contains(&id1));
    assert!(!set.contains(&id2));
}
