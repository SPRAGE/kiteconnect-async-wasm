//! # Comprehensive KiteConnect API Example
//! 
//! This example demonstrates various KiteConnect API operations including
//! authentication, portfolio management, and market data access.

use kiteconnect_async_wasm::connect::KiteConnect;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the KiteConnect client
    let mut client = KiteConnect::new("your_api_key", "");
    
    // Step 1: Authentication Flow
    println!("=== Authentication ===");
    
    // Generate login URL for user authentication
    let login_url = client.login_url();
    println!("1. Visit this URL to login: {}", login_url);
    println!("2. After login, copy the request_token from the redirect URL");
    
    // In a real application, you would:
    // - Open the login URL in a browser
    // - User completes login
    // - Extract request_token from callback URL
    // - Use it here:
    
    // Uncomment and use real tokens:
    // let session = client.generate_session("your_request_token", "your_api_secret").await?;
    // println!("Session created: {:?}", session);
    
    // For demo purposes, set access token directly
    client.set_access_token("your_access_token");
    
    // Step 2: Portfolio Information
    println!("\n=== Portfolio Information ===");
    
    // Get holdings
    match client.holdings().await {
        Ok(holdings) => {
            println!("Holdings retrieved successfully");
            if let Some(data) = holdings["data"].as_array() {
                println!("Number of holdings: {}", data.len());
                for holding in data.iter().take(3) { // Show first 3
                    println!("  - {} qty: {}", 
                        holding["tradingsymbol"].as_str().unwrap_or("N/A"),
                        holding["quantity"]);
                }
            }
        }
        Err(e) => println!("Failed to get holdings: {}", e),
    }
    
    // Get positions
    match client.positions().await {
        Ok(positions) => {
            println!("Positions retrieved successfully");
            if let Some(day_positions) = positions["data"]["day"].as_array() {
                let open_positions: Vec<_> = day_positions
                    .iter()
                    .filter(|p| p["quantity"].as_i64().unwrap_or(0) != 0)
                    .collect();
                    
                println!("Open positions: {}", open_positions.len());
                for position in open_positions.iter().take(3) {
                    println!("  - {} qty: {}", 
                        position["tradingsymbol"].as_str().unwrap_or("N/A"),
                        position["quantity"]);
                }
            }
        }
        Err(e) => println!("Failed to get positions: {}", e),
    }
    
    // Get margins
    match client.margins(None).await {
        Ok(margins) => {
            println!("Margins retrieved successfully");
            if let Some(equity) = margins["data"]["equity"].as_object() {
                println!("Equity available balance: {}", 
                    equity["available"]["live_balance"].as_f64().unwrap_or(0.0));
            }
        }
        Err(e) => println!("Failed to get margins: {}", e),
    }
    
    // Step 3: Orders and Trades
    println!("\n=== Orders and Trades ===");
    
    // Get today's orders
    match client.orders().await {
        Ok(orders) => {
            println!("Orders retrieved successfully");
            if let Some(data) = orders["data"].as_array() {
                println!("Total orders today: {}", data.len());
                
                // Group by status
                let mut status_count = std::collections::HashMap::new();
                for order in data {
                    let status = order["status"].as_str().unwrap_or("UNKNOWN");
                    *status_count.entry(status).or_insert(0) += 1;
                }
                
                for (status, count) in status_count {
                    println!("  - {}: {}", status, count);
                }
            }
        }
        Err(e) => println!("Failed to get orders: {}", e),
    }
    
    // Get trades
    match client.trades().await {
        Ok(trades) => {
            println!("Trades retrieved successfully");
            if let Some(data) = trades["data"].as_array() {
                println!("Total trades today: {}", data.len());
                
                let total_turnover: f64 = data
                    .iter()
                    .filter_map(|trade| {
                        let price = trade["price"].as_f64()?;
                        let quantity = trade["quantity"].as_f64()?;
                        Some(price * quantity)
                    })
                    .sum();
                    
                println!("Total turnover: ₹{:.2}", total_turnover);
            }
        }
        Err(e) => println!("Failed to get trades: {}", e),
    }
    
    // Step 4: Market Data
    println!("\n=== Market Data ===");
    
    // Get instruments (this can be large, so we'll just count)
    match client.instruments(None).await {
        Ok(instruments) => {
            println!("Instruments data retrieved successfully");
            // Note: instruments() returns CSV data, not JSON
            println!("Instruments data type: {}", 
                if instruments.is_string() { "CSV String" } else { "JSON" });
        }
        Err(e) => println!("Failed to get instruments: {}", e),
    }
    
    // Step 5: Mutual Funds
    println!("\n=== Mutual Funds ===");
    
    // Get MF orders
    match client.mf_orders(None).await {
        Ok(mf_orders) => {
            println!("MF orders retrieved successfully");
            if let Some(data) = mf_orders["data"].as_array() {
                println!("MF orders count: {}", data.len());
            }
        }
        Err(e) => println!("Failed to get MF orders: {}", e),
    }
    
    // Step 6: Concurrent API Calls
    println!("\n=== Concurrent Operations ===");
    
    // Demonstrate concurrent API calls
    let client1 = client.clone();
    let client2 = client.clone();
    let client3 = client.clone();
    
    let start = std::time::Instant::now();
    
    // Fetch multiple endpoints concurrently
    let results = tokio::try_join!(
        client1.holdings(),
        client2.positions(), 
        client3.margins(None)
    );
    
    let duration = start.elapsed();
    
    match results {
        Ok((holdings, positions, margins)) => {
            println!("✅ All concurrent requests completed in {:?}", duration);
            println!("   - Holdings: {} items", 
                holdings["data"].as_array().map_or(0, |a| a.len()));
            println!("   - Positions: {} day positions", 
                positions["data"]["day"].as_array().map_or(0, |a| a.len()));
            println!("   - Margins: {} segments", 
                margins["data"].as_object().map_or(0, |o| o.len()));
        }
        Err(e) => println!("❌ Concurrent requests failed: {}", e),
    }
    
    println!("\n=== Example completed ===");
    println!("This example demonstrates the main KiteConnect API features.");
    println!("Replace placeholder values with real API credentials to test.");
    
    Ok(())
}
