use super::triggers::{GTTCondition, GTTCreateParams, GTTOrderParams, GTTTriggerType};
use crate::models::common::{Exchange, OrderType, Product, TransactionType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GTT order builder for creating complex GTT orders
#[derive(Debug, Clone)]
pub struct GTTOrderBuilder {
    exchange: Option<Exchange>,
    trading_symbol: Option<String>,
    transaction_type: Option<TransactionType>,
    order_type: Option<OrderType>,
    product: Option<Product>,
    quantity: Option<u32>,
    price: Option<f64>,
}

/// GTT builder for creating complete GTT triggers with orders
#[derive(Debug, Clone)]
pub struct GTTBuilder {
    gtt_type: Option<GTTTriggerType>,
    condition: Option<GTTCondition>,
    orders: Vec<GTTOrderParams>,
    expires_at: Option<DateTime<Utc>>,
}

/// GTT condition builder
#[derive(Debug, Clone)]
pub struct GTTConditionBuilder {
    exchange: Option<Exchange>,
    trading_symbol: Option<String>,
    trigger_values: Vec<f64>,
    last_price: Option<f64>,
}

/// Stop-loss GTT builder
#[derive(Debug, Clone)]
pub struct StopLossGTTBuilder {
    exchange: Option<Exchange>,
    trading_symbol: Option<String>,
    transaction_type: Option<TransactionType>,
    product: Option<Product>,
    quantity: Option<u32>,
    trigger_price: Option<f64>,
    limit_price: Option<f64>,
    current_price: Option<f64>,
}

/// Target GTT builder
#[derive(Debug, Clone)]
pub struct TargetGTTBuilder {
    exchange: Option<Exchange>,
    trading_symbol: Option<String>,
    transaction_type: Option<TransactionType>,
    product: Option<Product>,
    quantity: Option<u32>,
    target_price: Option<f64>,
    current_price: Option<f64>,
}

/// Bracket GTT builder (combines stop-loss and target)
#[derive(Debug, Clone)]
pub struct BracketGTTBuilder {
    exchange: Option<Exchange>,
    trading_symbol: Option<String>,
    transaction_type: Option<TransactionType>,
    product: Option<Product>,
    quantity: Option<u32>,
    stop_loss_price: Option<f64>,
    target_price: Option<f64>,
    current_price: Option<f64>,
}

/// GTT template for common patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTTemplate {
    /// Template name
    pub name: String,

    /// Template description
    pub description: String,

    /// GTT type
    pub gtt_type: GTTTriggerType,

    /// Template parameters
    pub template: GTTCreateParams,
}

impl GTTOrderBuilder {
    /// Create a new GTT order builder
    pub fn new() -> Self {
        Self {
            exchange: None,
            trading_symbol: None,
            transaction_type: None,
            order_type: None,
            product: None,
            quantity: None,
            price: None,
        }
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.trading_symbol = Some(symbol.into());
        self
    }

    /// Set transaction type
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    /// Set order type
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Set product type
    pub fn product(mut self, product: Product) -> Self {
        self.product = Some(product);
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Set price
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Build the GTT order parameters
    pub fn build(self) -> Result<GTTOrderParams, String> {
        Ok(GTTOrderParams {
            exchange: self.exchange.ok_or("Exchange is required")?,
            trading_symbol: self.trading_symbol.ok_or("Trading symbol is required")?,
            transaction_type: self
                .transaction_type
                .ok_or("Transaction type is required")?,
            order_type: self.order_type.ok_or("Order type is required")?,
            product: self.product.ok_or("Product is required")?,
            quantity: self.quantity.ok_or("Quantity is required")?,
            price: self.price.unwrap_or(0.0),
            result: None,
        })
    }
}

impl GTTConditionBuilder {
    /// Create a new GTT condition builder
    pub fn new() -> Self {
        Self {
            exchange: None,
            trading_symbol: None,
            trigger_values: Vec::new(),
            last_price: None,
        }
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.trading_symbol = Some(symbol.into());
        self
    }

    /// Add a trigger value
    pub fn trigger_value(mut self, value: f64) -> Self {
        self.trigger_values.push(value);
        self
    }

    /// Set multiple trigger values
    pub fn trigger_values(mut self, values: Vec<f64>) -> Self {
        self.trigger_values = values;
        self
    }

    /// Set last price
    pub fn last_price(mut self, price: f64) -> Self {
        self.last_price = Some(price);
        self
    }

    /// Build the GTT condition
    pub fn build(self) -> Result<GTTCondition, String> {
        if self.trigger_values.is_empty() {
            return Err("At least one trigger value is required".to_string());
        }

        Ok(GTTCondition {
            exchange: self.exchange.ok_or("Exchange is required")?,
            trading_symbol: self.trading_symbol.ok_or("Trading symbol is required")?,
            trigger_values: self.trigger_values,
            last_price: self.last_price.ok_or("Last price is required")?,
        })
    }
}

impl GTTBuilder {
    /// Create a new GTT builder
    pub fn new() -> Self {
        Self {
            gtt_type: None,
            condition: None,
            orders: Vec::new(),
            expires_at: None,
        }
    }

    /// Set GTT type
    pub fn gtt_type(mut self, gtt_type: GTTTriggerType) -> Self {
        self.gtt_type = Some(gtt_type);
        self
    }

    /// Set condition
    pub fn condition(mut self, condition: GTTCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Add an order
    pub fn add_order(mut self, order: GTTOrderParams) -> Self {
        self.orders.push(order);
        self
    }

    /// Set orders
    pub fn orders(mut self, orders: Vec<GTTOrderParams>) -> Self {
        self.orders = orders;
        self
    }

    /// Set expiry time
    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Build the GTT create parameters
    pub fn build(self) -> Result<GTTCreateParams, String> {
        if self.orders.is_empty() {
            return Err("At least one order is required".to_string());
        }

        let params = GTTCreateParams {
            gtt_type: self.gtt_type.ok_or("GTT type is required")?,
            condition: self.condition.ok_or("Condition is required")?,
            orders: self.orders,
            expires_at: self.expires_at,
        };

        params.validate()?;
        Ok(params)
    }
}

impl StopLossGTTBuilder {
    /// Create a new stop-loss GTT builder
    pub fn new() -> Self {
        Self {
            exchange: None,
            trading_symbol: None,
            transaction_type: None,
            product: None,
            quantity: None,
            trigger_price: None,
            limit_price: None,
            current_price: None,
        }
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.trading_symbol = Some(symbol.into());
        self
    }

    /// Set transaction type (usually SELL for long positions, BUY for short positions)
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    /// Set product type
    pub fn product(mut self, product: Product) -> Self {
        self.product = Some(product);
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Set trigger price (stop-loss level)
    pub fn trigger_price(mut self, price: f64) -> Self {
        self.trigger_price = Some(price);
        self
    }

    /// Set limit price (for SL-L orders)
    pub fn limit_price(mut self, price: f64) -> Self {
        self.limit_price = Some(price);
        self
    }

    /// Set current market price
    pub fn current_price(mut self, price: f64) -> Self {
        self.current_price = Some(price);
        self
    }

    /// Build stop-loss GTT with market order
    pub fn build_market(self) -> Result<GTTCreateParams, String> {
        let condition = GTTConditionBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(
                self.trading_symbol
                    .clone()
                    .ok_or("Trading symbol is required")?,
            )
            .trigger_value(self.trigger_price.ok_or("Trigger price is required")?)
            .last_price(self.current_price.ok_or("Current price is required")?)
            .build()?;

        let order = GTTOrderBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(self.trading_symbol.ok_or("Trading symbol is required")?)
            .transaction_type(
                self.transaction_type
                    .ok_or("Transaction type is required")?,
            )
            .order_type(OrderType::MARKET)
            .product(self.product.ok_or("Product is required")?)
            .quantity(self.quantity.ok_or("Quantity is required")?)
            .build()?;

        GTTBuilder::new()
            .gtt_type(GTTTriggerType::Single)
            .condition(condition)
            .add_order(order)
            .build()
    }

    /// Build stop-loss GTT with limit order
    pub fn build_limit(self) -> Result<GTTCreateParams, String> {
        let condition = GTTConditionBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(
                self.trading_symbol
                    .clone()
                    .ok_or("Trading symbol is required")?,
            )
            .trigger_value(self.trigger_price.ok_or("Trigger price is required")?)
            .last_price(self.current_price.ok_or("Current price is required")?)
            .build()?;

        let order = GTTOrderBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(self.trading_symbol.ok_or("Trading symbol is required")?)
            .transaction_type(
                self.transaction_type
                    .ok_or("Transaction type is required")?,
            )
            .order_type(OrderType::LIMIT)
            .product(self.product.ok_or("Product is required")?)
            .quantity(self.quantity.ok_or("Quantity is required")?)
            .price(self.limit_price.ok_or("Limit price is required")?)
            .build()?;

        GTTBuilder::new()
            .gtt_type(GTTTriggerType::Single)
            .condition(condition)
            .add_order(order)
            .build()
    }
}

impl TargetGTTBuilder {
    /// Create a new target GTT builder
    pub fn new() -> Self {
        Self {
            exchange: None,
            trading_symbol: None,
            transaction_type: None,
            product: None,
            quantity: None,
            target_price: None,
            current_price: None,
        }
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.trading_symbol = Some(symbol.into());
        self
    }

    /// Set transaction type (usually SELL for long positions, BUY for short positions)
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    /// Set product type
    pub fn product(mut self, product: Product) -> Self {
        self.product = Some(product);
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Set target price
    pub fn target_price(mut self, price: f64) -> Self {
        self.target_price = Some(price);
        self
    }

    /// Set current market price
    pub fn current_price(mut self, price: f64) -> Self {
        self.current_price = Some(price);
        self
    }

    /// Build target GTT
    pub fn build(self) -> Result<GTTCreateParams, String> {
        let condition = GTTConditionBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(
                self.trading_symbol
                    .clone()
                    .ok_or("Trading symbol is required")?,
            )
            .trigger_value(self.target_price.ok_or("Target price is required")?)
            .last_price(self.current_price.ok_or("Current price is required")?)
            .build()?;

        let order = GTTOrderBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(self.trading_symbol.ok_or("Trading symbol is required")?)
            .transaction_type(
                self.transaction_type
                    .ok_or("Transaction type is required")?,
            )
            .order_type(OrderType::LIMIT)
            .product(self.product.ok_or("Product is required")?)
            .quantity(self.quantity.ok_or("Quantity is required")?)
            .price(self.target_price.ok_or("Target price is required")?)
            .build()?;

        GTTBuilder::new()
            .gtt_type(GTTTriggerType::Single)
            .condition(condition)
            .add_order(order)
            .build()
    }
}

impl BracketGTTBuilder {
    /// Create a new bracket GTT builder
    pub fn new() -> Self {
        Self {
            exchange: None,
            trading_symbol: None,
            transaction_type: None,
            product: None,
            quantity: None,
            stop_loss_price: None,
            target_price: None,
            current_price: None,
        }
    }

    /// Set exchange
    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    /// Set trading symbol
    pub fn trading_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.trading_symbol = Some(symbol.into());
        self
    }

    /// Set transaction type (usually SELL for long positions, BUY for short positions)
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    /// Set product type
    pub fn product(mut self, product: Product) -> Self {
        self.product = Some(product);
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Set stop-loss price
    pub fn stop_loss_price(mut self, price: f64) -> Self {
        self.stop_loss_price = Some(price);
        self
    }

    /// Set target price
    pub fn target_price(mut self, price: f64) -> Self {
        self.target_price = Some(price);
        self
    }

    /// Set current market price
    pub fn current_price(mut self, price: f64) -> Self {
        self.current_price = Some(price);
        self
    }

    /// Build bracket GTT (OCO - One Cancels Other)
    pub fn build(self) -> Result<GTTCreateParams, String> {
        let stop_loss_price = self.stop_loss_price.ok_or("Stop-loss price is required")?;
        let target_price = self.target_price.ok_or("Target price is required")?;

        let condition = GTTConditionBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(
                self.trading_symbol
                    .clone()
                    .ok_or("Trading symbol is required")?,
            )
            .trigger_values(vec![stop_loss_price, target_price])
            .last_price(self.current_price.ok_or("Current price is required")?)
            .build()?;

        // Stop-loss order (market order)
        let stop_loss_order = GTTOrderBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(
                self.trading_symbol
                    .clone()
                    .ok_or("Trading symbol is required")?,
            )
            .transaction_type(
                self.transaction_type
                    .ok_or("Transaction type is required")?,
            )
            .order_type(OrderType::MARKET)
            .product(self.product.ok_or("Product is required")?)
            .quantity(self.quantity.ok_or("Quantity is required")?)
            .build()?;

        // Target order (limit order)
        let target_order = GTTOrderBuilder::new()
            .exchange(self.exchange.ok_or("Exchange is required")?)
            .trading_symbol(self.trading_symbol.ok_or("Trading symbol is required")?)
            .transaction_type(
                self.transaction_type
                    .ok_or("Transaction type is required")?,
            )
            .order_type(OrderType::LIMIT)
            .product(self.product.ok_or("Product is required")?)
            .quantity(self.quantity.ok_or("Quantity is required")?)
            .price(target_price)
            .build()?;

        GTTBuilder::new()
            .gtt_type(GTTTriggerType::TwoLeg)
            .condition(condition)
            .orders(vec![stop_loss_order, target_order])
            .build()
    }
}

impl Default for GTTOrderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GTTConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GTTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StopLossGTTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TargetGTTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BracketGTTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Common GTT patterns and utilities
impl GTTTemplate {
    /// Create a stop-loss template
    pub fn stop_loss_template() -> Self {
        let condition = GTTCondition {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            trigger_values: vec![0.0],
            last_price: 0.0,
        };

        let order = GTTOrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            transaction_type: TransactionType::SELL,
            order_type: OrderType::MARKET,
            product: Product::CNC,
            quantity: 0,
            price: 0.0,
            result: None,
        };

        Self {
            name: "Stop Loss".to_string(),
            description: "Basic stop-loss GTT template".to_string(),
            gtt_type: GTTTriggerType::Single,
            template: GTTCreateParams {
                gtt_type: GTTTriggerType::Single,
                condition,
                orders: vec![order],
                expires_at: None,
            },
        }
    }

    /// Create a target template
    pub fn target_template() -> Self {
        let condition = GTTCondition {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            trigger_values: vec![0.0],
            last_price: 0.0,
        };

        let order = GTTOrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            transaction_type: TransactionType::SELL,
            order_type: OrderType::LIMIT,
            product: Product::CNC,
            quantity: 0,
            price: 0.0,
            result: None,
        };

        Self {
            name: "Target".to_string(),
            description: "Basic target GTT template".to_string(),
            gtt_type: GTTTriggerType::Single,
            template: GTTCreateParams {
                gtt_type: GTTTriggerType::Single,
                condition,
                orders: vec![order],
                expires_at: None,
            },
        }
    }

    /// Create a bracket (OCO) template
    pub fn bracket_template() -> Self {
        let condition = GTTCondition {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            trigger_values: vec![0.0, 0.0],
            last_price: 0.0,
        };

        let stop_loss_order = GTTOrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            transaction_type: TransactionType::SELL,
            order_type: OrderType::MARKET,
            product: Product::CNC,
            quantity: 0,
            price: 0.0,
            result: None,
        };

        let target_order = GTTOrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "TEMPLATE".to_string(),
            transaction_type: TransactionType::SELL,
            order_type: OrderType::LIMIT,
            product: Product::CNC,
            quantity: 0,
            price: 0.0,
            result: None,
        };

        Self {
            name: "Bracket Order".to_string(),
            description: "OCO bracket order with stop-loss and target".to_string(),
            gtt_type: GTTTriggerType::TwoLeg,
            template: GTTCreateParams {
                gtt_type: GTTTriggerType::TwoLeg,
                condition,
                orders: vec![stop_loss_order, target_order],
                expires_at: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::*;

    #[test]
    fn test_gtt_order_builder() {
        let order = GTTOrderBuilder::new()
            .exchange(Exchange::NSE)
            .trading_symbol("RELIANCE")
            .transaction_type(TransactionType::SELL)
            .order_type(OrderType::MARKET)
            .product(Product::CNC)
            .quantity(10)
            .build()
            .unwrap();

        assert_eq!(order.exchange, Exchange::NSE);
        assert_eq!(order.trading_symbol, "RELIANCE");
        assert_eq!(order.transaction_type, TransactionType::SELL);
        assert_eq!(order.quantity, 10);
    }

    #[test]
    fn test_stop_loss_gtt_builder() {
        let gtt = StopLossGTTBuilder::new()
            .exchange(Exchange::NSE)
            .trading_symbol("RELIANCE")
            .transaction_type(TransactionType::SELL)
            .product(Product::CNC)
            .quantity(10)
            .trigger_price(2000.0)
            .current_price(2100.0)
            .build_market()
            .unwrap();

        assert_eq!(gtt.gtt_type, GTTTriggerType::Single);
        assert_eq!(gtt.condition.trigger_values, vec![2000.0]);
        assert_eq!(gtt.orders.len(), 1);
        assert_eq!(gtt.orders[0].order_type, OrderType::MARKET);
    }

    #[test]
    fn test_bracket_gtt_builder() {
        let gtt = BracketGTTBuilder::new()
            .exchange(Exchange::NSE)
            .trading_symbol("RELIANCE")
            .transaction_type(TransactionType::SELL)
            .product(Product::CNC)
            .quantity(10)
            .stop_loss_price(2000.0)
            .target_price(2200.0)
            .current_price(2100.0)
            .build()
            .unwrap();

        assert_eq!(gtt.gtt_type, GTTTriggerType::TwoLeg);
        assert_eq!(gtt.condition.trigger_values, vec![2000.0, 2200.0]);
        assert_eq!(gtt.orders.len(), 2);
    }

    #[test]
    fn test_gtt_templates() {
        let template = GTTTemplate::stop_loss_template();
        assert_eq!(template.name, "Stop Loss");
        assert_eq!(template.gtt_type, GTTTriggerType::Single);

        let template = GTTTemplate::bracket_template();
        assert_eq!(template.name, "Bracket Order");
        assert_eq!(template.gtt_type, GTTTriggerType::TwoLeg);
    }
}
