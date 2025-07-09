use crate::models::common::{Exchange, InstrumentType, Segment};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Deserializer, Serialize};

/// Custom deserializer to convert string to u32
fn deserialize_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(serde::de::Error::custom)
}

/// Custom deserializer to convert string to f64
fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

/// Custom deserializer to convert optional string to Optional<NaiveDate>
fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "null" || s == "" {
        Ok(None)
    } else {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

/// Instrument data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument token (unique identifier)
    #[serde(rename = "instrument_token")]
    pub instrument_token: String,

    /// Exchange token
    #[serde(rename = "exchange_token")]
    pub exchange_token: String,

    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Company name or instrument name
    pub name: String,

    /// Last price
    #[serde(rename = "last_price", deserialize_with = "deserialize_string_to_f64")]
    pub last_price: f64,

    /// Expiry date (for derivatives, None for equity)
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub expiry: Option<NaiveDate>,

    /// Strike price (for options, 0.0 for others)
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub strike: f64,

    /// Tick size (minimum price movement)
    #[serde(rename = "tick_size", deserialize_with = "deserialize_string_to_f64")]
    pub tick_size: f64,

    /// Lot size (minimum quantity for trading)
    #[serde(rename = "lot_size", deserialize_with = "deserialize_string_to_u32")]
    pub lot_size: u32,

    /// Instrument type
    #[serde(rename = "instrument_type")]
    pub instrument_type: InstrumentType,

    /// Segment
    pub segment: Segment,

    /// Exchange
    pub exchange: Exchange,
}

/// Market status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatus {
    /// Exchange
    pub exchange: Exchange,

    /// Market status
    pub status: MarketState,

    /// Market open time
    #[serde(rename = "market_open")]
    pub market_open: Option<NaiveTime>,

    /// Market close time
    #[serde(rename = "market_close")]
    pub market_close: Option<NaiveTime>,

    /// Pre-market open time
    #[serde(rename = "pre_market_open")]
    pub pre_market_open: Option<NaiveTime>,

    /// Pre-market close time
    #[serde(rename = "pre_market_close")]
    pub pre_market_close: Option<NaiveTime>,

    /// Post-market open time
    #[serde(rename = "post_market_open")]
    pub post_market_open: Option<NaiveTime>,

    /// Post-market close time
    #[serde(rename = "post_market_close")]
    pub post_market_close: Option<NaiveTime>,
}

/// Market state enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MarketState {
    /// Market is open for trading
    Open,
    /// Market is closed
    Closed,
    /// Pre-market session
    #[serde(rename = "pre_market")]
    PreMarket,
    /// Post-market session
    #[serde(rename = "post_market")]
    PostMarket,
    /// Market break/lunch time
    Break,
    /// Holiday
    Holiday,
}

/// Instrument search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentSearch {
    /// Exchange to search in (optional)
    pub exchange: Option<Exchange>,

    /// Instrument type filter (optional)
    pub instrument_type: Option<InstrumentType>,

    /// Search query string
    pub query: String,

    /// Maximum number of results
    pub limit: Option<u32>,
}

/// Instrument lookup by token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentLookup {
    /// List of instrument tokens to lookup
    pub tokens: Vec<u32>,

    /// Exchange (optional, for validation)
    pub exchange: Option<Exchange>,
}

impl Instrument {
    /// Check if the instrument is an equity
    pub fn is_equity(&self) -> bool {
        matches!(self.instrument_type, InstrumentType::EQ)
    }

    /// Check if the instrument is a future
    pub fn is_future(&self) -> bool {
        matches!(self.instrument_type, InstrumentType::FUT)
    }

    /// Check if the instrument is an option
    pub fn is_option(&self) -> bool {
        matches!(
            self.instrument_type,
            InstrumentType::CE | InstrumentType::PE
        )
    }

    /// Check if the instrument is a call option
    pub fn is_call_option(&self) -> bool {
        matches!(self.instrument_type, InstrumentType::CE)
    }

    /// Check if the instrument is a put option
    pub fn is_put_option(&self) -> bool {
        matches!(self.instrument_type, InstrumentType::PE)
    }

    /// Check if the instrument has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            chrono::Utc::now().date_naive() > expiry
        } else {
            false
        }
    }

    /// Check if the instrument expires today
    pub fn expires_today(&self) -> bool {
        if let Some(expiry) = self.expiry {
            chrono::Utc::now().date_naive() == expiry
        } else {
            false
        }
    }

    /// Get days to expiry (None for non-expiring instruments)
    pub fn days_to_expiry(&self) -> Option<i64> {
        self.expiry.map(|expiry| {
            let today = chrono::Utc::now().date_naive();
            (expiry - today).num_days()
        })
    }

    /// Check if the instrument is in-the-money (for options)
    pub fn is_itm(&self, underlying_price: f64) -> Option<bool> {
        if !self.is_option() {
            return None;
        }

        Some(match self.instrument_type {
            InstrumentType::CE => underlying_price > self.strike,
            InstrumentType::PE => underlying_price < self.strike,
            _ => return None,
        })
    }

    /// Check if the instrument is at-the-money (for options)
    pub fn is_atm(&self, underlying_price: f64, tolerance: f64) -> Option<bool> {
        if !self.is_option() {
            return None;
        }

        Some((underlying_price - self.strike).abs() <= tolerance)
    }

    /// Check if the instrument is out-of-the-money (for options)
    pub fn is_otm(&self, underlying_price: f64) -> Option<bool> {
        self.is_itm(underlying_price).map(|itm| !itm)
    }

    /// Calculate intrinsic value (for options)
    pub fn intrinsic_value(&self, underlying_price: f64) -> Option<f64> {
        if !self.is_option() {
            return None;
        }

        let value = match self.instrument_type {
            InstrumentType::CE => (underlying_price - self.strike).max(0.0),
            InstrumentType::PE => (self.strike - underlying_price).max(0.0),
            _ => return None,
        };

        Some(value)
    }

    /// Calculate the tick value in rupees
    pub fn tick_value(&self) -> f64 {
        self.tick_size * self.lot_size as f64
    }
}

impl MarketStatus {
    /// Check if the market is currently open
    pub fn is_open(&self) -> bool {
        self.status == MarketState::Open
    }

    /// Check if the market is closed
    pub fn is_closed(&self) -> bool {
        self.status == MarketState::Closed
    }

    /// Check if it's pre-market session
    pub fn is_pre_market(&self) -> bool {
        self.status == MarketState::PreMarket
    }

    /// Check if it's post-market session
    pub fn is_post_market(&self) -> bool {
        self.status == MarketState::PostMarket
    }

    /// Check if the market is in break
    pub fn is_break(&self) -> bool {
        self.status == MarketState::Break
    }

    /// Check if it's a holiday
    pub fn is_holiday(&self) -> bool {
        self.status == MarketState::Holiday
    }

    /// Check if any trading is allowed (including pre/post market)
    pub fn is_trading_allowed(&self) -> bool {
        matches!(
            self.status,
            MarketState::Open | MarketState::PreMarket | MarketState::PostMarket
        )
    }
}

impl InstrumentSearch {
    /// Create a new instrument search
    pub fn new(query: String) -> Self {
        Self {
            exchange: None,
            instrument_type: None,
            query,
            limit: None,
        }
    }

    /// Set the exchange filter
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set the instrument type filter
    pub fn instrument_type(mut self, instrument_type: InstrumentType) -> Self {
        self.instrument_type = Some(instrument_type);
        self
    }

    /// Set the result limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Search for equity instruments only
    pub fn equity_only(mut self) -> Self {
        self.instrument_type = Some(InstrumentType::EQ);
        self
    }

    /// Search for options only
    pub fn options_only(mut self) -> Self {
        // Note: This sets to CE, but actual implementation should handle both CE and PE
        self.instrument_type = Some(InstrumentType::CE);
        self
    }

    /// Search for futures only
    pub fn futures_only(mut self) -> Self {
        self.instrument_type = Some(InstrumentType::FUT);
        self
    }
}

impl InstrumentLookup {
    /// Create a new instrument lookup
    pub fn new(tokens: Vec<u32>) -> Self {
        Self {
            tokens,
            exchange: None,
        }
    }

    /// Set the exchange for validation
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Add a token to lookup
    pub fn add_token(mut self, token: u32) -> Self {
        self.tokens.push(token);
        self
    }

    /// Add multiple tokens to lookup
    pub fn add_tokens(mut self, tokens: Vec<u32>) -> Self {
        self.tokens.extend(tokens);
        self
    }
}
