//! # Simple Typed Instruments Logging Example
//!
//! This example demonstrates how to fetch and log instruments using the typed instruments API.
//! It shows basic usage of the typed API with clean logging output.
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
//! cargo run --example log_instruments_example --features=native
//! ```

use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::common::Exchange;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    // Create KiteConnect client
    let client = KiteConnect::new(&api_key, &access_token);

    println!("📊 Instruments Logging Example");
    println!("==============================\n");

    // Example 1: Get all instruments
    println!("🔄 Fetching all instruments...");
    let all_instruments = client.instruments_typed(None).await?;
    println!("✅ Fetched {} total instruments\n", all_instruments.len());

    // Log first 10 instruments
    println!("📝 First 10 instruments:");
    for (i, instrument) in all_instruments.iter().take(10).enumerate() {
        println!(
            "  {}. {} | Token: {} | Type: {:?} | Exchange: {:?} | Last Price: ₹{}",
            i + 1,
            instrument.trading_symbol,
            instrument.instrument_token,
            instrument.instrument_type,
            instrument.exchange,
            instrument.last_price
        );
    }
    println!();

    // Example 2: Get NSE-specific instruments
    println!("🔄 Fetching NSE instruments...");
    let nse_instruments = client.instruments_typed(Some(Exchange::NSE)).await?;
    println!("✅ Fetched {} NSE instruments\n", nse_instruments.len());

    // Log first 5 NSE instruments
    println!("📝 First 5 NSE instruments:");
    for (i, instrument) in nse_instruments.iter().take(5).enumerate() {
        println!(
            "  {}. {} | {} | Type: {:?} | Lot Size: {} | Tick Size: ₹{}",
            i + 1,
            instrument.trading_symbol,
            instrument.name,
            instrument.instrument_type,
            instrument.lot_size,
            instrument.tick_size
        );
    }
    println!();

    // Example 3: Filter and log equity instruments
    println!("🔄 Filtering equity instruments...");
    let equity_instruments: Vec<_> = all_instruments
        .iter()
        .filter(|inst| inst.is_equity())
        .collect();

    println!("✅ Found {} equity instruments\n", equity_instruments.len());

    // Log first 5 equity instruments
    println!("📝 First 5 equity instruments:");
    for (i, instrument) in equity_instruments.iter().take(5).enumerate() {
        println!(
            "  {}. {} | {} | Exchange: {:?} | Last Price: ₹{}",
            i + 1,
            instrument.trading_symbol,
            instrument.name,
            instrument.exchange,
            instrument.last_price
        );
    }
    println!();

    // Example 4: Filter and log options
    println!("🔄 Filtering options instruments...");
    let options_instruments: Vec<_> = all_instruments
        .iter()
        .filter(|inst| inst.is_option())
        .collect();

    println!(
        "✅ Found {} options instruments\n",
        options_instruments.len()
    );

    if !options_instruments.is_empty() {
        println!("📝 First 5 options instruments:");
        for (i, instrument) in options_instruments.iter().take(5).enumerate() {
            let days_to_expiry = instrument
                .days_to_expiry()
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            println!(
                "  {}. {} | Strike: ₹{} | Type: {:?} | Days to expiry: {}",
                i + 1,
                instrument.trading_symbol,
                instrument.strike,
                instrument.instrument_type,
                days_to_expiry
            );
        }
        println!();
    }

    // Example 5: Find specific instruments (RELIANCE)
    println!("🔍 Searching for RELIANCE instruments...");
    let reliance_instruments: Vec<_> = all_instruments
        .iter()
        .filter(|inst| inst.name.contains("RELIANCE"))
        .collect();

    println!(
        "✅ Found {} RELIANCE instruments\n",
        reliance_instruments.len()
    );

    if !reliance_instruments.is_empty() {
        println!("📝 RELIANCE instruments:");
        for (i, instrument) in reliance_instruments.iter().enumerate() {
            println!(
                "  {}. {} | Type: {:?} | Exchange: {:?} | Last Price: ₹{}",
                i + 1,
                instrument.trading_symbol,
                instrument.instrument_type,
                instrument.exchange,
                instrument.last_price
            );

            if instrument.is_option() {
                if let Some(days) = instrument.days_to_expiry() {
                    println!(
                        "     Strike: ₹{} | Days to expiry: {}",
                        instrument.strike, days
                    );
                }
            }
        }
        println!();
    }

    // Example 6: Get mutual fund instruments
    println!("🔄 Fetching mutual fund instruments...");
    let mf_instruments = client.mf_instruments_typed().await?;
    println!(
        "✅ Fetched {} mutual fund instruments\n",
        mf_instruments.len()
    );

    // Log first 5 MF instruments
    println!("📝 First 5 mutual fund instruments:");
    for (i, instrument) in mf_instruments.iter().take(5).enumerate() {
        println!(
            "  {}. {} | AMC: {} | Min Purchase: ₹{} | NAV: ₹{}",
            i + 1,
            instrument.name,
            instrument.amc,
            instrument.minimum_purchase_amount,
            instrument.last_price
        );
    }

    println!("\n✨ Instruments logging complete!");
    println!("   💡 Tip: Use the typed API for better type safety and helper methods");

    Ok(())
}
