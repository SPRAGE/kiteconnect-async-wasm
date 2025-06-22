//! # GTT (Good Till Triggered) Models
//! 
//! This module contains data models for GTT operations in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/gtt.go

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// GTTType represents the available GTT order types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GTTType {
    /// GTTTypeSingle is used to monitor a single trigger value
    Single,
    /// GTTTypeOCO is used to monitor two trigger values where executing one cancels the other
    #[serde(rename = "two-leg")]
    TwoLeg,
}

/// GTTs represents a list of GTT orders.
pub type GTTs = Vec<GTT>;

/// GTTMeta contains information about the rejection reason received after GTT order was triggered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTMeta {
    #[serde(rename = "rejection_reason")]
    pub rejection_reason: Option<String>,
}

/// GTTCondition represents the condition inside a GTT order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTCondition {
    pub exchange: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "trigger_values")]
    pub trigger_values: Vec<f64>,
}

/// GTTOrder represents an order within a GTT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTOrder {
    pub exchange: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub quantity: f64,
    pub price: f64,
    
    #[serde(rename = "order_type")]
    pub order_type: String,
    
    pub product: String,
}

/// GTT represents a single GTT order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTT {
    pub id: u32,
    
    #[serde(rename = "user_id")]
    pub user_id: String,
    
    #[serde(rename = "type")]
    pub gtt_type: GTTType,
    
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    
    #[serde(rename = "expires_at")]
    pub expires_at: DateTime<Utc>,
    
    pub status: String,
    pub condition: GTTCondition,
    pub orders: Vec<GTTOrder>,
    pub meta: GTTMeta,
}

/// TriggerParams represents parameters for a single trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerParams {
    #[serde(rename = "trigger_value")]
    pub trigger_value: f64,
    
    #[serde(rename = "limit_price")]
    pub limit_price: f64,
    
    pub quantity: f64,
}

/// GTTSingleLegTrigger represents a single leg trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTSingleLegTrigger {
    #[serde(flatten)]
    pub trigger_params: TriggerParams,
}

/// GTTOneCancelsOtherTrigger represents an OCO trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTOneCancelsOtherTrigger {
    pub upper: TriggerParams,
    pub lower: TriggerParams,
}

/// GTTTrigger enum represents different types of triggers for GTT orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GTTTrigger {
    #[serde(rename = "single")]
    Single(GTTSingleLegTrigger),
    #[serde(rename = "two-leg")]
    TwoLeg(GTTOneCancelsOtherTrigger),
}

/// GTTParams is a helper struct used to populate an actual GTT before sending it to the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTParams {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub exchange: String,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub product: Option<String>,
    
    pub trigger: GTTTrigger,
}

/// GTTResponse is returned by the API calls to GTT API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTResponse {
    #[serde(rename = "trigger_id")]
    pub trigger_id: u32,
}

impl GTTTrigger {
    pub fn trigger_values(&self) -> Vec<f64> {
        match self {
            GTTTrigger::Single(trigger) => vec![trigger.trigger_params.trigger_value],
            GTTTrigger::TwoLeg(trigger) => vec![trigger.lower.trigger_value, trigger.upper.trigger_value],
        }
    }
    
    pub fn limit_prices(&self) -> Vec<f64> {
        match self {
            GTTTrigger::Single(trigger) => vec![trigger.trigger_params.limit_price],
            GTTTrigger::TwoLeg(trigger) => vec![trigger.lower.limit_price, trigger.upper.limit_price],
        }
    }
    
    pub fn quantities(&self) -> Vec<f64> {
        match self {
            GTTTrigger::Single(trigger) => vec![trigger.trigger_params.quantity],
            GTTTrigger::TwoLeg(trigger) => vec![trigger.lower.quantity, trigger.upper.quantity],
        }
    }
    
    pub fn gtt_type(&self) -> GTTType {
        match self {
            GTTTrigger::Single(_) => GTTType::Single,
            GTTTrigger::TwoLeg(_) => GTTType::TwoLeg,
        }
    }
}

impl GTTSingleLegTrigger {
    /// Create a new single leg trigger.
    pub fn new(trigger_value: f64, limit_price: f64, quantity: f64) -> Self {
        Self {
            trigger_params: TriggerParams {
                trigger_value,
                limit_price,
                quantity,
            },
        }
    }
}

impl GTTOneCancelsOtherTrigger {
    /// Create a new OCO trigger.
    pub fn new(
        lower_trigger: f64,
        lower_limit: f64,
        lower_quantity: f64,
        upper_trigger: f64,
        upper_limit: f64,
        upper_quantity: f64,
    ) -> Self {
        Self {
            lower: TriggerParams {
                trigger_value: lower_trigger,
                limit_price: lower_limit,
                quantity: lower_quantity,
            },
            upper: TriggerParams {
                trigger_value: upper_trigger,
                limit_price: upper_limit,
                quantity: upper_quantity,
            },
        }
    }
}

impl GTT {
    /// Check if the GTT is active.
    pub fn is_active(&self) -> bool {
        self.status.to_lowercase() == "active"
    }
    
    /// Check if the GTT is triggered.
    pub fn is_triggered(&self) -> bool {
        self.status.to_lowercase() == "triggered"
    }
    
    /// Check if the GTT is disabled.
    pub fn is_disabled(&self) -> bool {
        self.status.to_lowercase() == "disabled"
    }
    
    /// Check if the GTT is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
    
    /// Check if the GTT is a single leg type.
    pub fn is_single_leg(&self) -> bool {
        self.gtt_type == GTTType::Single
    }
    
    /// Check if the GTT is an OCO type.
    pub fn is_oco(&self) -> bool {
        self.gtt_type == GTTType::TwoLeg
    }
    
    /// Get the number of trigger values.
    pub fn trigger_count(&self) -> usize {
        self.condition.trigger_values.len()
    }
    
    /// Check if the current price would trigger this GTT.
    pub fn would_trigger(&self, current_price: f64) -> bool {
        match self.gtt_type {
            GTTType::Single => {
                if let Some(&trigger_value) = self.condition.trigger_values.first() {
                    // Simple logic: trigger if current price crosses the trigger value
                    (self.condition.last_price <= trigger_value && current_price >= trigger_value)
                        || (self.condition.last_price >= trigger_value && current_price <= trigger_value)
                } else {
                    false
                }
            }
            GTTType::TwoLeg => {
                // For OCO, check if either trigger value is crossed
                self.condition.trigger_values.iter().any(|&trigger_value| {
                    (self.condition.last_price <= trigger_value && current_price >= trigger_value)
                        || (self.condition.last_price >= trigger_value && current_price <= trigger_value)
                })
            }
        }
    }
}

impl GTTParams {
    /// Create a new GTTParams with a single leg trigger.
    pub fn single_leg(
        tradingsymbol: String,
        exchange: String,
        last_price: f64,
        transaction_type: String,
        trigger_value: f64,
        limit_price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            tradingsymbol,
            exchange,
            last_price,
            transaction_type,
            product: Some("CNC".to_string()),
            trigger: GTTTrigger::Single(GTTSingleLegTrigger::new(trigger_value, limit_price, quantity)),
        }
    }
    
    /// Create a new GTTParams with an OCO trigger.
    pub fn oco(
        tradingsymbol: String,
        exchange: String,
        last_price: f64,
        transaction_type: String,
        lower_trigger: f64,
        lower_limit: f64,
        lower_quantity: f64,
        upper_trigger: f64,
        upper_limit: f64,
        upper_quantity: f64,
    ) -> Self {
        Self {
            tradingsymbol,
            exchange,
            last_price,
            transaction_type,
            product: Some("CNC".to_string()),
            trigger: GTTTrigger::TwoLeg(GTTOneCancelsOtherTrigger::new(
                lower_trigger,
                lower_limit,
                lower_quantity,
                upper_trigger,
                upper_limit,
                upper_quantity,
            )),
        }
    }
    
    /// Set the product type.
    pub fn with_product(mut self, product: String) -> Self {
        self.product = Some(product);
        self
    }
}

// Constants for GTT status
pub const GTT_STATUS_ACTIVE: &str = "active";
pub const GTT_STATUS_TRIGGERED: &str = "triggered";
pub const GTT_STATUS_DISABLED: &str = "disabled";
pub const GTT_STATUS_EXPIRED: &str = "expired";
pub const GTT_STATUS_CANCELLED: &str = "cancelled";

// Constants for transaction types
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";

// Constants for product types
pub const PRODUCT_CNC: &str = "CNC";
pub const PRODUCT_MIS: &str = "MIS";
pub const PRODUCT_NRML: &str = "NRML";

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_single_leg_trigger() {
        let trigger = GTTTrigger::Single(GTTSingleLegTrigger::new(100.0, 105.0, 10.0));
        
        assert_eq!(trigger.trigger_values(), vec![100.0]);
        assert_eq!(trigger.limit_prices(), vec![105.0]);
        assert_eq!(trigger.quantities(), vec![10.0]);
        assert_eq!(trigger.gtt_type(), GTTType::Single);
    }
    
    #[test]
    fn test_oco_trigger() {
        let trigger = GTTTrigger::TwoLeg(GTTOneCancelsOtherTrigger::new(
            95.0, 90.0, 10.0, // Lower leg
            105.0, 110.0, 10.0, // Upper leg
        ));
        
        assert_eq!(trigger.trigger_values(), vec![95.0, 105.0]);
        assert_eq!(trigger.limit_prices(), vec![90.0, 110.0]);
        assert_eq!(trigger.quantities(), vec![10.0, 10.0]);
        assert_eq!(trigger.gtt_type(), GTTType::TwoLeg);
    }
    
    #[test]
    fn test_gtt_status_checks() {
        let mut gtt = GTT {
            id: 123,
            user_id: "test".to_string(),
            gtt_type: GTTType::Single,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(30),
            status: "active".to_string(),
            condition: GTTCondition {
                exchange: "NSE".to_string(),
                tradingsymbol: "INFY".to_string(),
                last_price: 100.0,
                trigger_values: vec![105.0],
            },
            orders: vec![],
            meta: GTTMeta {
                rejection_reason: None,
            },
        };
        
        assert!(gtt.is_active());
        assert!(!gtt.is_triggered());
        assert!(!gtt.is_disabled());
        assert!(!gtt.is_expired());
        assert!(gtt.is_single_leg());
        assert!(!gtt.is_oco());
        assert_eq!(gtt.trigger_count(), 1);
        
        // Test triggering logic
        assert!(gtt.would_trigger(106.0)); // Price crossed above trigger
        assert!(!gtt.would_trigger(104.0)); // Price didn't cross trigger
        
        gtt.status = "triggered".to_string();
        assert!(!gtt.is_active());
        assert!(gtt.is_triggered());
    }
    
    #[test]
    fn test_gtt_params_creation() {
        let params = GTTParams::single_leg(
            "INFY".to_string(),
            "NSE".to_string(),
            100.0,
            "BUY".to_string(),
            105.0,
            110.0,
            10.0,
        );
        
        assert_eq!(params.tradingsymbol, "INFY");
        assert_eq!(params.exchange, "NSE");
        assert_eq!(params.last_price, 100.0);
        assert_eq!(params.transaction_type, "BUY");
        assert_eq!(params.product, Some("CNC".to_string()));
        
        let trigger_values = params.trigger.trigger_values();
        assert_eq!(trigger_values, vec![105.0]);
    }
}
