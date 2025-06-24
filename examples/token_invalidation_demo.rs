//! # Authentication Example with Token Invalidation
//! 
//! This example demonstrates how to use the corrected `invalidate_access_token` method
//! which now properly uses DELETE request with query parameters according to the API documentation.

use kiteconnect_async_wasm::connect::KiteConnect;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== KiteConnect Authentication and Token Invalidation Example ===");
    
    // Initialize client with API key (access token will be set after login)
    let mut client = KiteConnect::new("your_api_key", "");
    
    // Step 1: Generate login URL
    let login_url = client.login_url();
    println!("1. Please visit the login URL:");
    println!("   {}", login_url);
    println!("   After login, you'll get a request_token in the callback URL");
    
    // Step 2: Generate session (commented out - requires actual tokens)
    /*
    let session_response = client
        .generate_session("request_token_from_callback", "your_api_secret")
        .await?;
    
    println!("2. Session created successfully:");
    println!("   Access Token: {}", session_response["data"]["access_token"]);
    println!("   User ID: {}", session_response["data"]["user_id"]);
    
    // The access token is automatically set in the client
    let current_token = session_response["data"]["access_token"].as_str().unwrap();
    */
    
    // For demonstration, let's use a dummy token
    let demo_token = "demo_access_token_123";
    client.set_access_token(demo_token);
    
    println!("2. Using demo access token: {}", demo_token);
    
    // Step 3: Use the API (this would work with a real token)
    println!("3. Making API calls with the access token...");
    println!("   (This would work with a real access token)");
    
    // Step 4: Invalidate the access token (logout)
    println!("4. Invalidating access token...");
    
    // Method 1: Legacy API (returns JsonValue)
    match client.invalidate_access_token(demo_token).await {
        Ok(response) => {
            println!("   ✅ Legacy API - Token invalidated successfully:");
            println!("   Response: {}", response);
        }
        Err(e) => {
            println!("   ❌ Legacy API - Failed to invalidate token: {}", e);
            println!("   (This is expected with a demo token)");
        }
    }
    
    // Method 2: Typed API (returns bool)
    match client.invalidate_access_token_typed(demo_token).await {
        Ok(success) => {
            println!("   ✅ Typed API - Token invalidation result: {}", success);
        }
        Err(e) => {
            println!("   ❌ Typed API - Failed to invalidate token: {}", e);
            println!("   (This is expected with a demo token)");
        }
    }
    
    println!();
    println!("=== API Call Details ===");
    println!("The invalidate_access_token method now correctly uses:");
    println!("• HTTP Method: DELETE");
    println!("• Endpoint: /session/token");
    println!("• Parameters: api_key and access_token as query parameters");
    println!("• Expected Response: {{ \"status\": \"success\", \"data\": true }}");
    
    println!();
    println!("=== Curl Equivalent ===");
    println!("curl --request DELETE \\");
    println!("  -H \"X-Kite-Version: 3\" \\");
    println!("  \"https://api.kite.trade/session/token?api_key=xxx&access_token=yyy\"");
    
    Ok(())
}
