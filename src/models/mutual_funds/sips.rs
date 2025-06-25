use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// SIP (Systematic Investment Plan) data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIP {
    /// SIP ID
    #[serde(rename = "sip_id")]
    pub sip_id: String,

    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Fund name
    pub fund: String,

    /// SIP status
    pub status: SIPStatus,

    /// Created timestamp
    pub created: DateTime<Utc>,

    /// Frequency (monthly, weekly, daily)
    pub frequency: SIPFrequency,

    /// Installment amount
    #[serde(rename = "installment_amount")]
    pub installment_amount: f64,

    /// Installments completed
    #[serde(rename = "completed_instalments")]
    pub completed_instalments: u32,

    /// Pending installments
    #[serde(rename = "pending_instalments")]
    pub pending_instalments: Option<u32>,

    /// Next installment date
    #[serde(rename = "next_instalment")]
    pub next_instalment: Option<NaiveDate>,

    /// Last installment date
    #[serde(rename = "last_instalment")]
    pub last_instalment: Option<NaiveDate>,

    /// SIP trigger price (if any)
    #[serde(rename = "trigger_price")]
    pub trigger_price: Option<f64>,

    /// Tag
    pub tag: Option<String>,

    /// Dividend type
    #[serde(rename = "dividend_type")]
    pub dividend_type: String,

    /// Step up configuration (if any)
    #[serde(rename = "step_up")]
    pub step_up: Option<SIPStepUp>,
}

/// SIP status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SIPStatus {
    /// SIP is active
    Active,
    /// SIP is paused
    Paused,
    /// SIP is cancelled
    Cancelled,
    /// SIP is completed
    Complete,
}

/// SIP frequency enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SIPFrequency {
    /// Monthly SIP
    Monthly,
    /// Weekly SIP
    Weekly,
    /// Daily SIP
    Daily,
    /// Quarterly SIP
    Quarterly,
}

/// SIP step-up configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPStepUp {
    /// Step-up amount
    pub amount: f64,

    /// Step-up frequency (yearly, half-yearly)
    pub frequency: String,

    /// Next step-up date
    #[serde(rename = "next_step_up")]
    pub next_step_up: Option<NaiveDate>,
}

/// SIP creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPParams {
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Installment amount
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Number of installments (optional for perpetual SIP)
    #[serde(rename = "instalments", skip_serializing_if = "Option::is_none")]
    pub instalments: Option<u32>,

    /// Frequency
    pub frequency: SIPFrequency,

    /// Initial amount (for first installment, optional)
    #[serde(rename = "initial_amount", skip_serializing_if = "Option::is_none")]
    pub initial_amount: Option<f64>,

    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// SIP modification parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPModifyParams {
    /// SIP ID
    #[serde(skip_serializing)]
    pub sip_id: String,

    /// New installment amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// New frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SIPFrequency>,

    /// New number of installments
    #[serde(rename = "instalments", skip_serializing_if = "Option::is_none")]
    pub instalments: Option<u32>,

    /// SIP status (to pause/resume)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SIPStatus>,
}

/// SIP response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPResponse {
    /// SIP ID
    #[serde(rename = "sip_id")]
    pub sip_id: String,
}

/// SIPs collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPs {
    /// List of SIPs
    pub sips: Vec<SIP>,
}

impl SIP {
    /// Check if SIP is active
    pub fn is_active(&self) -> bool {
        self.status == SIPStatus::Active
    }

    /// Check if SIP is paused
    pub fn is_paused(&self) -> bool {
        self.status == SIPStatus::Paused
    }

    /// Check if SIP is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == SIPStatus::Cancelled
    }

    /// Check if SIP is completed
    pub fn is_complete(&self) -> bool {
        self.status == SIPStatus::Complete
    }

    /// Calculate total invested amount
    pub fn total_invested(&self) -> f64 {
        self.installment_amount * self.completed_instalments as f64
    }

    /// Check if this is a perpetual SIP (no end date)
    pub fn is_perpetual(&self) -> bool {
        self.pending_instalments.is_none()
    }

    /// Get total installments (completed + pending)
    pub fn total_instalments(&self) -> Option<u32> {
        self.pending_instalments
            .map(|pending| self.completed_instalments + pending)
    }

    /// Calculate progress percentage
    pub fn progress_percentage(&self) -> Option<f64> {
        self.total_instalments().map(|total| {
            if total > 0 {
                (self.completed_instalments as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        })
    }

    /// Get remaining amount to be invested
    pub fn remaining_amount(&self) -> Option<f64> {
        self.pending_instalments
            .map(|pending| self.installment_amount * pending as f64)
    }

    /// Check if step-up is configured
    pub fn has_step_up(&self) -> bool {
        self.step_up.is_some()
    }

    /// Get monthly equivalent amount (for comparison across frequencies)
    pub fn monthly_equivalent_amount(&self) -> f64 {
        match self.frequency {
            SIPFrequency::Monthly => self.installment_amount,
            SIPFrequency::Weekly => self.installment_amount * 4.33, // Average weeks per month
            SIPFrequency::Daily => self.installment_amount * 30.0,  // Average days per month
            SIPFrequency::Quarterly => self.installment_amount / 3.0,
        }
    }

    /// Get annual investment amount
    pub fn annual_amount(&self) -> f64 {
        match self.frequency {
            SIPFrequency::Monthly => self.installment_amount * 12.0,
            SIPFrequency::Weekly => self.installment_amount * 52.0,
            SIPFrequency::Daily => self.installment_amount * 365.0,
            SIPFrequency::Quarterly => self.installment_amount * 4.0,
        }
    }

    /// Check if next installment is due soon (within days)
    pub fn is_due_soon(&self, days: i64) -> bool {
        if let Some(next_date) = self.next_instalment {
            let today = chrono::Utc::now().date_naive();
            let days_until_due = (next_date - today).num_days();
            days_until_due <= days && days_until_due >= 0
        } else {
            false
        }
    }
}

impl SIPParams {
    /// Create a new SIP
    pub fn new(trading_symbol: String, amount: f64, frequency: SIPFrequency) -> Self {
        Self {
            trading_symbol,
            amount,
            instalments: None,
            frequency,
            initial_amount: None,
            tag: None,
        }
    }

    /// Set number of installments
    pub fn instalments(mut self, instalments: u32) -> Self {
        self.instalments = Some(instalments);
        self
    }

    /// Set initial amount
    pub fn initial_amount(mut self, initial_amount: f64) -> Self {
        self.initial_amount = Some(initial_amount);
        self
    }

    /// Set tag
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Create monthly SIP
    pub fn monthly(trading_symbol: String, amount: f64) -> Self {
        Self::new(trading_symbol, amount, SIPFrequency::Monthly)
    }

    /// Create weekly SIP
    pub fn weekly(trading_symbol: String, amount: f64) -> Self {
        Self::new(trading_symbol, amount, SIPFrequency::Weekly)
    }

    /// Create daily SIP
    pub fn daily(trading_symbol: String, amount: f64) -> Self {
        Self::new(trading_symbol, amount, SIPFrequency::Daily)
    }

    /// Validate SIP parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.trading_symbol.is_empty() {
            return Err("Trading symbol is required".to_string());
        }

        if self.amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if let Some(instalments) = self.instalments {
            if instalments == 0 {
                return Err("Number of installments must be positive".to_string());
            }
        }

        Ok(())
    }
}

impl SIPs {
    /// Get active SIPs
    pub fn active_sips(&self) -> Vec<&SIP> {
        self.sips.iter().filter(|sip| sip.is_active()).collect()
    }

    /// Get paused SIPs
    pub fn paused_sips(&self) -> Vec<&SIP> {
        self.sips.iter().filter(|sip| sip.is_paused()).collect()
    }

    /// Calculate total monthly commitment
    pub fn total_monthly_commitment(&self) -> f64 {
        self.active_sips()
            .iter()
            .map(|sip| sip.monthly_equivalent_amount())
            .sum()
    }

    /// Calculate total annual commitment
    pub fn total_annual_commitment(&self) -> f64 {
        self.active_sips()
            .iter()
            .map(|sip| sip.annual_amount())
            .sum()
    }

    /// Get SIPs due soon
    pub fn sips_due_soon(&self, days: i64) -> Vec<&SIP> {
        self.active_sips()
            .into_iter()
            .filter(|sip| sip.is_due_soon(days))
            .collect()
    }

    /// Find SIP by ID
    pub fn find_sip(&self, sip_id: &str) -> Option<&SIP> {
        self.sips.iter().find(|sip| sip.sip_id == sip_id)
    }

    /// Get SIPs for a specific fund
    pub fn sips_for_fund(&self, trading_symbol: &str) -> Vec<&SIP> {
        self.sips
            .iter()
            .filter(|sip| sip.trading_symbol == trading_symbol)
            .collect()
    }
}

impl fmt::Display for SIPFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SIPFrequency::Monthly => "monthly",
            SIPFrequency::Weekly => "weekly",
            SIPFrequency::Daily => "daily",
            SIPFrequency::Quarterly => "quarterly",
        };
        write!(f, "{}", s)
    }
}
