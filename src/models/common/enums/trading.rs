/*!
Trading products and order-related enums.
*/

use serde::{Deserialize, Serialize};

/// Product types for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Product {
    #[serde(rename = "CNC")]
    CNC, // Cash & Carry for equity
    #[serde(rename = "NRML")]
    NRML, // Normal for futures and options
    #[serde(rename = "MIS")]
    MIS, // Margin Intraday Squareoff for futures and options
    #[serde(rename = "MTF")]
    MTF, // Margin Trading Facility
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Product::CNC => write!(f, "CNC"),
            Product::MIS => write!(f, "MIS"),
            Product::NRML => write!(f, "NRML"),
            Product::MTF => write!(f, "MTF"),
        }
    }
}

/// Order validity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Validity {
    #[serde(rename = "DAY")]
    DAY, // Day order
    #[serde(rename = "IOC")]
    IOC, // Immediate or Cancel
    #[serde(rename = "TTL")]
    TTL, // Time to Live (for GTT orders)
}

impl std::fmt::Display for Validity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Validity::DAY => write!(f, "DAY"),
            Validity::IOC => write!(f, "IOC"),
            Validity::TTL => write!(f, "TTL"),
        }
    }
}

/// Transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "BUY")]
    BUY,
    #[serde(rename = "SELL")]
    SELL,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::BUY => write!(f, "BUY"),
            TransactionType::SELL => write!(f, "SELL"),
        }
    }
}

/// Order types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    MARKET,
    #[serde(rename = "LIMIT")]
    LIMIT,
    #[serde(rename = "SL")]
    SL, // Stop Loss
    #[serde(rename = "SL-M")]
    SLM, // Stop Loss Market
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::MARKET => write!(f, "MARKET"),
            OrderType::LIMIT => write!(f, "LIMIT"),
            OrderType::SL => write!(f, "SL"),
            OrderType::SLM => write!(f, "SL-M"),
        }
    }
}

/// Order varieties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Variety {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "co")]
    CO, // Cover Order
    #[serde(rename = "amo")]
    AMO, // After Market Order
    #[serde(rename = "iceberg")]
    Iceberg,
    #[serde(rename = "auction")]
    Auction,
}

impl std::fmt::Display for Variety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variety::Regular => write!(f, "regular"),
            Variety::CO => write!(f, "co"),
            Variety::AMO => write!(f, "amo"),
            Variety::Iceberg => write!(f, "iceberg"),
            Variety::Auction => write!(f, "auction"),
        }
    }
}
