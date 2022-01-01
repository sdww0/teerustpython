use http::*;

//#[test]
pub fn inner_error_is_invalid_status_code() {
    if let Err(e) = status::StatusCode::from_u16(6666) {
        let err: Error = e.into();
        let ie = err.get_ref();
        assert!(!ie.is::<header::InvalidHeaderValue>());
        assert!( ie.is::<status::InvalidStatusCode>());
        ie.downcast_ref::<status::InvalidStatusCode>().unwrap();

        assert!(!err.is::<header::InvalidHeaderValue>());
        assert!( err.is::<status::InvalidStatusCode>());
    } else {
        panic!("Bad status allowed!");
    }
}
