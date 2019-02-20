use crate::error::SDKError;
use crate::storage::{Storage, StorageABI, StorageKey};
use alloc::vec::Vec;
use core::{borrow::Borrow, cmp::Eq, hash::Hash};
use hashbrown::hash_map::{HashMap, Iter};
use parity_codec::{Codec, Decode, Encode, Input, Output};

/// A map type for contract storage. Its keys types must derive parity Codec.
///
// This is a thin wrapper on top of `hashbrown::HashMap` with some serialization support.
// TODO: Currently we're eager loading the entire map from disk.
//       can we implement a form of lazy loading? So the contract only pays for it uses.
#[derive(Clone, Debug)]
pub struct Map<K: Eq + Hash + Codec, V>(HashMap<K, V>);

impl<K, V> Map<K, V>
where
    K: Eq + Hash + Codec,
    V: Codec,
{
    pub fn new() -> Self {
        Map { 0: HashMap::new() }
    }

    /// Return the number of entries in the map
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return the value under `key`, None if not found
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(key)
    }

    /// Insert `value` under `key`
    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }

    /// Remove the value under `key` if any
    pub fn remove<Q>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove(key);
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.contains_key(key)
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).
    pub fn iter(&self) -> Iter<K, V> {
        self.0.iter()
    }

    /// Load a map from persistent storage at `key`
    /// Returns a new map if no data was found
    /// !This will fail if the stored data has an invalid encoding.
    pub fn load_or_default(key: &StorageKey) -> Result<Self, SDKError> {
        let data = Storage::get_kv(key);
        if let Some(buf) = data {
            Decode::decode(&mut &buf[..])
                .ok_or(SDKError::Decode("Failed decoding got invalid data"))
        } else {
            Ok(Self::new())
        }
    }

    /// Write the map to persistent storage at `key`
    /// Consumes the map leaving it unusable afterwards.
    pub fn flush(&mut self, key: &StorageKey) {
        let data = Encode::encode(self);
        Storage::put_kv(key, Some(&data));
    }
}

impl<K, V> Encode for Map<K, V>
where
    K: Eq + Hash + Codec,
    V: Codec,
{
    /// Convert self to a slice and append it to the destination.
    fn encode_to<T: Output>(&self, dest: &mut T) {
        self.using_encoded(|buf| dest.write(buf));
    }

    /// Convert self to an owned vector.
    fn encode(&self) -> Vec<u8> {
        let mut data: Vec<(&K, &V)> = Vec::new();
        for (k, v) in self.0.iter() {
            data.push((k, v))
        }
        Encode::encode(&data)
    }

    /// Convert self to a slice and then invoke the given closure with it.
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(&self.encode())
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
        let mut map = Self::new();
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
        self.0
            .get(index)
            .expect("[contract_sdk::Map::index] Error: `index` is out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use alloc::vec::Vec;
    use parity_codec::{Decode, Encode};
    use parity_codec_derive::*;

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    struct MockValue {
        field1: u32,
        field2: Vec<u8>,
    }

    #[test]
    fn it_encodes_and_decodes_the_same() {
        let mut map: Map<u32, MockValue> = Map::new();
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
    fn nested_maps_are_ok_too() {
        let mut map: Map<u32, Map<u32, MockValue>> = Map::new();
        let v = MockValue {
            field1: 2u32,
            field2: vec![1, 2, 3, 4],
        };

        let mut nested_map: Map<u32, MockValue> = Map::new();
        nested_map.insert(1, v.clone());

        map.insert(1, nested_map.clone());
        map.insert(2, nested_map);

        let buf = Encode::encode(&map);
        let decoded_map: Map<u32, Map<u32, MockValue>> = Map::decode(&mut &buf[..]).unwrap();

        assert_eq!(map[&1][&1], decoded_map[&1][&1]);
        assert_eq!(map[&2][&1], decoded_map[&2][&1]);
    }

}
