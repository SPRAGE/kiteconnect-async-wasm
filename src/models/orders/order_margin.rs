use serde::{Deserialize, Serialize};

use crate::models::common::{Exchange, OrderType, Product, TransactionType, Variety};

/// Request item for order/basket margin calculation APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMarginRequest {
    /// Name of the exchange
    pub exchange: Exchange,

    /// Exchange tradingsymbol of the instrument
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// BUY or SELL
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,

    /// Order variety (regular, amo, co, iceberg, auction)
    pub variety: Variety,

    /// Margin product to use for the order
    pub product: Product,

    /// Order type (MARKET, LIMIT, SL, SL-M)
    #[serde(rename = "order_type")]
    pub order_type: OrderType,

    /// Quantity of the order
    pub quantity: u32,

    /// Price at which the order is going to be placed (for LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Trigger price (for SL, SL-M, CO orders)
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
}

// === Typed response models ===

/// GST split for charges block
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GstBreakup {
    pub igst: f64,
    pub cgst: f64,
    pub sgst: f64,
    pub total: f64,
}

/// Charges block for margin and virtual contract note responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargesBreakdown {
    /// Sum of all charges
    pub total: f64,
    /// Tax levied for each transaction on the exchanges
    pub transaction_tax: f64,
    /// Type of transaction tax (stt/ctt)
    pub transaction_tax_type: String,
    /// Exchange turnover charge
    pub exchange_turnover_charge: f64,
    /// SEBI turnover charge
    pub sebi_turnover_charge: f64,
    /// Brokerage charge
    pub brokerage: f64,
    /// Stamp duty
    pub stamp_duty: f64,
    /// GST split
    pub gst: GstBreakup,
}

/// Realised/unrealised PnL block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnlBreakdown {
    pub realised: f64,
    pub unrealised: f64,
}

/// Per-order margin result item from /margins/orders and also used inside basket responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMarginResult {
    /// Segment type (equity/commodity)
    #[serde(rename = "type")]
    pub type_field: String,
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Exchange
    pub exchange: Exchange,

    // Margin components
    pub span: f64,
    pub exposure: f64,
    pub option_premium: f64,
    pub additional: f64,
    pub bo: f64,
    pub cash: f64,
    pub var: f64,

    /// PnL split
    pub pnl: PnlBreakdown,
    /// Allowed leverage
    pub leverage: f64,
    /// Charges split
    pub charges: ChargesBreakdown,
    /// Total margin block
    pub total: f64,
}

/// Basket margins response data block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketMarginsData {
    pub initial: OrderMarginResult,
    pub final_: OrderMarginResult,
    pub orders: Vec<OrderMarginResult>,
    pub charges: ChargesBreakdown,
}

/// Full basket margins API typed response wrapper (data field only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketMarginsResponse {
    #[serde(flatten)]
    pub data: BasketMarginsData,
}

// === Virtual contract note (charges/orders) ===

/// Request item for virtual contract note (/charges/orders)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderChargesRequest {
    /// Arbitrary order id used for grouping (can be any string)
    pub order_id: String,
    pub exchange: Exchange,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    pub variety: Variety,
    pub product: Product,
    #[serde(rename = "order_type")]
    pub order_type: OrderType,
    pub quantity: u32,
    /// Average execution price (non-zero)
    #[serde(rename = "average_price")]
    pub average_price: f64,
}

/// Response item for virtual contract note (/charges/orders)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderChargesResult {
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: Exchange,
    pub variety: Variety,
    pub product: Product,
    #[serde(rename = "order_type")]
    pub order_type: OrderType,
    pub quantity: u32,
    pub price: f64,
    pub charges: ChargesBreakdown,
}
