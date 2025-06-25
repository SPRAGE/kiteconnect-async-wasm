/*!
GTT (Good Till Triggered) order status enum.
*/

use serde::{Deserialize, Serialize};

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
