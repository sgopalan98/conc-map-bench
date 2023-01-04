use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;
use leapfrog::LeapMap;

use super::Value;

#[derive(Clone)]
pub struct LeapfrogMapTable<K, H>(Arc<LeapMap<K, Value, H>>);

impl<K, H> Collection for LeapfrogMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(LeapMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for LeapfrogMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(&key).map_or(false, |_| true)
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        match self.0.get_mut(key) {
            Some(mut val_ref) => return val_ref.update(|val: &mut Value| *val += 1).is_some(),
            None => {
                return false;
            }
        };
    }
}