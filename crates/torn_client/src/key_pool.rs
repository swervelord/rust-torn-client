//! API key pool management with round-robin and random balancing.

use crate::config::ApiKeyBalancing;
use crate::Error;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Manages a pool of API keys with configurable balancing strategies.
///
/// This struct is thread-safe (`Send + Sync`) and can be shared across
/// multiple async tasks via `Arc<TornClient>`.
#[derive(Debug)]
pub(crate) struct KeyPool {
    keys: Vec<String>,
    balancing: ApiKeyBalancing,
    /// Current index for round-robin selection (atomic for thread safety).
    index: AtomicUsize,
}

impl KeyPool {
    /// Create a new key pool from a list of API keys.
    ///
    /// Returns an error if the keys list is empty.
    pub(crate) fn new(keys: Vec<String>, balancing: ApiKeyBalancing) -> Result<Self, Error> {
        if keys.is_empty() {
            return Err(Error::NoKeys);
        }
        Ok(Self {
            keys,
            balancing,
            index: AtomicUsize::new(0),
        })
    }

    /// Get the next API key according to the balancing strategy.
    ///
    /// For `RoundRobin`, keys are returned in a cyclic order.
    /// For `Random`, a key is selected randomly using a simple LCG algorithm
    /// (to avoid adding a `rand` dependency).
    pub(crate) fn next_key(&self) -> &str {
        match self.balancing {
            ApiKeyBalancing::RoundRobin => {
                let idx = self.index.fetch_add(1, Ordering::Relaxed) % self.keys.len();
                &self.keys[idx]
            }
            ApiKeyBalancing::Random => {
                // Simple random selection using a Linear Congruential Generator (LCG)
                // to avoid adding a `rand` dependency.
                // LCG formula: next = (a * seed + c) % m
                let seed = self.index.fetch_add(1, Ordering::Relaxed);
                let random = simple_lcg(seed);
                let idx = random % self.keys.len();
                &self.keys[idx]
            }
        }
    }

    /// Get the API key at a specific index (for testing and rate limiter).
    pub(crate) fn get_key(&self, index: usize) -> Option<&str> {
        self.keys.get(index).map(|s| s.as_str())
    }

    /// Number of keys in the pool.
    pub(crate) fn len(&self) -> usize {
        self.keys.len()
    }

    /// List all keys with masking (first 5 chars + "..." for logging/debugging).
    pub(crate) fn keys_masked(&self) -> Vec<String> {
        self.keys
            .iter()
            .map(|key| {
                if key.len() > 5 {
                    format!("{}...", &key[..5])
                } else {
                    key.clone()
                }
            })
            .collect()
    }

    /// Iterate over all keys in the pool.
    pub(crate) fn iter_keys(&self) -> impl Iterator<Item = &str> {
        self.keys.iter().map(|s| s.as_str())
    }
}

/// Simple Linear Congruential Generator (LCG) for pseudo-random numbers.
///
/// Uses parameters from Numerical Recipes:
/// - a = 1664525
/// - c = 1013904223
/// - m = 2^32 (implicit via wrapping)
fn simple_lcg(seed: usize) -> usize {
    const A: usize = 1664525;
    const C: usize = 1013904223;
    seed.wrapping_mul(A).wrapping_add(C)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin_single_key() {
        let pool = KeyPool::new(vec!["key1".to_string()], ApiKeyBalancing::RoundRobin).unwrap();
        assert_eq!(pool.next_key(), "key1");
        assert_eq!(pool.next_key(), "key1");
        assert_eq!(pool.next_key(), "key1");
    }

    #[test]
    fn test_round_robin_multiple_keys() {
        let pool = KeyPool::new(
            vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
            ],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();

        // Test that keys cycle correctly
        assert_eq!(pool.next_key(), "key1");
        assert_eq!(pool.next_key(), "key2");
        assert_eq!(pool.next_key(), "key3");
        assert_eq!(pool.next_key(), "key1");
        assert_eq!(pool.next_key(), "key2");
        assert_eq!(pool.next_key(), "key3");
    }

    #[test]
    fn test_random_selection_returns_valid_keys() {
        let pool = KeyPool::new(
            vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
            ],
            ApiKeyBalancing::Random,
        )
        .unwrap();

        // Generate 100 random selections and verify all are valid keys
        for _ in 0..100 {
            let key = pool.next_key();
            assert!(key == "key1" || key == "key2" || key == "key3");
        }
    }

    #[test]
    fn test_random_has_variation() {
        let pool = KeyPool::new(
            vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
            ],
            ApiKeyBalancing::Random,
        )
        .unwrap();

        // Collect 30 selections - with random, we should see at least 2 different keys
        let mut keys = std::collections::HashSet::new();
        for _ in 0..30 {
            keys.insert(pool.next_key().to_string());
        }

        // With LCG, we should see variation across 30 picks
        assert!(
            keys.len() >= 2,
            "Expected at least 2 different keys in random mode, got {:?}",
            keys
        );
    }

    #[test]
    fn test_empty_keys_error() {
        let result = KeyPool::new(vec![], ApiKeyBalancing::RoundRobin);
        assert!(matches!(result, Err(Error::NoKeys)));
    }

    #[test]
    fn test_len() {
        let pool = KeyPool::new(
            vec!["key1".to_string(), "key2".to_string()],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_keys_masked() {
        let pool = KeyPool::new(
            vec![
                "abcdefgh12345".to_string(),
                "xyz".to_string(),
                "12345678".to_string(),
            ],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();

        let masked = pool.keys_masked();
        assert_eq!(masked.len(), 3);
        assert_eq!(masked[0], "abcde...");
        assert_eq!(masked[1], "xyz"); // Short keys not masked
        assert_eq!(masked[2], "12345...");
    }

    #[test]
    fn test_get_key() {
        let pool = KeyPool::new(
            vec!["key1".to_string(), "key2".to_string()],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();

        assert_eq!(pool.get_key(0), Some("key1"));
        assert_eq!(pool.get_key(1), Some("key2"));
        assert_eq!(pool.get_key(2), None);
    }
}
