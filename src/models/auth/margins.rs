/*!
Margin data structures for account balance and trading limits.

Handles user margins, segment-wise balances, and fund information.
*/

use serde::{Deserialize, Serialize};

/// Complete margin data from the `margins` API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginData {
    /// Equity segment margins
    pub equity: Option<SegmentMargin>,
    
    /// Commodity segment margins
    pub commodity: Option<SegmentMargin>,
}

/// Margin data for a specific trading segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentMargin {
    /// Available cash margin
    pub available: MarginFunds,
    
    /// Utilised margin amounts
    pub utilised: MarginUtilisation,
    
    /// Net available margin (available - utilised)
    pub net: f64,
}

/// Available margin funds breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginFunds {
    /// Available cash in the account
    pub cash: f64,
    
    /// Opening balance
    pub opening_balance: f64,
    
    /// Live balance (real-time)
    pub live_balance: f64,
    
    /// Additional margin from holdings/collateral
    pub adhoc_margin: f64,
    
    /// Collateral margin from pledged securities
    pub collateral: f64,
    
    /// Intraday payin
    pub intraday_payin: f64,
}

impl MarginFunds {
    /// Calculate total available margin
    pub fn total(&self) -> f64 {
        self.cash + self.adhoc_margin + self.collateral + self.intraday_payin
    }
    
    /// Check if sufficient funds are available
    pub fn has_sufficient_funds(&self, required: f64) -> bool {
        self.total() >= required
    }
    
    /// Get cash-only balance (excluding collateral/margins)
    pub fn cash_only(&self) -> f64 {
        self.cash
    }
}

/// Margin utilisation breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginUtilisation {
    /// Debits from trades and charges
    pub debits: f64,
    
    /// Exposure margin utilised
    pub exposure: f64,
    
    /// M2M (Mark to Market) unrealised P&L
    pub m2m_unrealised: f64,
    
    /// M2M realised P&L
    pub m2m_realised: f64,
    
    /// Option premium
    pub option_premium: f64,
    
    /// Payout amount (funds on hold)
    pub payout: f64,
    
    /// SPAN margin utilised
    pub span: f64,
    
    /// Holding sales proceeds
    pub holding_sales: f64,
    
    /// Turnover charges
    pub turnover: f64,
    
    /// Liquid collateral utilised
    pub liquid: f64,
    
    /// Stock collateral utilised  
    pub stock_collateral: f64,
}

impl MarginUtilisation {
    /// Calculate total utilised margin
    pub fn total(&self) -> f64 {
        self.debits + self.exposure + self.option_premium + 
        self.payout + self.span + self.turnover + 
        self.liquid + self.stock_collateral
    }
    
    /// Get total P&L (realised + unrealised)
    pub fn total_pnl(&self) -> f64 {
        self.m2m_realised + self.m2m_unrealised
    }
    
    /// Check if account has unrealised losses
    pub fn has_unrealised_losses(&self) -> bool {
        self.m2m_unrealised < 0.0
    }
}

impl SegmentMargin {
    /// Calculate actual net margin (available total - utilised total)
    pub fn calculate_net(&self) -> f64 {
        self.available.total() - self.utilised.total()
    }
    
    /// Check if margin is sufficient for a trade
    pub fn can_place_order(&self, required_margin: f64) -> bool {
        self.net >= required_margin
    }
    
    /// Get margin utilisation percentage
    pub fn utilisation_percentage(&self) -> f64 {
        let total_available = self.available.total();
        if total_available > 0.0 {
            (self.utilised.total() / total_available) * 100.0
        } else {
            0.0
        }
    }
}

impl MarginData {
    /// Get margin for a specific segment
    pub fn get_segment(&self, segment: TradingSegment) -> Option<&SegmentMargin> {
        match segment {
            TradingSegment::Equity => self.equity.as_ref(),
            TradingSegment::Commodity => self.commodity.as_ref(),
        }
    }
    
    /// Get total available cash across all segments
    pub fn total_cash(&self) -> f64 {
        let mut total = 0.0;
        
        if let Some(equity) = &self.equity {
            total += equity.available.cash;
        }
        
        if let Some(commodity) = &self.commodity {
            total += commodity.available.cash;
        }
        
        total
    }
    
    /// Get combined net margin across segments
    pub fn total_net_margin(&self) -> f64 {
        let mut total = 0.0;
        
        if let Some(equity) = &self.equity {
            total += equity.net;
        }
        
        if let Some(commodity) = &self.commodity {
            total += commodity.net;
        }
        
        total
    }
    
    /// Check if any segment has sufficient margin
    pub fn has_sufficient_margin(&self, required: f64, segment: Option<TradingSegment>) -> bool {
        match segment {
            Some(seg) => {
                self.get_segment(seg)
                    .map(|margin| margin.can_place_order(required))
                    .unwrap_or(false)
            }
            None => {
                // Check any segment
                self.equity.as_ref().map(|m| m.can_place_order(required)).unwrap_or(false) ||
                self.commodity.as_ref().map(|m| m.can_place_order(required)).unwrap_or(false)
            }
        }
    }
}

/// Trading segments for margin segregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradingSegment {
    Equity,
    Commodity,
}

impl std::fmt::Display for TradingSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingSegment::Equity => write!(f, "equity"),
            TradingSegment::Commodity => write!(f, "commodity"),
        }
    }
}

/// Fund transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundTransaction {
    /// Transaction ID
    pub id: String,
    
    /// Transaction type (credit/debit)
    pub transaction_type: String,
    
    /// Amount
    pub amount: f64,
    
    /// Description/narration
    pub description: String,
    
    /// Transaction date
    pub date: String,
    
    /// Segment where transaction occurred
    #[serde(default)]
    pub segment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_margin_funds() {
        let funds = MarginFunds {
            cash: 10000.0,
            opening_balance: 10000.0,
            live_balance: 9500.0,
            adhoc_margin: 2000.0,
            collateral: 5000.0,
            intraday_payin: 0.0,
        };
        
        assert_eq!(funds.total(), 17000.0);
        assert!(funds.has_sufficient_funds(15000.0));
        assert!(!funds.has_sufficient_funds(20000.0));
        assert_eq!(funds.cash_only(), 10000.0);
    }
    
    #[test]
    fn test_margin_utilisation() {
        let utilisation = MarginUtilisation {
            debits: 1000.0,
            exposure: 2000.0,
            m2m_unrealised: -500.0,
            m2m_realised: 200.0,
            option_premium: 0.0,
            payout: 0.0,
            span: 1500.0,
            holding_sales: 0.0,
            turnover: 50.0,
            liquid: 0.0,
            stock_collateral: 0.0,
        };
        
        assert_eq!(utilisation.total(), 4550.0);
        assert_eq!(utilisation.total_pnl(), -300.0);
        assert!(utilisation.has_unrealised_losses());
    }
    
    #[test]
    fn test_segment_margin() {
        let available = MarginFunds {
            cash: 10000.0,
            opening_balance: 10000.0,
            live_balance: 9500.0,
            adhoc_margin: 0.0,
            collateral: 0.0,
            intraday_payin: 0.0,
        };
        
        let utilised = MarginUtilisation {
            debits: 2000.0,
            exposure: 1000.0,
            m2m_unrealised: 0.0,
            m2m_realised: 0.0,
            option_premium: 0.0,
            payout: 0.0,
            span: 0.0,
            holding_sales: 0.0,
            turnover: 0.0,
            liquid: 0.0,
            stock_collateral: 0.0,
        };
        
        let margin = SegmentMargin {
            available,
            utilised,
            net: 7000.0,
        };
        
        assert_eq!(margin.calculate_net(), 7000.0);
        assert!(margin.can_place_order(5000.0));
        assert!(!margin.can_place_order(8000.0));
        assert_eq!(margin.utilisation_percentage(), 30.0);
    }
    
    #[test]
    fn test_margin_data() {
        let equity_margin = SegmentMargin {
            available: MarginFunds {
                cash: 10000.0,
                opening_balance: 10000.0,
                live_balance: 9500.0,
                adhoc_margin: 0.0,
                collateral: 0.0,
                intraday_payin: 0.0,
            },
            utilised: MarginUtilisation {
                debits: 2000.0,
                exposure: 0.0,
                m2m_unrealised: 0.0,
                m2m_realised: 0.0,
                option_premium: 0.0,
                payout: 0.0,
                span: 0.0,
                holding_sales: 0.0,
                turnover: 0.0,
                liquid: 0.0,
                stock_collateral: 0.0,
            },
            net: 8000.0,
        };
        
        let margin_data = MarginData {
            equity: Some(equity_margin),
            commodity: None,
        };
        
        assert_eq!(margin_data.total_cash(), 10000.0);
        assert_eq!(margin_data.total_net_margin(), 8000.0);
        assert!(margin_data.has_sufficient_margin(5000.0, Some(TradingSegment::Equity)));
        assert!(!margin_data.has_sufficient_margin(5000.0, Some(TradingSegment::Commodity)));
    }
}
