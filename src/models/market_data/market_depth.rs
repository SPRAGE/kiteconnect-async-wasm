use serde::{Deserialize, Serialize};

/// Market depth represents the order book with buy and sell orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepthFull {
    /// Buy orders (bids)
    pub buy: Vec<DepthLevel>,

    /// Sell orders (asks)
    pub sell: Vec<DepthLevel>,
}

/// Individual depth level (price level in the order book)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthLevel {
    /// Price level
    pub price: f64,

    /// Total quantity at this price level
    pub quantity: u32,

    /// Number of orders at this price level
    pub orders: u32,
}

/// Level 2 market data (best bid/ask)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level2Data {
    /// Best bid price
    pub bid_price: f64,

    /// Best bid quantity
    pub bid_quantity: u32,

    /// Best ask price
    pub ask_price: f64,

    /// Best ask quantity
    pub ask_quantity: u32,

    /// Timestamp of the data
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MarketDepthFull {
    /// Get the best bid (highest buy price)
    pub fn best_bid(&self) -> Option<&DepthLevel> {
        self.buy.first()
    }

    /// Get the best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<&DepthLevel> {
        self.sell.first()
    }

    /// Get the spread between best bid and ask
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// Get the mid price (average of best bid and ask)
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid.price + ask.price) / 2.0),
            _ => None,
        }
    }

    /// Get total bid volume
    pub fn total_bid_volume(&self) -> u64 {
        self.buy.iter().map(|level| level.quantity as u64).sum()
    }

    /// Get total ask volume
    pub fn total_ask_volume(&self) -> u64 {
        self.sell.iter().map(|level| level.quantity as u64).sum()
    }
}

impl Level2Data {
    /// Get the spread
    pub fn spread(&self) -> f64 {
        self.ask_price - self.bid_price
    }

    /// Get the mid price
    pub fn mid_price(&self) -> f64 {
        (self.bid_price + self.ask_price) / 2.0
    }

    /// Get the spread percentage
    pub fn spread_percentage(&self) -> f64 {
        if self.bid_price > 0.0 {
            (self.spread() / self.bid_price) * 100.0
        } else {
            0.0
        }
    }
}
