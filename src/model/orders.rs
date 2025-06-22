//! # Orders Models
//! 
//! This module contains data models for order-related operations in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/orders.go

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Order represents an individual order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "account_id")]
    pub account_id: String,
    
    #[serde(rename = "placed_by")]
    pub placed_by: String,
    
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: String,
    
    #[serde(rename = "parent_order_id")]
    pub parent_order_id: Option<String>,
    
    pub status: String,
    
    #[serde(rename = "status_message")]
    pub status_message: String,
    
    #[serde(rename = "status_message_raw")]
    pub status_message_raw: String,
    
    #[serde(rename = "order_timestamp")]
    pub order_timestamp: DateTime<Utc>,
    
    #[serde(rename = "exchange_update_timestamp")]
    pub exchange_update_timestamp: DateTime<Utc>,
    
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: DateTime<Utc>,
    
    pub variety: String,
    pub modified: bool,
    pub meta: HashMap<String, serde_json::Value>,
    
    pub exchange: String,
    
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "order_type")]
    pub order_type: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub validity: String,
    
    #[serde(rename = "validity_ttl")]
    pub validity_ttl: i32,
    
    pub product: String,
    pub quantity: f64,
    
    #[serde(rename = "disclosed_quantity")]
    pub disclosed_quantity: f64,
    
    pub price: f64,
    
    #[serde(rename = "trigger_price")]
    pub trigger_price: f64,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "filled_quantity")]
    pub filled_quantity: f64,
    
    #[serde(rename = "pending_quantity")]
    pub pending_quantity: f64,
    
    #[serde(rename = "cancelled_quantity")]
    pub cancelled_quantity: f64,
    
    #[serde(rename = "auction_number")]
    pub auction_number: Option<String>,
    
    pub tag: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Orders represents a list of orders.
pub type Orders = Vec<Order>;

/// OrderParams represents parameters for placing an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderParams {
    pub exchange: Option<String>,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: Option<String>,
    
    pub validity: Option<String>,
    
    #[serde(rename = "validity_ttl")]
    pub validity_ttl: Option<i32>,
    
    pub product: Option<String>,
    
    #[serde(rename = "order_type")]
    pub order_type: Option<String>,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: Option<String>,
    
    pub quantity: Option<i32>,
    
    #[serde(rename = "disclosed_quantity")]
    pub disclosed_quantity: Option<i32>,
    
    pub price: Option<f64>,
    
    #[serde(rename = "trigger_price")]
    pub trigger_price: Option<f64>,
    
    pub squareoff: Option<f64>,
    pub stoploss: Option<f64>,
    
    #[serde(rename = "trailing_stoploss")]
    pub trailing_stoploss: Option<f64>,
    
    #[serde(rename = "iceberg_legs")]
    pub iceberg_legs: Option<i32>,
    
    #[serde(rename = "iceberg_quantity")]
    pub iceberg_qty: Option<i32>,
    
    #[serde(rename = "auction_number")]
    pub auction_number: Option<String>,
    
    pub tag: Option<String>,
}

/// OrderResponse represents the order place success response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
}

/// Trade represents an individual trade response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    pub quantity: f64,
    
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    
    pub product: String,
    
    #[serde(rename = "fill_timestamp")]
    pub fill_timestamp: DateTime<Utc>,
    
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: DateTime<Utc>,
    
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: String,
    
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    pub exchange: String,
    
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
}

/// Trades represents a list of trades.
pub type Trades = Vec<Trade>;

impl Order {
    /// Check if the order is complete (fully filled).
    pub fn is_complete(&self) -> bool {
        self.status.to_lowercase() == "complete"
    }
    
    /// Check if the order is pending.
    pub fn is_pending(&self) -> bool {
        self.status.to_lowercase() == "open"
    }
    
    /// Check if the order is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.status.to_lowercase() == "cancelled"
    }
    
    /// Check if the order is rejected.
    pub fn is_rejected(&self) -> bool {
        self.status.to_lowercase() == "rejected"
    }
    
    /// Get the remaining quantity to be filled.
    pub fn remaining_quantity(&self) -> f64 {
        self.quantity - self.filled_quantity
    }
}

impl OrderParams {
    /// Create a new OrderParams with required fields.
    pub fn new(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        quantity: i32,
        product: String,
        order_type: String,
    ) -> Self {
        Self {
            exchange: Some(exchange),
            tradingsymbol: Some(tradingsymbol),
            transaction_type: Some(transaction_type),
            quantity: Some(quantity),
            product: Some(product),
            order_type: Some(order_type),
            validity: Some("DAY".to_string()),
            validity_ttl: None,
            disclosed_quantity: None,
            price: None,
            trigger_price: None,
            squareoff: None,
            stoploss: None,
            trailing_stoploss: None,
            iceberg_legs: None,
            iceberg_qty: None,
            auction_number: None,
            tag: None,
        }
    }
    
    /// Create a market order.
    pub fn market_order(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        quantity: i32,
        product: String,
    ) -> Self {
        Self::new(
            exchange,
            tradingsymbol,
            transaction_type,
            quantity,
            product,
            "MARKET".to_string(),
        )
    }
    
    /// Create a limit order.
    pub fn limit_order(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        quantity: i32,
        price: f64,
        product: String,
    ) -> Self {
        let mut params = Self::new(
            exchange,
            tradingsymbol,
            transaction_type,
            quantity,
            product,
            "LIMIT".to_string(),
        );
        params.price = Some(price);
        params
    }
    
    /// Create a stop loss order.
    pub fn stop_loss_order(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        quantity: i32,
        trigger_price: f64,
        product: String,
    ) -> Self {
        let mut params = Self::new(
            exchange,
            tradingsymbol,
            transaction_type,
            quantity,
            product,
            "SL-M".to_string(),
        );
        params.trigger_price = Some(trigger_price);
        params
    }
    
    /// Create a stop loss limit order.
    pub fn stop_loss_limit_order(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        quantity: i32,
        price: f64,
        trigger_price: f64,
        product: String,
    ) -> Self {
        let mut params = Self::new(
            exchange,
            tradingsymbol,
            transaction_type,
            quantity,
            product,
            "SL".to_string(),
        );
        params.price = Some(price);
        params.trigger_price = Some(trigger_price);
        params
    }
    
    /// Set the validity of the order.
    pub fn with_validity(mut self, validity: String) -> Self {
        self.validity = Some(validity);
        self
    }
    
    /// Set the validity TTL for TTL orders.
    pub fn with_validity_ttl(mut self, ttl: i32) -> Self {
        self.validity_ttl = Some(ttl);
        self
    }
    
    /// Set the disclosed quantity.
    pub fn with_disclosed_quantity(mut self, disclosed_quantity: i32) -> Self {
        self.disclosed_quantity = Some(disclosed_quantity);
        self
    }
    
    /// Set the tag for the order.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
    
    /// Set iceberg parameters.
    pub fn with_iceberg(mut self, legs: i32, quantity: i32) -> Self {
        self.iceberg_legs = Some(legs);
        self.iceberg_qty = Some(quantity);
        self
    }
}

impl Trade {
    /// Calculate the total value of the trade.
    pub fn total_value(&self) -> f64 {
        self.average_price * self.quantity
    }
    
    /// Check if this is a buy trade.
    pub fn is_buy(&self) -> bool {
        self.transaction_type.to_uppercase() == "BUY"
    }
    
    /// Check if this is a sell trade.
    pub fn is_sell(&self) -> bool {
        self.transaction_type.to_uppercase() == "SELL"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_params_new() {
        let params = OrderParams::new(
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            100,
            "CNC".to_string(),
            "MARKET".to_string(),
        );
        
        assert_eq!(params.exchange, Some("NSE".to_string()));
        assert_eq!(params.tradingsymbol, Some("INFY".to_string()));
        assert_eq!(params.transaction_type, Some("BUY".to_string()));
        assert_eq!(params.quantity, Some(100));
        assert_eq!(params.product, Some("CNC".to_string()));
        assert_eq!(params.order_type, Some("MARKET".to_string()));
    }
    
    #[test]
    fn test_market_order() {
        let params = OrderParams::market_order(
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            100,
            "CNC".to_string(),
        );
        
        assert_eq!(params.order_type, Some("MARKET".to_string()));
        assert_eq!(params.price, None);
    }
    
    #[test]
    fn test_limit_order() {
        let params = OrderParams::limit_order(
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            100,
            1500.0,
            "CNC".to_string(),
        );
        
        assert_eq!(params.order_type, Some("LIMIT".to_string()));
        assert_eq!(params.price, Some(1500.0));
    }
    
    #[test]
    fn test_trade_total_value() {
        let trade = Trade {
            average_price: 1500.0,
            quantity: 100.0,
            trade_id: "123".to_string(),
            product: "CNC".to_string(),
            fill_timestamp: Utc::now(),
            exchange_timestamp: Utc::now(),
            exchange_order_id: "456".to_string(),
            order_id: "789".to_string(),
            transaction_type: "BUY".to_string(),
            trading_symbol: "INFY".to_string(),
            exchange: "NSE".to_string(),
            instrument_token: 408065,
        };
        
        assert_eq!(trade.total_value(), 150000.0);
        assert!(trade.is_buy());
        assert!(!trade.is_sell());
    }
}
