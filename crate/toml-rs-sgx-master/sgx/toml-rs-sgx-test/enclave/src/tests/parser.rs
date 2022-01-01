//extern crate toml;

use std::prelude::v1::*;
use toml::Value;

macro_rules! bad {
    ($s:expr, $msg:expr) => {{
        match $s.parse::<Value>() {
            Ok(s) => panic!("successfully parsed as {}", s),
            Err(e) => {
                let e = e.to_string();
                assert!(e.contains($msg), "error: {}", e);
            }
        }
    }};
}

//#[test]
pub fn crlf() {
    "\
     [project]\r\n\
     \r\n\
     name = \"splay\"\r\n\
     version = \"0.1.0\"\r\n\
     authors = [\"alex@crichton.co\"]\r\n\
     \r\n\
     [[lib]]\r\n\
     \r\n\
     path = \"lib.rs\"\r\n\
     name = \"splay\"\r\n\
     description = \"\"\"\
     A Rust implementation of a TAR file reader and writer. This library does not\r\n\
     currently handle compression, but it is abstract over all I/O readers and\r\n\
     writers. Additionally, great lengths are taken to ensure that the entire\r\n\
     contents are never required to be entirely resident in memory all at once.\r\n\
     \"\"\"\
     "
    .parse::<Value>()
    .unwrap();
}

//#[test]
pub fn fun_with_strings() {
    let table = r#"
bar = "\U00000000"
key1 = "One\nTwo"
key2 = """One\nTwo"""
key3 = """
One
Two"""

key4 = "The quick brown fox jumps over the lazy dog."
key5 = """
The quick brown \


fox jumps over \
the lazy dog."""
key6 = """\
   The quick brown \
   fox jumps over \
   the lazy dog.\
   """
# What you see is what you get.
winpath  = 'C:\Users\nodejs\templates'
winpath2 = '\\ServerX\admin$\system32\'
quoted   = 'Tom "Dubs" Preston-Werner'
regex    = '<\i\c*\s*>'

regex2 = '''I [dw]on't need \d{2} apples'''
lines  = '''
The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
'''
"#
    .parse::<Value>()
    .unwrap();
    assert_eq!(table["bar"].as_str(), Some("\0"));
    assert_eq!(table["key1"].as_str(), Some("One\nTwo"));
    assert_eq!(table["key2"].as_str(), Some("One\nTwo"));
    assert_eq!(table["key3"].as_str(), Some("One\nTwo"));

    let msg = "The quick brown fox jumps over the lazy dog.";
    assert_eq!(table["key4"].as_str(), Some(msg));
    assert_eq!(table["key5"].as_str(), Some(msg));
    assert_eq!(table["key6"].as_str(), Some(msg));

    assert_eq!(
        table["winpath"].as_str(),
        Some(r"C:\Users\nodejs\templates")
    );
    assert_eq!(
        table["winpath2"].as_str(),
        Some(r"\\ServerX\admin$\system32\")
    );
    assert_eq!(
        table["quoted"].as_str(),
        Some(r#"Tom "Dubs" Preston-Werner"#)
    );
    assert_eq!(table["regex"].as_str(), Some(r"<\i\c*\s*>"));
    assert_eq!(
        table["regex2"].as_str(),
        Some(r"I [dw]on't need \d{2} apples")
    );
    assert_eq!(
        table["lines"].as_str(),
        Some(
            "The first newline is\n\
             trimmed in raw strings.\n\
             All other whitespace\n\
             is preserved.\n"
        )
    );
}

//#[test]
pub fn tables_in_arrays() {
    let table = r#"
[[foo]]
#…
[foo.bar]
#…

[[foo]] # ...
#…
[foo.bar]
#...
"#
    .parse::<Value>()
    .unwrap();
    table["foo"][0]["bar"].as_table().unwrap();
    table["foo"][1]["bar"].as_table().unwrap();
}

//#[test]
pub fn empty_table() {
    let table = r#"
[foo]"#
        .parse::<Value>()
        .unwrap();
    table["foo"].as_table().unwrap();
}

//#[test]
pub fn fruit() {
    let table = r#"
[[fruit]]
name = "apple"

[fruit.physical]
color = "red"
shape = "round"

[[fruit.variety]]
name = "red delicious"

[[fruit.variety]]
name = "granny smith"

[[fruit]]
name = "banana"

[[fruit.variety]]
name = "plantain"
"#
    .parse::<Value>()
    .unwrap();
    assert_eq!(table["fruit"][0]["name"].as_str(), Some("apple"));
    assert_eq!(table["fruit"][0]["physical"]["color"].as_str(), Some("red"));
    assert_eq!(
        table["fruit"][0]["physical"]["shape"].as_str(),
        Some("round")
    );
    assert_eq!(
        table["fruit"][0]["variety"][0]["name"].as_str(),
        Some("red delicious")
    );
    assert_eq!(
        table["fruit"][0]["variety"][1]["name"].as_str(),
        Some("granny smith")
    );
    assert_eq!(table["fruit"][1]["name"].as_str(), Some("banana"));
    assert_eq!(
        table["fruit"][1]["variety"][0]["name"].as_str(),
        Some("plantain")
    );
}

//#[test]
pub fn stray_cr() {
    "\r".parse::<Value>().unwrap_err();
    "a = [ \r ]".parse::<Value>().unwrap_err();
    "a = \"\"\"\r\"\"\"".parse::<Value>().unwrap_err();
    "a = \"\"\"\\  \r  \"\"\"".parse::<Value>().unwrap_err();
    "a = '''\r'''".parse::<Value>().unwrap_err();
    "a = '\r'".parse::<Value>().unwrap_err();
    "a = \"\r\"".parse::<Value>().unwrap_err();
}

//#[test]
pub fn blank_literal_string() {
    let table = "foo = ''".parse::<Value>().unwrap();
    assert_eq!(table["foo"].as_str(), Some(""));
}

//#[test]
pub fn many_blank() {
    let table = "foo = \"\"\"\n\n\n\"\"\"".parse::<Value>().unwrap();
    assert_eq!(table["foo"].as_str(), Some("\n\n"));
}

//#[test]
pub fn literal_eats_crlf() {
    let table = "
        foo = \"\"\"\\\r\n\"\"\"
        bar = \"\"\"\\\r\n   \r\n   \r\n   a\"\"\"
    "
    .parse::<Value>()
    .unwrap();
    assert_eq!(table["foo"].as_str(), Some(""));
    assert_eq!(table["bar"].as_str(), Some("a"));
}

//#[test]
pub fn string_no_newline() {
    "a = \"\n\"".parse::<Value>().unwrap_err();
    "a = '\n'".parse::<Value>().unwrap_err();
}

//#[test]
pub fn bad_leading_zeros() {
    "a = 00".parse::<Value>().unwrap_err();
    "a = -00".parse::<Value>().unwrap_err();
    "a = +00".parse::<Value>().unwrap_err();
    "a = 00.0".parse::<Value>().unwrap_err();
    "a = -00.0".parse::<Value>().unwrap_err();
    "a = +00.0".parse::<Value>().unwrap_err();
    "a = 9223372036854775808".parse::<Value>().unwrap_err();
    "a = -9223372036854775809".parse::<Value>().unwrap_err();
}

//#[test]
pub fn bad_floats() {
    bad!("a = 0.", "invalid number at line 1 column 7");
    bad!("a = 0.e", "invalid number at line 1 column 7");
    bad!("a = 0.E", "invalid number at line 1 column 7");
    bad!("a = 0.0E", "invalid number at line 1 column 5");
    bad!("a = 0.0e", "invalid number at line 1 column 5");
    bad!("a = 0.0e-", "invalid number at line 1 column 9");
    bad!("a = 0.0e+", "invalid number at line 1 column 5");
}

//#[test]
pub fn floats() {
    macro_rules! t {
        ($actual:expr, $expected:expr) => {{
            let f = format!("foo = {}", $actual);
            println!("{}", f);
            let a = f.parse::<Value>().unwrap();
            assert_eq!(a["foo"].as_float().unwrap(), $expected);
        }};
    }

    t!("1.0", 1.0);
    t!("1.0e0", 1.0);
    t!("1.0e+0", 1.0);
    t!("1.0e-0", 1.0);
    t!("1E-0", 1.0);
    t!("1.001e-0", 1.001);
    t!("2e10", 2e10);
    t!("2e+10", 2e10);
    t!("2e-10", 2e-10);
    t!("2_0.0", 20.0);
    t!("2_0.0_0e1_0", 20.0e10);
    t!("2_0.1_0e1_0", 20.1e10);
}

//#[test]
pub fn bare_key_names() {
    let a = "
        foo = 3
        foo_3 = 3
        foo_-2--3--r23f--4-f2-4 = 3
        _ = 3
        - = 3
        8 = 8
        \"a\" = 3
        \"!\" = 3
        \"a^b\" = 3
        \"\\\"\" = 3
        \"character encoding\" = \"value\"
        'ʎǝʞ' = \"value\"
    "
    .parse::<Value>()
    .unwrap();
    &a["foo"];
    &a["-"];
    &a["_"];
    &a["8"];
    &a["foo_3"];
    &a["foo_-2--3--r23f--4-f2-4"];
    &a["a"];
    &a["!"];
    &a["\""];
    &a["character encoding"];
    &a["ʎǝʞ"];
}

//#[test]
pub fn bad_keys() {
    "key\n=3".parse::<Value>().unwrap_err();
    "key=\n3".parse::<Value>().unwrap_err();
    "key|=3".parse::<Value>().unwrap_err();
    "\"\"=3".parse::<Value>().unwrap_err();
    "=3".parse::<Value>().unwrap_err();
    "\"\"|=3".parse::<Value>().unwrap_err();
    "\"\n\"|=3".parse::<Value>().unwrap_err();
    "\"\r\"|=3".parse::<Value>().unwrap_err();
    "''''''=3".parse::<Value>().unwrap_err();
    "\"\"\"\"\"\"=3".parse::<Value>().unwrap_err();
    "'''key'''=3".parse::<Value>().unwrap_err();
    "\"\"\"key\"\"\"=3".parse::<Value>().unwrap_err();
}

//#[test]
pub fn bad_table_names() {
    "[]".parse::<Value>().unwrap_err();
    "[.]".parse::<Value>().unwrap_err();
    "[\"\".\"\"]".parse::<Value>().unwrap_err();
    "[a.]".parse::<Value>().unwrap_err();
    "[\"\"]".parse::<Value>().unwrap_err();
    "[!]".parse::<Value>().unwrap_err();
    "[\"\n\"]".parse::<Value>().unwrap_err();
    "[a.b]\n[a.\"b\"]".parse::<Value>().unwrap_err();
    "[']".parse::<Value>().unwrap_err();
    "[''']".parse::<Value>().unwrap_err();
    "['''''']".parse::<Value>().unwrap_err();
    "['''foo''']".parse::<Value>().unwrap_err();
    "[\"\"\"bar\"\"\"]".parse::<Value>().unwrap_err();
    "['\n']".parse::<Value>().unwrap_err();
    "['\r\n']".parse::<Value>().unwrap_err();
}

//#[test]
pub fn table_names() {
    let a = "
        [a.\"b\"]
        [\"f f\"]
        [\"f.f\"]
        [\"\\\"\"]
        ['a.a']
        ['\"\"']
    "
    .parse::<Value>()
    .unwrap();
    println!("{:?}", a);
    &a["a"]["b"];
    &a["f f"];
    &a["f.f"];
    &a["\""];
    &a["\"\""];
}

//#[test]
pub fn invalid_bare_numeral() {
    "4".parse::<Value>().unwrap_err();
}

//#[test]
pub fn inline_tables() {
    "a = {}".parse::<Value>().unwrap();
    "a = {b=1}".parse::<Value>().unwrap();
    "a = {   b   =   1    }".parse::<Value>().unwrap();
    "a = {a=1,b=2}".parse::<Value>().unwrap();
    "a = {a=1,b=2,c={}}".parse::<Value>().unwrap();
    "a = {a=1,}".parse::<Value>().unwrap_err();
    "a = {,}".parse::<Value>().unwrap_err();
    "a = {a=1,a=1}".parse::<Value>().unwrap_err();
    "a = {\n}".parse::<Value>().unwrap_err();
    "a = {".parse::<Value>().unwrap_err();
    "a = {a=[\n]}".parse::<Value>().unwrap();
    "a = {\"a\"=[\n]}".parse::<Value>().unwrap();
    "a = [\n{},\n{},\n]".parse::<Value>().unwrap();
}

//#[test]
pub fn number_underscores() {
    macro_rules! t {
        ($actual:expr, $expected:expr) => {{
            let f = format!("foo = {}", $actual);
            let table = f.parse::<Value>().unwrap();
            assert_eq!(table["foo"].as_integer().unwrap(), $expected);
        }};
    }

    t!("1_0", 10);
    t!("1_0_0", 100);
    t!("1_000", 1000);
    t!("+1_000", 1000);
    t!("-1_000", -1000);
}

//#[test]
pub fn bad_underscores() {
    bad!("foo = 0_", "invalid number");
    bad!("foo = 0__0", "invalid number");
    bad!("foo = __0", "invalid number");
    bad!("foo = 1_0_", "invalid number");
}

//#[test]
pub fn bad_unicode_codepoint() {
    bad!("foo = \"\\uD800\"", "invalid escape value");
}

//#[test]
pub fn bad_strings() {
    bad!("foo = \"\\uxx\"", "invalid hex escape");
    bad!("foo = \"\\u\"", "invalid hex escape");
    bad!("foo = \"\\", "unterminated");
    bad!("foo = '", "unterminated");
}

//#[test]
pub fn empty_string() {
    assert_eq!(
        "foo = \"\"".parse::<Value>().unwrap()["foo"]
            .as_str()
            .unwrap(),
        ""
    );
}

//#[test]
pub fn booleans() {
    let table = "foo = true".parse::<Value>().unwrap();
    assert_eq!(table["foo"].as_bool(), Some(true));

    let table = "foo = false".parse::<Value>().unwrap();
    assert_eq!(table["foo"].as_bool(), Some(false));

    assert!("foo = true2".parse::<Value>().is_err());
    assert!("foo = false2".parse::<Value>().is_err());
    assert!("foo = t1".parse::<Value>().is_err());
    assert!("foo = f2".parse::<Value>().is_err());
}

//#[test]
pub fn bad_nesting() {
    bad!(
        "
        a = [2]
        [[a]]
        b = 5
        ",
        "duplicate key: `a`"
    );
    bad!(
        "
        a = 1
        [a.b]
        ",
        "duplicate key: `a`"
    );
    bad!(
        "
        a = []
        [a.b]
        ",
        "duplicate key: `a`"
    );
    bad!(
        "
        a = []
        [[a.b]]
        ",
        "duplicate key: `a`"
    );
    bad!(
        "
        [a]
        b = { c = 2, d = {} }
        [a.b]
        c = 2
        ",
        "duplicate key: `b`"
    );
}

//#[test]
pub fn bad_table_redefine() {
    bad!(
        "
        [a]
        foo=\"bar\"
        [a.b]
        foo=\"bar\"
        [a]
        ",
        "redefinition of table `a`"
    );
    bad!(
        "
        [a]
        foo=\"bar\"
        b = { foo = \"bar\" }
        [a]
        ",
        "redefinition of table `a`"
    );
    bad!(
        "
        [a]
        b = {}
        [a.b]
        ",
        "duplicate key: `b`"
    );

    bad!(
        "
        [a]
        b = {}
        [a]
        ",
        "redefinition of table `a`"
    );
}

//#[test]
pub fn datetimes() {
    macro_rules! t {
        ($actual:expr) => {{
            let f = format!("foo = {}", $actual);
            let toml = f.parse::<Value>().expect(&format!("failed: {}", f));
            assert_eq!(toml["foo"].as_datetime().unwrap().to_string(), $actual);
        }};
    }

    t!("2016-09-09T09:09:09Z");
    t!("2016-09-09T09:09:09.1Z");
    t!("2016-09-09T09:09:09.2+10:00");
    t!("2016-09-09T09:09:09.123456789-02:00");
    bad!("foo = 2016-09-09T09:09:09.Z", "failed to parse date");
    bad!("foo = 2016-9-09T09:09:09Z", "failed to parse date");
    bad!("foo = 2016-09-09T09:09:09+2:00", "failed to parse date");
    bad!("foo = 2016-09-09T09:09:09-2:00", "failed to parse date");
    bad!("foo = 2016-09-09T09:09:09Z-2:00", "failed to parse date");
}

//#[test]
pub fn require_newline_after_value() {
    bad!("0=0r=false", "invalid number at line 1");
    bad!(
        r#"
0=""o=""m=""r=""00="0"q="""0"""e="""0"""
"#,
        "expected newline"
    );
    bad!(
        r#"
[[0000l0]]
0="0"[[0000l0]]
0="0"[[0000l0]]
0="0"l="0"
"#,
        "expected newline"
    );
    bad!(
        r#"
0=[0]00=[0,0,0]t=["0","0","0"]s=[1000-00-00T00:00:00Z,2000-00-00T00:00:00Z]
"#,
        "expected newline"
    );
    bad!(
        r#"
0=0r0=0r=false
"#,
        "invalid number at line 2"
    );
    bad!(
        r#"
0=0r0=0r=falsefal=false
"#,
        "invalid number at line 2"
    );
}
