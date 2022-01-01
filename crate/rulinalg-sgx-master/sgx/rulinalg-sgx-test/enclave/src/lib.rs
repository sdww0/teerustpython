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

#[macro_use]
extern crate rulinalg;
extern crate num as libnum;
#[macro_use]
extern crate quickcheck;
//#[macro_use]
extern crate itertools;
#[macro_use]
extern crate serde;

mod testsupport;
mod mat;
mod matrix;
mod convert;
mod vector;
mod internal_utils;
mod ulp;
mod macros;
mod norm;

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
mat::test_solve,
mat::test_l_triangular_solve_errs,
mat::test_u_triangular_solve_errs,
mat::matrix_lup_decomp,
mat::matrix_partial_piv_lu,
matrix::column_clone_into_slice,
matrix::column_mut_clone_into_slice,
matrix::column_mut_clone_from_slice,
matrix::mat_mul::matrix_mul_f32,
matrix::mat_mul::matrix_mul_f64,
matrix::mat_mul::matrix_mul_usize,
matrix::mat_mul::mul_slice_basic,
matrix::mat_mul::mul_slice_uneven_data,
matrix::mat_mul::mul_slice_uneven_data_usize,
matrix::impl_ops::indexing_mat,
matrix::impl_ops::matrix_vec_mul,
matrix::impl_ops::matrix_f32_mul,
matrix::impl_ops::matrix_add,
matrix::impl_ops::matrix_f32_add,
matrix::impl_ops::matrix_sub,
matrix::impl_ops::matrix_f32_sub,
matrix::impl_ops::matrix_f32_div,
matrix::impl_ops::add_slice,
matrix::impl_ops::sub_slice,
matrix::impl_ops::div_slice,
matrix::impl_ops::neg_slice,
matrix::impl_ops::index_slice,
matrix::impl_ops::matrix_add_assign,
matrix::impl_ops::matrix_sub_assign,
matrix::impl_ops::matrix_div_assign,
matrix::impl_ops::matrix_mul_assign,
matrix::impl_ops::slice_add_assign,
matrix::impl_ops::slice_sub_assign,
matrix::impl_ops::slice_div_assign,
matrix::impl_ops::slice_mul_assign,
|| should_panic!(matrix::slice::make_slice_bad_dim()),
matrix::slice::make_slice,
matrix::slice::make_slice_mut,
matrix::slice::matrix_min_max,
matrix::iter::test_diag_offset_equivalence,
matrix::iter::test_matrix_diag,
matrix::iter::test_empty_matrix_diag,
matrix::iter::test_matrix_slice_diag,
matrix::iter::test_matrix_diag_nth,
matrix::iter::test_matrix_slice_diag_nth,
matrix::iter::test_matrix_diag_last,
matrix::iter::test_matrix_slice_diag_last,
matrix::iter::test_matrix_diag_count,
matrix::iter::test_matrix_diag_size_hint,
matrix::iter::test_matrix_cols,
matrix::iter::test_matrix_slice_cols,
matrix::iter::test_matrix_slice_mut_cols,
matrix::iter::test_matrix_cols_nth,
matrix::iter::test_matrix_cols_last,
matrix::iter::test_matrix_cols_count,
matrix::iter::test_matrix_cols_size_hint,
matrix::iter::test_matrix_rows,
matrix::iter::test_matrix_slice_rows,
matrix::iter::test_matrix_slice_mut_rows,
matrix::iter::test_matrix_rows_nth,
matrix::iter::test_matrix_rows_last,
matrix::iter::test_matrix_rows_count,
matrix::iter::test_matrix_rows_size_hint,
matrix::iter::into_iter_compile,
matrix::iter::into_iter_mut_compile,
matrix::iter::iter_matrix_small_matrices,
matrix::iter::iter_matrix_slice,
matrix::iter::iter_empty_matrix,
matrix::decomposition::householder::compute_empty_vector,
matrix::decomposition::householder::compute_single_element_vector,
matrix::decomposition::householder::compute_examples,
matrix::decomposition::householder::householder_reflection_left_multiply,
matrix::decomposition::householder::householder_composition_left_multiply,
matrix::decomposition::householder::householder_composition_first_k_columns,
|| should_panic!(matrix::decomposition::lu::test_non_square_lup_decomp()),
matrix::decomposition::lu::test_lup_decomp,
matrix::decomposition::lu::partial_piv_lu_decompose_arbitrary,
matrix::decomposition::lu::partial_piv_lu_inverse_identity,
matrix::decomposition::lu::partial_piv_lu_inverse_arbitrary_invertible_matrix,
matrix::decomposition::lu::partial_piv_lu_det_identity,
matrix::decomposition::lu::partial_piv_lu_det_arbitrary_invertible_matrix,
matrix::decomposition::lu::partial_piv_lu_solve_arbitrary_matrix,
matrix::decomposition::lu::lu_forward_substitution,
matrix::decomposition::lu::full_piv_lu_decompose_arbitrary,
matrix::decomposition::lu::full_piv_lu_decompose_singular,
|| should_panic!(matrix::decomposition::lu::full_piv_lu_decompose_rectangular()),
matrix::decomposition::lu::full_piv_lu_solve_arbitrary_matrix,
matrix::decomposition::lu::full_piv_lu_inverse_arbitrary_invertible_matrix,
matrix::decomposition::lu::full_piv_lu_inverse_noninvertible,
matrix::decomposition::lu::full_piv_lu_empty_matrix,
|| should_panic!(matrix::decomposition::cholesky::test_non_square_cholesky()),
matrix::decomposition::cholesky::cholesky_unpack_empty,
matrix::decomposition::cholesky::cholesky_unpack_1x1,
matrix::decomposition::cholesky::cholesky_unpack_2x2,
matrix::decomposition::cholesky::cholesky_singular_fails,
matrix::decomposition::cholesky::cholesky_det_empty,
matrix::decomposition::cholesky::cholesky_det,
matrix::decomposition::cholesky::cholesky_solve_examples,
matrix::decomposition::cholesky::cholesky_inverse_examples,
matrix::decomposition::cholesky::transpose_back_substitution_examples,
matrix::decomposition::cholesky::property_cholesky_of_identity_is_identity,
matrix::decomposition::svd::test_sort_svd,
matrix::decomposition::svd::test_svd_tall_matrix,
matrix::decomposition::svd::test_svd_short_matrix,
matrix::decomposition::svd::test_svd_square_matrix,
|| should_panic!(matrix::decomposition::hessenberg::test_non_square_upper_hessenberg()),
|| should_panic!(matrix::decomposition::hessenberg::test_non_square_upper_hess_decomp()),
matrix::decomposition::eigen::test_1_by_1_matrix_eigenvalues,
matrix::decomposition::eigen::test_2_by_2_matrix_eigenvalues,
matrix::decomposition::eigen::test_2_by_2_matrix_zeros_eigenvalues,
matrix::decomposition::eigen::test_2_by_2_matrix_complex_eigenvalues,
matrix::decomposition::eigen::test_2_by_2_matrix_eigendecomp,
matrix::decomposition::eigen::test_3_by_3_eigenvals,
matrix::decomposition::eigen::test_5_by_5_eigenvals,
matrix::decomposition::qr::householder_qr_unpack_reconstruction,
matrix::decomposition::qr::householder_qr_unpack_square_reconstruction_corner_cases,
matrix::decomposition::qr::householder_qr_unpack_thin_reconstruction,
|| should_panic!(matrix::decomposition::eigen::test_non_square_eigenvalues()),
|| should_panic!(matrix::decomposition::eigen::test_non_square_eigendecomp()),
matrix::impl_permutation_mul::permutation_vector_mul,
matrix::impl_permutation_mul::permutation_matrix_left_mul_for_matrix,
matrix::impl_permutation_mul::permutation_matrix_left_mul_for_matrix_slice,
matrix::impl_permutation_mul::permutation_matrix_right_mul_for_matrix,
matrix::impl_permutation_mul::permutation_matrix_right_mul_for_matrix_slice,
matrix::impl_permutation_mul::permutation_matrix_self_multiply,
matrix::base::impl_base::test_sub_slice,
matrix::base::impl_base::slice_into_matrix,
matrix::base::impl_base::test_split_matrix,
matrix::base::impl_base::test_split_matrix_mut,
|| should_panic!(matrix::base::impl_base::test_diag_iter_too_high()),
|| should_panic!(matrix::base::impl_base::test_diag_iter_too_low()),
matrix::base::impl_base::test_swap_rows,
matrix::base::impl_base::test_matrix_swap_cols,
matrix::base::impl_base::test_matrix_swap_same_rows,
matrix::base::impl_base::test_matrix_swap_same_cols,
|| should_panic!(matrix::base::impl_base::test_matrix_swap_row_high_first()),
|| should_panic!(matrix::base::impl_base::test_matrix_swap_row_high_second()),
|| should_panic!(matrix::base::impl_base::test_matrix_swap_col_high_first()),
|| should_panic!(matrix::base::impl_base::test_matrix_swap_col_high_second()),
matrix::base::impl_base::test_matrix_select_rows,
matrix::base::impl_base::test_matrix_select_cols,
matrix::base::impl_base::test_matrix_select,
matrix::base::impl_base::matrix_diag,
matrix::base::impl_base::transpose_mat,
matrix::permutation_matrix::swap_rows,
matrix::permutation_matrix::as_matrix,
matrix::permutation_matrix::from_array,
matrix::permutation_matrix::from_array_unchecked,
matrix::permutation_matrix::from_array_invalid,
matrix::permutation_matrix::vec_from_permutation,
matrix::permutation_matrix::into_slice_ref,
matrix::permutation_matrix::map_row,
matrix::permutation_matrix::inverse,
matrix::permutation_matrix::parity,
matrix::permutation_matrix::det,
matrix::permutation_matrix::permute_by_swap_on_empty_array,
matrix::permutation_matrix::permute_by_swap_on_arbitrary_array,
matrix::permutation_matrix::permute_by_swap_identity_on_arbitrary_array,
matrix::permutation_matrix::compose_into_buffer,
matrix::permutation_matrix::compose_regression,
matrix::permutation_matrix::permute_rows_into_buffer,
matrix::permutation_matrix::permute_rows_in_place,
matrix::permutation_matrix::permute_cols_into_buffer,
matrix::permutation_matrix::permute_cols_in_place,
matrix::permutation_matrix::permute_vector_into_buffer,
matrix::permutation_matrix::permute_vector_in_place,
matrix::impl_mat::test_new_mat,
|| should_panic!(matrix::impl_mat::test_new_mat_bad_data()),
matrix::impl_mat::test_new_mat_from_fn,
matrix::impl_mat::test_equality,
matrix::impl_mat::test_new_from_slice,
matrix::impl_mat::test_display_formatting,
matrix::impl_mat::test_single_row_display_formatting,
matrix::impl_mat::test_display_formatting_precision,
matrix::impl_mat::test_matrix_index_mut,
matrix::impl_mat::test_matrix_select_rows,
matrix::impl_mat::test_matrix_select_cols,
matrix::impl_mat::test_matrix_select,
matrix::impl_mat::matrix_diag,
matrix::impl_mat::matrix_det,
matrix::impl_mat::matrix_solve,
matrix::impl_mat::create_mat_zeros,
matrix::impl_mat::create_mat_identity,
matrix::impl_mat::create_mat_diag,
matrix::impl_mat::test_empty_mean,
matrix::impl_mat::test_invalid_variance,
testsupport::constraints::is_lower_triangular_empty_matrix,
testsupport::constraints::is_lower_triangular_1x1,
testsupport::constraints::is_lower_triangular_square,
testsupport::constraints::is_upper_triangular_empty_matrix,
testsupport::constraints::is_upper_triangular_1x1,
testsupport::constraints::is_upper_triangular_square,
testsupport::constraints::is_upper_triangular_rectangular,
convert::inner_product_as_matrix_multiplication,
convert::matrix_from_slice,
convert::diag_offset_from_int,
convert::try_into_empty_matrix,
convert::try_into_f64_to_i64,
convert::try_into_f64_to_u64,
convert::try_into_i64_to_f64,
convert::try_into_u64_to_f64,
convert::try_into_signed_unsigned,
convert::test_row_convert,
convert::test_row_convert_mut,
convert::test_column_convert,
convert::test_column_convert_mut,
matrix::decomposition::bidiagonal::test_bidiagonal_square,
matrix::decomposition::bidiagonal::test_bidiagonal_non_square,
vector::impl_ops::vector_index_mut,
vector::impl_ops::vector_mul_f32_broadcast,
vector::impl_ops::vector_mul_int_broadcast,
vector::impl_ops::vector_div_f32_broadcast,
vector::impl_ops::vector_div_int_broadcast,
vector::impl_ops::vector_add_f32_broadcast,
vector::impl_ops::vector_add_int_broadcast,
vector::impl_ops::vector_add_f32_elemwise,
vector::impl_ops::vector_add_int_elemwise,
vector::impl_ops::vector_sub_f32_broadcast,
vector::impl_ops::vector_sub_int_broadcast,
vector::impl_ops::vector_sub_f32_elemwise,
vector::impl_ops::vector_sub_int_elemwise,
vector::impl_ops::vector_rem_f32_broadcast,
vector::impl_ops::vector_rem_int_broadcast,
vector::impl_ops::vector_rem_f32_elemwise,
vector::impl_ops::vector_rem_int_elemwise,
vector::impl_ops::vector_add_assign_int_broadcast,
vector::impl_ops::vector_add_assign_int_elemwise,
vector::impl_ops::vector_sub_assign_int_broadcast,
vector::impl_ops::vector_sub_assign_int_elemwise,
vector::impl_ops::vector_div_assign_f32_broadcast,
vector::impl_ops::vector_mul_assign_f32_broadcast,
vector::impl_ops::vector_rem_assign_int_broadcast,
vector::impl_ops::vector_rem_assign_int_elemwise,
vector::impl_ops::vector_bitand_int_broadcast,
vector::impl_ops::vector_bitand_bool_broadcast,
vector::impl_ops::vector_bitand_int_elemwise,
vector::impl_ops::vector_bitand_bool_elemwise,
vector::impl_ops::vector_bitor_int_broadcast,
vector::impl_ops::vector_bitor_bool_broadcast,
vector::impl_ops::vector_bitor_int_elemwise,
vector::impl_ops::vector_bitor_bool_elemwise,
vector::impl_ops::vector_bitxor_int_broadcast,
vector::impl_ops::vector_bitxor_bool_broadcast,
vector::impl_ops::vector_bitxor_int_elemwise,
vector::impl_ops::vector_bitxor_bool_elemwise,
vector::impl_ops::vector_bitand_assign_int_broadcast,
vector::impl_ops::vector_bitand_assign_bool_broadcast,
vector::impl_ops::vector_bitand_assign_int_elemwise,
vector::impl_ops::vector_bitand_assign_bool_elemwise,
vector::impl_ops::vector_bitor_assign_int_broadcast,
vector::impl_ops::vector_bitor_assign_bool_broadcast,
vector::impl_ops::vector_bitor_assign_int_elemwise,
vector::impl_ops::vector_bitor_assign_bool_elemwise,
vector::impl_ops::vector_bitxor_assign_int_broadcast,
vector::impl_ops::vector_bitxor_assign_bool_broadcast,
vector::impl_ops::vector_bitxor_assign_int_elemwise,
vector::impl_ops::vector_bitxor_assign_bool_elemwise,
vector::impl_ops::vector_neg_f32,
vector::impl_ops::vector_neg_int,
vector::impl_ops::vector_not_int,
vector::impl_ops::vector_not_bool,
vector::impl_vec::test_display,
vector::impl_vec::test_equality,
vector::impl_vec::create_vector_new,
vector::impl_vec::create_vector_new_from_slice,
vector::impl_vec::create_vector_from_fn,
vector::impl_vec::create_vector_zeros,
vector::impl_vec::vector_dot_product,
vector::impl_vec::vector_euclidean_norm,
vector::impl_vec::vector_iteration,
vector::impl_vec::vector_from_iter,
vector::impl_vec::vector_get_unchecked,
vector::impl_vec::vector_mul_f32_elemwise,
vector::impl_vec::vector_mul_int_elemwise,
vector::impl_vec::vector_div_f32_elemwise,
vector::impl_vec::vector_div_int_elemwise,
internal_utils::nullify_lower_triangular_part_examples,
internal_utils::nullify_upper_triangular_part_examples,
internal_utils::transpose_gemv_examples,
internal_utils::ger_examples,
ulp::plus_minus_zero_is_exact_match_f32,
ulp::plus_minus_zero_is_exact_match_f64,
ulp::f32_double_nan,
ulp::f64_double_nan,
ulp::property_exact_match_for_finite_f32_self_comparison,
ulp::property_exact_match_for_finite_f64_self_comparison,
ulp::property_recovers_ulp_diff_when_f32_constructed_from_i32,
ulp::property_recovers_ulp_diff_when_f64_constructed_from_i64,
ulp::property_f32_incompatible_signs_yield_corresponding_enum_value,
ulp::property_f64_incompatible_signs_yield_corresponding_enum_value,
ulp::property_f32_nan_gives_nan_enum_value,
ulp::property_f64_nan_gives_nan_enum_value,
macros::comparison::property_absolute_comparator_is_symmetric_i64,
macros::comparison::property_absolute_comparator_is_symmetric_f64,
macros::comparison::property_absolute_comparator_tolerance_is_not_strict_f64,
macros::comparison::property_exact_comparator_is_symmetric_i64,
macros::comparison::property_exact_comparator_is_symmetric_f64,
macros::comparison::property_exact_comparator_matches_equality_operator_i64,
macros::comparison::property_exact_comparator_matches_equality_operator_f64,
macros::comparison::property_ulp_comparator_is_symmetric,
macros::comparison::property_ulp_comparator_matches_ulp_trait,
macros::comparison::property_ulp_comparator_next_f64_is_ok_when_inside_tolerance,
macros::comparison::property_float_comparator_matches_abs_with_zero_ulp_tol,
macros::comparison::property_float_comparator_matches_ulp_with_zero_eps_tol,
macros::comparison::absolute_comparator_integer,
macros::comparison::absolute_comparator_floating_point,
macros::comparison::exact_comparator_integer,
macros::comparison::exact_comparator_floating_point,
macros::comparison::ulp_comparator_f64,
macros::vector::vector_macro,
macros::vector::vector_macro_constant_size,
macros::vector::vector_macro_empty_vec,
macros::matrix::matrix_macro,
macros::matrix::matrix_macro_empty_mat,
norm::test_euclidean_vector_norm,
norm::test_euclidean_matrix_norm,
norm::test_euclidean_matrix_slice_norm,
norm::test_euclidean_vector_metric,
|| should_panic!(norm::test_euclidean_vector_metric_bad_dim()),
norm::test_euclidean_matrix_metric,
|| should_panic!(norm::test_euclidean_matrix_metric_bad_dim()),
norm::test_euclidean_matrix_slice_metric,
|| should_panic!(norm::test_euclidean_matrix_slice_metric_bad_dim()),
norm::test_lp_vector_supremum,
norm::test_lp_matrix_supremum,
norm::test_lp_vector_one,
norm::test_lp_matrix_one,
norm::test_lp_vector_float,
norm::test_lp_matrix_float,
|| should_panic!(norm::test_lp_vector_bad_p()),
|| should_panic!(norm::test_lp_matrix_bad_p()),
|| should_panic!(norm::test_lp_vector_bad_int_p()),
|| should_panic!(norm::test_lp_matrix_bad_int_p()),
);

    use rulinalg::vector::Vector;
    use rulinalg::matrix::Matrix;
    #[derive(Serialize, Deserialize)]
    struct Vvv {
        p: Vector<f64>,
        q: Matrix<f64>,
    }

    sgx_status_t::SGX_SUCCESS
}
