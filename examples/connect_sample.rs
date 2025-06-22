//! # Basic KiteConnect API Example
//! 
//! This example demonstrates the basic authentication flow and a simple API call
//! using the modular KiteConnect client.

use kiteconnect_async_wasm::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize KiteConnect client with API key (access token empty for auth flow)
    let mut kiteconnect = KiteConnect::new("<YOUR-API-KEY>", "");

    // Step 1: Get login URL for user authentication
    let login_url = kiteconnect.login_url();
    println!("Please visit this URL to authenticate: {}", login_url);
    println!("After login, copy the 'request_token' from the callback URL");

    // Step 2: Generate session with request token (in real app, get this from callback)
    // Uncomment and replace with actual tokens:
    /*
    let session_response = kiteconnect
        .generate_session("<REQUEST-TOKEN>", "<API-SECRET>")
        .await?;
    println!("Session created successfully: {:?}", session_response);
    */

    // For demo purposes, set access token directly
    // In real applications, this would be set automatically by generate_session()
    kiteconnect.set_access_token("<YOUR-ACCESS-TOKEN>");

    // Step 3: Make API calls - returns typed structs instead of JSON
    match kiteconnect.holdings().await {
        Ok(holdings) => {
            println!("Holdings retrieved successfully!");
            println!("Number of holdings: {}", holdings.len());
            
            // Access typed data directly
            for holding in holdings.iter().take(3) {
                println!("  {} - Qty: {}, P&L: ₹{:.2}", 
                    holding.tradingsymbol, 
                    holding.quantity, 
                    holding.pnl
                );
            }
        }
        Err(e) => {
            eprintln!("Error fetching holdings: {}", e);
        }
    }

    Ok(())
}