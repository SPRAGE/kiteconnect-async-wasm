# Feature Testing Examples

This document shows how to test and use the different feature flags in kiteconnect-async-wasm.

## Available Features

### `native` (default)
Enables native platform support with full functionality:
- `tokio` runtime for async operations
- `sha2` for cryptographic operations
- `csv` for parsing CSV instrument files
- Native HTTP client optimizations

### `wasm`
Enables WebAssembly support for browser environments:
- `wasm-bindgen` for JavaScript interop
- `web-sys` for browser APIs
- `js-sys` for JavaScript standard objects
- `gloo-utils` for utility functions
- Browser-compatible HTTP client

### `debug`
Enables additional logging and debugging features:
- Enhanced `log` output
- Debug information in error messages

## Testing Different Configurations

### Native Build (Default)
```bash
# Build with native features
cargo build

# Run tests with native features
cargo test

# Check with native features
cargo check --features native
```

### WASM Build
```bash
# Add WASM target if not already installed
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm

# Check WASM build
cargo check --target wasm32-unknown-unknown --no-default-features --features wasm

# Build with wasm-pack (for web applications)
wasm-pack build --target web --no-default-features --features wasm
```

### Debug Build
```bash
# Build with debug features (native + debug)
cargo build --features "native,debug"

# Run tests with debug output
RUST_LOG=debug cargo test --features "native,debug"
```

### Minimal Build (No Features)
```bash
# Build with no features (limited functionality)
cargo build --no-default-features

# This will have very limited functionality and some methods may not work
```

## Feature Combinations

### Valid Combinations
```toml
# Default: Native support
kiteconnect-async-wasm = "0.1.0"

# WASM only
kiteconnect-async-wasm = { version = "0.1.0", default-features = false, features = ["wasm"] }

# Native with debug
kiteconnect-async-wasm = { version = "0.1.0", features = ["native", "debug"] }

# WASM with debug
kiteconnect-async-wasm = { version = "0.1.0", default-features = false, features = ["wasm", "debug"] }
```

### Invalid Combinations
```toml
# DON'T: Both native and wasm (will cause conflicts)
kiteconnect-async-wasm = { version = "0.1.0", features = ["native", "wasm"] }
```

## Platform-Specific Code Examples

### Native Platform
```rust
use kiteconnect_async_wasm::connect::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This will use native HTTP client and SHA-256 implementation
    let client = KiteConnect::new("api_key", "");
    
    // Native-specific features work
    let instruments = client.instruments(None).await?;
    // CSV parsing available
    // Full crypto support
    
    Ok(())
}
```

### WASM Platform
```rust
use kiteconnect_async_wasm::connect::KiteConnect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn init_client() -> Result<(), JsValue> {
    // This will use browser fetch API and Web Crypto API
    let client = KiteConnect::new("api_key", "");
    
    // WASM-specific features work
    let session = client.generate_session("request_token", "api_secret").await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(())
}
```

## Testing Feature Flags

### Unit Tests
```bash
# Test default features
cargo test

# Test native features explicitly
cargo test --features native

# Test with debug features
cargo test --features "native,debug"

# Test WASM build (compile only, no runtime tests)
cargo check --target wasm32-unknown-unknown --no-default-features --features wasm
```

### Integration Tests
```bash
# Test examples with different features
cargo run --example comprehensive_example --features native

# Test WASM with wasm-pack
wasm-pack test --node --no-default-features --features wasm
```

### Documentation Tests
```bash
# Generate docs with native features
cargo doc --features native

# Generate docs with WASM features
cargo doc --target wasm32-unknown-unknown --no-default-features --features wasm
```

## Troubleshooting

### Common Issues

1. **Compilation errors with `sha2` not found**
   - Solution: Ensure you're using `--features native` or the default features

2. **WASM compilation fails with missing dependencies**
   - Solution: Use `--no-default-features --features wasm` for WASM builds

3. **Both native and WASM features enabled**
   - Solution: Use either native OR wasm, not both

4. **Missing functionality with no features**
   - Solution: Enable at least one platform feature (native or wasm)

### Feature Detection in Code
```rust
// Check if native features are available
#[cfg(feature = "native")]
fn native_specific_function() {
    // This only compiles with native features
}

// Check if WASM features are available  
#[cfg(feature = "wasm")]
fn wasm_specific_function() {
    // This only compiles with wasm features
}

// Check if debug features are available
#[cfg(feature = "debug")]
fn debug_function() {
    log::debug!("Debug logging enabled");
}
```

This feature system ensures that the library can be optimized for different target platforms while maintaining a clean, focused API.
