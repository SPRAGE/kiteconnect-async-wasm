//! # Ticker/Real-time Models
//! 
//! This module contains models for real-time market data feeds and tick data
//! from the KiteConnect WebSocket API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Custom time wrapper that matches Go KiteConnect's Time model
/// Handles various timestamp formats used by the KiteConnect API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Time {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
}

impl Time {
    /// Create a new Time instance from Unix timestamp
    pub fn from_unix(timestamp: i64) -> Self {
        Self {
            time: DateTime::from_timestamp(timestamp, 0).unwrap_or_default(),
        }
    }

    /// Get Unix timestamp
    pub fn unix(&self) -> i64 {
        self.time.timestamp()
    }

    /// Check if time is zero/default
    pub fn is_zero(&self) -> bool {
        self.time.timestamp() == 0
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            time: DateTime::<Utc>::default(),
        }
    }
}

/// OHLC (Open, High, Low, Close) data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OHLC {
    /// Instrument token (optional, used internally)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_token: Option<u32>,
    
    /// Opening price
    pub open: f64,
    
    /// Highest price
    pub high: f64,
    
    /// Lowest price
    pub low: f64,
    
    /// Closing price
    pub close: f64,
}

impl OHLC {
    /// Calculate the typical price (HLC/3)
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Calculate the price range
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Check if it's a bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Check if it's a bearish candle
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// Check if it's a doji (open == close)
    pub fn is_doji(&self) -> bool {
        (self.close - self.open).abs() < f64::EPSILON
    }
}

/// Single market depth entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DepthItem {
    /// Price level
    pub price: f64,
    
    /// Quantity at this price level
    pub quantity: u32,
    
    /// Number of orders at this price level
    pub orders: u32,
}

/// Market depth structure containing buy and sell orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Depth {
    /// Top 5 buy orders (bids)
    pub buy: [DepthItem; 5],
    
    /// Top 5 sell orders (asks)
    pub sell: [DepthItem; 5],
}

impl Default for Depth {
    fn default() -> Self {
        Self {
            buy: [DepthItem::default(); 5],
            sell: [DepthItem::default(); 5],
        }
    }
}

impl Default for DepthItem {
    fn default() -> Self {
        Self {
            price: 0.0,
            quantity: 0,
            orders: 0,
        }
    }
}

impl Depth {
    /// Get the best bid price
    pub fn best_bid(&self) -> Option<f64> {
        self.buy.first().map(|item| item.price).filter(|&p| p > 0.0)
    }

    /// Get the best ask price
    pub fn best_ask(&self) -> Option<f64> {
        self.sell.first().map(|item| item.price).filter(|&p| p > 0.0)
    }

    /// Calculate the bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Calculate total bid quantity
    pub fn total_bid_quantity(&self) -> u32 {
        self.buy.iter().map(|item| item.quantity).sum()
    }

    /// Calculate total ask quantity
    pub fn total_ask_quantity(&self) -> u32 {
        self.sell.iter().map(|item| item.quantity).sum()
    }
}

/// Ticker modes for subscription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Last Traded Price only
    #[serde(rename = "ltp")]
    LTP,
    
    /// Quote data (OHLC + LTP + volume + etc)
    Quote,
    
    /// Full market data (Quote + Market depth)
    Full,
}

/// Real-time tick data structure
/// Represents a single packet in the market feed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tick {
    /// Current subscription mode
    pub mode: String,
    
    /// Instrument token
    pub instrument_token: u32,
    
    /// Whether the instrument is tradable
    pub is_tradable: bool,
    
    /// Whether the instrument is an index
    pub is_index: bool,

    /// Exchange timestamp
    pub timestamp: Time,
    
    /// Last trade timestamp
    pub last_trade_time: Time,
    
    /// Last traded price
    pub last_price: f64,
    
    /// Last traded quantity
    pub last_traded_quantity: u32,
    
    /// Total buy quantity across all price levels
    pub total_buy_quantity: u32,
    
    /// Total sell quantity across all price levels
    pub total_sell_quantity: u32,
    
    /// Volume traded for the day
    pub volume_traded: u32,
    
    /// Total buy value
    pub total_buy: u32,
    
    /// Total sell value
    pub total_sell: u32,
    
    /// Average trade price
    pub average_trade_price: f64,
    
    /// Open Interest (for F&O instruments)
    pub oi: u32,
    
    /// Day's high Open Interest
    pub oi_day_high: u32,
    
    /// Day's low Open Interest
    pub oi_day_low: u32,
    
    /// Net change from previous close
    pub net_change: f64,

    /// OHLC data
    pub ohlc: OHLC,
    
    /// Market depth (bid/ask data)
    pub depth: Depth,
}

impl Default for Tick {
    fn default() -> Self {
        Self {
            mode: String::new(),
            instrument_token: 0,
            is_tradable: false,
            is_index: false,
            timestamp: Time::default(),
            last_trade_time: Time::default(),
            last_price: 0.0,
            last_traded_quantity: 0,
            total_buy_quantity: 0,
            total_sell_quantity: 0,
            volume_traded: 0,
            total_buy: 0,
            total_sell: 0,
            average_trade_price: 0.0,
            oi: 0,
            oi_day_high: 0,
            oi_day_low: 0,
            net_change: 0.0,
            ohlc: OHLC {
                instrument_token: None,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
            },
            depth: Depth::default(),
        }
    }
}

impl Tick {
    /// Check if this is a positive price movement
    pub fn is_up(&self) -> bool {
        self.net_change > 0.0
    }

    /// Check if this is a negative price movement
    pub fn is_down(&self) -> bool {
        self.net_change < 0.0
    }

    /// Check if price is unchanged
    pub fn is_unchanged(&self) -> bool {
        self.net_change.abs() < f64::EPSILON
    }

    /// Calculate percentage change
    pub fn change_percentage(&self) -> f64 {
        if self.ohlc.close > 0.0 {
            (self.net_change / self.ohlc.close) * 100.0
        } else {
            0.0
        }
    }

    /// Get market capitalization (for equity instruments)
    /// Note: This would need additional data like shares outstanding
    pub fn market_value_from_price(&self, shares_outstanding: u64) -> f64 {
        self.last_price * shares_outstanding as f64
    }

    /// Calculate the day's range
    pub fn day_range(&self) -> f64 {
        self.ohlc.range()
    }

    /// Check if current price is near day's high (within threshold)
    pub fn is_near_high(&self, threshold_percent: f64) -> bool {
        if self.ohlc.high > 0.0 {
            let diff_percent = ((self.ohlc.high - self.last_price) / self.ohlc.high) * 100.0;
            diff_percent <= threshold_percent
        } else {
            false
        }
    }

    /// Check if current price is near day's low (within threshold)
    pub fn is_near_low(&self, threshold_percent: f64) -> bool {
        if self.ohlc.low > 0.0 {
            let diff_percent = ((self.last_price - self.ohlc.low) / self.ohlc.low) * 100.0;
            diff_percent <= threshold_percent
        } else {
            false
        }
    }
}

/// Segment constants for different exchanges
pub mod segments {
    pub const NSE_CM: u32 = 1;
    pub const NSE_FO: u32 = 2;
    pub const NSE_CD: u32 = 3;
    pub const BSE_CM: u32 = 4;
    pub const BSE_FO: u32 = 5;
    pub const BSE_CD: u32 = 6;
    pub const MCX_FO: u32 = 7;
    pub const MCX_SX: u32 = 8;
    pub const INDICES: u32 = 9;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_operations() {
        let time = Time::from_unix(1625461887);
        assert_eq!(time.unix(), 1625461887);
        assert!(!time.is_zero());
        
        let default_time = Time::default();
        assert!(default_time.is_zero());
    }

    #[test]
    fn test_ohlc_calculations() {
        let ohlc = OHLC {
            instrument_token: None,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0,
        };

        assert_eq!(ohlc.typical_price(), (110.0 + 95.0 + 105.0) / 3.0);
        assert_eq!(ohlc.range(), 15.0);
        assert!(ohlc.is_bullish());
        assert!(!ohlc.is_bearish());
        assert!(!ohlc.is_doji());
    }

    #[test]
    fn test_depth_calculations() {
        let mut depth = Depth::default();
        depth.buy[0] = DepthItem { price: 100.0, quantity: 50, orders: 1 };
        depth.sell[0] = DepthItem { price: 101.0, quantity: 40, orders: 1 };

        assert_eq!(depth.best_bid(), Some(100.0));
        assert_eq!(depth.best_ask(), Some(101.0));
        assert_eq!(depth.spread(), Some(1.0));
    }

    #[test]
    fn test_tick_analysis() {
        let mut tick = Tick::default();
        tick.last_price = 105.0;
        tick.net_change = 5.0;
        tick.ohlc.close = 100.0;
        tick.ohlc.high = 110.0;
        tick.ohlc.low = 95.0;

        assert!(tick.is_up());
        assert!(!tick.is_down());
        assert_eq!(tick.change_percentage(), 5.0);
        assert!(tick.is_near_high(10.0));
        assert!(!tick.is_near_low(5.0));
    }
}
