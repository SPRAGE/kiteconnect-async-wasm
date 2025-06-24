use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::{Exchange, Product, TransactionType, OrderType, GttStatus};

/// GTT trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTCondition {
    /// Exchange
    pub exchange: Exchange,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Trigger values (price levels)
    #[serde(rename = "trigger_values")]
    pub trigger_values: Vec<f64>,
    
    /// Last price (when condition was created)
    #[serde(rename = "last_price")]
    pub last_price: f64,
}

/// GTT order parameters for execution when triggered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTOrderParams {
    /// Exchange
    pub exchange: Exchange,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Transaction type
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Order type
    #[serde(rename = "order_type")]
    pub order_type: OrderType,
    
    /// Product type
    pub product: Product,
    
    /// Quantity
    pub quantity: u32,
    
    /// Price (for limit orders)
    pub price: f64,
    
    /// Result (order ID when triggered, if successful)
    pub result: Option<GTTOrderResult>,
}

/// GTT order execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTOrderResult {
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    /// Rejection reason (if order was rejected)
    #[serde(rename = "rejection_reason")]
    pub rejection_reason: Option<String>,
}

/// GTT trigger type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GTTTriggerType {
    /// Single trigger (one-time)
    Single,
    /// Two-leg trigger (OCO - One Cancels Other)
    #[serde(rename = "two-leg")]
    TwoLeg,
}

/// GTT data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTT {
    /// GTT ID
    pub id: u32,
    
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    
    /// Parent trigger (if this is part of a multi-leg GTT)
    #[serde(rename = "parent_trigger")]
    pub parent_trigger: Option<u32>,
    
    /// GTT type
    #[serde(rename = "type")]
    pub gtt_type: GTTTriggerType,
    
    /// Created timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    
    /// Updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    
    /// Expires at (optional expiry)
    #[serde(rename = "expires_at")]
    pub expires_at: Option<DateTime<Utc>>,
    
    /// GTT status
    pub status: GttStatus,
    
    /// Condition
    pub condition: GTTCondition,
    
    /// Orders to be placed when triggered
    pub orders: Vec<GTTOrderParams>,
    
    /// Metadata
    pub meta: Option<serde_json::Value>,
}

/// GTT creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTCreateParams {
    /// GTT type
    #[serde(rename = "type")]
    pub gtt_type: GTTTriggerType,
    
    /// Condition
    pub condition: GTTCondition,
    
    /// Orders to execute when triggered
    pub orders: Vec<GTTOrderParams>,
    
    /// Expiry time (optional)
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

/// GTT modification parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTModifyParams {
    /// GTT ID
    #[serde(skip_serializing)]
    pub gtt_id: u32,
    
    /// New condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<GTTCondition>,
    
    /// New orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<GTTOrderParams>>,
    
    /// New expiry time
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

/// GTT response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTResponse {
    /// GTT ID
    pub id: u32,
}

/// GTTs collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTs {
    /// List of GTTs
    pub triggers: Vec<GTT>,
}

impl GTT {
    /// Check if GTT is active
    pub fn is_active(&self) -> bool {
        self.status == GttStatus::Active
    }
    
    /// Check if GTT is triggered
    pub fn is_triggered(&self) -> bool {
        self.status == GttStatus::Triggered
    }
    
    /// Check if GTT is disabled
    pub fn is_disabled(&self) -> bool {
        self.status == GttStatus::Disabled
    }
    
    /// Check if GTT is expired
    pub fn is_expired(&self) -> bool {
        self.status == GttStatus::Expired
    }
    
    /// Check if GTT is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == GttStatus::Cancelled
    }
    
    /// Check if GTT is rejected
    pub fn is_rejected(&self) -> bool {
        self.status == GttStatus::Rejected
    }
    
    /// Check if GTT has expired based on expiry time
    pub fn has_time_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }
    
    /// Get time remaining until expiry
    pub fn time_to_expiry(&self) -> Option<chrono::Duration> {
        self.expires_at.map(|expires_at| expires_at - chrono::Utc::now())
    }
    
    /// Check if this is a single trigger GTT
    pub fn is_single_trigger(&self) -> bool {
        self.gtt_type == GTTTriggerType::Single
    }
    
    /// Check if this is a two-leg (OCO) GTT
    pub fn is_two_leg(&self) -> bool {
        self.gtt_type == GTTTriggerType::TwoLeg
    }
    
    /// Get all trigger values
    pub fn trigger_values(&self) -> &[f64] {
        &self.condition.trigger_values
    }
    
    /// Check if current price would trigger the GTT
    pub fn would_trigger(&self, current_price: f64) -> bool {
        let last_price = self.condition.last_price;
        
        self.condition.trigger_values.iter().any(|&trigger_price| {
            // Check if price has crossed the trigger level
            (last_price <= trigger_price && current_price >= trigger_price) ||
            (last_price >= trigger_price && current_price <= trigger_price)
        })
    }
    
    /// Get the number of orders to be executed
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }
    
    /// Check if any orders were successfully placed
    pub fn has_successful_orders(&self) -> bool {
        self.orders.iter().any(|order| {
            order.result.as_ref()
                .map(|result| !result.order_id.is_empty())
                .unwrap_or(false)
        })
    }
    
    /// Get successful order IDs
    pub fn successful_order_ids(&self) -> Vec<&str> {
        self.orders
            .iter()
            .filter_map(|order| {
                order.result.as_ref()
                    .and_then(|result| {
                        if !result.order_id.is_empty() {
                            Some(result.order_id.as_str())
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
    
    /// Get failed orders with rejection reasons
    pub fn failed_orders(&self) -> Vec<(&GTTOrderParams, &str)> {
        self.orders
            .iter()
            .filter_map(|order| {
                order.result.as_ref()
                    .and_then(|result| {
                        result.rejection_reason.as_ref()
                            .map(|reason| (order, reason.as_str()))
                    })
            })
            .collect()
    }
}

impl GTTCreateParams {
    /// Create a single trigger GTT
    pub fn single(
        condition: GTTCondition,
        order: GTTOrderParams,
    ) -> Self {
        Self {
            gtt_type: GTTTriggerType::Single,
            condition,
            orders: vec![order],
            expires_at: None,
        }
    }
    
    /// Create a two-leg (OCO) GTT
    pub fn two_leg(
        condition: GTTCondition,
        orders: Vec<GTTOrderParams>,
    ) -> Self {
        Self {
            gtt_type: GTTTriggerType::TwoLeg,
            condition,
            orders,
            expires_at: None,
        }
    }
    
    /// Set expiry time
    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
    
    /// Validate GTT parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.condition.trigger_values.is_empty() {
            return Err("At least one trigger value is required".to_string());
        }
        
        if self.orders.is_empty() {
            return Err("At least one order is required".to_string());
        }
        
        match self.gtt_type {
            GTTTriggerType::Single => {
                if self.orders.len() > 1 {
                    return Err("Single trigger GTT can have only one order".to_string());
                }
                if self.condition.trigger_values.len() > 1 {
                    return Err("Single trigger GTT can have only one trigger value".to_string());
                }
            }
            GTTTriggerType::TwoLeg => {
                if self.orders.len() != 2 {
                    return Err("Two-leg GTT must have exactly two orders".to_string());
                }
                if self.condition.trigger_values.len() != 2 {
                    return Err("Two-leg GTT must have exactly two trigger values".to_string());
                }
            }
        }
        
        Ok(())
    }
}

impl GTTs {
    /// Get active GTTs
    pub fn active_gtts(&self) -> Vec<&GTT> {
        self.triggers.iter().filter(|gtt| gtt.is_active()).collect()
    }
    
    /// Get triggered GTTs
    pub fn triggered_gtts(&self) -> Vec<&GTT> {
        self.triggers.iter().filter(|gtt| gtt.is_triggered()).collect()
    }
    
    /// Get expired GTTs
    pub fn expired_gtts(&self) -> Vec<&GTT> {
        self.triggers.iter().filter(|gtt| gtt.is_expired()).collect()
    }
    
    /// Get GTTs for a specific symbol
    pub fn gtts_for_symbol(&self, symbol: &str) -> Vec<&GTT> {
        self.triggers
            .iter()
            .filter(|gtt| gtt.condition.trading_symbol == symbol)
            .collect()
    }
    
    /// Find GTT by ID
    pub fn find_gtt(&self, gtt_id: u32) -> Option<&GTT> {
        self.triggers.iter().find(|gtt| gtt.id == gtt_id)
    }
    
    /// Get GTTs expiring soon
    pub fn expiring_soon(&self, hours: i64) -> Vec<&GTT> {
        let threshold = chrono::Utc::now() + chrono::Duration::hours(hours);
        
        self.active_gtts()
            .into_iter()
            .filter(|gtt| {
                gtt.expires_at
                    .map(|expires_at| expires_at <= threshold)
                    .unwrap_or(false)
            })
            .collect()
    }
}
