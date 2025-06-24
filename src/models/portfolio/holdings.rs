use serde::{Deserialize, Serialize};
use crate::models::common::{Exchange, Product};

/// Holdings data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    /// Account ID
    pub account_id: String,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Exchange
    pub exchange: Exchange,
    
    /// ISIN (International Securities Identification Number)
    pub isin: String,
    
    /// Product type
    pub product: Product,
    
    /// Instrument token
    #[serde(rename = "instrument_token")]
    pub instrument_token: u32,
    
    /// Quantity in the holding
    pub quantity: i32,
    
    /// T1 quantity (can be sold after T+1 day)
    #[serde(rename = "t1_quantity")]
    pub t1_quantity: i32,
    
    /// Realised quantity (can be sold immediately)
    #[serde(rename = "realised_quantity")]
    pub realised_quantity: i32,
    
    /// Authorized quantity (pledged/unpledged)
    #[serde(rename = "authorised_quantity")]
    pub authorised_quantity: i32,
    
    /// Authorised date
    #[serde(rename = "authorised_date")]
    pub authorised_date: Option<String>,
    
    /// Opening quantity at the start of the day
    #[serde(rename = "opening_quantity")]
    pub opening_quantity: i32,
    
    /// Collateral quantity
    #[serde(rename = "collateral_quantity")]
    pub collateral_quantity: i32,
    
    /// Collateral type
    #[serde(rename = "collateral_type")]
    pub collateral_type: Option<String>,
    
    /// Collateral update quantity
    #[serde(rename = "collateral_update_quantity")]
    pub collateral_update_quantity: i32,
    
    /// Discrepancy flag
    pub discrepancy: bool,
    
    /// Average price at which the stock was bought
    #[serde(rename = "average_price")]
    pub average_price: f64,
    
    /// Last price from exchange
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    /// Close price
    #[serde(rename = "close_price")]
    pub close_price: f64,
    
    /// Price change
    #[serde(rename = "price_change")]
    pub price_change: f64,
    
    /// P&L (profit and loss)
    pub pnl: f64,
    
    /// Day change
    #[serde(rename = "day_change")]
    pub day_change: f64,
    
    /// Day change percentage
    #[serde(rename = "day_change_percentage")]
    pub day_change_percentage: f64,
    
    /// Used quantity (used for pledging)
    #[serde(rename = "used_quantity")]
    pub used_quantity: i32,
}

/// Holdings summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsSummary {
    /// Total holdings value at current market price
    pub total_value: f64,
    
    /// Total investment value (at average price)
    pub total_investment: f64,
    
    /// Total P&L across all holdings
    pub total_pnl: f64,
    
    /// Total day change
    pub total_day_change: f64,
    
    /// Total day change percentage
    pub total_day_change_percentage: f64,
    
    /// Number of holdings
    pub holdings_count: usize,
}

/// Portfolio profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioProfile {
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    
    /// Equity used
    #[serde(rename = "equity_used")]
    pub equity_used: f64,
    
    /// Equity available
    #[serde(rename = "equity_available")]
    pub equity_available: f64,
    
    /// Commodity used
    #[serde(rename = "commodity_used")]
    pub commodity_used: f64,
    
    /// Commodity available
    #[serde(rename = "commodity_available")]
    pub commodity_available: f64,
}

impl Holding {
    /// Calculate current market value of the holding
    pub fn market_value(&self) -> f64 {
        self.last_price * self.quantity as f64
    }
    
    /// Calculate investment value (at average price)
    pub fn investment_value(&self) -> f64 {
        self.average_price * self.quantity as f64
    }
    
    /// Calculate the P&L percentage
    pub fn pnl_percentage(&self) -> f64 {
        let investment = self.investment_value();
        if investment > 0.0 {
            (self.pnl / investment) * 100.0
        } else {
            0.0
        }
    }
    
    /// Check if the holding is profitable
    pub fn is_profitable(&self) -> bool {
        self.pnl > 0.0
    }
    
    /// Check if the holding is in loss
    pub fn is_loss(&self) -> bool {
        self.pnl < 0.0
    }
    
    /// Get available quantity for trading
    pub fn available_quantity(&self) -> i32 {
        self.realised_quantity + self.t1_quantity
    }
    
    /// Check if quantity can be sold today
    pub fn can_sell_today(&self) -> bool {
        self.realised_quantity > 0
    }
    
    /// Get quantity that can be sold today
    pub fn sellable_today(&self) -> i32 {
        self.realised_quantity
    }
    
    /// Get quantity that can be sold tomorrow (T+1)
    pub fn sellable_tomorrow(&self) -> i32 {
        self.t1_quantity
    }
    
    /// Check if the holding has any discrepancy
    pub fn has_discrepancy(&self) -> bool {
        self.discrepancy
    }
    
    /// Get the change from previous day close
    pub fn change_from_close(&self) -> f64 {
        self.last_price - self.close_price
    }
    
    /// Get the change percentage from previous day close
    pub fn change_percentage_from_close(&self) -> f64 {
        if self.close_price > 0.0 {
            ((self.last_price - self.close_price) / self.close_price) * 100.0
        } else {
            0.0
        }
    }
    
    /// Check if the holding is pledged
    pub fn is_pledged(&self) -> bool {
        self.used_quantity > 0
    }
    
    /// Get unpledged quantity
    pub fn unpledged_quantity(&self) -> i32 {
        self.quantity - self.used_quantity
    }
}

impl HoldingsSummary {
    /// Calculate from a list of holdings
    pub fn from_holdings(holdings: &[Holding]) -> Self {
        let total_value = holdings.iter().map(|h| h.market_value()).sum();
        let total_investment = holdings.iter().map(|h| h.investment_value()).sum();
        let total_pnl = holdings.iter().map(|h| h.pnl).sum();
        let total_day_change = holdings.iter().map(|h| h.day_change).sum();
        
        let total_day_change_percentage = if total_investment > 0.0 {
            (total_day_change / total_investment) * 100.0
        } else {
            0.0
        };
        
        Self {
            total_value,
            total_investment,
            total_pnl,
            total_day_change,
            total_day_change_percentage,
            holdings_count: holdings.len(),
        }
    }
    
    /// Get the overall P&L percentage
    pub fn pnl_percentage(&self) -> f64 {
        if self.total_investment > 0.0 {
            (self.total_pnl / self.total_investment) * 100.0
        } else {
            0.0
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
}
