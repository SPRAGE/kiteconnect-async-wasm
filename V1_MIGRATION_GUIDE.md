# KiteConnect v1.0.0 Migration Guide for LLM Implementation

## Overview
This document provides a comprehensive guide for migrating the kiteconnect-async-wasm library from v0.x (using `JsonValue` returns) to v1.0.0 with fully typed data models and improved error handling.

## Current State Analysis
- **Version**: Currently on v1.0.0-dev branch
- **Current API**: All methods return `Result<JsonValue, anyhow::Error>`
- **Structure**: Modular connect module with auth, orders, portfolio, market_data, mutual_funds, utils
- **Models**: Empty `src/models/mod.rs` with TODO comments
- **Platform Support**: Native (tokio) and WASM compatibility maintained

## Target Architecture

### 1. Modular Models Structure
```
src/models/
├── mod.rs                 // Main public API and re-exports
├── common/                // Shared types and enums
│   ├── mod.rs
│   ├── enums.rs          // Exchange, Product, Validity, etc.
│   ├── response.rs       // KiteResponse<T>, Status, etc.
│   └── errors.rs         // Custom error types
├── auth/                  // Authentication & user data
│   ├── mod.rs
│   ├── session.rs        // SessionData, AccessToken, etc.
│   ├── user.rs           // UserProfile, UserType, etc.
│   └── margins.rs        // MarginData, SegmentMargins, etc.
├── orders/                // Order management
│   ├── mod.rs
│   ├── order.rs          // Order struct, OrderBuilder
│   ├── trade.rs          // Trade struct and related types
│   ├── enums.rs          // OrderStatus, OrderType, Variety, etc.
│   └── requests.rs       // PlaceOrderRequest, ModifyOrderRequest, etc.
├── portfolio/             // Holdings and positions
│   ├── mod.rs
│   ├── holding.rs        // Holding struct and related types
│   ├── position.rs       // Position, PositionType, etc.
│   └── conversion.rs     // Position conversion types
├── market_data/           // Market quotes and instruments
│   ├── mod.rs
│   ├── instrument.rs     // Instrument struct and CSV parsing
│   ├── quote.rs          // Quote, FullQuote, OHLC, LTP structs
│   ├── depth.rs          // Market depth data structures
│   └── historical.rs     // Historical data candles, etc.
├── mutual_funds/          // MF operations
│   ├── mod.rs
│   ├── order.rs          // MF order types
│   ├── instrument.rs     // MF instrument data
│   ├── sip.rs            // SIP (Systematic Investment Plan) types
│   └── holding.rs        // MF holdings
└── gtt/                   // GTT (Good Till Triggered) orders
    ├── mod.rs
    ├── gtt_order.rs      // GTT order types
    └── trigger.rs        // Trigger conditions
```

## Implementation Strategy: **Simultaneous Development**

**Recommendation**: Implement models and connect module changes simultaneously using a **dual-API approach**.

### Why Simultaneous Development?

1. **Faster Iteration**: See immediate results as you build
2. **Better Testing**: Test typed APIs as you develop them
3. **Reduced Merge Conflicts**: Avoid large refactoring sessions
4. **Incremental Progress**: Working features at each step

### Dual-API Approach
Maintain backward compatibility during transition:
```rust
// New typed API
pub async fn holdings(&self) -> Result<Vec<Holding>, KiteError>

// Legacy API (for compatibility)
pub async fn holdings_raw(&self) -> Result<JsonValue, anyhow::Error>
```

## Phase-by-Phase Implementation

### Phase 1: Foundation (Start Here)
**Priority**: Critical foundation for everything else

1. **Add Dependencies to `Cargo.toml`**:
```toml
[dependencies]
thiserror = "2.0"          # For custom error types
chrono = { version = "0.4", features = ["serde"] }  # For datetime handling
```

2. **Create `models/common/` Module**:
   - `models/common/errors.rs` - Custom error types using `thiserror`
   - `models/common/response.rs` - `KiteResponse<T>` wrapper
   - `models/common/enums.rs` - Shared enums (Exchange, Product, etc.)
   - `models/mod.rs` - Public API

3. **Core Types to Implement**:
```rust
// In errors.rs
#[derive(Debug, thiserror::Error)]
pub enum KiteError {
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },
    
    #[error("API error: {message}")]
    ApiError { message: String, error_type: Option<String> },
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Order error: {message}")]
    OrderError { message: String },
    
    #[error("Invalid response format")]
    InvalidResponse,
    
    #[error("Feature not available: {feature}")]
    FeatureNotAvailable { feature: String },
}

// In response.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KiteResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

impl<T> KiteResponse<T> {
    pub fn into_result(self) -> Result<T, KiteError> {
        match self.status.as_str() {
            "success" => self.data.ok_or(KiteError::InvalidResponse),
            "error" => Err(KiteError::ApiError {
                message: self.message.unwrap_or_default(),
                error_type: self.error_type,
            }),
            _ => Err(KiteError::InvalidResponse),
        }
    }
}

// In enums.rs
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Exchange {
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "NFO")]
    NFO,
    #[serde(rename = "BFO")]
    BFO,
    #[serde(rename = "CDS")]
    CDS,
    #[serde(rename = "MCX")]
    MCX,
    #[serde(rename = "BCD")]
    BCD,
    #[serde(rename = "MF")]
    MF,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Product {
    #[serde(rename = "CNC")]
    CNC,
    #[serde(rename = "NRML")]
    NRML,
    #[serde(rename = "MIS")]
    MIS,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "MTF")]
    MTF,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TransactionType {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "SL")]
    StopLoss,
    #[serde(rename = "SL-M")]
    StopLossMarket,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Validity {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "IOC")]
    IOC,
    #[serde(rename = "TTL")]
    TTL,
}
```

### Phase 2: Authentication Models + Connect Changes
**Parallel Development**: Models and connect module together

1. **Create `models/auth/` Module**:
   - Based on `/session/token` and `/user/profile` API responses
   - `SessionData`, `UserProfile`, `MarginData` structs

2. **Update `src/connect/auth.rs`**:
   - Add new typed methods alongside existing ones
   - Convert JSON responses to typed structs

3. **Example Implementation Pattern**:
```rust
// In models/auth/session.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::common::{Exchange, Product, OrderType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub user_id: String,
    pub user_name: String,
    pub user_shortname: String,
    pub email: String,
    pub user_type: String,
    pub broker: String,
    pub exchanges: Vec<Exchange>,
    pub products: Vec<Product>,
    pub order_types: Vec<OrderType>,
    pub api_key: String,
    pub access_token: String,
    pub public_token: String,
    pub refresh_token: Option<String>,
    pub login_time: String,
    pub avatar_url: Option<String>,
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub user_name: String,
    pub user_shortname: String,
    pub email: String,
    pub user_type: String,
    pub broker: String,
    pub exchanges: Vec<Exchange>,
    pub products: Vec<Product>,
    pub order_types: Vec<OrderType>,
    pub avatar_url: Option<String>,
    pub meta: serde_json::Value,
}

// In connect/auth.rs
impl KiteConnect {
    // New typed API
    pub async fn generate_session_typed(
        &mut self,
        request_token: &str,
        api_secret: &str,
    ) -> Result<SessionData, KiteError> {
        let json_response = self.generate_session(request_token, api_secret).await
            .map_err(|e| KiteError::ApiError { 
                message: e.to_string(), 
                error_type: None 
            })?;
        
        let response: KiteResponse<SessionData> = serde_json::from_value(json_response)
            .map_err(KiteError::SerializationError)?;
            
        response.into_result()
    }
    
    pub async fn user_profile(&self) -> Result<UserProfile, KiteError> {
        let json_response = self.profile().await
            .map_err(|e| KiteError::ApiError { 
                message: e.to_string(), 
                error_type: None 
            })?;
        
        let response: KiteResponse<UserProfile> = serde_json::from_value(json_response)
            .map_err(KiteError::SerializationError)?;
            
        response.into_result()
    }
}
```

### Phase 3: Orders Models + Connect Changes
**High Impact**: Most frequently used APIs

1. **Create `models/orders/` Module**:

```rust
// In models/orders/order.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::common::{Exchange, Product, TransactionType, OrderType, Validity};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "PUT ORDER REQ RECEIVED")]
    PutOrderReqReceived,
    #[serde(rename = "VALIDATION PENDING")]
    ValidationPending,
    #[serde(rename = "OPEN PENDING")]
    OpenPending,
    #[serde(rename = "MODIFY PENDING")]
    ModifyPending,
    #[serde(rename = "TRIGGER PENDING")]
    TriggerPending,
    #[serde(rename = "CANCEL PENDING")]
    CancelPending,
    #[serde(other)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OrderVariety {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "amo")]
    AMO,
    #[serde(rename = "co")]
    CoverOrder,
    #[serde(rename = "iceberg")]
    Iceberg,
    #[serde(rename = "auction")]
    Auction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub order_id: String,
    pub parent_order_id: Option<String>,
    pub exchange_order_id: Option<String>,
    pub placed_by: String,
    pub variety: OrderVariety,
    pub status: OrderStatus,
    pub status_message: Option<String>,
    pub status_message_raw: Option<String>,
    pub order_timestamp: String,
    pub exchange_timestamp: Option<String>,
    pub exchange_update_timestamp: Option<String>,
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub instrument_token: u64,
    pub transaction_type: TransactionType,
    pub order_type: OrderType,
    pub product: Product,
    pub validity: Validity,
    pub validity_ttl: Option<u32>,
    pub price: f64,
    pub quantity: i64,
    pub trigger_price: f64,
    pub average_price: f64,
    pub pending_quantity: i64,
    pub filled_quantity: i64,
    pub cancelled_quantity: i64,
    pub disclosed_quantity: i64,
    pub market_protection: f64,
    pub meta: serde_json::Value,
    pub tag: Option<String>,
    pub tags: Option<Vec<String>>,
    pub guid: Option<String>,
    pub modified: bool,
    pub auction_number: Option<String>,
}

// Builder pattern for order placement
#[derive(Debug, Clone)]
pub struct OrderBuilder {
    tradingsymbol: String,
    exchange: Exchange,
    transaction_type: TransactionType,
    order_type: OrderType,
    quantity: i64,
    product: Product,
    validity: Validity,
    price: Option<f64>,
    trigger_price: Option<f64>,
    disclosed_quantity: Option<i64>,
    validity_ttl: Option<u32>,
    tag: Option<String>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self {
            tradingsymbol: String::new(),
            exchange: Exchange::NSE,
            transaction_type: TransactionType::Buy,
            order_type: OrderType::Market,
            quantity: 0,
            product: Product::CNC,
            validity: Validity::Day,
            price: None,
            trigger_price: None,
            disclosed_quantity: None,
            validity_ttl: None,
            tag: None,
        }
    }
    
    pub fn symbol(mut self, symbol: &str) -> Self {
        self.tradingsymbol = symbol.to_string();
        self
    }
    
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = exchange;
        self
    }
    
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }
    
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }
    
    pub fn quantity(mut self, quantity: i64) -> Self {
        self.quantity = quantity;
        self
    }
    
    pub fn product(mut self, product: Product) -> Self {
        self.product = product;
        self
    }
    
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
    
    pub fn validity(mut self, validity: Validity) -> Self {
        self.validity = validity;
        self
    }
    
    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }
    
    pub fn build(self) -> Result<PlaceOrderRequest, KiteError> {
        if self.tradingsymbol.is_empty() {
            return Err(KiteError::ApiError {
                message: "Trading symbol is required".to_string(),
                error_type: None,
            });
        }
        
        if self.quantity <= 0 {
            return Err(KiteError::ApiError {
                message: "Quantity must be greater than 0".to_string(),
                error_type: None,
            });
        }
        
        Ok(PlaceOrderRequest {
            tradingsymbol: self.tradingsymbol,
            exchange: self.exchange,
            transaction_type: self.transaction_type,
            order_type: self.order_type,
            quantity: self.quantity,
            product: self.product,
            validity: self.validity,
            price: self.price,
            trigger_price: self.trigger_price,
            disclosed_quantity: self.disclosed_quantity,
            validity_ttl: self.validity_ttl,
            tag: self.tag,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PlaceOrderRequest {
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub transaction_type: TransactionType,
    pub order_type: OrderType,
    pub quantity: i64,
    pub product: Product,
    pub validity: Validity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderResponse {
    pub order_id: String,
}
```

2. **Update `src/connect/orders.rs`**:
```rust
impl KiteConnect {
    pub async fn place_order_typed(&self, order_request: PlaceOrderRequest) -> Result<OrderResponse, KiteError> {
        // Convert typed request to the format expected by existing method
        // ... implementation details
    }
    
    pub async fn orders_typed(&self) -> Result<Vec<Order>, KiteError> {
        let json_response = self.orders().await
            .map_err(|e| KiteError::ApiError { 
                message: e.to_string(), 
                error_type: None 
            })?;
        
        let response: KiteResponse<Vec<Order>> = serde_json::from_value(json_response)
            .map_err(KiteError::SerializationError)?;
            
        response.into_result()
    }
}
```

### Phase 4: Portfolio Models + Connect Changes
**Core Functionality**: Holdings and positions

```rust
// In models/portfolio/holding.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Holding {
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub instrument_token: u64,
    pub isin: String,
    pub product: Product,
    pub price: f64,
    pub quantity: i64,
    pub used_quantity: i64,
    pub t1_quantity: i64,
    pub realised_quantity: i64,
    pub authorised_quantity: i64,
    pub authorised_date: String,
    pub opening_quantity: i64,
    pub collateral_quantity: i64,
    pub collateral_type: Option<String>,
    pub discrepancy: bool,
    pub average_price: f64,
    pub last_price: f64,
    pub close_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
}

// In models/portfolio/position.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub instrument_token: u64,
    pub product: Product,
    pub quantity: i64,
    pub overnight_quantity: i64,
    pub multiplier: i64,
    pub average_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub value: f64,
    pub pnl: f64,
    pub m2m: f64,
    pub unrealised: f64,
    pub realised: f64,
    // ... other fields
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionsResponse {
    pub net: Vec<Position>,
    pub day: Vec<Position>,
}
```

### Phase 5: Market Data Models + Connect Changes
**Complex**: CSV parsing and various quote types

```rust
// In models/market_data/instrument.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instrument {
    pub instrument_token: u64,
    pub exchange_token: u64,
    pub tradingsymbol: String,
    pub name: String,
    pub last_price: f64,
    pub expiry: Option<String>,
    pub strike: Option<f64>,
    pub tick_size: f64,
    pub lot_size: i64,
    pub instrument_type: String,
    pub segment: String,
    pub exchange: Exchange,
}

// In models/market_data/quote.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub instrument_token: u64,
    pub timestamp: String,
    pub last_trade_time: Option<String>,
    pub last_price: f64,
    pub last_quantity: i64,
    pub buy_quantity: i64,
    pub sell_quantity: i64,
    pub volume: i64,
    pub average_price: f64,
    pub oi: f64,
    pub oi_day_high: f64,
    pub oi_day_low: f64,
    pub net_change: f64,
    pub lower_circuit_limit: f64,
    pub upper_circuit_limit: f64,
    pub ohlc: OHLC,
    pub depth: MarketDepth,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LTP {
    pub instrument_token: u64,
    pub last_price: f64,
}
```

### Phase 6: Mutual Funds + Advanced Features
**Final Phase**: MF orders, GTT orders

## Implementation Guidelines

### 1. Struct Design Principles
- Use `#[derive(Debug, Serialize, Deserialize, Clone)]` for all structs
- Use `Option<T>` for fields that might be null in API responses
- Use appropriate Rust types (e.g., `f64` for prices, `i64` for quantities)
- Add `#[serde(rename = "field_name")]` for API field mapping
- Use `#[serde(other)]` for enum variants to handle unknown values

### 2. Error Handling
- Always convert `anyhow::Error` to `KiteError` with context
- Preserve original error messages from API
- Use `thiserror` for clean error definitions
- Implement `From` traits for automatic conversions

### 3. Datetime Handling
```rust
// Use chrono for datetime fields
use chrono::{DateTime, Utc, NaiveDateTime};

// Custom serde functions for datetime parsing
mod datetime_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
    }
}
```

### 4. Testing Strategy
- Create unit tests for each model's serialization/deserialization
- Use the existing mock JSON files in `mocks/` directory
- Test both typed and legacy APIs during transition

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_deserialization() {
        let json_data = include_str!("../../../mocks/orders.json");
        let response: KiteResponse<Vec<Order>> = serde_json::from_str(json_data).unwrap();
        let orders = response.into_result().unwrap();
        
        assert!(!orders.is_empty());
        // Additional assertions...
    }
}
```

### 5. Documentation
- Add comprehensive doc comments with examples
- Show migration path from old to new API
- Include builder pattern examples

```rust
/// Places a new order on the exchange
/// 
/// # Example
/// 
/// ```rust
/// use kiteconnect_async_wasm::models::{OrderBuilder, Exchange, TransactionType, OrderType, Product};
/// 
/// let order = OrderBuilder::new()
///     .symbol("RELIANCE")
///     .exchange(Exchange::NSE)
///     .transaction_type(TransactionType::Buy)
///     .order_type(OrderType::Market)
///     .quantity(10)
///     .product(Product::CNC)
///     .build()?;
/// 
/// let response = client.place_order_typed(order).await?;
/// println!("Order placed with ID: {}", response.order_id);
/// ```
pub async fn place_order_typed(&self, order_request: PlaceOrderRequest) -> Result<OrderResponse, KiteError>
```

## Breaking Changes for v1.0.0

### Before (v0.x):
```rust
let holdings: JsonValue = client.holdings().await?;
let symbol = holdings["data"][0]["tradingsymbol"].as_str().unwrap();
let quantity = holdings["data"][0]["quantity"].as_i64().unwrap();
```

### After (v1.0.0):
```rust
let holdings: Vec<Holding> = client.holdings_typed().await?;
let symbol = &holdings[0].tradingsymbol;
let quantity = holdings[0].quantity;
```

## Migration Utilities
Provide helper functions for users migrating:
```rust
impl From<JsonValue> for Order {
    fn from(json: JsonValue) -> Self {
        serde_json::from_value(json).expect("Failed to convert JsonValue to Order")
    }
}

// Utility function for gradual migration
impl KiteConnect {
    pub async fn holdings_as_json(&self) -> Result<JsonValue, anyhow::Error> {
        self.holdings().await  // Legacy method
    }
    
    pub async fn holdings_typed(&self) -> Result<Vec<Holding>, KiteError> {
        // New typed method
    }
}
```

## Required Dependencies
Add these to `Cargo.toml`:
```toml
[dependencies]
thiserror = "2.0"          # Custom error types
chrono = { version = "0.4", features = ["serde"] }  # Datetime handling
```

## Key API Reference Sources
- **Response Structure**: https://kite.trade/docs/connect/v3/response-structure/
- **Orders API**: https://kite.trade/docs/connect/v3/orders/
- **Portfolio API**: https://kite.trade/docs/connect/v3/portfolio/
- **User API**: https://kite.trade/docs/connect/v3/user/
- **Market Data API**: https://kite.trade/docs/connect/v3/market-quotes/

## File Creation Checklist

### Phase 1: Foundation
- [ ] Add dependencies to `Cargo.toml`
- [ ] `src/models/common/errors.rs`
- [ ] `src/models/common/response.rs`
- [ ] `src/models/common/enums.rs`
- [ ] `src/models/common/mod.rs`
- [ ] Update `src/models/mod.rs`

### Phase 2: Authentication
- [ ] `src/models/auth/session.rs`
- [ ] `src/models/auth/user.rs`
- [ ] `src/models/auth/margins.rs`
- [ ] `src/models/auth/mod.rs`
- [ ] Update `src/connect/auth.rs` with typed methods

### Phase 3: Orders
- [ ] `src/models/orders/order.rs`
- [ ] `src/models/orders/trade.rs`
- [ ] `src/models/orders/enums.rs`
- [ ] `src/models/orders/requests.rs`
- [ ] `src/models/orders/mod.rs`
- [ ] Update `src/connect/orders.rs` with typed methods

### Phase 4: Portfolio
- [ ] `src/models/portfolio/holding.rs`
- [ ] `src/models/portfolio/position.rs`
- [ ] `src/models/portfolio/conversion.rs`
- [ ] `src/models/portfolio/mod.rs`
- [ ] Update `src/connect/portfolio.rs` with typed methods

### Phase 5: Market Data
- [ ] `src/models/market_data/instrument.rs`
- [ ] `src/models/market_data/quote.rs`
- [ ] `src/models/market_data/depth.rs`
- [ ] `src/models/market_data/historical.rs`
- [ ] `src/models/market_data/mod.rs`
- [ ] Update `src/connect/market_data.rs` with typed methods

### Phase 6: Mutual Funds
- [ ] `src/models/mutual_funds/order.rs`
- [ ] `src/models/mutual_funds/instrument.rs`
- [ ] `src/models/mutual_funds/sip.rs`
- [ ] `src/models/mutual_funds/holding.rs`
- [ ] `src/models/mutual_funds/mod.rs`
- [ ] Update `src/connect/mutual_funds.rs` with typed methods

## Success Metrics
- [ ] All existing functionality available with typed APIs
- [ ] Legacy APIs still work (backward compatibility)
- [ ] Comprehensive error handling with `KiteError`
- [ ] Builder patterns for complex operations
- [ ] Full test coverage with mock data
- [ ] Documentation with migration examples
- [ ] Zero breaking changes to existing users initially
- [ ] Performance comparable to current implementation

## Final Notes
- Maintain the existing async/WASM compatibility
- Keep the modular structure of the connect module
- Use the existing feature flags (native/wasm) appropriately
- Preserve all existing functionality while adding type safety
- Ensure the library remains easy to use and well-documented

This migration will transform the library from a JSON-based API to a fully typed, ergonomic Rust library suitable for production use while maintaining backward compatibility during the transition period.
