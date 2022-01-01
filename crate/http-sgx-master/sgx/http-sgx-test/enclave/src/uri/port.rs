use http::uri::*;
use std::prelude::v1::*;

//#[test]
pub fn partialeq_port() {
    let port_a = Port::from_str("8080").unwrap();
    let port_b = Port::from_str("8080").unwrap();
    assert_eq!(port_a, port_b);
}

//#[test]
pub fn partialeq_port_different_reprs() {
    let port_a = Port {
        repr: "8081",
        port: 8081,
    };
    let port_b = Port {
        repr: String::from("8081"),
        port: 8081,
    };
    assert_eq!(port_a, port_b);
    assert_eq!(port_b, port_a);
}

//#[test]
pub fn partialeq_u16() {
    let port = Port::from_str("8080").unwrap();
    // test equals in both directions
    assert_eq!(port, 8080);
    assert_eq!(8080, port);
}

//#[test]
pub fn u16_from_port() {
    let port = Port::from_str("8080").unwrap();
    assert_eq!(8080, u16::from(port));
}
