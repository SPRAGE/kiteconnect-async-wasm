/*!
Test for the Candle custom deserializer to handle different API response formats.

This test verifies that the Candle struct can deserialize both:
1. Array format: [date, open, high, low, close, volume] or [date, open, high, low, close, volume, oi]
2. Object format: {"date": "...", "open": ..., ...} (for backwards compatibility)
*/

use kiteconnect_async_wasm::models::market_data::Candle;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Candle Deserialization ===\n");

    // Test 1: Array format without OI (6 elements)
    println!("Test 1: Array format without OI (6 elements)");
    let array_without_oi = json!([
        "2024-12-20T09:15:00Z",
        100.5, // open
        102.0, // high
        99.8,  // low
        101.2, // close
        1500   // volume
    ]);

    let candle1: Candle = serde_json::from_value(array_without_oi)?;
    println!("✅ Successfully parsed array without OI");
    println!("   Date: {}", candle1.date.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "   OHLC: {}/{}/{}/{}",
        candle1.open, candle1.high, candle1.low, candle1.close
    );
    println!("   Volume: {}", candle1.volume);
    println!("   OI: {:?}", candle1.oi);
    println!();

    // Test 2: Array format with OI (7 elements)
    println!("Test 2: Array format with OI (7 elements)");
    let array_with_oi = json!([
        "2024-12-20T09:20:00Z",
        101.2, // open
        103.5, // high
        100.9, // low
        102.8, // close
        2500,  // volume
        12500  // oi
    ]);

    let candle2: Candle = serde_json::from_value(array_with_oi)?;
    println!("✅ Successfully parsed array with OI");
    println!("   Date: {}", candle2.date.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "   OHLC: {}/{}/{}/{}",
        candle2.open, candle2.high, candle2.low, candle2.close
    );
    println!("   Volume: {}", candle2.volume);
    println!("   OI: {:?}", candle2.oi);
    println!();

    // Test 3: Array format with timestamp (Unix timestamp)
    println!("Test 3: Array format with Unix timestamp");
    let array_with_timestamp = json!([
        1703059200, // Unix timestamp for 2023-12-20 12:00:00 UTC
        98.5,       // open
        99.8,       // high
        97.2,       // low
        98.9,       // close
        1800        // volume
    ]);

    let candle3: Candle = serde_json::from_value(array_with_timestamp)?;
    println!("✅ Successfully parsed array with Unix timestamp");
    println!("   Date: {}", candle3.date.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "   OHLC: {}/{}/{}/{}",
        candle3.open, candle3.high, candle3.low, candle3.close
    );
    println!("   Volume: {}", candle3.volume);
    println!("   OI: {:?}", candle3.oi);
    println!();

    // Test 4: Object format (for backwards compatibility)
    println!("Test 4: Object format (backwards compatibility)");
    let object_format = json!({
        "date": "2024-12-20T09:25:00Z",
        "open": 102.8,
        "high": 104.1,
        "low": 101.5,
        "close": 103.2,
        "volume": 3200,
        "oi": 15000
    });

    let candle4: Candle = serde_json::from_value(object_format)?;
    println!("✅ Successfully parsed object format");
    println!("   Date: {}", candle4.date.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "   OHLC: {}/{}/{}/{}",
        candle4.open, candle4.high, candle4.low, candle4.close
    );
    println!("   Volume: {}", candle4.volume);
    println!("   OI: {:?}", candle4.oi);
    println!();

    // Test 5: Test serialization (should always serialize to object format)
    println!("Test 5: Serialization test");
    let serialized = serde_json::to_string_pretty(&candle1)?;
    println!("✅ Serialized candle:");
    println!("{}", serialized);
    println!();

    // Test error handling
    println!("Test 6: Error handling");

    // Test with too few elements
    let invalid_array = json!([
        "2024-12-20T09:30:00Z",
        100.0,
        101.0,
        99.0 // Missing close and volume
    ]);

    match serde_json::from_value::<Candle>(invalid_array) {
        Ok(_) => println!("❌ Should have failed with too few elements"),
        Err(e) => println!("✅ Correctly failed with error: {}", e),
    }

    println!("\n=== All Tests Completed Successfully! ===");
    Ok(())
}
