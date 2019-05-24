use crate::storage::{to_storage_key, Storage, StorageABI, StorageKey, STORAGE_KEY_ZERO};
use alloc::vec::Vec;
use core::{borrow::Borrow, cmp::Eq, hash::Hash};
use hashbrown::hash_map::{HashMap, Iter};
use parity_codec::{Codec, Decode, Encode, Input};

/// A map type for contract storage. Its keys types must derive parity Codec.
/// New isnstances are in-memory only and only persist upon calling `.flush()`.
///
/// Note!: This implementation is not (gas) efficient when encoding/decoding
/// to/from storage and is meant to serve as a placeholder for improved
/// versions in future iterations.
///
// This is a thin wrapper on top of `hashbrown::HashMap` with some serialization support.
// TODO: Currently we're eager loading the entire map from disk.
//       can we implement a form of lazy loading? So the contract only pays for it uses.
#[cfg_attr(test, derive(Clone, Debug))]
pub struct Map<K: Eq + Hash, V> {
    inner: HashMap<K, V>,
    storage_key: StorageKey,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash + Codec,
    V: Codec,
{
    /// Create a new Map at the given storage key
    pub fn new(storage_key: &[u8]) -> Self {
        Map {
            inner: HashMap::new(),
            storage_key: to_storage_key(storage_key),
        }
    }

    /// Return a default Map at the default storage key
    pub fn default() -> Self {
        Map {
            inner: HashMap::default(),
            storage_key: STORAGE_KEY_ZERO,
        }
    }

    /// Return the number of entries in the map
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the map is empty or not
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the value under `key`, None if not found
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(key)
    }

    /// Return the value under `key` as mutable, None if not found
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_mut(key)
    }

    /// Insert `value` under `key`
    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    /// Remove the value under `key` if any
    pub fn remove<Q>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.remove(key);
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.contains_key(key)
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).
    pub fn iter(&self) -> Iter<K, V> {
        self.inner.iter()
    }

    /// Load a map from persistent storage at `key`
    /// Returns a new map if no data was found
    /// !This will panic if the stored data has an invalid encoding.
    pub fn load_or_create(key: &[u8]) -> Self {
        let storage_key = to_storage_key(key);
        let buf = Storage::get_kv(&storage_key).unwrap_or(vec![]);
        Decode::decode(&mut &buf[..])
            .map(|mut m: Self| {
                m.storage_key = storage_key; // Set the storage key, avoids needing to encode/decode it
                m
            })
            .unwrap_or(Self::new(key))
    }

    /// Load a map from persistent storage at `key`
    /// !This will panic if the stored data has an invalid encoding.
    pub fn load(key: &[u8]) -> Self {
        let storage_key = to_storage_key(key);
        let buf = Storage::get_kv(&storage_key).unwrap();
        Decode::decode(&mut &buf[..])
            .map(|mut m: Self| {
                m.storage_key = storage_key; // Set the storage key, avoids needing to encode/decode it
                m
            })
            .unwrap()
    }

    /// Write the map to persistent storage at `key`
    pub fn flush(&mut self) {
        let storage_key = to_storage_key(&self.storage_key);
        let data = Encode::encode(self);
        Storage::put_kv(&storage_key, Some(&data));
    }
}

impl<K, V> Encode for Map<K, V>
where
    K: Eq + Hash + Codec,
    V: Codec,
{
    /// Convert self to an owned vector.
    fn encode(&self) -> Vec<u8> {
        let mut data: Vec<(&K, &V)> = Vec::new();
        for (k, v) in self.inner.iter() {
            data.push((k, v))
        }
        Encode::encode(&data)
    }
}

/// Trait that allows zero-copy read of value-references from slices in LE format.
impl<K, V> Decode for Map<K, V>
where
    K: Eq + Hash + Codec,
    V: Codec,
{
    /// Attempt to deserialise the value from input.
    fn decode<I: Input>(value: &mut I) -> Option<Self> {
        // Deserialize entries
        let data: Vec<(K, V)> = Decode::decode(value)?;
        // Rebuild map
        let mut map = Self::default();
        for (k, v) in data {
            map.insert(k, v);
        }
        Some(map)
    }
}

impl<'a, K, Q: ?Sized, V> core::ops::Index<&'a Q> for Map<K, V>
where
    K: Eq + Hash + Codec + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.inner
            .get(index)
            .expect("[contract_sdk::Map::index] Error: `index` is out of bounds")
    }
}

impl<'a, K, Q: ?Sized, V> core::ops::IndexMut<&'a Q> for Map<K, V>
where
    K: Eq + Hash + Codec + Borrow<Q>,
    Q: Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.inner
            .get_mut(index)
            .expect("[contract_sdk::Map::index] Error: `index` is out of bounds")
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::{to_storage_key, Map};
    use alloc::vec::Vec;
    use parity_codec::{Decode, Encode};
    use parity_codec_derive::*;

    #[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
    struct MockValue {
        field1: u32,
        field2: Vec<u8>,
    }

    #[test]
    fn it_encodes_and_decodes_the_same() {
        let mut map: Map<u32, MockValue> = Map::default();
        map.insert(
            1,
            MockValue {
                field1: 2u32,
                field2: vec![1, 2, 3, 4],
            },
        );
        map.insert(
            2,
            MockValue {
                field1: 3u32,
                field2: vec![5, 6, 7, 8],
            },
        );
        let buf = Encode::encode(&map);
        let decoded_map: Map<u32, MockValue> = Map::decode(&mut &buf[..]).unwrap();

        assert_eq!(map[&1], decoded_map[&1]);
        assert_eq!(map[&2], decoded_map[&2]);
    }

    #[test]
    fn nested_maps_work() {
        let mut map: Map<u32, Map<u32, MockValue>> = Map::default();
        let v = MockValue {
            field1: 2u32,
            field2: vec![1, 2, 3, 4],
        };

        let mut nested_map: Map<u32, MockValue> = Map::default();
        nested_map.insert(1, v.clone());

        map.insert(1, nested_map.clone());
        map.insert(2, nested_map);

        let buf = Encode::encode(&map);
        let decoded_map: Map<u32, Map<u32, MockValue>> = Map::decode(&mut &buf[..]).unwrap();

        assert_eq!(map[&1][&1], decoded_map[&1][&1]);
        assert_eq!(map[&2][&1], decoded_map[&2][&1]);
    }

    #[test]
    fn load_or_create_preserves_storage_key() {
        // Fake external runtime ABI calls used by `Map::load_or_create`
        // TODO: Currently we can only mock these extern functions once per create :(
        #[no_mangle]
        fn ext_scratch_size() -> u32 {
            1
        }
        #[no_mangle]
        fn ext_get_storage(_: u32) -> u32 {
            0
        }
        #[no_mangle]
        // Fill `dest_ptr` with encoded Map bytes
        fn ext_scratch_copy(dest_ptr: u32, _offset: u32, len: u32) {
            let m: Map<u32, u32> = Map::new(b"_");
            let mut buf = Encode::encode(&m);
            unsafe {
                let mut _slice = core::slice::from_raw_parts_mut(dest_ptr as *mut u8, len as usize);
                _slice = &mut buf[..];
            }
        }

        let map: Map<u32, u32> = Map::load_or_create(b"my map");
        assert_eq!(to_storage_key(b"my map"), map.storage_key);
    }

}
