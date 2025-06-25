/*!
Common types and utilities shared across all KiteConnect models.

This module provides:
- Custom error types (`KiteError`)
- Response wrapper types (`KiteResponse<T>`)
- Shared enums organized in logical submodules:
  - `enums::exchange`: Stock exchanges and trading venues
  - `enums::trading`: Trading-related enums (products, validity, transaction types, etc.)
  - `enums::instruments`: Instrument types and market segments
  - `enums::interval`: Time intervals for historical data
  - `enums::gtt`: Good Till Triggered order status
- Common data types and utilities

All enums are re-exported at the module level for convenient access.
*/

pub mod enums;
pub mod errors;
pub mod response;

// Re-export main types for convenient access
pub use enums::*;
pub use errors::*;
pub use response::*;
