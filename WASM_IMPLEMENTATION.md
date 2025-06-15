# WASM Support Implementation Guide

## Overview
Adding WASM support will allow the kiteconnect library to run in web browsers, enabling direct API calls from web applications.

## Implementation Strategy

### 1. Update Cargo.toml with Feature Flags

```toml
[package]
name = "kiteconnect"
version = "0.3.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
url = "2.0"
log = "0.4"

# Async runtime
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
sha2 = "0.10"
csv = "1.3"

# WASM-specific dependencies
[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = { version = "0.12", features = ["json"], default-features = false }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "console",
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "Storage",
    "WebSocket",
    "MessageEvent",
    "ErrorEvent",
    "CloseEvent",
    "BinaryType",
    "Crypto",
    "SubtleCrypto",
] }
gloo-utils = "0.1"

# WebSocket support
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio-tungstenite = { version = "0.21", optional = true }

[features]
default = ["native"]
native = []
wasm = []
websocket = ["tokio-tungstenite"]

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### 2. Conditional Compilation Structure

```rust
// lib.rs
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export the appropriate implementation
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

pub mod common;  // Shared code
```

### 3. Crypto Implementation

```rust
// common/crypto.rs
use anyhow::Result;

#[cfg(not(target_arch = "wasm32"))]
pub async fn sha256_hash(input: &str) -> Result<String> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(target_arch = "wasm32")]
pub async fn sha256_hash(input: &str) -> Result<String> {
    use js_sys::Uint8Array;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{window, SubtleCrypto};
    
    let window = window().ok_or_else(|| anyhow::anyhow!("No window object"))?;
    let crypto = window.crypto().map_err(|_| anyhow::anyhow!("No crypto object"))?;
    let subtle = crypto.subtle();
    
    let data = Uint8Array::from(input.as_bytes());
    let digest_promise = subtle.digest_with_str_and_u8_array("SHA-256", &data)
        .map_err(|_| anyhow::anyhow!("Failed to create digest"))?;
    
    let digest_result = JsFuture::from(digest_promise).await
        .map_err(|_| anyhow::anyhow!("Failed to compute hash"))?;
    
    let digest_array = Uint8Array::new(&digest_result);
    let digest_vec: Vec<u8> = digest_array.to_vec();
    
    Ok(hex::encode(digest_vec))
}
```

### 4. HTTP Client Implementation

```rust
// native/client.rs
use reqwest;
use anyhow::Result;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        Ok(self.client.get(url).send().await?)
    }
    
    // ... other methods
}

// wasm/client.rs
use reqwest;
use anyhow::Result;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        // In WASM, reqwest uses fetch API under the hood
        Ok(self.client.get(url).send().await?)
    }
}
```

### 5. WebSocket Implementation

```rust
// native/websocket.rs
#[cfg(feature = "websocket")]
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};

// wasm/websocket.rs
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use wasm_bindgen::prelude::*;
```

## Usage Examples

### Native (Tokio Runtime)
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kite = KiteConnect::new("api_key", "access_token");
    let holdings = kite.holdings().await?;
    println!("{:?}", holdings);
    Ok(())
}
```

### WASM (Browser)
```rust
use wasm_bindgen_futures::spawn_local;

spawn_local(async {
    let mut kite = KiteConnect::new("api_key", "access_token");
    let holdings = kite.holdings().await.unwrap();
    web_sys::console::log_1(&format!("{:?}", holdings).into());
});
```

## Build Commands

### Native
```bash
cargo build --features native
cargo test --features native
```

### WASM
```bash
wasm-pack build --features wasm --target web
wasm-pack build --features wasm --target nodejs
```

## Considerations

1. **CORS**: Browser security restrictions may require proxy server for API calls
2. **Authentication**: Handle tokens securely in browser environment
3. **Bundle Size**: WASM bundles can be large; optimize dependencies
4. **Error Handling**: Different error types between native and WASM
5. **Testing**: Use wasm-bindgen-test for WASM-specific tests
