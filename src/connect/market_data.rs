//! # Market Data Module
//! 
//! This module contains market data methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use crate::connect::endpoints::KiteEndpoint;

// Native platform imports
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use csv::ReaderBuilder;

use crate::connect::KiteConnect;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use crate::connect::utils::parse_csv_with_core;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::market_data::{Quote, OHLC, LTP, HistoricalData};

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

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::TriggerRange, 
            &[],
            Some(params),
            None
        ).await.map_err(|e| anyhow::anyhow!("Get trigger range failed: {:?}", e))?;
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
                            return Ok(cached_data);
                        }
                    }
                }
            }
        }

        let endpoint = if exchange.is_some() {
            KiteEndpoint::Instruments
        } else {
            KiteEndpoint::Instruments
        };

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        let resp = self.send_request_with_rate_limiting_and_retry(
            endpoint, 
            &path_segments,
            None,
            None
        ).await.map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;
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

        let endpoint = if exchange.is_some() {
            KiteEndpoint::Instruments
        } else {
            KiteEndpoint::Instruments
        };

        let path_segments = if let Some(exchange) = exchange {
            vec![exchange]
        } else {
            vec![]
        };

        let resp = self.send_request_with_rate_limiting_and_retry(
            endpoint, 
            &path_segments,
            None,
            None
        ).await.map_err(|e| anyhow::anyhow!("Get instruments failed: {:?}", e))?;
        let body = resp.text().await?;
        
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
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MFInstruments, 
            &[],
            None,
            None
        ).await.map_err(|e| anyhow::anyhow!("Get MF instruments failed: {:?}", e))?;
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
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MFInstruments, 
            &[],
            None,
            None
        ).await.map_err(|e| anyhow::anyhow!("Get MF instruments failed: {:?}", e))?;
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
        Err(anyhow!("Instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Get mutual fund instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        Err(anyhow!("MF instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
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
        let mut params = Vec::new();
        params.push(("from", from_date));
        params.push(("to", to_date));
        params.push(("continuous", continuous));
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::HistoricalData, 
            &[instrument_token, interval],
            Some(params),
            None
        ).await.map_err(|e| anyhow::anyhow!("Get historical data failed: {:?}", e))?;

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
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::Quote, 
            &[],
            Some(params),
            None
        ).await.map_err(|e| anyhow::anyhow!("Get quote failed: {:?}", e))?;
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
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::OHLC,
            &[],
            Some(params),
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to get OHLC data: {}", e))?;
        
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
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::LTP,
            &[],
            Some(params),
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to get LTP data: {}", e))?;
        
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
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MarketMargins,
            &[segment],
            None,
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to get instrument margins: {}", e))?;
        
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
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::Quote,
            &[],
            Some(params),
            None,
        ).await?;
        
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
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::OHLC,
            &[],
            Some(params),
            None,
        ).await?;
        
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
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::LTP,
            &[],
            Some(params),
            None,
        ).await?;
        
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
    /// * `instrument_token` - The instrument token
    /// * `from_date` - Start date in YYYY-MM-DD format
    /// * `to_date` - End date in YYYY-MM-DD format
    /// * `interval` - Time interval for candlesticks
    /// * `continuous` - Whether to include continuous data
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<HistoricalData>` containing typed historical data
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
    /// let historical_data = client.historical_data_typed(
    ///     "738561",           // RELIANCE instrument token
    ///     "2023-11-01",       // From date
    ///     "2023-11-30",       // To date
    ///     "day",              // Daily interval
    ///     "0"                 // No continuous data
    /// ).await?;
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
        instrument_token: &str,
        from_date: &str,
        to_date: &str,
        interval: &str,
        continuous: &str,
    ) -> KiteResult<HistoricalData> {
        let mut params = Vec::new();
        params.push(("from", from_date));
        params.push(("to", to_date));
        params.push(("continuous", continuous));
        
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::HistoricalData,
            &[instrument_token, interval],
            Some(params),
            None,
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
