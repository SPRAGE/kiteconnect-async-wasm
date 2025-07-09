//! # Comprehensive Debug for Instruments Issue
//!
//! This example provides extensive debugging to identify why instruments fetching returns 0 results.
//! It checks authentication, rate limiting, response format, and CSV parsing.
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
//! cargo run --example comprehensive_debug --features=native,debug
//! ```

use kiteconnect_async_wasm::connect::KiteConnect;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    println!("🔧 Comprehensive Instruments Debug");
    println!("==================================\n");
    
    println!("📋 Configuration:");
    println!("   API Key: {}****", &api_key[..4.min(api_key.len())]);
    println!("   Access Token: {}****", &access_token[..4.min(access_token.len())]);
    println!();

    // Create KiteConnect client
    let client = KiteConnect::new(&api_key, &access_token);

    // Check basic authentication first with a simple endpoint
    println!("🔐 Testing authentication with user profile...");
    match client.profile().await {
        Ok(profile) => {
            println!("✅ Authentication successful");
            println!("   User: {}", profile.get("user_name").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
        }
        Err(e) => {
            println!("❌ Authentication failed: {}", e);
            println!("   This suggests invalid API key or access token");
            return Ok(());
        }
    }
    println!();

    // Test rate limiter
    println!("⏱️  Rate limiter status:");
    let stats = client.rate_limiter_stats().await;
    println!("   Enabled: {}", client.is_rate_limiting_enabled());
    println!("   Stats: {:?}", stats);
    println!();

    // Test instruments endpoint step by step
    println!("📊 Testing instruments endpoint...");
    
    // Step 1: Test the raw API call
    println!("   Step 1: Raw instruments API call");
    match client.instruments(None).await {
        Ok(response) => {
            println!("   ✅ API call successful");
            println!("   📄 Response type: {:?}", response);
            
            if let Some(array) = response.as_array() {
                println!("   📊 Array length: {}", array.len());
                if array.is_empty() {
                    println!("   ⚠️  Empty array returned - this indicates the issue");
                    
                    // Check if this is an error disguised as an empty array
                    if let Some(obj) = response.as_object() {
                        if obj.contains_key("error_type") || obj.contains_key("message") {
                            println!("   ❌ Response contains error fields: {:?}", obj);
                        }
                    }
                } else {
                    println!("   ✅ Instruments found!");
                    if let Some(first) = array.first() {
                        println!("   📄 First instrument: {}", serde_json::to_string_pretty(first)?);
                    }
                }
            } else if let Some(obj) = response.as_object() {
                println!("   📄 Object response: {}", serde_json::to_string_pretty(obj)?);
                if obj.contains_key("error_type") {
                    println!("   ❌ API returned error object");
                }
            } else {
                println!("   ❌ Unexpected response format: {:?}", response);
            }
        }
        Err(e) => {
            println!("   ❌ API call failed: {}", e);
            return Ok(());
        }
    }
    println!();

    // Step 2: Test specific exchange
    println!("   Step 2: Testing NSE-specific instruments");
    match client.instruments(Some("NSE")).await {
        Ok(response) => {
            if let Some(array) = response.as_array() {
                println!("   ✅ NSE instruments: {}", array.len());
            } else {
                println!("   ❌ NSE response not an array: {:?}", response);
            }
        }
        Err(e) => {
            println!("   ❌ NSE instruments failed: {}", e);
        }
    }
    println!();

    // Step 3: Test typed API
    println!("   Step 3: Testing typed instruments API");
    match client.instruments_typed(None).await {
        Ok(instruments) => {
            println!("   ✅ Typed API returned {} instruments", instruments.len());
            if !instruments.is_empty() {
                let first = &instruments[0];
                println!("   📄 First typed instrument: {}", first.trading_symbol);
            }
        }
        Err(e) => {
            println!("   ❌ Typed API failed: {}", e);
        }
    }
    println!();

    // Step 4: Test other endpoints to verify general connectivity
    println!("🔍 Testing other endpoints for comparison:");
    
    println!("   Testing holdings...");
    match client.holdings().await {
        Ok(holdings) => {
            if let Some(array) = holdings.as_array() {
                println!("   ✅ Holdings: {} items", array.len());
            } else {
                println!("   📄 Holdings response: {:?}", holdings);
            }
        }
        Err(e) => {
            println!("   ❌ Holdings failed: {}", e);
        }
    }

    println!("   Testing margins...");
    match client.margins(None).await {
        Ok(_margins) => {
            println!("   ✅ Margins call successful");
        }
        Err(e) => {
            println!("   ❌ Margins failed: {}", e);
        }
    }
    println!();

    // Final statistics
    println!("📈 Final statistics:");
    println!("   Total requests made: {}", client.request_count());
    let final_stats = client.rate_limiter_stats().await;
    println!("   Rate limiter stats: {:?}", final_stats);
    
    println!("\n💡 Debugging tips:");
    println!("   - If authentication works but instruments returns 0, check API permissions");
    println!("   - If other endpoints work, the issue is specific to instruments endpoint");
    println!("   - Enable RUST_LOG=debug for detailed request/response logging");
    println!("   - Check if you have a sandbox vs production API key mismatch");

    Ok(())
}
