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

    /// Validate the date range against API limits for the specified interval
    ///
    /// Checks if the requested date range exceeds the maximum allowed days
    /// for the specified interval according to KiteConnect API limits.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the date range is valid
    /// - `Err(String)` with error description if validation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     Interval::Day,
    /// );
    ///
    /// assert!(request.validate_date_range().is_ok());
    /// ```
    pub fn validate_date_range(&self) -> Result<(), String> {
        if self.to <= self.from {
            return Err("End date must be after start date".to_string());
        }

        if !self.interval.is_date_range_valid(&self.from, &self.to) {
            let duration = self.to - self.from;
            let days = duration.num_days();
            let max_days = self.interval.max_days_allowed();
            
            return Err(format!(
                "Date range of {} days exceeds maximum allowed {} days for {} interval",
                days, max_days, self.interval
            ));
        }

        Ok(())
    }

    /// Create a new historical data request with automatic validation
    ///
    /// This is an alternative constructor that validates the date range
    /// against API limits before creating the request.
    ///
    /// # Arguments
    ///
    /// * `instrument_token` - Instrument token
    /// * `from` - Start date and time
    /// * `to` - End date and time  
    /// * `interval` - Time interval
    ///
    /// # Returns
    ///
    /// - `Ok(HistoricalDataRequest)` if validation passes
    /// - `Err(String)` if validation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// let request = HistoricalDataRequest::new_validated(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     Interval::Day,
    /// )?;
    /// # Ok::<(), String>(())
    /// ```
    pub fn new_validated(
        instrument_token: u32,
        from: NaiveDateTime,
        to: NaiveDateTime,
        interval: Interval,
    ) -> Result<Self, String> {
        let request = Self::new(instrument_token, from, to, interval);
        request.validate_date_range()?;
        Ok(request)
    }

    /// Split a large date range into multiple smaller requests that respect API limits
    ///
    /// If the requested date range exceeds the maximum allowed for the interval,
    /// this method splits it into multiple smaller requests that can be made separately.
    ///
    /// # Returns
    ///
    /// Vector of `HistoricalDataRequest` objects, each within API limits
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// // Request 200 days of 5-minute data (exceeds 90-day limit)
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     NaiveDateTime::parse_from_str("2023-07-20 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     Interval::FiveMinute,
    /// );
    ///
    /// let sub_requests = request.split_into_valid_requests();
    /// println!("Split into {} requests", sub_requests.len());
    /// ```
    pub fn split_into_valid_requests(&self) -> Vec<Self> {
        let max_days = self.interval.max_days_allowed() as i64;
        let total_duration = self.to - self.from;
        let total_days = total_duration.num_days();

        if total_days <= max_days {
            // Already within limits
            return vec![self.clone()];
        }

        let mut requests = Vec::new();
        let mut current_from = self.from;

        while current_from < self.to {
            let max_to = current_from + chrono::Duration::days(max_days);
            let current_to = if max_to > self.to { self.to } else { max_to };

            let request = Self {
                instrument_token: self.instrument_token,
                from: current_from,
                to: current_to,
                interval: self.interval,
                continuous: self.continuous,
                oi: self.oi,
            };

            requests.push(request);

            // Fix data duplication: Move to next day to avoid both-inclusive overlap
            // For intraday intervals, move by appropriate time increment
            current_from = match self.interval {
                crate::models::common::Interval::Minute => current_to + chrono::Duration::minutes(1),
                crate::models::common::Interval::ThreeMinute => current_to + chrono::Duration::minutes(3),
                crate::models::common::Interval::FiveMinute => current_to + chrono::Duration::minutes(5),
                crate::models::common::Interval::TenMinute => current_to + chrono::Duration::minutes(10),
                crate::models::common::Interval::FifteenMinute => current_to + chrono::Duration::minutes(15),
                crate::models::common::Interval::ThirtyMinute => current_to + chrono::Duration::minutes(30),
                crate::models::common::Interval::SixtyMinute => current_to + chrono::Duration::hours(1),
                crate::models::common::Interval::Day => current_to + chrono::Duration::days(1),
            };
        }

        requests
    }

    /// Split a large date range into multiple smaller requests in reverse chronological order
    ///
    /// This method splits the request from newest to oldest dates, which is more efficient
    /// for early termination when data is not available for older periods.
    ///
    /// # Returns
    ///
    /// Vector of `HistoricalDataRequest` objects in reverse chronological order (newest first)
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// // Request 200 days of 5-minute data (exceeds 90-day limit)
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     NaiveDateTime::parse_from_str("2023-07-20 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     Interval::FiveMinute,
    /// );
    ///
    /// let sub_requests = request.split_into_valid_requests_reverse();
    /// println!("Split into {} requests (newest first)", sub_requests.len());
    /// ```
    pub fn split_into_valid_requests_reverse(&self) -> Vec<Self> {
        let max_days = self.interval.max_days_allowed() as i64;
        let total_duration = self.to - self.from;
        let total_days = total_duration.num_days();

        if total_days <= max_days {
            // Already within limits
            return vec![self.clone()];
        }

        let mut requests = Vec::new();
        let mut current_to = self.to;

        while current_to > self.from {
            let min_from = current_to - chrono::Duration::days(max_days);
            let current_from = if min_from < self.from { self.from } else { min_from };

            let request = Self {
                instrument_token: self.instrument_token,
                from: current_from,
                to: current_to,
                interval: self.interval,
                continuous: self.continuous,
                oi: self.oi,
            };

            requests.push(request);

            // Fix data duplication: Move to previous time increment to avoid overlap
            current_to = match self.interval {
                crate::models::common::Interval::Minute => current_from - chrono::Duration::minutes(1),
                crate::models::common::Interval::ThreeMinute => current_from - chrono::Duration::minutes(3),
                crate::models::common::Interval::FiveMinute => current_from - chrono::Duration::minutes(5),
                crate::models::common::Interval::TenMinute => current_from - chrono::Duration::minutes(10),
                crate::models::common::Interval::FifteenMinute => current_from - chrono::Duration::minutes(15),
                crate::models::common::Interval::ThirtyMinute => current_from - chrono::Duration::minutes(30),
                crate::models::common::Interval::SixtyMinute => current_from - chrono::Duration::hours(1),
                crate::models::common::Interval::Day => current_from - chrono::Duration::days(1),
            };
        }

        requests
    }

    /// Get the number of days in this request
    ///
    /// # Returns
    ///
    /// Number of days between from and to dates
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ///     Interval::Day,
    /// );
    ///
    /// println!("Request spans {} days", request.days_span());
    /// ```
    pub fn days_span(&self) -> i64 {
        (self.to - self.from).num_days()
    }

    /// Check if this request is within API limits
    ///
    /// # Returns
    ///
    /// `true` if the request is within limits, `false` otherwise
    pub fn is_within_limits(&self) -> bool {
        self.validate_date_range().is_ok()
    }

    /// Create a helper for fetching data with automatic chunking
    ///
    /// This method creates a convenient interface for fetching historical data
    /// that automatically handles large date ranges by splitting them into
    /// multiple API calls. This provides the best user experience.
    ///
    /// # Arguments
    ///
    /// * `client` - The KiteConnect client to use for API calls
    /// * `continue_on_error` - Whether to continue if a chunk fails (default: false)
    ///
    /// # Returns
    ///
    /// A `Result<HistoricalData>` containing all candles from the entire date range
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// // Create request for large date range
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    ///     NaiveDateTime::parse_from_str("2023-12-31 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    ///     Interval::Day,
    /// );
    ///
    /// // Fetch with automatic chunking - single call handles everything!
    /// let all_data = request.fetch_with_chunking(&client, false).await?;
    /// println!("Retrieved {} candles for the entire year!", all_data.candles.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Features
    ///
    /// - **Transparent**: Works exactly like a single API call from user perspective
    /// - **Automatic**: Detects when chunking is needed and handles it automatically
    /// - **Fast Path**: Uses regular API for requests within limits (no overhead)
    /// - **Robust**: Optional error recovery for failed chunks
    /// - **Progress**: Logs progress for large requests (when debug enabled)
    pub async fn fetch_with_chunking(
        self,
        client: &crate::connect::KiteConnect,
        continue_on_error: bool,
    ) -> crate::models::common::KiteResult<HistoricalData> {
        client.historical_data_chunked(self, continue_on_error).await
    }

    /// Convenience method for fetching data with default error handling
    ///
    /// This is equivalent to `fetch_with_chunking(client, false)` - stops on first error.
    /// Use this for most cases where you want all data or nothing.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    /// use kiteconnect_async_wasm::models::common::Interval;
    /// use chrono::NaiveDateTime;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let data = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    ///     NaiveDateTime::parse_from_str("2024-01-01 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    ///     Interval::Day,
    /// ).fetch(&client).await?;
    /// 
    /// println!("Got {} candles", data.candles.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch(
        self,
        client: &crate::connect::KiteConnect,
    ) -> crate::models::common::KiteResult<HistoricalData> {
        self.fetch_with_chunking(client, false).await
    }
}
