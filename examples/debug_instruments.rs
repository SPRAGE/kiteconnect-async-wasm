//! # Debug Instruments Fetching
//!
//! Simple debug example to troubleshoot instruments fetching issues.
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
//! cargo run --example debug_instruments --features=native
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

    println!("🔧 Debug Instruments Fetching");
    println!("=============================\n");

    // Check rate limiter status
    let stats = client.rate_limiter_stats().await;
    println!(
        "Rate limiter enabled: {}",
        client.is_rate_limiting_enabled()
    );
    println!("Rate limiter stats: {:?}\n", stats);

    // Test 0: Instruments API
    println!("🔄 Testing instruments API...");
    match client.instruments(None).await {
        Ok(instruments_json) => {
            if let Some(instruments_array) = instruments_json.as_array() {
                println!(
                    "✅ API returned {} instruments",
                    instruments_array.len()
                );

                if instruments_array.is_empty() {
                    println!("⚠️  Empty instruments array");
                    if let Some(obj) = instruments_json.as_object() {
                        println!("📄 Response object: {}", serde_json::to_string_pretty(obj)?);
                    }
                } else {
                    // Show first instrument details for debugging
                    if let Some(first_instrument) = instruments_array.first() {
                        println!(
                            "📄 First instrument (JSON): {}",
                            serde_json::to_string_pretty(first_instrument)?
                        );
                    }
                }
            } else {
                println!(
                    "❌ API returned non-array response: {:?}",
                    instruments_json
                );
            }
        }
        Err(e) => {
            println!("❌ Instruments API failed: {:?}", e);
            println!("   Error details: {}", e);
        }
    }
    println!();

    // Test 1: Get all instruments using legacy JSON API (same as above)

    // Test 1.5: Removed gzip-specific method (implementation consolidated)

    // Test 2: Get instruments using typed API
    println!("🔄 Testing typed instruments API...");
    match client.instruments_typed(None).await {
         Ok(instruments) => {
            println!("✅ Typed API returned {} instruments", instruments.len());

             // Show first instrument details for debugging
             if let Some(first_instrument) = instruments.first() {
                println!("📄 First instrument (typed): {:#?}", first_instrument);
             }
         }
         Err(e) => {
            println!("❌ Typed API failed: {:?}", e);
         }
     }
     println!();

     // Test 3: Get instruments using regular typed API
    println!("🔄 Testing regular typed instruments API again...");
    match client.instruments_typed(None).await {
         Ok(instruments) => {
             println!(
                 "✅ Regular typed API returned {} instruments",
                 instruments.len()
             );

             // Show first instrument details for debugging
             if let Some(first_instrument) = instruments.first() {
                 println!("📄 First instrument (typed): {:#?}", first_instrument);
             }
         }
         Err(e) => {
             println!("❌ Regular typed API failed: {:?}", e);
         }
     }
     println!();

    // Test 3: Try NSE specific instruments
    println!("🔄 Testing NSE-specific instruments...");
    match client.instruments(Some("NSE")).await {
        Ok(nse_instruments_json) => {
            if let Some(nse_instruments_array) = nse_instruments_json.as_array() {
                println!("✅ NSE instruments: {}", nse_instruments_array.len());
            } else {
                println!(
                    "❌ NSE API returned non-array response: {:?}",
                    nse_instruments_json
                );
            }
        }
        Err(e) => {
            println!("❌ NSE instruments failed: {:?}", e);
        }
    }
    println!();

    // Test 4: Check request counter
    println!("📊 Request counter: {}", client.request_count());

    Ok(())
}
