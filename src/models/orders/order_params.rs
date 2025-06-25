use crate::models::common::{Exchange, OrderType, Product, TransactionType, Validity};
use serde::{Deserialize, Serialize};

/// Order placement parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderParams {
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Exchange
    pub exchange: Exchange,

    /// Transaction type (BUY/SELL)
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,

    /// Order type
    #[serde(rename = "order_type")]
    pub order_type: OrderType,

    /// Quantity
    pub quantity: u32,

    /// Product type
    pub product: Product,

    /// Price (required for LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Trigger price (required for SL and SL-M orders)
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,

    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,

    /// Disclosed quantity for iceberg orders
    #[serde(rename = "disclosed_quantity", skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<u32>,

    /// Tag for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Square off value for bracket orders
    #[serde(rename = "squareoff", skip_serializing_if = "Option::is_none")]
    pub squareoff: Option<f64>,

    /// Stoploss value for bracket orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stoploss: Option<f64>,

    /// Trailing stoploss value for bracket orders
    #[serde(rename = "trailing_stoploss", skip_serializing_if = "Option::is_none")]
    pub trailing_stoploss: Option<f64>,

    /// Market protection percentage
    #[serde(rename = "market_protection", skip_serializing_if = "Option::is_none")]
    pub market_protection: Option<f64>,

    /// ICEBERG legs for iceberg orders
    #[serde(rename = "iceberg_legs", skip_serializing_if = "Option::is_none")]
    pub iceberg_legs: Option<u32>,

    /// ICEBERG quantity for iceberg orders
    #[serde(rename = "iceberg_quantity", skip_serializing_if = "Option::is_none")]
    pub iceberg_quantity: Option<u32>,

    /// Auction number
    #[serde(rename = "auction_number", skip_serializing_if = "Option::is_none")]
    pub auction_number: Option<String>,
}

/// Bracket order parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketOrderParams {
    /// Base order parameters
    #[serde(flatten)]
    pub order_params: OrderParams,

    /// Square off value (mandatory for bracket orders)
    pub squareoff: f64,

    /// Stoploss value (mandatory for bracket orders)
    pub stoploss: f64,

    /// Trailing stoploss (optional)
    #[serde(rename = "trailing_stoploss", skip_serializing_if = "Option::is_none")]
    pub trailing_stoploss: Option<f64>,
}

/// Cover order parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverOrderParams {
    /// Base order parameters
    #[serde(flatten)]
    pub order_params: OrderParams,

    /// Trigger price (mandatory for cover orders)
    #[serde(rename = "trigger_price")]
    pub trigger_price: f64,
}

/// Order modification parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderModifyParams {
    /// Order ID to modify
    #[serde(skip_serializing)]
    pub order_id: String,

    /// New quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,

    /// New price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// New trigger price
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,

    /// New order type
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// New validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,

    /// New disclosed quantity
    #[serde(rename = "disclosed_quantity", skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<u32>,

    /// Parent order ID for bracket/cover orders
    #[serde(rename = "parent_order_id", skip_serializing_if = "Option::is_none")]
    pub parent_order_id: Option<String>,
}

/// Builder for order parameters
#[derive(Debug, Clone)]
pub struct OrderBuilder {
    params: OrderParams,
}

impl OrderBuilder {
    /// Create a new order builder
    pub fn new() -> Self {
        Self {
            params: OrderParams {
                trading_symbol: String::new(),
                exchange: Exchange::NSE,
                transaction_type: TransactionType::BUY,
                order_type: OrderType::LIMIT,
                quantity: 0,
                product: Product::CNC,
                price: None,
                trigger_price: None,
                validity: Some(Validity::DAY),
                disclosed_quantity: None,
                tag: None,
                squareoff: None,
                stoploss: None,
                trailing_stoploss: None,
                market_protection: None,
                iceberg_legs: None,
                iceberg_quantity: None,
                auction_number: None,
            },
        }
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.params.trading_symbol = symbol.into();
        self
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.params.exchange = exchange;
        self
    }

    /// Set transaction type
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.params.transaction_type = transaction_type;
        self
    }

    /// Set order type
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.params.order_type = order_type;
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.params.quantity = quantity;
        self
    }

    /// Set product
    pub fn product(mut self, product: Product) -> Self {
        self.params.product = product;
        self
    }

    /// Set price (for limit orders)
    pub fn price(mut self, price: f64) -> Self {
        self.params.price = Some(price);
        self
    }

    /// Set trigger price (for SL orders)
    pub fn trigger_price(mut self, trigger_price: f64) -> Self {
        self.params.trigger_price = Some(trigger_price);
        self
    }

    /// Set validity
    pub fn validity(mut self, validity: Validity) -> Self {
        self.params.validity = Some(validity);
        self
    }

    /// Set disclosed quantity
    pub fn disclosed_quantity(mut self, disclosed_quantity: u32) -> Self {
        self.params.disclosed_quantity = Some(disclosed_quantity);
        self
    }

    /// Set tag
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.params.tag = Some(tag.into());
        self
    }

    /// Set market protection
    pub fn market_protection(mut self, market_protection: f64) -> Self {
        self.params.market_protection = Some(market_protection);
        self
    }

    /// Configure for iceberg order
    pub fn iceberg(mut self, legs: u32, quantity: u32) -> Self {
        self.params.iceberg_legs = Some(legs);
        self.params.iceberg_quantity = Some(quantity);
        self
    }

    /// Build the order parameters
    pub fn build(self) -> Result<OrderParams, String> {
        // Validate required fields
        if self.params.trading_symbol.is_empty() {
            return Err("Trading symbol is required".to_string());
        }

        if self.params.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }

        // Validate price for limit orders
        if self.params.order_type == OrderType::LIMIT && self.params.price.is_none() {
            return Err("Price is required for LIMIT orders".to_string());
        }

        // Validate trigger price for SL orders
        if matches!(self.params.order_type, OrderType::SL | OrderType::SLM)
            && self.params.trigger_price.is_none()
        {
            return Err("Trigger price is required for SL/SL-M orders".to_string());
        }

        Ok(self.params)
    }
}

impl Default for OrderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for bracket order parameters
pub struct BracketOrderBuilder {
    params: OrderParams,
    squareoff: Option<f64>,
    stoploss: Option<f64>,
    trailing_stoploss: Option<f64>,
}

impl BracketOrderBuilder {
    /// Create a new bracket order builder
    pub fn new() -> Self {
        Self {
            params: OrderParams {
                trading_symbol: String::new(),
                exchange: Exchange::NSE,
                transaction_type: TransactionType::BUY,
                order_type: OrderType::LIMIT,
                quantity: 0,
                product: Product::MIS,
                price: None,
                trigger_price: None,
                validity: Some(Validity::DAY),
                disclosed_quantity: None,
                tag: None,
                squareoff: None,
                stoploss: None,
                trailing_stoploss: None,
                market_protection: None,
                iceberg_legs: None,
                iceberg_quantity: None,
                auction_number: None,
            },
            squareoff: None,
            stoploss: None,
            trailing_stoploss: None,
        }
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.params.trading_symbol = symbol.into();
        self
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.params.exchange = exchange;
        self
    }

    /// Set transaction type
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.params.transaction_type = transaction_type;
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.params.quantity = quantity;
        self
    }

    /// Set price
    pub fn price(mut self, price: f64) -> Self {
        self.params.price = Some(price);
        self
    }

    /// Set square off value
    pub fn squareoff(mut self, squareoff: f64) -> Self {
        self.squareoff = Some(squareoff);
        self
    }

    /// Set stoploss value
    pub fn stoploss(mut self, stoploss: f64) -> Self {
        self.stoploss = Some(stoploss);
        self
    }

    /// Set trailing stoploss
    pub fn trailing_stoploss(mut self, trailing_stoploss: f64) -> Self {
        self.trailing_stoploss = Some(trailing_stoploss);
        self
    }

    /// Build the bracket order parameters
    pub fn build(self) -> Result<BracketOrderParams, String> {
        // Validate required fields
        if self.params.trading_symbol.is_empty() {
            return Err("Trading symbol is required".to_string());
        }

        if self.params.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }

        if self.params.price.is_none() {
            return Err("Price is required for bracket orders".to_string());
        }

        let squareoff = self
            .squareoff
            .ok_or("Square off value is required for bracket orders")?;
        let stoploss = self
            .stoploss
            .ok_or("Stoploss value is required for bracket orders")?;

        let mut order_params = self.params;
        order_params.squareoff = Some(squareoff);
        order_params.stoploss = Some(stoploss);
        order_params.trailing_stoploss = self.trailing_stoploss;

        Ok(BracketOrderParams {
            order_params,
            squareoff,
            stoploss,
            trailing_stoploss: self.trailing_stoploss,
        })
    }
}

impl Default for BracketOrderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
