//! # Endpoint Management and Rate Limiting Example
//!
//! This example demonstrates the new centralized endpoint management system
//! and built-in rate limiting functionality in KiteConnect v1.0.0.

use kiteconnect_async_wasm::connect::{
    KiteConnect, KiteConnectConfig, KiteEndpoint, RateLimitCategory,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KiteConnect Endpoint Management & Rate Limiting Demo ===\n");

    // Example 1: Endpoint Information System
    demonstrate_endpoint_system().await;

    // Example 2: Rate Limiting Configuration
    demonstrate_rate_limiting_config().await?;

    // Example 3: Rate Limiting in Action
    demonstrate_rate_limiting_in_action().await?;

    // Example 4: Rate Limiter Statistics
    demonstrate_rate_limiter_stats().await?;

    Ok(())
}

async fn demonstrate_endpoint_system() {
    println!("=== 1. Endpoint Information System ===");

    // Show endpoint configurations
    let endpoints = vec![
        KiteEndpoint::Quote,
        KiteEndpoint::HistoricalData,
        KiteEndpoint::PlaceOrder,
        KiteEndpoint::Holdings,
    ];

    for endpoint in endpoints {
        let config = endpoint.config();
        println!("Endpoint: {:?}", endpoint);
        println!("  Method: {:?}", config.method);
        println!("  Path: {}", config.path);
        println!(
            "  Rate Category: {:?} ({} req/sec)",
            config.rate_limit_category,
            config.rate_limit_category.requests_per_second()
        );
        println!("  Requires Auth: {}", config.requires_auth);
        println!("  Min Delay: {:?}", config.rate_limit_category.min_delay());
        println!();
    }

    // Show path building
    println!("Dynamic path building examples:");
    println!(
        "  Order History: {}",
        KiteEndpoint::OrderHistory.build_path(&["order_123"])
    );
    println!(
        "  Order Trades: {}",
        KiteEndpoint::OrderTrades.build_path(&["order_123", "trades"])
    );
    println!(
        "  Historical Data: {}",
        KiteEndpoint::HistoricalData.build_path(&["738561", "day"])
    );
    println!();

    // Show endpoints by category
    println!("Endpoints by rate limit category:");
    for category in [
        RateLimitCategory::Quote,
        RateLimitCategory::Historical,
        RateLimitCategory::Orders,
        RateLimitCategory::Standard,
    ] {
        let endpoints = KiteEndpoint::by_rate_limit_category(category.clone());
        println!(
            "  {:?} ({} req/sec): {} endpoints",
            category,
            category.requests_per_second(),
            endpoints.len()
        );
    }
    println!();
}

async fn demonstrate_rate_limiting_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 2. Rate Limiting Configuration ===");

    // Create client with rate limiting enabled (default)
    let config_with_limits = KiteConnectConfig {
        enable_rate_limiting: true,
        ..Default::default()
    };
    let client_with_limits = KiteConnect::new_with_config("demo_key", config_with_limits);
    println!("✓ Client created with rate limiting ENABLED");
    println!(
        "  Rate limiting enabled: {}",
        client_with_limits.is_rate_limiting_enabled()
    );

    // Create client with rate limiting disabled
    let config_no_limits = KiteConnectConfig {
        enable_rate_limiting: false,
        ..Default::default()
    };
    let client_no_limits = KiteConnect::new_with_config("demo_key", config_no_limits);
    println!("✓ Client created with rate limiting DISABLED");
    println!(
        "  Rate limiting enabled: {}",
        client_no_limits.is_rate_limiting_enabled()
    );

    // Show rate limit information
    println!("\nOfficial KiteConnect API Rate Limits:");
    println!(
        "  Quote endpoints: {} request/second",
        RateLimitCategory::Quote.requests_per_second()
    );
    println!(
        "  Historical endpoints: {} requests/second",
        RateLimitCategory::Historical.requests_per_second()
    );
    println!(
        "  Order endpoints: {} requests/second",
        RateLimitCategory::Orders.requests_per_second()
    );
    println!(
        "  Standard endpoints: {} requests/second",
        RateLimitCategory::Standard.requests_per_second()
    );
    println!();

    Ok(())
}

async fn demonstrate_rate_limiting_in_action() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 3. Rate Limiting in Action ===");

    let client = KiteConnect::new("demo_key", "demo_token");

    println!("Testing Quote endpoint rate limiting (1 req/sec):");

    // First request should be immediate
    let start = Instant::now();
    let can_request = client.can_request_immediately(&KiteEndpoint::Quote).await;
    println!("  First request - Can request immediately: {}", can_request);

    // Simulate making a request
    client.wait_for_request(&KiteEndpoint::Quote).await;
    println!("  First request completed in: {:?}", start.elapsed());

    // Second request should require waiting
    let can_request_again = client.can_request_immediately(&KiteEndpoint::Quote).await;
    println!(
        "  Second request - Can request immediately: {}",
        can_request_again
    );

    let delay_needed = client.get_delay_for_request(&KiteEndpoint::Quote).await;
    println!("  Delay needed for second request: {:?}", delay_needed);

    // Test different category - should be available immediately
    let can_request_different = client
        .can_request_immediately(&KiteEndpoint::Holdings)
        .await;
    println!(
        "  Different category (Holdings) - Can request immediately: {}",
        can_request_different
    );

    println!();

    // Demonstrate automatic waiting
    println!("Testing automatic rate limiting with multiple requests:");
    let overall_start = Instant::now();

    for i in 1..=3 {
        let request_start = Instant::now();
        client.wait_for_request(&KiteEndpoint::Quote).await;
        println!(
            "  Request {} completed in: {:?} (total: {:?})",
            i,
            request_start.elapsed(),
            overall_start.elapsed()
        );
    }

    println!();

    Ok(())
}

async fn demonstrate_rate_limiter_stats() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 4. Rate Limiter Statistics ===");

    let client = KiteConnect::new("demo_key", "demo_token");

    // Make some requests to different endpoints
    client.wait_for_request(&KiteEndpoint::Quote).await;
    client.wait_for_request(&KiteEndpoint::Holdings).await;
    client.wait_for_request(&KiteEndpoint::PlaceOrder).await;

    // Get statistics
    let stats = client.rate_limiter_stats().await;

    println!("Rate Limiter Statistics:");
    println!("  Enabled: {}", stats.enabled);
    println!("  Categories tracked: {}", stats.categories.len());

    for (category, category_stats) in &stats.categories {
        println!("  {:?}:", category);
        println!(
            "    Request count: {}/{}",
            category_stats.request_count, category_stats.requests_per_second
        );
        println!(
            "    Remaining capacity: {}",
            category_stats.remaining_capacity()
        );
        println!("    At limit: {}", category_stats.is_at_limit());
        if let Some(last) = category_stats.last_request {
            println!("    Last request: {:?} ago", last.elapsed());
        }
        if let Some(next) = category_stats.next_available {
            if next > Instant::now() {
                println!(
                    "    Next available: {:?}",
                    next.duration_since(Instant::now())
                );
            } else {
                println!("    Next available: now");
            }
        }
    }

    println!();

    // Show general client statistics
    println!("General Client Statistics:");
    println!("  Total requests made: {}", client.request_count());
    println!(
        "  Rate limiting enabled: {}",
        client.is_rate_limiting_enabled()
    );

    Ok(())
}

/// Example of how endpoints would be used in actual API methods
#[allow(dead_code)]
async fn example_api_method_usage() -> Result<(), Box<dyn std::error::Error>> {
    let _client = KiteConnect::new("api_key", "access_token");

    // This is how API methods would use the new endpoint system:
    // Note: This is pseudocode as we haven't updated the actual API methods yet

    // For quote request:
    // let response = client.send_request_with_rate_limiting_and_retry(
    //     KiteEndpoint::Quote,
    //     &[],  // No path segments
    //     Some(vec![("i", "NSE:RELIANCE")]),  // Query parameters
    //     None,  // No form data
    // ).await?;

    // For order placement:
    // let mut form_data = HashMap::new();
    // form_data.insert("tradingsymbol", "RELIANCE");
    // form_data.insert("exchange", "NSE");
    // let response = client.send_request_with_rate_limiting_and_retry(
    //     KiteEndpoint::PlaceOrder,
    //     &[],  // No path segments
    //     None,  // No query parameters
    //     Some(form_data),  // Form data for POST
    // ).await?;

    // For order history:
    // let response = client.send_request_with_rate_limiting_and_retry(
    //     KiteEndpoint::OrderHistory,
    //     &["order_123"],  // Path segment for specific order
    //     None,  // No query parameters
    //     None,  // No form data
    // ).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kiteconnect_async_wasm::connect::endpoints::HttpMethod;

    #[tokio::test]
    async fn test_endpoint_system() {
        // Test endpoint configuration
        let quote_endpoint = KiteEndpoint::Quote;
        let config = quote_endpoint.config();

        assert_eq!(config.method, HttpMethod::GET);
        assert_eq!(config.path, "/quote");
        assert_eq!(config.rate_limit_category, RateLimitCategory::Quote);
        assert!(config.requires_auth);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let client = KiteConnect::new("test_key", "test_token");

        // First request should be immediate
        assert!(client.can_request_immediately(&KiteEndpoint::Quote).await);

        // Make the request
        client.wait_for_request(&KiteEndpoint::Quote).await;

        // Second request should require waiting (quote is 1 req/sec)
        assert!(!client.can_request_immediately(&KiteEndpoint::Quote).await);

        // Different category should still be available
        assert!(
            client
                .can_request_immediately(&KiteEndpoint::Holdings)
                .await
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_stats() {
        let client = KiteConnect::new("test_key", "test_token");

        // Make some requests
        client.wait_for_request(&KiteEndpoint::Quote).await;
        client.wait_for_request(&KiteEndpoint::Holdings).await;

        let stats = client.rate_limiter_stats().await;
        assert!(stats.enabled);
        assert_eq!(stats.categories.len(), 4); // Quote, Historical, Orders, Standard

        // Quote category should have 1 request
        let quote_stats = &stats.categories[&RateLimitCategory::Quote];
        assert_eq!(quote_stats.request_count, 1);

        // Standard category should have 1 request
        let standard_stats = &stats.categories[&RateLimitCategory::Standard];
        assert_eq!(standard_stats.request_count, 1);
    }
}
