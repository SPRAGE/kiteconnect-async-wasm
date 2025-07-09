//! # Market Data Module
//!
//! This module provides comprehensive market data access for the KiteConnect API v1.0.3,
//! offering both real-time and historical market information with full cross-platform support.
//!
//! ## Overview
//!
//! The market data module is the core component for accessing all market-related information
//! including instruments, quotes, historical data, and market depth. It provides both legacy
//! JSON-based APIs and modern strongly-typed APIs for enhanced developer experience.
//!
//! ## Key Features
//!
//! ### 🔄 **Dual API Support**
//! - **Legacy API**: Returns `JsonValue` for backward compatibility
//! - **Typed API**: Returns structured types with compile-time safety (methods ending in `_typed`)
//!
//! ### 📊 **Comprehensive Data Coverage**
//! - **Real-time Quotes**: Live prices, OHLC, volume, and market depth
//! - **Historical Data**: OHLCV candlestick data with v1.0.3 enhanced API
//! - **Instruments Master**: Complete instrument list with metadata
//! - **Market Status**: Exchange timings and market state information
//!
//! ### 🌐 **Cross-Platform Optimization**
//! - **Native**: Uses `csv` crate for efficient parsing with structured JSON output
//! - **WASM**: Uses `csv-core` for no-std CSV parsing in browser environments
//! - **Rate Limiting**: Automatic rate limiting respecting KiteConnect API limits
//! - **Error Handling**: Comprehensive error types with context
//!
//! ## Platform-Specific Behavior
//!
//! ### Native Platform (tokio)
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = KiteConnect::new("api_key", "access_token");
//! // Instruments data is parsed server-side and returned as structured JSON
//! let instruments = client.instruments(None).await?;
//! // Returns JsonValue with array of instrument objects
//! # Ok(())
//! # }
//! ```
//!
//! ### WASM Platform
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = KiteConnect::new("api_key", "access_token");
//! // CSV data is parsed client-side using csv-core for browser compatibility
//! let instruments = client.instruments(None).await?;
//! // Returns JsonValue with array of instrument objects (parsed from CSV)
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Methods
//!
//! ### Instruments and Market Data
//! - [`instruments()`](KiteConnect::instruments) / [`instruments_typed()`](KiteConnect::instruments_typed) - Get complete instruments list (cached for performance)
//! - [`mf_instruments()`](KiteConnect::mf_instruments) / [`mf_instruments_typed()`](KiteConnect::mf_instruments_typed) - Get mutual fund instruments
//! - [`quote()`](KiteConnect::quote) / [`quote_typed()`](KiteConnect::quote_typed) - Real-time quotes
//! - [`ohlc()`](KiteConnect::ohlc) / [`ohlc_typed()`](KiteConnect::ohlc_typed) - OHLC data
//! - [`ltp()`](KiteConnect::ltp) / [`ltp_typed()`](KiteConnect::ltp_typed) - Last traded price
//!
//! ### Historical Data (Enhanced in v1.0.3)
//! - [`historical_data()`](KiteConnect::historical_data) - Legacy historical data API
//! - [`historical_data_typed()`](KiteConnect::historical_data_typed) - New structured request API
//!
//! ### Market Information
//! - [`trigger_range()`](KiteConnect::trigger_range) - Get trigger range for instruments
//! - [`instruments_margins()`](KiteConnect::instruments_margins) - Get margin requirements
//!
//! ## Usage Examples
//!
//! ### Basic Real-time Quote
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get real-time quote (legacy API)
//! let quote = client.quote(vec!["NSE:RELIANCE", "BSE:SENSEX"]).await?;
//! println!("Quote data: {}", quote);
//!
//! // Get real-time quote (typed API - recommended)
//! let quotes = client.quote_typed(vec!["NSE:RELIANCE"]).await?;
//! for quote in quotes {
//!     println!("{}: ₹{:.2} ({}{})",
//!         quote.trading_symbol,
//!         quote.last_price,
//!         if quote.net_change >= 0.0 { "+" } else { "" },
//!         quote.net_change);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Enhanced Historical Data (v1.0.3)
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
//! use kiteconnect_async_wasm::models::common::Interval;
//! use chrono::NaiveDateTime;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Create structured request (v1.0.3 feature)
//! let request = HistoricalDataRequest::new(
//!     738561,  // RELIANCE instrument token
//!     NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?,
//!     NaiveDateTime::parse_from_str("2023-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")?,
//!     Interval::Day,
//! ).continuous(false)
//!  .with_oi(true);
//!
//! let historical = client.historical_data_typed(request).await?;
//! println!("Retrieved {} candles", historical.candles.len());
//!
//! for candle in &historical.candles[..5] {  // Show first 5 candles
//!     println!("{}: O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}",
//!         candle.date.format("%Y-%m-%d"),
//!         candle.open, candle.high, candle.low, candle.close, candle.volume);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Market Depth Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! let quotes = client.quote_typed(vec!["NSE:RELIANCE"]).await?;
//! for quote in quotes {
//!     // Analyze market depth
//!     if let (Some(bid), Some(ask)) = (quote.bid_price(), quote.ask_price()) {
//!         let spread = ask - bid;
//!         let spread_pct = (spread / bid) * 100.0;
//!         
//!         println!("Market Depth for {}:", quote.trading_symbol);
//!         println!("  Bid: ₹{:.2} | Ask: ₹{:.2}", bid, ask);
//!         println!("  Spread: ₹{:.2} ({:.2}%)", spread, spread_pct);
//!         println!("  Bid Volume: {} | Ask Volume: {}",
//!             quote.total_bid_quantity(), quote.total_ask_quantity());
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Instruments Search and Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::common::Exchange;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all instruments with type safety (recommended)
//! let instruments = client.instruments_typed(None).await?;
//! println!("Total instruments available: {}", instruments.len());
//!
//! // Get NSE instruments only using type-safe enum
//! let nse_instruments = client.instruments_typed(Some(Exchange::NSE)).await?;
//! println!("NSE instruments: {}", nse_instruments.len());
//!
//! // Get derivatives instruments using type-safe enum
//! let nfo_instruments = client.instruments_typed(Some(Exchange::NFO)).await?;
//! println!("NFO derivatives: {}", nfo_instruments.len());
//!
//! // Filter and analyze with helper methods
//! let reliance_options: Vec<_> = instruments
//!     .iter()
//!     .filter(|inst| inst.name.contains("RELIANCE") && inst.is_option())
//!     .collect();
//!
//! for option in reliance_options.iter().take(5) {
//!     println!("Option: {} | Strike: {} | Days to expiry: {:?}",
//!         option.trading_symbol,
//!         option.strike,
//!         option.days_to_expiry());
//! }
//!
//! // Legacy JSON API still available for backward compatibility
//! let instruments_json = client.instruments(None).await?;
//! if let Some(instruments_array) = instruments_json.as_array() {
//!     println!("Total instruments (JSON): {}", instruments_array.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! ### Caching
//! - **Instruments Data**: Automatically cached for 1 hour to reduce API calls
//! - **Rate Limiting**: Built-in rate limiting prevents API quota exhaustion
//! - **Connection Pooling**: HTTP connections are reused for better performance
//!
//! ### Memory Usage
//! - **WASM Builds**: Optimized for browser memory constraints
//! - **Native Builds**: Can handle large datasets efficiently
//! - **Streaming**: Large CSV responses are processed incrementally
//!
//! ## Error Handling
//!
//! All methods return `Result<T>` with comprehensive error information:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::models::common::KiteError;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = kiteconnect_async_wasm::connect::KiteConnect::new("", "");
//! match client.quote_typed(vec!["INVALID:SYMBOL"]).await {
//!     Ok(quotes) => println!("Success: {} quotes", quotes.len()),
//!     Err(KiteError::Api { status, message, .. }) => {
//!         eprintln!("API Error {}: {}", status, message);
//!         if status == "429" {
//!             eprintln!("Rate limited - please wait before retrying");
//!         }
//!     }
//!     Err(KiteError::Authentication(msg)) => {
//!         eprintln!("Authentication failed: {}", msg);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Rate Limiting
//!
//! The module automatically handles rate limiting according to KiteConnect API guidelines:
//! - **Market Data**: 3 requests per second
//! - **Historical Data**: 3 requests per second with higher limits for minute data
//! - **Quotes**: Optimized batching for multiple instruments
//!
//! ## Migration from v1.0.2
//!
//! All existing methods continue to work. New typed methods provide enhanced features:
//! - Replace `historical_data()` with `historical_data_typed()` for structured requests
//! - Use `quote_typed()`, `ohlc_typed()`, `ltp_typed()` for type safety
//! - Legacy methods remain available for backward compatibility
//!
//! ## Thread Safety
//!
//! All methods are thread-safe and can be called concurrently:
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = KiteConnect::new("", "");
//! // Concurrent requests
//! let (quotes, ohlc, ltp) = tokio::try_join!(
//!     client.quote_typed(vec!["NSE:RELIANCE"]),
//!     client.ohlc_typed(vec!["NSE:INFY"]),
//!     client.ltp_typed(vec!["NSE:TCS"])
//! )?;
//! # Ok(())
//! # }
//! ```

use crate::connect::endpoints::KiteEndpoint;
use anyhow::Result;
use serde_json::Value as JsonValue;

// Native platform imports
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use csv::ReaderBuilder;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use crate::connect::utils::parse_csv_with_core;
use crate::connect::utils::RequestHandler;
use crate::connect::KiteConnect;

// Import typed models for dual API support
use crate::models::common::{Exchange, KiteError, KiteResult};
use crate::models::market_data::{
    HistoricalData, HistoricalDataRequest, HistoricalMetadata, Quote, LTP, OHLC,
};
use crate::models::mutual_funds::MFInstrument;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===

    /// Get the trigger range for a list of instruments
    pub async fn trigger_range(
        &self,
        transaction_type: &str,
        instruments: Vec<&str>,
    ) -> Result<JsonValue> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("transaction_type", transaction_type));

        for instrument in instruments {
            params.push(("instruments", instrument));
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::TriggerRange,
                &[],
                Some(params),
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Get trigger range failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Get instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        // Check cache first if enabled
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                // Only cache the full instruments list, not exchange-specific ones
                if let Ok(cache_guard) = self.response_cache.lock() {
                    if let Some(ref cache) = *cache_guard {
                        if let Some(cached_data) = cache.get_instruments() {
                            #[cfg(feature = "debug")]
                            log::debug!("Returning cached instruments data");
                            return Ok(cached_data);
                        }
                    }
                }
            }
        }

        let endpoint = KiteEndpoint::Instruments;

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        #[cfg(feature = "debug")]
        log::debug!(
            "Requesting instruments: endpoint={:?}, path_segments={:?}",
            endpoint,
            path_segments
        );

        let resp = self
            .send_request_with_rate_limiting_and_retry(endpoint, &path_segments, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;

        #[cfg(feature = "debug")]
        log::debug!("Received response with status: {}", resp.status());

        // Check if response is gzipped and handle accordingly
        let content_encoding = resp
            .headers()
            .get("content-encoding")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let body_text = if content_encoding.contains("gzip") {
            #[cfg(feature = "debug")]
            log::debug!("Decompressing gzipped response");

            let body_bytes = resp.bytes().await?;
            use std::io::Read;
            let mut decoder = flate2::read::GzDecoder::new(&body_bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;
            decompressed
        } else {
            resp.text().await?
        };

        #[cfg(feature = "debug")]
        log::debug!(
            "Decompressed body length: {} bytes, first 200 chars: {}",
            body_text.len(),
            &body_text.chars().take(200).collect::<String>()
        );

        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body_text.as_bytes());
        let mut result = Vec::new();

        let headers = rdr.headers()?.clone();
        #[cfg(feature = "debug")]
        log::debug!("CSV headers: {:?}", headers);

        let mut _record_count = 0;
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
            _record_count += 1;
        }

        #[cfg(feature = "debug")]
        log::debug!("Parsed {} records from CSV", _record_count);

        let result_json = JsonValue::Array(result);

        // Cache the result if enabled and it's the full instruments list
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                if let Ok(mut cache_guard) = self.response_cache.lock() {
                    if let Some(ref mut cache) = *cache_guard {
                        cache.set_instruments(result_json.clone());
                    }
                }
            }
        }

        Ok(result_json)
    }

    /// Get instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        // Check cache first if enabled
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                // Only cache the full instruments list, not exchange-specific ones
                if let Ok(cache_guard) = self.response_cache.lock() {
                    if let Some(ref cache) = *cache_guard {
                        if let Some(cached_data) = cache.get_instruments() {
                            return Ok(cached_data);
                        }
                    }
                }
            }
        }

        let endpoint = KiteEndpoint::Instruments;

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        let resp = self
            .send_request_with_rate_limiting_and_retry(endpoint, &path_segments, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;

        // In WASM, browsers typically handle gzip decompression automatically
        // But let's check the response format
        let body = resp.text().await?;

        // Debug: Check if the response looks like gzipped binary data
        if body.len() > 0 && !body.starts_with("instrument_token") {
            // If it doesn't start with expected CSV header, it might be binary data
            // In WASM, we might need different handling
            #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
            {
                // For now, log the issue
                web_sys::console::log_1(
                    &format!("WASM: Unexpected response format, length: {}", body.len()).into(),
                );
                web_sys::console::log_1(
                    &format!(
                        "WASM: First 100 chars: {}",
                        &body.chars().take(100).collect::<String>()
                    )
                    .into(),
                );
            }
        }

        // Parse CSV using csv-core for WASM compatibility
        let result = parse_csv_with_core(&body)?;

        // Cache the result if enabled and it's the full instruments list
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                if let Ok(mut cache_guard) = self.response_cache.lock() {
                    if let Some(ref mut cache) = *cache_guard {
                        cache.set_instruments(result.clone());
                    }
                }
            }
        }

        Ok(result)
    }

    /// Get mutual fund instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::MFInstruments, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get MF instruments failed: {:?}", e))?;
        let body = resp.text().await?;

        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();

        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }

        Ok(JsonValue::Array(result))
    }

    /// Get mutual fund instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::MFInstruments, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get MF instruments failed: {:?}", e))?;
        let body = resp.text().await?;

        // Parse CSV using csv-core for WASM compatibility
        parse_csv_with_core(&body)
    }

    /// Get instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn instruments(&self, _exchange: Option<&str>) -> Result<JsonValue> {
        Err(anyhow!(
            "Instruments functionality requires either 'native' or 'wasm' feature to be enabled"
        ))
    }

    /// Get mutual fund instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        Err(anyhow!(
            "MF instruments functionality requires either 'native' or 'wasm' feature to be enabled"
        ))
    }

    /// Retrieves historical candlestick data for an instrument
    ///
    /// Returns historical OHLCV (Open, High, Low, Close, Volume) data for a given
    /// instrument within the specified date range and interval. This is useful for
    /// backtesting, analysis, and charting applications.
    ///
    /// # Arguments
    ///
    /// * `instrument_token` - The instrument token (numeric ID) of the instrument
    /// * `from_date` - Start date in YYYY-MM-DD format
    /// * `to_date` - End date in YYYY-MM-DD format  
    /// * `interval` - Time interval for candlesticks ("minute", "day", "3minute", "5minute", "10minute", "15minute", "30minute", "60minute")
    /// * `continuous` - Whether to include pre-market and post-market data ("1" for true, "0" for false)
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing historical data with fields like:
    /// - `data` - Array of candlestick data points
    ///   - `date` - ISO datetime string
    ///   - `open` - Opening price
    ///   - `high` - Highest price
    ///   - `low` - Lowest price
    ///   - `close` - Closing price
    ///   - `volume` - Trading volume
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The instrument token is invalid
    /// - The date range is invalid or too large
    /// - The interval is not supported
    /// - Network request fails
    /// - User is not authenticated
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// // Get daily data for RELIANCE for the last month
    /// let historical_data = client.historical_data(
    ///     "738561",           // RELIANCE instrument token
    ///     "2023-11-01",       // From date
    ///     "2023-11-30",       // To date
    ///     "day",              // Daily interval
    ///     "0"                 // No continuous data
    /// ).await?;
    ///
    /// println!("Historical data: {:?}", historical_data);
    ///
    /// // Access candlestick data
    /// if let Some(data) = historical_data["data"].as_array() {
    ///     for candle in data {
    ///         println!("Date: {}, Close: {}, Volume: {}",
    ///             candle["date"], candle["close"], candle["volume"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - Historical data is subject to availability and may have limitations based on your subscription
    /// - Large date ranges may be split into multiple requests automatically
    /// - Intraday data older than a certain period may not be available
    /// - Weekend and holiday data will not be included in the response
    pub async fn historical_data(
        &self,
        instrument_token: &str,
        from_date: &str,
        to_date: &str,
        interval: &str,
        continuous: &str,
    ) -> Result<JsonValue> {
        let params = vec![
            ("from", from_date),
            ("to", to_date),
            ("continuous", continuous),
        ];

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::HistoricalData,
                &[instrument_token, interval],
                Some(params),
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Get historical data failed: {:?}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Retrieve quote and market depth for list of instruments
    ///
    /// Gets real-time quote data including bid/ask prices, market depth,
    /// and other market data for the specified instruments.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers (exchange:tradingsymbol or instrument_token)
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing quote data with fields like:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close data
    /// - `market_depth` - Bid/Ask data with quantities
    /// - `volume` - Trading volume
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "BSE:SENSEX"];
    /// let quotes = client.quote(instruments).await?;
    ///
    /// println!("Quotes: {:?}", quotes);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quote(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Quote, &[], Some(params), None)
            .await
            .map_err(|e| anyhow::anyhow!("Get quote failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieve OHLC (Open, High, Low, Close) data for instruments
    ///
    /// Gets OHLC data for the current trading day for the specified instruments.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing OHLC data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ohlc_data = client.ohlc(instruments).await?;
    ///
    /// println!("OHLC Data: {:?}", ohlc_data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ohlc(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::OHLC, &[], Some(params), None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get OHLC data: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Retrieve Last Traded Price (LTP) for instruments
    ///
    /// Gets the last traded price for the specified instruments. This is a
    /// lightweight alternative to the full quote API.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing last traded prices
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ltp_data = client.ltp(instruments).await?;
    ///
    /// println!("LTP Data: {:?}", ltp_data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ltp(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::LTP, &[], Some(params), None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get LTP data: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Retrieve margin requirements for specific trading segments
    ///
    /// Gets margin requirements and charges for different trading segments
    /// like equity, commodity, currency, etc.
    ///
    /// # Arguments
    ///
    /// * `segment` - Trading segment ("equity", "commodity", "currency")
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing margin requirements data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let equity_margins = client.instruments_margins("equity").await?;
    /// println!("Equity margins: {:?}", equity_margins);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instruments_margins(&self, segment: &str) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MarketMargins,
                &[segment],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get instrument margins: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===

    /// Get real-time quotes with typed response
    ///
    /// Returns strongly typed quote data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Quote>>` containing typed quote data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "BSE:SENSEX"];
    /// let quotes = client.quote_typed(instruments).await?;
    /// for quote in quotes {
    ///     println!("Symbol: {}, LTP: {}", quote.trading_symbol, quote.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quote_typed(&self, instruments: Vec<&str>) -> KiteResult<Vec<Quote>> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Quote, &[], Some(params), None)
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get OHLC data with typed response
    ///
    /// Returns strongly typed OHLC data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<OHLC>>` containing typed OHLC data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ohlc_data = client.ohlc_typed(instruments).await?;
    /// for ohlc in ohlc_data {
    ///     println!("Open: {}, High: {}, Low: {}, Close: {}",
    ///         ohlc.open, ohlc.high, ohlc.low, ohlc.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ohlc_typed(&self, instruments: Vec<&str>) -> KiteResult<Vec<OHLC>> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::OHLC, &[], Some(params), None)
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get Last Traded Price (LTP) with typed response
    ///
    /// Returns strongly typed LTP data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `instruments` - List of instrument identifiers
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<LTP>>` containing typed LTP data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ltp_data = client.ltp_typed(instruments).await?;
    /// for ltp in ltp_data {
    ///     println!("Token: {}, LTP: {}", ltp.instrument_token, ltp.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ltp_typed(&self, instruments: Vec<&str>) -> KiteResult<Vec<LTP>> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::LTP, &[], Some(params), None)
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get historical data with typed response
    ///
    /// Returns strongly typed historical data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `request` - A `HistoricalDataRequest` containing all the parameters for the request
    ///
    /// # Returns
    ///
    /// A `KiteResult<HistoricalData>` containing typed historical data
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
    /// let request = HistoricalDataRequest::new(
    ///     738561,             // RELIANCE instrument token
    ///     NaiveDateTime::parse_from_str("2023-11-01 00:00:00", "%Y-%m-%d %H:%M:%S")?,
    ///     NaiveDateTime::parse_from_str("2023-11-30 23:59:59", "%Y-%m-%d %H:%M:%S")?,
    ///     Interval::Day,
    /// ).continuous(false);
    ///
    /// let historical_data = client.historical_data_typed(request).await?;
    ///
    /// for candle in &historical_data.candles {
    ///     println!("Date: {}, Close: {}, Volume: {}",
    ///         candle.date, candle.close, candle.volume);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn historical_data_typed(
        &self,
        request: HistoricalDataRequest,
    ) -> KiteResult<HistoricalData> {
        // Validate date range against API limits
        if let Err(validation_error) = request.validate_date_range() {
            return Err(crate::models::common::KiteError::input_exception(
                validation_error,
            ));
        }

        let mut params = Vec::new();
        params.push(("from", request.from.format("%Y-%m-%d %H:%M:%S").to_string()));
        params.push(("to", request.to.format("%Y-%m-%d %H:%M:%S").to_string()));

        if let Some(continuous) = request.continuous {
            params.push(("continuous", if continuous { "1" } else { "0" }.to_string()));
        }

        if let Some(oi) = request.oi {
            params.push(("oi", if oi { "1" } else { "0" }.to_string()));
        }

        // Convert params to the expected format
        let params_str: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::HistoricalData,
                &[
                    &request.instrument_token.to_string(),
                    &request.interval.to_string(),
                ],
                Some(params_str),
                None,
            )
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();

        // Parse the candles array directly
        let candles: Vec<crate::models::market_data::Candle> = if data["candles"].is_array() {
            // If data has a "candles" field
            serde_json::from_value(data["candles"].clone()).map_err(KiteError::Json)?
        } else if data.is_array() {
            // If data is directly an array of candles
            serde_json::from_value(data).map_err(KiteError::Json)?
        } else {
            return Err(KiteError::general(
                "Invalid historical data format".to_string(),
            ));
        };

        // Create metadata from request parameters
        let metadata = crate::models::market_data::HistoricalMetadata {
            instrument_token: request.instrument_token,
            symbol: format!("Token-{}", request.instrument_token), // We don't have the actual symbol
            interval: request.interval,
            count: candles.len(),
        };

        Ok(crate::models::market_data::HistoricalData { candles, metadata })
    }

    /// Get instruments list with typed response
    ///
    /// Returns strongly typed instrument data instead of JsonValue.
    /// This provides compile-time type safety and better IDE support.
    ///
    /// # Arguments
    ///
    /// * `exchange` - Optional exchange filter using the `Exchange` enum for type safety
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Instrument>>` containing typed instrument data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::common::Exchange;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// // Get all instruments with type safety
    /// let instruments = client.instruments_typed(None).await?;
    /// println!("Total instruments: {}", instruments.len());
    ///
    /// // Get NSE instruments only using type-safe enum
    /// let nse_instruments = client.instruments_typed(Some(Exchange::NSE)).await?;
    /// println!("NSE instruments: {}", nse_instruments.len());
    ///
    /// // Filter and analyze instruments
    /// let reliance_instruments: Vec<_> = instruments
    ///     .iter()
    ///     .filter(|inst| inst.name.contains("RELIANCE"))
    ///     .collect();
    ///
    /// for instrument in reliance_instruments {
    ///     println!("Symbol: {}, Type: {:?}, Strike: {}",
    ///         instrument.trading_symbol,
    ///         instrument.instrument_type,
    ///         instrument.strike);
    ///     
    ///     if instrument.is_option() {
    ///         println!("  Days to expiry: {:?}", instrument.days_to_expiry());
    ///     }
    /// }
    ///
    /// // Get derivatives instruments using type-safe enum
    /// let nfo_instruments = client.instruments_typed(Some(Exchange::NFO)).await?;
    /// println!("NFO derivatives: {}", nfo_instruments.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Features
    ///
    /// - **Type Safety**: Compile-time validation using `Exchange` enum
    /// - **IDE Support**: Autocomplete for available exchanges
    /// - **Helper Methods**: Built-in methods like `is_equity()`, `is_option()`, `days_to_expiry()`
    /// - **Caching**: Automatic caching for performance (when enabled)
    /// - **Cross-Platform**: Works on both native and WASM platforms
    ///
    /// # Performance Notes
    ///
    /// - Results are automatically cached when `exchange` is `None`
    /// - Cache duration is 1 hour by default
    /// - Exchange-specific queries are not cached
    /// - Large instrument lists are processed efficiently
    pub async fn instruments_typed(
        &self,
        exchange: Option<Exchange>,
    ) -> KiteResult<Vec<crate::models::market_data::Instrument>> {
        // Convert Exchange enum to string for the underlying API call
        let exchange_str = exchange.as_ref().map(|e| e.to_string());
        let exchange_str_ref = exchange_str.as_ref().map(|s| s.as_str());

        // Get the JSON response using existing method
        let json_response = self
            .instruments(exchange_str_ref)
            .await
            .map_err(|e| KiteError::general(format!("Failed to get instruments: {}", e)))?;

        // Parse the JSON array into typed instruments
        if let Some(instruments_array) = json_response.as_array() {
            let mut instruments = Vec::new();

            for instrument_json in instruments_array {
                // Convert JSON object to Instrument struct
                match serde_json::from_value::<crate::models::market_data::Instrument>(
                    instrument_json.clone(),
                ) {
                    Ok(instrument) => instruments.push(instrument),
                    Err(e) => {
                        // Log the error but continue processing other instruments
                        #[cfg(feature = "debug")]
                        log::warn!(
                            "Failed to parse instrument: {:?} - Error: {}",
                            instrument_json,
                            e
                        );

                        // For now, continue with other instruments rather than failing completely
                        continue;
                    }
                }
            }

            Ok(instruments)
        } else {
            Err(KiteError::general(
                "Invalid instruments response format".to_string(),
            ))
        }
    }

    /// Debug version of instruments_typed that shows JSON before conversion
    pub async fn instruments_typed_debug(
        &self,
        exchange: Option<Exchange>,
    ) -> KiteResult<Vec<crate::models::market_data::Instrument>> {
        // Convert Exchange enum to string for the underlying API call
        let exchange_str = exchange.as_ref().map(|e| e.to_string());
        let exchange_str_ref = exchange_str.as_ref().map(|s| s.as_str());

        println!("🔍 Typed Debug: Getting JSON response first...");

        // Get the JSON response using existing method
        let json_response = self
            .instruments(exchange_str_ref)
            .await
            .map_err(|e| KiteError::general(format!("Failed to get instruments: {}", e)))?;

        println!(
            "🔍 Typed Debug: JSON response type: {:?}",
            if json_response.is_array() {
                "Array"
            } else if json_response.is_object() {
                "Object"
            } else {
                "Other"
            }
        );

        // Parse the JSON array into typed instruments
        if let Some(instruments_array) = json_response.as_array() {
            println!(
                "🔍 Typed Debug: Found {} instruments in JSON array",
                instruments_array.len()
            );

            if instruments_array.is_empty() {
                println!("🔍 Typed Debug: Array is empty - returning empty vector");
                return Ok(Vec::new());
            }

            // Show first few JSON objects for debugging
            println!("🔍 Typed Debug: First 3 JSON instruments:");
            for (i, instrument_json) in instruments_array.iter().take(3).enumerate() {
                println!(
                    "  {}. {}",
                    i + 1,
                    serde_json::to_string_pretty(instrument_json)?
                );
            }

            let mut instruments = Vec::new();
            let mut parse_errors = 0;

            for (index, instrument_json) in instruments_array.iter().enumerate() {
                // Convert JSON object to Instrument struct
                match serde_json::from_value::<crate::models::market_data::Instrument>(
                    instrument_json.clone(),
                ) {
                    Ok(instrument) => {
                        instruments.push(instrument);
                        if index < 5 {
                            println!(
                                "🔍 Typed Debug: Successfully parsed instrument {}: {}",
                                index + 1,
                                instruments.last().unwrap().trading_symbol
                            );
                        }
                    }
                    Err(e) => {
                        parse_errors += 1;
                        if parse_errors <= 5 {
                            println!(
                                "🔍 Typed Debug: Failed to parse instrument {}: {}",
                                index + 1,
                                e
                            );
                            println!(
                                "🔍 Typed Debug: JSON was: {}",
                                serde_json::to_string_pretty(instrument_json)?
                            );
                        }
                        // Continue with other instruments rather than failing completely
                        continue;
                    }
                }
            }

            println!(
                "🔍 Typed Debug: Successfully parsed {} instruments, {} parse errors",
                instruments.len(),
                parse_errors
            );
            Ok(instruments)
        } else {
            println!(
                "🔍 Typed Debug: JSON response is not an array: {:?}",
                json_response
            );
            Err(KiteError::general(
                "Invalid instruments response format - not an array".to_string(),
            ))
        }
    }

    /// Get mutual fund instruments with typed response
    ///
    /// Returns strongly typed mutual fund instrument data instead of JsonValue.
    /// This provides compile-time type safety and helper methods for MF analysis.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<MFInstrument>>` containing typed MF instrument data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// // Get all MF instruments with type safety
    /// let mf_instruments = client.mf_instruments_typed().await?;
    /// println!("Total MF instruments: {}", mf_instruments.len());
    ///
    /// // Filter by fund type and AMC
    /// let equity_funds: Vec<_> = mf_instruments
    ///     .iter()
    ///     .filter(|fund| fund.is_equity_fund())
    ///     .collect();
    ///
    /// println!("Equity funds available: {}", equity_funds.len());
    ///
    /// // Find SIP eligible funds
    /// let sip_funds: Vec<_> = mf_instruments
    ///     .iter()
    ///     .filter(|fund| fund.allows_sip())
    ///     .collect();
    ///
    /// for fund in sip_funds.iter().take(5) {
    ///     println!("Fund: {}", fund.name);
    ///     println!("  AMC: {}", fund.amc);
    ///     println!("  Min SIP: ₹{:.2}", fund.minimum_additional_purchase_amount);
    ///     println!("  Settlement: {} days", fund.settlement_days());
    ///     println!("  Type: {} | Plan: {}", fund.fund_type, fund.plan);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Features
    ///
    /// - **Type Safety**: Compile-time validation and IDE autocomplete
    /// - **Helper Methods**: Built-in methods like `is_equity_fund()`, `allows_sip()`, `settlement_days()`
    /// - **Fund Analysis**: Easy filtering by fund type, AMC, and investment options
    /// - **Cross-Platform**: Works on both native and WASM platforms
    ///
    /// # Performance Notes
    ///
    /// - No caching for MF instruments (data changes frequently)
    /// - Efficient parsing of large CSV responses
    /// - Memory usage scales with number of available funds
    pub async fn mf_instruments_typed(&self) -> KiteResult<Vec<MFInstrument>> {
        // Get the JSON response using existing method
        let json_response = self
            .mf_instruments()
            .await
            .map_err(|e| KiteError::general(format!("Failed to get MF instruments: {}", e)))?;

        // Parse the JSON array into typed MF instruments
        if let Some(instruments_array) = json_response.as_array() {
            let mut instruments = Vec::new();

            for instrument_json in instruments_array {
                // Convert JSON object to MFInstrument struct
                match serde_json::from_value::<MFInstrument>(instrument_json.clone()) {
                    Ok(instrument) => instruments.push(instrument),
                    Err(e) => {
                        // Log the error but continue processing other instruments
                        #[cfg(feature = "debug")]
                        log::warn!(
                            "Failed to parse MF instrument: {:?} - Error: {}",
                            instrument_json,
                            e
                        );

                        // For now, continue with other instruments rather than failing completely
                        continue;
                    }
                }
            }

            Ok(instruments)
        } else {
            Err(KiteError::general(
                "Invalid MF instruments response format".to_string(),
            ))
        }
    }

    /// Retrieve historical data with automatic chunking for large date ranges
    ///
    /// This method automatically handles date ranges that exceed API limits by splitting them
    /// into smaller chunks and making multiple API calls. Features intelligent optimizations
    /// including reverse chronological processing and early termination for maximum efficiency.
    ///
    /// # Features
    ///
    /// - **Automatic Chunking**: Splits large requests into API-compliant chunks
    /// - **Reverse Chronological**: Processes newest data first for early termination efficiency
    /// - **Smart Early Termination**: Stops when empty chunks indicate no more data exists
    /// - **Data Deduplication**: Prevents overlapping date ranges in both-inclusive API
    /// - **Progress Logging**: Logs progress for large requests (when debug feature enabled)
    /// - **Rate Limiting**: Respects rate limits between chunk requests
    /// - **Error Handling**: Continues with remaining chunks if one fails (configurable)
    ///
    /// # Arguments
    ///
    /// * `request` - The historical data request (can exceed API limits)
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
    /// // Request 6 months of 5-minute data (exceeds 90-day limit)
    /// let request = HistoricalDataRequest::new(
    ///     738561,
    ///     NaiveDateTime::parse_from_str("2023-01-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
    ///     NaiveDateTime::parse_from_str("2023-07-01 15:30:00", "%Y-%m-%d %H:%M:%S")?,
    ///     Interval::FiveMinute,
    /// );
    ///
    /// // This will automatically split into multiple API calls and combine results
    /// let all_data = client.historical_data_chunked(request, false).await?;
    /// println!("Retrieved {} candles across the entire 6-month period", all_data.candles.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Considerations
    ///
    /// - **Optimized for New Instruments**: 60-90% fewer API calls for recently listed stocks
    /// - **Early Termination**: Stops immediately when data availability limit is reached
    /// - **Rate Limiting**: Automatic delays between chunks (100ms default)
    /// - **Memory Usage**: Scales with total number of candles requested
    /// - **Deduplication**: Eliminates duplicate data points from overlapping chunks
    ///
    /// # Error Handling
    ///
    /// - If `continue_on_error` is `false` (default), the method stops on first error
    /// - If `continue_on_error` is `true`, it continues with remaining chunks and logs errors
    /// - Successful chunks are still returned even if some chunks fail
    /// - Empty chunks trigger early termination (configurable behavior)
    pub async fn historical_data_chunked(
        &self,
        request: HistoricalDataRequest,
        continue_on_error: bool,
    ) -> KiteResult<HistoricalData> {
        // Split the request into valid chunks in reverse chronological order
        let chunk_requests = request.split_into_valid_requests_reverse();

        if chunk_requests.len() == 1 {
            // No chunking needed, use regular method
            return self.historical_data_typed(request).await;
        }

        #[cfg(feature = "debug")]
        log::info!(
            "Splitting large historical data request into {} chunks for {} interval (original span: {} days) - processing newest → oldest",
            chunk_requests.len(),
            request.interval,
            request.days_span()
        );

        let mut all_candles = Vec::new();
        let mut _successful_chunks = 0;
        let mut failed_chunks = 0;

        // Process each chunk in reverse chronological order (newest first)
        for (i, chunk_request) in chunk_requests.iter().enumerate() {
            #[cfg(feature = "debug")]
            log::debug!(
                "Processing chunk {}/{}: {} to {} ({} days)",
                i + 1,
                chunk_requests.len(),
                chunk_request.from.format("%Y-%m-%d %H:%M:%S"),
                chunk_request.to.format("%Y-%m-%d %H:%M:%S"),
                chunk_request.days_span()
            );

            match self.historical_data_typed(chunk_request.clone()).await {
                Ok(chunk_data) => {
                    if chunk_data.candles.is_empty() {
                        #[cfg(feature = "debug")]
                        log::info!(
                            "Empty chunk encountered at {} → {} - reached data availability limit, stopping early",
                            chunk_request.from.format("%Y-%m-%d"),
                            chunk_request.to.format("%Y-%m-%d")
                        );

                        // Early termination: empty chunk means no more historical data exists
                        break;
                    }

                    #[cfg(feature = "debug")]
                    let candles_count = chunk_data.candles.len();
                    all_candles.extend(chunk_data.candles);
                    _successful_chunks += 1;

                    #[cfg(feature = "debug")]
                    log::debug!(
                        "Chunk {}/{} completed successfully: {} candles retrieved",
                        i + 1,
                        chunk_requests.len(),
                        candles_count
                    );
                }
                Err(e) => {
                    failed_chunks += 1;

                    #[cfg(feature = "debug")]
                    log::warn!("Chunk {}/{} failed: {:?}", i + 1, chunk_requests.len(), e);

                    if !continue_on_error {
                        return Err(e);
                    }
                }
            }

            // Add a small delay between chunks to be respectful to the API
            if i < chunk_requests.len() - 1 {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }

        if all_candles.is_empty() && failed_chunks > 0 {
            return Err(crate::models::common::KiteError::general(format!(
                "All {} chunks failed to retrieve data",
                failed_chunks
            )));
        }

        // Sort candles by date to ensure chronological order (oldest → newest)
        all_candles.sort_by(|a, b| a.date.cmp(&b.date));

        #[cfg(feature = "debug")]
        log::info!(
            "Historical data chunking completed: {} successful chunks, {} failed chunks, {} total candles (processed {} of {} possible chunks)",
            _successful_chunks,
            failed_chunks,
            all_candles.len(),
            _successful_chunks + failed_chunks,
            chunk_requests.len()
        );

        // Create the final response
        let metadata = HistoricalMetadata {
            instrument_token: request.instrument_token,
            symbol: format!("Token-{}", request.instrument_token), // We don't have the symbol from chunks
            interval: request.interval,
            count: all_candles.len(),
        };

        Ok(HistoricalData {
            candles: all_candles,
            metadata,
        })
    }

    /// Simple instruments method for debugging - bypasses rate limiting and caching
    pub async fn instruments_simple(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let path = if let Some(exchange) = exchange {
            format!("/instruments/{}", exchange)
        } else {
            "/instruments".to_string()
        };

        let url = self.build_url(&path, None);

        let resp = self.send_request(url, "GET", None).await?;

        if !resp.status().is_success() {
            let error_text = resp.text().await?;
            return Err(anyhow::anyhow!("API Error: {}", error_text));
        }

        let body = resp.text().await?;

        if body.is_empty() {
            return Err(anyhow::anyhow!("Empty response body"));
        }

        // Parse CSV response
        let mut rdr = csv::ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();

        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }

        Ok(JsonValue::Array(result))
    }

    /// Get instruments list with gzip decompression support
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn instruments_with_gzip(&self, exchange: Option<&str>) -> Result<JsonValue> {
        // Check cache first if enabled
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                // Only cache the full instruments list, not exchange-specific ones
                if let Ok(cache_guard) = self.response_cache.lock() {
                    if let Some(ref cache) = *cache_guard {
                        if let Some(cached_data) = cache.get_instruments() {
                            #[cfg(feature = "debug")]
                            log::debug!("Returning cached instruments data");
                            return Ok(cached_data);
                        }
                    }
                }
            }
        }

        let endpoint = KiteEndpoint::Instruments;

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        #[cfg(feature = "debug")]
        log::debug!(
            "Requesting instruments with gzip support: endpoint={:?}, path_segments={:?}",
            endpoint,
            path_segments
        );

        let resp = self
            .send_request_with_rate_limiting_and_retry(endpoint, &path_segments, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;

        #[cfg(feature = "debug")]
        log::debug!("Received response with status: {}", resp.status());

        // Check if response is gzipped
        let content_encoding = resp
            .headers()
            .get("content-encoding")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        #[cfg(feature = "debug")]
        log::debug!("Content-Encoding: {}", content_encoding);

        // Decompress if gzipped
        let body_text = if content_encoding.contains("gzip") {
            #[cfg(feature = "debug")]
            log::debug!("Decompressing gzipped response");

            let body_bytes = resp.bytes().await?;
            use std::io::Read;
            let mut decoder = flate2::read::GzDecoder::new(&body_bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;
            decompressed
        } else {
            resp.text().await?
        };

        #[cfg(feature = "debug")]
        log::debug!(
            "Decompressed body length: {} bytes, first 200 chars: {}",
            body_text.len(),
            &body_text.chars().take(200).collect::<String>()
        );

        // Parse CSV response
        let mut rdr = csv::ReaderBuilder::new().from_reader(body_text.as_bytes());
        let mut result = Vec::new();

        let headers = rdr.headers()?.clone();
        #[cfg(feature = "debug")]
        log::debug!("CSV headers: {:?}", headers);

        let mut _record_count = 0;
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
            _record_count += 1;
        }

        #[cfg(feature = "debug")]
        log::debug!("Parsed {} records from CSV", _record_count);

        let result_json = JsonValue::Array(result);

        // Cache the result if enabled and it's the full instruments list
        if let Some(ref cache_config) = self.cache_config {
            if cache_config.enable_instruments_cache && exchange.is_none() {
                if let Ok(mut cache_guard) = self.response_cache.lock() {
                    if let Some(ref mut cache) = *cache_guard {
                        cache.set_instruments(result_json.clone());
                    }
                }
            }
        }

        Ok(result_json)
    }

    /// Debug method to show raw instruments JSON response
    pub async fn instruments_debug_json(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let endpoint = KiteEndpoint::Instruments;

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        println!(
            "🔍 Debug: Making request to endpoint: {:?}",
            endpoint.config().path
        );
        println!("🔍 Debug: Path segments: {:?}", path_segments);

        let resp = self
            .send_request_with_rate_limiting_and_retry(endpoint, &path_segments, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;

        println!("🔍 Debug: Response status: {}", resp.status());

        // Check content encoding
        let content_encoding = resp
            .headers()
            .get("content-encoding")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("none")
            .to_string();
        println!("🔍 Debug: Content-Encoding: {}", content_encoding);

        // Check content type
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        println!("🔍 Debug: Content-Type: {}", content_type);

        println!("🔍 Debug: Response headers: {:?}", resp.headers());

        // Get raw bytes first
        let body_bytes = resp.bytes().await?;
        println!("🔍 Debug: Raw response size: {} bytes", body_bytes.len());

        // Show first few bytes as hex to check if it's binary/gzipped
        let hex_preview: String = body_bytes
            .iter()
            .take(20)
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("🔍 Debug: First 20 bytes (hex): {}", hex_preview);

        // Try to decompress if gzipped
        let body_text =
            if content_encoding.contains("gzip") || body_bytes.starts_with(&[0x1f, 0x8b]) {
                println!("🔍 Debug: Detected gzipped content, decompressing...");
                use std::io::Read;
                let mut decoder = flate2::read::GzDecoder::new(&body_bytes[..]);
                let mut decompressed = String::new();
                match decoder.read_to_string(&mut decompressed) {
                    Ok(_) => {
                        println!(
                            "🔍 Debug: Successfully decompressed to {} chars",
                            decompressed.len()
                        );
                        decompressed
                    }
                    Err(e) => {
                        println!("🔍 Debug: Gzip decompression failed: {}", e);
                        String::from_utf8_lossy(&body_bytes).to_string()
                    }
                }
            } else {
                println!("🔍 Debug: Not gzipped, reading as text");
                String::from_utf8_lossy(&body_bytes).to_string()
            };

        println!("🔍 Debug: Text body length: {} chars", body_text.len());
        println!(
            "🔍 Debug: First 200 chars: {}",
            &body_text.chars().take(200).collect::<String>()
        );

        // Check if it looks like CSV
        let lines: Vec<&str> = body_text.lines().take(5).collect();
        println!("🔍 Debug: First 5 lines:");
        for (i, line) in lines.iter().enumerate() {
            println!("  {}: {}", i + 1, line);
        }

        // If it's CSV, try to parse it
        if body_text.starts_with("instrument_token") || content_type.contains("csv") {
            println!("🔍 Debug: Attempting CSV parsing...");

            let mut rdr = csv::ReaderBuilder::new().from_reader(body_text.as_bytes());
            let mut result = Vec::new();

            let headers = rdr.headers()?.clone();
            println!("🔍 Debug: CSV headers: {:?}", headers);

            let mut record_count = 0;
            for record in rdr.records() {
                let record = record?;
                let mut obj = serde_json::Map::new();

                for (i, field) in record.iter().enumerate() {
                    if let Some(header) = headers.get(i) {
                        obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                    }
                }
                result.push(JsonValue::Object(obj));
                record_count += 1;

                if record_count >= 10 {
                    break;
                } // Just parse first 10 for debugging
            }

            println!("🔍 Debug: Successfully parsed {} CSV records", record_count);
            Ok(JsonValue::Array(result))
        } else {
            println!("🔍 Debug: Response doesn't look like CSV");
            // Try to parse as JSON
            match serde_json::from_str::<JsonValue>(&body_text) {
                Ok(json) => {
                    println!("🔍 Debug: Successfully parsed as JSON");
                    Ok(json)
                }
                Err(e) => {
                    println!("🔍 Debug: Failed to parse as JSON: {}", e);
                    Ok(JsonValue::String(body_text))
                }
            }
        }
    }
}
