/*!
Shared enums and constants used across all KiteConnect models.

This module contains all the enums that are used by multiple modules,
organized into logical submodules for better maintainability.

## Modules

- `exchange`: Stock exchanges and trading venues
- `trading`: Trading-related enums (products, validity, transaction types, etc.)
- `instruments`: Instrument types and market segments  
- `interval`: Time intervals for historical data
- `gtt`: Good Till Triggered order status
*/

pub mod exchange;
pub mod gtt;
pub mod instruments;
pub mod interval;
pub mod trading;

// Re-export all enums for backward compatibility
pub use exchange::Exchange;
pub use gtt::GttStatus;
pub use instruments::{InstrumentType, Segment};
pub use interval::Interval;
pub use trading::{OrderType, Product, TransactionType, Validity, Variety};
