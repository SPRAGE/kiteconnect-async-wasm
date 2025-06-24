use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::{Exchange, Product, Validity, TransactionType, OrderType};

/// Order data structure as returned by KiteConnect API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Account ID
    pub account_id: String,
    
    /// Order ID assigned by the system
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// Exchange order ID
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: Option<String>,
    
    /// Parent order ID for bracket/cover orders
    #[serde(rename = "parent_order_id")]
    pub parent_order_id: Option<String>,
    
    /// Order status
    pub status: OrderStatus,
    
    /// Status message from exchange
    #[serde(rename = "status_message")]
    pub status_message: Option<String>,
    
    /// Status message from OMS
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
    
    /// Transaction type (BUY/SELL)
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Validity
    pub validity: Validity,
    
    /// Product type
    pub product: Product,
    
    /// Quantity
    pub quantity: u32,
    
    /// Disclosed quantity
    #[serde(rename = "disclosed_quantity")]
    pub disclosed_quantity: u32,
    
    /// Price
    pub price: f64,
    
    /// Trigger price for SL orders
    #[serde(rename = "trigger_price")]
    pub trigger_price: f64,
    
    /// Average price at which the order was executed
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
    
    /// Market protection percentage
    #[serde(rename = "market_protection")]
    pub market_protection: f64,
    
    /// Meta information
    pub meta: Option<OrderMeta>,
    
    /// Tag for the order
    pub tag: Option<String>,
    
    /// GUID for idempotency
    pub guid: String,
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    /// Order is in the system but not yet sent to exchange
    Open,
    /// Order sent to exchange and confirmed
    Complete,
    /// Order is cancelled
    Cancelled,
    /// Order is rejected by system or exchange
    Rejected,
    /// Order placed successfully
    Put,
    /// Order validation passed
    Validated,
    /// Order modification validation passed
    #[serde(rename = "MODIFY VALIDATION PENDING")]
    ModifyValidationPending,
    /// Modification pending
    #[serde(rename = "MODIFY PENDING")]
    ModifyPending,
    /// Order trigger is pending
    #[serde(rename = "TRIGGER PENDING")]
    TriggerPending,
    /// Order cancellation is pending
    #[serde(rename = "CANCEL PENDING")]
    CancelPending,
    /// AMO (After Market Order) placed
    #[serde(rename = "AMO REQ RECEIVED")]
    AmoReqReceived,
}

/// Order meta information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMeta {
    /// Demat consent
    pub demat_consent: Option<String>,
    
    /// ICEBERG leg count
    pub iceberg_legs: Option<u32>,
    
    /// ICEBERG quantity
    pub iceberg_quantity: Option<u32>,
}

/// Order modification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderModification {
    /// Order ID to modify
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// New quantity
    pub quantity: Option<u32>,
    
    /// New price
    pub price: Option<f64>,
    
    /// New trigger price
    #[serde(rename = "trigger_price")]
    pub trigger_price: Option<f64>,
    
    /// New order type
    #[serde(rename = "order_type")]
    pub order_type: Option<OrderType>,
    
    /// New validity
    pub validity: Option<Validity>,
    
    /// New disclosed quantity
    #[serde(rename = "disclosed_quantity")]
    pub disclosed_quantity: Option<u32>,
}

/// Order cancellation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancellation {
    /// Order ID that was cancelled
    #[serde(rename = "order_id")]
    pub order_id: String,
}

/// Bracket order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketOrderResponse {
    /// Parent order details
    pub order_id: String,
    
    /// Child order details (if any)
    pub child_order_ids: Vec<String>,
}

/// Cover order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverOrderResponse {
    /// Order ID
    pub order_id: String,
}

impl Order {
    /// Check if order is open (pending)
    pub fn is_open(&self) -> bool {
        matches!(self.status, OrderStatus::Open | OrderStatus::Put | OrderStatus::Validated)
    }
    
    /// Check if order is completed
    pub fn is_complete(&self) -> bool {
        self.status == OrderStatus::Complete
    }
    
    /// Check if order is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == OrderStatus::Cancelled
    }
    
    /// Check if order is rejected
    pub fn is_rejected(&self) -> bool {
        self.status == OrderStatus::Rejected
    }
    
    /// Get remaining quantity
    pub fn remaining_quantity(&self) -> u32 {
        self.quantity.saturating_sub(self.filled_quantity)
    }
    
    /// Check if order is partially filled
    pub fn is_partially_filled(&self) -> bool {
        self.filled_quantity > 0 && self.filled_quantity < self.quantity
    }
    
    /// Get fill percentage
    pub fn fill_percentage(&self) -> f64 {
        if self.quantity == 0 {
            0.0
        } else {
            (self.filled_quantity as f64 / self.quantity as f64) * 100.0
        }
    }
}

impl OrderStatus {
    /// Check if the status indicates the order is still active
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            OrderStatus::Open
                | OrderStatus::Put
                | OrderStatus::Validated
                | OrderStatus::ModifyValidationPending
                | OrderStatus::ModifyPending
                | OrderStatus::TriggerPending
                | OrderStatus::CancelPending
                | OrderStatus::AmoReqReceived
        )
    }
    
    /// Check if the status indicates the order is final (no more updates expected)
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            OrderStatus::Complete | OrderStatus::Cancelled | OrderStatus::Rejected
        )
    }
}
