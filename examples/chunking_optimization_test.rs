use chrono::NaiveDateTime;
use kiteconnect_async_wasm::models::common::Interval;
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing KiteConnect Historical Data Chunking Optimizations");
    println!("{}", "=".repeat(70));

    // Test 1: Data Duplication Fix - Verify no overlapping chunks
    test_data_duplication_fix()?;
    
    // Test 2: Reverse Chronological Splitting
    test_reverse_chronological_splitting()?;
    
    // Test 3: Date Range Validation
    test_date_range_validation()?;
    
    // Test 4: Different Intervals with Proper Time Increments
    test_interval_time_increments()?;
    
    println!("\n✅ All chunking optimization tests passed!");
    Ok(())
}

fn test_data_duplication_fix() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Test 1: Data Duplication Fix");
    println!("{}", "-".repeat(40));

    // Create a request that needs chunking (200 days for 5-minute data, which exceeds 90-day limit)
    let from_date = NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-07-20 15:30:00", "%Y-%m-%d %H:%M:%S")?;

    let request = HistoricalDataRequest::new(
        738561, // RELIANCE
        from_date,
        to_date,
        Interval::FiveMinute,
    );

    let chunks = request.split_into_valid_requests();
    
    println!("📊 Original request: {} to {}", 
        request.from.format("%Y-%m-%d %H:%M:%S"), 
        request.to.format("%Y-%m-%d %H:%M:%S")
    );
    println!("📦 Split into {} chunks", chunks.len());

    // Verify no overlapping chunks
    for i in 0..chunks.len() - 1 {
        let current_chunk = &chunks[i];
        let next_chunk = &chunks[i + 1];
        
        println!("   Chunk {}: {} to {}", 
            i + 1,
            current_chunk.from.format("%Y-%m-%d %H:%M:%S"),
            current_chunk.to.format("%Y-%m-%d %H:%M:%S")
        );
        
        // Verify current chunk's end is before next chunk's start (no overlap)
        assert!(current_chunk.to < next_chunk.from, 
            "Chunk {} overlaps with chunk {}: {} >= {}", 
            i + 1, i + 2, 
            current_chunk.to.format("%Y-%m-%d %H:%M:%S"),
            next_chunk.from.format("%Y-%m-%d %H:%M:%S")
        );
    }
    
    // Print the last chunk
    if let Some(last_chunk) = chunks.last() {
        println!("   Chunk {}: {} to {}", 
            chunks.len(),
            last_chunk.from.format("%Y-%m-%d %H:%M:%S"),
            last_chunk.to.format("%Y-%m-%d %H:%M:%S")
        );
    }

    println!("✅ No overlapping chunks found - data duplication fixed!");
    Ok(())
}

fn test_reverse_chronological_splitting() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Test 2: Reverse Chronological Splitting");
    println!("{}", "-".repeat(40));

    let from_date = NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-07-20 15:30:00", "%Y-%m-%d %H:%M:%S")?;

    let request = HistoricalDataRequest::new(
        738561,
        from_date,
        to_date,
        Interval::Day,
    );

    let forward_chunks = request.split_into_valid_requests();
    let reverse_chunks = request.split_into_valid_requests_reverse();

    println!("📊 Forward chunks: {}", forward_chunks.len());
    println!("📊 Reverse chunks: {}", reverse_chunks.len());

    // Both should have the same number of chunks
    assert_eq!(forward_chunks.len(), reverse_chunks.len(), 
        "Forward and reverse chunking should produce same number of chunks");

    println!("📦 Forward chunking order (oldest → newest):");
    for (i, chunk) in forward_chunks.iter().enumerate() {
        println!("   Chunk {}: {} to {}", 
            i + 1,
            chunk.from.format("%Y-%m-%d"),
            chunk.to.format("%Y-%m-%d")
        );
    }

    println!("📦 Reverse chunking order (newest → oldest):");
    for (i, chunk) in reverse_chunks.iter().enumerate() {
        println!("   Chunk {}: {} to {}", 
            i + 1,
            chunk.from.format("%Y-%m-%d"),
            chunk.to.format("%Y-%m-%d")
        );
    }

    // Verify reverse chunks are in newest → oldest order
    for i in 0..reverse_chunks.len() - 1 {
        assert!(reverse_chunks[i].to > reverse_chunks[i + 1].to,
            "Reverse chunks should be in newest → oldest order");
    }

    // Verify no overlapping in reverse chunks
    for i in 0..reverse_chunks.len() - 1 {
        let current_chunk = &reverse_chunks[i];
        let next_chunk = &reverse_chunks[i + 1];
        
        assert!(current_chunk.from > next_chunk.to,
            "Reverse chunk {} overlaps with chunk {}", i + 1, i + 2);
    }

    println!("✅ Reverse chronological splitting working correctly!");
    Ok(())
}

fn test_date_range_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Test 3: Date Range Validation");
    println!("{}", "-".repeat(40));

    let from_date = NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?;

    // Valid request
    let valid_request = HistoricalDataRequest::new(
        738561,
        from_date,
        to_date,
        Interval::Day,
    );

    assert!(valid_request.validate_date_range().is_ok(), "Valid request should pass validation");
    assert!(valid_request.is_within_limits(), "Valid request should be within limits");
    println!("✅ Valid 30-day daily request passes validation");

    // Invalid request (end before start)
    let invalid_request = HistoricalDataRequest::new(
        738561,
        to_date,   // End date as start
        from_date, // Start date as end
        Interval::Day,
    );

    assert!(invalid_request.validate_date_range().is_err(), "Invalid request should fail validation");
    println!("✅ Invalid request (end before start) correctly fails validation");

    // Request that exceeds limits
    let from_date_long = NaiveDateTime::parse_from_str("2022-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date_long = NaiveDateTime::parse_from_str("2023-12-31 15:30:00", "%Y-%m-%d %H:%M:%S")?;
    
    let long_request = HistoricalDataRequest::new(
        738561,
        from_date_long,
        to_date_long,
        Interval::FiveMinute, // 5-minute data for 2 years (exceeds 90-day limit)
    );

    assert!(!long_request.is_within_limits(), "Long request should exceed limits");
    println!("✅ Long request correctly identified as exceeding limits");

    Ok(())
}

fn test_interval_time_increments() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Test 4: Interval-Specific Time Increments");
    println!("{}", "-".repeat(40));

    let intervals_and_expected_gaps = vec![
        (Interval::Minute, chrono::Duration::minutes(1)),
        (Interval::FiveMinute, chrono::Duration::minutes(5)),
        (Interval::FifteenMinute, chrono::Duration::minutes(15)),
        (Interval::ThirtyMinute, chrono::Duration::minutes(30)),
        (Interval::SixtyMinute, chrono::Duration::hours(1)),
        (Interval::Day, chrono::Duration::days(1)),
    ];

    for (interval, expected_gap) in intervals_and_expected_gaps {
        // Create a request that will be split into exactly 2 chunks
        let from_date = NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
        let to_date = from_date + chrono::Duration::days((interval.max_days_allowed() as i64) + 10);

        let request = HistoricalDataRequest::new(738561, from_date, to_date, interval);
        let chunks = request.split_into_valid_requests();

        if chunks.len() >= 2 {
            let first_chunk_end = chunks[0].to;
            let second_chunk_start = chunks[1].from;
            let actual_gap = second_chunk_start - first_chunk_end;

            println!("📊 {}: gap = {}, expected = {}", 
                interval, 
                format_duration(actual_gap),
                format_duration(expected_gap)
            );

            assert_eq!(actual_gap, expected_gap,
                "Gap between chunks for {} should be {}", interval, format_duration(expected_gap));
        }
    }

    println!("✅ All interval-specific time increments working correctly!");
    Ok(())
}

fn format_duration(duration: chrono::Duration) -> String {
    if duration.num_days() > 0 {
        format!("{} days", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes", duration.num_minutes())
    } else {
        format!("{} seconds", duration.num_seconds())
    }
}
