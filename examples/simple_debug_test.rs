//! # Simple Instruments Debug Test
//!
//! Test using the simplified instruments method that bypasses rate limiting and caching.
//!
//! ## Usage
//!
//! Set environment variables:
//! ```bash
//! export KITE_API_KEY="your_api_key"
//! export KITE_ACCESS_TOKEN="your_access_token"
//! ```
//!
//! Run with:
//! ```bash
//! cargo run --example simple_debug_test --features=native
//! ```

use kiteconnect_async_wasm::connect::KiteConnect;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    // Create KiteConnect client
    let client = KiteConnect::new(&api_key, &access_token);

    println!("🧪 Simple Instruments Debug Test");
    println!("=================================\n");

    // Test 1: Use simplified instruments method
    println!("🔄 Testing simplified instruments method...");
    match client.instruments_simple(None).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ Simple method returned {} instruments", array.len());
                
                if !array.is_empty() {
                    println!("📄 First instrument:");
                    if let Some(first) = array.first() {
                        if let Some(obj) = first.as_object() {
                            println!("   Trading Symbol: {}", obj.get("tradingsymbol").unwrap_or(&serde_json::Value::String("N/A".to_string())));
                            println!("   Name: {}", obj.get("name").unwrap_or(&serde_json::Value::String("N/A".to_string())));
                            println!("   Exchange: {}", obj.get("exchange").unwrap_or(&serde_json::Value::String("N/A".to_string())));
                        }
                    }
                }
            } else {
                println!("❌ Simple method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ Simple method failed: {}", e);
        }
    }
    println!();

    // Test 2: Compare with regular method
    println!("🔄 Testing regular instruments method...");
    match client.instruments(None).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ Regular method returned {} instruments", array.len());
            } else {
                println!("❌ Regular method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ Regular method failed: {}", e);
        }
    }
    println!();

    // Test 3: Test NSE-specific
    println!("🔄 Testing NSE-specific with simple method...");
    match client.instruments_simple(Some("NSE")).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ NSE simple method returned {} instruments", array.len());
            } else {
                println!("❌ NSE simple method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ NSE simple method failed: {}", e);
        }
    }

    println!("\n💡 Results Analysis:");
    println!("   - If simple method works but regular doesn't, issue is in rate limiting/caching");
    println!("   - If simple method also fails, issue is in authentication or API access");
    println!("   - If both return 0, check API permissions or contact Zerodha support");

    Ok(())
}
