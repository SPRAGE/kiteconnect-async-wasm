# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.7] - 2025-06-22

### Improved
- **DateTime Support for login_time**: Enhanced `UserSession.login_time` field with proper `DateTime<Utc>` parsing
  - Changed from `String` to `DateTime<Utc>` with custom deserializer for "yyyy-mm-dd hh:mm:ss" format
  - Added utility methods: `login_time_formatted()`, `time_since_login()`, `logged_in_within_hours()`, `logged_in_today()`
  - Maintains automatic deserialization from KiteConnect API string format
  - Updated examples to demonstrate datetime functionality and calculation methods
  - Added comprehensive tests for datetime parsing and utility methods

### Documentation
- Updated examples to showcase proper DateTime usage and time-based calculations
- Enhanced comprehensive_models_demo to display login time analysis features

## [0.1.6] - 2025-06-22

### Improved
- **Typed Session Response**: Updated `generate_session()` method to return strongly-typed `UserSession` struct
  - Changed return type from `Result<JsonValue>` to `Result<UserSession>` for better type safety
  - Automatic access token extraction and setting from typed response
  - Improved documentation examples showing typed field access (`session.access_token`, `session.user_name`)
  - Maintains backwards compatibility for all functionality while providing better developer experience

### Documentation  
- Updated README.md and lib.rs examples to demonstrate typed session response usage
- Enhanced documentation showing how to access user information from session response
- All documentation tests continue to pass with updated examples

## [0.1.5] - 2025-06-22

### Fixed
- **Examples Compatibility**: Updated all examples to work with current modular API structure
  - Fixed GTT API method calls in examples to use correct signature (`gtts(None)` instead of `gtts()`)
  - Corrected type mismatches in example struct initialization (`instrument_token: u32` instead of `Option<u32>`)
  - Updated imports in examples to use new modular organization (`use kiteconnect_async_wasm::model::*`)
  - Updated examples to use proper constants modules (`products::CNC`, `order_types::LIMIT`, etc.)

### Documentation
- All examples now compile successfully and demonstrate proper usage of typed API responses
- Examples properly showcase the transition from JSON-based to struct-based responses
- Fixed example code to match current API patterns and conventions

## [0.1.4] - 2025-06-15

### Added
- **Complete API Parity**: Added all missing APIs from original implementation to achieve 100% feature parity
  - `historical_data()` - Retrieve historical candlestick data for backtesting and analysis
  - `place_mf_order()` - Place mutual fund buy/sell orders with quantity or amount options
  - `cancel_mf_order()` - Cancel pending mutual fund orders
  - `mf_sips()` - Get SIP (Systematic Investment Plan) details for all or specific SIPs
  - `place_mf_sip()` - Create new SIPs with flexible frequency and amount options
  - `modify_mf_sip()` - Modify existing SIPs (amount, frequency, status)
  - `cancel_mf_sip()` - Cancel active SIPs
  - `mf_holdings()` - Get mutual fund holdings with current values and returns
  - `quote()` - Get real-time quotes with market depth and bid/ask data
  - `ohlc()` - Get OHLC (Open, High, Low, Close) data for instruments
  - `ltp()` - Get Last Traded Price for quick price checks
  - `instruments_margins()` - Get margin requirements for specific trading segments

### Enhanced
- **Comprehensive Documentation**: Each new method includes detailed rustdoc with:
  - Complete parameter descriptions and examples
  - Error handling scenarios
  - Usage patterns for common trading workflows
  - Real-world code examples that can be copied and used
- **Modern Async Patterns**: All new methods follow consistent async/await patterns
- **Cross-Platform Support**: All APIs work seamlessly across native (Tokio) and WASM platforms
- **Type Safety**: Consistent parameter validation and return types across all methods

### Technical Improvements
- **37 Total APIs**: Now provides complete coverage of all KiteConnect REST endpoints
- **Mutual Fund Ecosystem**: Full support for MF orders, SIPs, and portfolio management
- **Market Data Access**: Real-time and historical data retrieval for trading strategies
- **Platform Agnostic**: Identical API surface across native and browser environments

### Developer Experience
- **Example-Driven Documentation**: Every method includes practical usage examples
- **Error Handling**: Proper Result types with descriptive error messages
- **IDE Support**: Full IntelliSense/LSP support with rich documentation
- **Testing Coverage**: Comprehensive test suite covering all new functionality

## [0.1.3] - 2025-06-15

### Added
- **CSV Parsing for WASM**: Implemented proper CSV parsing in WASM builds using csv-core
  - WASM builds now return structured JSON data instead of raw CSV strings
  - Added csv-core dependency for no-std CSV parsing compatibility
  - Both `instruments()` and `mf_instruments()` now provide consistent output across platforms

### Changed
- **WASM Feature Enhancement**: Updated WASM feature to include csv-core for better CSV handling
- **Documentation**: Updated platform-specific feature descriptions to reflect CSV parsing improvements
- **API Consistency**: WASM and native builds now return identical JSON structures

### Removed
- **Cleanup**: Removed unnecessary documentation files and build artifacts
- **Travis CI**: Removed `.travis.yml` in favor of GitHub Actions
- **Legacy Files**: Cleaned up redundant documentation and generated files

### Technical Details
- Added `parse_csv_with_core()` helper function for WASM-compatible CSV parsing
- Updated feature flags to include csv-core dependency for WASM builds
- Maintained backward compatibility while improving WASM developer experience

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
