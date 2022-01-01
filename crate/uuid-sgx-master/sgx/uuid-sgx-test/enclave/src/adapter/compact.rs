use serde_test;

//#[test]
pub fn test_serialize_compact() {
    #[derive(serde::Serialize, Debug, serde::Deserialize, PartialEq)]
    struct UuidContainer {
        #[serde(with = "uuid::adapter::compact")]
        u: uuid::Uuid,
    }
    use serde_test::Configure;

    let uuid_bytes = b"F9168C5E-CEB2-4F";
    let container = UuidContainer {
        u: uuid::Uuid::from_slice(uuid_bytes).unwrap(),
    };

    // more complex because of the struct wrapping the actual UUID
    // serialization
    serde_test::assert_tokens(
        &container.compact(),
        &[
            serde_test::Token::Struct {
                name: "UuidContainer",
                len: 1,
            },
            serde_test::Token::Str("u"),
            serde_test::Token::Tuple { len: 16 },
            serde_test::Token::U8(uuid_bytes[0]),
            serde_test::Token::U8(uuid_bytes[1]),
            serde_test::Token::U8(uuid_bytes[2]),
            serde_test::Token::U8(uuid_bytes[3]),
            serde_test::Token::U8(uuid_bytes[4]),
            serde_test::Token::U8(uuid_bytes[5]),
            serde_test::Token::U8(uuid_bytes[6]),
            serde_test::Token::U8(uuid_bytes[7]),
            serde_test::Token::U8(uuid_bytes[8]),
            serde_test::Token::U8(uuid_bytes[9]),
            serde_test::Token::U8(uuid_bytes[10]),
            serde_test::Token::U8(uuid_bytes[11]),
            serde_test::Token::U8(uuid_bytes[12]),
            serde_test::Token::U8(uuid_bytes[13]),
            serde_test::Token::U8(uuid_bytes[14]),
            serde_test::Token::U8(uuid_bytes[15]),
            serde_test::Token::TupleEnd,
            serde_test::Token::StructEnd,
        ],
    )
}
