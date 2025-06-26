/*!
Example demonstrating the usage of historical_data_typed with HistoricalDataRequest.

This example shows how to use the strongly typed historical data API
with the new HistoricalDataRequest struct for better type safety and cleaner code.
*/

use chrono::NaiveDateTime;
use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::common::Interval;
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize KiteConnect client
    let _client = KiteConnect::new("your_api_key", "your_access_token");

    println!("=== Historical Data Typed API Example ===\n");

    // Example 1: Basic daily data request
    let daily_request = HistoricalDataRequest::new(
        738561, // RELIANCE instrument token
        NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::Day,
    );

    println!("Example 1: Daily data request");
    println!(
        "  Instrument: {} (RELIANCE)",
        daily_request.instrument_token
    );
    println!("  From: {}", daily_request.from.format("%Y-%m-%d %H:%M:%S"));
    println!("  To: {}", daily_request.to.format("%Y-%m-%d %H:%M:%S"));
    println!("  Interval: {}", daily_request.interval);
    println!("  Continuous: {:?}", daily_request.continuous);
    println!("  OI: {:?}\n", daily_request.oi);

    // Example 2: Intraday data with continuous flag
    let intraday_request = HistoricalDataRequest::new(
        738561,
        NaiveDateTime::parse_from_str("2023-11-20 09:00:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2023-11-20 16:00:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::FiveMinute,
    )
    .continuous(true) // Include pre-market and post-market data
    .with_oi(false); // No open interest for equity

    println!("Example 2: Intraday 5-minute data request");
    println!(
        "  Instrument: {} (RELIANCE)",
        intraday_request.instrument_token
    );
    println!(
        "  From: {}",
        intraday_request.from.format("%Y-%m-%d %H:%M:%S")
    );
    println!("  To: {}", intraday_request.to.format("%Y-%m-%d %H:%M:%S"));
    println!("  Interval: {}", intraday_request.interval);
    println!("  Continuous: {:?}", intraday_request.continuous);
    println!("  OI: {:?}\n", intraday_request.oi);

    // Example 3: Futures data with open interest
    let futures_request = HistoricalDataRequest::new(
        11536642, // NIFTY futures token (example)
        NaiveDateTime::parse_from_str("2023-11-15 09:15:00", "%Y-%m-%d %H:%M:%S")?,
        NaiveDateTime::parse_from_str("2023-11-15 15:30:00", "%Y-%m-%d %H:%M:%S")?,
        Interval::FifteenMinute,
    )
    .continuous(false)
    .with_oi(true); // Include open interest for derivatives

    println!("Example 3: Futures 15-minute data with OI");
    println!(
        "  Instrument: {} (NIFTY Future)",
        futures_request.instrument_token
    );
    println!(
        "  From: {}",
        futures_request.from.format("%Y-%m-%d %H:%M:%S")
    );
    println!("  To: {}", futures_request.to.format("%Y-%m-%d %H:%M:%S"));
    println!("  Interval: {}", futures_request.interval);
    println!("  Continuous: {:?}", futures_request.continuous);
    println!("  OI: {:?}\n", futures_request.oi);

    // Example 4: Demonstrate all available intervals
    println!("Example 4: Available intervals with their representations");
    let intervals = Interval::all();
    for interval in intervals {
        println!(
            "  {:?} -> String: '{}', Integer: {}",
            interval,
            interval,
            interval.as_i8()
        );
    }
    println!();

    // Example 5: Demonstrate interval dual serialization
    println!("Example 5: Interval dual serialization/deserialization");

    // Serialize interval to string
    let interval_json = serde_json::to_string(&Interval::ThirtyMinute)?;
    println!("  ThirtyMinute serialized: {}", interval_json);

    // Deserialize from string
    let from_string: Interval = serde_json::from_str(r#""30minute""#)?;
    println!("  From string '30minute': {:?}", from_string);

    // Deserialize from integer
    let from_int: Interval = serde_json::from_str("6")?;
    println!("  From integer 6: {:?}", from_int);

    println!("\n=== Note ===");
    println!("To actually fetch data, you would call:");
    println!("let historical_data = client.historical_data_typed(request).await?;");
    println!("\nThis example only demonstrates the request creation.");
    println!("For actual API calls, ensure you have valid API credentials.");

    // Uncomment the following lines to actually make API calls
    // (requires valid API credentials)
    /*
    match client.historical_data_typed(daily_request).await {
        Ok(historical_data) => {
            println!("\n=== Historical Data Response ===");
            println!("Candles received: {}", historical_data.candles.len());

            for (i, candle) in historical_data.candles.iter().take(5).enumerate() {
                println!("Candle {}: Date: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
                    i + 1, candle.date, candle.open, candle.high, candle.low, candle.close, candle.volume);
                if let Some(oi) = candle.oi {
                    println!("  Open Interest: {}", oi);
                }
            }

            if historical_data.candles.len() > 5 {
                println!("... and {} more candles", historical_data.candles.len() - 5);
            }
        }
        Err(e) => {
            println!("Error fetching historical data: {:?}", e);
        }
    }
    */

    Ok(())
}
