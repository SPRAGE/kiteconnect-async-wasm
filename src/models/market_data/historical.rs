use crate::models::common::Interval;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Historical data request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataRequest {
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,

    /// From date
    pub from: NaiveDateTime,

    /// To date
    pub to: NaiveDateTime,

    /// Interval (timeframe)
    pub interval: Interval,

    /// Continuous data (for futures, to get continuous contract data)
    pub continuous: Option<bool>,

    /// Open interest data (for derivatives)
    pub oi: Option<bool>,
}

/// Historical candle data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Timestamp
    pub date: DateTime<Utc>,

    /// Open price
    pub open: f64,

    /// High price
    pub high: f64,

    /// Low price
    pub low: f64,

    /// Close price
    pub close: f64,

    /// Volume
    pub volume: u64,

    /// Open interest (for derivatives)
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
