// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(target_env = "sgx")]
extern crate core;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;
use std::panic;

extern crate uuid;
extern crate serde_test;

mod test_util;
mod lib_test;
mod core_support;
mod v1;
mod v3;
mod v4;
mod v5;
mod adapter;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
lib_test::test_uuid_compare,
lib_test::test_uuid_default,
lib_test::test_uuid_display,
lib_test::test_uuid_lowerhex,
lib_test::test_uuid_operator_eq,
lib_test::test_uuid_to_string,
lib_test::test_uuid_upperhex,
lib_test::test_nil,
lib_test::test_predefined_namespaces,
lib_test::test_get_version_v3,
lib_test::test_get_variant,
lib_test::test_to_simple_string,
lib_test::test_to_hyphenated_string,
lib_test::test_upper_lower_hex,
lib_test::test_to_urn_string,
lib_test::test_to_simple_string_matching,
lib_test::test_string_roundtrip,
lib_test::test_from_fields,
lib_test::test_from_fields_le,
lib_test::test_as_fields,
lib_test::test_fields_roundtrip,
lib_test::test_fields_le_roundtrip,
lib_test::test_fields_le_are_actually_le,
lib_test::test_from_u128,
lib_test::test_from_u128_le,
lib_test::test_u128_roundtrip,
lib_test::test_u128_le_roundtrip,
lib_test::test_u128_le_is_actually_le,
lib_test::test_from_slice,
lib_test::test_from_bytes,
lib_test::test_as_bytes,
lib_test::test_bytes_roundtrip,
lib_test::test_iterbytes_impl_for_uuid,
core_support::test_uuid_compare,
core_support::test_uuid_default,
core_support::test_uuid_display,
core_support::test_uuid_lowerhex,
core_support::test_uuid_operator_eq,
core_support::test_uuid_to_string,
core_support::test_uuid_upperhex,
v1::test_new_v1,
v3::test_new,
v3::test_to_hyphenated_string,
v4::test_new,
v4::test_get_version,
v5::test_get_version,
v5::test_hyphenated,
v5::test_new,
adapter::hyphenated_trailing,
adapter::hyphenated_ref_trailing,
adapter::simple_trailing,
adapter::simple_ref_trailing,
adapter::urn_trailing,
adapter::urn_ref_trailing,
|| should_panic!(adapter::hyphenated_too_small()),
|| should_panic!(adapter::hyphenated_ref_too_small()),
|| should_panic!(adapter::simple_too_small()),
|| should_panic!(adapter::simple_ref_too_small()),
|| should_panic!(adapter::urn_too_small()),
|| should_panic!(adapter::urn_ref_too_small()),
adapter::compact::test_serialize_compact,
);

    sgx_status_t::SGX_SUCCESS
}
