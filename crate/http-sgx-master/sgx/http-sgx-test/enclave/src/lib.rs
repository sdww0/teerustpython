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
use std::panic;
use sgx_tunittest::*;

extern crate http;
extern crate rand;
extern crate quickcheck;

mod status_code;
mod header_map;
mod header_map_fuzz;
mod response;
mod request;
mod error;
mod method;
mod uri;
mod extensions;

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
status_code::from_bytes,
status_code::equates_with_u16,
header_map::smoke,
header_map::drain,
header_map::drain_entry,
header_map::eq,
header_map::into_header_name,
header_map::as_header_name,
header_map::insert_all_std_headers,
header_map::insert_79_custom_std_headers,
header_map::append_multiple_values,
header_map::get_invalid,
|| should_panic!(header_map::insert_invalid()),
header_map::value_htab,
|| should_panic!(header_map::reserve_over_capacity()),
|| should_panic!(header_map::reserve_overflow()),
header_map::drain_drop_immediately,
header_map::drain_forget,
header_map_fuzz::header_map_fuzz,
response::it_can_map_a_body_from_one_type_to_another,
request::it_can_map_a_body_from_one_type_to_another,
error::inner_error_is_invalid_status_code,
method::test_method_eq,
method::test_invalid_method,
uri::tests::test_char_table,
uri::tests::test_uri_parse_error,
uri::tests::test_max_uri_len,
uri::tests::test_uri_parse_long_host_with_no_scheme,
uri::tests::test_uri_to_path_and_query,
uri::tests::test_authority_uri_parts_round_trip,
uri::tests::test_partial_eq_path_with_terminating_questionmark,
uri::tests::test_uri_parse_long_host_with_port_and_no_scheme,
uri::tests::test_overflowing_scheme,
uri::tests::test_max_length_scheme,
uri::port::partialeq_port,
uri::port::partialeq_port_different_reprs,
uri::port::partialeq_u16,
uri::port::u16_from_port,
uri::path::equal_to_self_of_same_path,
uri::path::not_equal_to_self_of_different_path,
uri::path::equates_with_a_str,
uri::path::not_equal_with_a_str_of_a_different_path,
uri::path::equates_with_a_string,
uri::path::not_equal_with_a_string_of_a_different_path,
uri::path::compares_to_self,
uri::path::compares_with_a_str,
uri::path::compares_with_a_string,
uri::path::ignores_valid_percent_encodings,
uri::path::ignores_invalid_percent_encodings,
uri::authority::parse_empty_string_is_error,
uri::authority::equal_to_self_of_same_authority,
uri::authority::not_equal_to_self_of_different_authority,
uri::authority::equates_with_a_str,
uri::authority::not_equal_with_a_str_of_a_different_authority,
uri::authority::equates_with_a_string,
uri::authority::equates_with_a_string_of_a_different_authority,
uri::authority::compares_to_self,
uri::authority::compares_with_a_str,
uri::authority::compares_with_a_string,
uri::authority::allows_percent_in_userinfo,
uri::authority::rejects_percent_in_hostname,
uri::tests::test_uri_parse_path_and_query,
uri::tests::test_uri_parse_absolute_form,
uri::tests::test_uri_parse_absolute_form_without_path,
uri::tests::test_uri_parse_asterisk_form,
uri::tests::test_uri_parse_authority_no_port,
uri::tests::test_uri_authority_only_one_character_issue_197,
uri::tests::test_uri_parse_authority_form,
uri::tests::test_uri_parse_absolute_with_default_port_http,
uri::tests::test_uri_parse_absolute_with_default_port_https,
uri::tests::test_uri_parse_fragment_questionmark,
uri::tests::test_uri_parse_path_with_terminating_questionmark,
uri::tests::test_uri_parse_absolute_form_with_empty_path_and_nonempty_query,
uri::tests::test_uri_parse_absolute_form_with_empty_path_and_fragment_with_slash,
uri::tests::test_uri_parse_absolute_form_with_empty_path_and_fragment_with_questionmark,
uri::tests::test_userinfo1,
uri::tests::test_userinfo2,
uri::tests::test_userinfo3,
uri::tests::test_userinfo_with_port,
uri::tests::test_userinfo_pass_with_port,
uri::tests::test_ipv6,
uri::tests::test_ipv6_shorthand,
uri::tests::test_ipv6_shorthand2,
uri::tests::test_ipv6_shorthand3,
uri::tests::test_ipv6_with_port,
uri::tests::test_percentage_encoded_path,
uri::tests::test_path_permissive,
uri::tests::test_query_permissive,
extensions::test_extensions,
);

    sgx_status_t::SGX_SUCCESS
}
