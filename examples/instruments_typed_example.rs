//! # Typed Instruments API Example
//!
//! This example demonstrates the new typed instruments API that provides
//! compile-time type safety and helper methods for working with instruments data.
//!
//! ## Features Demonstrated
//!
//! - **Type Safety**: Strongly typed `Instrument` and `MFInstrument` structs
//! - **Helper Methods**: Built-in methods like `is_equity()`, `is_option()`, `days_to_expiry()`
//! - **Easy Filtering**: Type-safe filtering and analysis
//! - **Performance**: Automatic caching for better performance
//!
//! ## Usage
//!
//! Set environment variables:
//! ```bash
//! export KITE_API_KEY="your_api_key"
//! export KITE_ACCESS_TOKEN="your_access_token"
//! ```
//!
//! Run with:
//! ```bash
//! cargo run --example instruments_typed_example --features=native
//! ```

use kiteconnect_async_wasm::connect::KiteConnect;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    // Create KiteConnect client
    let client = KiteConnect::new(&api_key, &access_token);

    println!("🔧 Typed Instruments API Example");
    println!("==================================\n");

    // === Regular Instruments Analysis ===
    println!("📊 Fetching instruments with type safety...");
    let instruments = client.instruments_typed(None).await?;
    println!("✅ Total instruments available: {}\n", instruments.len());

    // Analyze instrument types
    let equity_count = instruments.iter().filter(|i| i.is_equity()).count();
    let options_count = instruments.iter().filter(|i| i.is_option()).count();
    let futures_count = instruments.iter().filter(|i| i.is_future()).count();

    println!("📈 Instrument Distribution:");
    println!("  • Equity: {}", equity_count);
    println!("  • Options: {}", options_count);
    println!("  • Futures: {}", futures_count);
    println!();

    // Find RELIANCE instruments
    println!("🔍 Finding RELIANCE instruments...");
    let reliance_instruments: Vec<_> = instruments
        .iter()
        .filter(|inst| inst.name.contains("RELIANCE"))
        .collect();

    println!("Found {} RELIANCE instruments:", reliance_instruments.len());
    for (i, instrument) in reliance_instruments.iter().take(5).enumerate() {
        println!(
            "  {}. {} | Type: {:?} | Exchange: {:?}",
            i + 1,
            instrument.trading_symbol,
            instrument.instrument_type,
            instrument.exchange
        );

        // Show additional info for options
        if instrument.is_option() {
            if let Some(expiry) = instrument.expiry {
                println!(
                    "     Strike: ₹{} | Expiry: {} | Days to expiry: {:?}",
                    instrument.strike,
                    expiry,
                    instrument.days_to_expiry()
                );
            }
        }
    }
    if reliance_instruments.len() > 5 {
        println!("     ... and {} more", reliance_instruments.len() - 5);
    }
    println!();

    // Analyze expiring options (if any)
    let expiring_soon: Vec<_> = instruments
        .iter()
        .filter(|inst| {
            inst.is_option()
                && inst
                    .days_to_expiry()
                    .map_or(false, |days| days <= 7 && days >= 0)
        })
        .collect();

    if !expiring_soon.is_empty() {
        println!("⏰ Options expiring within 7 days: {}", expiring_soon.len());
        for option in expiring_soon.iter().take(3) {
            if let Some(days) = option.days_to_expiry() {
                println!(
                    "  • {} | Strike: ₹{} | Expires in {} days",
                    option.trading_symbol, option.strike, days
                );
            }
        }
        println!();
    }

    // === Mutual Funds Analysis ===
    println!("💰 Fetching mutual fund instruments...");
    let mf_instruments = client.mf_instruments_typed().await?;
    println!("✅ Total MF instruments: {}\n", mf_instruments.len());

    // Analyze fund types
    let equity_funds_count = mf_instruments.iter().filter(|f| f.is_equity_fund()).count();
    let debt_funds_count = mf_instruments.iter().filter(|f| f.is_debt_fund()).count();
    let hybrid_funds_count = mf_instruments.iter().filter(|f| f.is_hybrid_fund()).count();
    let sip_eligible_count = mf_instruments.iter().filter(|f| f.allows_sip()).count();

    println!("📊 Mutual Fund Distribution:");
    println!("  • Equity Funds: {}", equity_funds_count);
    println!("  • Debt Funds: {}", debt_funds_count);
    println!("  • Hybrid Funds: {}", hybrid_funds_count);
    println!("  • SIP Eligible: {}", sip_eligible_count);
    println!();

    // Show top AMCs by fund count
    let mut amc_counts = std::collections::HashMap::new();
    for fund in &mf_instruments {
        *amc_counts.entry(&fund.amc).or_insert(0) += 1;
    }

    let mut sorted_amcs: Vec<_> = amc_counts.iter().collect();
    sorted_amcs.sort_by(|a, b| b.1.cmp(a.1));

    println!("🏢 Top AMCs by fund count:");
    for (i, (amc, count)) in sorted_amcs.iter().take(5).enumerate() {
        println!("  {}. {}: {} funds", i + 1, amc, count);
    }
    println!();

    // Find SIP-eligible equity funds with low minimum amounts
    let affordable_sip_funds: Vec<_> = mf_instruments
        .iter()
        .filter(|fund| {
            fund.is_equity_fund()
                && fund.allows_sip()
                && fund.minimum_additional_purchase_amount <= 1000.0
        })
        .collect();

    if !affordable_sip_funds.is_empty() {
        println!("💎 Affordable SIP Equity Funds (≤₹1000):");
        for (i, fund) in affordable_sip_funds.iter().take(5).enumerate() {
            println!("  {}. {}", i + 1, fund.name);
            println!(
                "     AMC: {} | Min SIP: ₹{:.0} | Settlement: {} days",
                fund.amc,
                fund.minimum_additional_purchase_amount,
                fund.settlement_days()
            );
        }
        println!();
    }

    // === Exchange-specific Analysis ===
    println!("🏛️ Fetching NSE-specific instruments...");
    let nse_instruments = client
        .instruments_typed(Some(kiteconnect_async_wasm::models::Exchange::NSE))
        .await?;
    println!("✅ NSE instruments: {}", nse_instruments.len());

    let nse_options: Vec<_> = nse_instruments
        .iter()
        .filter(|inst| inst.is_option())
        .collect();

    println!("📈 NSE Options available: {}", nse_options.len());

    // Show tick sizes analysis
    let unique_tick_sizes: std::collections::HashSet<_> = nse_instruments
        .iter()
        .map(|inst| (inst.tick_size * 100.0) as u32) // Convert to paise for easier handling
        .collect();

    println!("💰 Tick sizes in NSE: {:?} paise", {
        let mut sizes: Vec<_> = unique_tick_sizes.into_iter().collect();
        sizes.sort();
        sizes
    });

    println!("\n✨ Typed Instruments API Demo Complete!");
    println!("   - Type safety ensures compile-time correctness");
    println!("   - Helper methods make analysis easier");
    println!("   - Performance is optimized with caching");
    println!("   - Both regular and MF instruments are supported");

    Ok(())
}
