# Phase 8: Connect Module Integration Guide

## Overview

Phase 8 focuses on integrating the completed typed models with the existing connect modules, implementing dual API support (typed + legacy), and enhancing the KiteConnect client with modern Rust patterns.

## Current State

**✅ Completed (Phases 1-7):**
- All domain models implemented: `auth`, `orders`, `portfolio`, `market_data`, `mutual_funds`, `gtt`
- Comprehensive error handling with `KiteError`
- Builder patterns for complex operations
- 5,000+ lines of production-ready Rust code
- 18 passing unit tests + 30 passing doc tests

**🎯 Phase 8 Goals:**
- Integrate typed models with `src/connect/` modules
- Implement dual API support (typed + legacy)
- Add retry logic and reconnection mechanisms
- Performance optimizations and caching
- Enhanced error handling and logging

## Implementation Plan

### 8.1 Connect Module Enhancement

#### 8.1.1 Update `src/connect/mod.rs`
**Current Structure:**
```rust
pub struct KiteConnect {
    api_key: String,
    access_token: Option<String>,
    root: String,
    timeout: u64,
    session_expiry_hook: Option<fn() -> ()>,
    client: reqwest::Client,
}
```

**Enhanced Structure:**
```rust
use crate::models::prelude::*;

pub struct KiteConnect {
    // Existing fields
    api_key: String,
    access_token: Option<String>,
    root: String,
    timeout: u64,
    session_expiry_hook: Option<fn() -> ()>,
    client: reqwest::Client,
    
    // New fields for v1.0.0
    retry_config: RetryConfig,
    cache_config: Option<CacheConfig>,
    request_counter: Arc<AtomicU64>,
    rate_limiter: Option<RateLimiter>,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub exponential_backoff: bool,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enable_instruments_cache: bool,
    pub cache_ttl_minutes: u64,
    pub max_cache_size: usize,
}
```

#### 8.1.2 Implement Dual API Pattern
Each connect method should have two variants:

**Pattern:**
```rust
impl KiteConnect {
    // Typed API (new, recommended)
    pub async fn holdings(&self) -> KiteResult<Vec<Holding>> {
        let response = self.holdings_raw().await?;
        self.parse_response(response)
    }
    
    // Legacy API (existing, for backward compatibility)
    pub async fn holdings_raw(&self) -> Result<JsonValue> {
        // Existing implementation
    }
    
    // Internal helper for parsing
    fn parse_response<T: DeserializeOwned>(&self, response: JsonValue) -> KiteResult<T> {
        serde_json::from_value(response)
            .map_err(|e| KiteError::Json(e))
    }
}
```

### 8.2 Authentication Module (`src/connect/auth.rs`)

#### 8.2.1 Add Typed Methods
```rust
impl KiteConnect {
    // Typed session generation
    pub async fn generate_session_typed(&self, request_token: &str, api_secret: &str) -> KiteResult<SessionData> {
        let response = self.generate_session(request_token, api_secret).await?;
        self.parse_response(response)
    }
    
    // Typed user profile
    pub async fn profile(&self) -> KiteResult<UserProfile> {
        let response = self.profile_raw().await?;
        self.parse_response(response)
    }
    
    // Typed margins
    pub async fn margins(&self, segment: Option<TradingSegment>) -> KiteResult<MarginData> {
        let response = self.margins_raw(segment.map(|s| s.as_str())).await?;
        self.parse_response(response)
    }
}
```

#### 8.2.2 Add Authentication Utilities
```rust
impl KiteConnect {
    /// Validate access token and refresh if needed
    pub async fn validate_session(&self) -> KiteResult<bool> {
        match self.profile().await {
            Ok(_) => Ok(true),
            Err(KiteError::Authentication(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    /// Auto-refresh session using stored credentials
    pub async fn refresh_session(&mut self) -> KiteResult<SessionData> {
        if let Some(hook) = &self.session_expiry_hook {
            hook();
        }
        // Implementation depends on stored refresh token
        todo!("Implement refresh logic")
    }
}
```

### 8.3 Orders Module (`src/connect/orders.rs`)

#### 8.3.1 Enhanced Order Methods
```rust
impl KiteConnect {
    // Typed order placement with builder
    pub async fn place_order(&self, params: OrderParams) -> KiteResult<OrderResponse> {
        params.validate()?;
        let response = self.place_order_raw(
            params.variety.as_str(),
            params.exchange.as_str(),
            &params.trading_symbol,
            params.transaction_type.as_str(),
            params.quantity,
            params.product.as_str(),
            params.order_type.as_str(),
            params.price,
            params.trigger_price,
            params.validity.as_str(),
            params.disclosed_quantity,
            params.squareoff,
            params.stoploss,
            params.trailing_stoploss,
            params.tag.as_deref(),
        ).await?;
        self.parse_response(response)
    }
    
    // Typed order history
    pub async fn orders(&self) -> KiteResult<Vec<Order>> {
        let response = self.orders_raw().await?;
        self.parse_response(response)
    }
    
    // Typed order details
    pub async fn order_history(&self, order_id: &str) -> KiteResult<OrderHistory> {
        let response = self.order_history_raw(order_id).await?;
        self.parse_response(response)
    }
    
    // Enhanced order modification with validation
    pub async fn modify_order(&self, order_id: &str, params: OrderModifyParams) -> KiteResult<OrderResponse> {
        params.validate()?;
        let response = self.modify_order_raw(
            order_id,
            params.variety.as_str(),
            params.quantity,
            params.price,
            params.trigger_price,
            params.validity.as_str(),
            params.disclosed_quantity,
        ).await?;
        self.parse_response(response)
    }
    
    // Bulk order operations
    pub async fn place_orders_bulk(&self, orders: Vec<OrderParams>) -> KiteResult<Vec<OrderResponse>> {
        let mut results = Vec::new();
        for order in orders {
            let result = self.place_order(order).await;
            results.push(result);
        }
        Ok(results.into_iter().collect::<KiteResult<Vec<_>>>()?)
    }
}
```

#### 8.3.2 Order Management Utilities
```rust
impl KiteConnect {
    /// Get orders filtered by status
    pub async fn orders_by_status(&self, status: OrderStatus) -> KiteResult<Vec<Order>> {
        let orders = self.orders().await?;
        Ok(orders.into_iter().filter(|o| o.status == status).collect())
    }
    
    /// Get today's trades with P&L calculation
    pub async fn trades_with_pnl(&self) -> KiteResult<TradeHistory> {
        let trades = self.trades().await?;
        // Add P&L calculation logic
        Ok(trades)
    }
    
    /// Cancel all pending orders for a symbol
    pub async fn cancel_all_orders(&self, symbol: &str) -> KiteResult<Vec<OrderResponse>> {
        let orders = self.orders_by_status(OrderStatus::OPEN).await?;
        let symbol_orders: Vec<_> = orders.into_iter()
            .filter(|o| o.trading_symbol == symbol)
            .collect();
            
        let mut results = Vec::new();
        for order in symbol_orders {
            let result = self.cancel_order(&order.order_id, order.variety.as_str()).await;
            results.push(result);
        }
        Ok(results.into_iter().collect::<KiteResult<Vec<_>>>()?)
    }
}
```

### 8.4 Portfolio Module (`src/connect/portfolio.rs`)

#### 8.4.1 Enhanced Portfolio Methods
```rust
impl KiteConnect {
    // Typed holdings with analytics
    pub async fn holdings(&self) -> KiteResult<Vec<Holding>> {
        let response = self.holdings_raw().await?;
        self.parse_response(response)
    }
    
    // Holdings summary with aggregated data
    pub async fn holdings_summary(&self) -> KiteResult<HoldingsSummary> {
        let holdings = self.holdings().await?;
        Ok(HoldingsSummary::from_holdings(&holdings))
    }
    
    // Typed positions
    pub async fn positions(&self) -> KiteResult<PositionsSummary> {
        let response = self.positions_raw().await?;
        self.parse_response(response)
    }
    
    // Position conversion with validation
    pub async fn convert_position(&self, request: PositionConversionRequest) -> KiteResult<ConversionResponse> {
        request.validate()?;
        let response = self.convert_position_raw(
            request.exchange.as_str(),
            &request.trading_symbol,
            request.transaction_type.as_str(),
            request.position_type.as_str(),
            request.quantity.to_string().as_str(),
            request.old_product.as_str(),
            request.new_product.as_str(),
        ).await?;
        self.parse_response(response)
    }
}
```

#### 8.4.2 Portfolio Analytics
```rust
impl KiteConnect {
    /// Get portfolio performance metrics
    pub async fn portfolio_performance(&self) -> KiteResult<PortfolioProfile> {
        let holdings = self.holdings().await?;
        let positions = self.positions().await?;
        
        // Calculate comprehensive portfolio metrics
        Ok(PortfolioProfile::calculate(&holdings, &positions))
    }
    
    /// Get holdings for specific exchange
    pub async fn holdings_by_exchange(&self, exchange: Exchange) -> KiteResult<Vec<Holding>> {
        let holdings = self.holdings().await?;
        Ok(holdings.into_iter()
            .filter(|h| h.exchange == exchange)
            .collect())
    }
    
    /// Get positions with unrealized P&L
    pub async fn positions_with_pnl(&self) -> KiteResult<Vec<Position>> {
        let positions = self.positions().await?;
        Ok(positions.day.into_iter()
            .chain(positions.net.into_iter())
            .collect())
    }
}
```

### 8.5 Market Data Module (`src/connect/market_data.rs`)

#### 8.5.1 Enhanced Market Data Methods
```rust
impl KiteConnect {
    // Typed instruments with caching
    pub async fn instruments(&self, exchange: Option<Exchange>) -> KiteResult<Vec<Instrument>> {
        if let Some(cached) = self.get_cached_instruments(exchange).await? {
            return Ok(cached);
        }
        
        let response = self.instruments_raw(exchange.map(|e| e.as_str())).await?;
        let instruments: Vec<Instrument> = self.parse_csv_response(response)?;
        
        self.cache_instruments(exchange, &instruments).await?;
        Ok(instruments)
    }
    
    // Typed quotes with batch support
    pub async fn quotes(&self, instruments: &[String]) -> KiteResult<Quote> {
        let response = self.quote_raw(instruments).await?;
        self.parse_response(response)
    }
    
    // Historical data with typed intervals
    pub async fn historical_data(&self, request: HistoricalDataRequest) -> KiteResult<HistoricalData> {
        request.validate()?;
        let response = self.historical_data_raw(
            request.instrument_token,
            &request.from.format("%Y-%m-%d %H:%M:%S").to_string(),
            &request.to.format("%Y-%m-%d %H:%M:%S").to_string(),
            request.interval.as_str(),
            request.continuous.unwrap_or(false),
            request.oi.unwrap_or(false),
        ).await?;
        self.parse_response(response)
    }
}
```

#### 8.5.2 Market Data Utilities
```rust
impl KiteConnect {
    /// Search instruments by symbol/name
    pub async fn search_instruments(&self, query: &str, exchange: Option<Exchange>) -> KiteResult<Vec<Instrument>> {
        let instruments = self.instruments(exchange).await?;
        Ok(instruments.into_iter()
            .filter(|i| i.trading_symbol.contains(query) || i.name.contains(query))
            .collect())
    }
    
    /// Get market status for all exchanges
    pub async fn market_status_all(&self) -> KiteResult<Vec<MarketStatus>> {
        let response = self.market_status_raw().await?;
        self.parse_response(response)
    }
    
    /// Get live market depth
    pub async fn market_depth(&self, instruments: &[String]) -> KiteResult<MarketDepthFull> {
        let response = self.quote_raw(instruments).await?;
        self.parse_response(response)
    }
}
```

### 8.6 Mutual Funds Module (`src/connect/mutual_funds.rs`)

#### 8.6.1 Enhanced MF Methods
```rust
impl KiteConnect {
    // Typed MF orders
    pub async fn mf_orders(&self, order_id: Option<&str>) -> KiteResult<MFOrders> {
        let response = self.mf_orders_raw(order_id).await?;
        self.parse_response(response)
    }
    
    // Place MF order with params
    pub async fn place_mf_order(&self, params: MFOrderParams) -> KiteResult<MFOrderResponse> {
        params.validate()?;
        let response = self.place_mf_order_raw(
            &params.trading_symbol,
            params.transaction_type.as_str(),
            params.quantity.map(|q| q.to_string()).as_deref(),
            params.amount.map(|a| a.to_string()).as_deref(),
            params.tag.as_deref(),
        ).await?;
        self.parse_response(response)
    }
    
    // Typed SIP management
    pub async fn mf_sips(&self, sip_id: Option<&str>) -> KiteResult<SIPs> {
        let response = self.mf_sips_raw(sip_id).await?;
        self.parse_response(response)
    }
    
    // Place SIP with params
    pub async fn place_mf_sip(&self, params: SIPParams) -> KiteResult<SIPResponse> {
        params.validate()?;
        let response = self.place_mf_sip_raw(
            &params.trading_symbol,
            &params.amount.to_string(),
            &params.instalments.to_string(),
            params.frequency.as_str(),
            params.initial_amount.map(|a| a.to_string()).as_deref(),
            params.instalment_day.map(|d| d.to_string()).as_deref(),
            params.tag.as_deref(),
        ).await?;
        self.parse_response(response)
    }
}
```

### 8.7 GTT Module (New)

#### 8.7.1 Add GTT Support to Connect
Create `src/connect/gtt.rs`:

```rust
use anyhow::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::connect::utils::RequestHandler;
use crate::connect::KiteConnect;
use crate::models::prelude::*;

impl KiteConnect {
    /// Get all GTTs or specific GTT
    pub async fn gtts(&self, gtt_id: Option<u32>) -> KiteResult<GTTs> {
        let response = self.gtts_raw(gtt_id).await?;
        self.parse_response(response)
    }
    
    /// Place GTT
    pub async fn place_gtt(&self, params: GTTCreateParams) -> KiteResult<GTTResponse> {
        params.validate()?;
        let response = self.place_gtt_raw(params).await?;
        self.parse_response(response)
    }
    
    /// Modify GTT
    pub async fn modify_gtt(&self, gtt_id: u32, params: GTTModifyParams) -> KiteResult<GTTResponse> {
        let response = self.modify_gtt_raw(gtt_id, params).await?;
        self.parse_response(response)
    }
    
    /// Cancel GTT
    pub async fn cancel_gtt(&self, gtt_id: u32) -> KiteResult<GTTResponse> {
        let response = self.cancel_gtt_raw(gtt_id).await?;
        self.parse_response(response)
    }
    
    // Raw methods for backward compatibility
    pub async fn gtts_raw(&self, gtt_id: Option<u32>) -> Result<JsonValue> {
        let url = if let Some(id) = gtt_id {
            self.build_url(&format!("/gtt/triggers/{}", id), None)
        } else {
            self.build_url("/gtt/triggers", None)
        };
        
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }
    
    pub async fn place_gtt_raw(&self, params: GTTCreateParams) -> Result<JsonValue> {
        let mut data = HashMap::new();
        data.insert("type", params.gtt_type.as_str());
        data.insert("condition", serde_json::to_string(&params.condition)?);
        data.insert("orders", serde_json::to_string(&params.orders)?);
        
        if let Some(expires_at) = params.expires_at {
            data.insert("expires_at", &expires_at.format("%Y-%m-%d %H:%M:%S").to_string());
        }
        
        let url = self.build_url("/gtt/triggers", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;
        self.raise_or_return_json(resp).await
    }
}
```

### 8.8 Enhanced Error Handling and Retry Logic

#### 8.8.1 Implement Retry Mechanism
```rust
impl KiteConnect {
    async fn send_request_with_retry<T>(&self, request: Request) -> KiteResult<T>
    where
        T: DeserializeOwned,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            match self.send_request_internal(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    
                    if attempt < self.retry_config.max_retries && self.should_retry(&e) {
                        let delay = self.calculate_retry_delay(attempt);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
    
    fn should_retry(&self, error: &KiteError) -> bool {
        match error {
            KiteError::Http(_) => true,
            KiteError::Api { status, .. } => {
                // Retry on server errors, rate limits
                status.parse::<u16>().map(|s| s >= 500 || s == 429).unwrap_or(false)
            }
            _ => false,
        }
    }
    
    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        if self.retry_config.exponential_backoff {
            let delay = self.retry_config.base_delay * 2_u32.pow(attempt);
            std::cmp::min(delay, self.retry_config.max_delay)
        } else {
            self.retry_config.base_delay
        }
    }
}
```

#### 8.8.2 Enhanced Rate Limiting
```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    requests_per_second: u32,
    last_reset: std::sync::Mutex<std::time::Instant>,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(requests_per_second as usize)),
            requests_per_second,
            last_reset: std::sync::Mutex::new(std::time::Instant::now()),
        }
    }
    
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        // Reset permits every second
        {
            let mut last_reset = self.last_reset.lock().unwrap();
            if last_reset.elapsed() >= Duration::from_secs(1) {
                // Add back permits for new second
                self.semaphore.add_permits(self.requests_per_second as usize);
                *last_reset = std::time::Instant::now();
            }
        }
        
        self.semaphore.acquire().await.unwrap()
    }
}
```

### 8.9 Performance Optimizations

#### 8.9.1 Response Caching
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ResponseCache {
    cache: Arc<RwLock<HashMap<String, CachedResponse>>>,
    config: CacheConfig,
}

#[derive(Clone)]
struct CachedResponse {
    data: JsonValue,
    timestamp: std::time::Instant,
}

impl ResponseCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<JsonValue> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(key) {
            if cached.timestamp.elapsed() < Duration::from_secs(self.config.cache_ttl_minutes * 60) {
                return Some(cached.data.clone());
            }
        }
        None
    }
    
    pub async fn set(&self, key: String, value: JsonValue) {
        let mut cache = self.cache.write().await;
        
        // Clean up expired entries
        cache.retain(|_, v| v.timestamp.elapsed() < Duration::from_secs(self.config.cache_ttl_minutes * 60));
        
        // Limit cache size
        if cache.len() >= self.config.max_cache_size {
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(key, CachedResponse {
            data: value,
            timestamp: std::time::Instant::now(),
        });
    }
}
```

#### 8.9.2 Connection Pooling
```rust
impl KiteConnect {
    pub fn new_with_config(api_key: &str, config: KiteConnectConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .pool_max_idle_per_host(config.max_idle_connections)
            .pool_idle_timeout(Duration::from_secs(config.idle_timeout))
            .user_agent(&format!("kiteconnect-rust/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            api_key: api_key.to_string(),
            access_token: None,
            root: config.base_url,
            timeout: config.timeout,
            session_expiry_hook: None,
            client,
            retry_config: config.retry_config,
            cache_config: config.cache_config,
            request_counter: Arc::new(AtomicU64::new(0)),
            rate_limiter: config.rate_limiter,
        }
    }
}
```

## Testing Strategy

### 8.1 Integration Tests
Create `tests/integration/` directory with:

```rust
// tests/integration/connect_auth.rs
use kiteconnect_async_wasm::prelude::*;

#[tokio::test]
async fn test_typed_session_generation() {
    let client = KiteConnect::new("test_key", "test_token");
    
    // Mock the response
    let session = client.generate_session_typed("request_token", "api_secret").await;
    assert!(session.is_ok());
}

#[tokio::test]
async fn test_dual_api_compatibility() {
    let client = KiteConnect::new("test_key", "test_token");
    
    // Test both APIs return equivalent data
    let typed_result = client.holdings().await;
    let raw_result = client.holdings_raw().await;
    
    // Compare serialized results
    assert!(typed_result.is_ok());
    assert!(raw_result.is_ok());
}
```

### 8.2 Performance Tests
```rust
// tests/performance/rate_limiting.rs
use std::time::Instant;
use kiteconnect_async_wasm::prelude::*;

#[tokio::test]
async fn test_rate_limiting() {
    let client = KiteConnect::new_with_config("test_key", KiteConnectConfig {
        rate_limiter: Some(RateLimiter::new(10)), // 10 requests per second
        ..Default::default()
    });
    
    let start = Instant::now();
    
    // Make 20 requests
    for _ in 0..20 {
        let _ = client.profile().await;
    }
    
    let duration = start.elapsed();
    assert!(duration >= Duration::from_secs(1)); // Should take at least 1 second due to rate limiting
}
```

### 8.3 Error Handling Tests
```rust
// tests/integration/error_handling.rs
#[tokio::test]
async fn test_retry_mechanism() {
    let client = KiteConnect::new_with_config("test_key", KiteConnectConfig {
        retry_config: RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(1),
            exponential_backoff: true,
        },
        ..Default::default()
    });
    
    // Test with simulated network failure
    // Should retry 3 times before failing
}
```

## Migration Guidelines

### 8.1 Backward Compatibility
- All existing `*_raw()` methods remain unchanged
- New typed methods are additive, not breaking
- Gradual migration path with deprecation warnings

### 8.2 Configuration Migration
```rust
// Old configuration
let client = KiteConnect::new("api_key", "access_token");

// New configuration (optional)
let config = KiteConnectConfig {
    base_url: "https://api.kite.trade".to_string(),
    timeout: 30,
    retry_config: RetryConfig::default(),
    cache_config: Some(CacheConfig::default()),
    rate_limiter: Some(RateLimiter::new(10)),
    max_idle_connections: 10,
    idle_timeout: 30,
};
let client = KiteConnect::new_with_config("api_key", config);
client.set_access_token("access_token");
```

### 8.3 Error Handling Migration
```rust
// Old error handling
match client.holdings().await {
    Ok(data) => { /* handle JsonValue */ },
    Err(e) => { /* handle anyhow::Error */ },
}

// New error handling
match client.holdings().await {
    Ok(holdings) => { /* handle Vec<Holding> */ },
    Err(KiteError::Authentication(msg)) => { /* specific auth error handling */ },
    Err(KiteError::Api { status, message, .. }) => { /* API error with details */ },
    Err(e) => { /* other errors */ },
}
```

## Success Criteria

1. **✅ All existing tests pass** - No breaking changes to existing API
2. **✅ New typed methods work correctly** - Proper serialization/deserialization
3. **✅ Performance improvements** - Caching, connection pooling, rate limiting
4. **✅ Enhanced error handling** - Specific error types with proper context
5. **✅ Comprehensive testing** - Unit, integration, and performance tests
6. **✅ Documentation** - Updated examples and migration guides

## Timeline Estimate

- **Week 1**: Authentication and basic connect module updates
- **Week 2**: Orders and portfolio module integration
- **Week 3**: Market data and mutual funds integration  
- **Week 4**: GTT module, error handling, and retry logic
- **Week 5**: Performance optimizations and caching
- **Week 6**: Testing, documentation, and final integration

## Next Phase Preview

**Phase 9: WebSocket Integration**
- Real-time data streaming with typed events
- Connection management and auto-reconnection
- Order and trade real-time updates
- Market data live feeds
