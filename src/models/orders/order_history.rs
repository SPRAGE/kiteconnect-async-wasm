use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::{Exchange, Product, Validity, TransactionType, OrderType};
use super::{OrderStatus};

/// Trade data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// Exchange order ID
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: String,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Exchange
    pub exchange: Exchange,
    
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    /// Product type
    pub product: Product,
    
    /// Average price at which the trade was executed
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    /// Quantity traded
    pub quantity: u32,
    
    /// Fill timestamp
    #[serde(rename = "fill_timestamp")]
    pub fill_timestamp: DateTime<Utc>,
    
    /// Exchange timestamp
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: DateTime<Utc>,
    
    /// Transaction type (BUY/SELL)
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
}

/// Order history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderHistoryEntry {
    /// Account ID
    pub account_id: String,
    
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// Exchange order ID
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: Option<String>,
    
    /// Parent order ID
    #[serde(rename = "parent_order_id")]
    pub parent_order_id: Option<String>,
    
    /// Order status
    pub status: OrderStatus,
    
    /// Status message
    #[serde(rename = "status_message")]
    pub status_message: Option<String>,
    
    /// Raw status message from exchange
    #[serde(rename = "status_message_raw")]
    pub status_message_raw: Option<String>,
    
    /// Order timestamp
    #[serde(rename = "order_timestamp")]
    pub order_timestamp: DateTime<Utc>,
    
    /// Exchange timestamp
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: Option<DateTime<Utc>>,
    
    /// Exchange update timestamp
    #[serde(rename = "exchange_update_timestamp")]
    pub exchange_update_timestamp: Option<DateTime<Utc>>,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Exchange
    pub exchange: Exchange,
    
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    /// Order type
    #[serde(rename = "order_type")]
    pub order_type: OrderType,
    
    /// Transaction type
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Validity
    pub validity: Validity,
    
    /// Product
    pub product: Product,
    
    /// Quantity
    pub quantity: u32,
    
    /// Disclosed quantity
    #[serde(rename = "disclosed_quantity")]
    pub disclosed_quantity: u32,
    
    /// Price
    pub price: f64,
    
    /// Trigger price
    #[serde(rename = "trigger_price")]
    pub trigger_price: f64,
    
    /// Average price
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    /// Filled quantity
    #[serde(rename = "filled_quantity")]
    pub filled_quantity: u32,
    
    /// Pending quantity
    #[serde(rename = "pending_quantity")]
    pub pending_quantity: u32,
    
    /// Cancelled quantity
    #[serde(rename = "cancelled_quantity")]
    pub cancelled_quantity: u32,
    
    /// Market protection
    #[serde(rename = "market_protection")]
    pub market_protection: f64,
    
    /// Tag
    pub tag: Option<String>,
    
    /// GUID
    pub guid: String,
    
    /// Variety (regular, bo, co, amo)
    pub variety: Option<String>,
}

/// Order history container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderHistory {
    /// List of order history entries
    pub entries: Vec<OrderHistoryEntry>,
}

/// Trade history container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistory {
    /// List of trades
    pub trades: Vec<Trade>,
}

/// Order book (list of all orders)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// List of orders
    pub orders: Vec<super::Order>,
}

/// Trade book (list of all trades)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeBook {
    /// List of trades
    pub trades: Vec<Trade>,
}

/// Order placement response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Order ID assigned by the system
    #[serde(rename = "order_id")]
    pub order_id: String,
}

impl Trade {
    /// Calculate the total value of the trade
    pub fn total_value(&self) -> f64 {
        self.average_price * self.quantity as f64
    }
    
    /// Check if this is a buy trade
    pub fn is_buy(&self) -> bool {
        self.transaction_type == TransactionType::BUY
    }
    
    /// Check if this is a sell trade
    pub fn is_sell(&self) -> bool {
        self.transaction_type == TransactionType::SELL
    }
}

impl OrderHistory {
    /// Get the latest status of the order
    pub fn latest_status(&self) -> Option<&OrderStatus> {
        self.entries
            .iter()
            .max_by_key(|entry| &entry.order_timestamp)
            .map(|entry| &entry.status)
    }
    
    /// Get all status transitions
    pub fn status_transitions(&self) -> Vec<(&OrderStatus, &DateTime<Utc>)> {
        let mut entries: Vec<_> = self.entries
            .iter()
            .map(|entry| (&entry.status, &entry.order_timestamp))
            .collect();
        
        entries.sort_by_key(|(_, timestamp)| *timestamp);
        entries
    }
    
    /// Check if order was ever rejected
    pub fn was_rejected(&self) -> bool {
        self.entries.iter().any(|entry| entry.status == OrderStatus::Rejected)
    }
    
    /// Check if order was cancelled
    pub fn was_cancelled(&self) -> bool {
        self.entries.iter().any(|entry| entry.status == OrderStatus::Cancelled)
    }
    
    /// Get total filled quantity across all fills
    pub fn total_filled_quantity(&self) -> u32 {
        self.entries
            .iter()
            .map(|entry| entry.filled_quantity)
            .max()
            .unwrap_or(0)
    }
}

impl TradeHistory {
    /// Calculate total traded value
    pub fn total_value(&self) -> f64 {
        self.trades.iter().map(|trade| trade.total_value()).sum()
    }
    
    /// Calculate total quantity traded
    pub fn total_quantity(&self) -> u32 {
        self.trades.iter().map(|trade| trade.quantity).sum()
    }
    
    /// Calculate average price across all trades
    pub fn average_price(&self) -> f64 {
        let total_value = self.total_value();
        let total_quantity = self.total_quantity();
        
        if total_quantity > 0 {
            total_value / total_quantity as f64
        } else {
            0.0
        }
    }
    
    /// Get trades by transaction type
    pub fn trades_by_type(&self, transaction_type: TransactionType) -> Vec<&Trade> {
        self.trades
            .iter()
            .filter(|trade| trade.transaction_type == transaction_type)
            .collect()
    }
    
    /// Get buy trades
    pub fn buy_trades(&self) -> Vec<&Trade> {
        self.trades_by_type(TransactionType::BUY)
    }
    
    /// Get sell trades
    pub fn sell_trades(&self) -> Vec<&Trade> {
        self.trades_by_type(TransactionType::SELL)
    }
}

impl OrderBook {
    /// Get orders by status
    pub fn orders_by_status(&self, status: OrderStatus) -> Vec<&super::Order> {
        self.orders
            .iter()
            .filter(|order| order.status == status)
            .collect()
    }
    
    /// Get open orders
    pub fn open_orders(&self) -> Vec<&super::Order> {
        self.orders.iter().filter(|order| order.is_open()).collect()
    }
    
    /// Get completed orders
    pub fn completed_orders(&self) -> Vec<&super::Order> {
        self.orders.iter().filter(|order| order.is_complete()).collect()
    }
    
    /// Get cancelled orders
    pub fn cancelled_orders(&self) -> Vec<&super::Order> {
        self.orders.iter().filter(|order| order.is_cancelled()).collect()
    }
    
    /// Get rejected orders
    pub fn rejected_orders(&self) -> Vec<&super::Order> {
        self.orders.iter().filter(|order| order.is_rejected()).collect()
    }
    
    /// Find order by ID
    pub fn find_order(&self, order_id: &str) -> Option<&super::Order> {
        self.orders.iter().find(|order| order.order_id == order_id)
    }
    
    /// Get orders by trading symbol
    pub fn orders_by_symbol(&self, symbol: &str) -> Vec<&super::Order> {
        self.orders
            .iter()
            .filter(|order| order.trading_symbol == symbol)
            .collect()
    }
}

impl TradeBook {
    /// Calculate total traded value
    pub fn total_value(&self) -> f64 {
        self.trades.iter().map(|trade| trade.total_value()).sum()
    }
    
    /// Calculate total quantity traded
    pub fn total_quantity(&self) -> u32 {
        self.trades.iter().map(|trade| trade.quantity).sum()
    }
    
    /// Get trades by trading symbol
    pub fn trades_by_symbol(&self, symbol: &str) -> Vec<&Trade> {
        self.trades
            .iter()
            .filter(|trade| trade.trading_symbol == symbol)
            .collect()
    }
    
    /// Get trades by transaction type
    pub fn trades_by_type(&self, transaction_type: TransactionType) -> Vec<&Trade> {
        self.trades
            .iter()
            .filter(|trade| trade.transaction_type == transaction_type)
            .collect()
    }
    
    /// Group trades by trading symbol
    pub fn group_by_symbol(&self) -> std::collections::HashMap<String, Vec<&Trade>> {
        let mut grouped = std::collections::HashMap::new();
        
        for trade in &self.trades {
            grouped
                .entry(trade.trading_symbol.clone())
                .or_insert_with(Vec::new)
                .push(trade);
        }
        
        grouped
    }
}
