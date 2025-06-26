/*!
# KiteConnect v1.0.3 Data Models

This module provides comprehensive typed models for all KiteConnect API operations.
The models are organized into domain-specific submodules and offer both legacy
compatibility and enhanced v1.0.3 features.

## Model Organization

- **`common`**: Shared types, enums, errors, and response wrappers
- **`auth`**: Authentication, sessions, user profiles, and margins
- **`orders`**: Order management, trades, and order-related types
- **`portfolio`**: Holdings, positions, and portfolio conversions
- **`market_data`**: Instruments, quotes, market depth, and historical data
- **`mutual_funds`**: MF orders, instruments, SIPs, and holdings
- **`gtt`**: GTT (Good Till Triggered) orders and triggers

## What's New in v1.0.3

### Enhanced Historical Data API
- **Structured Requests**: New `HistoricalDataRequest` with builder pattern
- **Precise DateTime**: `NaiveDateTime` support for hour/minute/second precision
- **Type Safety**: Compile-time validation of parameters

### Dual Serde Support
- **Flexible Intervals**: Accept both strings ("day") and integers (0) for `Interval`
- **Consistent Output**: Always serialize as strings for API compatibility
- **Backward Compatible**: Existing code continues to work

### Organized Enum System
- **Modular Structure**: Enums split into logical submodules
- **Better Maintainability**: Easier navigation and development
- **Full Compatibility**: All imports work through re-exports

```rust
use kiteconnect_async_wasm::models::prelude::*;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Current: Create typed models from JSON responses
let json_response = r#"{
    "account_id": "ABCD123",
    "tradingsymbol": "RELIANCE",
    "exchange": "NSE",
    "isin": "INE002A01018",
    "product": "CNC",
    "instrument_token": 738561,
    "quantity": 100,
    "t1_quantity": 0,
    "realised_quantity": 100,
    "authorised_quantity": 0,
    "opening_quantity": 100,
    "collateral_quantity": 0,
    "collateral_type": null,
    "collateral_update_quantity": 0,
    "discrepancy": false,
    "average_price": 2400.0,
    "last_price": 2450.0,
    "close_price": 2445.0,
    "price_change": 5.0,
    "pnl": 5000.0,
    "day_change": 5.0,
    "day_change_percentage": 0.2,
    "used_quantity": 0
}"#;
let holding: Holding = serde_json::from_str(&json_response)?;

// Future: Direct typed API methods (roadmap)
// let holdings: Vec<Holding> = kite.holdings_typed().await?;
# Ok(())
# }
```

## Error Handling

All models include comprehensive error types with `KiteError`:

```rust
use kiteconnect_async_wasm::models::KiteError;

// Example error handling with models
let error = KiteError::Api {
    status: "400".to_string(),
    message: "Invalid parameters".to_string(),
    error_type: Some("BadRequest".to_string()),
};

match error {
    KiteError::Authentication(msg) => { /* handle auth error */ },
    KiteError::Api { status, message, .. } => { /* handle API error */ },
    KiteError::Json(err) => { /* handle JSON parsing error */ },
    _ => { /* handle other errors */ },
}
```
*/

// Core common types (always available)
pub mod common;

// Phase 2: Authentication models (completed)
pub mod auth;

// Phase 3: Orders models (completed)
pub mod orders;

// Phase 4: Portfolio models (completed)
pub mod portfolio;

// Phase 5: Market data models (completed)
pub mod market_data;

// Phase 6: Mutual funds models (completed)
pub mod mutual_funds;

// Phase 7: GTT models (completed)
pub mod gtt;

// Public API - re-export main types for convenience
pub use common::*;

/// Prelude module for convenient imports
///
/// Import commonly used types with a single `use` statement
///
/// ```rust
/// use kiteconnect_async_wasm::models::prelude::*;
/// ```
pub mod prelude {

    // Common types
    pub use super::common::{
        // Common enums
        Exchange,
        GttStatus,
        InstrumentType,
        Interval,
        // Error types
        KiteError,
        // Response types
        KiteResponse,
        KiteResult,

        OrderType,
        Product,
        RawResponse,
        Status,

        TransactionType,
        Validity,
        Variety,
    };

    // Authentication types
    pub use super::auth::{
        AccountStatus,

        FundTransaction,
        LoginUrlConfig,
        LogoutResponse,

        // Margin data
        MarginData,
        MarginFunds,
        MarginUtilisation,
        RequestToken,
        SegmentMargin,
        // Session management
        SessionData,
        SessionMeta,
        TradingSegment,
        UserMeta,
        // User profiles
        UserProfile,
        UserType,
    };

    // Order types
    pub use super::orders::{
        BracketOrderBuilder,
        BracketOrderParams,
        BracketOrderResponse,
        CoverOrderParams,
        CoverOrderResponse,
        // Order data
        Order,
        OrderBook,
        OrderBuilder,
        OrderCancellation,
        OrderHistory,
        OrderHistoryEntry,
        OrderMeta,

        // Order operations
        OrderModification,
        OrderModifyParams,

        // Order parameters and builders
        OrderParams,
        OrderResponse,

        OrderStatus,
        // Order history and trades
        Trade,
        TradeBook,
        TradeHistory,
    };

    // Portfolio types
    pub use super::portfolio::{
        BulkConversionRequest,
        BulkConversionResponse,
        ConversionRequest,
        ConversionResponse,
        ConversionResult,
        // Conversions
        ConversionType,
        // Holdings
        Holding,
        HoldingsSummary,
        PortfolioProfile,

        // Positions
        Position,
        PositionConversionRequest,

        PositionType,
        PositionsSummary,
    };

    // Market data types
    pub use super::market_data::{
        Candle,
        DepthItem,
        DepthLevel,
        HistoricalData,
        // Historical data
        HistoricalDataRequest,
        HistoricalMetadata,
        HistoricalQuote,
        // Instruments
        Instrument,
        InstrumentLookup,

        InstrumentSearch,
        Level2Data,

        // Market depth
        MarketDepth,
        MarketDepthFull,
        MarketState,
        MarketStatus,
        // Quotes
        Quote,
        QuoteRequest,
        LTP,
        OHLC,
        OHLCV,
    };

    // Mutual funds types
    pub use super::mutual_funds::{
        // MF holdings
        MFHolding,
        MFHoldings,
        // MF instruments
        MFInstrument,
        MFInstrumentSearch,

        // MF orders
        MFOrder,
        MFOrderParams,
        MFOrderResponse,
        MFOrderStatus,
        MFOrders,

        MFPerformance,
        MFPortfolioSummary,

        SIPFrequency,
        SIPModifyParams,
        SIPParams,
        SIPResponse,
        SIPStatus,
        SIPStepUp,
        SIPs,
        // SIPs
        SIP,
    };

    // GTT types
    pub use super::gtt::{
        BracketGTTBuilder,

        // GTT builders
        GTTBuilder,
        GTTCondition,
        GTTConditionBuilder,
        GTTCreateParams,
        GTTModifyParams,
        GTTOrderBuilder,
        GTTOrderParams,
        GTTOrderResult,
        GTTResponse,
        // GTT templates
        GTTTemplate,
        GTTTriggerType,
        GTTs,

        StopLossGTTBuilder,
        TargetGTTBuilder,
        // GTT triggers
        GTT,
    };
}
