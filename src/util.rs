//!
//! Misc. helper functions
//!
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
