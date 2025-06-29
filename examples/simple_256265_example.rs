/*!
Simple example for fetching historical data for instrument token 256265.

This example demonstrates the most common use cases for historical data retrieval.
*/

use chrono::NaiveDateTime;
use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::common::Interval;
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
use std::env;

/// Get API credentials from environment variables
fn get_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    let api_key =
        env::var("KITE_API_KEY").map_err(|_| "KITE_API_KEY environment variable not set")?;
    let access_token = env::var("KITE_ACCESS_TOKEN")
        .map_err(|_| "KITE_ACCESS_TOKEN environment variable not set")?;
    Ok((api_key, access_token))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Historical Data Example for 256265 ===\n");

    // Initialize client with credentials from environment variables
    let (api_key, access_token) = get_credentials()?;
    #[allow(unused_variables)]
    let client = KiteConnect::new(&api_key, &access_token);

    // Example 1: Get last 30 days of daily data
    println!("📊 Daily Data (Last 30 Days)");
    let daily_request = HistoricalDataRequest::new(
        256265, // Your instrument token
        NaiveDateTime::parse_from_str("2024-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-01 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    println!("Request details:");
    println!("  Instrument: {}", daily_request.instrument_token);
    println!("  From: {}", daily_request.from.format("%Y-%m-%d"));
    println!("  To: {}", daily_request.to.format("%Y-%m-%d"));
    println!("  Interval: {}", daily_request.interval);

    // Validate the request
    match daily_request.validate_date_range() {
        Ok(()) => println!("✅ Request is valid!"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // To actually fetch data, uncomment this line:
    // let data = daily_request.fetch(&client).await?;
    // println!("Received {} candles", data.candles.len());

    println!();

    // Example 2: Get intraday 5-minute data for today
    println!("📈 Intraday 5-Minute Data");
    let intraday_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-12-20 09:00:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-20 16:00:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::FiveMinute,
    );

    println!("Request details:");
    println!("  Date: {}", intraday_request.from.format("%Y-%m-%d"));
    println!(
        "  Time: {} to {}",
        intraday_request.from.format("%H:%M"),
        intraday_request.to.format("%H:%M")
    );

    // To actually fetch data, uncomment this line:
    let intraday_data = intraday_request.fetch(&client).await?;
    print_basic_stats(&intraday_data);

    println!();

    // Example 3: Request with options (for derivatives)
    println!("📊 Data with Options (Continuous + OI)");
    let options_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-11-15 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-11-25 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    )
    .continuous(true) // For futures: continuous contract data
    .with_oi(true); // Include open interest

    println!("Request with options:");
    println!("  Continuous: {:?}", options_request.continuous);
    println!("  Include OI: {:?}", options_request.oi);

    println!("\n=== Ready to Use! ===");
    println!("💡 Set environment variables before running:");
    println!("💡   export KITE_API_KEY=your_actual_api_key");
    println!("💡   export KITE_ACCESS_TOKEN=your_actual_access_token");
    println!("💡 Uncomment the fetch lines to actually get data");

    Ok(())
}

// Helper function to show basic statistics
#[allow(dead_code)]
fn print_basic_stats(data: &kiteconnect_async_wasm::models::market_data::HistoricalData) {
    println!("\n📊 Data Summary:");
    println!("  Total candles: {}", data.candles.len());

    if let (Some(first), Some(last)) = (data.candles.first(), data.candles.last()) {
        println!(
            "  First: {} - ₹{:.2}",
            first.date.format("%H:%M"),
            first.close
        );
        println!(
            "  Last:  {} - ₹{:.2}",
            last.date.format("%H:%M"),
            last.close
        );

        let change = last.close - first.open;
        let change_pct = (change / first.open) * 100.0;
        println!("  Change: ₹{:.2} ({:.2}%)", change, change_pct);
    }
}
