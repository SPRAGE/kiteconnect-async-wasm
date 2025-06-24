# KiteConnect Async WASM v1.0.0 Migration Guide

## Overview

KiteConnect v1.0.0 introduces significant improvements while maintaining backward compatibility. This guide helps you migrate from earlier versions and leverage the new features.

## 🚀 Key Features in v1.0.0

### 1. Dual API Support
- **Legacy API**: All existing methods still work as before (returns `JsonValue`)
- **Typed API**: New strongly-typed methods with `_typed` suffix (returns structured types)

### 2. Enhanced Error Handling
- Comprehensive error types with proper context
- Automatic retry mechanism with exponential backoff
- Better error messages and debugging information

### 3. Performance Optimizations
- Response caching for instruments data
- Connection pooling and reuse
- Optimized JSON parsing and serialization

### 4. WASM Compatibility
- Full WebAssembly support with platform-specific optimizations
- CSV parsing optimized for both native and WASM environments

## 📈 Migration Examples

### Basic Client Usage (No Changes Required)
```rust
// This code continues to work exactly as before
let client = KiteConnect::new("api_key", "access_token");
let holdings = client.holdings().await?;
println!("Holdings: {:?}", holdings);
```

### Upgrading to Typed APIs
```rust
// Before (v0.x) - returns JsonValue
let holdings = client.holdings().await?;
let first_holding = &holdings["data"][0];
let isin = first_holding["isin"].as_str().unwrap_or("");

// After (v1.0.0) - returns strongly typed data
let holdings = client.holdings_typed().await?;
let first_holding = &holdings[0];
let isin = &first_holding.isin; // Direct access, no unwrapping needed
```

### Error Handling Improvements
```rust
// Before (v0.x)
match client.holdings().await {
    Ok(data) => println!("Holdings: {:?}", data),
    Err(e) => eprintln!("Error: {}", e), // Generic error message
}

// After (v1.0.0) - Enhanced error information
use kiteconnect_async_wasm::models::common::KiteError;

match client.holdings_typed().await {
    Ok(holdings) => println!("Holdings: {} items", holdings.len()),
    Err(KiteError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(KiteError::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

### Configuration with Retry and Caching
```rust
use kiteconnect_async_wasm::connect::{KiteConnect, KiteConnectConfig, RetryConfig, CacheConfig};

// Create custom configuration
let config = KiteConnectConfig {
    base_url: "https://api.kite.trade".to_string(),
    timeout: 60, // 60 seconds timeout
    retry_config: RetryConfig {
        max_retries: 5,
        base_delay: Duration::from_millis(200),
        max_delay: Duration::from_secs(10),
        exponential_backoff: true,
    },
    cache_config: Some(CacheConfig {
        enable_instruments_cache: true,
        cache_ttl_minutes: 60, // Cache for 1 hour
        max_cache_size: 1000,
    }),
    max_idle_connections: 20,
    idle_timeout: 60,
};

let client = KiteConnect::with_config("api_key", "access_token", config);
```

## 🔄 API Method Mapping

### Portfolio APIs
| Legacy Method | Typed Method | Return Type |
|---------------|--------------|-------------|
| `holdings()` | `holdings_typed()` | `Vec<Holding>` |
| `positions()` | `positions_typed()` | `Vec<Position>` |
| `auctions()` | `auctions_typed()` | `Vec<Auction>` |

### Order APIs
| Legacy Method | Typed Method | Return Type |
|---------------|--------------|-------------|
| `orders()` | `orders_typed()` | `Vec<Order>` |
| `place_order()` | `place_order_typed()` | `OrderResponse` |
| `trades()` | `trades_typed()` | `Vec<Trade>` |

### Market Data APIs
| Legacy Method | Typed Method | Return Type |
|---------------|--------------|-------------|
| `quote()` | `quote_typed()` | `Vec<Quote>` |
| `ohlc()` | `ohlc_typed()` | `Vec<OHLC>` |
| `ltp()` | `ltp_typed()` | `Vec<LTP>` |
| `historical_data()` | `historical_data_typed()` | `HistoricalData` |

### Authentication APIs
| Legacy Method | Typed Method | Return Type |
|---------------|--------------|-------------|
| `generate_session()` | `generate_session_typed()` | `SessionData` |
| `profile()` | `profile_typed()` | `UserProfile` |

### Mutual Funds APIs
| Legacy Method | Typed Method | Return Type |
|---------------|--------------|-------------|
| `mf_orders()` | `mf_orders_typed()` | `Vec<MFOrder>` |
| `place_mf_order()` | `place_mf_order_typed()` | `MFOrderResponse` |
| `mf_sips()` | `mf_sips_typed()` | `Vec<SIP>` |
| `place_mf_sip()` | `place_mf_sip_typed()` | `SIPResponse` |
| `mf_holdings()` | `mf_holdings_typed()` | `Vec<MFHolding>` |

## 🛠️ New Features Guide

### Automatic Retry Mechanism
All typed methods automatically retry failed requests:
```rust
// Requests will be retried up to 3 times with exponential backoff
let holdings = client.holdings_typed().await?; // May retry internally
```

### Response Caching
Instruments data is cached automatically:
```rust
// First call hits the API
let instruments1 = client.instruments(None).await?;

// Second call uses cache (if within TTL)
let instruments2 = client.instruments(None).await?; // Fast cached response
```

### Enhanced Type Safety
```rust
use kiteconnect_async_wasm::models::prelude::*;

// Create orders with compile-time validation
let order_params = OrderParams {
    exchange: Exchange::NSE,
    trading_symbol: "RELIANCE".to_string(),
    transaction_type: TransactionType::BUY,
    order_type: OrderType::LIMIT,
    quantity: 10,
    price: Some(2500.0),
    product: Product::CNC,
    validity: Some(Validity::DAY),
    // ... other fields
};

let response = client.place_order_typed(&order_params).await?;
println!("Order placed with ID: {}", response.order_id);
```

### Concurrent Operations
```rust
use tokio::try_join;

// Fetch multiple data sources concurrently
let (holdings, positions, orders) = try_join!(
    client.holdings_typed(),
    client.positions_typed(),
    client.orders_typed()
)?;

println!("Portfolio summary:");
println!("  Holdings: {} items", holdings.len());
println!("  Positions: {} items", positions.len());
println!("  Orders: {} items", orders.len());
```

## 🔧 Platform-Specific Features

### Native (Tokio) Platform
```toml
[dependencies]
kiteconnect-async-wasm = { version = "1.0.0", features = ["native"] }
```

### WebAssembly Platform
```toml
[dependencies]
kiteconnect-async-wasm = { version = "1.0.0", features = ["wasm"] }
```

### Usage in WASM
```rust
use wasm_bindgen::prelude::*;
use kiteconnect_async_wasm::connect::KiteConnect;

#[wasm_bindgen]
pub async fn fetch_portfolio() -> Result<String, JsValue> {
    let client = KiteConnect::new("api_key", "access_token");
    let holdings = client.holdings_typed().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(format!("You have {} holdings", holdings.len()))
}
```

## 🚨 Breaking Changes (Minimal)

### Error Types
If you were pattern matching on specific error types, update them:
```rust
// Before (if using custom error handling)
match error {
    // Old error patterns
}

// After
use kiteconnect_async_wasm::models::common::KiteError;
match error {
    KiteError::Authentication(msg) => { /* handle auth error */ }
    KiteError::Api { status, message, .. } => { /* handle API error */ }
    KiteError::Http(reqwest_error) => { /* handle network error */ }
    _ => { /* handle other errors */ }
}
```

### Configuration Structure
If you were using custom configurations (advanced usage):
```rust
// Before (if custom config was used)
// Custom configuration was not available

// After
let config = KiteConnectConfig {
    timeout: 60,
    retry_config: RetryConfig::default(),
    cache_config: Some(CacheConfig::default()),
    // ... other fields
};
let client = KiteConnect::with_config("api_key", "access_token", config);
```

## 📊 Performance Improvements

1. **Request Caching**: Instruments data cached for 1 hour by default
2. **Connection Pooling**: HTTP connections reused across requests  
3. **Retry Logic**: Smart retry with exponential backoff
4. **Memory Usage**: Optimized struct layouts and reduced allocations
5. **WASM Optimizations**: Specialized CSV parsing for WebAssembly

## ✅ Testing Your Migration

```rust
#[tokio::test]
async fn test_migration() {
    let client = KiteConnect::new("test_key", "test_token");
    
    // Test legacy API still works
    assert!(client.holdings().await.is_err()); // Expected without real auth
    
    // Test typed API is available
    assert!(client.holdings_typed().await.is_err()); // Expected without real auth
    
    // Test configuration
    let config = KiteConnectConfig::default();
    let client_with_config = KiteConnect::with_config("test_key", "test_token", config);
    assert!(format!("{:?}", client_with_config).contains("KiteConnect"));
}
```

## 📚 Additional Resources

- [API Documentation](https://docs.rs/kiteconnect-async-wasm)
- [Examples](./examples/)
- [GitHub Repository](https://github.com/username/kiteconnect-async-wasm)
- [Kite Connect API Docs](https://kite.trade/docs/connect/v3/)

## 💡 Best Practices

1. **Use Typed APIs**: Prefer `*_typed()` methods for new code
2. **Handle Errors Properly**: Use specific error types for better UX
3. **Enable Caching**: Keep instruments cache enabled for better performance
4. **Configure Retries**: Adjust retry settings based on your use case
5. **Concurrent Requests**: Use `tokio::join!` for parallel API calls
6. **Monitor Performance**: Use request counters and timing for optimization

## 🤝 Support

For questions and support:
- GitHub Issues: Report bugs and feature requests
- Documentation: Check the comprehensive API docs
- Examples: Reference the example implementations
