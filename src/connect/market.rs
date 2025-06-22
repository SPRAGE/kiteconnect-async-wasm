use serde_json::Value as JsonValue;
use anyhow::Result;
use super::{client::KiteConnect, request::RequestHandler};

// Native platform imports
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use csv::ReaderBuilder;

// WASM platform imports
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use super::utils::{parse_csv_to_instruments, parse_csv_to_mf_instruments};

use chrono::Utc;

// Import model types for typed responses
use crate::model::{
    Quote, QuoteOHLC, QuoteLTP, Instrument, MFInstrument, TriggerRangeResponse,
};

impl KiteConnect {
    /// Get the trigger range for a list of instruments
    /// 
    /// Retrieves the allowed price range for placing stoploss orders on given instruments.
    /// This is useful for determining valid trigger prices for bracket and cover orders.
    /// 
    /// # Arguments
    /// 
    /// * `transaction_type` - Transaction type ("BUY" or "SELL")
    /// * `instruments` - List of instrument identifiers (exchange:tradingsymbol or instrument_token)
    /// 
    /// # Returns
    /// 
    /// A `Result<TriggerRangeResponse>` containing a HashMap mapping instrument symbols to their trigger range data with fields like:
    /// - `instrument_token` - Token of the instrument
    /// - `lower` - Lower trigger price limit
    /// - `upper` - Upper trigger price limit
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
    /// let trigger_ranges = client.trigger_range("BUY", instruments).await?;
    /// 
    /// // Access trigger range data directly
    /// for (symbol, range_data) in &trigger_ranges {
    ///     println!("Symbol: {}, Lower: {}, Upper: {}", 
    ///         symbol, range_data.lower, range_data.upper);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trigger_range(
        &self,
        transaction_type: &str,
        instruments: Vec<&str>,
    ) -> Result<TriggerRangeResponse> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("transaction_type", transaction_type));
        
        for instrument in instruments {
            params.push(("instruments", instrument));
        }

        let url = self.build_url("/instruments/trigger_range", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get instruments list
    /// 
    /// Retrieves the list of trading instruments available on the platform.
    /// Returns detailed information about each instrument including symbols, tokens,
    /// expiry dates, strike prices, and other instrument-specific details.
    /// 
    /// # Arguments
    /// 
    /// * `exchange` - Optional exchange name to filter instruments (e.g., "NSE", "BSE", "NFO")
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Instrument>>` containing instrument data with fields like:
    /// - `instrument_token` - Unique instrument identifier
    /// - `tradingsymbol` - Trading symbol
    /// - `name` - Instrument name
    /// - `exchange` - Exchange on which the instrument is listed
    /// - `segment` - Market segment
    /// - `instrument_type` - Type of instrument (EQ, FUT, OPT, etc.)
    /// - `expiry` - Expiry date for derivatives (None for equity)
    /// - `strike_price` - Strike price for options
    /// - `lot_size` - Minimum trading lot size
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails or CSV parsing fails
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
    /// // Get all instruments
    /// let all_instruments = client.instruments(None).await?;
    /// println!("Total instruments: {}", all_instruments.len());
    /// 
    /// // Get NSE instruments only
    /// let nse_instruments = client.instruments(Some("NSE")).await?;
    /// for instrument in nse_instruments.iter().take(5) {
    ///     println!("Instrument: {} ({})", instrument.tradingsymbol, instrument.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response into typed Instrument structs
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        for record in rdr.records() {
            let record = record?;
            if record.len() >= 12 {
                let instrument = Instrument {
                    instrument_token: record[0].parse().unwrap_or(0),
                    exchange_token: record[1].parse().unwrap_or(0),
                    tradingsymbol: record[2].to_string(),
                    name: record[3].to_string(),
                    last_price: record[4].parse().unwrap_or(0.0),
                    expiry: if record[5].is_empty() { 
                        None 
                    } else { 
                        record[5].parse().ok() 
                    },
                    strike_price: record[6].parse().unwrap_or(0.0),
                    tick_size: record[7].parse().unwrap_or(0.0),
                    lot_size: record[8].parse().unwrap_or(0.0),
                    instrument_type: record[9].to_string(),
                    segment: record[10].to_string(),
                    exchange: record[11].to_string(),
                };
                result.push(instrument);
            }
        }
        
        Ok(result)
    }

    /// Get instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility and convert to Instrument structs
        parse_csv_to_instruments(&body)
    }

    /// Get mutual fund instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response to MFInstrument structs
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        for record_result in rdr.records() {
            let record = record_result?;
            if record.len() >= 16 {
                let mf_instrument = MFInstrument {
                    tradingsymbol: record[0].to_string(),
                    name: record[1].to_string(),
                    last_price: record[2].parse().unwrap_or(0.0),
                    amc: record[3].to_string(),
                    purchase_allowed: record[4].parse().unwrap_or(false),
                    redemption_allowed: record[5].parse().unwrap_or(false),
                    minimum_purchase_amount: record[6].parse().unwrap_or(0.0),
                    purchase_amount_multiplier: record[7].parse().unwrap_or(0.0),
                    minimum_additional_purchase_amount: record[8].parse().unwrap_or(0.0),
                    minimum_redemption_quantity: record[9].parse().unwrap_or(0.0),
                    redemption_quantity_multiplier: record[10].parse().unwrap_or(0.0),
                    dividend_type: record[11].to_string(),
                    scheme_type: record[12].to_string(),
                    plan: record[13].to_string(),
                    settlement_type: record[14].to_string(),
                    last_price_date: record[15].parse().unwrap_or_else(|_| Utc::now()),
                };
                result.push(mf_instrument);
            }
        }
        
        Ok(result)
    }

    /// Get mutual fund instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility and convert to MFInstrument structs
        parse_csv_to_mf_instruments(&body)
    }

    /// Get instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn instruments(&self, _exchange: Option<&str>) -> Result<Vec<Instrument>> {
        Err(anyhow!("Instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Get mutual fund instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
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
        
        let url = self.build_url(
            &format!("/instruments/historical/{}/{}", instrument_token, interval),
            Some(params),
        );

        let resp = self.send_request(url, "GET", None).await?;
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
    /// A `Result<Quote>` containing a HashMap mapping instrument symbols to quote data with fields like:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close data  
    /// - `depth` - Market depth with bid/ask prices and quantities
    /// - `volume` - Trading volume
    /// - `net_change` - Price change from previous close
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
    /// // Access quote data directly
    /// for (symbol, quote_data) in &quotes {
    ///     println!("Symbol: {}, LTP: {}, Change: {}", 
    ///         symbol, quote_data.last_price, quote_data.net_change);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quote(&self, instruments: Vec<&str>) -> Result<Quote> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<QuoteOHLC>` containing a HashMap mapping instrument symbols to OHLC data with fields:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close values for the day
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
    /// // Access OHLC data directly
    /// for (symbol, data) in &ohlc_data {
    ///     println!("Symbol: {}, LTP: {}, Open: {}, High: {}, Low: {}, Close: {}", 
    ///         symbol, data.last_price, data.ohlc.open, data.ohlc.high, 
    ///         data.ohlc.low, data.ohlc.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ohlc(&self, instruments: Vec<&str>) -> Result<QuoteOHLC> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ohlc", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<QuoteLTP>` containing a HashMap mapping instrument symbols to LTP data with fields:
    /// - `last_price` - Last traded price
    /// - `instrument_token` - Instrument token
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
    /// // Access LTP data directly  
    /// for (symbol, data) in &ltp_data {
    ///     println!("Symbol: {}, LTP: {}", symbol, data.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ltp(&self, instruments: Vec<&str>) -> Result<QuoteLTP> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ltp", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }
}
