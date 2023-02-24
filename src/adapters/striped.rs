use std::{sync::{Arc, RwLock}, collections::HashMap};

use bustle::*;

#[derive(Clone)]
pub struct StripedMapTable(Arc<Vec<RwLock<HashMap<u128, u128>>>>);

impl Collection for StripedMapTable
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        let no_buckets = (num_cpus::get() * 4).next_power_of_two();
        println!("No of buckets = {}", no_buckets);
        let capacity_per_bucket = capacity / no_buckets;
        let mut buckets = Vec::new();
        for _i in 0..no_buckets {
            buckets.push(RwLock::new(HashMap::with_capacity(capacity_per_bucket)));
        }
        Self(Arc::new(buckets))
    }

    fn pin(&self) -> Self::Handle {
        let map = &self.0;
        Self(Arc::clone(&map))
    }
}

impl CollectionHandle for StripedMapTable
{
    type Key = u128;

    fn get(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let bucket = match buckets[index].read() {
            Ok(bucket) => bucket,
            Err(_) => panic!("BUCKET NOT FOUND"),
        };
        bucket.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = match buckets[index].write() {
            Ok(bucket) => bucket,
            Err(_) => panic!("BUCKET NOT FOUND"),
        };
        bucket.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = match buckets[index].write() {
            Ok(bucket) => bucket,
            Err(_) => panic!("BUCKET NOT FOUND"),
        };
        bucket.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = match buckets[index].write() {
            Ok(bucket) => bucket,
            Err(_) => panic!("BUCKET NOT FOUND"),
        };
        bucket.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}
