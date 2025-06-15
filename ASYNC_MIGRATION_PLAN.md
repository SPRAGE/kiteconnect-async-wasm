# Async Migration Plan for kiteconnect-rs

## Phase 1: Update Dependencies

1. **Update Cargo.toml**:
   ```toml
   [dependencies]
   reqwest = { version = "0.12", features = ["json"] }
   tokio = { version = "1.0", features = ["full"] }
   serde = "1.0"
   serde_derive = "1.0"
   serde_json = "1.0"
   sha2 = "0.10"  # Replace rust-crypto
   url = "2.0"
   anyhow = "1.0"
   csv = "1.3"
   log = "0.4"

   # WASM support
   [target.'cfg(target_arch = "wasm32")'.dependencies]
   reqwest = { version = "0.12", features = ["json"], default-features = false }
   tokio = { version = "1.0", features = ["macros", "rt"], default-features = false }
   
   # WebSocket for ticker (consider tokio-tungstenite for async)
   tokio-tungstenite = { version = "0.21", optional = true }
   
   [features]
   default = ["native"]
   native = ["reqwest/default-tls"]
   wasm = ["reqwest/wasm"]
   ```

## Phase 2: Core Structure Changes

1. **Make all HTTP methods async**:
   - `generate_session` -> `async fn generate_session`
   - `holdings` -> `async fn holdings`
   - `orders` -> `async fn orders`
   - All other API methods

2. **Update RequestHandler trait**:
   ```rust
   #[async_trait]
   trait RequestHandler {
       async fn send_request(
           &self,
           url: reqwest::Url,
           method: &str,
           data: Option<HashMap<&str, &str>>
       ) -> Result<reqwest::Response>;
   }
   ```

3. **Update KiteConnect implementation**:
   ```rust
   impl KiteConnect {
       async fn send_request(&self, ...) -> Result<reqwest::Response> {
           // Use reqwest async client
           let response = client.get(url).send().await?;
           Ok(response)
       }
   }
   ```

## Phase 3: WASM Compatibility

1. **Feature flags for different targets**:
   ```rust
   #[cfg(not(target_arch = "wasm32"))]
   use tokio;
   
   #[cfg(target_arch = "wasm32")]
   use wasm_bindgen_futures;
   ```

2. **Conditional compilation for crypto**:
   ```rust
   #[cfg(not(target_arch = "wasm32"))]
   use sha2::{Sha256, Digest};
   
   #[cfg(target_arch = "wasm32")]
   use web_sys::crypto;
   ```

3. **WebSocket handling**:
   - Native: Use tokio-tungstenite
   - WASM: Use web-sys WebSocket API

## Phase 4: Breaking Changes Management

1. **Version bump**: 0.3.0 (breaking changes)
2. **Migration guide** for users
3. **Backward compatibility** layer (optional)
