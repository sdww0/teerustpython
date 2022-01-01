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

#[cfg(target_env = "sgx")]
extern crate core;

use sgx_types::*;
use std::prelude::v1::*;
use sgx_tunittest::*;

extern crate parity_wasm;

mod io;
mod builder;
mod elements;

#[no_mangle]
pub extern "C" fn say_something(_some_string: *const u8, _some_len: usize) -> sgx_status_t {

    rsgx_unit_tests!(io::cursor,
                     io::overflow_in_cursor,
                     builder::code::example,
                     builder::code::func_example,
                     builder::export::example,
                     builder::import::example,
                     builder::global::example,
                     builder::module::smoky,
                     builder::module::functions,
                     builder::module::export,
                     builder::module::global,
                     builder::module::data,
                     builder::module::reuse_types,
elements::index_map::default_is_empty_no_matter_how_we_look_at_it,
elements::index_map::with_capacity_creates_empty_map,
elements::index_map::clear_removes_all_values,
elements::index_map::get_returns_elements_that_are_there_but_nothing_else,
elements::index_map::contains_key_returns_true_when_a_key_is_present,
elements::index_map::insert_behaves_like_other_maps,
elements::index_map::remove_behaves_like_other_maps,
elements::index_map::partial_eq_works_as_expected_in_simple_cases,
elements::index_map::partial_eq_is_smart_about_none_values_at_the_end,
elements::index_map::from_iterator_builds_a_map,
elements::index_map::iterators_are_well_behaved,
elements::index_map::serialize_and_deserialize,
elements::index_map::deserialize_requires_elements_to_be_in_order,
elements::index_map::deserialize_enforces_max_idx,
elements::module::hello,
elements::module::serde,
elements::module::serde_type,
elements::module::serde_import,
elements::module::serde_code,
elements::module::const_,
elements::module::store,
elements::module::peek,
elements::module::peek_2,
elements::module::peek_3,
elements::module::module_default_round_trip,
elements::module::names,
elements::module::wrong_varuint1_case,
elements::module::memory_space,
elements::name_section::serialize_module_name,
elements::name_section::serialize_function_names,
elements::name_section::serialize_local_names,
elements::name_section::serialize_all_subsections,
elements::name_section::deserialize_local_names,
elements::ops::ifelse,
elements::ops::display,
elements::ops::size_off,
elements::primitives::varuint32_0,
elements::primitives::varuint32_1,
elements::primitives::varuint32_135,
elements::primitives::varuint32_8192,
elements::primitives::varint32_8192,
elements::primitives::varint32_neg_8192,
elements::primitives::varuint64_0,
elements::primitives::varuint64_1,
elements::primitives::varuint64_135,
elements::primitives::varuint64_8192,
elements::primitives::varint64_8192,
elements::primitives::varint64_neg_8192,
elements::primitives::varint64_min,
elements::primitives::varint64_bad_extended,
elements::primitives::varint32_bad_extended,
elements::primitives::varint32_bad_extended2,
elements::primitives::varint64_max,
elements::primitives::varint64_too_long,
elements::primitives::varint32_too_long,
elements::primitives::varuint64_too_long,
elements::primitives::varuint32_too_long,
elements::primitives::varuint32_too_long_trailing,
elements::primitives::varuint64_too_long_trailing,
elements::primitives::varint32_min,
elements::primitives::varint7_invalid,
elements::primitives::varint7_neg,
elements::primitives::varuint32_too_long_nulled,
elements::primitives::varint32_max,
elements::primitives::counted_list,
elements::reloc_section::reloc_section,
elements::section::import_section,
elements::section::fn_section_detect,
elements::section::fn_section_number,
elements::section::fn_section_ref,
elements::section::type_section_len,
elements::section::type_section_infer,
elements::section::export_detect,
elements::section::code_detect,
elements::section::data_section_ser,
elements::section::data_section_detect,
elements::section::element_section_ser,
elements::section::code_section_ser,
elements::section::start_section
);

    sgx_status_t::SGX_SUCCESS
}
