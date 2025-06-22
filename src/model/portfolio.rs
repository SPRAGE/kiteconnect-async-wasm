//! # Portfolio Models
//! 
//! This module contains data models for portfolio-related operations in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/portfolio.go

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Holding represents an individual holdings response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub exchange: String,
    
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "isin")]
    pub isin: String,
    
    pub product: String,
    
    pub price: f64,
    
    #[serde(rename = "used_quantity")]
    pub used_quantity: i32,
    
    pub quantity: i32,
    
    #[serde(rename = "t1_quantity")]
    pub t1_quantity: i32,
    
    #[serde(rename = "realised_quantity")]
    pub realised_quantity: i32,
    
    #[serde(rename = "authorised_quantity")]
    pub authorised_quantity: i32,
    
    #[serde(rename = "authorised_date")]
    pub authorised_date: DateTime<Utc>,
    
    #[serde(rename = "opening_quantity")]
    pub opening_quantity: i32,
    
    #[serde(rename = "collateral_quantity")]
    pub collateral_quantity: i32,
    
    #[serde(rename = "collateral_type")]
    pub collateral_type: String,
    
    pub discrepancy: bool,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "close_price")]
    pub close_price: f64,
    
    #[serde(rename = "pnl")]
    pub pnl: f64,
    
    #[serde(rename = "day_change")]
    pub day_change: f64,
    
    #[serde(rename = "day_change_percentage")]
    pub day_change_percentage: f64,
    
    pub mtf: MTFHolding,
}

/// MTFHolding represents the MTF details for a holding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTFHolding {
    pub quantity: i32,
    
    #[serde(rename = "used_quantity")]
    pub used_quantity: i32,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    pub value: f64,
    
    #[serde(rename = "initial_margin")]
    pub initial_margin: f64,
}

/// Holdings represents a list of holdings.
pub type Holdings = Vec<Holding>;

/// Position represents an individual position response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    pub exchange: String,
    
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    pub product: String,
    
    pub quantity: i32,
    
    #[serde(rename = "overnight_quantity")]
    pub overnight_quantity: i32,
    
    pub multiplier: f64,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "close_price")]
    pub close_price: f64,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    pub value: f64,
    
    #[serde(rename = "pnl")]
    pub pnl: f64,
    
    #[serde(rename = "m2m")]
    pub m2m: f64,
    
    pub unrealised: f64,
    pub realised: f64,
    
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: i32,
    
    #[serde(rename = "buy_price")]
    pub buy_price: f64,
    
    #[serde(rename = "buy_value")]
    pub buy_value: f64,
    
    #[serde(rename = "buy_m2m")]
    pub buy_m2m_value: f64,
    
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: i32,
    
    #[serde(rename = "sell_price")]
    pub sell_price: f64,
    
    #[serde(rename = "sell_value")]
    pub sell_value: f64,
    
    #[serde(rename = "sell_m2m")]
    pub sell_m2m_value: f64,
    
    #[serde(rename = "day_buy_quantity")]
    pub day_buy_quantity: i32,
    
    #[serde(rename = "day_buy_price")]
    pub day_buy_price: f64,
    
    #[serde(rename = "day_buy_value")]
    pub day_buy_value: f64,
    
    #[serde(rename = "day_sell_quantity")]
    pub day_sell_quantity: i32,
    
    #[serde(rename = "day_sell_price")]
    pub day_sell_price: f64,
    
    #[serde(rename = "day_sell_value")]
    pub day_sell_value: f64,
}

/// Positions represents a list of net and day positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Positions {
    pub net: Vec<Position>,
    pub day: Vec<Position>,
}

/// ConvertPositionParams represents the input params for a position conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertPositionParams {
    pub exchange: String,
    
    #[serde(rename = "tradingsymbol")]
    pub tradingsymbol: String,
    
    #[serde(rename = "old_product")]
    pub old_product: String,
    
    #[serde(rename = "new_product")]
    pub new_product: String,
    
    #[serde(rename = "position_type")]
    pub position_type: String,
    
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    
    pub quantity: i32,
}

/// AuctionInstrument represents the auction instrument available for an auction session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionInstrument {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    pub exchange: String,
    
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    #[serde(rename = "isin")]
    pub isin: String,
    
    pub product: String,
    pub price: f64,
    pub quantity: i32,
    
    #[serde(rename = "t1_quantity")]
    pub t1_quantity: i32,
    
    #[serde(rename = "realised_quantity")]
    pub realised_quantity: i32,
    
    #[serde(rename = "authorised_quantity")]
    pub authorised_quantity: i32,
    
    #[serde(rename = "authorised_date")]
    pub authorised_date: String,
    
    #[serde(rename = "opening_quantity")]
    pub opening_quantity: i32,
    
    #[serde(rename = "collateral_quantity")]
    pub collateral_quantity: i32,
    
    #[serde(rename = "collateral_type")]
    pub collateral_type: String,
    
    pub discrepancy: bool,
    
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    #[serde(rename = "close_price")]
    pub close_price: f64,
    
    #[serde(rename = "pnl")]
    pub pnl: f64,
    
    #[serde(rename = "day_change")]
    pub day_change: f64,
    
    #[serde(rename = "day_change_percentage")]
    pub day_change_percentage: f64,
    
    #[serde(rename = "auction_number")]
    pub auction_number: String,
}

/// HoldingsAuthInstruments represents the instruments and respective quantities for holdings auth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsAuthInstruments {
    #[serde(rename = "isin")]
    pub isin: String,
    
    pub quantity: f64,
}

/// HoldingAuthParams represents the inputs for initiating holdings authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingAuthParams {
    #[serde(rename = "type")]
    pub auth_type: Option<String>,
    
    #[serde(rename = "transfer_type")]
    pub transfer_type: Option<String>,
    
    #[serde(rename = "exec_date")]
    pub exec_date: Option<String>,
    
    /// Instruments are optional
    pub instruments: Option<Vec<HoldingsAuthInstruments>>,
}

/// HoldingsAuthResp represents the response from initiating holdings authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsAuthResp {
    #[serde(rename = "request_id")]
    pub request_id: String,
    
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
}

// Constants for holdings authorization
pub const HOL_AUTH_TYPE_MF: &str = "mf";
pub const HOL_AUTH_TYPE_EQUITY: &str = "equity";
pub const HOL_AUTH_TRANSFER_TYPE_PRE_TRADE: &str = "pre";
pub const HOL_AUTH_TRANSFER_TYPE_POST_TRADE: &str = "post";
pub const HOL_AUTH_TRANSFER_TYPE_OFF_MARKET: &str = "off";
pub const HOL_AUTH_TRANSFER_TYPE_GIFT: &str = "gift";

impl Holding {
    /// Calculate the current market value of the holding.
    pub fn market_value(&self) -> f64 {
        self.last_price * self.quantity as f64
    }
    
    /// Calculate the investment value (cost basis).
    pub fn investment_value(&self) -> f64 {
        self.average_price * self.quantity as f64
    }
    
    /// Calculate the total PnL.
    pub fn total_pnl(&self) -> f64 {
        self.market_value() - self.investment_value()
    }
    
    /// Calculate the PnL percentage.
    pub fn pnl_percentage(&self) -> f64 {
        if self.investment_value() == 0.0 {
            0.0
        } else {
            (self.total_pnl() / self.investment_value()) * 100.0
        }
    }
    
    /// Check if the holding is profitable.
    pub fn is_profitable(&self) -> bool {
        self.pnl > 0.0
    }
}

impl Position {
    /// Check if the position is long.
    pub fn is_long(&self) -> bool {
        self.quantity > 0
    }
    
    /// Check if the position is short.
    pub fn is_short(&self) -> bool {
        self.quantity < 0
    }
    
    /// Check if the position is flat (no quantity).
    pub fn is_flat(&self) -> bool {
        self.quantity == 0
    }
    
    /// Calculate the absolute quantity.
    pub fn abs_quantity(&self) -> i32 {
        self.quantity.abs()
    }
    
    /// Calculate the total PnL including M2M.
    pub fn total_pnl(&self) -> f64 {
        self.pnl + self.m2m
    }
}

impl ConvertPositionParams {
    /// Create a new ConvertPositionParams.
    pub fn new(
        exchange: String,
        tradingsymbol: String,
        old_product: String,
        new_product: String,
        position_type: String,
        transaction_type: String,
        quantity: i32,
    ) -> Self {
        Self {
            exchange,
            tradingsymbol,
            old_product,
            new_product,
            position_type,
            transaction_type,
            quantity,
        }
    }
}

impl HoldingAuthParams {
    /// Create a new HoldingAuthParams.
    pub fn new() -> Self {
        Self {
            auth_type: None,
            transfer_type: None,
            exec_date: None,
            instruments: None,
        }
    }
    
    /// Set the authorization type.
    pub fn with_type(mut self, auth_type: String) -> Self {
        self.auth_type = Some(auth_type);
        self
    }
    
    /// Set the transfer type.
    pub fn with_transfer_type(mut self, transfer_type: String) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }
    
    /// Set the execution date.
    pub fn with_exec_date(mut self, exec_date: String) -> Self {
        self.exec_date = Some(exec_date);
        self
    }
    
    /// Set the instruments.
    pub fn with_instruments(mut self, instruments: Vec<HoldingsAuthInstruments>) -> Self {
        self.instruments = Some(instruments);
        self
    }
}

impl Default for HoldingAuthParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_holding_calculations() {
        let holding = Holding {
            tradingsymbol: "INFY".to_string(),
            exchange: "NSE".to_string(),
            instrument_token: 408065,
            isin: "INE009A01021".to_string(),
            product: "CNC".to_string(),
            price: 1500.0,
            used_quantity: 0,
            quantity: 100,
            t1_quantity: 0,
            realised_quantity: 100,
            authorised_quantity: 100,
            authorised_date: Utc::now(),
            opening_quantity: 100,
            collateral_quantity: 0,
            collateral_type: "".to_string(),
            discrepancy: false,
            average_price: 1400.0,
            last_price: 1500.0,
            close_price: 1495.0,
            pnl: 10000.0,
            day_change: 5.0,
            day_change_percentage: 0.33,
            mtf: MTFHolding {
                quantity: 0,
                used_quantity: 0,
                average_price: 0.0,
                value: 0.0,
                initial_margin: 0.0,
            },
        };
        
        assert_eq!(holding.market_value(), 150000.0);
        assert_eq!(holding.investment_value(), 140000.0);
        assert_eq!(holding.total_pnl(), 10000.0);
        assert!((holding.pnl_percentage() - 7.142857142857143).abs() < 0.00001);
        assert!(holding.is_profitable());
    }
    
    #[test]
    fn test_position_checks() {
        let long_position = Position {
            tradingsymbol: "INFY".to_string(),
            exchange: "NSE".to_string(),
            instrument_token: 408065,
            product: "MIS".to_string(),
            quantity: 100,
            overnight_quantity: 0,
            multiplier: 1.0,
            average_price: 1500.0,
            close_price: 1495.0,
            last_price: 1500.0,
            value: 150000.0,
            pnl: 0.0,
            m2m: 500.0,
            unrealised: 500.0,
            realised: 0.0,
            buy_quantity: 100,
            buy_price: 1500.0,
            buy_value: 150000.0,
            buy_m2m_value: 150500.0,
            sell_quantity: 0,
            sell_price: 0.0,
            sell_value: 0.0,
            sell_m2m_value: 0.0,
            day_buy_quantity: 100,
            day_buy_price: 1500.0,
            day_buy_value: 150000.0,
            day_sell_quantity: 0,
            day_sell_price: 0.0,
            day_sell_value: 0.0,
        };
        
        assert!(long_position.is_long());
        assert!(!long_position.is_short());
        assert!(!long_position.is_flat());
        assert_eq!(long_position.abs_quantity(), 100);
        assert_eq!(long_position.total_pnl(), 500.0);
    }
}
