use kiteconnect_async_wasm::models::common::Interval;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test serialization (always outputs strings)
    let interval = Interval::FiveMinute;
    let serialized = serde_json::to_string(&interval)?;
    println!("Serialized FiveMinute: {}", serialized);
    
    // Test deserialization from strings
    let from_string: Interval = serde_json::from_str(r#""5minute""#)?;
    println!("Deserialized from string '5minute': {:?}", from_string);
    
    // Test deserialization from integers
    let from_int: Interval = serde_json::from_str("3")?;
    println!("Deserialized from integer 3: {:?}", from_int);
    
    // Test integer conversion methods
    println!("Day as i8: {}", Interval::Day.as_i8());
    println!("From i8(7): {:?}", Interval::from_i8(7));
    
    // Test all intervals
    println!("All intervals: {:?}", Interval::all());
    
    // Test various integer formats
    let test_cases = vec![
        ("0", "Day"),
        ("1", "Minute"), 
        ("2", "ThreeMinute"),
        ("3", "FiveMinute"),
        ("4", "TenMinute"),
        ("5", "FifteenMinute"),
        ("6", "ThirtyMinute"),
        ("7", "SixtyMinute"),
    ];
    
    println!("\nTesting integer deserialization:");
    for (json_int, expected_name) in test_cases {
        let interval: Interval = serde_json::from_str(json_int)?;
        println!("  {} -> {:?} ({})", json_int, interval, expected_name);
    }
    
    // Test string deserialization
    let string_cases = vec![
        (r#""day""#, "Day"),
        (r#""minute""#, "Minute"),
        (r#""3minute""#, "ThreeMinute"),
        (r#""5minute""#, "FiveMinute"),
        (r#""10minute""#, "TenMinute"),
        (r#""15minute""#, "FifteenMinute"),
        (r#""30minute""#, "ThirtyMinute"),
        (r#""60minute""#, "SixtyMinute"),
    ];
    
    println!("\nTesting string deserialization:");
    for (json_str, expected_name) in string_cases {
        let interval: Interval = serde_json::from_str(json_str)?;
        println!("  {} -> {:?} ({})", json_str, interval, expected_name);
    }
    
    Ok(())
}
