//! # Comprehensive KiteConnect API Example
//! 
//! This example demonstrates various KiteConnect API operations including
//! authentication, portfolio management, order operations, and market data access
//! using the new modular, type-safe API.

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
    
    // Get holdings - now returns typed structs
    match client.holdings().await {
        Ok(holdings) => {
            println!("Holdings retrieved successfully");
            println!("Number of holdings: {}", holdings.len());
            
            for holding in holdings.iter().take(3) { // Show first 3
                println!("  📈 {} - Qty: {}, Avg: ₹{:.2}, LTP: ₹{:.2}, P&L: ₹{:.2}", 
                    holding.tradingsymbol,
                    holding.quantity,
                    holding.average_price,
                    holding.last_price,
                    holding.pnl
                );
            }
        }
        Err(e) => println!("Failed to get holdings: {}", e),
    }

    // Get positions - typed response
    match client.positions().await {
        Ok(positions) => {
            println!("Positions retrieved successfully");
            println!("Day positions: {}, Net positions: {}", 
                positions.day.len(), 
                positions.net.len()
            );
            
            // Show day positions with non-zero quantity
            for position in &positions.day {
                if position.quantity != 0 {
                    println!("  📊 {} - Qty: {}, P&L: ₹{:.2}", 
                        position.tradingsymbol,
                        position.quantity,
                        position.pnl
                    );
                }
            }
        }
        Err(e) => println!("Failed to get positions: {}", e),
    }

    // Get margins
    match client.margins(None).await {
        Ok(margins) => {
            println!("Margins retrieved successfully");
            println!("Available margin: ₹{:.2}", margins.equity.available.live_balance);
        }
        Err(e) => println!("Failed to get margins: {}", e),
    }
    
    // Step 3: Orders and Trades
    println!("\n=== Orders and Trades ===");
    
    // Get today's orders - now returns typed structs
    match client.orders().await {
        Ok(orders) => {
            println!("Orders retrieved successfully");
            println!("Total orders today: {}", orders.len());
            
            // Group by status
            let mut status_count = std::collections::HashMap::new();
            for order in &orders {
                *status_count.entry(&order.status).or_insert(0) += 1;
            }
            
            for (status, count) in status_count {
                println!("  - {}: {}", status, count);
            }
            
            // Show recent orders
            for order in orders.iter().take(3) {
                println!("  📝 {} {} {} @ ₹{:.2} - {}", 
                    order.trading_symbol,
                    order.transaction_type,
                    order.quantity,
                    order.price,
                    order.status
                );
            }
        }
        Err(e) => println!("Failed to get orders: {}", e),
    }
    
    // Get trades - typed response
    match client.trades().await {
        Ok(trades) => {
            println!("Trades retrieved successfully");
            println!("Total trades today: {}", trades.len());
            
            let total_turnover: f64 = trades
                .iter()
                .map(|trade| trade.average_price * trade.quantity)
                .sum();
                
            println!("Total turnover: ₹{:.2}", total_turnover);
            
            // Show recent trades
            for trade in trades.iter().take(3) {
                println!("  💰 {} {} {} @ ₹{:.2}", 
                    trade.trading_symbol,
                    trade.transaction_type,
                    trade.quantity,
                    trade.average_price
                );
            }
        }
        Err(e) => println!("Failed to get trades: {}", e),
    }
    
    // Step 4: Market Data
    println!("\n=== Market Data ===");
    
    // Get instruments - returns Vec<Instrument>
    match client.instruments(None).await {
        Ok(instruments) => {
            println!("Instruments data retrieved successfully");
            println!("Total instruments: {}", instruments.len());
            
            // Show some instrument examples
            for instrument in instruments.iter().take(3) {
                println!("  🏢 {} ({}) - Token: {}", 
                    instrument.tradingsymbol,
                    instrument.exchange,
                    instrument.instrument_token
                );
            }
        }
        Err(e) => println!("Failed to get instruments: {}", e),
    }
    
    // Demo quote data (you would use real instrument tokens)
    // let quote_result = client.quote(&["NSE:INFY"]).await;
    // match quote_result {
    //     Ok(quotes) => {
    //         println!("Quote data retrieved successfully");
    //         for (symbol, quote) in quotes {
    //             println!("  📊 {} - LTP: ₹{:.2}, Volume: {}", 
    //                 symbol, quote.last_price, quote.volume);
    //         }
    //     }
    //     Err(e) => println!("Failed to get quotes: {}", e),
    // }
    
    // Step 5: Mutual Funds
    println!("\n=== Mutual Funds ===");
    
    // Get MF orders - typed response
    match client.mf_orders(None).await {
        Ok(mf_orders) => {
            println!("MF orders retrieved successfully");
            println!("MF orders count: {}", mf_orders.len());
            
            for order in mf_orders.iter().take(3) {
                println!("  🏦 {} - {} ₹{:.2} - {}", 
                    order.tradingsymbol,
                    order.transaction_type,
                    order.amount,
                    order.status
                );
            }
        }
        Err(e) => println!("Failed to get MF orders: {}", e),
    }
    
    // Get MF holdings
    match client.mf_holdings().await {
        Ok(mf_holdings) => {
            println!("MF holdings retrieved successfully");
            println!("MF holdings count: {}", mf_holdings.len());
            
            for holding in mf_holdings.iter().take(3) {
                println!("  💼 {} - Qty: {:.3}, P&L: ₹{:.2}", 
                    holding.tradingsymbol,
                    holding.quantity,
                    holding.pnl
                );
            }
        }
        Err(e) => println!("Failed to get MF holdings: {}", e),
    }
    
    // Step 6: Concurrent API Calls
    println!("\n=== Concurrent Operations ===");
    
    // Demonstrate concurrent API calls with cloned clients
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
            println!("   - Holdings: {} items", holdings.len());
            println!("   - Day positions: {} items", positions.day.len());
            println!("   - Net positions: {} items", positions.net.len());
            println!("   - Available margin: ₹{:.2}", margins.equity.available.live_balance);
        }
        Err(e) => println!("❌ Concurrent requests failed: {}", e),
    }
    
    // Step 7: GTT (Good Till Triggered) Orders
    println!("\n=== GTT Orders ===");
    
    match client.gtts(None).await {
        Ok(gtts) => {
            println!("GTT orders retrieved successfully");
            println!("Active GTT orders: {}", gtts.len());
            
            for gtt in gtts.iter().take(3) {
                println!("  ⏰ {} - Trigger: ₹{:.2}, Status: {}", 
                    gtt.condition.tradingsymbol,
                    gtt.condition.trigger_values.first().unwrap_or(&0.0),
                    gtt.status
                );
            }
        }
        Err(e) => println!("Failed to get GTT orders: {}", e),
    }
    
    println!("\n=== Example completed ===");
    println!("This example demonstrates the main KiteConnect API features.");
    println!("Replace placeholder values with real API credentials to test.");
    println!("\nKey benefits of the new modular API:");
    println!("✓ Type-safe responses (no more JSON parsing!)");
    println!("✓ Modular architecture for better organization");
    println!("✓ Comprehensive error handling");
    println!("✓ Full async/await support");
    println!("✓ Clone support for concurrent operations");
    
    Ok(())
}
