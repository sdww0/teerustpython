#[warn(unused_imports)]
pub type Instant = sgx_tstd::time::Instant;

/// The current time, in milliseconds.
#[cfg(feature = "now")]
pub fn now() -> f64 {
    time::precise_time_s() * 1000.0
}
