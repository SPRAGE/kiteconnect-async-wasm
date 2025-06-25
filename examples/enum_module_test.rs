use kiteconnect_async_wasm::models::common::*;

fn main() {
    // Test that all enums are accessible and work correctly
    
    // Exchange enum
    let exchange = Exchange::NSE;
    println!("Exchange: {} (is_equity: {})", exchange, exchange.is_equity());
    
    // Trading enums
    let product = Product::CNC;
    let validity = Validity::DAY;
    let transaction_type = TransactionType::BUY;
    let order_type = OrderType::MARKET;
    let variety = Variety::Regular;
    
    println!("Trading - Product: {}, Validity: {}, Type: {}, Order: {}, Variety: {}", 
             product, validity, transaction_type, order_type, variety);
    
    // Instrument enums
    let instrument_type = InstrumentType::EQ;
    let segment = Segment::NSE;
    
    println!("Instruments - Type: {}, Segment: {}", instrument_type, segment);
    
    // Interval enum with dual serde support
    let interval = Interval::FiveMinute;
    println!("Interval: {} (as_i8: {})", interval, interval.as_i8());
    
    // GTT enum
    let gtt_status = GttStatus::Active;
    println!("GTT Status: {}", gtt_status);
    
    // Test serialization
    let json = serde_json::to_string(&interval).unwrap();
    println!("Interval serialized: {}", json);
    
    // Test deserialization from both formats
    let from_string: Interval = serde_json::from_str(r#""5minute""#).unwrap();
    let from_int: Interval = serde_json::from_str("3").unwrap();
    
    println!("Deserialized from string: {:?}, from int: {:?}", from_string, from_int);
    
    println!("✅ All enum modules are working correctly!");
}
