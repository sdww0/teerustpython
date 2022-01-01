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
use sgx_tunittest::*;

extern crate snap;
extern crate quickcheck;
extern crate rand;

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
tests::empty::roundtrip_raw,
tests::empty::roundtrip_frame,
tests::one_zero::roundtrip_raw,
tests::one_zero::roundtrip_frame,
tests::data_html::roundtrip_raw,
tests::data_html::roundtrip_frame,
tests::data_urls::roundtrip_raw,
tests::data_urls::roundtrip_frame,
tests::data_jpg::roundtrip_raw,
tests::data_jpg::roundtrip_frame,
tests::data_pdf::roundtrip_raw,
tests::data_pdf::roundtrip_frame,
tests::data_html4::roundtrip_raw,
tests::data_html4::roundtrip_frame,
tests::data_txt1::roundtrip_raw,
tests::data_txt1::roundtrip_frame,
tests::data_txt2::roundtrip_raw,
tests::data_txt2::roundtrip_frame,
tests::data_txt3::roundtrip_raw,
tests::data_txt3::roundtrip_frame,
tests::data_txt4::roundtrip_raw,
tests::data_txt4::roundtrip_frame,
tests::data_pb::roundtrip_raw,
tests::data_pb::roundtrip_frame,
tests::data_gaviota::roundtrip_raw,
tests::data_gaviota::roundtrip_frame,
tests::data_golden::roundtrip_raw,
tests::data_golden::roundtrip_frame,
tests::data_golden_rev,
tests::small_copy,
tests::small_regular,
tests::decompress_copy_close_to_end_1,
tests::decompress_copy_close_to_end_2,
tests::err_empty,
tests::err_header_mismatch,
tests::err_varint1,
tests::err_varint2,
tests::err_varint3,
tests::err_lit,
tests::err_lit_big1,
tests::err_lit_big2a,
tests::err_lit_big2b,
tests::err_copy1,
tests::err_copy2a,
tests::err_copy2b,
tests::err_copy3a,
tests::err_copy3b,
tests::err_copy3c,
tests::err_copy3d,
tests::err_copy_offset_zero,
tests::err_copy_offset_big,
tests::err_copy_len_big,
tests::random1::roundtrip_raw,
tests::random1::roundtrip_frame,
tests::random2::roundtrip_raw,
tests::random2::roundtrip_frame,
tests::random3::roundtrip_raw,
tests::random3::roundtrip_frame,
tests::random4::roundtrip_raw,
tests::random4::roundtrip_frame,
tests::qc_roundtrip,
tests::qc_roundtrip_stream,
tests::err_lit_len_overflow2,
);
    sgx_status_t::SGX_SUCCESS
}
