use crate::models::common::Exchange;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Real-time quote data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,

    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Exchange
    pub exchange: Exchange,

    /// Last traded price
    #[serde(rename = "last_price")]
    pub last_price: f64,

    /// Last traded quantity
    #[serde(rename = "last_quantity")]
    pub last_quantity: u32,

    /// Last traded time
    #[serde(rename = "last_trade_time")]
    pub last_trade_time: DateTime<Utc>,

    /// Average traded price
    #[serde(rename = "average_price")]
    pub average_price: f64,

    /// Volume traded
    pub volume: u64,

    /// Buy quantity
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: u64,

    /// Sell quantity
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: u64,

    /// Open interest (for derivatives)
    #[serde(rename = "oi")]
    pub open_interest: Option<u64>,

    /// Open interest day change
    #[serde(rename = "oi_day_high")]
    pub oi_day_high: Option<u64>,

    /// Open interest day low
    #[serde(rename = "oi_day_low")]
    pub oi_day_low: Option<u64>,

    /// Net change from previous close
    #[serde(rename = "net_change")]
    pub net_change: f64,

    /// OHLC data
    pub ohlc: OHLC,

    /// Market depth
    pub depth: MarketDepth,
}

/// OHLC (Open, High, Low, Close) data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLC {
    /// Opening price
    pub open: f64,

    /// Highest price of the day
    pub high: f64,

    /// Lowest price of the day
    pub low: f64,

    /// Closing price (previous day's close)
    pub close: f64,
}

/// Market depth (order book)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepth {
    /// Buy orders (bids)
    pub buy: Vec<DepthItem>,

    /// Sell orders (asks)
    pub sell: Vec<DepthItem>,
}

/// Individual depth item (bid/ask)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthItem {
    /// Price
    pub price: f64,

    /// Quantity
    pub quantity: u32,

    /// Number of orders
    pub orders: u32,
}

/// LTP (Last Traded Price) data - lightweight quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LTP {
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,

    /// Last traded price
    #[serde(rename = "last_price")]
    pub last_price: f64,
}

/// Quote request for multiple instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRequest {
    /// List of instrument tokens or trading symbols
    pub instruments: Vec<String>,

    /// Exchange (optional for validation)
    pub exchange: Option<Exchange>,
}

/// Historical quote data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalQuote {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// OHLCV data
    #[serde(flatten)]
    pub ohlcv: OHLCV,
}

/// OHLCV (Open, High, Low, Close, Volume) data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCV {
    /// Opening price
    pub open: f64,

    /// Highest price
    pub high: f64,

    /// Lowest price
    pub low: f64,

    /// Closing price
    pub close: f64,

    /// Volume traded
    pub volume: u64,

    /// Open interest (for derivatives)
    pub oi: Option<u64>,
}

impl Quote {
    /// Get the current bid price (highest buy price)
    pub fn bid_price(&self) -> Option<f64> {
        self.depth.buy.first().map(|item| item.price)
    }

    /// Get the current ask price (lowest sell price)
    pub fn ask_price(&self) -> Option<f64> {
        self.depth.sell.first().map(|item| item.price)
    }

    /// Get the bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.bid_price(), self.ask_price()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get the bid-ask spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.bid_price(), self.ask_price()) {
            (Some(bid), Some(ask)) if bid > 0.0 => Some(((ask - bid) / bid) * 100.0),
            _ => None,
        }
    }

    /// Check if the stock is trading higher than previous close
    pub fn is_up(&self) -> bool {
        self.net_change > 0.0
    }

    /// Check if the stock is trading lower than previous close
    pub fn is_down(&self) -> bool {
        self.net_change < 0.0
    }

    /// Check if the stock is unchanged from previous close
    pub fn is_unchanged(&self) -> bool {
        self.net_change == 0.0
    }

    /// Get the percentage change from previous close
    pub fn change_percentage(&self) -> f64 {
        if self.ohlc.close > 0.0 {
            (self.net_change / self.ohlc.close) * 100.0
        } else {
            0.0
        }
    }

    /// Check if the current price is at day high
    pub fn is_at_day_high(&self) -> bool {
        (self.last_price - self.ohlc.high).abs() < 0.01
    }

    /// Check if the current price is at day low
    pub fn is_at_day_low(&self) -> bool {
        (self.last_price - self.ohlc.low).abs() < 0.01
    }

    /// Get the day's range (high - low)
    pub fn day_range(&self) -> f64 {
        self.ohlc.high - self.ohlc.low
    }

    /// Get the day's range percentage
    pub fn day_range_percentage(&self) -> f64 {
        if self.ohlc.low > 0.0 {
            (self.day_range() / self.ohlc.low) * 100.0
        } else {
            0.0
        }
    }

    /// Get position within the day's range (0.0 = at low, 1.0 = at high)
    pub fn position_in_range(&self) -> f64 {
        let range = self.day_range();
        if range > 0.0 {
            (self.last_price - self.ohlc.low) / range
        } else {
            0.5 // If no range, assume middle
        }
    }

    /// Get the total bid quantity (sum of all bid quantities)
    pub fn total_bid_quantity(&self) -> u64 {
        self.depth.buy.iter().map(|item| item.quantity as u64).sum()
    }

    /// Get the total ask quantity (sum of all ask quantities)
    pub fn total_ask_quantity(&self) -> u64 {
        self.depth
            .sell
            .iter()
            .map(|item| item.quantity as u64)
            .sum()
    }

    /// Get the order flow ratio (bid quantity / ask quantity)
    pub fn order_flow_ratio(&self) -> Option<f64> {
        let ask_qty = self.total_ask_quantity();
        if ask_qty > 0 {
            Some(self.total_bid_quantity() as f64 / ask_qty as f64)
        } else {
            None
        }
    }
}

impl OHLC {
    /// Get the trading range (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Get the range percentage relative to close
    pub fn range_percentage(&self) -> f64 {
        if self.close > 0.0 {
            (self.range() / self.close) * 100.0
        } else {
            0.0
        }
    }

    /// Check if it was a positive day (close > open)
    pub fn is_positive(&self) -> bool {
        self.close > self.open
    }

    /// Check if it was a negative day (close < open)
    pub fn is_negative(&self) -> bool {
        self.close < self.open
    }

    /// Check if it was a flat day (close == open)
    pub fn is_flat(&self) -> bool {
        (self.close - self.open).abs() < 0.01
    }

    /// Get the body size (close - open)
    pub fn body_size(&self) -> f64 {
        self.close - self.open
    }

    /// Get the upper shadow size
    pub fn upper_shadow(&self) -> f64 {
        self.high - self.close.max(self.open)
    }

    /// Get the lower shadow size
    pub fn lower_shadow(&self) -> f64 {
        self.close.min(self.open) - self.low
    }
}

impl MarketDepth {
    /// Get the best bid (highest buy price)
    pub fn best_bid(&self) -> Option<&DepthItem> {
        self.buy.first()
    }

    /// Get the best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<&DepthItem> {
        self.sell.first()
    }

    /// Get the total bid volume
    pub fn total_bid_volume(&self) -> u64 {
        self.buy.iter().map(|item| item.quantity as u64).sum()
    }

    /// Get the total ask volume
    pub fn total_ask_volume(&self) -> u64 {
        self.sell.iter().map(|item| item.quantity as u64).sum()
    }

    /// Get the total number of bid orders
    pub fn total_bid_orders(&self) -> u32 {
        self.buy.iter().map(|item| item.orders).sum()
    }

    /// Get the total number of ask orders
    pub fn total_ask_orders(&self) -> u32 {
        self.sell.iter().map(|item| item.orders).sum()
    }
}

impl QuoteRequest {
    /// Create a new quote request
    pub fn new(instruments: Vec<String>) -> Self {
        Self {
            instruments,
            exchange: None,
        }
    }

    /// Add an instrument by token
    pub fn add_token(mut self, token: u32) -> Self {
        self.instruments.push(token.to_string());
        self
    }

    /// Add an instrument by trading symbol
    pub fn add_symbol(mut self, symbol: String) -> Self {
        self.instruments.push(symbol);
        self
    }

    /// Set the exchange for validation
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }
}

impl OHLCV {
    /// Get the typical price (HLC/3)
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Get the weighted close (HLCC/4)
    pub fn weighted_close(&self) -> f64 {
        (self.high + self.low + 2.0 * self.close) / 4.0
    }

    /// Get the midpoint price
    pub fn midpoint(&self) -> f64 {
        (self.high + self.low) / 2.0
    }

    /// Check if it's a bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Check if it's a bearish candle
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// Check if it's a doji candle (open ≈ close)
    pub fn is_doji(&self, threshold: f64) -> bool {
        (self.close - self.open).abs() <= threshold
    }

    /// Get the volume-weighted average price (VWAP) - simplified version
    pub fn vwap(&self) -> f64 {
        self.typical_price() // Simplified; actual VWAP requires more data
    }
}
