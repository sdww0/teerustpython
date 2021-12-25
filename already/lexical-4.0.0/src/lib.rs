//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both std and no_std environments.
//! Lexical provides routines to convert numbers to and from decimal
//! strings. Lexical also supports non-base 10 numbers, with the `radix`
//! feature, for both integers and floats. Lexical is simple to use,
//! and exports up to 10 functions in the high-level API.
//!
//! # Getting Started
//!
//! ```rust
//! extern crate lexical;
//!
//! // Number to string
//! lexical::to_string(3.0);            // "3.0", always has a fraction suffix.
//! lexical::to_string(3);              // "3"
//!
//! // String to number.
//! let i: i32 = lexical::parse("3").unwrap();      // 3, auto-type deduction.
//! let f: f32 = lexical::parse("3.5").unwrap();    // 3.5
//! let d = lexical::parse::<f64, _>("3.5");        // Ok(3.5), successful parse.
//! let d = lexical::parse::<f64, _>("3a");         // Err(Error(_)), failed to parse.
//! ```
//!
//! # Conversion API
//!
//! **To String**
//! - [`to_string`]
#![cfg_attr(feature = "radix", doc = " - [`to_string_radix`]")]
//!
//! **From String**
//! - [`parse`]
#![cfg_attr(feature = "radix", doc = " - [`parse_radix`]")]
//! - [`parse_partial`]
#![cfg_attr(feature = "radix", doc = " - [`parse_partial_radix`]")]
//! - [`parse_lossy`]
#![cfg_attr(feature = "radix", doc = " - [`parse_lossy_radix`]")]
//! - [`parse_partial_lossy`]
#![cfg_attr(feature = "radix", doc = " - [`parse_partial_lossy_radix`]")]
//!
//! # Configuration Settings
//!
//! **Get Configuration**
//! - [`get_exponent_default_char`]
#![cfg_attr(feature = "radix", doc = " - [`get_exponent_backup_char`]")]
#![cfg_attr(all(feature = "correct", feature = "rounding"), doc = " - [`get_float_rounding`]")]
//! - [`get_nan_string`]
//! - [`get_inf_string`]
//! - [`get_infinity_string`]
//!
//! **Set Configuration**
//! - [`set_exponent_default_char`]
#![cfg_attr(feature = "radix", doc = " - [`set_exponent_backup_char`]")]
#![cfg_attr(all(feature = "correct", feature = "rounding"), doc = " - [`set_float_rounding`]")]
//! - [`set_nan_string`]
//! - [`set_inf_string`]
//! - [`set_infinity_string`]
//!
//! [`to_string`]: fn.to_string.html
#![cfg_attr(feature = "radix", doc = " [`to_string_radix`]: fn.to_string_radix.html")]
//! [`parse`]: fn.parse.html
#![cfg_attr(feature = "radix", doc = " [`parse_radix`]: fn.parse_radix.html")]
//! [`parse_partial`]: fn.parse_partial.html
#![cfg_attr(feature = "radix", doc = " [`parse_partial_radix`]: fn.parse_partial_radix.html")]
//! [`parse_lossy`]: fn.parse_lossy.html
#![cfg_attr(feature = "radix", doc = " [`parse_lossy_radix`]: fn.parse_lossy_radix.html")]
//! [`parse_partial_lossy`]: fn.parse_partial_lossy.html
#![cfg_attr(feature = "radix", doc = " [`parse_partial_lossy_radix`]: fn.parse_partial_lossy_radix.html")]
//!
//! [`get_exponent_default_char`]: fn.get_exponent_default_char.html
#![cfg_attr(feature = "radix", doc = " [`get_exponent_backup_char`]: fn.get_exponent_backup_char.html")]
#![cfg_attr(all(feature = "correct", feature = "rounding"), doc = " [`get_float_rounding`]: fn.get_float_rounding.html")]
//! [`get_nan_string`]: fn.get_nan_string.html
//! [`get_inf_string`]: fn.get_inf_string.html
//! [`get_infinity_string`]: fn.get_infinity_string.html
//!
//! [`set_exponent_default_char`]: fn.set_exponent_default_char.html
#![cfg_attr(feature = "radix", doc = " [`set_exponent_backup_char`]: fn.set_exponent_backup_char.html")]
#![cfg_attr(all(feature = "correct", feature = "rounding"), doc = " [`set_float_rounding`]: fn.set_float_rounding.html")]
//! [`set_nan_string`]: fn.set_nan_string.html
//! [`set_inf_string`]: fn.set_inf_string.html
//! [`set_infinity_string`]: fn.set_infinity_string.html

// FEATURES

// Require intrinsics and alloc in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

// EXTERNAL

#[macro_use]
extern crate cfg_if;

extern crate lexical_core;

// CONFIG

// Need an allocator for String/Vec.
#[cfg(not(feature = "std"))]
extern crate alloc;

/// Facade around the core features for name mangling.
extern crate sgx_tstd as std;
pub(crate) mod lib {
cfg_if! {
if #[cfg(feature = "std")] {
    pub(crate) use std::*;
} else {
    pub(crate) use core::*;
}}  // cfg_if

cfg_if! {
if #[cfg(feature = "std")] {
    pub(crate) use std::string::String;
    pub(crate) use std::vec::Vec;
} else {
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::vec::Vec;
}
}}  // cfg_if

// API

// Re-export exponent character getters and setters.
pub use lexical_core::{get_exponent_default_char, set_exponent_default_char};

#[cfg(feature = "radix")]
pub use lexical_core::{get_exponent_backup_char, set_exponent_backup_char};

// Re-export NaN, short INF, and long INFINITY string getters and setters.
pub use lexical_core::{get_inf_string, get_infinity_string, get_nan_string};
pub use lexical_core::{set_inf_string, set_infinity_string, set_nan_string};

// Re-export the float rounding scheme used.
#[cfg(all(feature = "correct", feature = "rounding"))]
pub use lexical_core::{get_float_rounding, set_float_rounding, RoundingKind};

// Re-export the Result, Error and ErrorCode globally.
pub use lexical_core::{Error, ErrorCode, Result};

// Publicly expose traits so they may be used for generic programming.
pub use lexical_core::{FromLexical, FromLexicalLossy, ToLexical};

// HELPERS

/// Get a vector as a slice, including the capacity.
#[inline]
unsafe fn vector_as_slice<'a, T>(buf: &'a mut Vec<T>)
    -> &'a mut [T]
{
    let first = buf.as_mut_ptr();
    lib::slice::from_raw_parts_mut(first, buf.capacity())
}

// HIGH LEVEL

use lib::convert::AsRef;

/// High-level conversion of a number to a decimal-encoded string.
///
/// * `n`       - Number to convert to string.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// assert_eq!(lexical::to_string(5), "5");
/// ##[cfg(not(feature = "trim_floats"))]
/// assert_eq!(lexical::to_string(0.0), "0.0");
/// # }
/// ```
#[inline]
pub fn to_string<N: ToLexical>(n: N) -> lib::String {
    unsafe {
        let mut buf = lib::Vec::<u8>::with_capacity(N::FORMATTED_SIZE_DECIMAL);
        let len = lexical_core::write(n, vector_as_slice(&mut buf)).len();
        buf.set_len(len);
        lib::String::from_utf8_unchecked(buf)
    }
}

/// High-level conversion of a number to string with a custom radix.
///
/// * `n`       - Number to convert to string.
/// * `base`    - Number of unique digits for the number (radix).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// assert_eq!(lexical::to_string_radix(5, 10), "5");
/// ##[cfg(not(feature = "trim_floats"))]
/// assert_eq!(lexical::to_string_radix(0.0, 10), "0.0");
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
#[cfg(feature = "radix")]
#[inline]
pub fn to_string_radix<N: ToLexical>(n: N, radix: u8) -> lib::String {
    unsafe {
        let mut buf = lib::Vec::<u8>::with_capacity(N::FORMATTED_SIZE);
        let len = lexical_core::write_radix(n, radix, vector_as_slice(&mut buf)).len();
        buf.set_len(len);
        lib::String::from_utf8_unchecked(buf)
    }
}

/// High-level conversion of decimal-encoded bytes to a number.
///
/// This function only returns a value if the entire string is
/// successfully parsed.
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
/// // Create our error.
/// fn err_code<T>(r: lexical::Result<T>) -> ErrorCode {
///     r.err().unwrap().code
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse::<i32, _>("5"), Ok(5));
/// assert_eq!(err_code(lexical::parse::<i32, _>("1a")), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>("1."), Ok(1.0));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse::<i32, _>(b"5"), Ok(5));
/// assert_eq!(err_code(lexical::parse::<i32, _>(b"1a")), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1."), Ok(1.0));
/// # assert_eq!(lexical::parse::<f32, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # assert_eq!(lexical::parse::<f64, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # }
/// ```
#[inline]
pub fn parse<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N>
{
    N::from_lexical(bytes.as_ref())
}

/// High-level, partial conversion of decimal-encoded bytes to a number.
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
///
/// // String overloads
/// assert_eq!(lexical::parse_partial::<i32, _>("5"), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial::<i32, _>("1a"), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>("0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>("1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial::<f32, _>("1."), Ok((1.0, 2)));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_partial::<i32, _>(b"5"), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial::<i32, _>(b"1a"), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"1."), Ok((1.0, 2)));
/// # assert_eq!(lexical::parse_partial::<f32, _>(b"5.002868148396374"), Ok((5.002868148396374, 17)));
/// # assert_eq!(lexical::parse_partial::<f64, _>(b"5.002868148396374"), Ok((5.002868148396374, 17)));
/// # }
/// ```
#[inline]
pub fn parse_partial<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<(N, usize)>
{
    N::from_lexical_partial(bytes.as_ref())
}

/// High-level lossy conversion of decimal-encoded bytes to a number.
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse`].
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
/// // Get our error code.
/// fn err_code<T>(r: lexical::Result<T>) -> ErrorCode {
///     r.err().unwrap().code
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse_lossy::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::parse_lossy::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::parse_lossy::<f32, _>("1."), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_lossy::<f32, _>("1a")), ErrorCode::InvalidDigit);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"1."), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_lossy::<f32, _>(b"1a")), ErrorCode::InvalidDigit);
/// # }
/// ```
///
/// [`parse`]: fn.parse.html
#[inline]
pub fn parse_lossy<N: FromLexicalLossy, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N>
{
    N::from_lexical_lossy(bytes.as_ref())
}

/// High-level, partial, lossy conversion of decimal-encoded bytes to a number.
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse_partial`].
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
///
/// // String overloads
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>("0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>("1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>("1."), Ok((1.0, 2)));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>(b"0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>(b"1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_lossy::<f32, _>(b"1."), Ok((1.0, 2)));
/// # }
/// ```
///
/// [`parse_partial`]: fn.parse_partial.html
#[inline]
pub fn parse_partial_lossy<N: FromLexicalLossy, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<(N, usize)>
{
    N::from_lexical_partial_lossy(bytes.as_ref())
}

/// High-level conversion of bytes to a number with a custom radix.
///
/// This function only returns a value if the entire string is
/// successfully parsed.
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
/// // Get our error code wrapper.
/// fn err_code<T>(r: lexical::Result<T>) -> ErrorCode {
///     r.err().unwrap().code
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse_radix::<i32, _>("5", 10), Ok(5));
/// assert_eq!(err_code(lexical::parse_radix::<i32, _>("1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(err_code(lexical::parse_radix::<i32, _>("1.", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::parse_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::parse_radix::<f32, _>("1.", 10), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_radix::<f32, _>("1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(err_code(lexical::parse_radix::<f32, _>("1.0.", 10)), ErrorCode::InvalidDigit);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_radix::<i32, _>(b"5", 10), Ok(5));
/// assert_eq!(err_code(lexical::parse_radix::<i32, _>(b"1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::parse_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::parse_radix::<f32, _>(b"1.", 10), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_radix::<f32, _>(b"1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(err_code(lexical::parse_radix::<f32, _>(b"1.0.", 10)), ErrorCode::InvalidDigit);
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
#[cfg(feature = "radix")]
#[inline]
pub fn parse_radix<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N>
{
    N::from_lexical_radix(bytes.as_ref(), radix)
}

/// High-level, partial conversion of bytes to a number with a custom radix.
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
///
/// // String overloads
/// assert_eq!(lexical::parse_partial_radix::<i32, _>("5", 10), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial_radix::<i32, _>("1a", 10), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial_radix::<i32, _>("1.", 10), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>("0", 10), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>("1.0", 10), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>("1.", 10), Ok((1.0, 2)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>("1a", 10), Ok((1.0, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>("1.0.", 10), Ok((1.0, 3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_partial_radix::<i32, _>(b"5", 10), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial_radix::<i32, _>(b"1a", 10), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial_radix::<i32, _>(b"1.", 10), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>(b"0", 10), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>(b"1.0", 10), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>(b"1.", 10), Ok((1.0, 2)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>(b"1a", 10), Ok((1.0, 1)));
/// assert_eq!(lexical::parse_partial_radix::<f32, _>(b"1.0.", 10), Ok((1.0, 3)));
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
#[cfg(feature = "radix")]
#[inline]
pub fn parse_partial_radix<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<(N, usize)>
{
    N::from_lexical_partial_radix(bytes.as_ref(), radix)
}

/// High-level, lossy conversion of bytes to a float with a custom radix.
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse_radix`].
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
/// // Create our error wrapper.
/// fn err_code<T>(r: lexical::Result<T>) -> ErrorCode {
///     r.err().unwrap().code
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("1.", 10), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_lossy_radix::<f32, _>("1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(err_code(lexical::parse_lossy_radix::<f32, _>("1.0.", 10)), ErrorCode::InvalidDigit);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"1.", 10), Ok(1.0));
/// assert_eq!(err_code(lexical::parse_lossy_radix::<f32, _>(b"1a", 10)), ErrorCode::InvalidDigit);
/// assert_eq!(err_code(lexical::parse_lossy_radix::<f32, _>(b"1.0.", 10)), ErrorCode::InvalidDigit);
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
///
/// [`parse_radix`]: fn.parse_radix.html
#[cfg(feature = "radix")]
#[inline]
pub fn parse_lossy_radix<N: FromLexicalLossy, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N>
{
    N::from_lexical_lossy_radix(bytes.as_ref(), radix)
}

/// High-level, partial lossy conversion of bytes to a number with a custom radix.
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse_radix`].
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorCode;
/// # pub fn main() {
///
/// // String overloads
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>("0", 10), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>("1.0", 10), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>("1.", 10), Ok((1.0, 2)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>("1a", 10), Ok((1.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>("1.0.", 10), Ok((1.0, 3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>(b"0", 10), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>(b"1.0", 10), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>(b"1.", 10), Ok((1.0, 2)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>(b"1a", 10), Ok((1.0, 1)));
/// assert_eq!(lexical::parse_partial_lossy_radix::<f32, _>(b"1.0.", 10), Ok((1.0, 3)));
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
#[cfg(feature = "radix")]
#[inline]
pub fn parse_partial_lossy_radix<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<(N, usize)>
{
    N::from_lexical_partial_radix(bytes.as_ref(), radix)
}
