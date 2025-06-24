/*!
 * Mutual Funds module for KiteConnect API v1.0.0
 * 
 * This module contains all mutual fund-related data structures:
 * - MF instruments and fund information
 * - MF orders and SIP management
 * - MF holdings and portfolio tracking
 * - Fund performance and NAV data
 */

pub mod instruments;
pub mod orders;
pub mod holdings;
pub mod sips;

// Re-export all public types
pub use instruments::*;
pub use orders::*;
pub use holdings::*;
pub use sips::*;
