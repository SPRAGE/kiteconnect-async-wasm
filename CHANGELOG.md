# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **CSV Parsing for WASM**: Implemented proper CSV parsing in WASM builds using csv-core
  - WASM builds now return structured JSON data instead of raw CSV strings
  - Added csv-core dependency for no-std CSV parsing compatibility
  - Both `instruments()` and `mf_instruments()` now provide consistent output across platforms

### Changed
- **WASM Feature Enhancement**: Updated WASM feature to include csv-core for better CSV handling
- **Documentation**: Updated platform-specific feature descriptions to reflect CSV parsing improvements

## [0.1.2] - 2025-06-15

### Fixed
- **Documentation Build**: Added proper docs.rs configuration for documentation generation
  - Added `[package.metadata.docs.rs]` section with feature specifications
  - Added `#![cfg_attr(docsrs, feature(doc_cfg))]` for proper feature gating in docs
  - Configured docs.rs to build with `native` and `debug` features
  - Added multi-target support for native and WASM documentation

### Technical Details
- Ensures docs.rs builds documentation with all relevant features enabled
- Improves feature flag visibility in generated documentation
- Fixes any potential docs.rs build failures

## [0.1.1] - 2025-06-15

### Added
- **Meaningful Feature Flags**: Implemented functional feature flags that provide real platform-specific capabilities
  - `native`: Enables tokio, SHA-256, CSV parsing with structured JSON output
  - `wasm`: Enables browser APIs, Web Crypto, raw CSV output for client-side parsing
  - `debug`: Enables enhanced logging and debugging features
- **Platform-Specific Implementations**: Different code paths optimized for native vs WASM environments
- **Feature-Gated Dependencies**: Optional dependencies controlled by feature flags
- **Comprehensive Testing Documentation**: Added `FEATURE_TESTING.md` with examples and troubleshooting
- **CI/CD Enhancement**: Updated workflows to test different feature combinations

### Fixed
- **Documentation Warning**: Resolved "wasm feature does not enable additional features" warning
- **CSV Parsing**: Now properly feature-gated behind `native` feature
- **Import Organization**: Cleaned up conditional imports for different platforms

### Changed
- **Build Configurations**: Multiple valid build configurations now supported:
  - Native: `cargo build --features native` (default)
  - WASM: `cargo build --target wasm32-unknown-unknown --features wasm`
  - Debug: `cargo build --features "native,debug"`
  - Minimal: `cargo build --no-default-features`
- **Crypto Implementation**: Platform-specific SHA-256 (native crypto vs Web Crypto API)
- **CSV Handling**: Native gets structured JSON, WASM gets raw CSV strings

### Technical Details
- Feature flags now control actual functionality differences
- Conditional compilation ensures optimal builds for each target
- Fallback implementations provide clear error messages when features are missing
- All 30 tests passing across all build configurations

## [0.1.0] - 2025-06-15

### Added
- Initial release of kiteconnect-async-wasm
- Full async/await support for all KiteConnect REST APIs
- WASM compatibility with web-sys integration
- Comprehensive documentation with examples
- Modern error handling with anyhow
- Complete test suite with mocked responses
- Support for all major KiteConnect operations:
  - Authentication and session management
  - Portfolio management (holdings, positions)
  - Order management (place, modify, cancel)
  - Market data access (instruments, quotes)
  - Mutual fund operations
  - Historical data retrieval

### Features
- **Async-first design**: Built from ground up for async/await
- **WASM support**: Works in browsers and Node.js
- **Type safety**: Comprehensive type definitions for all API responses
- **Error handling**: Detailed error types and recovery patterns
- **Documentation**: Extensive docs with real-world examples
- **Testing**: Mock-based testing for reliable development

### Technical Details
- REST-only implementation (WebSocket/ticker functionality removed for simplicity)
- Platform-specific HTTP clients (reqwest for native, fetch API for WASM)
- Clean separation between native and WASM code paths
- Comprehensive API coverage matching official KiteConnect specification

### Dependencies
- Core: serde, anyhow, url, async-trait
- Native: tokio, reqwest, sha2, csv
- WASM: wasm-bindgen, web-sys, js-sys, gloo-utils

[0.1.0]: https://github.com/SPRAGE/kiteconnect-async-wasm/releases/tag/v0.1.0
