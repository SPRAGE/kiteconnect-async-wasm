//! # Dual API Support Example
//!
//! This example demonstrates the dual API support (legacy + typed),
//! including typed methods, retry logic, and enhanced error handling.

use kiteconnect_async_wasm::connect::{KiteConnect, KiteConnectConfig, RetryConfig};
use std::env;
use std::time::Duration;

/// Get API credentials from environment variables
fn get_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    let api_key =
        env::var("KITE_API_KEY").map_err(|_| "KITE_API_KEY environment variable not set")?;
    let api_secret =
        env::var("KITE_API_SECRET").map_err(|_| "KITE_API_SECRET environment variable not set")?;
    Ok((api_key, api_secret))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KiteConnect v1.0.2 Dual API Example ===\n");

    // Get credentials from environment variables
    let (api_key, _api_secret) = get_credentials()?;

    // Example 1: Basic client with default configuration
    println!("1. Creating basic KiteConnect client...");
    let mut basic_client = KiteConnect::new(&api_key, "");
    println!("   ✓ Basic client created\n");

    // Example 2: Advanced client with custom configuration
    println!("2. Creating advanced client with custom retry configuration...");
    let custom_config = KiteConnectConfig {
        retry_config: RetryConfig {
            max_retries: 5,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            exponential_backoff: true,
        },
        timeout: 60, // 60 seconds timeout
        ..Default::default()
    };

    let advanced_client = KiteConnect::new_with_config(&api_key, custom_config);
    println!("   ✓ Advanced client created with custom configuration\n");

    // Example 3: Authentication flow (placeholder)
    println!("3. Authentication flow...");
    let login_url = basic_client.login_url();
    println!("   Login URL: {}", login_url);
    println!("   (In a real application, user would visit this URL)\n");

    // Set a dummy access token for demonstration
    basic_client.set_access_token("dummy_access_token");
    println!("   ✓ Access token set\n");

    // Example 4: Demonstrate legacy vs typed API methods
    println!("4. Dual API Support Examples:\n");

    // Note: These methods would normally make actual API calls
    // For demonstration, we're just showing the method signatures

    println!("   Legacy API methods (returning JsonValue):");
    println!("   - client.holdings().await?              -> Result<JsonValue>");
    println!("   - client.positions().await?             -> Result<JsonValue>");
    println!("   - client.margins(None).await?           -> Result<JsonValue>");

    println!("\n   New Typed API methods (returning structured data):");
    println!("   - client.holdings_typed().await?        -> KiteResult<Vec<Holding>>");
    println!("   - client.positions_typed().await?       -> KiteResult<Vec<Position>>");
    println!("   - client.margins_typed(None).await?     -> KiteResult<MarginData>");

    // Example 5: Show request counting feature
    println!("\n5. Request monitoring:");
    let initial_count = advanced_client.request_count();
    println!("   Initial request count: {}", initial_count);

    // Example 6: GTT Operations
    println!("\n6. GTT (Good Till Triggered) Operations:");
    println!("   - client.get_gtts(None).await?          -> Result<JsonValue>");
    println!("   - client.place_gtt(...).await?          -> Result<JsonValue>");
    println!("   - client.modify_gtt(...).await?         -> Result<JsonValue>");
    println!("   - client.delete_gtt(\"123\").await?       -> Result<JsonValue>");

    // Example 7: Configuration examples
    println!("\n7. Configuration Features:");
    println!("   ✓ Retry mechanism with exponential backoff");
    println!("   ✓ Request timeout configuration");
    println!("   ✓ Connection pooling and idle timeouts");
    println!("   ✓ Response caching for instruments data");
    println!("   ✓ Request counting for monitoring");
    println!("   ✓ Enhanced error handling with KiteError");

    println!("\n=== Key Benefits of Phase 8 ===");
    println!("✓ Dual API support: Use legacy JsonValue or new typed responses");
    println!("✓ Backward compatibility: All existing code continues to work");
    println!("✓ Enhanced reliability: Retry mechanism with configurable backoff");
    println!("✓ Better performance: Connection pooling and response caching");
    println!("✓ Improved monitoring: Request counting and structured errors");
    println!("✓ Type safety: Strongly typed models for compile-time validation");

    println!("\n=== Migration Path ===");
    println!("1. Immediate: Continue using existing methods (holdings, positions, etc.)");
    println!("2. Gradual: Migrate to typed methods (holdings_typed, positions_typed, etc.)");
    println!("3. Enhanced: Use KiteConnectConfig for advanced features");
    println!("4. Future: Full transition to typed APIs in v2.0.0");

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
