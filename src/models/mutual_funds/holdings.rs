use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// MF holding data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFHolding {
    /// Folio number
    pub folio: String,

    /// Fund name
    pub fund: String,

    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Average price (average NAV at which units were purchased)
    #[serde(rename = "average_price")]
    pub average_price: f64,

    /// Last price (current NAV)
    #[serde(rename = "last_price")]
    pub last_price: f64,

    /// Last price date
    #[serde(rename = "last_price_date")]
    pub last_price_date: NaiveDate,

    /// Quantity (units held)
    pub quantity: f64,

    /// P&L (profit and loss)
    pub pnl: f64,

    /// Pledged quantity
    #[serde(rename = "pledged_quantity")]
    pub pledged_quantity: f64,
}

/// MF holdings collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFHoldings {
    /// List of MF holdings
    pub holdings: Vec<MFHolding>,
}

/// MF portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFPortfolioSummary {
    /// Total investment value
    pub total_investment: f64,

    /// Total current value
    pub total_current_value: f64,

    /// Total P&L
    pub total_pnl: f64,

    /// Total P&L percentage
    pub total_pnl_percentage: f64,

    /// Number of holdings
    pub holdings_count: usize,

    /// Number of profitable holdings
    pub profitable_holdings: usize,

    /// Number of loss-making holdings
    pub loss_holdings: usize,
}

impl MFHolding {
    /// Calculate current value of the holding
    pub fn current_value(&self) -> f64 {
        self.last_price * self.quantity
    }

    /// Calculate investment value (cost)
    pub fn investment_value(&self) -> f64 {
        self.average_price * self.quantity
    }

    /// Calculate P&L percentage
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

    /// Get available quantity (non-pledged)
    pub fn available_quantity(&self) -> f64 {
        self.quantity - self.pledged_quantity
    }

    /// Check if any units are pledged
    pub fn is_pledged(&self) -> bool {
        self.pledged_quantity > 0.0
    }

    /// Get the day's change in NAV
    pub fn nav_change(&self) -> f64 {
        self.last_price - self.average_price
    }

    /// Get the day's change in NAV percentage
    pub fn nav_change_percentage(&self) -> f64 {
        if self.average_price > 0.0 {
            ((self.last_price - self.average_price) / self.average_price) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate absolute P&L
    pub fn abs_pnl(&self) -> f64 {
        self.pnl.abs()
    }

    /// Get holding weight in portfolio (requires portfolio value)
    pub fn weight_in_portfolio(&self, total_portfolio_value: f64) -> f64 {
        if total_portfolio_value > 0.0 {
            (self.current_value() / total_portfolio_value) * 100.0
        } else {
            0.0
        }
    }

    /// Check if this is a small holding (less than specified value)
    pub fn is_small_holding(&self, threshold: f64) -> bool {
        self.current_value() < threshold
    }

    /// Calculate redemption value for specified units
    pub fn redemption_value(&self, units: f64) -> f64 {
        if units <= self.available_quantity() {
            self.last_price * units
        } else {
            0.0 // Cannot redeem more than available
        }
    }
}

impl MFHoldings {
    /// Calculate portfolio summary
    pub fn portfolio_summary(&self) -> MFPortfolioSummary {
        let total_investment = self.holdings.iter().map(|h| h.investment_value()).sum();
        let total_current_value = self.holdings.iter().map(|h| h.current_value()).sum();
        let total_pnl = self.holdings.iter().map(|h| h.pnl).sum();

        let total_pnl_percentage = if total_investment > 0.0 {
            (total_pnl / total_investment) * 100.0
        } else {
            0.0
        };

        let profitable_holdings = self.holdings.iter().filter(|h| h.is_profitable()).count();
        let loss_holdings = self.holdings.iter().filter(|h| h.is_loss()).count();

        MFPortfolioSummary {
            total_investment,
            total_current_value,
            total_pnl,
            total_pnl_percentage,
            holdings_count: self.holdings.len(),
            profitable_holdings,
            loss_holdings,
        }
    }

    /// Get profitable holdings
    pub fn profitable_holdings(&self) -> Vec<&MFHolding> {
        self.holdings.iter().filter(|h| h.is_profitable()).collect()
    }

    /// Get loss-making holdings
    pub fn loss_holdings(&self) -> Vec<&MFHolding> {
        self.holdings.iter().filter(|h| h.is_loss()).collect()
    }

    /// Get holdings by fund name pattern
    pub fn holdings_by_fund_pattern(&self, pattern: &str) -> Vec<&MFHolding> {
        self.holdings
            .iter()
            .filter(|h| h.fund.to_lowercase().contains(&pattern.to_lowercase()))
            .collect()
    }

    /// Get top performers by P&L percentage
    pub fn top_performers(&self, limit: usize) -> Vec<&MFHolding> {
        let mut holdings = self.holdings.iter().collect::<Vec<_>>();
        holdings.sort_by(|a, b| {
            b.pnl_percentage()
                .partial_cmp(&a.pnl_percentage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        holdings.into_iter().take(limit).collect()
    }

    /// Get worst performers by P&L percentage
    pub fn worst_performers(&self, limit: usize) -> Vec<&MFHolding> {
        let mut holdings = self.holdings.iter().collect::<Vec<_>>();
        holdings.sort_by(|a, b| {
            a.pnl_percentage()
                .partial_cmp(&b.pnl_percentage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        holdings.into_iter().take(limit).collect()
    }

    /// Get largest holdings by current value
    pub fn largest_holdings(&self, limit: usize) -> Vec<&MFHolding> {
        let mut holdings = self.holdings.iter().collect::<Vec<_>>();
        holdings.sort_by(|a, b| {
            b.current_value()
                .partial_cmp(&a.current_value())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        holdings.into_iter().take(limit).collect()
    }

    /// Get small holdings below threshold
    pub fn small_holdings(&self, threshold: f64) -> Vec<&MFHolding> {
        self.holdings
            .iter()
            .filter(|h| h.is_small_holding(threshold))
            .collect()
    }

    /// Find holding by folio number
    pub fn find_by_folio(&self, folio: &str) -> Option<&MFHolding> {
        self.holdings.iter().find(|h| h.folio == folio)
    }

    /// Find holding by trading symbol
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&MFHolding> {
        self.holdings.iter().find(|h| h.trading_symbol == symbol)
    }

    /// Get total pledged value
    pub fn total_pledged_value(&self) -> f64 {
        self.holdings
            .iter()
            .map(|h| h.pledged_quantity * h.last_price)
            .sum()
    }

    /// Get total available value (non-pledged)
    pub fn total_available_value(&self) -> f64 {
        self.holdings
            .iter()
            .map(|h| h.available_quantity() * h.last_price)
            .sum()
    }
}

impl MFPortfolioSummary {
    /// Check if the overall portfolio is profitable
    pub fn is_profitable(&self) -> bool {
        self.total_pnl > 0.0
    }

    /// Check if the overall portfolio is in loss
    pub fn is_loss(&self) -> bool {
        self.total_pnl < 0.0
    }

    /// Get the win rate (percentage of profitable holdings)
    pub fn win_rate(&self) -> f64 {
        if self.holdings_count > 0 {
            (self.profitable_holdings as f64 / self.holdings_count as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get the loss rate (percentage of loss-making holdings)
    pub fn loss_rate(&self) -> f64 {
        if self.holdings_count > 0 {
            (self.loss_holdings as f64 / self.holdings_count as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get absolute P&L
    pub fn abs_pnl(&self) -> f64 {
        self.total_pnl.abs()
    }
}
