//!
//! Runtime Storage API
//!
use crate::runtime::{cabi, read_scratch_buffer};

// Export map under storage namespace
pub use crate::map::Map;

use alloc::vec::Vec;
use parity_codec::{Codec, Decode, Encode};
use primitives::H256;

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
pub struct Storage;

/// A key for blockchain storage, its inner value is a `[u8; 32]`
#[cfg_attr(test, derive(Clone, Debug))]
#[derive(Default)]
pub struct StorageKey(pub H256);

impl StorageKey {
    /// Return a zeroed-out storage key
    pub fn zero() -> Self {
        StorageKey(H256::zero())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

/// A type which implements K-V storage on the external blockchain
/// There are only two operations R/W
pub trait StorageABI {
    /// Read a value from storage under key
    fn get_kv(k: &StorageKey) -> Option<Vec<u8>>;
    /// Write a value to storage under key
    fn put_kv(k: &StorageKey, v: Option<&[u8]>);
}

/// Convert T into a StorageKey
impl<T> From<T> for StorageKey
where
    T: AsRef<[u8]>,
{
    fn from(k: T) -> Self {
        let k_len = k.as_ref().len();
        // Pad or truncate keys to length 32
        if k_len >= 32 {
            let key = H256::from_slice(&k.as_ref()[..32]);
            return StorageKey(key);
        }

        let mut buf: [u8; 32] = [0; 32];
        buf[..k_len].clone_from_slice(&k.as_ref()[..k_len]);
        StorageKey(H256::from_slice(&buf))
    }
}

/// High-level storage API
impl Storage {
    /// Put a `value` into storage under `key`
    pub fn put<K, V>(key: K, value: V)
    where
        K: AsRef<[u8]> + Sized,
        V: Codec,
    {
        let k: StorageKey = key.into();
        let v = Encode::encode(&value);
        <Self as StorageABI>::put_kv(&k, Some(&v));
    }

    /// Retreive a value from storage at `key`.
    /// Returning `None` if not found.
    pub fn get<K, V>(key: &K) -> Option<V>
    where
        K: AsRef<[u8]> + Sized,
        V: Codec,
    {
        let k: StorageKey = key.into();
        if let Some(v) = <Self as StorageABI>::get_kv(&k) {
            return Decode::decode(&mut &v[..]);
        }
        None
    }

    /// Remove a key from storage by zero-ing out the value.
    pub fn remove<K>(key: &K)
    where
        K: AsRef<[u8]> + Sized,
    {
        let k: StorageKey = key.into();
        <Self as StorageABI>::put_kv(&k, Some(&StorageKey::zero().as_bytes()));
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
    use super::StorageKey;

    #[test]
    fn from_short_storage_key_is_padded() {
        let key = StorageKey::from("my key");
        let target = b"my key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        assert_eq!(StorageKey::from(target).0, key.0);
    }

    #[test]
    fn from_long_storage_key_is_truncated() {
        let key = StorageKey::from("myreallylongstoragekeythatislongerthan32bytes");
        let target = &b"myreallylongstoragekeythatislongerthan32bytes"[..32];
        assert_eq!(target, key.0.as_bytes());
    }
}
