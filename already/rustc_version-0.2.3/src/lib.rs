// Copyright 2016 rustc-version-rs developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]

//! Simple library for getting the version information of a `rustc`
//! compiler.
//!
//! This can be used by build scripts or other tools dealing with Rust sources
//! to make decisions based on the version of the compiler.
//!
//! It calls `$RUSTC --version -v` and parses the output, falling
//! back to `rustc` if `$RUSTC` is not set.
//!
//! # Example
//!
//! ```rust
//! // This could be a cargo build script
//!
//! extern crate rustc_version;
//! use rustc_version::{version, version_meta, Channel, Version};
//!
//! fn main() {
//!     // Assert we haven't travelled back in time
//!     assert!(version().unwrap().major >= 1);
//!
//!     // Set cfg flags depending on release channel
//!     match version_meta().unwrap().channel {
//!         Channel::Stable => {
//!             println!("cargo:rustc-cfg=RUSTC_IS_STABLE");
//!         }
//!         Channel::Beta => {
//!             println!("cargo:rustc-cfg=RUSTC_IS_BETA");
//!         }
//!         Channel::Nightly => {
//!             println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
//!         }
//!         Channel::Dev => {
//!             println!("cargo:rustc-cfg=RUSTC_IS_DEV");
//!         }
//!     }
//!
//!     // Check for a minimum version
//!     if version().unwrap() >= Version::parse("1.4.0").unwrap() {
//!         println!("cargo:rustc-cfg=compiler_has_important_bugfix");
//!     }
//! }
//! ```
#![no_std]
extern crate semver;
extern crate sgx_tstd as std;
use semver::Identifier;
use std::{env, str};
use std::ffi::OsString;
use std::string::String;

// Convenience re-export to allow version comparison without needing to add
// semver crate.
pub use semver::Version;



/// Release channel of the compiler.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Channel {
    /// Development release channel
    Dev,
    /// Nightly release channel
    Nightly,
    /// Beta release channel
    Beta,
    /// Stable release channel
    Stable,
}

/// Rustc version plus metada like git short hash and build date.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VersionMeta {
    /// Version of the compiler
    pub semver: Version,

    /// Git short hash of the build of the compiler
    pub commit_hash: Option<String>,

    /// Commit date of the compiler
    pub commit_date: Option<String>,

    /// Build date of the compiler; this was removed between Rust 1.0.0 and 1.1.0.
    pub build_date: Option<String>,

    /// Release channel of the compiler
    pub channel: Channel,

    /// Host target triple of the compiler
    pub host: String,

    /// Short version string of the compiler
    pub short_version_string: String,
}


