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

#![recursion_limit = "256"]

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

#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde;
extern crate serde_json;

mod enum_external_deserialize;
mod tests;

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
enum_external_deserialize::invalid_variant_returns_error_with_good_message_string,
enum_external_deserialize::invalid_variant_returns_error_with_good_message_inline_table,
enum_external_deserialize::extra_field_returns_expected_empty_table_error,
enum_external_deserialize::extra_field_returns_expected_empty_table_error_struct_variant,
enum_external_deserialize::enum_unit::from_str,
enum_external_deserialize::enum_unit::from_inline_table,
enum_external_deserialize::enum_unit::from_dotted_table,
enum_external_deserialize::enum_tuple::from_inline_table,
enum_external_deserialize::enum_tuple::from_dotted_table,
enum_external_deserialize::enum_newtype::from_inline_table,
enum_external_deserialize::enum_newtype::from_inline_table,
enum_external_deserialize::enum_struct::from_inline_table,
enum_external_deserialize::enum_struct::from_dotted_table,
enum_external_deserialize::enum_array::from_inline_tables,
tests::backcompat::newlines_after_tables,
tests::backcompat::allow_duplicate_after_longer,
tests::datetime::times,
tests::datetime::bad_times,
tests::display::simple_show,
tests::display::table,
tests::display_tricky::both_ends,
tests::float::float_inf,
tests::formatting::no_unnecessary_newlines_array,
tests::formatting::no_unnecessary_newlines_table,
tests::invalid_misc::bad,
tests::invalid::array_mixed_types_arrays_and_ints,
tests::invalid::array_mixed_types_ints_and_floats,
tests::invalid::array_mixed_types_strings_and_ints,
tests::invalid::datetime_malformed_no_leads,
tests::invalid::datetime_malformed_no_secs,
tests::invalid::datetime_malformed_no_t,
tests::invalid::datetime_malformed_with_milli,
tests::invalid::duplicate_key_table,
tests::invalid::duplicate_keys,
tests::invalid::duplicate_table,
tests::invalid::duplicate_tables,
tests::invalid::empty_implicit_table,
tests::invalid::empty_table,
tests::invalid::float_no_leading_zero,
tests::invalid::float_no_suffix,
tests::invalid::float_no_trailing_digits,
tests::invalid::key_after_array,
tests::invalid::key_after_table,
tests::invalid::key_empty,
tests::invalid::key_hash,
tests::invalid::key_newline,
tests::invalid::key_open_bracket,
tests::invalid::key_single_open_bracket,
tests::invalid::key_space,
tests::invalid::key_start_bracket,
tests::invalid::key_two_equals,
tests::invalid::string_bad_byte_escape,
tests::invalid::string_bad_escape,
tests::invalid::string_bad_line_ending_escape,
tests::invalid::string_byte_escapes,
tests::invalid::string_no_close,
tests::invalid::table_array_implicit,
tests::invalid::table_array_malformed_bracket,
tests::invalid::table_array_malformed_empty,
tests::invalid::table_empty,
tests::invalid::table_nested_brackets_close,
tests::invalid::table_nested_brackets_open,
tests::invalid::table_whitespace,
tests::invalid::table_with_pound,
tests::invalid::text_after_array_entries,
tests::invalid::text_after_integer,
tests::invalid::text_after_string,
tests::invalid::text_after_table,
tests::invalid::text_before_array_separator,
tests::invalid::text_in_array,
tests::macros::test_cargo_toml,
tests::macros::test_array,
tests::macros::test_number,
tests::macros::test_nan,
tests::macros::test_datetime,
tests::macros::test_quoted_key,
tests::macros::test_empty,
tests::macros::test_dotted_keys,
tests::parser::crlf,
tests::parser::fun_with_strings,
tests::parser::tables_in_arrays,
tests::parser::empty_table,
tests::parser::fruit,
tests::parser::stray_cr,
tests::parser::blank_literal_string,
tests::parser::many_blank,
tests::parser::literal_eats_crlf,
tests::parser::string_no_newline,
tests::parser::bad_leading_zeros,
tests::parser::bad_floats,
tests::parser::floats,
tests::parser::bare_key_names,
tests::parser::bad_keys,
tests::parser::bad_table_names,
tests::parser::table_names,
tests::parser::invalid_bare_numeral,
tests::parser::inline_tables,
tests::parser::number_underscores,
tests::parser::bad_underscores,
tests::parser::bad_unicode_codepoint,
tests::parser::bad_strings,
tests::parser::empty_string,
tests::parser::booleans,
tests::parser::bad_nesting,
tests::parser::bad_table_redefine,
tests::parser::datetimes,
tests::parser::require_newline_after_value,
tests::pretty::no_pretty,
tests::pretty::disable_pretty,
tests::pretty::pretty_std,
tests::pretty::pretty_indent_2,
tests::pretty::pretty_indent_2_other,
tests::pretty::pretty_indent_array_no_comma,
tests::pretty::pretty_no_string,
tests::pretty::pretty_tricky,
tests::pretty::pretty_table_array,
tests::pretty::table_array,
tests::pretty::pretty_tricky_non_literal,
tests::serde::smoke,
tests::serde::smoke_hyphen,
tests::serde::nested,
tests::serde::application_decode_error,
tests::serde::array,
tests::serde::inner_structs_with_options,
tests::serde::hashmap,
tests::serde::table_array,
tests::serde::type_errors,
tests::serde::missing_errors,
tests::serde::parse_enum,
tests::serde::parse_enum_string,
tests::serde::empty_arrays,
tests::serde::empty_arrays2,
tests::serde::extra_keys,
tests::serde::newtypes,
tests::serde::newtypes2,
tests::serde::table_structs_empty,
tests::serde::fixed_size_array,
tests::serde::homogeneous_tuple,
tests::serde::homogeneous_tuple_struct,
tests::serde::json_interoperability,
tests::spanned::good_datetimes,
tests::spanned::test_spanned_field,
tests::tables_last::always_works,
tests::valid::array_empty,
tests::valid::array_nospaces,
tests::valid::arrays_hetergeneous,
tests::valid::arrays,
tests::valid::arrays_nested,
tests::valid::empty,
tests::valid::bool,
tests::valid::comments_everywhere,
tests::valid::datetime,
tests::valid::example,
tests::valid::float,
tests::valid::implicit_and_explicit_after,
tests::valid::implicit_and_explicit_before,
tests::valid::implicit_groups,
tests::valid::integer,
tests::valid::key_equals_nospace,
tests::valid::key_space,
tests::valid::key_special_chars,
tests::valid::key_with_pound,
tests::valid::long_float,
tests::valid::long_integer,
tests::valid::multiline_string,
tests::valid::raw_multiline_string,
tests::valid::raw_string,
tests::valid::string_empty,
tests::valid::string_escapes,
tests::valid::string_simple,
tests::valid::string_with_pound,
tests::valid::table_array_implicit,
tests::valid::table_array_many,
tests::valid::table_array_nest,
tests::valid::table_array_one,
tests::valid::table_empty,
tests::valid::table_sub_empty,
tests::valid::table_multi_empty,
tests::valid::table_whitespace,
tests::valid::table_with_pound,
tests::valid::unicode_escape,
tests::valid::unicode_literal,
tests::valid::hard_example,
tests::valid::example2,
tests::valid::example3,
tests::valid::example4,
tests::valid::example_bom,
tests::valid::datetime_truncate,
tests::valid::key_quote_newline,
tests::valid::table_array_nest_no_keys,
tests::valid::dotted_keys,
tests::valid::quote_surrounded_value,
);

    sgx_status_t::SGX_SUCCESS
}
