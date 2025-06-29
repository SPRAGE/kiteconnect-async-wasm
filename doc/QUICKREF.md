# Quick Reference Guide

## Authentication
```rust
let mut client = KiteConnect::new("api_key", "");
let login_url = client.login_url();
// User completes login...
let session = client.generate_session("request_token", "api_secret").await?;
```

## Portfolio Operations
```rust
let holdings = client.holdings().await?;
let positions = client.positions().await?;
let margins = client.margins(None).await?;
```

## Order Management
```rust
let orders = client.orders().await?;
let trades = client.trades().await?;
```

## Market Data
```rust
let instruments = client.instruments().await?;
let trigger_range = client.trigger_range("NSE", "RELIANCE").await?;
```

## Error Handling
```rust
match client.holdings().await {
    Ok(holdings) => println!("Success: {:?}", holdings),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Concurrent Operations
```rust
let (holdings, positions) = tokio::try_join!(
    client.holdings(),
    client.positions()
)?;
```
