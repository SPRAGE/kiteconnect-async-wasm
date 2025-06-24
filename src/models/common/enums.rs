/*!
Shared enums and constants used across all KiteConnect models.

This module contains all the enums that are used by multiple modules,
such as Exchange, Product, Validity, OrderType, etc.
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

/// Instrument types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "EQ")]
    EQ, // Equity
    #[serde(rename = "FUT")]
    FUT, // Futures
    #[serde(rename = "CE")]
    CE, // Call Option
    #[serde(rename = "PE")]
    PE, // Put Option
    #[serde(rename = "COMMODITY")]
    COMMODITY,
}

impl std::fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrumentType::EQ => write!(f, "EQ"),
            InstrumentType::FUT => write!(f, "FUT"),
            InstrumentType::CE => write!(f, "CE"),
            InstrumentType::PE => write!(f, "PE"),
            InstrumentType::COMMODITY => write!(f, "COMMODITY"),
        }
    }
}

/// GTT order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GttStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "triggered")]
    Triggered,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "rejected")]
    Rejected,
}

impl std::fmt::Display for GttStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GttStatus::Active => write!(f, "active"),
            GttStatus::Triggered => write!(f, "triggered"),
            GttStatus::Disabled => write!(f, "disabled"),
            GttStatus::Expired => write!(f, "expired"),
            GttStatus::Cancelled => write!(f, "cancelled"),
            GttStatus::Rejected => write!(f, "rejected"),
        }
    }
}

/// Interval types for historical data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "3minute")]
    ThreeMinute,
    #[serde(rename = "5minute")]
    FiveMinute,
    #[serde(rename = "10minute")]
    TenMinute,
    #[serde(rename = "15minute")]
    FifteenMinute,
    #[serde(rename = "30minute")]
    ThirtyMinute,
    #[serde(rename = "60minute")]
    SixtyMinute,
    #[serde(rename = "day")]
    Day,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Minute => write!(f, "minute"),
            Interval::ThreeMinute => write!(f, "3minute"),
            Interval::FiveMinute => write!(f, "5minute"),
            Interval::TenMinute => write!(f, "10minute"),
            Interval::FifteenMinute => write!(f, "15minute"),
            Interval::ThirtyMinute => write!(f, "30minute"),
            Interval::SixtyMinute => write!(f, "60minute"),
            Interval::Day => write!(f, "day"),
        }
    }
}

/// Instrument segments (exchange + type combinations)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Segment {
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "INDICES")]
    INDICES,
    #[serde(rename = "NCO")]
    NCO,
    #[serde(rename = "BFO-FUT")]
    BfoFut,
    #[serde(rename = "BFO-OPT")]
    BfoOpt,
    #[serde(rename = "CDS-FUT")]
    CdsFut,
    #[serde(rename = "CDS-OPT")]
    CdsOpt,
    #[serde(rename = "MCX-FUT")]
    McxFut,
    #[serde(rename = "MCX-OPT")]
    McxOpt,
    #[serde(rename = "NCO-FUT")]
    NcoFut,
    #[serde(rename = "NCO-OPT")]
    NcoOpt,
    #[serde(rename = "NFO-FUT")]
    NfoFut,
    #[serde(rename = "NFO-OPT")]
    NfoOpt,
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Segment::BSE => write!(f, "BSE"),
            Segment::NSE => write!(f, "NSE"),
            Segment::INDICES => write!(f, "INDICES"),
            Segment::NCO => write!(f, "NCO"),
            Segment::BfoFut => write!(f, "BFO-FUT"),
            Segment::BfoOpt => write!(f, "BFO-OPT"),
            Segment::CdsFut => write!(f, "CDS-FUT"),
            Segment::CdsOpt => write!(f, "CDS-OPT"),
            Segment::McxFut => write!(f, "MCX-FUT"),
            Segment::McxOpt => write!(f, "MCX-OPT"),
            Segment::NcoFut => write!(f, "NCO-FUT"),
            Segment::NcoOpt => write!(f, "NCO-OPT"),
            Segment::NfoFut => write!(f, "NFO-FUT"),
            Segment::NfoOpt => write!(f, "NFO-OPT"),
        }
    }
}
