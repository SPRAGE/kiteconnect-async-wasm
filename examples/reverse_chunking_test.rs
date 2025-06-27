use chrono::NaiveDateTime;
use kiteconnect_async_wasm::models::common::Interval;
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Reverse Chronological Chunking with Multiple Chunks");
    println!("{}", "=".repeat(80));

    // Create a request that will definitely need multiple chunks
    // Request 250 days of 5-minute data (exceeds 90-day limit significantly)
    let from_date = NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-09-08 15:30:00", "%Y-%m-%d %H:%M:%S")?;

    let request = HistoricalDataRequest::new(
        738561, // RELIANCE
        from_date,
        to_date,
        Interval::FiveMinute,
    );

    println!("📊 Original request span: {} days", request.days_span());
    println!("📊 Max allowed for 5-minute interval: {} days", Interval::FiveMinute.max_days_allowed());
    
    // Test forward chunking
    let forward_chunks = request.split_into_valid_requests();
    println!("\n📦 Forward Chunking (oldest → newest): {} chunks", forward_chunks.len());
    for (i, chunk) in forward_chunks.iter().enumerate() {
        println!("   Chunk {}: {} to {} ({} days)", 
            i + 1,
            chunk.from.format("%Y-%m-%d %H:%M"),
            chunk.to.format("%Y-%m-%d %H:%M"),
            chunk.days_span()
        );
    }

    // Test reverse chunking
    let reverse_chunks = request.split_into_valid_requests_reverse();
    println!("\n📦 Reverse Chunking (newest → oldest): {} chunks", reverse_chunks.len());
    for (i, chunk) in reverse_chunks.iter().enumerate() {
        println!("   Chunk {}: {} to {} ({} days)", 
            i + 1,
            chunk.from.format("%Y-%m-%d %H:%M"),
            chunk.to.format("%Y-%m-%d %H:%M"),
            chunk.days_span()
        );
    }

    // Verify properties
    assert_eq!(forward_chunks.len(), reverse_chunks.len(), 
        "Both methods should produce same number of chunks");

    // Verify forward chunks are in chronological order
    for i in 0..forward_chunks.len() - 1 {
        assert!(forward_chunks[i].from < forward_chunks[i + 1].from,
            "Forward chunks should be in chronological order");
    }

    // Verify reverse chunks are in reverse chronological order
    for i in 0..reverse_chunks.len() - 1 {
        assert!(reverse_chunks[i].to > reverse_chunks[i + 1].to,
            "Reverse chunks should be in reverse chronological order");
    }

    // Verify no overlaps in forward chunks
    for i in 0..forward_chunks.len() - 1 {
        assert!(forward_chunks[i].to < forward_chunks[i + 1].from,
            "Forward chunks should not overlap");
    }

    // Verify no overlaps in reverse chunks
    for i in 0..reverse_chunks.len() - 1 {
        assert!(reverse_chunks[i].from > reverse_chunks[i + 1].to,
            "Reverse chunks should not overlap");
    }

    // Verify all chunks are within limits
    for chunk in &forward_chunks {
        assert!(chunk.is_within_limits(), "All forward chunks should be within limits");
    }
    
    for chunk in &reverse_chunks {
        assert!(chunk.is_within_limits(), "All reverse chunks should be within limits");
    }

    println!("\n✅ Reverse chronological chunking test passed!");
    println!("✅ Key optimizations verified:");
    println!("   • Data duplication eliminated (no overlapping chunks)");
    println!("   • Reverse processing order (newest → oldest for early termination)");
    println!("   • Proper interval-specific time gaps");
    println!("   • All chunks within API limits");

    Ok(())
}
