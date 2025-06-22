use anyhow::Result;
use std::collections::HashMap;
use super::{client::KiteConnect, request::RequestHandler};

// Import model types for typed responses
use crate::model::{
    Order, OrderResponse, Trade, Trades,
};

impl KiteConnect {
    /// Place an order
    /// 
    /// Places a new order on the exchange with specified parameters.
    /// 
    /// # Arguments
    /// 
    /// * `variety` - Order variety (regular, amo, co, iceberg, etc.)
    /// * `exchange` - Exchange to place the order on (NSE, BSE, etc.)
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `transaction_type` - BUY or SELL
    /// * `quantity` - Order quantity
    /// * `product` - Product type (MIS, CNC, NRML)
    /// * `order_type` - Order type (MARKET, LIMIT, SL, SL-M)
    /// * `price` - Order price (required for LIMIT orders)
    /// * `validity` - Order validity (DAY, IOC)
    /// * `disclosed_quantity` - Iceberg quantity
    /// * `trigger_price` - Trigger price for stop-loss orders
    /// * `squareoff` - Squareoff price for bracket orders
    /// * `stoploss` - Stoploss price for bracket orders
    /// * `trailing_stoploss` - Trailing stoploss for bracket orders
    /// * `tag` - Optional tag to identify the order
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the order ID of the placed order
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let order_response = client.place_order(
    ///     "regular",
    ///     "NSE",
    ///     "RELIANCE",
    ///     "BUY", 
    ///     "1",
    ///     Some("CNC"),
    ///     Some("MARKET"),
    ///     None,
    ///     Some("DAY"),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     Some("my_order")
    /// ).await?;
    /// 
    /// println!("Order placed with ID: {}", order_response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order(
        &self,
        variety: &str,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        quantity: &str,
        product: Option<&str>,
        order_type: Option<&str>,
        price: Option<&str>,
        validity: Option<&str>,
        disclosed_quantity: Option<&str>,
        trigger_price: Option<&str>,
        squareoff: Option<&str>,
        stoploss: Option<&str>,
        trailing_stoploss: Option<&str>,
        tag: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("variety", variety);
        params.insert("exchange", exchange);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        params.insert("quantity", quantity);
        
        if let Some(product) = product { params.insert("product", product); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(squareoff) = squareoff { params.insert("squareoff", squareoff); }
        if let Some(stoploss) = stoploss { params.insert("stoploss", stoploss); }
        if let Some(trailing_stoploss) = trailing_stoploss { params.insert("trailing_stoploss", trailing_stoploss); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url(&format!("/orders/{}", variety), None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Modify an open order
    /// 
    /// Modifies the parameters of an existing open order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to modify
    /// * `variety` - Order variety
    /// * `quantity` - New order quantity
    /// * `price` - New order price
    /// * `order_type` - New order type
    /// * `validity` - New order validity
    /// * `disclosed_quantity` - New disclosed quantity
    /// * `trigger_price` - New trigger price
    /// * `parent_order_id` - Parent order ID for child orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the order modification response
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let response = client.modify_order(
    ///     "123456",          // Order ID
    ///     "regular",         // Variety
    ///     Some("2"),         // New quantity
    ///     Some("2500.0"),    // New price
    ///     Some("LIMIT"),     // Order type
    ///     Some("DAY"),       // Validity
    ///     None,              // Disclosed quantity
    ///     None,              // Trigger price
    ///     None               // Parent order ID
    /// ).await?;
    /// 
    /// println!("Order modified: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn modify_order(
        &self,
        order_id: &str,
        variety: &str,
        quantity: Option<&str>,
        price: Option<&str>,
        order_type: Option<&str>,
        validity: Option<&str>,
        disclosed_quantity: Option<&str>,
        trigger_price: Option<&str>,
        parent_order_id: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(parent_order_id) = parent_order_id { params.insert("parent_order_id", parent_order_id); }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Cancel an order
    /// 
    /// Cancels an open order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to cancel
    /// * `variety` - Order variety
    /// * `parent_order_id` - Parent order ID for bracket/cover orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the cancellation response
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let response = client.cancel_order(
    ///     "123456",    // Order ID
    ///     "regular",   // Variety
    ///     None         // Parent order ID
    /// ).await?;
    /// 
    /// println!("Order cancelled: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        if let Some(parent_order_id) = parent_order_id {
            params.insert("parent_order_id", parent_order_id);
        }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "DELETE", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Exit a BO/CO order
    /// 
    /// Exits a bracket order or cover order (alias for cancel_order).
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to exit
    /// * `variety` - Order variety
    /// * `parent_order_id` - Parent order ID for bracket/cover orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the exit response
    pub async fn exit_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<OrderResponse> {
        self.cancel_order(order_id, variety, parent_order_id).await
    }

    /// Retrieves a list of all orders for the current trading day
    /// 
    /// Returns all orders placed by the user for the current trading day,
    /// including pending, completed, rejected, and cancelled orders.
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Order>>` containing orders data with fields like:
    /// - `order_id` - Unique order identifier
    /// - `tradingsymbol` - Trading symbol
    /// - `quantity` - Order quantity
    /// - `price` - Order price
    /// - `status` - Order status (OPEN, COMPLETE, CANCELLED, REJECTED)
    /// - `order_type` - Order type (MARKET, LIMIT, SL, SL-M)
    /// - `product` - Product type (MIS, CNC, NRML)
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails or the user is not authenticated.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let orders = client.orders().await?;
    /// println!("Orders: {:?}", orders);
    /// 
    /// // Check order statuses
    /// for order in &orders {
    ///     println!("Order {}: {} - {}", 
    ///         order.order_id, 
    ///         order.trading_symbol, 
    ///         order.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn orders(&self) -> Result<Vec<Order>> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get the list of order history
    /// 
    /// Retrieves the complete order history/lifecycle for a specific order,
    /// showing all state changes and modifications made to the order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The unique order ID to retrieve history for
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Order>>` containing the order history as a list of order states
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main] 
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let history = client.order_history("171229000724687").await?;
    /// 
    /// // Access order history directly
    /// for order_state in &history {
    ///     println!("Order status: {}, Time: {:?}", 
    ///         order_state.status, order_state.order_timestamp);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_history(&self, order_id: &str) -> Result<Vec<Order>> {
        let params = vec![("order_id", order_id)];
        let url = self.build_url("/orders", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get all trades
    /// 
    /// Retrieves all executed trades for the current trading session.
    /// Trades represent completed transactions with details about execution price,
    /// quantity, and timing.
    /// 
    /// # Returns
    /// 
    /// A `Result<Trades>` containing the trades data
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails or the user is not authenticated.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let trades = client.trades().await?;
    /// println!("Trades: {:?}", trades);
    /// 
    /// // Calculate total volume
    /// let total_volume: f64 = trades.iter().map(|t| t.quantity).sum();
    /// println!("Total volume: {}", total_volume);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades(&self) -> Result<Trades> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get all trades for a specific order
    /// 
    /// Retrieves all executed trades/fills for a specific order. This is useful
    /// for orders that might be partially filled or executed in multiple lots.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The unique order ID to retrieve trades for
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Trade>>` containing the list of trades for the order
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let trades = client.order_trades("171229000724687").await?;
    /// 
    /// // Access trade details directly
    /// for trade in &trades {
    ///     println!("Trade: {} shares at {}", trade.quantity, trade.average_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_trades(&self, order_id: &str) -> Result<Vec<Trade>> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Modify an open position product type
    /// 
    /// Converts the product type of an existing position (e.g., from MIS to CNC).
    /// This allows changing the position's product without exiting and re-entering.
    /// 
    /// # Arguments
    /// 
    /// * `exchange` - Exchange where the position exists (NSE, BSE, etc.)
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `transaction_type` - BUY or SELL 
    /// * `position_type` - "day" or "overnight"
    /// * `quantity` - Quantity of position to convert
    /// * `old_product` - Current product type (MIS, CNC, NRML)
    /// * `new_product` - Target product type (MIS, CNC, NRML)
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing conversion confirmation
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// // Convert MIS position to CNC
    /// let result = client.convert_position(
    ///     "NSE",        // Exchange
    ///     "RELIANCE",   // Symbol
    ///     "BUY",        // Transaction type
    ///     "day",        // Position type
    ///     "10",         // Quantity
    ///     "MIS",        // Old product
    ///     "CNC"         // New product
    /// ).await?;
    /// 
    /// println!("Position converted: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_position(
        &self,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        position_type: &str,
        quantity: &str,
        old_product: &str,
        new_product: &str,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("exchange", exchange);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        params.insert("position_type", position_type);
        params.insert("quantity", quantity);
        params.insert("old_product", old_product);
        params.insert("new_product", new_product);

        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.parse_response(resp).await
    }
}
