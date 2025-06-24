/*!
 * Market Data module for KiteConnect API v1.0.0
 * 
 * This module contains all market data-related data structures:
 * - Instruments and their metadata
 * - Real-time quotes and market depth
 * - Historical data (OHLCV) and candlestick patterns
 * - Market status and exchange information
 */

pub mod instruments;
pub mod quotes;
pub mod historical;
pub mod market_depth;

// Re-export all public types
pub use instruments::*;
pub use quotes::*;
pub use historical::*;
pub use market_depth::*;
