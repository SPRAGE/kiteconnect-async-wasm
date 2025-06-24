/*!
 * GTT (Good Till Triggered) module for KiteConnect API v1.0.0
 * 
 * This module contains all GTT-related data structures:
 * - GTT orders and triggers
 * - GTT conditions and execution rules
 * - GTT status tracking and management
 */

pub mod triggers;
pub mod orders;

// Re-export all public types
pub use triggers::*;
pub use orders::*;
