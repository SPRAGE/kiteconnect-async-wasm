use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
use kiteconnect_async_wasm::models::common::Interval;
use chrono::NaiveDateTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test creating a HistoricalDataRequest
    let from_date = NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?;
    
    let request = HistoricalDataRequest::new(
        738561,              // RELIANCE instrument token
        from_date,
        to_date,
        Interval::Day,
    )
    .continuous(false)       // No continuous data
    .with_oi(true);         // Include open interest
    
    println!("✅ HistoricalDataRequest created successfully!");
    println!("Instrument Token: {}", request.instrument_token);
    println!("From: {}", request.from.format("%Y-%m-%d %H:%M:%S"));
    println!("To: {}", request.to.format("%Y-%m-%d %H:%M:%S"));
    println!("Interval: {}", request.interval);
    println!("Continuous: {:?}", request.continuous);
    println!("OI: {:?}", request.oi);
    
    // Test interval serialization
    let interval_json = serde_json::to_string(&request.interval)?;
    println!("Interval as JSON: {}", interval_json);
    
    // Test that the interval can be both string and integer
    let interval_from_string: Interval = serde_json::from_str(r#""day""#)?;
    let interval_from_int: Interval = serde_json::from_str("0")?;
    
    println!("Interval from string 'day': {:?}", interval_from_string);
    println!("Interval from integer 0: {:?}", interval_from_int);
    
    println!("✅ HistoricalDataRequest test completed successfully!");
    
    Ok(())
}
