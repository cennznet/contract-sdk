//!
//! Misc. helper functions
//!
use crate::runtime::{Context, ExecutionContext};
use core::mem::transmute;

/// Convert a `u32` into its byte representation
pub fn u32_to_bytes(x: u32) -> [u8; 4] {
    unsafe { transmute::<u32, [u8; 4]>(x) }
}

/// Convert a `u64` into its byte representation
pub fn u64_to_bytes(x: u64) -> [u8; 8] {
    unsafe { transmute::<u64, [u8; 8]>(x) }
}

/// Load a u32 from bytes
pub fn bytes_to_u32(x: [u8; 4]) -> u32 {
    unsafe { transmute::<[u8; 4], u32>(x) }
}

/// Load a u64 from bytes
pub fn bytes_to_u64(x: [u8; 8]) -> u64 {
    unsafe { transmute::<[u8; 8], u64>(x) }
}

// Get a one-time random u64, bound by `min` and/or `max`
pub fn random_in_range(min: u64, max: u64) -> u64 {
    let seed = Context::random_seed();
    let r = bytes_to_u64([
        seed[0], seed[1], seed[2], seed[3], seed[4], seed[5], seed[6], seed[7],
    ]);
    min + (r % max)
}
