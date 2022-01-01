//extern crate toml;

use std::prelude::v1::*;
use std::str::FromStr;

use toml::Value;

//#[test]
pub fn times() {
    fn dogood(s: &str, serialized: &str) {
        let to_parse = format!("foo = {}", s);
        let value = Value::from_str(&to_parse).unwrap();
        assert_eq!(value["foo"].as_datetime().unwrap().to_string(), serialized);
    }
    fn good(s: &str) {
        dogood(s, s);
        dogood(&s.replace("T", " "), s);
        dogood(&s.replace("T", "t"), s);
        dogood(&s.replace("Z", "z"), s);
    }

    good("1997-09-09T09:09:09Z");
    good("1997-09-09T09:09:09+09:09");
    good("1997-09-09T09:09:09-09:09");
    good("1997-09-09T09:09:09");
    good("1997-09-09");
    dogood("1997-09-09 ", "1997-09-09");
    dogood("1997-09-09 # comment", "1997-09-09");
    good("09:09:09");
    good("1997-09-09T09:09:09.09Z");
    good("1997-09-09T09:09:09.09+09:09");
    good("1997-09-09T09:09:09.09-09:09");
    good("1997-09-09T09:09:09.09");
    good("09:09:09.09");
}

//#[test]
pub fn bad_times() {
    fn bad(s: &str) {
        let to_parse = format!("foo = {}", s);
        assert!(Value::from_str(&to_parse).is_err());
    }

    bad("199-09-09");
    bad("199709-09");
    bad("1997-9-09");
    bad("1997-09-9");
    bad("1997-09-0909:09:09");
    bad("1997-09-09T09:09:09.");
    bad("T");
    bad("T.");
    bad("TZ");
    bad("1997-09-09T09:09:09.09+");
    bad("1997-09-09T09:09:09.09+09");
    bad("1997-09-09T09:09:09.09+09:9");
    bad("1997-09-09T09:09:09.09+0909");
    bad("1997-09-09T09:09:09.09-");
    bad("1997-09-09T09:09:09.09-09");
    bad("1997-09-09T09:09:09.09-09:9");
    bad("1997-09-09T09:09:09.09-0909");

    bad("1997-00-09T09:09:09.09Z");
    bad("1997-09-00T09:09:09.09Z");
    bad("1997-09-09T30:09:09.09Z");
    bad("1997-09-09T12:69:09.09Z");
    bad("1997-09-09T12:09:69.09Z");
}
