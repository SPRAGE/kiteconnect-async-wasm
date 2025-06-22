# kiteconnect-async-wasm

[![Crates.io](https://img.shields.io/crates/v/kiteconnect-async-wasm.svg)](https://crates.io/crates/kiteconnect-async-wasm)
[![Documentation](https://docs.rs/kiteconnect-async-wasm/badge.svg)](https://docs.rs/kiteconnect-async-wasm)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

**Modern async Rust client for KiteConnect REST APIs with WASM support**

A clean, well-documented, and focused Rust library for KiteConnect API integration. This library provides:

## Features

- ✅ **Async-first design** with tokio support
- ✅ **WASM compatibility** for web applications  
- ✅ **REST-only focus** - no WebSocket complexity
- ✅ **Comprehensive documentation** with examples
- ✅ **Type safety** with proper error handling
- ✅ **No warranties license** (Unlicense)  

## Platform Support

- **Native**: Full API support with CSV parsing for instruments
- **WASM**: All APIs supported (instruments return raw CSV for client-side parsing)

## Docs

https://docs.rs/kiteconnect

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
kiteconnect-async-wasm = "0.1.1", features = ["native"] }

# For WASM targets
# kiteconnect-async-wasm = "0.1.1", features = ["wasm"] }
```

### KiteConnect REST APIs (Async)

```rust
use kiteconnect_async_wasm::connect::KiteConnect;
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kiteconnect = KiteConnect::new("<API-KEY>", "");

    // Open browser with this URL and get the request token from the callback
    let loginurl = kiteconnect.login_url();
    println!("{:?}", loginurl);

    // Generate access token with the above request token
    let session = kiteconnect.generate_session("<REQUEST-TOKEN>", "<API-SECRET>").await?;
    // `generate_session` internally sets the access token from the response
    println!("Access token: {}", session.access_token);
    println!("User ID: {}", session.user_id);

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

## Attribution

**Current Maintainer:** SPRAGE <shauna.pai@gmail.com>

This project was originally created by Joe Paul and other contributors. 
The current version has been significantly rewritten and modernized with:
- Complete async/await implementation
- WASM compatibility
- Enhanced feature flags system
- Comprehensive documentation
- Modern CI/CD pipeline

## License

This software is released into the public domain under The Unlicense. 
See the [LICENSE](LICENSE) file for details.

**No warranties provided** - This software is provided "as is" without warranty of any kind.
