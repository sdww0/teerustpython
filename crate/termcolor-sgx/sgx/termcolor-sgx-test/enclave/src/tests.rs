use std::prelude::v1::*;
use termcolor::{
    Ansi, Color, ParseColorError, ParseColorErrorKind, StandardStream,
};

fn assert_is_send<T: Send>() {}

//#[test]
pub fn standard_stream_is_send() {
    assert_is_send::<StandardStream>();
}

//#[test]
pub fn test_simple_parse_ok() {
    let color = "green".parse::<Color>();
    assert_eq!(color, Ok(Color::Green));
}

//#[test]
pub fn test_256_parse_ok() {
    let color = "7".parse::<Color>();
    assert_eq!(color, Ok(Color::Ansi256(7)));

    let color = "32".parse::<Color>();
    assert_eq!(color, Ok(Color::Ansi256(32)));

    let color = "0xFF".parse::<Color>();
    assert_eq!(color, Ok(Color::Ansi256(0xFF)));
}

//#[test]
pub fn test_256_parse_err_out_of_range() {
    let color = "256".parse::<Color>();
    assert_eq!(color, Err(ParseColorError {
        kind: ParseColorErrorKind::InvalidAnsi256,
        given: "256".to_string(),
    }));
}

//#[test]
pub fn test_rgb_parse_ok() {
    let color = "0,0,0".parse::<Color>();
    assert_eq!(color, Ok(Color::Rgb(0, 0, 0)));

    let color = "0,128,255".parse::<Color>();
    assert_eq!(color, Ok(Color::Rgb(0, 128, 255)));

    let color = "0x0,0x0,0x0".parse::<Color>();
    assert_eq!(color, Ok(Color::Rgb(0, 0, 0)));

    let color = "0x33,0x66,0xFF".parse::<Color>();
    assert_eq!(color, Ok(Color::Rgb(0x33, 0x66, 0xFF)));
}

//#[test]
pub fn test_rgb_parse_err_out_of_range() {
    let color = "0,0,256".parse::<Color>();
    assert_eq!(color, Err(ParseColorError {
        kind: ParseColorErrorKind::InvalidRgb,
        given: "0,0,256".to_string(),
    }));
}

//#[test]
pub fn test_rgb_parse_err_bad_format() {
    let color = "0,0".parse::<Color>();
    assert_eq!(color, Err(ParseColorError {
        kind: ParseColorErrorKind::InvalidRgb,
        given: "0,0".to_string(),
    }));

    let color = "not_a_color".parse::<Color>();
    assert_eq!(color, Err(ParseColorError {
        kind: ParseColorErrorKind::InvalidName,
        given: "not_a_color".to_string(),
    }));
}

//#[test]
pub fn test_var_ansi_write_rgb() {
    let mut buf = Ansi::new(vec![]);
    let _ = buf.write_color(true, &Color::Rgb(254, 253, 255), false);
    assert_eq!(*buf.get_ref(), b"\x1B[38;2;254;253;255m");
}

//#[test]
pub fn test_var_ansi_write_256() {
    let mut buf = Ansi::new(vec![]);
    let _ = buf.write_color(false, &Color::Ansi256(7), false);
    assert_eq!(*buf.get_ref(), b"\x1B[48;5;7m");

    let mut buf = Ansi::new(vec![]);
    let _ = buf.write_color(false, &Color::Ansi256(208), false);
    assert_eq!(*buf.get_ref(), b"\x1B[48;5;208m");
}
