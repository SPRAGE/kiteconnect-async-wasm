//! # Mutual Funds Models
//! 
//! This module contains data models for mutual fund operations in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/mutualfunds.go

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// MFHolding represents an individual mutual fund holding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFHolding {
    pub folio: String,
    pub fund: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "last_price_date")]
    pub last_price_date: String,
    
    #[serde(rename = "pnl")]
    pub pnl: f64,
    
    pub quantity: f64,
}

/// MFHoldings represents a list of mutual fund holdings.
pub type MFHoldings = Vec<MFHolding>;

/// MFTrade represents an individual trade of a mutual fund holding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFTrade {
    pub fund: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    pub variety: String,
    
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: DateTime<Utc>,
    
    pub amount: f64,
    pub folio: String,
    pub quantity: f64,
}

/// MFHoldingBreakdown represents a list of mutual fund holdings breakdown.
pub type MFHoldingBreakdown = Vec<MFTrade>;

/// MFOrder represents an individual mutual fund order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    
    #[serde(rename = "exchange_order_id")]
    pub exchange_order_id: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub status: String,
    
    #[serde(rename = "status_message")]
    pub status_message: String,
    
    pub folio: String,
    pub fund: String,
    
    #[serde(rename = "order_timestamp")]
    pub order_timestamp: DateTime<Utc>,
    
    #[serde(rename = "exchange_timestamp")]
    pub exchange_timestamp: DateTime<Utc>,
    
    #[serde(rename = "settlement_id")]
    pub settlement_id: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub variety: String,
    
    #[serde(rename = "purchase_type")]
    pub purchase_type: String,
    
    pub quantity: f64,
    pub amount: f64,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "placed_by")]
    pub placed_by: String,
    
    pub tag: String,
}

/// MFOrders represents a list of mutual fund orders.
pub type MFOrders = Vec<MFOrder>;

/// MFAllottedISINs represents a list of all ISINs in which at least one allotment is present.
pub type MFAllottedISINs = Vec<String>;

/// MFSIPStepUp represents stepup date and percentage for SIPs.
pub type MFSIPStepUp = HashMap<String, i32>;

/// MFSIP represents an individual mutual fund SIP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFSIP {
    #[serde(rename = "sip_id")]
    pub id: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "fund")]
    pub fund_name: String,
    
    #[serde(rename = "dividend_type")]
    pub dividend_type: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub status: String,
    
    #[serde(rename = "sip_type")]
    pub sip_type: String,
    
    pub created: DateTime<Utc>,
    pub frequency: String,
    
    #[serde(rename = "instalment_amount")]
    pub instalment_amount: f64,
    
    pub instalments: i32,
    
    #[serde(rename = "last_instalment")]
    pub last_instalment: DateTime<Utc>,
    
    #[serde(rename = "pending_instalments")]
    pub pending_instalments: i32,
    
    #[serde(rename = "instalment_day")]
    pub instalment_day: i32,
    
    #[serde(rename = "completed_instalments")]
    pub completed_instalments: i32,
    
    #[serde(rename = "next_instalment")]
    pub next_instalment: String,
    
    #[serde(rename = "trigger_price")]
    pub trigger_price: f64,
    
    #[serde(rename = "step_up")]
    pub step_up: MFSIPStepUp,
    
    pub tag: String,
}

/// MFSIPs represents a list of mutual fund SIPs.
pub type MFSIPs = Vec<MFSIP>;

/// MFOrderResponse represents the successful order place response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
}

/// MFSIPResponse represents the successful SIP place response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFSIPResponse {
    #[serde(rename = "order_id")]
    pub order_id: Option<String>,
    
    #[serde(rename = "sip_id")]
    pub sip_id: String,
}

/// MFOrderParams represents parameters for placing a mutual fund order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFOrderParams {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub quantity: Option<f64>,
    pub amount: Option<f64>,
    pub tag: Option<String>,
}

/// MFSIPParams represents parameters for placing a SIP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFSIPParams {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub amount: f64,
    pub instalments: i32,
    pub frequency: String,
    
    #[serde(rename = "instalment_day")]
    pub instalment_day: Option<i32>,
    
    #[serde(rename = "initial_amount")]
    pub initial_amount: Option<f64>,
    
    #[serde(rename = "trigger_price")]
    pub trigger_price: Option<f64>,
    
    #[serde(rename = "step_up")]
    pub step_up: Option<String>,
    
    #[serde(rename = "sip_type")]
    pub sip_type: Option<String>,
    
    pub tag: Option<String>,
}

/// MFSIPModifyParams represents parameters for modifying a SIP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFSIPModifyParams {
    pub amount: Option<f64>,
    pub frequency: Option<String>,
    
    #[serde(rename = "instalment_day")]
    pub instalment_day: Option<i32>,
    
    pub instalments: Option<i32>,
    
    #[serde(rename = "step_up")]
    pub step_up: Option<String>,
    
    pub status: Option<String>,
}

impl MFHolding {
    /// Calculate the current market value of the holding.
    pub fn market_value(&self) -> f64 {
        self.last_price * self.quantity
    }
    
    /// Calculate the investment value (cost basis).
    pub fn investment_value(&self) -> f64 {
        self.average_price * self.quantity
    }
    
    /// Calculate the absolute PnL.
    pub fn absolute_pnl(&self) -> f64 {
        self.market_value() - self.investment_value()
    }
    
    /// Calculate the PnL percentage.
    pub fn pnl_percentage(&self) -> f64 {
        if self.investment_value() == 0.0 {
            0.0
        } else {
            (self.absolute_pnl() / self.investment_value()) * 100.0
        }
    }
    
    /// Check if the holding is profitable.
    pub fn is_profitable(&self) -> bool {
        self.pnl > 0.0
    }
}

impl MFOrder {
    /// Check if the order is complete.
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
    
    /// Check if this is a purchase order.
    pub fn is_purchase(&self) -> bool {
        self.transaction_type.to_uppercase() == "BUY"
    }
    
    /// Check if this is a redemption order.
    pub fn is_redemption(&self) -> bool {
        self.transaction_type.to_uppercase() == "SELL"
    }
    
    /// Calculate the total value of the order.
    pub fn total_value(&self) -> f64 {
        if self.amount > 0.0 {
            self.amount
        } else {
            self.quantity * self.average_price
        }
    }
}

impl MFSIP {
    /// Check if the SIP is active.
    pub fn is_active(&self) -> bool {
        self.status.to_lowercase() == "active"
    }
    
    /// Check if the SIP is paused.
    pub fn is_paused(&self) -> bool {
        self.status.to_lowercase() == "paused"
    }
    
    /// Check if the SIP is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.status.to_lowercase() == "cancelled"
    }
    
    /// Check if the SIP is completed.
    pub fn is_completed(&self) -> bool {
        self.status.to_lowercase() == "completed"
    }
    
    /// Calculate the total amount invested so far.
    pub fn total_invested(&self) -> f64 {
        self.instalment_amount * self.completed_instalments as f64
    }
    
    /// Calculate the remaining amount to be invested.
    pub fn remaining_amount(&self) -> f64 {
        self.instalment_amount * self.pending_instalments as f64
    }
    
    /// Calculate the completion percentage.
    pub fn completion_percentage(&self) -> f64 {
        if self.instalments == 0 {
            0.0
        } else {
            (self.completed_instalments as f64 / self.instalments as f64) * 100.0
        }
    }
}

impl MFOrderParams {
    /// Create a new MFOrderParams for purchase by amount.
    pub fn buy_by_amount(tradingsymbol: String, amount: f64) -> Self {
        Self {
            tradingsymbol,
            transaction_type: "BUY".to_string(),
            quantity: None,
            amount: Some(amount),
            tag: None,
        }
    }
    
    /// Create a new MFOrderParams for purchase by quantity.
    pub fn buy_by_quantity(tradingsymbol: String, quantity: f64) -> Self {
        Self {
            tradingsymbol,
            transaction_type: "BUY".to_string(),
            quantity: Some(quantity),
            amount: None,
            tag: None,
        }
    }
    
    /// Create a new MFOrderParams for redemption by quantity.
    pub fn sell_by_quantity(tradingsymbol: String, quantity: f64) -> Self {
        Self {
            tradingsymbol,
            transaction_type: "SELL".to_string(),
            quantity: Some(quantity),
            amount: None,
            tag: None,
        }
    }
    
    /// Set the tag for the order.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
}

impl MFSIPParams {
    /// Create a new MFSIPParams.
    pub fn new(
        tradingsymbol: String,
        amount: f64,
        instalments: i32,
        frequency: String,
    ) -> Self {
        Self {
            tradingsymbol,
            amount,
            instalments,
            frequency,
            instalment_day: None,
            initial_amount: None,
            trigger_price: None,
            step_up: None,
            sip_type: None,
            tag: None,
        }
    }
    
    /// Set the instalment day.
    pub fn with_instalment_day(mut self, day: i32) -> Self {
        self.instalment_day = Some(day);
        self
    }
    
    /// Set the initial amount.
    pub fn with_initial_amount(mut self, amount: f64) -> Self {
        self.initial_amount = Some(amount);
        self
    }
    
    /// Set the trigger price.
    pub fn with_trigger_price(mut self, price: f64) -> Self {
        self.trigger_price = Some(price);
        self
    }
    
    /// Set the tag.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
}

impl MFSIPModifyParams {
    /// Create a new MFSIPModifyParams.
    pub fn new() -> Self {
        Self {
            amount: None,
            frequency: None,
            instalment_day: None,
            instalments: None,
            step_up: None,
            status: None,
        }
    }
    
    /// Set the amount.
    pub fn with_amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    
    /// Set the frequency.
    pub fn with_frequency(mut self, frequency: String) -> Self {
        self.frequency = Some(frequency);
        self
    }
    
    /// Set the status.
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    
    /// Pause the SIP.
    pub fn pause(mut self) -> Self {
        self.status = Some("paused".to_string());
        self
    }
    
    /// Resume the SIP.
    pub fn resume(mut self) -> Self {
        self.status = Some("active".to_string());
        self
    }
}

impl Default for MFSIPModifyParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mf_holding_calculations() {
        let holding = MFHolding {
            folio: "12345".to_string(),
            fund: "Test Fund".to_string(),
            tradingsymbol: "TESTFUND".to_string(),
            average_price: 100.0,
            last_price: 110.0,
            last_price_date: "2023-01-01".to_string(),
            pnl: 100.0,
            quantity: 10.0,
        };
        
        assert_eq!(holding.market_value(), 1100.0);
        assert_eq!(holding.investment_value(), 1000.0);
        assert_eq!(holding.absolute_pnl(), 100.0);
        assert_eq!(holding.pnl_percentage(), 10.0);
        assert!(holding.is_profitable());
    }
    
    #[test]
    fn test_mf_order_checks() {
        let order = MFOrder {
            order_id: "123".to_string(),
            exchange_order_id: "456".to_string(),
            tradingsymbol: "TESTFUND".to_string(),
            status: "COMPLETE".to_string(),
            status_message: "Order executed".to_string(),
            folio: "12345".to_string(),
            fund: "Test Fund".to_string(),
            order_timestamp: Utc::now(),
            exchange_timestamp: Utc::now(),
            settlement_id: "789".to_string(),
            transaction_type: "BUY".to_string(),
            variety: "regular".to_string(),
            purchase_type: "FRESH".to_string(),
            quantity: 10.0,
            amount: 1000.0,
            last_price: 100.0,
            average_price: 100.0,
            placed_by: "user".to_string(),
            tag: "".to_string(),
        };
        
        assert!(order.is_complete());
        assert!(!order.is_pending());
        assert!(order.is_purchase());
        assert!(!order.is_redemption());
        assert_eq!(order.total_value(), 1000.0);
    }
    
    #[test]
    fn test_mf_sip_calculations() {
        let sip = MFSIP {
            id: "123".to_string(),
            tradingsymbol: "TESTFUND".to_string(),
            fund_name: "Test Fund".to_string(),
            dividend_type: "growth".to_string(),
            transaction_type: "BUY".to_string(),
            status: "ACTIVE".to_string(),
            sip_type: "NORMAL".to_string(),
            created: Utc::now(),
            frequency: "monthly".to_string(),
            instalment_amount: 1000.0,
            instalments: 12,
            last_instalment: Utc::now(),
            pending_instalments: 8,
            instalment_day: 5,
            completed_instalments: 4,
            next_instalment: "2023-02-05".to_string(),
            trigger_price: 0.0,
            step_up: HashMap::new(),
            tag: "".to_string(),
        };
        
        assert!(sip.is_active());
        assert!(!sip.is_paused());
        assert_eq!(sip.total_invested(), 4000.0);
        assert_eq!(sip.remaining_amount(), 8000.0);
        assert!((sip.completion_percentage() - 33.333333333333336).abs() < 0.00001);
    }
    
    #[test]
    fn test_mf_order_params() {
        let params = MFOrderParams::buy_by_amount("TESTFUND".to_string(), 1000.0)
            .with_tag("test".to_string());
        
        assert_eq!(params.tradingsymbol, "TESTFUND");
        assert_eq!(params.transaction_type, "BUY");
        assert_eq!(params.amount, Some(1000.0));
        assert_eq!(params.quantity, None);
        assert_eq!(params.tag, Some("test".to_string()));
    }
}
