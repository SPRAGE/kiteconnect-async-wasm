extern crate kiteconnect_async_wasm;
extern crate serde_json as json;

use kiteconnect_async_wasm::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kiteconnect = KiteConnect::new("<API-KEY>", "");

    // Open browser with this URL and get the request token from the callback
    let loginurl = kiteconnect.login_url();
    println!("{:?}", loginurl);

    // Generate access token with the above request token
    let resp = kiteconnect.generate_session("<REQUEST-TOKEN>", "<API-SECRET>").await?;
    // `generate_session` internally sets the access token from the response
    println!("{:?}", resp);

    let holdings: json::Value = kiteconnect.holdings().await?;
    println!("{:?}", holdings);

    Ok(())
}