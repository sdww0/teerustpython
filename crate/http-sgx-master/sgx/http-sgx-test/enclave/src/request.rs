use http::*;

//#[test]
pub fn it_can_map_a_body_from_one_type_to_another() {
    let request= Request::builder().body("some string").unwrap();
    let mapped_request = request.map(|s| {
        assert_eq!(s, "some string");
        123u32
    });
    assert_eq!(mapped_request.body(), &123u32);
}
