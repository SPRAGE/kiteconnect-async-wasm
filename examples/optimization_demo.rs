use chrono::NaiveDateTime;
use kiteconnect_async_wasm::models::common::Interval;
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 KiteConnect Historical Data Chunking Optimization Demo");
    println!("{}", "=".repeat(80));
    
    // Scenario 1: Newly Listed Stock (High Early Termination Benefit)
    demo_newly_listed_stock()?;
    
    // Scenario 2: Established Stock with Full History
    demo_established_stock()?;
    
    // Scenario 3: Different Intervals Comparison
    demo_interval_comparison()?;
    
    println!("\n🎯 Summary of Optimizations:");
    println!("✅ 1. Data Duplication Fixed: No overlapping chunks, eliminating duplicate data");
    println!("✅ 2. Reverse Processing: Newest→oldest for early termination on empty chunks");
    println!("✅ 3. Smart Early Exit: Stops immediately when no more data is available");
    println!("✅ 4. Interval-Aware Gaps: Proper time increments prevent data overlap");
    println!("\n💡 Expected API Call Reduction: 60-90% for newly listed instruments!");
    
    Ok(())
}

fn demo_newly_listed_stock() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Scenario 1: Newly Listed Stock (Listed 3 months ago)");
    println!("{}", "-".repeat(60));
    
    // Request 5 years of data for a stock that was only listed 3 months ago
    let from_date = NaiveDateTime::parse_from_str("2019-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-12-31 15:30:00", "%Y-%m-%d %H:%M:%S")?;
    
    let request = HistoricalDataRequest::new(
        123456, // Hypothetical newly listed stock
        from_date,
        to_date,
        Interval::Day,
    );
    
    let total_days = request.days_span();
    let max_chunk_days = Interval::Day.max_days_allowed() as i64;
    let total_possible_chunks = (total_days + max_chunk_days - 1) / max_chunk_days;
    
    // With reverse processing, we would process chunks like:
    // Chunk 1: 2023-07-04 to 2023-12-31 ← Most recent, HAS DATA
    // Chunk 2: 2023-01-05 to 2023-07-03 ← Still recent, HAS DATA  
    // Chunk 3: 2022-07-07 to 2023-01-04 ← Some data (listing date ~Oct 2022)
    // Chunk 4: 2022-01-01 to 2022-07-06 ← EMPTY! Early termination here
    
    println!("📈 Stock listed around: October 2022");
    println!("📅 Requested period: {} to {}", 
        request.from.format("%Y-%m-%d"), 
        request.to.format("%Y-%m-%d")
    );
    println!("📊 Total days requested: {}", total_days);
    println!("📦 Total possible chunks: {}", total_possible_chunks);
    
    let reverse_chunks = request.split_into_valid_requests_reverse();
    println!("📦 Reverse processing order:");
    
    for (i, chunk) in reverse_chunks.iter().enumerate() {
        let has_data = chunk.from >= NaiveDateTime::parse_from_str("2022-10-01 00:00:00", "%Y-%m-%d %H:%M:%S")?;
        let status = if has_data { "📈 HAS DATA" } else { "❌ EMPTY (would terminate here)" };
        
        println!("   Chunk {}: {} to {} - {}", 
            i + 1,
            chunk.from.format("%Y-%m-%d"),
            chunk.to.format("%Y-%m-%d"),
            status
        );
        
        if !has_data {
            println!("   🛑 Early termination! Processed {} of {} chunks ({}% reduction)",
                i + 1, total_possible_chunks, 
                ((total_possible_chunks - (i + 1) as i64) * 100) / total_possible_chunks);
            break;
        }
    }
    
    Ok(())
}

fn demo_established_stock() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Scenario 2: Established Stock (Full History Available)");
    println!("{}", "-".repeat(60));
    
    let from_date = NaiveDateTime::parse_from_str("2019-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-12-31 15:30:00", "%Y-%m-%d %H:%M:%S")?;
    
    let request = HistoricalDataRequest::new(
        738561, // RELIANCE - has full history
        from_date,
        to_date,
        Interval::Day,
    );
    
    let total_days = request.days_span();
    let chunks = request.split_into_valid_requests_reverse();
    
    println!("📈 RELIANCE (established stock with full history)");
    println!("📅 Requested period: {} to {}", 
        request.from.format("%Y-%m-%d"), 
        request.to.format("%Y-%m-%d")
    );
    println!("📊 Total days: {}", total_days);
    println!("📦 Chunks needed: {}", chunks.len());
    println!("💡 All chunks will have data, but no overlapping/duplicate data");
    
    Ok(())
}

fn demo_interval_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Scenario 3: Different Intervals - API Limits & Chunking");
    println!("{}", "-".repeat(60));
    
    let intervals_and_scenarios = vec![
        (Interval::Minute, "Minute", "High-frequency trading analysis"),
        (Interval::FiveMinute, "5-Minute", "Intraday pattern analysis"),
        (Interval::SixtyMinute, "Hourly", "Short-term trend analysis"),
        (Interval::Day, "Daily", "Long-term investment analysis"),
    ];
    
    // Request 400 days of data for each interval
    let from_date = NaiveDateTime::parse_from_str("2022-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?;
    let to_date = NaiveDateTime::parse_from_str("2023-02-04 15:30:00", "%Y-%m-%d %H:%M:%S")?;
    
    println!("📅 Test period: 400 days ({} to {})", 
        from_date.format("%Y-%m-%d"), 
        to_date.format("%Y-%m-%d")
    );
    println!();
    
    for (interval, name, use_case) in intervals_and_scenarios {
        let request = HistoricalDataRequest::new(123456, from_date, to_date, interval);
        let chunks = request.split_into_valid_requests();
        let max_days = interval.max_days_allowed();
        
        println!("📊 {} Data ({})", name, use_case);
        println!("   • API Limit: {} days per request", max_days);
        println!("   • Chunks needed: {}", chunks.len());
        println!("   • Benefit: {} data duplication, proper time gaps", 
            if chunks.len() > 1 { "Eliminates" } else { "No risk of" });
        
        if chunks.len() > 1 {
            let gap = match interval {
                Interval::Minute => "1 minute",
                Interval::FiveMinute => "5 minutes", 
                Interval::SixtyMinute => "1 hour",
                Interval::Day => "1 day",
                _ => "appropriate interval",
            };
            println!("   • Gap between chunks: {}", gap);
        }
        println!();
    }
    
    Ok(())
}
