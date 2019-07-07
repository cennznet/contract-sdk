//!
//! Runtime Storage API
//!
use crate::runtime::{cabi, read_scratch_buffer};

use alloc::vec::Vec;
use parity_codec::{Codec, Decode, Encode};

/// A simple map-like API over contract storage.
///
/// Example usage:
///  Put a K/V in storage:
///      `Storage::put("some_key", "some_value")`
///
/// Retreive a K/V from storage:
///     `let some_value = Storage::get("some_key").unwrap()`
///     `Storage::get("some_missing_key").is_none() == true`
///
/// Remove a K/V from storage (writes zero value):
///     `Storage::remove("some_key") == StorageKey::zero()`
///
// The index operator `[]` is unsupported since the `Storage` struct holds no data itself
// it merley interfaces with the underlying storage.
// See: https://github.com/rust-lang/rfcs/issues/997
//
pub const STORAGE_KEY_ZERO: [u8; 32] = [0; 32];
pub struct Storage;

/// A key for blockchain storage, its inner value is a `[u8; 32]`
pub type StorageKey = [u8; 32];

/// A type which implements K-V storage on the external blockchain
/// There are only two operations R/W
pub trait StorageABI {
    /// Read a value from storage under key
    fn get_kv(k: &StorageKey) -> Option<Vec<u8>>;
    /// Write a value to storage under key
    fn put_kv(k: &StorageKey, v: Option<&[u8]>);
}

/// Convert T into a StorageKey
pub fn to_storage_key(k: &[u8]) -> StorageKey {
    let mut buf = STORAGE_KEY_ZERO;
    // Pad or truncate keys to length 32
    match k.len() {
        l if (l > 32) => {
            buf[..32].clone_from_slice(&k[..32]);
        }
        _ => {
            buf[..k.len()].clone_from_slice(&k[..k.len()]);
        }
    };

    buf
}

/// High-level storage API
impl Storage {
    /// Put a `value` into storage under `key`
    pub fn put<K, V>(key: &[u8], value: V)
    where
        V: Codec,
    {
        let k: StorageKey = to_storage_key(key);
        let v = Encode::encode(&value);
        <Self as StorageABI>::put_kv(&k, Some(&v));
    }

    /// Retreive a value from storage at `key`.
    /// Returning `None` if not found.
    pub fn get<K, V>(key: &[u8]) -> Option<V>
    where
        V: Codec,
    {
        let k: StorageKey = to_storage_key(key);
        if let Some(v) = <Self as StorageABI>::get_kv(&k) {
            return Decode::decode(&mut &v[..]);
        }
        None
    }

    /// Remove a key from storage by zero-ing out the value.
    pub fn remove<K>(key: &[u8]) {
        let k: StorageKey = to_storage_key(key);
        <Self as StorageABI>::put_kv(&k, Some(&STORAGE_KEY_ZERO));
    }
}

/// Low level storage ABI
impl StorageABI for Storage {
    /// Store `value` under `key` in storage
    fn put_kv(key: &StorageKey, value: Option<&[u8]>) {
        unsafe {
            let mut value_ptr = 0;
            let mut value_len = 0;
            let value_non_null = if let Some(v) = value {
                value_ptr = v.as_ptr() as u32;
                value_len = v.len() as u32;
                1
            } else {
                0
            };
            cabi::ext_set_storage(key.as_ptr() as u32, value_non_null, value_ptr, value_len);
        }
    }

    /// Load stored value at `key`, returns `None` if not found
    fn get_kv(key: &StorageKey) -> Option<Vec<u8>> {
        const SUCCESS: u32 = 0;
        unsafe {
            let result = cabi::ext_get_storage(key.as_ptr() as u32);
            if result != SUCCESS {
                return None;
            }
            Some(read_scratch_buffer())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::to_storage_key;

    #[test]
    fn from_short_storage_key_is_padded() {
        assert_eq!(
            b"my key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            &to_storage_key("my key".as_bytes()),
        );
    }

    #[test]
    fn from_long_storage_key_is_truncated() {
        let key = &to_storage_key("myreallylongstoragekeythatislongerthan32bytes".as_bytes());
        let target = &b"myreallylongstoragekeythatislongerthan32bytes"[..32];
        assert_eq!(target, key);
    }
}
