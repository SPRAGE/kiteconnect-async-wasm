/*!
Instrument types and market segments.
*/

use serde::{Deserialize, Serialize};

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
