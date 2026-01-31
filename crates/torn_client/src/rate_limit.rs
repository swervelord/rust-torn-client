//! Rate limiting with per-key and per-IP tracking.
//!
//! Implements sliding window rate limiting:
//! - 100 requests per 60 seconds per API key
//! - 1000 requests per 60 seconds per IP (across all keys)
//!
//! Supports three rate limit modes:
//! - `AutoDelay`: Automatically wait when rate limit is reached
//! - `ThrowOnLimit`: Return an error when rate limit would be exceeded
//! - `Ignore`: Bypass rate limiting entirely

use crate::config::RateLimitMode;
use crate::key_pool::KeyPool;
use crate::Error;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Maximum requests per 60-second window per API key.
const PER_KEY_LIMIT: usize = 100;

/// Maximum requests per 60-second window per IP (across all keys).
const PER_IP_LIMIT: usize = 1000;

/// Rate limit window duration (60 seconds).
const WINDOW_DURATION: Duration = Duration::from_secs(60);

/// Extra buffer time to add when waiting for rate limit reset.
const WAIT_BUFFER: Duration = Duration::from_millis(100);

/// Tracks rate limit state for API requests.
///
/// Uses a sliding window algorithm: timestamps older than 60 seconds
/// are pruned before each availability check.
///
/// This struct is thread-safe (`Send + Sync`) via internal `Mutex`.
#[derive(Debug)]
pub(crate) struct RateLimiter {
    /// Timestamps of recent requests, per API key.
    timestamps: Mutex<HashMap<String, Vec<Instant>>>,
    /// Timestamps of all requests (for per-IP tracking).
    ip_timestamps: Mutex<Vec<Instant>>,
    /// Rate limit mode.
    mode: RateLimitMode,
}

impl RateLimiter {
    /// Create a new rate limiter with the specified mode.
    pub(crate) fn new(mode: RateLimitMode) -> Self {
        Self {
            timestamps: Mutex::new(HashMap::new()),
            ip_timestamps: Mutex::new(Vec::new()),
            mode,
        }
    }

    /// Check if a specific API key is available (under its rate limit).
    ///
    /// This method prunes expired timestamps before checking.
    pub(crate) fn is_key_available(&self, key: &str) -> bool {
        match self.mode {
            RateLimitMode::Ignore => true,
            _ => {
                let mut timestamps = self.timestamps.lock().unwrap();
                Self::prune_timestamps(timestamps.entry(key.to_string()).or_default());

                let key_count = timestamps.get(key).map(|v| v.len()).unwrap_or(0);
                key_count < PER_KEY_LIMIT
            }
        }
    }

    /// Check if the per-IP limit has been reached.
    fn is_ip_available(&self) -> bool {
        match self.mode {
            RateLimitMode::Ignore => true,
            _ => {
                let mut timestamps = self.ip_timestamps.lock().unwrap();
                Self::prune_timestamps(&mut timestamps);
                timestamps.len() < PER_IP_LIMIT
            }
        }
    }

    /// Record that a request was made with the given key.
    ///
    /// Updates both per-key and per-IP timestamp tracking.
    pub(crate) fn record_request(&self, key: &str) {
        if matches!(self.mode, RateLimitMode::Ignore) {
            return;
        }

        let now = Instant::now();

        // Record per-key timestamp
        let mut timestamps = self.timestamps.lock().unwrap();
        timestamps.entry(key.to_string()).or_default().push(now);

        // Record per-IP timestamp
        let mut ip_timestamps = self.ip_timestamps.lock().unwrap();
        ip_timestamps.push(now);
    }

    /// Find an available key from the pool, respecting rate limits.
    ///
    /// Tries each key in the pool until one is found that's under its limit.
    /// Returns `None` if all keys are exhausted.
    pub(crate) fn find_available_key(&self, pool: &KeyPool) -> Option<String> {
        match self.mode {
            RateLimitMode::Ignore => Some(pool.next_key().to_string()),
            _ => {
                // First check if we've hit the per-IP limit
                if !self.is_ip_available() {
                    return None;
                }

                // Try to find an available key by checking each one
                let key_count = pool.len();
                for i in 0..key_count {
                    if let Some(key) = pool.get_key(i) {
                        if self.is_key_available(key) {
                            return Some(key.to_string());
                        }
                    }
                }
                None
            }
        }
    }

    /// Wait until a key becomes available (for AutoDelay mode).
    ///
    /// Returns the key when available. In `Ignore` mode, returns immediately.
    /// In `ThrowOnLimit` mode, returns an error if no key is available.
    /// In `AutoDelay` mode, waits until a key becomes available.
    pub(crate) async fn wait_for_available_key(&self, pool: &KeyPool) -> Result<String, Error> {
        match self.mode {
            RateLimitMode::Ignore => Ok(pool.next_key().to_string()),
            RateLimitMode::ThrowOnLimit => {
                self.find_available_key(pool).ok_or(Error::RateLimited)
            }
            RateLimitMode::AutoDelay => {
                loop {
                    if let Some(key) = self.find_available_key(pool) {
                        return Ok(key);
                    }

                    // Calculate how long to wait
                    let wait_time = self.min_wait_time();
                    tokio::time::sleep(wait_time).await;
                }
            }
        }
    }

    /// Get rate limit information for all tracked keys.
    ///
    /// Returns a map of key prefix -> rate limit info.
    pub(crate) fn get_rate_limit_info(&self) -> HashMap<String, RateLimitInfo> {
        let mut result = HashMap::new();

        if matches!(self.mode, RateLimitMode::Ignore) {
            return result;
        }

        let mut timestamps = self.timestamps.lock().unwrap();

        for (key, times) in timestamps.iter_mut() {
            Self::prune_timestamps(times);

            let used = times.len() as u32;
            let remaining = PER_KEY_LIMIT.saturating_sub(used as usize) as u32;

            // Calculate reset time (time until oldest timestamp expires)
            let reset_in_ms = if let Some(oldest) = times.first() {
                let elapsed = Instant::now().duration_since(*oldest);
                WINDOW_DURATION
                    .saturating_sub(elapsed)
                    .as_millis()
                    .min(u64::MAX as u128) as u64
            } else {
                0
            };

            // Mask the key (first 5 chars)
            let key_prefix = if key.len() > 5 {
                format!("{}...", &key[..5])
            } else {
                key.clone()
            };

            result.insert(
                key_prefix,
                RateLimitInfo {
                    used,
                    remaining,
                    reset_in_ms,
                },
            );
        }

        result
    }

    /// Compute the minimum wait time until any key becomes available.
    fn min_wait_time(&self) -> Duration {
        let timestamps = self.timestamps.lock().unwrap();
        let now = Instant::now();

        let mut min_wait = Duration::from_secs(61); // Default fallback

        // Check per-key limits
        for times in timestamps.values() {
            if times.is_empty() {
                continue;
            }

            // If this key has requests, calculate when the oldest will expire
            if times.len() >= PER_KEY_LIMIT {
                let oldest = times[0];
                let elapsed = now.duration_since(oldest);
                let wait = WINDOW_DURATION.saturating_sub(elapsed) + WAIT_BUFFER;
                if wait < min_wait {
                    min_wait = wait;
                }
            } else {
                // This key has capacity, so no need to wait
                return Duration::from_millis(0);
            }
        }

        // Check per-IP limit
        let ip_timestamps = self.ip_timestamps.lock().unwrap();
        if ip_timestamps.len() >= PER_IP_LIMIT {
            if let Some(oldest) = ip_timestamps.first() {
                let elapsed = now.duration_since(*oldest);
                let wait = WINDOW_DURATION.saturating_sub(elapsed) + WAIT_BUFFER;
                if wait < min_wait {
                    min_wait = wait;
                }
            }
        }

        min_wait
    }

    /// Prune timestamps older than the window duration.
    fn prune_timestamps(timestamps: &mut Vec<Instant>) {
        let now = Instant::now();
        timestamps.retain(|&ts| now.duration_since(ts) < WINDOW_DURATION);
    }
}

/// Rate limit information for a single API key.
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Number of requests used in the current window.
    pub used: u32,
    /// Number of requests remaining in the current window.
    pub remaining: u32,
    /// Milliseconds until the rate limit resets.
    pub reset_in_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ApiKeyBalancing;

    #[test]
    fn test_rate_limiter_allows_requests_under_limit() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let key = "test-key";

        // Should be available initially
        assert!(limiter.is_key_available(key));

        // Record 50 requests - should still be under limit
        for _ in 0..50 {
            limiter.record_request(key);
        }

        assert!(limiter.is_key_available(key));
    }

    #[test]
    fn test_rate_limiter_blocks_at_limit() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let key = "test-key";

        // Fill up to the limit
        for _ in 0..PER_KEY_LIMIT {
            limiter.record_request(key);
        }

        // Should now be unavailable
        assert!(!limiter.is_key_available(key));
    }

    #[test]
    fn test_timestamp_pruning() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let key = "test-key";

        // Manually insert old timestamps
        {
            let mut timestamps = limiter.timestamps.lock().unwrap();
            let old_time = Instant::now() - Duration::from_secs(70);
            timestamps.insert(
                key.to_string(),
                vec![old_time; PER_KEY_LIMIT],
            );
        }

        // After pruning, should be available again
        assert!(limiter.is_key_available(key));
    }

    #[test]
    fn test_get_rate_limit_info() {
        let limiter = RateLimiter::new(RateLimitMode::AutoDelay);
        let key = "abcdef123456";

        // Record some requests
        for _ in 0..25 {
            limiter.record_request(key);
        }

        let info = limiter.get_rate_limit_info();
        assert_eq!(info.len(), 1);

        let key_info = info.get("abcde...").unwrap();
        assert_eq!(key_info.used, 25);
        assert_eq!(key_info.remaining, 75);
        assert!(key_info.reset_in_ms > 0);
    }

    #[test]
    fn test_ignore_mode_always_available() {
        let limiter = RateLimiter::new(RateLimitMode::Ignore);
        let key = "test-key";

        // Record way over the limit
        for _ in 0..200 {
            limiter.record_request(key);
        }

        // Should still be available in Ignore mode
        assert!(limiter.is_key_available(key));
    }

    #[tokio::test]
    async fn test_throw_on_limit_returns_error() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let pool = KeyPool::new(vec!["key1".to_string()], ApiKeyBalancing::RoundRobin).unwrap();

        // Fill up the limit
        for _ in 0..PER_KEY_LIMIT {
            limiter.record_request("key1");
        }

        // Should return RateLimited error
        let result = limiter.wait_for_available_key(&pool).await;
        assert!(matches!(result, Err(Error::RateLimited)));
    }

    #[tokio::test]
    async fn test_ignore_mode_always_returns_key() {
        let limiter = RateLimiter::new(RateLimitMode::Ignore);
        let pool = KeyPool::new(vec!["key1".to_string()], ApiKeyBalancing::RoundRobin).unwrap();

        // Fill way over the limit
        for _ in 0..200 {
            limiter.record_request("key1");
        }

        // Should still return a key
        let result = limiter.wait_for_available_key(&pool).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "key1");
    }

    #[tokio::test]
    async fn test_find_available_key_cycles_through_pool() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let pool = KeyPool::new(
            vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
            ],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();

        // Fill up key1
        for _ in 0..PER_KEY_LIMIT {
            limiter.record_request("key1");
        }

        // Should find key2 or key3
        let available = limiter.find_available_key(&pool);
        assert!(available.is_some());
        let key = available.unwrap();
        assert!(key == "key2" || key == "key3");
    }

    #[tokio::test]
    async fn test_auto_delay_waits_correctly() {
        // Use tokio's time pause for deterministic testing
        tokio::time::pause();

        let limiter = RateLimiter::new(RateLimitMode::AutoDelay);
        let pool = KeyPool::new(vec!["key1".to_string()], ApiKeyBalancing::RoundRobin).unwrap();

        // Fill up to the limit
        for _ in 0..PER_KEY_LIMIT {
            limiter.record_request("key1");
        }

        // Record the start time
        let start = Instant::now();

        // Spawn the wait task
        let wait_task = tokio::spawn(async move {
            limiter.wait_for_available_key(&pool).await
        });

        // Advance time by 60 seconds + buffer
        tokio::time::advance(WINDOW_DURATION + WAIT_BUFFER).await;

        // The task should complete now
        let result = wait_task.await.unwrap();
        assert!(result.is_ok());

        // Verify some time has passed (in test time)
        let elapsed = Instant::now().duration_since(start);
        assert!(elapsed >= WINDOW_DURATION);
    }

    #[test]
    fn test_per_ip_limit() {
        let limiter = RateLimiter::new(RateLimitMode::ThrowOnLimit);
        let pool = KeyPool::new(
            vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
            ],
            ApiKeyBalancing::RoundRobin,
        )
        .unwrap();

        // Record requests across multiple keys up to per-IP limit
        for i in 0..PER_IP_LIMIT {
            let key = pool.get_key(i % 3).unwrap();
            limiter.record_request(key);
        }

        // Should now hit per-IP limit
        let available = limiter.find_available_key(&pool);
        assert!(available.is_none());
    }

    #[test]
    fn test_concurrent_access_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let limiter = Arc::new(RateLimiter::new(RateLimitMode::AutoDelay));
        let mut handles = vec![];

        // Spawn multiple threads that record requests concurrently
        for i in 0..10 {
            let limiter_clone = Arc::clone(&limiter);
            let handle = thread::spawn(move || {
                let key = format!("key{}", i % 3);
                for _ in 0..10 {
                    limiter_clone.record_request(&key);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify that some requests were recorded
        let info = limiter.get_rate_limit_info();
        assert!(!info.is_empty());
    }
}
