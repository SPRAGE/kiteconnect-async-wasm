/*!
Example demonstrating historical data retrieval for instrument token 256265.

This example shows various ways to fetch and analyze historical data for
instrument token 256265 using the KiteConnect async WASM library.
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
    // Initialize KiteConnect client with credentials from environment variables
    let (api_key, access_token) = get_credentials()?;
    #[allow(unused_variables)]
    let client = KiteConnect::new(&api_key, &access_token);

    println!("=== Historical Data Example for Instrument 256265 ===\n");

    // Example 1: Fetch daily data for the last 30 days
    println!("📊 Example 1: Daily data for the last 30 days");
    let daily_request = HistoricalDataRequest::new(
        256265, // Instrument token
        NaiveDateTime::parse_from_str("2024-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-01 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    match daily_request.validate_date_range() {
        Ok(()) => {
            println!("✅ Daily request is valid");
            println!("   Instrument: {}", daily_request.instrument_token);
            println!(
                "   From: {}",
                daily_request.from.format("%Y-%m-%d %H:%M:%S")
            );
            println!("   To: {}", daily_request.to.format("%Y-%m-%d %H:%M:%S"));
            println!("   Days span: {} days", daily_request.days_span());

            // Uncomment to actually fetch data
            // let daily_data = daily_request.fetch(&client).await?;
            // println!("   Received {} candles", daily_data.candles.len());
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }

    println!();

    // Example 2: Intraday 5-minute data for a single trading day
    println!("📈 Example 2: Intraday 5-minute data for a single day");
    let intraday_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-12-20 09:00:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-20 16:00:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::FiveMinute,
    );

    println!("✅ Intraday request details:");
    println!("   Instrument: {}", intraday_request.instrument_token);
    println!("   Date: {}", intraday_request.from.format("%Y-%m-%d"));
    println!(
        "   Time range: {} to {}",
        intraday_request.from.format("%H:%M:%S"),
        intraday_request.to.format("%H:%M:%S")
    );
    println!("   Interval: {}", intraday_request.interval);

    // Uncomment to actually fetch data
    // let intraday_data = intraday_request.fetch(&client).await?;
    // analyze_intraday_data(&intraday_data);

    println!();

    // Example 3: Large date range with automatic chunking
    println!("🔄 Example 3: Large date range with automatic chunking");
    let large_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-31 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    if !large_request.is_within_limits() {
        println!("⚠️  Large request exceeds API limits, will need chunking");
        let chunks = large_request.split_into_valid_requests();
        println!("   Will be split into {} chunks", chunks.len());

        for (i, chunk) in chunks.iter().enumerate() {
            println!(
                "   Chunk {}: {} to {}",
                i + 1,
                chunk.from.format("%Y-%m-%d"),
                chunk.to.format("%Y-%m-%d")
            );
        }

        // Uncomment to fetch with automatic chunking
        // let all_data = large_request.fetch_with_chunking(&client, false).await?;
        // println!("   Total candles received: {}", all_data.candles.len());
    } else {
        println!("✅ Request is within limits");
    }

    println!();

    // Example 4: Futures/derivatives with continuous data and OI
    println!("📊 Example 4: Derivatives data with continuous flag and OI");
    let derivatives_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    )
    .continuous(true) // Continuous contract data for futures
    .with_oi(true); // Include open interest data

    println!("✅ Derivatives request with options:");
    println!("   Continuous: {:?}", derivatives_request.continuous);
    println!("   Include OI: {:?}", derivatives_request.oi);

    // Uncomment to actually fetch data
    // let derivatives_data = derivatives_request.fetch(&client).await?;
    // analyze_derivatives_data(&derivatives_data);

    println!();

    // Example 5: Error handling and validation
    println!("🔍 Example 5: Error handling and validation");

    // Invalid date range (end before start)
    let invalid_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-12-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-11-01 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    match invalid_request.validate_date_range() {
        Ok(()) => println!("✅ Request is valid"),
        Err(e) => println!("❌ Expected validation error: {}", e),
    }

    // Too large intraday range
    let too_large_request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-01-01 09:00:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-31 16:00:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::FiveMinute, // 5-minute data for a whole year
    );

    match too_large_request.validate_date_range() {
        Ok(()) => println!("✅ Request is valid"),
        Err(e) => println!("❌ Expected validation error: {}", e),
    }

    println!();

    // Example 6: Using new_validated constructor
    println!("✨ Example 6: Using validated constructor");
    match HistoricalDataRequest::new_validated(
        256265,
        NaiveDateTime::parse_from_str("2024-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    ) {
        Ok(request) => {
            println!("✅ Validated request created successfully");
            println!("   Spans {} days", request.days_span());
        }
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    println!("\n=== Example Complete ===");
    println!("💡 To actually fetch data, uncomment the API calls and set environment variables:");
    println!("💡   export KITE_API_KEY=your_actual_api_key");
    println!("💡   export KITE_ACCESS_TOKEN=your_actual_access_token");

    Ok(())
}

/// Analyze intraday data and show basic statistics
#[allow(dead_code)]
fn analyze_intraday_data(data: &kiteconnect_async_wasm::models::market_data::HistoricalData) {
    println!("\n📊 Intraday Data Analysis:");
    println!("   Total candles: {}", data.candles.len());

    if !data.candles.is_empty() {
        let first = &data.candles[0];
        let last = &data.candles[data.candles.len() - 1];

        println!(
            "   First candle: {} - OHLC: {:.2}/{:.2}/{:.2}/{:.2}",
            first.date.format("%H:%M:%S"),
            first.open,
            first.high,
            first.low,
            first.close
        );
        println!(
            "   Last candle:  {} - OHLC: {:.2}/{:.2}/{:.2}/{:.2}",
            last.date.format("%H:%M:%S"),
            last.open,
            last.high,
            last.low,
            last.close
        );

        // Calculate price change
        let change = last.close - first.open;
        let change_pct = (change / first.open) * 100.0;
        println!("   Day change: ₹{:.2} ({:.2}%)", change, change_pct);

        // Calculate total volume
        let total_volume: u64 = data.candles.iter().map(|c| c.volume).sum();
        println!("   Total volume: {} shares", total_volume);

        // Find highest and lowest prices
        let high = data.candles.iter().map(|c| c.high).fold(0.0, f64::max);
        let low = data
            .candles
            .iter()
            .map(|c| c.low)
            .fold(f64::INFINITY, f64::min);
        println!("   Day high: ₹{:.2}, Day low: ₹{:.2}", high, low);
    }
}

/// Analyze derivatives data with open interest
#[allow(dead_code)]
fn analyze_derivatives_data(data: &kiteconnect_async_wasm::models::market_data::HistoricalData) {
    println!("\n📈 Derivatives Data Analysis:");
    println!("   Total candles: {}", data.candles.len());

    if !data.candles.is_empty() {
        let first = &data.candles[0];
        let last = &data.candles[data.candles.len() - 1];

        println!(
            "   Period: {} to {}",
            first.date.format("%Y-%m-%d"),
            last.date.format("%Y-%m-%d")
        );

        // Price analysis
        let price_change = last.close - first.close;
        let price_change_pct = (price_change / first.close) * 100.0;
        println!(
            "   Price change: ₹{:.2} ({:.2}%)",
            price_change, price_change_pct
        );

        // Open interest analysis (if available)
        if let (Some(first_oi), Some(last_oi)) = (first.oi, last.oi) {
            let oi_change = last_oi as i64 - first_oi as i64;
            let oi_change_pct = (oi_change as f64 / first_oi as f64) * 100.0;
            println!(
                "   OI change: {} contracts ({:.2}%)",
                oi_change, oi_change_pct
            );

            // Interpret price and OI combination
            match (price_change > 0.0, oi_change > 0) {
                (true, true) => println!("   📊 Bullish sentiment (rising price + rising OI)"),
                (false, true) => println!("   📉 Bearish sentiment (falling price + rising OI)"),
                (true, false) => println!("   🔄 Short covering (rising price + falling OI)"),
                (false, false) => println!("   🔄 Long unwinding (falling price + falling OI)"),
            }
        } else {
            println!("   ℹ️  Open interest data not available");
        }

        // Volume analysis
        let avg_volume: f64 =
            data.candles.iter().map(|c| c.volume as f64).sum::<f64>() / data.candles.len() as f64;
        println!("   Average daily volume: {:.0} contracts", avg_volume);
    }
}

/// Sample function to demonstrate processing historical data
#[allow(dead_code)]
async fn process_historical_data_example() -> Result<(), Box<dyn std::error::Error>> {
    let (api_key, access_token) = get_credentials()?;
    let client = KiteConnect::new(&api_key, &access_token);

    // Create a simple request
    let request = HistoricalDataRequest::new(
        256265,
        NaiveDateTime::parse_from_str("2024-12-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2024-12-20 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    // Fetch the data
    let data = request.fetch(&client).await?;

    // Process each candle
    for candle in &data.candles {
        println!(
            "Date: {}, Close: ₹{:.2}, Volume: {}",
            candle.date.format("%Y-%m-%d"),
            candle.close,
            candle.volume
        );

        // Your processing logic here
        // e.g., calculate moving averages, detect patterns, etc.
    }

    Ok(())
}
