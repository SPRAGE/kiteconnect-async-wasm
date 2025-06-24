//! # Rate Limiter Module
//! 
//! This module implements per-endpoint rate limiting based on official
//! KiteConnect API limits to prevent exceeding rate limits.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use super::endpoints::{KiteEndpoint, RateLimitCategory};

/// Per-category rate limiter state
#[derive(Debug)]
struct CategoryLimiter {
    /// Last request time for this category
    last_request: Option<Instant>,
    /// Minimum delay between requests
    min_delay: Duration,
    /// Number of requests in current window
    request_count: u32,
    /// Requests per second limit
    requests_per_second: u32,
}

impl CategoryLimiter {
    fn new(category: RateLimitCategory) -> Self {
        Self {
            last_request: None,
            min_delay: category.min_delay(),
            request_count: 0,
            requests_per_second: category.requests_per_second(),
        }
    }

    /// Check if a request can be made immediately
    fn can_request_now(&self) -> bool {
        if let Some(last) = self.last_request {
            last.elapsed() >= self.min_delay
        } else {
            true
        }
    }

    /// Calculate delay needed before next request
    fn delay_until_next_request(&self) -> Duration {
        if let Some(last) = self.last_request {
            let elapsed = last.elapsed();
            if elapsed < self.min_delay {
                self.min_delay - elapsed
            } else {
                Duration::ZERO
            }
        } else {
            Duration::ZERO
        }
    }

    /// Record a request
    fn record_request(&mut self) {
        self.last_request = Some(Instant::now());
        self.request_count += 1;
    }

    /// Reset request count (called every second)
    fn reset_count(&mut self) {
        self.request_count = 0;
    }
}

/// Rate limiter for KiteConnect API endpoints
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Rate limiters per category
    limiters: Arc<Mutex<HashMap<RateLimitCategory, CategoryLimiter>>>,
    /// Whether rate limiting is enabled
    enabled: bool,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(enabled: bool) -> Self {
        let mut limiters = HashMap::new();
        
        // Initialize limiters for all categories
        limiters.insert(RateLimitCategory::Quote, CategoryLimiter::new(RateLimitCategory::Quote));
        limiters.insert(RateLimitCategory::Historical, CategoryLimiter::new(RateLimitCategory::Historical));
        limiters.insert(RateLimitCategory::Orders, CategoryLimiter::new(RateLimitCategory::Orders));
        limiters.insert(RateLimitCategory::Standard, CategoryLimiter::new(RateLimitCategory::Standard));
        
        Self {
            limiters: Arc::new(Mutex::new(limiters)),
            enabled,
        }
    }

    /// Wait for rate limit compliance before making a request
    /// 
    /// This method will return immediately if no delay is needed,
    /// or will sleep for the required duration to comply with rate limits.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - The endpoint being accessed
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::{RateLimiter, KiteEndpoint};
    /// 
    /// # #[tokio::main]
    /// # async fn main() {
    /// let rate_limiter = RateLimiter::new(true);
    /// 
    /// // This will wait if needed to comply with rate limits
    /// rate_limiter.wait_for_request(&KiteEndpoint::Quote).await;
    /// 
    /// // Now it's safe to make the API request
    /// println!("Making quote request...");
    /// # }
    /// ```
    pub async fn wait_for_request(&self, endpoint: &KiteEndpoint) {
        if !self.enabled {
            return;
        }

        let category = endpoint.rate_limit_category();
        let delay = {
            let limiters = self.limiters.lock().await;
            if let Some(limiter) = limiters.get(&category) {
                limiter.delay_until_next_request()
            } else {
                Duration::ZERO
            }
        };

        if delay > Duration::ZERO {
            #[cfg(feature = "debug")]
            log::debug!("Rate limiting: waiting {:?} for {:?} category", delay, category);
            
            tokio::time::sleep(delay).await;
        }

        // Record the request
        let mut limiters = self.limiters.lock().await;
        if let Some(limiter) = limiters.get_mut(&category) {
            limiter.record_request();
        }
    }

    /// Check if a request can be made without waiting
    /// 
    /// Returns true if the request can be made immediately,
    /// false if rate limiting would cause a delay.
    pub async fn can_request_immediately(&self, endpoint: &KiteEndpoint) -> bool {
        if !self.enabled {
            return true;
        }

        let category = endpoint.rate_limit_category();
        let limiters = self.limiters.lock().await;
        
        if let Some(limiter) = limiters.get(&category) {
            limiter.can_request_now()
        } else {
            true
        }
    }

    /// Get the delay required before making a request
    /// 
    /// Returns Duration::ZERO if no delay is needed.
    pub async fn get_delay_for_request(&self, endpoint: &KiteEndpoint) -> Duration {
        if !self.enabled {
            return Duration::ZERO;
        }

        let category = endpoint.rate_limit_category();
        let limiters = self.limiters.lock().await;
        
        if let Some(limiter) = limiters.get(&category) {
            limiter.delay_until_next_request()
        } else {
            Duration::ZERO
        }
    }

    /// Get rate limiter statistics
    /// 
    /// Returns information about current rate limiter state for monitoring.
    pub async fn get_stats(&self) -> RateLimiterStats {
        let limiters = self.limiters.lock().await;
        let mut categories = HashMap::new();
        
        for (category, limiter) in limiters.iter() {
            categories.insert(category.clone(), CategoryStats {
                request_count: limiter.request_count,
                requests_per_second: limiter.requests_per_second,
                last_request: limiter.last_request,
                next_available: limiter.last_request.map(|last| last + limiter.min_delay),
            });
        }
        
        RateLimiterStats {
            enabled: self.enabled,
            categories,
        }
    }

    /// Enable or disable rate limiting
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if rate limiting is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(true)
    }
}

/// Statistics about rate limiter state
#[derive(Debug, Clone)]
pub struct RateLimiterStats {
    /// Whether rate limiting is enabled
    pub enabled: bool,
    /// Per-category statistics
    pub categories: HashMap<RateLimitCategory, CategoryStats>,
}

/// Statistics for a specific rate limit category
#[derive(Debug, Clone)]
pub struct CategoryStats {
    /// Current request count in this second
    pub request_count: u32,
    /// Maximum requests per second for this category
    pub requests_per_second: u32,
    /// When the last request was made
    pub last_request: Option<Instant>,
    /// When the next request can be made
    pub next_available: Option<Instant>,
}

impl CategoryStats {
    /// Check if this category is currently at its rate limit
    pub fn is_at_limit(&self) -> bool {
        if let Some(next) = self.next_available {
            next > Instant::now()
        } else {
            false
        }
    }

    /// Get remaining capacity for this category
    pub fn remaining_capacity(&self) -> u32 {
        self.requests_per_second.saturating_sub(self.request_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rate_limiter_basic_functionality() {
        let rate_limiter = RateLimiter::new(true);
        
        // First request should be immediate
        assert!(rate_limiter.can_request_immediately(&KiteEndpoint::Quote).await);
        
        // Wait for request (should be immediate)
        let start = Instant::now();
        rate_limiter.wait_for_request(&KiteEndpoint::Quote).await;
        assert!(start.elapsed() < Duration::from_millis(10));
        
        // Second request should require waiting (quote is 1 req/sec)
        assert!(!rate_limiter.can_request_immediately(&KiteEndpoint::Quote).await);
        
        let delay = rate_limiter.get_delay_for_request(&KiteEndpoint::Quote).await;
        assert!(delay > Duration::from_millis(900)); // Should be close to 1 second
    }

    #[tokio::test]
    async fn test_rate_limiter_disabled() {
        let rate_limiter = RateLimiter::new(false);
        
        // All requests should be immediate when disabled
        rate_limiter.wait_for_request(&KiteEndpoint::Quote).await;
        assert!(rate_limiter.can_request_immediately(&KiteEndpoint::Quote).await);
        
        let delay = rate_limiter.get_delay_for_request(&KiteEndpoint::Quote).await;
        assert_eq!(delay, Duration::ZERO);
    }

    #[tokio::test]
    async fn test_different_categories() {
        let rate_limiter = RateLimiter::new(true);
        
        // Make a quote request
        rate_limiter.wait_for_request(&KiteEndpoint::Quote).await;
        
        // Historical request should still be available (different category)
        assert!(rate_limiter.can_request_immediately(&KiteEndpoint::HistoricalData).await);
        
        // Standard requests should be available (different category)
        assert!(rate_limiter.can_request_immediately(&KiteEndpoint::Holdings).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_stats() {
        let rate_limiter = RateLimiter::new(true);
        
        // Make some requests
        rate_limiter.wait_for_request(&KiteEndpoint::Quote).await;
        rate_limiter.wait_for_request(&KiteEndpoint::Holdings).await;
        
        let stats = rate_limiter.get_stats().await;
        assert!(stats.enabled);
        
        // Quote category should have 1 request
        let quote_stats = &stats.categories[&RateLimitCategory::Quote];
        assert_eq!(quote_stats.request_count, 1);
        assert_eq!(quote_stats.requests_per_second, 1);
        assert!(quote_stats.last_request.is_some());
        
        // Standard category should have 1 request
        let standard_stats = &stats.categories[&RateLimitCategory::Standard];
        assert_eq!(standard_stats.request_count, 1);
        assert_eq!(standard_stats.requests_per_second, 10);
    }

    #[tokio::test]
    async fn test_category_stats() {
        let stats = CategoryStats {
            request_count: 5,
            requests_per_second: 10,
            last_request: Some(Instant::now()),
            next_available: Some(Instant::now() + Duration::from_millis(100)),
        };
        
        assert!(stats.is_at_limit());
        assert_eq!(stats.remaining_capacity(), 5);
        
        let stats_not_at_limit = CategoryStats {
            request_count: 3,
            requests_per_second: 10,
            last_request: Some(Instant::now() - Duration::from_secs(1)),
            next_available: Some(Instant::now() - Duration::from_millis(100)),
        };
        
        assert!(!stats_not_at_limit.is_at_limit());
        assert_eq!(stats_not_at_limit.remaining_capacity(), 7);
    }
}
