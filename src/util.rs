//!
//! Misc. helper functions
//!

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
