# Migration Guide: v1.0.2 → v1.0.3

This guide helps you migrate from v1.0.2 to v1.0.3, which introduces enhanced APIs and improved type safety while maintaining backward compatibility.

## 📋 Summary of Changes

v1.0.3 introduces several improvements:
- **Enhanced Historical Data API** with structured request parameters
- **Dual Serde Support** for Interval enum (accepts strings and integers)
- **Organized Enum System** with better module structure
- **Improved Code Quality** with clippy optimizations

## ✅ Backward Compatibility

**Good news**: Most existing code will continue to work without changes! We've maintained full backward compatibility for:
- All existing API methods
- All enum imports (through re-exports)
- All data structures
- All error types

## 🔄 Recommended Migrations

While not required, these migrations will give you access to new features and better type safety:

### 1. Historical Data API Migration

#### Before (v1.0.2 - still works)
```rust
// Old approach - still functional
let historical_data = client.historical_data(
    "738561",           // instrument_token as string
    "2023-11-01",       // from_date as string
    "2023-11-30",       // to_date as string  
    "day",              // interval as string
    "0"                 // continuous as string
).await?;
```

#### After (v1.0.3 - recommended)
```rust
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
use kiteconnect_async_wasm::models::common::Interval;
use chrono::NaiveDateTime;

// New structured approach with better type safety
let request = HistoricalDataRequest::new(
    738561,  // instrument_token as u32
    NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    Interval::Day,
).continuous(false).with_oi(true);

let historical_data = client.historical_data_typed(request).await?;
```

#### Benefits of Migration:
- ✅ Precise datetime handling (hour/minute/second precision)
- ✅ Type safety (compile-time error checking)
- ✅ Builder pattern for optional parameters
- ✅ Better IDE autocomplete and documentation
- ✅ Structured error handling

### 2. Interval Enum Usage

#### Before (v1.0.2 - still works)
```rust
// String-based approach
let interval_str = "day";
```

#### After (v1.0.3 - more flexible)
```rust
use kiteconnect_async_wasm::models::common::Interval;

// Type-safe enum approach
let interval = Interval::Day;

// Now accepts both formats in deserialization:
let from_string: Interval = serde_json::from_str("\"day\"").unwrap();
let from_integer: Interval = serde_json::from_str("0").unwrap();  // 0 = Day

// Always serializes as strings for API compatibility
assert_eq!(serde_json::to_string(&Interval::Day).unwrap(), "\"day\"");
```

### 3. Enhanced Error Handling

#### Before (v1.0.2)
```rust
match client.holdings().await {
    Ok(holdings) => println!("Holdings: {:?}", holdings),
    Err(e) => eprintln!("Error: {}", e),
}
```

#### After (v1.0.3 - enhanced)
```rust
use kiteconnect_async_wasm::models::common::KiteError;

match client.holdings_typed().await {
    Ok(holdings) => {
        println!("Found {} holdings", holdings.len());
        for holding in holdings {
            println!("{}: {} shares @ ₹{}", 
                holding.trading_symbol, 
                holding.quantity, 
                holding.last_price
            );
        }
    }
    Err(KiteError::Authentication(msg)) => {
        eprintln!("Authentication failed: {}", msg);
        // Handle re-authentication
    }
    Err(KiteError::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
        // Handle specific API errors
    }
    Err(KiteError::RateLimit { retry_after, .. }) => {
        eprintln!("Rate limited. Retry after: {:?}", retry_after);
        // Handle rate limiting
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## 📦 Import Changes

### Enum Imports (No Changes Required)

Even though enums are now organized in submodules, your existing imports continue to work:

```rust
// These imports still work (backward compatible)
use kiteconnect_async_wasm::models::common::{
    Exchange, Product, Validity, OrderType, TransactionType,
    Interval, InstrumentType, Segment, GttStatus
};

// New modular imports (optional, for better organization)
use kiteconnect_async_wasm::models::common::enums::{
    exchange::Exchange,
    trading::{Product, Validity, OrderType, TransactionType},
    interval::Interval,
    instruments::{InstrumentType, Segment},
    gtt::GttStatus,
};
```

### New Imports for Enhanced Features

```rust
// For new historical data API
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;

// For enhanced datetime handling
use chrono::NaiveDateTime;

// For better error handling
use kiteconnect_async_wasm::models::common::KiteError;
```

## 🚀 Migration Strategy

### Step 1: Update Dependencies
```toml
[dependencies]
kiteconnect-async-wasm = "1.0.3"
chrono = { version = "0.4", features = ["serde"] }  # For datetime handling
```

### Step 2: Test Existing Code
Your existing code should work without changes. Run your tests to verify.

### Step 3: Gradually Migrate APIs
Start with non-critical APIs and gradually migrate to the new typed versions:

1. **Start with market data**: Migrate `quote()` → `quote_typed()`
2. **Move to portfolio**: Migrate `holdings()` → `holdings_typed()`
3. **Update historical data**: Migrate to `HistoricalDataRequest`
4. **Enhance error handling**: Add specific error type handling

### Step 4: Update Error Handling
Enhance your error handling to take advantage of specific error types.

### Step 5: Leverage New Features
- Use precise datetime in historical data requests
- Take advantage of dual serde support for intervals
- Benefit from improved type safety

## 📊 Performance Improvements

v1.0.3 includes several performance optimizations from clippy suggestions:

- ✅ Optimized vector initialization patterns
- ✅ Reduced unnecessary allocations
- ✅ Improved range checking implementations
- ✅ Better error handling patterns
- ✅ Cleaner code organization

These improvements are automatic and require no code changes.

## 🔧 Troubleshooting

### Common Issues

1. **DateTime Parsing Errors**
   ```rust
   // If you get parsing errors, ensure correct format
   let datetime = NaiveDateTime::parse_from_str(
       "2023-11-01 09:15:00", 
       "%Y-%m-%d %H:%M:%S"
   )?;
   ```

2. **Missing chrono dependency**
   ```toml
   # Add to Cargo.toml
   chrono = { version = "0.4", features = ["serde"] }
   ```

3. **Import Errors**
   ```rust
   // If you get import errors, try the modular imports
   use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
   ```

### Getting Help

- Check the [examples directory](examples/) for complete working examples
- Review the [API documentation](https://docs.rs/kiteconnect-async-wasm)
- See the [CHANGELOG.md](CHANGELOG.md) for detailed changes

## 🎯 Benefits of Migration

After migration, you'll enjoy:

- ✅ **Better Type Safety**: Compile-time error checking
- ✅ **Enhanced IDE Support**: Better autocomplete and documentation
- ✅ **Precise Datetime Control**: Hour/minute/second precision
- ✅ **Flexible Data Handling**: Dual serde support
- ✅ **Improved Error Handling**: Specific error types
- ✅ **Better Performance**: Clippy-optimized code
- ✅ **Future-Proof**: Ready for upcoming enhancements

## 📋 Checklist

- [ ] Updated dependencies to v1.0.3
- [ ] Tested existing code (should work unchanged)
- [ ] Migrated historical data API (recommended)
- [ ] Enhanced error handling (recommended)
- [ ] Updated datetime handling for precision
- [ ] Leveraged new typed APIs
- [ ] Updated documentation and examples

Remember: Migration is optional but recommended for better type safety and new features!