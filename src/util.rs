//!
//! Misc. helper functions
//!
use crate::alloc::vec::Vec;

/// Convert a `u32` into a `Vec<u8>`
pub fn u32_to_bytes(x: u32) -> [u8; 4] {
    let mut buf: [u8; 4] = [0u8; 4];
    for i in 0..4 {
        buf[3 - i] = ((x >> i * 8) & 0xFF) as u8;
    }

    buf
}

/// Convert a `u64` into a `Vec<u8>`
pub fn u64_to_bytes(x: u64) -> [u8; 8] {
    let mut buf: [u8; 8] = [0u8; 8];
    for i in 0..8 {
        buf[7 - i] = ((x >> i * 8) & 0xFF) as u8;
    }

    buf
}

/// Generate a `u64` from using `seed` bytes as the genesis
pub fn gen_random(seed: Vec<u8>) -> u64 {
    let mut x: u8 = 0;
    for i in 0..8 {
        x = x + seed[i];
    }

    x as u64
}
