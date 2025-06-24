use serde::{Deserialize, Serialize};
use crate::models::common::{Exchange, Product, TransactionType};

/// Position data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Account ID
    pub account_id: String,
    
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
    
    /// Net quantity (positive for long, negative for short)
    pub quantity: i32,
    
    /// Overnight quantity
    #[serde(rename = "overnight_quantity")]
    pub overnight_quantity: i32,
    
    /// Multiplier for the instrument
    pub multiplier: f64,
    
    /// Average price at which the position was taken
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    /// Close price
    #[serde(rename = "close_price")]
    pub close_price: f64,
    
    /// Last price from exchange
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    /// Current value of the position
    pub value: f64,
    
    /// P&L (profit and loss)
    pub pnl: f64,
    
    /// M2M (Mark to Market) P&L
    pub m2m: f64,
    
    /// Unrealised P&L
    pub unrealised: f64,
    
    /// Realised P&L
    pub realised: f64,
    
    /// Buy quantity
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: u32,
    
    /// Buy price
    #[serde(rename = "buy_price")]
    pub buy_price: f64,
    
    /// Buy value
    #[serde(rename = "buy_value")]
    pub buy_value: f64,
    
    /// Buy M2M
    #[serde(rename = "buy_m2m")]
    pub buy_m2m: f64,
    
    /// Sell quantity
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: u32,
    
    /// Sell price
    #[serde(rename = "sell_price")]
    pub sell_price: f64,
    
    /// Sell value
    #[serde(rename = "sell_value")]
    pub sell_value: f64,
    
    /// Sell M2M
    #[serde(rename = "sell_m2m")]
    pub sell_m2m: f64,
    
    /// Day buy quantity
    #[serde(rename = "day_buy_quantity")]
    pub day_buy_quantity: u32,
    
    /// Day buy price
    #[serde(rename = "day_buy_price")]
    pub day_buy_price: f64,
    
    /// Day buy value
    #[serde(rename = "day_buy_value")]
    pub day_buy_value: f64,
    
    /// Day sell quantity
    #[serde(rename = "day_sell_quantity")]
    pub day_sell_quantity: u32,
    
    /// Day sell price
    #[serde(rename = "day_sell_price")]
    pub day_sell_price: f64,
    
    /// Day sell value
    #[serde(rename = "day_sell_value")]
    pub day_sell_value: f64,
}

/// Position type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionType {
    /// Day positions (intraday)
    Day,
    /// Net positions (overnight + intraday)
    Net,
}

/// Positions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionsSummary {
    /// Total P&L across all positions
    pub total_pnl: f64,
    
    /// Total M2M across all positions
    pub total_m2m: f64,
    
    /// Total unrealised P&L
    pub total_unrealised: f64,
    
    /// Total realised P&L
    pub total_realised: f64,
    
    /// Number of positions
    pub positions_count: usize,
    
    /// Number of profitable positions
    pub profitable_positions: usize,
    
    /// Number of loss positions
    pub loss_positions: usize,
}

/// Position conversion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionConversionRequest {
    /// Exchange
    pub exchange: Exchange,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Transaction type
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    
    /// Position type to convert from
    #[serde(rename = "position_type")]
    pub position_type: PositionType,
    
    /// Quantity to convert
    pub quantity: u32,
    
    /// Old product type
    #[serde(rename = "old_product")]
    pub old_product: Product,
    
    /// New product type
    #[serde(rename = "new_product")]
    pub new_product: Product,
}

impl Position {
    /// Check if the position is long (positive quantity)
    pub fn is_long(&self) -> bool {
        self.quantity > 0
    }
    
    /// Check if the position is short (negative quantity)
    pub fn is_short(&self) -> bool {
        self.quantity < 0
    }
    
    /// Check if the position is flat (zero quantity)
    pub fn is_flat(&self) -> bool {
        self.quantity == 0
    }
    
    /// Check if the position is profitable
    pub fn is_profitable(&self) -> bool {
        self.pnl > 0.0
    }
    
    /// Check if the position is in loss
    pub fn is_loss(&self) -> bool {
        self.pnl < 0.0
    }
    
    /// Get the absolute quantity
    pub fn abs_quantity(&self) -> u32 {
        self.quantity.unsigned_abs()
    }
    
    /// Calculate the P&L percentage
    pub fn pnl_percentage(&self) -> f64 {
        let cost = if self.is_long() {
            self.buy_value
        } else {
            self.sell_value
        };
        
        if cost > 0.0 {
            (self.pnl / cost) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get the current market value of the position
    pub fn market_value(&self) -> f64 {
        self.last_price * self.quantity.abs() as f64 * self.multiplier
    }
    
    /// Check if this is a day position (intraday)
    pub fn is_day_position(&self) -> bool {
        self.overnight_quantity == 0
    }
    
    /// Check if this is an overnight position
    pub fn is_overnight_position(&self) -> bool {
        self.overnight_quantity != 0
    }
    
    /// Get the day P&L
    pub fn day_pnl(&self) -> f64 {
        self.pnl - self.unrealised
    }
    
    /// Calculate the change from close price
    pub fn change_from_close(&self) -> f64 {
        self.last_price - self.close_price
    }
    
    /// Calculate the change percentage from close price
    pub fn change_percentage_from_close(&self) -> f64 {
        if self.close_price > 0.0 {
            ((self.last_price - self.close_price) / self.close_price) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get net day quantity
    pub fn net_day_quantity(&self) -> i32 {
        self.day_buy_quantity as i32 - self.day_sell_quantity as i32
    }
    
    /// Check if there was day trading activity
    pub fn has_day_activity(&self) -> bool {
        self.day_buy_quantity > 0 || self.day_sell_quantity > 0
    }
}

impl PositionsSummary {
    /// Calculate from a list of positions
    pub fn from_positions(positions: &[Position]) -> Self {
        let total_pnl = positions.iter().map(|p| p.pnl).sum();
        let total_m2m = positions.iter().map(|p| p.m2m).sum();
        let total_unrealised = positions.iter().map(|p| p.unrealised).sum();
        let total_realised = positions.iter().map(|p| p.realised).sum();
        
        let profitable_positions = positions.iter().filter(|p| p.is_profitable()).count();
        let loss_positions = positions.iter().filter(|p| p.is_loss()).count();
        
        Self {
            total_pnl,
            total_m2m,
            total_unrealised,
            total_realised,
            positions_count: positions.len(),
            profitable_positions,
            loss_positions,
        }
    }
    
    /// Check if the overall portfolio is profitable
    pub fn is_profitable(&self) -> bool {
        self.total_pnl > 0.0
    }
    
    /// Check if the overall portfolio is in loss
    pub fn is_loss(&self) -> bool {
        self.total_pnl < 0.0
    }
    
    /// Get the win rate (percentage of profitable positions)
    pub fn win_rate(&self) -> f64 {
        if self.positions_count > 0 {
            (self.profitable_positions as f64 / self.positions_count as f64) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get the loss rate (percentage of loss positions)
    pub fn loss_rate(&self) -> f64 {
        if self.positions_count > 0 {
            (self.loss_positions as f64 / self.positions_count as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl PositionConversionRequest {
    /// Create a new position conversion request
    pub fn new(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        position_type: PositionType,
        quantity: u32,
        old_product: Product,
        new_product: Product,
    ) -> Self {
        Self {
            exchange,
            trading_symbol,
            transaction_type,
            position_type,
            quantity,
            old_product,
            new_product,
        }
    }
    
    /// Create a request to convert from CNC to MIS
    pub fn cnc_to_mis(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            PositionType::Net,
            quantity,
            Product::CNC,
            Product::MIS,
        )
    }
    
    /// Create a request to convert from MIS to CNC
    pub fn mis_to_cnc(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            PositionType::Day,
            quantity,
            Product::MIS,
            Product::CNC,
        )
    }
}
