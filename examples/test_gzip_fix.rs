//! # Test Gzip Fix
//!
//! Test the gzip decompression fix for the instruments API.
//!
//! ## Usage
//!
//! Set environment variables:
//! ```bash
//! export KITE_API_KEY="your_api_key"
//! export KITE_ACCESS_TOKEN="your_access_token"
//! export RUST_LOG=debug
//! ```
//!
//! Run with:
//! ```bash
//! cargo run --example test_gzip_fix --features=native,debug
//! ```

use kiteconnect_async_wasm::connect::KiteConnect;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    // Create KiteConnect client
    let client = KiteConnect::new(&api_key, &access_token);

    println!("🔧 Testing Gzip Decompression Fix");
    println!("=================================\n");

    // Test 1: Original method (should now work with gzip)
    println!("🔄 Testing fixed instruments method...");
    match client.instruments(None).await {
        Ok(instruments_json) => {
            if let Some(instruments_array) = instruments_json.as_array() {
                println!("✅ Fixed method returned {} instruments", instruments_array.len());
                
                if !instruments_array.is_empty() {
                    // Show first few instruments
                    println!("\n📄 First 3 instruments:");
                    for (i, instrument) in instruments_array.iter().take(3).enumerate() {
                        if let Some(obj) = instrument.as_object() {
                            let symbol = obj.get("tradingsymbol").and_then(|v| v.as_str()).unwrap_or("N/A");
                            let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("N/A");
                            let exchange = obj.get("exchange").and_then(|v| v.as_str()).unwrap_or("N/A");
                            let instrument_type = obj.get("instrument_type").and_then(|v| v.as_str()).unwrap_or("N/A");
                            
                            println!("  {}. {} | {} | {} | {}", i + 1, symbol, name, exchange, instrument_type);
                        }
                    }
                    
                    // Count by exchange
                    let mut exchange_counts = std::collections::HashMap::new();
                    for instrument in instruments_array.iter().take(1000) { // Sample first 1000
                        if let Some(obj) = instrument.as_object() {
                            if let Some(exchange) = obj.get("exchange").and_then(|v| v.as_str()) {
                                *exchange_counts.entry(exchange).or_insert(0) += 1;
                            }
                        }
                    }
                    
                    println!("\n📊 Exchange distribution (first 1000 instruments):");
                    for (exchange, count) in exchange_counts {
                        println!("   {}: {}", exchange, count);
                    }
                }
            } else {
                println!("❌ Method returned non-array response");
            }
        }
        Err(e) => {
            println!("❌ Fixed method failed: {}", e);
        }
    }
    println!();

    // Test 2: Test NSE specific
    println!("🔄 Testing NSE-specific instruments...");
    match client.instruments(Some("NSE")).await {
        Ok(nse_instruments) => {
            if let Some(nse_array) = nse_instruments.as_array() {
                println!("✅ NSE instruments: {}", nse_array.len());
                
                if !nse_array.is_empty() {
                    println!("📄 First NSE instrument:");
                    if let Some(first) = nse_array.first() {
                        if let Some(obj) = first.as_object() {
                            let symbol = obj.get("tradingsymbol").and_then(|v| v.as_str()).unwrap_or("N/A");
                            let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("N/A");
                            println!("   {} | {}", symbol, name);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ NSE instruments failed: {}", e);
        }
    }
    println!();

    // Test 3: Test typed API
    println!("🔄 Testing typed instruments API...");
    match client.instruments_typed(None).await {
        Ok(typed_instruments) => {
            println!("✅ Typed API returned {} instruments", typed_instruments.len());
            
            if !typed_instruments.is_empty() {
                println!("📄 First typed instrument:");
                let first = &typed_instruments[0];
                println!("   {} | {} | {:?} | {:?}", 
                    first.trading_symbol, 
                    first.name, 
                    first.instrument_type, 
                    first.exchange
                );
            }
        }
        Err(e) => {
            println!("❌ Typed API failed: {}", e);
        }
    }

    println!("\n🎉 If you see instruments above, the gzip fix is working!");
    println!("💡 The issue was that KiteConnect API returns gzipped CSV data");
    println!("   but our library was trying to parse it as plain CSV.");

    Ok(())
}
