#![allow(deprecated)]

use std::prelude::v1::*;

use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::error::Error;
use std::path::{Path, PathBuf};

use quick_error::ResultExt;

quick_error! {
    #[derive(Debug)]
    pub enum Bare {
        One
        Two
    }
}

//#[test]
pub fn bare_item_direct() {
    assert_eq!(format!("{}", Bare::One), "One".to_string());
    assert_eq!(format!("{:?}", Bare::One), "One".to_string());
    assert_eq!(Bare::One.description(), "One".to_string());
    assert!(Bare::One.cause().is_none());
}
//#[test]
pub fn bare_item_trait() {
    let err: &Error = &Bare::Two;
    assert_eq!(format!("{}", err), "Two".to_string());
    assert_eq!(format!("{:?}", err), "Two".to_string());
    assert_eq!(err.description(), "Two".to_string());
    assert!(err.cause().is_none());
}

quick_error! {
    #[derive(Debug)]
    pub enum Wrapper wraps Wrapped {
        One
        Two(s: String) {
            display("two: {}", s)
            from()
        }
    }
}

//#[test]
pub fn wrapper() {
    assert_eq!(format!("{}", Wrapper::from(Wrapped::One)),
        "One".to_string());
    assert_eq!(format!("{}",
        Wrapper::from(Wrapped::from(String::from("hello")))),
        "two: hello".to_string());
    assert_eq!(format!("{:?}", Wrapper::from(Wrapped::One)),
        "Wrapper(One)".to_string());
    assert_eq!(Wrapper::from(Wrapped::One).description(),
        "One".to_string());
}

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum TupleWrapper {
        /// ParseFloat Error
        ParseFloatError(err: ParseFloatError) {
            from()
            description(err.description())
            display("parse float error: {err}", err=err)
            cause(err)
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error: {}", descr)
        }
        /// FromUtf8 Error
        FromUtf8Error(err: Utf8Error, source: Vec<u8>) {
            cause(err)
            display(me) -> ("{desc} at index {pos}: {err}", desc=me.description(), pos=err.valid_up_to(), err=err)
            description("utf8 error")
            from(err: FromUtf8Error) -> (err.utf8_error().clone(), err.into_bytes())
        }
        Discard {
            from(&'static str)
        }
        Singleton {
            display("Just a string")
        }
    }
}

//#[test]
pub fn tuple_wrapper_err() {
    let cause = "one and a half times pi".parse::<f32>().unwrap_err();
    let err = TupleWrapper::ParseFloatError(cause.clone());
    assert_eq!(format!("{}", err), format!("parse float error: {}", cause));
    assert_eq!(format!("{:?}", err), format!("ParseFloatError({:?})", cause));
    assert_eq!(err.description(), cause.description());
    assert_eq!(format!("{:?}", err.cause().unwrap()), format!("{:?}", cause));
}

//#[test]
pub fn tuple_wrapper_trait_str() {
    let desc = "hello";
    let err: &Error = &TupleWrapper::Other(desc);
    assert_eq!(format!("{}", err), format!("Error: {}", desc));
    assert_eq!(format!("{:?}", err), format!("Other({:?})", desc));
    assert_eq!(err.description(), desc);
    assert!(err.cause().is_none());
}

//#[test]
pub fn tuple_wrapper_trait_two_fields() {
    let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
    let cause = String::from_utf8(invalid_utf8.clone()).unwrap_err().utf8_error();
    let err: &Error = &TupleWrapper::FromUtf8Error(cause.clone(), invalid_utf8.clone());
    assert_eq!(format!("{}", err), format!("{desc} at index {pos}: {cause}", desc=err.description(), pos=cause.valid_up_to(), cause=cause));
    assert_eq!(format!("{:?}", err), format!("FromUtf8Error({:?}, {:?})", cause, invalid_utf8));
    assert_eq!(err.description(), "utf8 error");
    assert_eq!(format!("{:?}", err.cause().unwrap()), format!("{:?}", cause));
}

//#[test]
pub fn tuple_wrapper_from() {
    let cause = "one and a half times pi".parse::<f32>().unwrap_err();
    let err = TupleWrapper::ParseFloatError(cause.clone());
    let err_from: TupleWrapper = From::from(cause);
    assert_eq!(err_from, err);
}

//#[test]
pub fn tuple_wrapper_custom_from() {
    let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
    let cause = String::from_utf8(invalid_utf8.clone()).unwrap_err();
    let err = TupleWrapper::FromUtf8Error(cause.utf8_error().clone(), invalid_utf8);
    let err_from: TupleWrapper = From::from(cause);
    assert_eq!(err_from, err);
}

//#[test]
pub fn tuple_wrapper_discard() {
    let err: TupleWrapper = From::from("hello");
    assert_eq!(format!("{}", err), format!("Discard"));
    assert_eq!(format!("{:?}", err), format!("Discard"));
    assert_eq!(err.description(), "Discard");
    assert!(err.cause().is_none());
}

//#[test]
pub fn tuple_wrapper_singleton() {
    let err: TupleWrapper = TupleWrapper::Singleton;
    assert_eq!(format!("{}", err), format!("Just a string"));
    assert_eq!(format!("{:?}", err), format!("Singleton"));
    assert_eq!(err.description(), "Singleton");
    assert!(err.cause().is_none());
}

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum StructWrapper {
        // Utf8 Error
        Utf8Error{ err: Utf8Error, hint: Option<&'static str> } {
            cause(err)
            display(me) -> ("{desc} at index {pos}: {err}", desc=me.description(), pos=err.valid_up_to(), err=err)
            description("utf8 error")
            from(err: Utf8Error) -> { err: err, hint: None }
        }
        // Utf8 Error
        ExcessComma { descr: &'static str, } {
            description(descr)
            display("Error: {}", descr)
        }
    }
}

//#[test]
pub fn struct_wrapper_err() {
    let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
    let cause = String::from_utf8(invalid_utf8.clone()).unwrap_err().utf8_error();
    let err: &Error = &StructWrapper::Utf8Error{ err: cause.clone(), hint: Some("nonsense") };
    assert_eq!(format!("{}", err), format!("{desc} at index {pos}: {cause}", desc=err.description(), pos=cause.valid_up_to(), cause=cause));
    assert_eq!(format!("{:?}", err), format!("Utf8Error {{ err: {:?}, hint: {:?} }}", cause, Some("nonsense")));
    assert_eq!(err.description(), "utf8 error");
    assert_eq!(format!("{:?}", err.cause().unwrap()), format!("{:?}", cause));
}

//#[test]
pub fn struct_wrapper_struct_from() {
    let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
    let cause = String::from_utf8(invalid_utf8.clone()).unwrap_err().utf8_error();
    let err = StructWrapper::Utf8Error{ err: cause.clone(), hint: None };
    let err_from: StructWrapper = From::from(cause);
    assert_eq!(err_from, err);
}

//#[test]
pub fn struct_wrapper_excess_comma() {
    let descr = "hello";
    let err = StructWrapper::ExcessComma { descr: descr };
    assert_eq!(format!("{}", err), format!("Error: {}", descr));
    assert_eq!(format!("{:?}", err), format!("ExcessComma {{ descr: {:?} }}", descr));
    assert_eq!(err.description(), descr);
    assert!(err.cause().is_none());
}

quick_error! {
    #[derive(Debug)]
    pub enum ContextErr {
        Float(src: String, err: ParseFloatError) {
            context(s: &'a str, e: ParseFloatError) -> (s.to_string(), e)
            display("Float error {:?}: {}", src, err)
        }
        Int { src: String, err: ParseIntError } {
            context(s: &'a str, e: ParseIntError)
                -> {src: s.to_string(), err: e}
            display("Int error {:?}: {}", src, err)
        }
        Utf8(path: PathBuf, err: Utf8Error) {
            context(p: AsRef<Path>, e: Utf8Error)
                -> (p.as_ref().to_path_buf(), e)
            display("Path error at {:?}: {}", path, err)
        }
        Utf8Str(s: String, err: ::std::io::Error) {
            context(s: AsRef<str>, e: ::std::io::Error)
                -> (s.as_ref().to_string(), e)
            display("Str error {:?}: {}", s, err)
        }
    }
}

//#[test]
pub fn parse_float_error() {
    fn parse_float(s: &str) -> Result<f32, ContextErr> {
        Ok(s.parse().context(s)?)
    }
    assert_eq!(format!("{}", parse_float("12ab").unwrap_err()),
        r#"Float error "12ab": invalid float literal"#);
}

//#[test]
pub fn parse_int_error() {
    fn parse_int(s: &str) -> Result<i32, ContextErr> {
        Ok(s.parse().context(s)?)
    }
    assert_eq!(format!("{}", parse_int("12.5").unwrap_err()),
        r#"Int error "12.5": invalid digit found in string"#);
}

//#[test]
pub fn debug_context() {
    fn parse_int(s: &str) -> i32 {
        s.parse().context(s).unwrap()
    }
    assert_eq!(parse_int("12"), 12);
    assert_eq!(format!("{:?}", "x".parse::<i32>().context("x")),
        r#"Err(Context("x", ParseIntError { kind: InvalidDigit }))"#);
}

//#[test]
pub fn path_context() {
    fn parse_utf<P: AsRef<Path>>(s: &[u8], p: P)
        -> Result<(), ContextErr>
    {
        ::std::str::from_utf8(s).context(p)?;
        Ok(())
    }
    let etext = parse_utf(b"a\x80\x80", "/etc").unwrap_err().to_string();
    assert!(etext.starts_with(
        "Path error at \"/etc\": invalid utf-8"));
    let etext = parse_utf(b"\x80\x80", PathBuf::from("/tmp")).unwrap_err()
        .to_string();
    assert!(etext.starts_with(
        "Path error at \"/tmp\": invalid utf-8"));
}

//#[test]
pub fn conditional_compilation() {
    quick_error! {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum Test {
            #[cfg(feature = "foo")]
            Variant
        }
    }
}
