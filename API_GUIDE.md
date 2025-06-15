# KiteConnect Rust API Guide

This guide provides detailed information about using the KiteConnect Rust library for trading and portfolio management.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Authentication](#authentication)
3. [Portfolio Management](#portfolio-management)
4. [Order Management](#order-management)
5. [Market Data](#market-data)
6. [Mutual Funds](#mutual-funds)
7. [Error Handling](#error-handling)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kiteconnect = { version = "0.3.0", features = ["native"] }

# For WebAssembly targets
# kiteconnect = { version = "0.3.0", features = ["wasm"] }
```

### Basic Setup

```rust
use kiteconnect::connect::KiteConnect;

let mut client = KiteConnect::new("your_api_key", "");
```

## Authentication

### 1. Generate Login URL

```rust
let login_url = client.login_url();
println!("Visit: {}", login_url);
```

### 2. Exchange Request Token

After user login, exchange the request token for an access token:

```rust
let session = client
    .generate_session("request_token", "api_secret")
    .await?;
```

### 3. Set Access Token

```rust
client.set_access_token("access_token");
```

## Portfolio Management

### Holdings

Get stocks held in demat account:

```rust
let holdings = client.holdings().await?;

// Process holdings
if let Some(data) = holdings["data"].as_array() {
    for holding in data {
        println!("Stock: {}, Quantity: {}", 
            holding["tradingsymbol"], 
            holding["quantity"]);
    }
}
```

**Response Fields:**
- `tradingsymbol` - Stock symbol
- `quantity` - Total quantity held
- `average_price` - Average buying price
- `last_price` - Current market price
- `pnl` - Profit/Loss
- `product` - Product type (CNC/MIS)

### Positions

Get open trading positions:

```rust
let positions = client.positions().await?;

// Check day positions
if let Some(day_positions) = positions["data"]["day"].as_array() {
    for position in day_positions {
        if position["quantity"].as_i64().unwrap_or(0) != 0 {
            println!("Open position: {} qty {}", 
                position["tradingsymbol"], 
                position["quantity"]);
        }
    }
}
```

**Response Fields:**
- `quantity` - Net position quantity
- `buy_quantity` - Total buy quantity  
- `sell_quantity` - Total sell quantity
- `average_price` - Average position price
- `pnl` - Realized/unrealized P&L

### Margins

Get account balance and margin details:

```rust
// All segments
let margins = client.margins(None).await?;

// Specific segment
let equity_margins = client.margins(Some("equity".to_string())).await?;

// Access margin data
if let Some(equity) = margins["data"]["equity"].as_object() {
    let available = equity["available"]["live_balance"].as_f64().unwrap_or(0.0);
    println!("Available margin: ₹{:.2}", available);
}
```

## Order Management

### Get Orders

```rust
let orders = client.orders().await?;

if let Some(data) = orders["data"].as_array() {
    for order in data {
        println!("Order {}: {} - {}", 
            order["order_id"], 
            order["tradingsymbol"], 
            order["status"]);
    }
}
```

### Order History

```rust
let history = client.order_history("order_id").await?;
```

### Get Trades

```rust
let trades = client.trades().await?;

// Calculate total turnover
let total_turnover: f64 = trades["data"]
    .as_array()
    .unwrap_or(&vec![])
    .iter()
    .filter_map(|trade| {
        let price = trade["price"].as_f64()?;
        let quantity = trade["quantity"].as_f64()?;
        Some(price * quantity)
    })
    .sum();

println!("Total turnover: ₹{:.2}", total_turnover);
```

### Order Trades

```rust
let order_trades = client.order_trades("order_id").await?;
```

## Market Data

### Instruments

Get list of all trading instruments:

```rust
let instruments = client.instruments().await?;
// Returns CSV data for parsing
```

### Trigger Range

Get trigger range for stop-loss orders:

```rust
let trigger_range = client
    .trigger_range("NSE", "RELIANCE")
    .await?;
```

## Mutual Funds

### MF Orders

```rust
let mf_orders = client.mf_orders().await?;
```

### MF Instruments

```rust
let mf_instruments = client.mf_instruments().await?;
```

## Error Handling

### Basic Error Handling

```rust
match client.holdings().await {
    Ok(holdings) => {
        // Process successful response
        println!("Holdings: {:?}", holdings);
    }
    Err(e) => {
        // Handle error
        eprintln!("Error fetching holdings: {}", e);
    }
}
```

### Using `anyhow::Result`

```rust
use anyhow::Result;

async fn get_portfolio_data(client: &KiteConnect) -> Result<()> {
    let holdings = client.holdings().await?;
    let positions = client.positions().await?;
    let margins = client.margins(None).await?;
    
    // Process data...
    
    Ok(())
}
```

## Best Practices

### 1. Connection Reuse

The KiteConnect client is designed to be reused and cloned:

```rust
let client = KiteConnect::new("api_key", "access_token");

// Clone for concurrent use
let client1 = client.clone();
let client2 = client.clone();

// Use in different tasks
tokio::spawn(async move {
    let holdings = client1.holdings().await;
});

tokio::spawn(async move {
    let positions = client2.positions().await;
});
```

### 2. Concurrent API Calls

Use `tokio::try_join!` for concurrent requests:

```rust
let (holdings, positions, margins) = tokio::try_join!(
    client.holdings(),
    client.positions(),
    client.margins(None)
)?;
```

### 3. Session Management

Set up session expiry handling:

```rust
fn handle_session_expiry() {
    println!("Session expired! Please re-authenticate.");
    // Implement re-authentication logic
}

client.set_session_expiry_hook(handle_session_expiry);
```

### 4. Rate Limiting

Be mindful of API rate limits. Implement appropriate delays between requests if needed:

```rust
use tokio::time::{sleep, Duration};

// Add small delay between rapid requests
let holdings = client.holdings().await?;
sleep(Duration::from_millis(100)).await;
let positions = client.positions().await?;
```

## Examples

### Complete Authentication Flow

```rust
use kiteconnect::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KiteConnect::new("your_api_key", "");
    
    // 1. Generate login URL
    let login_url = client.login_url();
    println!("Login at: {}", login_url);
    
    // 2. User completes login, you get request_token
    // 3. Generate session
    let session = client
        .generate_session("request_token", "api_secret")
        .await?;
    
    // 4. Use API
    let holdings = client.holdings().await?;
    println!("Holdings: {:?}", holdings);
    
    Ok(())
}
```

### Portfolio Summary

```rust
async fn portfolio_summary(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
    let (holdings, positions, margins) = tokio::try_join!(
        client.holdings(),
        client.positions(),
        client.margins(None)
    )?;
    
    // Holdings summary
    if let Some(holdings_data) = holdings["data"].as_array() {
        println!("Holdings: {} stocks", holdings_data.len());
        
        let total_value: f64 = holdings_data
            .iter()
            .filter_map(|h| {
                let quantity = h["quantity"].as_f64()?;
                let last_price = h["last_price"].as_f64()?;
                Some(quantity * last_price)
            })
            .sum();
            
        println!("Total holdings value: ₹{:.2}", total_value);
    }
    
    // Positions summary
    if let Some(day_positions) = positions["data"]["day"].as_array() {
        let open_positions: Vec<_> = day_positions
            .iter()
            .filter(|p| p["quantity"].as_i64().unwrap_or(0) != 0)
            .collect();
            
        println!("Open positions: {}", open_positions.len());
    }
    
    // Margin summary
    if let Some(equity) = margins["data"]["equity"].as_object() {
        let available = equity["available"]["live_balance"].as_f64().unwrap_or(0.0);
        println!("Available margin: ₹{:.2}", available);
    }
    
    Ok(())
}
```

## Common Response Formats

### Standard API Response

```json
{
    "status": "success",
    "data": {
        // Response data
    }
}
```

### Error Response

```json
{
    "status": "error",
    "message": "Error description",
    "error_type": "TokenException"
}
```

## Platform-Specific Notes

### Native (Tokio)
- Full CSV parsing for instruments
- Complete async/await support
- All features available

### WASM (Browser)
- Raw CSV returned for client-side parsing
- All APIs supported
- Compatible with web frameworks

## Support and Resources

- **Documentation**: [Cargo docs](https://docs.rs/kiteconnect)
- **Examples**: See `examples/` directory
- **KiteConnect API Docs**: [Official API documentation](https://kite.trade/docs/connect/v3/)

---

This guide covers the main features of the KiteConnect Rust library. For detailed API documentation, see the generated rustdoc documentation.
