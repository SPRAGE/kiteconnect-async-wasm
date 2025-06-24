use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Mutual Fund instrument data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFInstrument {
    /// Trading symbol (unique identifier)
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// AMC (Asset Management Company) code
    pub amc: String,
    
    /// Fund name
    pub name: String,
    
    /// Fund type (growth, dividend, etc.)
    #[serde(rename = "fund_type")]
    pub fund_type: String,
    
    /// Fund category (equity, debt, hybrid, etc.)
    pub plan: String,
    
    /// Settlement type (T+1, T+3, etc.)
    #[serde(rename = "settlement_type")]
    pub settlement_type: String,
    
    /// Minimum purchase amount
    #[serde(rename = "minimum_purchase_amount")]
    pub minimum_purchase_amount: f64,
    
    /// Purchase amount multiple
    #[serde(rename = "purchase_amount_multiplier")]
    pub purchase_amount_multiplier: f64,
    
    /// Minimum additional purchase amount
    #[serde(rename = "minimum_additional_purchase_amount")]
    pub minimum_additional_purchase_amount: f64,
    
    /// Minimum redemption quantity
    #[serde(rename = "minimum_redemption_quantity")]
    pub minimum_redemption_quantity: f64,
    
    /// Redemption quantity multiple
    #[serde(rename = "redemption_quantity_multiplier")]
    pub redemption_quantity_multiplier: f64,
    
    /// Dividend reinvestment flag
    #[serde(rename = "dividend_type")]
    pub dividend_type: String,
    
    /// Scheme type (open-ended, close-ended)
    #[serde(rename = "scheme_type")]
    pub scheme_type: String,
    
    /// Last price (NAV)
    #[serde(rename = "last_price")]
    pub last_price: f64,
    
    /// Last price date
    #[serde(rename = "last_price_date")]
    pub last_price_date: NaiveDate,
}

/// MF fund performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFPerformance {
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Fund name
    pub name: String,
    
    /// Current NAV
    pub nav: f64,
    
    /// NAV date
    #[serde(rename = "nav_date")]
    pub nav_date: NaiveDate,
    
    /// 1 day return
    #[serde(rename = "return_1d")]
    pub return_1d: Option<f64>,
    
    /// 1 week return
    #[serde(rename = "return_1w")]
    pub return_1w: Option<f64>,
    
    /// 1 month return
    #[serde(rename = "return_1m")]
    pub return_1m: Option<f64>,
    
    /// 3 months return
    #[serde(rename = "return_3m")]
    pub return_3m: Option<f64>,
    
    /// 6 months return
    #[serde(rename = "return_6m")]
    pub return_6m: Option<f64>,
    
    /// 1 year return
    #[serde(rename = "return_1y")]
    pub return_1y: Option<f64>,
    
    /// 3 years return
    #[serde(rename = "return_3y")]
    pub return_3y: Option<f64>,
    
    /// 5 years return
    #[serde(rename = "return_5y")]
    pub return_5y: Option<f64>,
    
    /// Since inception return
    #[serde(rename = "return_inception")]
    pub return_inception: Option<f64>,
}

/// MF instrument search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFInstrumentSearch {
    /// Search query (fund name or AMC)
    pub query: String,
    
    /// AMC filter
    pub amc: Option<String>,
    
    /// Fund type filter
    pub fund_type: Option<String>,
    
    /// Plan filter (growth, dividend)
    pub plan: Option<String>,
    
    /// Limit results
    pub limit: Option<u32>,
}

impl MFInstrument {
    /// Check if this is an equity fund
    pub fn is_equity_fund(&self) -> bool {
        self.plan.to_lowercase().contains("equity")
    }
    
    /// Check if this is a debt fund
    pub fn is_debt_fund(&self) -> bool {
        self.plan.to_lowercase().contains("debt")
    }
    
    /// Check if this is a hybrid fund
    pub fn is_hybrid_fund(&self) -> bool {
        self.plan.to_lowercase().contains("hybrid")
    }
    
    /// Check if this is a growth plan
    pub fn is_growth_plan(&self) -> bool {
        self.dividend_type.to_lowercase().contains("growth")
    }
    
    /// Check if this is a dividend plan
    pub fn is_dividend_plan(&self) -> bool {
        self.dividend_type.to_lowercase().contains("dividend")
    }
    
    /// Check if fund allows SIP
    pub fn allows_sip(&self) -> bool {
        self.minimum_additional_purchase_amount > 0.0
    }
    
    /// Get the settlement days
    pub fn settlement_days(&self) -> u32 {
        // Parse settlement type like "T+1", "T+3"
        self.settlement_type
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(1)
    }
    
    /// Check if amount is valid for purchase
    pub fn is_valid_purchase_amount(&self, amount: f64) -> bool {
        if amount < self.minimum_purchase_amount {
            return false;
        }
        
        let remainder = (amount - self.minimum_purchase_amount) % self.purchase_amount_multiplier;
        remainder.abs() < 0.01 // Allow for floating point precision
    }
    
    /// Get next valid purchase amount
    pub fn next_valid_purchase_amount(&self, amount: f64) -> f64 {
        if amount <= self.minimum_purchase_amount {
            return self.minimum_purchase_amount;
        }
        
        let excess = amount - self.minimum_purchase_amount;
        let multiplier_count = (excess / self.purchase_amount_multiplier).ceil();
        self.minimum_purchase_amount + (multiplier_count * self.purchase_amount_multiplier)
    }
}

impl MFPerformance {
    /// Get the best performing period return
    pub fn best_return(&self) -> Option<f64> {
        [
            self.return_1d,
            self.return_1w,
            self.return_1m,
            self.return_3m,
            self.return_6m,
            self.return_1y,
            self.return_3y,
            self.return_5y,
            self.return_inception,
        ]
        .iter()
        .filter_map(|&r| r)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Get the worst performing period return
    pub fn worst_return(&self) -> Option<f64> {
        [
            self.return_1d,
            self.return_1w,
            self.return_1m,
            self.return_3m,
            self.return_6m,
            self.return_1y,
            self.return_3y,
            self.return_5y,
            self.return_inception,
        ]
        .iter()
        .filter_map(|&r| r)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Check if fund is consistently performing (positive returns across periods)
    pub fn is_consistently_positive(&self) -> bool {
        [
            self.return_1m,
            self.return_3m,
            self.return_6m,
            self.return_1y,
        ]
        .iter()
        .filter_map(|&r| r)
        .all(|r| r > 0.0)
    }
    
    /// Get volatility indicator based on return spread
    pub fn volatility_indicator(&self) -> Option<f64> {
        match (self.best_return(), self.worst_return()) {
            (Some(best), Some(worst)) => Some(best - worst),
            _ => None,
        }
    }
}

impl MFInstrumentSearch {
    /// Create a new search
    pub fn new(query: String) -> Self {
        Self {
            query,
            amc: None,
            fund_type: None,
            plan: None,
            limit: None,
        }
    }
    
    /// Filter by AMC
    pub fn amc<S: Into<String>>(mut self, amc: S) -> Self {
        self.amc = Some(amc.into());
        self
    }
    
    /// Filter by fund type
    pub fn fund_type<S: Into<String>>(mut self, fund_type: S) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }
    
    /// Filter by plan
    pub fn plan<S: Into<String>>(mut self, plan: S) -> Self {
        self.plan = Some(plan.into());
        self
    }
    
    /// Limit results
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Search for equity funds only
    pub fn equity_only(mut self) -> Self {
        self.plan = Some("equity".to_string());
        self
    }
    
    /// Search for debt funds only
    pub fn debt_only(mut self) -> Self {
        self.plan = Some("debt".to_string());
        self
    }
}
