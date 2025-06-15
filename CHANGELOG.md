# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
