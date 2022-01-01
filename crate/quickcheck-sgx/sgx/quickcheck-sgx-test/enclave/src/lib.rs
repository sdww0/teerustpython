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

extern crate quickcheck;
extern crate rand;

mod tests;
mod tester;
mod arbitrary;

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
tests::prop_oob,
tests::prop_reverse_reverse,
tests::reverse_single,
tests::reverse_app,
tests::max,
tests::sort,
tests::testable_result,
tests::testable_unit,
tests::testable_unit_panic,
tests::regression_issue_83,
tests::regression_issue_83_signed,
tests::all_tests_discarded_min_tests_passed_missing,
|| should_panic!(tests::sieve_not_prime()),
|| should_panic!(tests::sieve_not_all_primes()),
|| should_panic!(tests::testable_result_err()),
|| should_panic!(tests::panic_msg_1()),
|| should_panic!(tests::panic_msg_2()),
|| should_panic!(tests::panic_msg_3()),
|| should_panic!(tests::regression_issue_107_hang()),
|| should_panic!(tests::all_tests_discarded_min_tests_passed_set()),
tests::pathbuf,
tests::basic_hashset,
tests::basic_hashmap,
tests::substitute_hashset,
tests::substitute_hashmap,
tests::prop_reverse_reverse_macro,
|| should_panic!(tests::prop_macro_panic()),
arbitrary::arby_unit,
arbitrary::arby_int,
arbitrary::arby_uint,
arbitrary::unit,
arbitrary::bools,
arbitrary::options,
arbitrary::results,
arbitrary::tuples,
arbitrary::triples,
arbitrary::quads,
arbitrary::ints,
arbitrary::ints8,
arbitrary::ints16,
arbitrary::ints32,
arbitrary::ints64,
arbitrary::ints128,
arbitrary::uints,
arbitrary::uints8,
arbitrary::uints16,
arbitrary::uints32,
arbitrary::uints64,
arbitrary::uints128,
arbitrary::floats32,
arbitrary::floats64,
arbitrary::wrapping_ints32,
arbitrary::vecs,
arbitrary::binaryheaps,
arbitrary::chars,
arbitrary::bounds,
arbitrary::ranges,
arbitrary::pathbuf,
arbitrary::btreemap,
arbitrary::hashmap,
arbitrary::btreesets,
arbitrary::hashsets,
arbitrary::linkedlists,
arbitrary::vecdeques,
tester::shrinking_regression_issue_126,
tester::size_for_small_types_issue_143,
tester::different_generator,
);

    sgx_status_t::SGX_SUCCESS
}
