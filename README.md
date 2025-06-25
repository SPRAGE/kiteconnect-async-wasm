# KiteConnect Async WASM v1.0.2

> ⚠️ **IMPORTANT DISCLAIMER** ⚠️
> 
> **🤖 AI-GENERATED CODE - USE AT YOUR OWN RISK**
> 
> This entire codebase has been generated using artificial intelligence and automated tools. While comprehensive testing has been performed, users should:
> - **Thoroughly test** all functionality in their specific use cases
> - **Review and validate** code before production use
> - **Use at their own risk** and responsibility
> - **Not rely on this** for critical financial operations without extensive validation
> 
> The maintainers provide no warranties or guarantees regarding the correctness, reliability, or suitability of this AI-generated code.

[![Crates.io](https://img.shields.io/crates/v/kiteconnect-async-wasm.svg)](https://crates.io/crates/kiteconnect-async-wasm)
[![Documentation](https://docs.rs/kiteconnect-async-wasm/badge.svg)](https://docs.rs/kiteconnect-async-wasm)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

**Modern async Rust client for KiteConnect REST APIs with dual API support, enhanced error handling, and WASM compatibility**

A production-ready, high-performance Rust library for KiteConnect API integration featuring both legacy and strongly-typed APIs.

## 🚀 Features v1.0.2

- ✅ **Dual API Support** - Legacy JSON + new strongly-typed APIs
- ✅ **Automatic Retry Logic** with exponential backoff
- ✅ **Response Caching** for performance optimization
- ✅ **Enhanced Error Handling** with detailed error types
- ✅ **Full WASM Compatibility** for web applications  
- ✅ **Thread-Safe Design** with connection pooling
- ✅ **Comprehensive Documentation** with migration guide
- ✅ **Backward Compatibility** - all existing code continues to work
- ✅ **Automated Releases** - CI/CD with automated publishing

## 🎯 Quick Start

### Installation

```toml
[dependencies]
kiteconnect-async-wasm = { version = "1.0.2", features = ["native"] }

# For WASM targets
# kiteconnect-async-wasm = { version = "1.0.2", features = ["wasm"] }
```

### Basic Usage (Legacy API - Backward Compatible)

```rust
use kiteconnect_async_wasm::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KiteConnect::new("your_api_key", "");

    // Step 1: Get login URL and complete authentication
    println!("Login URL: {}", client.login_url());
    
    // Step 2: After authentication, generate session
    let session = client.generate_session("request_token", "api_secret").await?;
    println!("Session: {:?}", session);
    
    // Step 3: Use APIs (existing code works as before)
    let holdings = client.holdings().await?;
    println!("Holdings: {:?}", holdings);
    
    Ok(())
}
```

### New Typed API (Recommended for v1.0.2)

```rust
use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KiteConnect::new("api_key", "access_token");
    
    // Strongly-typed responses with automatic retry and error handling
    let holdings: Vec<Holding> = client.holdings_typed().await?;
    let positions: Vec<Position> = client.positions_typed().await?;
    
    // Enhanced error handling
    match client.orders_typed().await {
        Ok(orders) => {
            println!("Found {} orders", orders.len());
            for order in orders {
                println!("Order {}: {} {} @ ₹{}", 
                    order.order_id, 
                    order.transaction_type, 
                    order.trading_symbol, 
                    order.price.unwrap_or(0.0)
                );
            }
        }
        Err(KiteError::Authentication(msg)) => {
            eprintln!("Authentication failed: {}", msg);
        }
        Err(KiteError::Api { status, message, .. }) => {
            eprintln!("API error {}: {}", status, message);
        }
        Err(e) => eprintln!("Other error: {}", e),
    }
    
    Ok(())
}
    let loginurl = kiteconnect.login_url();
    println!("{:?}", loginurl);

    // Generate access token with the above request token
    let resp = kiteconnect.generate_session("<REQUEST-TOKEN>", "<API-SECRET>").await?;
    // `generate_session` internally sets the access token from the response
    println!("{:?}", resp);

    let holdings: JsonValue = kiteconnect.holdings().await?;
    println!("{:?}", holdings);

    Ok(())
}
```

## Running Examples

### KiteConnect REST API sample

```bash
cargo run --example connect_sample
```

## ✅ **Completed Features**

- ✅ **Comprehensive serializer structs** for all KiteConnect data structures
  - Full typed models for all API responses (see `/src/models/`)
  - Dual API support: Legacy `JsonValue` + new strongly-typed APIs
  - Type-safe access to holdings, orders, positions, market data, etc.

- ✅ **Advanced reconnection mechanism** with intelligent retry logic
  - Automatic retry with exponential backoff
  - Configurable retry attempts and delays  
  - Built-in rate limiting respecting official API limits
  - Connection pooling and automatic error recovery

## Attribution

**Current Maintainer:** SPRAGE <shauna.pai@gmail.com>

This project was originally created by Joe Paul and other contributors. 
The current version has been significantly rewritten and modernized with:
- Complete async/await implementation
- WASM compatibility
- Enhanced feature flags system
- Comprehensive documentation
- Modern CI/CD pipeline

## License

This software is released into the public domain under The Unlicense. 
See the [LICENSE](LICENSE) file for details.

**No warranties provided** - This software is provided "as is" without warranty of any kind.

## 📊 Performance Features

### Automatic Caching
```rust
// Instruments data is automatically cached for 1 hour
let instruments1 = client.instruments(None).await?; // API call
let instruments2 = client.instruments(None).await?; // Cached response (fast!)
```

### Intelligent Retry Logic
```rust
// All typed methods automatically retry with exponential backoff
let holdings = client.holdings_typed().await?; // Retries on network errors
```

### Concurrent Operations
```rust
use tokio::try_join;

// Fetch multiple endpoints concurrently
let (holdings, positions, orders) = try_join!(
    client.holdings_typed(),
    client.positions_typed(), 
    client.orders_typed()
)?;
```

## 🔧 Advanced Configuration

```rust
use kiteconnect_async_wasm::connect::{KiteConnect, KiteConnectConfig, RetryConfig, CacheConfig};
use std::time::Duration;

let config = KiteConnectConfig {
    timeout: 60,
    retry_config: RetryConfig {
        max_retries: 5,
        base_delay: Duration::from_millis(200),
        max_delay: Duration::from_secs(10),
        exponential_backoff: true,
    },
    cache_config: Some(CacheConfig {
        enable_instruments_cache: true,
        cache_ttl_minutes: 60,
        max_cache_size: 1000,
    }),
    max_idle_connections: 20,
    idle_timeout: 60,
    ..Default::default()
};

let client = KiteConnect::with_config("api_key", "access_token", config);
```

## 🌐 WebAssembly Support

```rust
use wasm_bindgen::prelude::*;
use kiteconnect_async_wasm::connect::KiteConnect;

#[wasm_bindgen]
pub async fn get_portfolio_summary() -> Result<String, JsValue> {
    let client = KiteConnect::new("api_key", "access_token");
    
    let holdings = client.holdings_typed().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let total_value: f64 = holdings.iter()
        .map(|h| h.last_price * h.quantity as f64)
        .sum();
        
    Ok(format!("Portfolio value: ₹{:.2}", total_value))
}
```

## 📈 API Coverage

### ✅ Complete API Support

| Module | Legacy Methods | Typed Methods | Status |
|--------|---------------|---------------|--------|
| **Authentication** | ✅ | ✅ | Complete |
| **Portfolio** | ✅ | ✅ | Complete |
| **Orders** | ✅ | ✅ | Complete |
| **Market Data** | ✅ | ✅ | Complete |
| **Mutual Funds** | ✅ | ✅ | Complete |

### Key Methods Available

**Portfolio APIs:**
- `holdings()` / `holdings_typed()` - Get stock holdings
- `positions()` / `positions_typed()` - Get trading positions  
- `auctions()` / `auctions_typed()` - Get auction instruments

**Order Management:**
- `orders()` / `orders_typed()` - Get all orders
- `place_order()` / `place_order_typed()` - Place new orders
- `modify_order()` / `modify_order_typed()` - Modify existing orders
- `cancel_order()` - Cancel orders
- `trades()` / `trades_typed()` - Get trade history

**Market Data:**
- `instruments()` - Get instrument master (cached)
- `quote()` / `quote_typed()` - Get real-time quotes
- `ohlc()` / `ohlc_typed()` - Get OHLC data
- `ltp()` / `ltp_typed()` - Get last traded price
- `historical_data()` / `historical_data_typed()` - Get historical candles

**Mutual Funds:**
- `mf_orders()` / `mf_orders_typed()` - Get MF orders
- `place_mf_order()` / `place_mf_order_typed()` - Place MF orders
- `mf_sips()` / `mf_sips_typed()` - Get SIP details
- `place_mf_sip()` / `place_mf_sip_typed()` - Create SIPs
- `mf_holdings()` / `mf_holdings_typed()` - Get MF holdings

## 🔄 Migration from v0.x

All existing code continues to work without changes! For new projects, use the typed APIs:

```rust
// Old way (still works)
let holdings = client.holdings().await?;
let first_isin = holdings["data"][0]["isin"].as_str().unwrap();

// New way (recommended)
let holdings = client.holdings_typed().await?;
let first_isin = &holdings[0].isin; // Type-safe access
```

See [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md) for detailed migration instructions.

## 🛠️ Error Handling

v1.0.0 provides comprehensive error types:

```rust
use kiteconnect_async_wasm::models::common::KiteError;

match client.place_order_typed(&order_params).await {
    Ok(response) => println!("Order placed: {}", response.order_id),
    Err(KiteError::Authentication(msg)) => {
        // Handle authentication errors
        println!("Please re-authenticate: {}", msg);
    }
    Err(KiteError::Api { status, message, error_type }) => {
        // Handle API errors with context
        println!("API Error {}: {} ({})", status, message, error_type.unwrap_or_default());
    }
    Err(KiteError::Http(reqwest_err)) => {
        // Handle network errors (automatically retried)
        println!("Network error: {}", reqwest_err);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## 📚 Examples

Check out the [examples](./examples/) directory:

- **Basic Usage**: Simple API calls and authentication
- **Portfolio Tracker**: Real-time portfolio monitoring  
- **Order Management**: Placing and managing orders
- **Market Data**: Fetching quotes and historical data
- **WASM Integration**: Using in web applications

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with specific features
cargo test --features native
cargo test --features wasm

# Run integration tests (requires network)
cargo test --test integration_tests
```

## 📋 Requirements

- **Rust**: 1.70.0 or later
- **Tokio**: For async runtime (native)
- **Valid KiteConnect API credentials**

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Automated Release Process

This repository uses automated releases:

- **Version Management**: Use `./scripts/bump-version.sh [patch|minor|major|VERSION]` to create release branches
- **Automated Publishing**: When PRs are merged to `main`, GitHub Actions automatically:
  - Creates git tags
  - Publishes to crates.io
  - Generates GitHub releases
- **Documentation**: See [`AUTOMATED_RELEASES.md`](AUTOMATED_RELEASES.md) for detailed workflow information

```bash
# Example: Bump to next patch version
./scripts/bump-version.sh patch

# Example: Bump to specific version  
./scripts/bump-version.sh 1.0.3
```

### Template Workflows

This repository includes a reusable template under the `template/` directory.
Copy `template/.github` and `template/scripts` into a new project to reuse the
release and CI workflows. Set the `CRATE_NAME` environment variable in
`template/.github/workflows/release.yml` to match your crate name.


## 📄 License

This project is released under the [Unlicense](http://unlicense.org/) - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This is an unofficial library. Use at your own risk. Not affiliated with Zerodha or KiteConnect.

## 🔗 Links

- [Official KiteConnect Documentation](https://kite.trade/docs/connect/v3/)
- [API Reference](https://docs.rs/kiteconnect-async-wasm)
- [Crates.io](https://crates.io/crates/kiteconnect-async-wasm)
- [GitHub Repository](https://github.com/SPRAGE/kiteconnect-async-wasm)

---

**Built with ❤️ in Rust for the trading community**
