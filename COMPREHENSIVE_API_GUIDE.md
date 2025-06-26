# KiteConnect Async WASM - API Documentation Guide

This guide provides comprehensive documentation for using the KiteConnect Async WASM library v1.0.3 with real-world examples and best practices.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Authentication Flow](#authentication-flow)
3. [Portfolio Management](#portfolio-management)
4. [Order Management](#order-management)
5. [Market Data](#market-data)
6. [Historical Data (Enhanced v1.0.3)](#historical-data-enhanced-v103)
7. [Mutual Funds](#mutual-funds)
8. [Error Handling](#error-handling)
9. [Rate Limiting](#rate-limiting)
10. [Best Practices](#best-practices)

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kiteconnect-async-wasm = "1.0.3"
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

### Basic Setup

```rust
use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KiteConnect::new("your_api_key", "your_access_token");
    
    // Your trading logic here
    
    Ok(())
}
```

## Authentication Flow

### Complete Authentication Example

```rust
use kiteconnect_async_wasm::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize client with API key
    let mut client = KiteConnect::new("your_api_key", "");
    
    // Step 2: Get login URL
    let login_url = client.login_url();
    println!("Please visit: {}", login_url);
    println!("After login, copy the request_token from the callback URL");
    
    // Step 3: Get request token from user input
    let request_token = "your_request_token_from_callback";
    
    // Step 4: Generate session
    let session = client.generate_session(request_token, "your_api_secret").await?;
    println!("Access token: {}", session.access_token);
    
    // The client now has the access token and is ready for trading
    
    Ok(())
}
```

### User Profile Information

```rust
// Get user profile (typed API)
let profile = client.profile_typed().await?;
println!("User: {} ({})", profile.user_name, profile.user_id);
println!("Broker: {}", profile.broker);
println!("Email: {}", profile.email);

// Get margin information
let margins = client.margins_typed(None).await?;
println!("Available margin: ₹{:.2}", margins.equity.available.live_balance);
```

## Portfolio Management

### Holdings Management

```rust
use kiteconnect_async_wasm::models::portfolio::Holding;

// Get all holdings (typed API)
let holdings: Vec<Holding> = client.holdings_typed().await?;

println!("Holdings Summary:");
println!("================");

let mut total_investment = 0.0;
let mut total_current_value = 0.0;
let mut total_pnl = 0.0;

for holding in &holdings {
    let investment = holding.average_price * holding.quantity as f64;
    let current_value = holding.last_price * holding.quantity as f64;
    let pnl = current_value - investment;
    
    total_investment += investment;
    total_current_value += current_value;
    total_pnl += pnl;
    
    println!("  {}: {} shares", holding.trading_symbol, holding.quantity);
    println!("    Avg: ₹{:.2}, LTP: ₹{:.2}", holding.average_price, holding.last_price);
    println!("    Investment: ₹{:.2}, Current: ₹{:.2}", investment, current_value);
    println!("    P&L: ₹{:.2} ({:.2}%)", pnl, (pnl / investment) * 100.0);
    println!();
}

println!("Total Investment: ₹{:.2}", total_investment);
println!("Total Current Value: ₹{:.2}", total_current_value);
println!("Total P&L: ₹{:.2} ({:.2}%)", total_pnl, (total_pnl / total_investment) * 100.0);
```

### Positions Monitoring

```rust
use kiteconnect_async_wasm::models::portfolio::Position;

// Get all positions
let positions: Vec<Position> = client.positions_typed().await?;

println!("Active Positions:");
println!("=================");

for position in &positions {
    if position.quantity != 0 {
        let pnl = position.pnl + position.unrealised;
        
        println!("  {}: {} {} shares", 
            position.trading_symbol, 
            position.quantity.abs(), 
            if position.quantity > 0 { "LONG" } else { "SHORT" }
        );
        println!("    Avg: ₹{:.2}, LTP: ₹{:.2}", position.average_price, position.last_price);
        println!("    P&L: ₹{:.2}", pnl);
        println!();
    }
}
```

## Order Management

### Placing Orders

```rust
use kiteconnect_async_wasm::models::orders::{OrderParams, OrderBuilder};
use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, OrderType, Product, Validity};

// Method 1: Using OrderParams directly
let order_params = OrderParams {
    exchange: Exchange::NSE,
    tradingsymbol: "RELIANCE".to_string(),
    transaction_type: TransactionType::BUY,
    quantity: 10,
    order_type: OrderType::LIMIT,
    product: Product::CNC,
    price: Some(2500.0),
    validity: Validity::DAY,
    ..Default::default()
};

let order_response = client.place_order_typed(&order_params).await?;
println!("Order placed: {}", order_response.order_id);

// Method 2: Using OrderBuilder (more ergonomic)
let order = OrderBuilder::new()
    .exchange(Exchange::NSE)
    .symbol("TCS")
    .transaction_type(TransactionType::BUY)
    .quantity(5)
    .order_type(OrderType::MARKET)
    .product(Product::CNC)
    .validity(Validity::DAY)
    .build();

let response = client.place_order_typed(&order).await?;
println!("Market order placed: {}", response.order_id);
```

### Order Status and History

```rust
// Get all orders
let orders = client.orders_typed().await?;

println!("Recent Orders:");
println!("==============");

for order in &orders {
    println!("  {} ({}): {} {} {}",
        order.order_id,
        order.status,
        order.transaction_type,
        order.quantity,
        order.trading_symbol
    );
    
    if let Some(price) = order.price {
        println!("    Price: ₹{:.2}", price);
    }
    
    if order.filled_quantity > 0 {
        println!("    Filled: {} @ ₹{:.2}", 
            order.filled_quantity, 
            order.average_price.unwrap_or(0.0)
        );
    }
    println!();
}
```

### Modifying and Cancelling Orders

```rust
use kiteconnect_async_wasm::models::orders::OrderModifyParams;

// Modify an order
let modify_params = OrderModifyParams {
    order_id: "order_id_here".to_string(),
    price: Some(2450.0),  // New price
    quantity: Some(15),   // New quantity
    order_type: Some(OrderType::LIMIT),
    validity: Some(Validity::DAY),
    ..Default::default()
};

let modify_response = client.modify_order_typed(&modify_params).await?;
println!("Order modified: {}", modify_response.order_id);

// Cancel an order
let cancel_response = client.cancel_order("order_id_here").await?;
println!("Order cancelled: {}", cancel_response.order_id);
```

## Market Data

### Real-time Quotes

```rust
// Get quotes for multiple instruments
let instruments = vec!["NSE:RELIANCE", "NSE:TCS", "NSE:INFY"];
let quotes = client.quote_typed(instruments).await?;

println!("Live Quotes:");
println!("============");

for quote in &quotes {
    println!("  {}: ₹{:.2}", quote.trading_symbol, quote.last_price);
    println!("    OHLC: {:.2}/{:.2}/{:.2}/{:.2}", 
        quote.ohlc.open, quote.ohlc.high, quote.ohlc.low, quote.ohlc.close);
    println!("    Volume: {}", quote.volume_traded);
    println!("    Change: {:.2} ({:.2}%)", quote.net_change, quote.oi_day_change);
    println!();
}
```

### OHLC and LTP Data

```rust
// Get OHLC data
let instruments = vec!["NSE:NIFTY50", "NSE:BANKNIFTY"];
let ohlc_data = client.ohlc_typed(instruments.clone()).await?;

for ohlc in &ohlc_data {
    println!("{}: O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
        ohlc.instrument_token, ohlc.open, ohlc.high, ohlc.low, ohlc.close);
}

// Get just the last traded prices
let ltp_data = client.ltp_typed(instruments).await?;

for ltp in &ltp_data {
    println!("LTP for {}: ₹{:.2}", ltp.instrument_token, ltp.last_price);
}
```

## Historical Data (Enhanced v1.0.3)

### Basic Historical Data Request

```rust
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
use kiteconnect_async_wasm::models::common::Interval;
use chrono::NaiveDateTime;

// Enhanced v1.0.3 approach with structured request
let request = HistoricalDataRequest::new(
    738561,  // RELIANCE instrument token
    NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    Interval::Day,
);

let historical_data = client.historical_data_typed(request).await?;

println!("Historical Data for RELIANCE:");
println!("=============================");

for candle in &historical_data.candles {
    println!("  {}: O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}",
        candle.date.format("%Y-%m-%d"),
        candle.open,
        candle.high,
        candle.low,
        candle.close,
        candle.volume
    );
}
```

### Advanced Historical Data with Options

```rust
// Intraday data with continuous flag and OI
let intraday_request = HistoricalDataRequest::new(
    738561,
    NaiveDateTime::parse_from_str("2023-11-20 09:00:00", "%Y-%m-%d %H:%M:%S")?,
    NaiveDateTime::parse_from_str("2023-11-20 16:00:00", "%Y-%m-%d %H:%M:%S")?,
    Interval::FiveMinute,
).continuous(true)  // Include pre/post market
 .with_oi(false);   // No OI for equity

let data = client.historical_data_typed(intraday_request).await?;

// Calculate statistics
let prices: Vec<f64> = data.candles.iter().map(|c| c.close).collect();
let high = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
let low = prices.iter().cloned().fold(f64::INFINITY, f64::min);
let avg = prices.iter().sum::<f64>() / prices.len() as f64;

println!("Intraday Statistics:");
println!("  High: ₹{:.2}", high);
println!("  Low: ₹{:.2}", low);
println!("  Average: ₹{:.2}", avg);
println!("  Range: ₹{:.2} ({:.2}%)", high - low, ((high - low) / low) * 100.0);
```

## Mutual Funds

### MF Orders and Holdings

```rust
// Get MF orders
let mf_orders = client.mf_orders_typed().await?;

println!("Mutual Fund Orders:");
println!("===================");

for order in &mf_orders {
    println!("  {}: {} units of {}",
        order.order_id,
        order.quantity,
        order.trading_symbol
    );
    println!("    Status: {}, Amount: ₹{:.2}",
        order.status,
        order.amount.unwrap_or(0.0)
    );
}

// Get MF holdings
let mf_holdings = client.mf_holdings_typed().await?;

let mut total_value = 0.0;
for holding in &mf_holdings {
    let value = holding.last_price * holding.quantity;
    total_value += value;
    
    println!("  {}: {} units @ ₹{:.4}",
        holding.trading_symbol,
        holding.quantity,
        holding.last_price
    );
    println!("    Value: ₹{:.2}, P&L: ₹{:.2}",
        value,
        holding.pnl
    );
}

println!("Total MF Portfolio Value: ₹{:.2}", total_value);
```

## Error Handling

### Comprehensive Error Handling

```rust
use kiteconnect_async_wasm::models::common::KiteError;

async fn robust_trading_operation(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
    match client.holdings_typed().await {
        Ok(holdings) => {
            println!("Successfully retrieved {} holdings", holdings.len());
            // Process holdings...
        }
        Err(KiteError::Authentication(msg)) => {
            eprintln!("Authentication failed: {}", msg);
            // Redirect to login or refresh token
            return Err("Please re-authenticate".into());
        }
        Err(KiteError::Api { status, message, error_type }) => {
            eprintln!("API error {}: {}", status, message);
            
            match status.as_str() {
                "429" => {
                    eprintln!("Rate limited - waiting before retry");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    // Retry logic here
                }
                "500" | "502" | "503" => {
                    eprintln!("Server error - will retry");
                    // Implement exponential backoff
                }
                _ => {
                    eprintln!("Unhandled API error: {}", error_type.unwrap_or_default());
                }
            }
        }
        Err(KiteError::RateLimit { retry_after, .. }) => {
            eprintln!("Rate limited. Retry after: {:?}", retry_after);
            if let Some(delay) = retry_after {
                tokio::time::sleep(delay).await;
                // Retry the operation
            }
        }
        Err(KiteError::Json(json_err)) => {
            eprintln!("JSON parsing error: {}", json_err);
            // This usually indicates an API change or network corruption
        }
        Err(KiteError::Http(http_err)) => {
            eprintln!("Network error: {}", http_err);
            // Check internet connection, DNS, etc.
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }
    
    Ok(())
}
```

## Rate Limiting

### Understanding Rate Limits

The library automatically handles rate limiting, but understanding the limits helps optimize your application:

```rust
// Quote data: 1 request/second (most restrictive)
async fn get_multiple_quotes_safely(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
    let symbols = vec!["NSE:RELIANCE", "NSE:TCS", "NSE:INFY", "NSE:HDFC"];
    
    // Option 1: Single request for multiple symbols (recommended)
    let quotes = client.quote_typed(symbols).await?;
    
    // Option 2: Multiple requests (automatically rate-limited)
    // for symbol in &symbols {
    //     let quote = client.quote_typed(vec![symbol]).await?;
    //     // Library automatically waits 1 second between requests
    // }
    
    Ok(())
}

// Historical data: 3 requests/second
async fn get_historical_data_batch(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
    let instruments = vec![738561, 492033, 408065]; // RELIANCE, TCS, INFY
    
    for instrument in instruments {
        let request = HistoricalDataRequest::new(
            instrument,
            NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
            NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
            Interval::Day,
        );
        
        let data = client.historical_data_typed(request).await?;
        // Library automatically enforces 3 req/sec limit
        
        println!("Got {} candles for instrument {}", data.candles.len(), instrument);
    }
    
    Ok(())
}
```

## Best Practices

### 1. Connection Management

```rust
// Reuse client instances
lazy_static::lazy_static! {
    static ref KITE_CLIENT: KiteConnect = {
        KiteConnect::new(
            std::env::var("KITE_API_KEY").expect("KITE_API_KEY not set"),
            std::env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN not set")
        )
    };
}

async fn efficient_data_access() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch multiple data types concurrently
    let (holdings, positions, orders) = tokio::try_join!(
        KITE_CLIENT.holdings_typed(),
        KITE_CLIENT.positions_typed(),
        KITE_CLIENT.orders_typed()
    )?;
    
    // Process results...
    
    Ok(())
}
```

### 2. Error Recovery with Exponential Backoff

```rust
async fn retry_with_backoff<F, T>(mut operation: F, max_retries: u32) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnMut() -> Result<T, KiteError>,
{
    let mut delay = std::time::Duration::from_millis(100);
    
    for attempt in 0..max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(KiteError::RateLimit { retry_after, .. }) => {
                if let Some(delay) = retry_after {
                    tokio::time::sleep(delay).await;
                } else {
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
            Err(KiteError::Api { status, .. }) if status == "500" || status == "502" => {
                if attempt == max_retries - 1 {
                    return Err("Max retries exceeded".into());
                }
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(e) => return Err(e.into()),
        }
    }
    
    Err("Max retries exceeded".into())
}
```

### 3. Portfolio Monitoring Example

```rust
use std::collections::HashMap;

async fn monitor_portfolio(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Get current portfolio state
        let holdings = client.holdings_typed().await?;
        let positions = client.positions_typed().await?;
        
        // Build symbol list for quotes
        let mut symbols = Vec::new();
        for holding in &holdings {
            symbols.push(format!("{}:{}", holding.exchange, holding.trading_symbol));
        }
        
        // Get live quotes
        let quotes = client.quote_typed(symbols.iter().map(|s| s.as_str()).collect()).await?;
        let quote_map: HashMap<_, _> = quotes.iter()
            .map(|q| (q.trading_symbol.clone(), q))
            .collect();
        
        // Calculate portfolio metrics
        let mut total_investment = 0.0;
        let mut total_value = 0.0;
        
        for holding in &holdings {
            if let Some(quote) = quote_map.get(&holding.trading_symbol) {
                let investment = holding.average_price * holding.quantity as f64;
                let current_value = quote.last_price * holding.quantity as f64;
                
                total_investment += investment;
                total_value += current_value;
                
                let pnl = current_value - investment;
                let pnl_pct = (pnl / investment) * 100.0;
                
                if pnl_pct.abs() > 5.0 {
                    println!("ALERT: {} moved {:.2}% (₹{:.2})", 
                        holding.trading_symbol, pnl_pct, pnl);
                }
            }
        }
        
        let total_pnl = total_value - total_investment;
        let total_pnl_pct = (total_pnl / total_investment) * 100.0;
        
        println!("Portfolio: ₹{:.2} ({:.2}%) | Last update: {}",
            total_pnl, total_pnl_pct, chrono::Utc::now().format("%H:%M:%S"));
        
        // Wait before next update (respect rate limits)
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
```

This documentation provides comprehensive examples for all major use cases of the KiteConnect Async WASM library. The examples demonstrate both the legacy and new typed APIs, proper error handling, and best practices for production use.
