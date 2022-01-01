use std::prelude::v1::*;
use uuid::*;
use uuid::v1::*;

//#[test]
pub fn test_new_v1() {
    let time: u64 = 1_496_854_535;
    let time_fraction: u32 = 812_946_000;
    let node = [1, 2, 3, 4, 5, 6];
    let context = Context::new(0);

    {
        let uuid = Uuid::new_v1(
            Timestamp::from_unix(&context, time, time_fraction),
            &node,
        )
        .unwrap();

        assert_eq!(uuid.get_version(), Some(Version::Mac));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
        assert_eq!(
            uuid.to_hyphenated().to_string(),
            "20616934-4ba2-11e7-8000-010203040506"
        );

        let ts = uuid.to_timestamp().unwrap().to_rfc4122();

        assert_eq!(ts.0 - 0x01B2_1DD2_1381_4000, 14_968_545_358_129_460);
        assert_eq!(ts.1, 0);
    };

    {
        let uuid2 = Uuid::new_v1(
            Timestamp::from_unix(&context, time, time_fraction),
            &node,
        )
        .unwrap();

        assert_eq!(
            uuid2.to_hyphenated().to_string(),
            "20616934-4ba2-11e7-8001-010203040506"
        );
        assert_eq!(uuid2.to_timestamp().unwrap().to_rfc4122().1, 1)
    };
}
