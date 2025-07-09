//! # Minimal Instruments Test
//!
//! A minimal test to isolate the instruments issue using direct HTTP requests.
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
//! cargo run --example minimal_instruments_test --features=native
//! ```

use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    println!("🧪 Minimal Instruments Test");
    println!("============================\n");

    // Create a simple HTTP client
    let client = reqwest::Client::new();

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert("XKiteVersion", "3".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("token {}:{}", api_key, access_token).parse().unwrap(),
    );
    headers.insert(USER_AGENT, "Rust".parse().unwrap());

    // Test 1: Direct HTTP request to instruments endpoint
    println!("🔄 Making direct HTTP request to /instruments...");
    let url = "https://api.kite.trade/instruments";
    
    match client.get(url).headers(headers.clone()).send().await {
        Ok(response) => {
            let status = response.status();
            let headers_ref = response.headers().clone();
            let content_type = response.headers().get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown").to_string();
            
            println!("✅ HTTP Response Status: {}", status);
            println!("📊 Response Headers: {:?}", headers_ref);
            println!("📄 Content-Type: {}", content_type);
            
            let body = response.text().await?;
            println!("📐 Response Length: {} bytes", body.len());
            
            if body.is_empty() {
                println!("❌ Empty response body!");
            } else {
                // Show first few lines to check CSV format
                let lines: Vec<&str> = body.lines().take(5).collect();
                println!("📝 First 5 lines:");
                for (i, line) in lines.iter().enumerate() {
                    println!("   {}: {}", i + 1, line);
                }
                
                // Count total lines
                let total_lines = body.lines().count();
                println!("📊 Total lines: {}", total_lines);
                
                // Try to parse as CSV
                if content_type.contains("text/csv") || body.starts_with("instrument_token") {
                    println!("✅ Appears to be CSV format");
                    
                    // Simple CSV parsing test
                    let mut csv_reader = csv::ReaderBuilder::new().from_reader(body.as_bytes());
                    let headers = csv_reader.headers()?;
                    println!("📋 CSV Headers: {:?}", headers);
                    
                    let mut record_count = 0;
                    for result in csv_reader.records() {
                        let _record = result?;
                        record_count += 1;
                        if record_count >= 10 { break; } // Just test first 10 records
                    }
                    println!("✅ Successfully parsed {} CSV records", record_count);
                } else {
                    println!("❓ Unexpected format - first 200 chars:");
                    println!("{}", &body.chars().take(200).collect::<String>());
                }
            }
        }
        Err(e) => {
            println!("❌ HTTP request failed: {}", e);
        }
    }
    println!();

    // Test 2: Try NSE specific endpoint
    println!("🔄 Testing NSE-specific endpoint...");
    let nse_url = "https://api.kite.trade/instruments/NSE";
    
    match client.get(nse_url).headers(headers.clone()).send().await {
        Ok(response) => {
            println!("✅ NSE Response Status: {}", response.status());
            let body = response.text().await?;
            let lines_count = body.lines().count();
            println!("📊 NSE instruments lines: {}", lines_count);
        }
        Err(e) => {
            println!("❌ NSE request failed: {}", e);
        }
    }
    println!();

    // Test 3: Verify authentication with profile endpoint
    println!("🔐 Testing authentication with /user/profile...");
    let profile_url = "https://api.kite.trade/user/profile";
    
    match client.get(profile_url).headers(headers).send().await {
        Ok(response) => {
            println!("✅ Profile Response Status: {}", response.status());
            if response.status().is_success() {
                let profile_body = response.text().await?;
                println!("✅ Authentication working - profile data length: {} bytes", profile_body.len());
            } else {
                let error_body = response.text().await?;
                println!("❌ Authentication issue: {}", error_body);
            }
        }
        Err(e) => {
            println!("❌ Profile request failed: {}", e);
        }
    }

    println!("\n💡 Analysis:");
    println!("   - If profile works but instruments doesn't, check API permissions");
    println!("   - If instruments return empty CSV, contact Zerodha support");
    println!("   - If CSV parsing fails, there might be format changes in the API");

    Ok(())
}
