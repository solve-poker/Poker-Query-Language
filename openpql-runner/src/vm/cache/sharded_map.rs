use std::{
    hash::{BuildHasher, Hash},
    sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use rustc_hash::{FxBuildHasher, FxHashMap};

const N_SHARDS: usize = 16;
const MAX_SHARD_LEN: usize = 1 << 30;

type Shard<K, V> = RwLock<FxHashMap<K, V>>;

/// Hash map split into independently locked shards to reduce contention.
#[derive(Debug)]
pub(super) struct ShardedMap<K, V> {
    shards: [Shard<K, V>; N_SHARDS],
}

impl<K, V> Default for ShardedMap<K, V> {
    fn default() -> Self {
        Self {
            shards: Default::default(),
        }
    }
}

impl<K: Hash + Eq, V: Copy> ShardedMap<K, V> {
    /// Returns the value stored for `key`.
    pub(super) fn get(&self, key: &K) -> Option<V> {
        read(self.shard(key)).get(key).copied()
    }

    /// Inserts `key -> value`, clearing the shard first when full.
    pub(super) fn insert(&self, key: K, value: V) {
        insert_bounded(&mut write(self.shard(&key)), key, value, MAX_SHARD_LEN);
    }

    fn shard(&self, key: &K) -> &Shard<K, V> {
        let idx = (FxBuildHasher.hash_one(key) >> 60) as usize;

        &self.shards[idx]
    }
}

/// Inserts into `map`, clearing it first when `max_len` is reached.
fn insert_bounded<K: Hash + Eq, V>(map: &mut FxHashMap<K, V>, key: K, value: V, max_len: usize) {
    if map.len() >= max_len {
        map.clear();
    }

    map.insert(key, value);
}

fn read<K, V>(shard: &Shard<K, V>) -> RwLockReadGuard<'_, FxHashMap<K, V>> {
    shard.read().unwrap_or_else(PoisonError::into_inner)
}

fn write<K, V>(shard: &Shard<K, V>) -> RwLockWriteGuard<'_, FxHashMap<K, V>> {
    shard.write().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_insert() {
        let map = ShardedMap::default();

        assert_eq!(map.get(&7), None);

        map.insert(7, 49);

        assert_eq!(map.get(&7), Some(49));
    }

    #[test]
    fn test_keys_spread_across_shards() {
        let map = ShardedMap::default();

        for key in 0..100 {
            map.insert(key, key * 2);
        }

        for key in 0..100 {
            assert_eq!(map.get(&key), Some(key * 2));
        }

        let n_used = map.shards.iter().filter(|s| !read(s).is_empty()).count();

        assert!(n_used > 1);
    }

    #[test]
    fn test_insert_bounded_clears_full_map() {
        let mut map = FxHashMap::default();
        let max_len = 4;

        map.extend((0..max_len).map(|i| (i, i)));

        assert_eq!(map.len(), max_len);

        insert_bounded(&mut map, 9, 9, max_len);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&9), Some(&9));
    }
}
