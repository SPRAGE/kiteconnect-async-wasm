# kiteconnect-rs
[![Crates.io](https://img.shields.io/crates/v/kiteconnect.svg)](https://crates.io/crates/kiteconnect)
[![Travis](https://img.shields.io/travis/zerodhatech/kiteconnect-rs/master.svg)](https://travis-ci.org/zerodhatech/kiteconnect-rs/)

Async API wrapper for Kite Connect with WASM support

## Features

✨ **Async/Await Support**: Built with modern Rust async patterns for better performance  
🌐 **WASM Compatible**: Run in browsers with WebAssembly support  
🔄 **Multi-platform**: Native and Web targets supported  
📦 **Modern Dependencies**: Updated to latest Rust ecosystem libraries  
🧪 **Well Tested**: Comprehensive test coverage  

## Platform Support

- **Native**: Full API support with CSV parsing for instruments
- **WASM**: All APIs supported (instruments return raw CSV for client-side parsing)

## Docs

https://docs.rs/kiteconnect

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
kiteconnect = { version = "0.3.0", features = ["native"] }

# For WASM targets
# kiteconnect = { version = "0.3.0", features = ["wasm"] }
```

### KiteConnect REST APIs (Async)

```rust
use kiteconnect::connect::KiteConnect;
use serde_json::Value as JsonValue;

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

    let holdings: JsonValue = kiteconnect.holdings().await?;
    println!("{:?}", holdings);

    Ok(())
}
```

## Running Examples

### KiteConnect REST API sample

```bash
cargo run --example connect_sample
```

## TODO
- [ ] Add serializer structs for all kiteconnect returning datastructures
- [ ] Reconnection mechanism
