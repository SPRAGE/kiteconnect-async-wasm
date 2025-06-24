use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::TransactionType;

/// MF order data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrder {
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// Exchange order ID
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: Option<String>,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Fund name
    pub fund: String,
    
    /// Order status
    pub status: MFOrderStatus,
    
    /// Status message
    #[serde(rename = "status_message")]
    pub status_message: Option<String>,
    
    /// Folio number
    pub folio: Option<String>,
    
    /// Transaction type (BUY/SELL)
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Amount (for purchases)
    pub amount: Option<f64>,
    
    /// Quantity (for redemptions)
    pub quantity: Option<f64>,
    
    /// Purchase type (FRESH/ADDITIONAL)
    #[serde(rename = "purchase_type")]
    pub purchase_type: Option<String>,
    
    /// Order timestamp
    #[serde(rename = "order_timestamp")]
    pub order_timestamp: DateTime<Utc>,
    
    /// Exchange timestamp
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: Option<DateTime<Utc>>,
    
    /// Settlement ID
    #[serde(rename = "settlement_id")]
    pub settlement_id: Option<String>,
    
    /// Average price (NAV at which units were allotted/redeemed)
    #[serde(rename = "average_price")]
    pub average_price: Option<f64>,
    
    /// Placed by user ID
    #[serde(rename = "placed_by")]
    pub placed_by: String,
    
    /// Tag
    pub tag: Option<String>,
}

/// MF order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MFOrderStatus {
    /// Order placed and pending
    Complete,
    /// Order is cancelled
    Cancelled,
    /// Order is rejected
    Rejected,
}

/// MF order placement parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrderParams {
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Transaction type
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Amount (for purchases)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    
    /// Quantity (for redemptions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    
    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// MF order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrderResponse {
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
}

/// MF orders list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrders {
    /// List of MF orders
    pub orders: Vec<MFOrder>,
}

impl MFOrder {
    /// Check if order is completed
    pub fn is_complete(&self) -> bool {
        self.status == MFOrderStatus::Complete
    }
    
    /// Check if order is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == MFOrderStatus::Cancelled
    }
    
    /// Check if order is rejected
    pub fn is_rejected(&self) -> bool {
        self.status == MFOrderStatus::Rejected
    }
    
    /// Check if this is a purchase order
    pub fn is_purchase(&self) -> bool {
        self.transaction_type == TransactionType::BUY
    }
    
    /// Check if this is a redemption order
    pub fn is_redemption(&self) -> bool {
        self.transaction_type == TransactionType::SELL
    }
    
    /// Get the investment amount (for completed orders)
    pub fn investment_amount(&self) -> Option<f64> {
        match (self.is_purchase(), self.amount, self.average_price, self.quantity) {
            (true, Some(amount), _, _) => Some(amount),
            (false, _, Some(nav), Some(qty)) => Some(nav * qty),
            _ => None,
        }
    }
    
    /// Get the units allotted/redeemed
    pub fn units(&self) -> Option<f64> {
        match (self.is_purchase(), self.amount, self.average_price, self.quantity) {
            (true, Some(amount), Some(nav), _) if nav > 0.0 => Some(amount / nav),
            (false, _, _, Some(qty)) => Some(qty),
            _ => None,
        }
    }
    
    /// Check if order has folio number assigned
    pub fn has_folio(&self) -> bool {
        self.folio.is_some() && !self.folio.as_ref().unwrap().is_empty()
    }
    
    /// Get order value (amount for purchases, NAV*quantity for redemptions)
    pub fn order_value(&self) -> Option<f64> {
        if self.is_purchase() {
            self.amount
        } else {
            match (self.average_price, self.quantity) {
                (Some(nav), Some(qty)) => Some(nav * qty),
                _ => None,
            }
        }
    }
}

impl MFOrderParams {
    /// Create a purchase order
    pub fn purchase(trading_symbol: String, amount: f64) -> Self {
        Self {
            trading_symbol,
            transaction_type: TransactionType::BUY,
            amount: Some(amount),
            quantity: None,
            tag: None,
        }
    }
    
    /// Create a redemption order
    pub fn redemption(trading_symbol: String, quantity: f64) -> Self {
        Self {
            trading_symbol,
            transaction_type: TransactionType::SELL,
            amount: None,
            quantity: Some(quantity),
            tag: None,
        }
    }
    
    /// Add a tag to the order
    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }
    
    /// Validate the order parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.trading_symbol.is_empty() {
            return Err("Trading symbol is required".to_string());
        }
        
        match self.transaction_type {
            TransactionType::BUY => {
                if self.amount.is_none() || self.amount.unwrap() <= 0.0 {
                    return Err("Amount is required and must be positive for purchase orders".to_string());
                }
            }
            TransactionType::SELL => {
                if self.quantity.is_none() || self.quantity.unwrap() <= 0.0 {
                    return Err("Quantity is required and must be positive for redemption orders".to_string());
                }
            }
        }
        
        Ok(())
    }
}

impl MFOrders {
    /// Get orders by status
    pub fn orders_by_status(&self, status: MFOrderStatus) -> Vec<&MFOrder> {
        self.orders
            .iter()
            .filter(|order| order.status == status)
            .collect()
    }
    
    /// Get completed orders
    pub fn completed_orders(&self) -> Vec<&MFOrder> {
        self.orders_by_status(MFOrderStatus::Complete)
    }
    
    /// Get cancelled orders
    pub fn cancelled_orders(&self) -> Vec<&MFOrder> {
        self.orders_by_status(MFOrderStatus::Cancelled)
    }
    
    /// Get rejected orders
    pub fn rejected_orders(&self) -> Vec<&MFOrder> {
        self.orders_by_status(MFOrderStatus::Rejected)
    }
    
    /// Get purchase orders
    pub fn purchase_orders(&self) -> Vec<&MFOrder> {
        self.orders
            .iter()
            .filter(|order| order.is_purchase())
            .collect()
    }
    
    /// Get redemption orders
    pub fn redemption_orders(&self) -> Vec<&MFOrder> {
        self.orders
            .iter()
            .filter(|order| order.is_redemption())
            .collect()
    }
    
    /// Calculate total invested amount (completed purchase orders)
    pub fn total_invested(&self) -> f64 {
        self.purchase_orders()
            .iter()
            .filter(|order| order.is_complete())
            .filter_map(|order| order.amount)
            .sum()
    }
    
    /// Calculate total redeemed amount (completed redemption orders)
    pub fn total_redeemed(&self) -> f64 {
        self.redemption_orders()
            .iter()
            .filter(|order| order.is_complete())
            .filter_map(|order| order.order_value())
            .sum()
    }
    
    /// Find order by ID
    pub fn find_order(&self, order_id: &str) -> Option<&MFOrder> {
        self.orders.iter().find(|order| order.order_id == order_id)
    }
    
    /// Get orders for a specific fund
    pub fn orders_for_fund(&self, trading_symbol: &str) -> Vec<&MFOrder> {
        self.orders
            .iter()
            .filter(|order| order.trading_symbol == trading_symbol)
            .collect()
    }
}
