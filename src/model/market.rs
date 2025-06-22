//! # Market Data Models
//! 
//! This module contains data models for market data operations in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/market.go

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::model::ticker::{OHLC, Depth};

/// QuoteData represents the individual quote data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteData {
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    pub timestamp: DateTime<Utc>,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "last_quantity")]
    pub last_quantity: u32,
    
    #[serde(rename = "last_trade_time")]
    pub last_trade_time: DateTime<Utc>,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    pub volume: u32,
    
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: u32,
    
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: u32,
    
    pub ohlc: OHLC,
    
    #[serde(rename = "net_change")]
    pub net_change: f64,
    
    #[serde(rename = "oi")]
    pub oi: f64,
    
    #[serde(rename = "oi_day_high")]
    pub oi_day_high: f64,
    
    #[serde(rename = "oi_day_low")]
    pub oi_day_low: f64,
    
    #[serde(rename = "lower_circuit_limit")]
    pub lower_circuit_limit: f64,
    
    #[serde(rename = "upper_circuit_limit")]
    pub upper_circuit_limit: f64,
    
    pub depth: Depth,
}

/// Quote represents the full quote response as a map of instrument symbols to quote data.
pub type Quote = HashMap<String, QuoteData>;

/// QuoteOHLCData represents OHLC quote data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteOHLCData {
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    pub ohlc: OHLC,
}

/// QuoteOHLC represents OHLC quote response as a map.
pub type QuoteOHLC = HashMap<String, QuoteOHLCData>;

/// QuoteLTPData represents LTP quote data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteLTPData {
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
}

/// QuoteLTP represents last price quote response as a map.
pub type QuoteLTP = HashMap<String, QuoteLTPData>;

/// HistoricalData represents individual historical data response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalData {
    pub date: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
    
    #[serde(rename = "oi")]
    pub oi: u32,
}

/// Instrument represents individual instrument response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "exchange_token")]
    pub exchange_token: u32,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub name: String,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    pub expiry: Option<DateTime<Utc>>,
    
    #[serde(rename = "strike")]
    pub strike_price: f64,
    
    #[serde(rename = "tick_size")]
    pub tick_size: f64,
    
    #[serde(rename = "lot_size")]
    pub lot_size: f64,
    
    #[serde(rename = "instrument_type")]
    pub instrument_type: String,
    
    pub segment: String,
    pub exchange: String,
}

/// Instruments represents a list of instruments.
pub type Instruments = Vec<Instrument>;

/// MFInstrument represents individual mutual fund instrument response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFInstrument {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub name: String,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "amc")]
    pub amc: String,
    
    #[serde(rename = "purchase_allowed")]
    pub purchase_allowed: bool,
    
    #[serde(rename = "redemption_allowed")]
    pub redemption_allowed: bool,
    
    #[serde(rename = "minimum_purchase_amount")]
    pub minimum_purchase_amount: f64,
    
    #[serde(rename = "purchase_amount_multiplier")]
    pub purchase_amount_multiplier: f64,
    
    #[serde(rename = "minimum_additional_purchase_amount")]
    pub minimum_additional_purchase_amount: f64,
    
    #[serde(rename = "minimum_redemption_quantity")]
    pub minimum_redemption_quantity: f64,
    
    #[serde(rename = "redemption_quantity_multiplier")]
    pub redemption_quantity_multiplier: f64,
    
    #[serde(rename = "dividend_type")]
    pub dividend_type: String,
    
    #[serde(rename = "scheme_type")]
    pub scheme_type: String,
    
    pub plan: String,
    
    #[serde(rename = "settlement_type")]
    pub settlement_type: String,
    
    #[serde(rename = "last_price_date")]
    pub last_price_date: DateTime<Utc>,
}

/// MFInstruments represents a list of mutual fund instruments.
pub type MFInstruments = Vec<MFInstrument>;

/// TriggerRange represents trigger range for GTT orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerRange {
    pub start: f64,
    pub end: f64,
    pub percent: f64,
}

/// Trigger range data for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerRangeData {
    /// Instrument token
    pub instrument_token: u32,
    /// Lower trigger price limit
    pub lower: f64,
    /// Upper trigger price limit
    pub upper: f64,
}

/// Trigger range response containing mapping of instruments to their trigger ranges
pub type TriggerRangeResponse = std::collections::HashMap<String, TriggerRangeData>;

impl HistoricalData {
    /// Create a new HistoricalData structure.
    pub fn new(date: DateTime<Utc>, open: f64, high: f64, low: f64, close: f64, volume: u32, oi: u32) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
            volume,
            oi,
        }
    }
    
    /// Calculate the typical price (HLC/3).
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
    
    /// Calculate the weighted close price (HLCC/4).
    pub fn weighted_close(&self) -> f64 {
        (self.high + self.low + self.close + self.close) / 4.0
    }
    
    /// Calculate the median price (HL/2).
    pub fn median_price(&self) -> f64 {
        (self.high + self.low) / 2.0
    }
    
    /// Calculate the price range (H-L).
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
    
    /// Check if this is a bullish candle.
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
    
    /// Check if this is a bearish candle.
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
    
    /// Check if this is a doji candle.
    pub fn is_doji(&self) -> bool {
        self.close == self.open
    }
    
    /// Calculate percentage change from open to close.
    pub fn percentage_change(&self) -> f64 {
        if self.open == 0.0 {
            0.0
        } else {
            ((self.close - self.open) / self.open) * 100.0
        }
    }
    
    /// Calculate True Range (TR) for volatility analysis.
    pub fn true_range(&self, previous_close: Option<f64>) -> f64 {
        match previous_close {
            Some(prev_close) => {
                let high_low = self.high - self.low;
                let high_prev_close = (self.high - prev_close).abs();
                let low_prev_close = (self.low - prev_close).abs();
                high_low.max(high_prev_close).max(low_prev_close)
            }
            None => self.high - self.low,
        }
    }
}

impl Instrument {
    /// Check if the instrument is an equity instrument.
    pub fn is_equity(&self) -> bool {
        self.segment.to_uppercase() == "EQ" || 
        self.exchange == "NSE" || self.exchange == "BSE"
    }
    
    /// Check if the instrument is a derivative.
    pub fn is_derivative(&self) -> bool {
        self.segment.to_uppercase() == "NFO" || 
        self.segment.to_uppercase() == "BFO" ||
        self.instrument_type == "FUT" || 
        self.instrument_type == "CE" || 
        self.instrument_type == "PE"
    }
    
    /// Check if the instrument is a futures contract.
    pub fn is_futures(&self) -> bool {
        self.instrument_type == "FUT"
    }
    
    /// Check if the instrument is an options contract.
    pub fn is_options(&self) -> bool {
        self.instrument_type == "CE" || self.instrument_type == "PE"
    }
    
    /// Check if the instrument is a call option.
    pub fn is_call_option(&self) -> bool {
        self.instrument_type == "CE"
    }
    
    /// Check if the instrument is a put option.
    pub fn is_put_option(&self) -> bool {
        self.instrument_type == "PE"
    }
    
    /// Check if the instrument has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            expiry < Utc::now()
        } else {
            false
        }
    }
    
    /// Get days to expiry (negative if already expired).
    pub fn days_to_expiry(&self) -> Option<i64> {
        self.expiry.map(|expiry| {
            let duration = expiry.signed_duration_since(Utc::now());
            duration.num_days()
        })
    }
    
    /// Calculate minimum price movement based on tick size.
    pub fn min_price_movement(&self) -> f64 {
        self.tick_size
    }
    
    /// Calculate position value for given quantity.
    pub fn position_value(&self, quantity: f64) -> f64 {
        quantity * self.last_price * self.lot_size
    }
}

impl MFInstrument {
    /// Check if purchases are allowed.
    pub fn can_purchase(&self) -> bool {
        self.purchase_allowed
    }
    
    /// Check if redemptions are allowed.
    pub fn can_redeem(&self) -> bool {
        self.redemption_allowed
    }
    
    /// Check if the minimum purchase amount is met.
    pub fn is_valid_purchase_amount(&self, amount: f64) -> bool {
        amount >= self.minimum_purchase_amount && 
        amount % self.purchase_amount_multiplier == 0.0
    }
    
    /// Check if the redemption quantity is valid.
    pub fn is_valid_redemption_quantity(&self, quantity: f64) -> bool {
        quantity >= self.minimum_redemption_quantity &&
        quantity % self.redemption_quantity_multiplier == 0.0
    }
    
    /// Get valid purchase amount (rounded up to nearest valid amount).
    pub fn round_purchase_amount(&self, amount: f64) -> f64 {
        let min_amount = self.minimum_purchase_amount.max(amount);
        let multiplier = self.purchase_amount_multiplier;
        (min_amount / multiplier).ceil() * multiplier
    }
    
    /// Get valid redemption quantity (rounded up to nearest valid quantity).
    pub fn round_redemption_quantity(&self, quantity: f64) -> f64 {
        let min_quantity = self.minimum_redemption_quantity.max(quantity);
        let multiplier = self.redemption_quantity_multiplier;
        (min_quantity / multiplier).ceil() * multiplier
    }
}

impl QuoteData {
    /// Check if the stock hit upper circuit.
    pub fn is_upper_circuit(&self) -> bool {
        (self.last_price - self.upper_circuit_limit).abs() < f64::EPSILON
    }
    
    /// Check if the stock hit lower circuit.
    pub fn is_lower_circuit(&self) -> bool {
        (self.last_price - self.lower_circuit_limit).abs() < f64::EPSILON
    }
    
    /// Calculate percentage change from previous close.
    pub fn percentage_change(&self) -> f64 {
        if self.ohlc.close == 0.0 {
            0.0
        } else {
            ((self.last_price - self.ohlc.close) / self.ohlc.close) * 100.0
        }
    }
    
    /// Check if there's strong buying pressure based on buy/sell quantity ratio.
    pub fn has_buying_pressure(&self) -> bool {
        if self.sell_quantity == 0 {
            self.buy_quantity > 0
        } else {
            self.buy_quantity as f64 / self.sell_quantity as f64 > 2.0
        }
    }
    
    /// Check if there's strong selling pressure.
    pub fn has_selling_pressure(&self) -> bool {
        if self.buy_quantity == 0 {
            self.sell_quantity > 0
        } else {
            self.sell_quantity as f64 / self.buy_quantity as f64 > 2.0
        }
    }
    
    /// Calculate market capitalization estimate (if shares outstanding is known).
    pub fn market_cap_estimate(&self, shares_outstanding: u64) -> f64 {
        self.last_price * shares_outstanding as f64
    }
}