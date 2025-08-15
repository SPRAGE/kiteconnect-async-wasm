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

    // Test 1: Use official instruments method
    println!("🔄 Testing instruments method...");
    match client.instruments(None).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ Method returned {} instruments", array.len());
                
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
                println!("❌ Method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ Method failed: {}", e);
        }
    }
    println!();

    // Test 2: Compare with regular method
    println!("🔄 Testing instruments method again...");
    match client.instruments(None).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ Method returned {} instruments", array.len());
            } else {
                println!("❌ Method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ Method failed: {}", e);
        }
    }
    println!();

    // Test 3: Test NSE-specific
    println!("🔄 Testing NSE-specific...");
    match client.instruments(Some("NSE")).await {
        Ok(instruments) => {
            if let Some(array) = instruments.as_array() {
                println!("✅ NSE method returned {} instruments", array.len());
            } else {
                println!("❌ NSE method returned non-array: {:?}", instruments);
            }
        }
        Err(e) => {
            println!("❌ NSE method failed: {}", e);
        }
    }

    println!("\n💡 Results Analysis:");
    println!("   - If method works intermittently, issue may be rate limiting/caching");
    println!("   - If method consistently fails, issue is in authentication or API access");
    println!("   - If both return 0, check API permissions or contact Zerodha support");

    Ok(())
}
