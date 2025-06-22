//! # Charges Models
//! 
//! This module contains data models for order charges and fees calculation in the KiteConnect API.
//! Based on the official Go library: https://github.com/zerodha/gokiteconnect/blob/main/margins.go

// Re-export charges types from margin module to avoid duplication
pub use crate::model::margin::{OrderChargesParam, OrderCharges, Charges, GST};
