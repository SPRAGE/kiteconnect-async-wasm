/*!
# Type-Safe Exchange Usage Example

This example demonstrates the improved type-safe Exchange enum usage
with the `instruments_typed()` method in the KiteConnect async WASM client.

## Features Demonstrated

- **Type Safety**: Using `Exchange` enum instead of string literals
- **IDE Support**: Autocomplete for available exchanges
- **Compile-Time Validation**: Prevention of typos in exchange names
- **Exchange Categorization**: Using helper methods to filter by exchange type

## Usage

```bash
cargo run --example exchange_typed_example
```
*/

use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::common::Exchange;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client (you would use real credentials)
    let client = KiteConnect::new("your_api_key", "your_access_token");

    println!("🔍 Demonstrating Type-Safe Exchange Usage\n");

    // Example 1: Get all instruments (no exchange filter)
    println!("📊 Getting all instruments...");
    match client.instruments_typed(None).await {
        Ok(all_instruments) => {
            println!(
                "✅ Total instruments available: {}\n",
                all_instruments.len()
            );
        }
        Err(e) => println!("❌ Error getting all instruments: {:?}\n", e),
    }

    // Example 2: Get NSE instruments using type-safe enum
    println!("🏢 Getting NSE instruments using Exchange::NSE...");
    match client.instruments_typed(Some(Exchange::NSE)).await {
        Ok(nse_instruments) => {
            println!("✅ NSE instruments: {}", nse_instruments.len());

            // Show some examples
            let equity_count = nse_instruments.iter().filter(|i| i.is_equity()).count();
            println!("  └─ Equity instruments: {}", equity_count);
        }
        Err(e) => println!("❌ Error getting NSE instruments: {:?}", e),
    }

    // Example 3: Get derivatives instruments using type-safe enum
    println!("\n📈 Getting NFO (derivatives) instruments using Exchange::NFO...");
    match client.instruments_typed(Some(Exchange::NFO)).await {
        Ok(nfo_instruments) => {
            println!("✅ NFO derivatives: {}", nfo_instruments.len());

            // Analyze derivatives
            let futures_count = nfo_instruments.iter().filter(|i| i.is_future()).count();
            let options_count = nfo_instruments.iter().filter(|i| i.is_option()).count();

            println!("  ├─ Futures: {}", futures_count);
            println!("  └─ Options: {}", options_count);
        }
        Err(e) => println!("❌ Error getting NFO instruments: {:?}", e),
    }

    // Example 4: Get commodity instruments
    println!("\n🌾 Getting MCX (commodity) instruments using Exchange::MCX...");
    match client.instruments_typed(Some(Exchange::MCX)).await {
        Ok(mcx_instruments) => {
            println!("✅ MCX commodities: {}", mcx_instruments.len());

            // Show some commodity examples
            for commodity in mcx_instruments.iter().take(3) {
                println!(
                    "  └─ {}: {} (Token: {})",
                    commodity.trading_symbol, commodity.name, commodity.instrument_token
                );
            }
        }
        Err(e) => println!("❌ Error getting MCX instruments: {:?}", e),
    }

    // Example 5: Demonstrate exchange categorization
    println!("\n🏷️  Available Exchange Categories:");
    let all_exchanges = Exchange::all();

    for exchange in all_exchanges {
        let category = if exchange.is_equity() {
            "Equity"
        } else if exchange.is_derivative() {
            "Derivatives"
        } else if exchange.is_commodity() {
            "Commodity"
        } else if exchange.is_global() {
            "Global/International"
        } else {
            "Other"
        };

        println!("  ├─ {}: {}", exchange, category);
    }

    println!("\n✨ Type-safe Exchange usage provides:");
    println!("  ├─ Compile-time validation");
    println!("  ├─ IDE autocomplete support");
    println!("  ├─ Prevention of typos");
    println!("  └─ Better integration with typed APIs");

    Ok(())
}
