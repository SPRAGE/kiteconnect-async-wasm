extern crate kiteconnect_async_wasm;
extern crate serde_json as json;

use kiteconnect_async_wasm::connect::KiteConnect;
use std::env;

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
    let (api_key, api_secret) = get_credentials()?;
    let mut kiteconnect = KiteConnect::new(&api_key, "");

    // Open browser with this URL and get the request token from the callback
    let loginurl = kiteconnect.login_url();
    println!("{:?}", loginurl);

    // Generate access token with the above request token
    let resp = kiteconnect
        .generate_session("<REQUEST-TOKEN>", &api_secret)
        .await?;
    // `generate_session` internally sets the access token from the response
    println!("{:?}", resp);

    let holdings: json::Value = kiteconnect.holdings().await?;
    println!("{:?}", holdings);

    Ok(())
}
