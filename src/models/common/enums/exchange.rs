/*!
Stock exchanges supported by KiteConnect.
*/

use serde::{Deserialize, Serialize};

/// Stock exchanges supported by KiteConnect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "NFO")]
    NFO,
    #[serde(rename = "CDS")]
    CDS,
    #[serde(rename = "BFO")]
    BFO,
    #[serde(rename = "MCX")]
    MCX,
    #[serde(rename = "GLOBAL")]
    GLOBAL,
    #[serde(rename = "NCO")]
    NCO,
    #[serde(rename = "NSEIX")]
    NSEIX,
}

impl Exchange {
    /// Get all supported exchanges
    pub fn all() -> Vec<Self> {
        vec![
            Exchange::NSE,
            Exchange::BSE,
            Exchange::NFO,
            Exchange::CDS,
            Exchange::BFO,
            Exchange::MCX,
            Exchange::GLOBAL,
            Exchange::NCO,
            Exchange::NSEIX,
        ]
    }

    /// Check if exchange supports equity trading
    pub fn is_equity(self) -> bool {
        matches!(self, Exchange::NSE | Exchange::BSE | Exchange::NSEIX)
    }

    /// Check if exchange supports derivatives trading
    pub fn is_derivative(self) -> bool {
        matches!(self, Exchange::NFO | Exchange::BFO)
    }

    /// Check if exchange supports commodity trading
    pub fn is_commodity(self) -> bool {
        matches!(self, Exchange::MCX | Exchange::CDS | Exchange::NCO)
    }

    /// Check if exchange is international/global
    pub fn is_global(self) -> bool {
        matches!(self, Exchange::GLOBAL)
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exchange::NSE => write!(f, "NSE"),
            Exchange::BSE => write!(f, "BSE"),
            Exchange::NFO => write!(f, "NFO"),
            Exchange::CDS => write!(f, "CDS"),
            Exchange::BFO => write!(f, "BFO"),
            Exchange::MCX => write!(f, "MCX"),
            Exchange::GLOBAL => write!(f, "GLOBAL"),
            Exchange::NCO => write!(f, "NCO"),
            Exchange::NSEIX => write!(f, "NSEIX"),
        }
    }
}
