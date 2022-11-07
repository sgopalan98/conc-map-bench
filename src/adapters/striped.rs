use std::sync::Arc;

use bustle::*;
use striped_hashmap::StripedHashMap;

#[derive(Clone)]
pub struct StripedMapTable(Arc<StripedHashMap>);

impl Collection for StripedMapTable
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(StripedHashMap::with_capacity(
            capacity
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl CollectionHandle for StripedMapTable
{
    type Key = u128;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(*key as usize).is_ok()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert_or_update(*key as usize, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(*key as usize).is_ok()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let result = self.0.get(*key as usize);
        let value = match result {
            Ok(val) => val,
            Err(_) => return false,
        };
        return self.0.insert_or_update(*key as usize, value + 1).is_ok();
    }
}
