/*!
Stock exchanges supported by KiteConnect.

This module provides the `Exchange` enum which represents all major Indian stock exchanges
and international markets accessible through the KiteConnect API. Each exchange has specific
characteristics such as trading hours, instrument types, and settlement processes.

# Example

```rust
use kiteconnect_async_wasm::models::common::Exchange;

// Check exchange types
assert!(Exchange::NSE.is_equity());
assert!(Exchange::NFO.is_derivative());
assert!(Exchange::MCX.is_commodity());

// Get all available exchanges
let all_exchanges = Exchange::all();
println!("Available exchanges: {}", all_exchanges.len());
```
*/

use serde::{Deserialize, Serialize};

/// Stock exchanges supported by KiteConnect
///
/// This enum represents all major Indian stock exchanges and international markets
/// that can be accessed through the KiteConnect API. Each exchange has specific
/// trading rules, timings, and supported instrument types.
///
/// # Exchange Categories
///
/// - **Equity**: NSE, BSE, NSEIX - Cash market trading in stocks
/// - **Derivatives**: NFO, BFO - Futures and options trading
/// - **Commodity**: MCX, CDS, NCO - Commodity futures and options
/// - **Global**: GLOBAL - International markets and instruments
///
/// # Trading Hours
///
/// Most Indian exchanges operate from 9:15 AM to 3:30 PM IST on trading days,
/// with pre-market and post-market sessions available on some exchanges.
///
/// # Example
///
/// ```rust
/// use kiteconnect_async_wasm::models::common::Exchange;
///
/// let exchange = Exchange::NSE;
/// println!("Exchange: {}", exchange); // Prints: "NSE"
///
/// if exchange.is_equity() {
///     println!("This is an equity exchange");
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    /// National Stock Exchange of India (NSE)
    ///
    /// Primary equity exchange in India by volume. Supports:
    /// - Equity cash market
    /// - ETFs and mutual funds
    /// - Government securities
    #[serde(rename = "NSE")]
    NSE,

    /// Bombay Stock Exchange (BSE)
    ///
    /// Asia's oldest stock exchange. Supports:
    /// - Equity cash market  
    /// - SME platform
    /// - Government securities
    #[serde(rename = "BSE")]
    BSE,

    /// NSE Futures & Options (NFO)
    ///
    /// Derivatives segment of NSE. Supports:
    /// - Index futures and options
    /// - Stock futures and options
    /// - Currency derivatives
    #[serde(rename = "NFO")]
    NFO,

    /// Currency Derivatives Segment (CDS)
    ///
    /// Currency trading segment. Supports:
    /// - Currency futures
    /// - Currency options
    /// - Cross-currency pairs
    #[serde(rename = "CDS")]
    CDS,

    /// BSE Futures & Options (BFO)
    ///
    /// Derivatives segment of BSE. Supports:
    /// - Index derivatives
    /// - Stock derivatives
    /// - Weekly options
    #[serde(rename = "BFO")]
    BFO,

    /// Multi Commodity Exchange (MCX)
    ///
    /// India's largest commodity exchange. Supports:
    /// - Precious metals (gold, silver)
    /// - Base metals (copper, aluminum)
    /// - Energy commodities (crude oil, natural gas)
    /// - Agricultural commodities
    #[serde(rename = "MCX")]
    MCX,

    /// Global/International markets
    ///
    /// Access to international instruments and markets.
    /// Availability depends on broker permissions.
    #[serde(rename = "GLOBAL")]
    GLOBAL,

    /// National Commodity & Derivatives Exchange (NCDEX)
    ///
    /// Agricultural commodity exchange. Supports:
    /// - Agricultural futures
    /// - Agricultural options
    /// - Weather derivatives
    #[serde(rename = "NCO")]
    NCO,

    /// NSE Indices Exchange (NSEIX)
    ///
    /// Index-based trading platform. Supports:
    /// - Index trading
    /// - Specialized index products
    #[serde(rename = "NSEIX")]
    NSEIX,
}

impl Exchange {
    /// Get all supported exchanges
    pub fn all() -> Vec<Self> {
        vec![
            Exchange::NSE,
            Exchange::BSE,
            Exchange::NFO,
            Exchange::CDS,
            Exchange::BFO,
            Exchange::MCX,
            Exchange::GLOBAL,
            Exchange::NCO,
            Exchange::NSEIX,
        ]
    }

    /// Check if exchange supports equity trading
    pub fn is_equity(self) -> bool {
        matches!(self, Exchange::NSE | Exchange::BSE | Exchange::NSEIX)
    }

    /// Check if exchange supports derivatives trading
    pub fn is_derivative(self) -> bool {
        matches!(self, Exchange::NFO | Exchange::BFO)
    }

    /// Check if exchange supports commodity trading
    pub fn is_commodity(self) -> bool {
        matches!(self, Exchange::MCX | Exchange::CDS | Exchange::NCO)
    }

    /// Check if exchange is international/global
    pub fn is_global(self) -> bool {
        matches!(self, Exchange::GLOBAL)
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exchange::NSE => write!(f, "NSE"),
            Exchange::BSE => write!(f, "BSE"),
            Exchange::NFO => write!(f, "NFO"),
            Exchange::CDS => write!(f, "CDS"),
            Exchange::BFO => write!(f, "BFO"),
            Exchange::MCX => write!(f, "MCX"),
            Exchange::GLOBAL => write!(f, "GLOBAL"),
            Exchange::NCO => write!(f, "NCO"),
            Exchange::NSEIX => write!(f, "NSEIX"),
        }
    }
}
