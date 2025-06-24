/*!
 * Orders module for KiteConnect API v1.0.0
 * 
 * This module contains all order-related data structures:
 * - Order placement, modification, and cancellation
 * - Order status and history
 * - Order types and parameters
 */

pub mod order_data;
pub mod order_params;
pub mod order_history;

// Re-export all public types
pub use order_data::*;
pub use order_params::*;
pub use order_history::*;
