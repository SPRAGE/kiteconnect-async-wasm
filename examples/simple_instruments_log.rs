//! # Simple Instruments Logging
//!
//! A basic example showing how to fetch and log instruments using the typed API.
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
//! cargo run --example simple_instruments_log --features=native
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

    println!("Fetching instruments...");

    // Get all instruments using the typed API
    let instruments = client.instruments_typed(None).await?;

    println!("Total instruments: {}", instruments.len());
    println!("\nFirst 10 instruments:");

    for (i, instrument) in instruments.iter().take(10).enumerate() {
        println!(
            "{}. {} | {} | {:?} | {:?} | ₹{}",
            i + 1,
            instrument.trading_symbol,
            instrument.name,
            instrument.instrument_type,
            instrument.exchange,
            instrument.last_price
        );
    }

    // Get NSE instruments specifically
    println!("\nFetching NSE instruments...");
    let nse_instruments = client.instruments_typed(Some(Exchange::NSE)).await?;

    println!("NSE instruments: {}", nse_instruments.len());
    println!("\nFirst 5 NSE instruments:");

    for (i, instrument) in nse_instruments.iter().take(5).enumerate() {
        println!(
            "{}. {} | {} | Lot Size: {}",
            i + 1,
            instrument.trading_symbol,
            instrument.name,
            instrument.lot_size
        );
    }

    // Filter and show equity instruments
    let equities: Vec<_> = instruments.iter().filter(|i| i.is_equity()).collect();
    println!("\nTotal equity instruments: {}", equities.len());

    // Filter and show options
    let options: Vec<_> = instruments.iter().filter(|i| i.is_option()).collect();
    println!("Total options instruments: {}", options.len());

    // Filter and show futures
    let futures: Vec<_> = instruments.iter().filter(|i| i.is_future()).collect();
    println!("Total futures instruments: {}", futures.len());

    Ok(())
}
