# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.7] - 2025-06-29

### Fixed
- **Documentation Generation**: Fixed documentation generation script path issues
  - Corrected crate name references from `kiteconnect` to `kiteconnect_async_wasm`
  - Fixed all documentation links and paths in generation script
  - Documentation now generates successfully with proper navigation links

### Enhanced
- **Documentation**: Comprehensive documentation updates and regeneration
  - Generated complete API documentation with 38,356+ HTML files
  - Added Quick Reference guide with updated examples including v1.0.6+ features
  - Updated documentation to reflect environment variable usage patterns
  - Added proper documentation for custom Candle deserializer functionality

### Improved
- **Code Quality**: Removed unused imports and warnings
  - Fixed unused import warnings in `candle_deserialization_test.rs`
  - Ensured all examples compile cleanly without warnings
  - Improved overall code quality and maintenance

## [1.0.6] - 2025-06-29

### Fixed
- **Historical Data Deserialization**: Major improvements to historical data API robustness
  - Fixed custom `Candle` deserializer to handle both array and object formats from KiteConnect API
  - Resolved missing metadata field issue by synthesizing metadata from request parameters
  - Added support for various date/time formats including timezone-aware parsing (+0530)
  - Fixed handling of missing Open Interest (OI) data when API returns 6 elements instead of 7
  - Enhanced error handling for different API response formats

- **Environment Variable Support**: Updated all examples to use environment variables
  - All historical data examples now use `KITE_API_KEY`, `KITE_ACCESS_TOKEN`, and `KITE_API_SECRET`
  - Added proper error messages when environment variables are not set
  - Improved example code robustness and security

### Enhanced
- **Example Code**: Updated and tested all historical data examples
  - `simple_256265_example.rs` - Successfully fetches real market data for instrument token 256265
  - `historical_data_256265_example.rs` - Comprehensive example with validation
  - `candle_deserialization_test.rs` - Test suite for various data formats
  - All examples now compile and run without errors

- **API Robustness**: Improved resilience to KiteConnect API response variations
  - Custom deserializer handles both legacy and current API response formats
  - Graceful degradation when optional fields are missing
  - Better error reporting for debugging API issues

## [1.0.5] - 2025-06-26

### Added
- **Enhanced Historical Data API**: Complete refactor of historical data handling
  - New `HistoricalDataRequest` struct with `NaiveDateTime` support for precise datetime handling
  - Updated `historical_data_typed()` method to use structured request parameter instead of multiple strings
  - Support for datetime format `yyyy-mm-dd hh:mm:ss` for better precision
  - Builder pattern methods: `.continuous()` and `.with_oi()` for optional parameters

- **Dual Serde Support for Interval Enum**: Flexible serialization/deserialization
  - Accepts both string ("minute", "day") and integer (1, 0) formats during deserialization
  - Always serializes as strings for API consistency
  - Backward compatible with existing code

- **Enhanced Enum Organization**: Major refactoring for better maintainability
  - Split monolithic `enums.rs` (500+ lines) into focused submodules:
    - `exchange.rs` - Exchange enum with helper methods
    - `trading.rs` - Trading-related enums (Product, Validity, etc.)
    - `instruments.rs` - InstrumentType and Segment enums  
    - `interval.rs` - Interval enum with custom serde implementation
    - `gtt.rs` - GttStatus enum
  - Maintained full backward compatibility through re-exports
  - Added comprehensive helper methods and documentation

### Improved
- **Code Quality**: Comprehensive linting and formatting improvements
  - Fixed clippy warnings for performance and best practices
  - Applied consistent code formatting with rustfmt
  - Removed redundant code patterns and optimized implementations
  - Enhanced error handling patterns

- **Type Safety**: Enhanced type definitions and validation
  - Added missing `COMMODITY` variant to `InstrumentType` enum
  - Improved error handling with more specific error types
  - Better validation for datetime parsing and formatting

- **Documentation**: Comprehensive updates across all modules
  - Enhanced examples with real-world usage patterns
  - Updated all docstrings with current API patterns
  - Added migration examples for breaking changes
  - Improved error handling documentation

### Fixed
- **API Consistency**: Resolved various API inconsistencies
  - Fixed datetime formatting across different methods
  - Standardized error handling patterns
  - Improved parameter validation

- **Build Issues**: Resolved compilation and clippy warnings
  - Fixed unused imports and dead code warnings
  - Optimized vector initialization patterns
  - Improved range checking implementations

### Technical Details
- All 30 unit tests + 11 integration tests + 58 documentation tests passing
- Maintains full backward compatibility for existing code
- Enhanced type safety without breaking changes
- Improved performance through clippy optimizations
- Professional code formatting and organization

### Migration Notes
- `historical_data_typed()` now takes `HistoricalDataRequest` struct instead of individual parameters
- `NaiveDateTime` replaces `NaiveDate` for `from` and `to` fields in historical data requests
- Enum imports remain unchanged due to re-exports (no action required)

## [0.1.8] - 2025-06-24

### Changed
- **Major Refactoring**: Split monolithic `connect.rs` file (2,134 lines) into modular structure
  - Created focused modules: `auth.rs`, `portfolio.rs`, `orders.rs`, `market_data.rs`, `mutual_funds.rs`, `utils.rs`
  - Added `models` module for future data structure definitions
  - Improved code organization and maintainability
  - All existing functionality preserved with no breaking changes
  - Enhanced documentation and examples throughout modules

### Technical
- Fixed compilation errors and import issues across all modules
- Cleaned up unused imports and optimized module structure
- All tests continue to pass (4/4 unit tests, 27/27 doc tests)
- Better separation of concerns and single responsibility principle

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
