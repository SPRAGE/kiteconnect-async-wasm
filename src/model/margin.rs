//! Margin calculation models for KiteConnect API
//!
//! This module contains models for margin calculations, charges computation,
//! and basket margin analysis. These models are used with the KiteConnect
//! margin calculator APIs to determine required margins and charges for orders.

use serde::{Deserialize, Serialize};

/// Parameters for margin calculation request
///
/// Represents an order in the Margin Calculator API for calculating
/// required margins before placing an order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderMarginParam {
    /// Exchange where the order will be placed (NSE, BSE, NFO, etc.)
    pub exchange: String,
    
    /// Trading symbol of the instrument
    pub tradingsymbol: String,
    
    /// Transaction type (BUY or SELL)
    pub transaction_type: String,
    
    /// Order variety (regular, amo, bo, co, iceberg, auction)
    pub variety: String,
    
    /// Product type (CNC, MIS, NRML, MTF)
    pub product: String,
    
    /// Order type (MARKET, LIMIT, SL, SL-M)
    pub order_type: String,
    
    /// Order quantity
    pub quantity: f64,
    
    /// Order price (optional for MARKET orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    
    /// Trigger price for stop loss orders (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
}

/// Parameters for charges calculation request
///
/// Represents an order in the Charges Calculator API for calculating
/// brokerage and other charges for executed orders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderChargesParam {
    /// Order ID for the executed order
    pub order_id: String,
    
    /// Exchange where the order was placed
    pub exchange: String,
    
    /// Trading symbol of the instrument
    pub tradingsymbol: String,
    
    /// Transaction type (BUY or SELL)
    pub transaction_type: String,
    
    /// Order variety
    pub variety: String,
    
    /// Product type
    pub product: String,
    
    /// Order type
    pub order_type: String,
    
    /// Order quantity
    pub quantity: f64,
    
    /// Average price at which the order was executed
    pub average_price: f64,
}

/// PnL details for margin calculations
///
/// Represents realized and unrealized profit and loss for positions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PNL {
    /// Realized PnL from closed positions
    pub realised: f64,
    
    /// Unrealized PnL from open positions
    pub unrealised: f64,
}

/// GST breakdown for charges
///
/// Represents the various GST charges applied to orders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GST {
    /// Integrated Goods and Services Tax
    #[serde(rename = "igst")]
    pub igst: f64,
    
    /// Central Goods and Services Tax
    #[serde(rename = "cgst")]
    pub cgst: f64,
    
    /// State Goods and Services Tax
    #[serde(rename = "sgst")]
    pub sgst: f64,
    
    /// Total GST amount
    pub total: f64,
}

/// Breakdown of various charges applied to an order
///
/// Represents comprehensive charge breakdown including taxes,
/// brokerage, and regulatory charges.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Charges {
    /// Transaction tax (STT/CTT)
    pub transaction_tax: f64,
    
    /// Type of transaction tax applied
    pub transaction_tax_type: String,
    
    /// Exchange turnover charge
    pub exchange_turnover_charge: f64,
    
    /// SEBI turnover charge
    #[serde(rename = "sebi_turnover_charge")]
    pub sebi_turnover_charge: f64,
    
    /// Brokerage charged
    pub brokerage: f64,
    
    /// Stamp duty
    pub stamp_duty: f64,
    
    /// GST breakdown
    pub gst: GST,
    
    /// Total charges
    pub total: f64,
}

/// Response from the Margin Calculator API
///
/// Contains margin requirements and charges for a specific order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderMargins {
    /// Type of margin calculation
    #[serde(rename = "type")]
    pub margin_type: String,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Exchange
    pub exchange: String,
    
    /// SPAN margin requirement
    #[serde(rename = "span")]
    pub span: f64,
    
    /// Exposure margin requirement
    pub exposure: f64,
    
    /// Option premium margin
    pub option_premium: f64,
    
    /// Additional margin requirement
    pub additional: f64,
    
    /// Bracket order margin
    #[serde(rename = "bo")]
    pub bo: f64,
    
    /// Cash requirement
    pub cash: f64,
    
    /// Value at Risk margin
    #[serde(rename = "var")]
    pub var: f64,
    
    /// PnL details
    #[serde(rename = "pnl")]
    pub pnl: PNL,
    
    /// Leverage multiplier
    pub leverage: f64,
    
    /// Charges breakdown
    pub charges: Charges,
    
    /// Total margin requirement
    pub total: f64,
}

/// Response from the Charges Calculator API
///
/// Contains detailed charge breakdown for an executed order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderCharges {
    /// Exchange where the order was placed
    pub exchange: String,
    
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    
    /// Transaction type
    pub transaction_type: String,
    
    /// Order variety
    pub variety: String,
    
    /// Product type
    pub product: String,
    
    /// Order type
    pub order_type: String,
    
    /// Order quantity
    pub quantity: f64,
    
    /// Order price
    pub price: f64,
    
    /// Charges breakdown
    pub charges: Charges,
}

/// Response from the Basket Margin Calculator API
///
/// Contains margin calculations for a basket of orders including
/// initial requirements, final requirements after netting, and
/// individual order margins.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasketMargins {
    /// Initial margin requirements (sum of individual orders)
    pub initial: OrderMargins,
    
    /// Final margin requirements (after netting and optimization)
    #[serde(rename = "final")]
    pub r#final: OrderMargins,
    
    /// Individual order margin details
    pub orders: Vec<OrderMargins>,
}

/// Parameters for margin calculation requests
///
/// Wrapper for multiple order margin calculations with options
/// for compact response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginParams {
    /// List of orders for margin calculation
    pub order_params: Vec<OrderMarginParam>,
    
    /// Whether to return compact response (less detailed)
    pub compact: bool,
}

/// Parameters for basket margin calculation requests
///
/// Wrapper for basket margin calculations with options for
/// compact response and position consideration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBasketParams {
    /// List of orders for basket margin calculation
    pub order_params: Vec<OrderMarginParam>,
    
    /// Whether to return compact response
    pub compact: bool,
    
    /// Whether to consider existing positions for netting
    pub consider_positions: bool,
}

/// Parameters for charges calculation requests
///
/// Wrapper for multiple order charges calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChargesParams {
    /// List of orders for charges calculation
    pub order_params: Vec<OrderChargesParam>,
}

impl OrderMarginParam {
    /// Creates a new OrderMarginParam with required fields
    ///
    /// # Arguments
    /// 
    /// * `exchange` - Exchange name (NSE, BSE, NFO, etc.)
    /// * `tradingsymbol` - Trading symbol
    /// * `transaction_type` - BUY or SELL
    /// * `variety` - Order variety
    /// * `product` - Product type
    /// * `order_type` - Order type
    /// * `quantity` - Order quantity
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::model::margin::OrderMarginParam;
    ///
    /// let param = OrderMarginParam::new(
    ///     "NSE".to_string(),
    ///     "INFY".to_string(),
    ///     "BUY".to_string(),
    ///     "regular".to_string(),
    ///     "CNC".to_string(),
    ///     "MARKET".to_string(),
    ///     1.0
    /// );
    /// ```
    pub fn new(
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        variety: String,
        product: String,
        order_type: String,
        quantity: f64,
    ) -> Self {
        Self {
            exchange,
            tradingsymbol,
            transaction_type,
            variety,
            product,
            order_type,
            quantity,
            price: None,
            trigger_price: None,
        }
    }
    
    /// Sets the order price (for LIMIT orders)
    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
    
    /// Sets the trigger price (for stop loss orders)
    pub fn with_trigger_price(mut self, trigger_price: f64) -> Self {
        self.trigger_price = Some(trigger_price);
        self
    }
    
    /// Checks if this is a market order
    pub fn is_market_order(&self) -> bool {
        self.order_type.to_uppercase() == "MARKET"
    }
    
    /// Checks if this is a stop loss order
    pub fn is_stop_loss_order(&self) -> bool {
        matches!(self.order_type.to_uppercase().as_str(), "SL" | "SL-M")
    }
}

impl OrderChargesParam {
    /// Creates a new OrderChargesParam for charges calculation
    ///
    /// # Arguments
    ///
    /// * `order_id` - Order ID of the executed order
    /// * `exchange` - Exchange name
    /// * `tradingsymbol` - Trading symbol
    /// * `transaction_type` - BUY or SELL
    /// * `variety` - Order variety
    /// * `product` - Product type
    /// * `order_type` - Order type
    /// * `quantity` - Order quantity
    /// * `average_price` - Average execution price
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::model::margin::OrderChargesParam;
    ///
    /// let param = OrderChargesParam::new(
    ///     "12345".to_string(),
    ///     "NSE".to_string(),
    ///     "INFY".to_string(),
    ///     "BUY".to_string(),
    ///     "regular".to_string(),
    ///     "CNC".to_string(),
    ///     "MARKET".to_string(),
    ///     1.0,
    ///     1500.0
    /// );
    /// ```
    pub fn new(
        order_id: String,
        exchange: String,
        tradingsymbol: String,
        transaction_type: String,
        variety: String,
        product: String,
        order_type: String,
        quantity: f64,
        average_price: f64,
    ) -> Self {
        Self {
            order_id,
            exchange,
            tradingsymbol,
            transaction_type,
            variety,
            product,
            order_type,
            quantity,
            average_price,
        }
    }
    
    /// Calculates the total trade value
    pub fn trade_value(&self) -> f64 {
        self.quantity * self.average_price
    }
}

impl GetMarginParams {
    /// Creates new GetMarginParams for single order
    pub fn single(order_param: OrderMarginParam, compact: bool) -> Self {
        Self {
            order_params: vec![order_param],
            compact,
        }
    }
    
    /// Creates new GetMarginParams for multiple orders
    pub fn multiple(order_params: Vec<OrderMarginParam>, compact: bool) -> Self {
        Self {
            order_params,
            compact,
        }
    }
}

impl GetBasketParams {
    /// Creates new GetBasketParams with default settings
    pub fn new(order_params: Vec<OrderMarginParam>) -> Self {
        Self {
            order_params,
            compact: false,
            consider_positions: false,
        }
    }
    
    /// Sets compact mode
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
    
    /// Sets position consideration
    pub fn with_consider_positions(mut self, consider_positions: bool) -> Self {
        self.consider_positions = consider_positions;
        self
    }
}

impl GetChargesParams {
    /// Creates new GetChargesParams for single order
    pub fn single(order_param: OrderChargesParam) -> Self {
        Self {
            order_params: vec![order_param],
        }
    }
    
    /// Creates new GetChargesParams for multiple orders
    pub fn multiple(order_params: Vec<OrderChargesParam>) -> Self {
        Self {
            order_params,
        }
    }
}

impl Charges {
    /// Calculates total brokerage and taxes (excluding GST)
    pub fn total_before_gst(&self) -> f64 {
        self.transaction_tax + 
        self.exchange_turnover_charge + 
        self.sebi_turnover_charge + 
        self.brokerage + 
        self.stamp_duty
    }
    
    /// Checks if any charges are applied
    pub fn has_charges(&self) -> bool {
        self.total > 0.0
    }
}

impl GST {
    /// Validates that individual GST components sum to total
    pub fn is_valid(&self) -> bool {
        let calculated_total = self.igst + self.cgst + self.sgst;
        (calculated_total - self.total).abs() < 0.01 // Allow small floating point differences
    }
}

impl OrderMargins {
    /// Calculates total margin requirement without charges
    pub fn margin_before_charges(&self) -> f64 {
        self.span + self.exposure + self.option_premium + 
        self.additional + self.bo + self.var
    }
    
    /// Checks if leverage is available for this order
    pub fn has_leverage(&self) -> bool {
        self.leverage > 1.0
    }
    
    /// Gets effective margin requirement
    pub fn effective_margin(&self) -> f64 {
        if self.leverage > 1.0 {
            self.total / self.leverage
        } else {
            self.total
        }
    }
}

impl BasketMargins {
    /// Calculates margin savings from basket netting
    pub fn margin_savings(&self) -> f64 {
        self.initial.total - self.r#final.total
    }
    
    /// Calculates margin savings percentage
    pub fn margin_savings_percentage(&self) -> f64 {
        if self.initial.total > 0.0 {
            (self.margin_savings() / self.initial.total) * 100.0
        } else {
            0.0
        }
    }
    
    /// Gets the number of orders in the basket
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_margin_param_creation() {
        let param = OrderMarginParam::new(
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            "regular".to_string(),
            "CNC".to_string(),
            "MARKET".to_string(),
            1.0,
        );
        
        assert_eq!(param.exchange, "NSE");
        assert_eq!(param.tradingsymbol, "INFY");
        assert_eq!(param.quantity, 1.0);
        assert!(param.price.is_none());
        assert!(param.is_market_order());
        assert!(!param.is_stop_loss_order());
    }
    
    #[test]
    fn test_order_margin_param_with_price() {
        let param = OrderMarginParam::new(
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            "regular".to_string(),
            "CNC".to_string(),
            "LIMIT".to_string(),
            1.0,
        ).with_price(1500.0).with_trigger_price(1490.0);
        
        assert_eq!(param.price, Some(1500.0));
        assert_eq!(param.trigger_price, Some(1490.0));
        assert!(!param.is_market_order());
    }
    
    #[test]
    fn test_order_charges_param() {
        let param = OrderChargesParam::new(
            "12345".to_string(),
            "NSE".to_string(),
            "INFY".to_string(),
            "BUY".to_string(),
            "regular".to_string(),
            "CNC".to_string(),
            "MARKET".to_string(),
            10.0,
            1500.0,
        );
        
        assert_eq!(param.trade_value(), 15000.0);
        assert_eq!(param.order_id, "12345");
    }
    
    #[test]
    fn test_gst_validation() {
        let valid_gst = GST {
            igst: 10.0,
            cgst: 5.0,
            sgst: 5.0,
            total: 20.0,
        };
        
        let invalid_gst = GST {
            igst: 10.0,
            cgst: 5.0,
            sgst: 5.0,
            total: 25.0, // Wrong total
        };
        
        assert!(valid_gst.is_valid());
        assert!(!invalid_gst.is_valid());
    }
    
    #[test]
    fn test_basket_margins_calculations() {
        let initial = OrderMargins {
            margin_type: "equity".to_string(),
            trading_symbol: "INFY".to_string(),
            exchange: "NSE".to_string(),
            span: 1000.0,
            exposure: 500.0,
            option_premium: 0.0,
            additional: 0.0,
            bo: 0.0,
            cash: 0.0,
            var: 0.0,
            pnl: PNL { realised: 0.0, unrealised: 0.0 },
            leverage: 1.0,
            charges: Charges {
                transaction_tax: 10.0,
                transaction_tax_type: "STT".to_string(),
                exchange_turnover_charge: 5.0,
                sebi_turnover_charge: 1.0,
                brokerage: 20.0,
                stamp_duty: 2.0,
                gst: GST { igst: 0.0, cgst: 3.6, sgst: 3.6, total: 7.2 },
                total: 45.2,
            },
            total: 1545.2,
        };
        
        let final_margins = OrderMargins {
            margin_type: "equity".to_string(),
            trading_symbol: "INFY".to_string(),
            exchange: "NSE".to_string(),
            span: 800.0,
            exposure: 400.0,
            option_premium: 0.0,
            additional: 0.0,
            bo: 0.0,
            cash: 0.0,
            var: 0.0,
            pnl: PNL { realised: 0.0, unrealised: 0.0 },
            leverage: 1.0,
            charges: Charges {
                transaction_tax: 10.0,
                transaction_tax_type: "STT".to_string(),
                exchange_turnover_charge: 5.0,
                sebi_turnover_charge: 1.0,
                brokerage: 20.0,
                stamp_duty: 2.0,
                gst: GST { igst: 0.0, cgst: 3.6, sgst: 3.6, total: 7.2 },
                total: 45.2,
            },
            total: 1245.2,
        };
        
        let basket = BasketMargins {
            initial: initial.clone(),
            r#final: final_margins,
            orders: vec![initial],
        };
        
        assert_eq!(basket.margin_savings(), 300.0);
        assert!((basket.margin_savings_percentage() - 19.42).abs() < 0.1);
        assert_eq!(basket.order_count(), 1);
    }
}
