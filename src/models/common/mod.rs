/*!
Common types and utilities shared across all KiteConnect models.

This module provides:
- Custom error types (`KiteError`)
- Response wrapper types (`KiteResponse<T>`)
- Shared enums (Exchange, Product, Validity, etc.)
- Common data types and utilities
*/

pub mod enums;
pub mod errors;
pub mod response;

// Re-export main types for convenient access
pub use enums::*;
pub use errors::*;
pub use response::*;
