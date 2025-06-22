use anyhow::Result;
use std::collections::HashMap;
use super::{client::KiteConnect, request::RequestHandler};

// Import model types for typed responses
use crate::model::{
    MFOrder, OrderResponse, MFSIP, MFHolding,
};

impl KiteConnect {
    /// Get all mutual fund orders or individual order info
    /// 
    /// Retrieves mutual fund orders with details about order status, settlement, 
    /// fund information, and execution details.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - Optional order ID. If None, returns all orders; if Some, returns specific order
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<MFOrder>>` containing mutual fund order data with fields like:
    /// - `order_id` - Unique order identifier
    /// - `tradingsymbol` - Trading symbol of the mutual fund
    /// - `fund` - Fund name
    /// - `status` - Order status (COMPLETE, CANCELLED, etc.)
    /// - `transaction_type` - BUY or SELL
    /// - `quantity` - Number of units
    /// - `amount` - Order amount
    /// - `average_price` - Execution price per unit
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails
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
    /// // Get all MF orders
    /// let all_orders = client.mf_orders(None).await?;
    /// for order in &all_orders {
    ///     println!("Order {}: {} {} units of {}", 
    ///         order.order_id, order.transaction_type, order.quantity, order.fund);
    /// }
    /// 
    /// // Get specific order
    /// let specific_order = client.mf_orders(Some("123456")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<Vec<MFOrder>> {
        let url: reqwest::Url = if let Some(order_id) = order_id {
            self.build_url(&format!("/mf/orders/{}", order_id), None)
        } else {
            self.build_url("/mf/orders", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Place a mutual fund order
    /// 
    /// Places a mutual fund buy or sell order. For buy orders, you can specify either
    /// quantity (units) or amount (monetary value). For sell orders, quantity is required.
    /// 
    /// # Arguments
    /// 
    /// * `tradingsymbol` - Trading symbol of the mutual fund
    /// * `transaction_type` - "BUY" or "SELL"
    /// * `quantity` - Quantity (units) for the order (optional for buy orders)
    /// * `amount` - Amount in rupees for buy orders (alternative to quantity)
    /// * `tag` - Optional tag to identify orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing order confirmation with order_id
    /// 
    /// # Errors
    /// 
    /// Returns an error if the order placement fails or parameters are invalid
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
    /// // Buy order with amount
    /// let buy_order = client.place_mf_order(
    ///     "INF846K01DP8",    // MF trading symbol
    ///     "BUY",             // Transaction type
    ///     None,              // No quantity
    ///     Some("1000"),      // Amount in rupees
    ///     Some("my_tag")     // Optional tag
    /// ).await?;
    /// 
    /// println!("Order placed with ID: {}", buy_order.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_order(
        &self,
        tradingsymbol: &str,
        transaction_type: &str,
        quantity: Option<&str>,
        amount: Option<&str>,
        tag: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(amount) = amount { params.insert("amount", amount); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url("/mf/orders", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Cancel a mutual fund order
    /// 
    /// Cancels a pending mutual fund order. Only orders in OPEN status can be cancelled.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The mutual fund order ID to cancel
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing cancellation confirmation
    /// 
    /// # Errors
    /// 
    /// Returns an error if the order cannot be cancelled or doesn't exist
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
    /// let result = client.cancel_mf_order("123456789").await?;
    /// println!("Order cancelled: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_order(&self, order_id: &str) -> Result<OrderResponse> {
        let url = self.build_url(&format!("/mf/orders/{}", order_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
        self.parse_response(resp).await
    }

    /// Get mutual fund SIPs (Systematic Investment Plans)
    /// 
    /// Retrieves all active SIPs or details of a specific SIP.
    /// 
    /// # Arguments
    /// 
    /// * `sip_id` - Optional SIP ID. If None, returns all SIPs
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<MFSIP>>` containing SIP information with details like:
    /// - `sip_id` - Unique SIP identifier
    /// - `tradingsymbol` - Fund trading symbol
    /// - `amount` - SIP installment amount
    /// - `frequency` - SIP frequency (monthly, weekly, etc.)
    /// - `status` - SIP status (ACTIVE, PAUSED, CANCELLED)
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails
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
    /// // Get all SIPs
    /// let all_sips = client.mf_sips(None).await?;
    /// 
    /// // Access SIP details directly
    /// for sip in &all_sips {
    ///     println!("SIP: {} - {} ({})", sip.sip_id, sip.tradingsymbol, sip.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips(&self, sip_id: Option<&str>) -> Result<Vec<MFSIP>> {
        let url: reqwest::Url = if let Some(sip_id) = sip_id {
            self.build_url(&format!("/mf/sips/{}", sip_id), None)
        } else {
            self.build_url("/mf/sips", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Place a mutual fund SIP (Systematic Investment Plan)
    /// 
    /// Creates a new SIP for systematic investment in mutual funds.
    /// 
    /// # Arguments
    /// 
    /// * `tradingsymbol` - Trading symbol of the mutual fund
    /// * `amount` - SIP amount per installment
    /// * `instalments` - Total number of installments (max 99 for lifetime)
    /// * `frequency` - SIP frequency ("weekly", "monthly", "quarterly")
    /// * `initial_amount` - Optional initial lump sum amount
    /// * `instalment_day` - Day of month for monthly SIPs (1-28) or day of week for weekly
    /// * `tag` - Optional tag to identify the SIP
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing SIP creation confirmation with SIP ID
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
    /// let sip = client.place_mf_sip(
    ///     "INF846K01DP8",    // MF trading symbol
    ///     "1000",            // Amount per installment
    ///     "12",              // 12 installments
    ///     "monthly",         // Monthly frequency
    ///     Some("5000"),      // Initial amount
    ///     Some("15"),        // 15th of every month
    ///     Some("retirement_sip") // Tag
    /// ).await?;
    /// 
    /// println!("SIP created with ID: {}", sip.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_sip(
        &self,
        tradingsymbol: &str,
        amount: &str,
        instalments: &str,
        frequency: &str,
        initial_amount: Option<&str>,
        instalment_day: Option<&str>,
        tag: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("amount", amount);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);
        
        if let Some(initial_amount) = initial_amount { params.insert("initial_amount", initial_amount); }
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url("/mf/sips", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Modify a mutual fund SIP
    /// 
    /// Modifies an existing SIP's parameters like amount, frequency, or status.
    /// 
    /// # Arguments
    /// 
    /// * `sip_id` - The SIP ID to modify
    /// * `amount` - New SIP amount per installment
    /// * `status` - SIP status ("ACTIVE" or "PAUSED")
    /// * `instalments` - New total number of installments
    /// * `frequency` - New SIP frequency
    /// * `instalment_day` - New day for installments
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing modification confirmation
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
    /// // Increase SIP amount and change frequency
    /// let result = client.modify_mf_sip(
    ///     "123456",          // SIP ID
    ///     "1500",            // New amount
    ///     "ACTIVE",          // Status
    ///     "24",              // New installment count
    ///     "monthly",         // Frequency
    ///     Some("20")         // New instalment day
    /// ).await?;
    /// 
    /// println!("SIP modified with ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn modify_mf_sip(
        &self,
        sip_id: &str,
        amount: &str,
        status: &str,
        instalments: &str,
        frequency: &str,
        instalment_day: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("amount", amount);
        params.insert("status", status);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);
        
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }

        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Cancel a mutual fund SIP
    /// 
    /// Cancels an active SIP. This will stop all future installments.
    /// 
    /// # Arguments
    /// 
    /// * `sip_id` - The SIP ID to cancel
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing cancellation confirmation
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
    /// let result = client.cancel_mf_sip("123456").await?;
    /// println!("SIP cancelled with ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_sip(&self, sip_id: &str) -> Result<OrderResponse> {
        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
        self.parse_response(resp).await
    }

    /// Get mutual fund holdings
    /// 
    /// Retrieves the user's mutual fund holdings with current values and returns.
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<MFHolding>>` containing mutual fund holdings data with fields like:
    /// - `tradingsymbol` - Trading symbol of the mutual fund
    /// - `folio` - Folio number 
    /// - `fund` - Fund name
    /// - `quantity` - Number of units held
    /// - `average_price` - Average purchase price per unit
    /// - `last_price` - Current NAV/price per unit
    /// - `pnl` - Profit/loss amount
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails
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
    /// let holdings = client.mf_holdings().await?;
    /// println!("MF Holdings: {:?}", holdings);
    /// 
    /// // Access holding details directly
    /// for holding in &holdings {
    ///     println!("Fund: {}, Units: {}, Current Value: {}", 
    ///         holding.fund, holding.quantity, holding.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings(&self) -> Result<Vec<MFHolding>> {
        let url = self.build_url("/mf/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }
}
