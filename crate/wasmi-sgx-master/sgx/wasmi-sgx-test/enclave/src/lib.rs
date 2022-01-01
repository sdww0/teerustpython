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

#![crate_name = "wasmienclave"]
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
use std::panic;
use std::prelude::v1::*;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate wasmi;
extern crate wasmi_validation;
extern crate parity_wasm;
extern crate assert_matches;
extern crate wabt;
extern crate rand;
#[macro_use]
extern crate serde;
extern crate typetag;

mod tests;
mod module;
mod prepare;
mod nan_preserving_float;
//mod host;
mod memory;

use wasmi::Error;

extern {
    fn ocall_wat2wasm(ret_val : * mut sgx_status_t,
                      ptr_in  : * const u8,
                      len_in  : usize,
                      ptr_out : * mut u8,
                      max_out : usize,
                      len_out : * mut usize) -> sgx_status_t;
}

fn fake_wat2wasm<S: AsRef<[u8]>>(source: S) -> Result<Vec<u8>, Error> {
    let ptr_in = source.as_ref().as_ptr();
    let len_in = source.as_ref().len();
    let max_out = 1024 * 1024;
    let mut result_buf: Vec<u8> = vec![0;max_out];
    let ptr_out = result_buf.as_mut_ptr();
    let mut len_out = 0;
    let mut return_status: sgx_status_t = sgx_status_t::SGX_SUCCESS;

    match unsafe {
        ocall_wat2wasm(
            &mut return_status,
            ptr_in,
            len_in,
            ptr_out,
            max_out,
            &mut len_out)
    } {
        sgx_status_t::SGX_SUCCESS => {
            // println!("fake_wat2wasm ocall successfully returned");
        }
        x => {
            println!("fake_wat2wasm ocall return failed! {:?}", x);
            return Err(Error::Validation("Cannot convert wat2wasm".to_string()));
        }
    }

    unsafe {
        result_buf.set_len(len_out);
    }
    Ok(result_buf)
}

fn test_wat2wasm() {
    let s = r#"
            (module
                (import "foo" "bar" (func))
                )
            "#;
    let r = fake_wat2wasm(s);

    println!("{:?}", r);
}

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
tests::assert_error_properties,
tests::unsigned_to_runtime_value,
tests::host::call_host_func,
tests::host::resume_call_host_func,
tests::host::resume_call_host_func_type_mismatch,
tests::host::host_err,
tests::host::modify_mem_with_host_funcs,
tests::host::pull_internal_mem_from_module,
tests::host::recursion,
tests::host::defer_providing_externals,
tests::host::two_envs_one_externals,
tests::host::dynamically_add_host_func,
tests::wasm::interpreter_inc_i32,
tests::wasm::interpreter_accumulate_u8,
|| should_panic!(module::assert_no_start_panics_on_module_with_start()),
module::imports_provided_by_externvals,
prepare::implicit_return_no_value,
prepare::implicit_return_with_value,
prepare::implicit_return_param,
prepare::get_local,
prepare::explicit_return,
prepare::add_params,
prepare::drop_locals,
prepare::if_without_else,
prepare::if_else,
prepare::if_else_returns_result,
prepare::if_else_branch_from_true_branch,
prepare::if_else_branch_from_false_branch,
prepare::loop_,
prepare::loop_empty,
prepare::spec_as_br_if_value_cond,
prepare::brtable,
prepare::brtable_returns_result,
prepare::wabt_example,
nan_preserving_float::test_ops_f32,
nan_preserving_float::test_ops_f64,
nan_preserving_float::test_neg_nan_f32,
nan_preserving_float::test_neg_nan_f64,
//host::i32_runtime_args,
//host::i64_invalid_arg_cast,
memory::alloc,
memory::ensure_page_size,
memory::copy_overlaps_1,
memory::copy_overlaps_2,
memory::copy_nonoverlapping,
memory::copy_nonoverlapping_overlaps_1,
memory::copy_nonoverlapping_overlaps_2,
memory::transfer_works,
memory::transfer_still_works_with_same_memory,
memory::transfer_oob_with_same_memory_errors,
memory::transfer_oob_errors,
memory::clear,
memory::get_into,
memory::zero_copy,
|| should_panic!(memory::zero_copy_panics_on_nested_access())
);
    test_wat2wasm();
    sgx_status_t::SGX_SUCCESS
}
