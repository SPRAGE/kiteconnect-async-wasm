/*!
Historical market data types and request structures.

This module provides strongly typed models for historical OHLCV (Open, High, Low, Close, Volume)
data retrieval from KiteConnect API. The v1.0.3 enhancement introduces structured request
parameters with precise datetime control and builder patterns.

# Key Features

- **Precise DateTime Control**: Use `NaiveDateTime` for hour/minute/second precision
- **Builder Pattern**: Fluent API for constructing requests
- **Type Safety**: Compile-time validation of parameters
- **Flexible Options**: Support for continuous data and open interest

# Example

```rust,no_run
use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
use kiteconnect_async_wasm::models::common::Interval;
use chrono::NaiveDateTime;

# fn example() -> Result<(), Box<dyn std::error::Error>> {
let request = HistoricalDataRequest::new(
    738561,  // RELIANCE instrument token
    NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    Interval::Day,
).continuous(false).with_oi(true);
# Ok(())
# }
```
*/

use crate::models::common::Interval;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Historical data request parameters (v1.0.3 enhanced)
///
/// This struct provides a structured way to request historical OHLCV data with
/// precise datetime control and optional parameters. It replaces the previous
/// approach of passing multiple string parameters.
///
/// # Benefits over Legacy API
///
/// - **Type Safety**: Compile-time validation of parameters
/// - **Precise Timing**: Hour/minute/second precision with `NaiveDateTime`
/// - **Builder Pattern**: Fluent API for optional parameters
/// - **Better IDE Support**: Autocomplete and documentation
///
/// # Date/Time Handling
///
/// The API expects datetime in IST (Indian Standard Time). When using `NaiveDateTime`,
/// ensure your timestamps represent IST. The API will treat naive datetimes as IST.
///
/// # Instrument Tokens
///
/// Instrument tokens are unique identifiers for tradable instruments. You can obtain
/// them from the instruments API or by searching instruments by symbol.
///
/// # Examples
///
/// ## Basic Daily Data Request
///
/// ```rust,no_run
/// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
/// use kiteconnect_async_wasm::models::common::Interval;
/// use chrono::NaiveDateTime;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let request = HistoricalDataRequest::new(
///     738561,  // RELIANCE
///     NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
///     NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
///     Interval::Day,
/// );
/// # Ok(())
/// # }
/// ```
///
/// ## Intraday Data with Options
///
/// ```rust,no_run
/// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
/// use kiteconnect_async_wasm::models::common::Interval;
/// use chrono::NaiveDateTime;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let request = HistoricalDataRequest::new(
///     738561,
///     NaiveDateTime::parse_from_str("2023-11-20 09:00:00", "%Y-%m-%d %H:%M:%S")?,
///     NaiveDateTime::parse_from_str("2023-11-20 16:00:00", "%Y-%m-%d %H:%M:%S")?,
///     Interval::FiveMinute,
/// ).continuous(true)  // Include pre/post market
///  .with_oi(false);   // No open interest for equity
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataRequest {
    /// Instrument token - unique identifier for the trading instrument
    ///
    /// This is a numeric identifier assigned to each tradable instrument.
    /// You can obtain instrument tokens from:
    /// - The instruments API (`client.instruments()`)
    /// - Instrument search functionality
    /// - KiteConnect documentation
    ///
    /// Example: 738561 for RELIANCE on NSE
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,

    /// From date and time (IST)
    ///
    /// The start date and time for the historical data request.
    /// Must be in Indian Standard Time (IST). For daily data,
    /// you can use any time but 09:15:00 is recommended.
    ///
    /// # Limits
    /// - Intraday data: Limited to recent days (varies by broker)
    /// - Daily data: Several years of history available
    /// - Cannot be in the future
    pub from: NaiveDateTime,

    /// To date and time (IST)
    ///
    /// The end date and time for the historical data request.
    /// Must be in Indian Standard Time (IST) and after `from` date.
    /// For daily data, you can use any time but 15:30:00 is recommended.
    pub to: NaiveDateTime,

    /// Time interval for the historical data
    ///
    /// Determines the granularity of the returned candles:
    /// - `Minute`: 1-minute candles (for intraday analysis)
    /// - `ThreeMinute`, `FiveMinute`, etc.: Various intraday intervals
    /// - `Day`: Daily candles (for longer-term analysis)
    ///
    /// Note: Shorter intervals generate more data points and may hit
    /// API rate limits faster.
    pub interval: Interval,

    /// Continuous data flag (for futures)
    ///
    /// When `true`, provides continuous contract data for futures instruments
    /// by stitching together data from different contract expiries. This gives
    /// a continuous price series without gaps from contract rollovers.
    ///
    /// - `true`: Continuous contract data (recommended for futures analysis)
    /// - `false`: Individual contract data (default)
    /// - `None`: Uses API default (false)
    ///
    /// Only applicable to futures instruments. Has no effect on equity/options.
    pub continuous: Option<bool>,

    /// Open interest data flag (for derivatives)
    ///
    /// When `true`, includes open interest data in the response for derivatives
    /// instruments (futures and options). Open interest represents the total
    /// number of outstanding contracts.
    ///
    /// - `true`: Include open interest data
    /// - `false`: Exclude open interest data (smaller response)
    /// - `None`: Uses API default (false)
    ///
    /// Only applicable to futures and options. Has no effect on equity instruments.
    pub oi: Option<bool>,
}

/// Historical candle data point
///
/// Represents a single OHLCV (Open, High, Low, Close, Volume) data point
/// for a specific time period. Each candle contains price and volume information
/// for the specified interval.
///
/// # Data Quality
///
/// - Prices are adjusted for corporate actions (splits, bonuses, dividends)
/// - Volume is in number of shares/contracts traded
/// - Open interest (if available) is the total outstanding contracts
///
/// # Example
///
/// ```rust,no_run
/// use kiteconnect_async_wasm::models::market_data::Candle;
/// use chrono::{DateTime, Utc};
///
/// # fn example(candle: Candle) {
/// println!("Date: {}", candle.date.format("%Y-%m-%d %H:%M:%S"));
/// println!("OHLC: {}/{}/{}/{}", candle.open, candle.high, candle.low, candle.close);
/// println!("Volume: {}", candle.volume);
///
/// if let Some(oi) = candle.oi {
///     println!("Open Interest: {}", oi);
/// }
///
/// // Calculate price change
/// let change = candle.close - candle.open;
/// let change_pct = (change / candle.open) * 100.0;
/// println!("Change: ₹{:.2} ({:.2}%)", change, change_pct);
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Timestamp in UTC
    ///
    /// The date and time for this candle in UTC. For daily candles,
    /// this typically represents the market close time. For intraday
    /// candles, it represents the end time of the interval.
    ///
    /// Note: Even though stored as UTC, the original data is based on IST.
    pub date: DateTime<Utc>,

    /// Opening price for the interval
    ///
    /// The first traded price during this time interval.
    /// For daily candles, this is the opening price of the trading day.
    pub open: f64,

    /// Highest price during the interval
    ///
    /// The maximum price reached during this time interval.
    pub high: f64,

    /// Lowest price during the interval
    ///
    /// The minimum price reached during this time interval.
    pub low: f64,

    /// Closing price for the interval
    ///
    /// The last traded price during this time interval.
    /// For daily candles, this is the closing price of the trading day.
    pub close: f64,

    /// Volume traded during the interval
    ///
    /// Total number of shares/contracts traded during this time interval.
    /// Higher volume often indicates stronger price moves and better liquidity.
    pub volume: u64,

    /// Open interest (for derivatives only)
    ///
    /// The total number of outstanding contracts at the end of this interval.
    /// Only available for futures and options. `None` for equity instruments.
    ///
    /// Open interest helps gauge market sentiment:
    /// - Increasing OI + Rising prices = Bullish sentiment
    /// - Increasing OI + Falling prices = Bearish sentiment
    /// - Decreasing OI = Position unwinding
    pub oi: Option<u64>,
}

/// Historical data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalData {
    /// List of candles
    pub candles: Vec<Candle>,

    /// Metadata
    pub metadata: HistoricalMetadata,
}

/// Historical data metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMetadata {
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,

    /// Trading symbol
    pub symbol: String,

    /// Interval
    pub interval: Interval,

    /// Total candles count
    pub count: usize,
}

impl HistoricalDataRequest {
    /// Create a new historical data request
    pub fn new(
        instrument_token: u32,
        from: NaiveDateTime,
        to: NaiveDateTime,
        interval: Interval,
    ) -> Self {
        Self {
            instrument_token,
            from,
            to,
            interval,
            continuous: None,
            oi: None,
        }
    }

    /// Enable continuous data for futures
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = Some(continuous);
        self
    }

    /// Include open interest data
    pub fn with_oi(mut self, oi: bool) -> Self {
        self.oi = Some(oi);
        self
    }
}

impl Candle {
    /// Get the typical price (HLC/3)
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Check if it's a bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Check if it's a bearish candle
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// Get the body size
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Get the range
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
}
