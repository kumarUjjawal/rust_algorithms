use std::collections::{HashMap, VecDeque};

struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: std::hash::Hash + Eq + Clone, V> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.cache.contains_key(key) {
            // Move key to the back (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.cache.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.cache.contains_key(&key) {
            // Remove existing key from order
            self.order.retain(|k| k != &key);
        } else if self.cache.len() >= self.capacity {
            // Remove least recently used key
            if let Some(lru_key) = self.order.pop_front() {
                self.cache.remove(&lru_key);
            }
        }
        // Insert new key-value pair
        self.cache.insert(key.clone(), value);
        self.order.push_back(key);
    }
}

#[cfg(test)] // Runs only during tests
mod tests {
    use super::*; // Import everything from the main module

    #[test]
    fn test_insert_and_retrieve() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));
    }
}
