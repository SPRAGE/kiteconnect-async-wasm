//! # Mutual Funds Module
//! 
//! This module contains mutual fund operations for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use std::collections::HashMap;
use crate::connect::endpoints::KiteEndpoint;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::mutual_funds::{MFOrder, MFOrderParams, MFOrderResponse, SIP, SIPParams, SIPResponse, MFHolding};

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===
    
    /// Get all mutual fund orders or individual order info
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
        let resp = if let Some(order_id) = order_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrderInfo,
                &[order_id],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get MF order info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrders,
                &[],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get MF orders: {}", e))?
        };

        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing order confirmation with order_id
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
    /// println!("Order placed: {:?}", buy_order);
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
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(amount) = amount { params.insert("amount", amount); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::PlaceMFOrder,
            &[],
            None,
            Some(params),
        ).await.map_err(|e| anyhow::anyhow!("Failed to place MF order: {}", e))?;
        
        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing cancellation confirmation
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
    /// println!("Cancellation result: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_order(&self, order_id: &str) -> Result<JsonValue> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::CancelMFOrder,
            &[order_id],
            None,
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to cancel MF order: {}", e))?;
        
        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing SIP information
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
    /// // Get specific SIP
    /// let sip_details = client.mf_sips(Some("123456")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips(&self, sip_id: Option<&str>) -> Result<JsonValue> {
        let resp = if let Some(sip_id) = sip_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPInfo,
                &[sip_id],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get SIP info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPs,
                &[],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get SIPs: {}", e))?
        };

        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing SIP creation confirmation
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
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("amount", amount);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);
        
        if let Some(initial_amount) = initial_amount { params.insert("initial_amount", initial_amount); }
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::PlaceSIP,
            &[],
            None,
            Some(params),
        ).await.map_err(|e| anyhow::anyhow!("Failed to place MF SIP: {}", e))?;
        
        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing modification confirmation
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
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("amount", amount);
        params.insert("status", status);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);
        
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::ModifySIP,
            &[sip_id],
            None,
            Some(params),
        ).await.map_err(|e| anyhow::anyhow!("Failed to modify MF SIP: {}", e))?;
        
        self.raise_or_return_json(resp).await
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
    /// A `Result<JsonValue>` containing cancellation confirmation
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
    /// println!("SIP cancelled: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_sip(&self, sip_id: &str) -> Result<JsonValue> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::CancelSIP,
            &[sip_id],
            None,
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to cancel MF SIP: {}", e))?;
        
        self.raise_or_return_json(resp).await
    }

    /// Get mutual fund holdings
    /// 
    /// Retrieves the user's mutual fund holdings with current values and returns.
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing mutual fund holdings data
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
    /// // Access specific fields
    /// if let Some(data) = holdings["data"].as_array() {
    ///     for holding in data {
    ///         println!("Fund: {}, Units: {}, Current Value: {}", 
    ///             holding["tradingsymbol"], 
    ///             holding["quantity"], 
    ///             holding["last_price"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings(&self) -> Result<JsonValue> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MFHoldings,
            &[],
            None,
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to get MF holdings: {}", e))?;
        
        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===
    
    /// Get mutual fund orders with typed response
    /// 
    /// Returns strongly typed MF order data instead of JsonValue.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - Optional order ID. If None, returns all orders
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<MFOrder>>` for all orders or `KiteResult<MFOrder>` for specific order
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
    /// let all_orders = client.mf_orders_typed(None).await?;
    /// for order in all_orders {
    ///     println!("Order ID: {}, Status: {:?}", order.order_id, order.status);
    /// }
    /// 
    /// // Get specific order
    /// let specific_order = client.mf_order_typed("123456").await?;
    /// println!("Order: {:?}", specific_order);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_orders_typed(&self, order_id: Option<&str>) -> KiteResult<Vec<MFOrder>> {
        let resp = if let Some(order_id) = order_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrderInfo,
                &[order_id],
                None,
                None,
            ).await?
        } else {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrders,
                &[],
                None,
                None,
            ).await?
        };

        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get single mutual fund order with typed response
    /// 
    /// Returns a single strongly typed MF order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to fetch
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<MFOrder>` containing the typed order data
    pub async fn mf_order_typed(&self, order_id: &str) -> KiteResult<MFOrder> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MFOrderInfo,
            &[order_id],
            None,
            None,
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Place a mutual fund order with typed response
    /// 
    /// Places a mutual fund order and returns typed response.
    /// 
    /// # Arguments
    /// 
    /// * `order_params` - Typed order parameters
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<MFOrderResponse>` containing order confirmation
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::mutual_funds::MFOrderParams;
    /// use kiteconnect_async_wasm::models::common::TransactionType;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let order_params = MFOrderParams {
    ///     trading_symbol: "INF846K01DP8".to_string(),
    ///     transaction_type: TransactionType::BUY,
    ///     amount: Some(1000.0),
    ///     quantity: None,
    ///     tag: Some("my_tag".to_string()),
    /// };
    /// 
    /// let response = client.place_mf_order_typed(&order_params).await?;
    /// println!("Order placed with ID: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_order_typed(&self, order_params: &MFOrderParams) -> KiteResult<MFOrderResponse> {
        // Create all string conversions upfront to avoid lifetime issues
        let transaction_type_str = order_params.transaction_type.to_string();
        let amount_str = order_params.amount.map(|a| a.to_string());
        let quantity_str = order_params.quantity.map(|q| q.to_string());
        
        let mut params = HashMap::new();
        params.insert("tradingsymbol", order_params.trading_symbol.as_str());
        params.insert("transaction_type", transaction_type_str.as_str());
        
        if let Some(ref amount) = amount_str {
            params.insert("amount", amount.as_str());
        }
        if let Some(ref quantity) = quantity_str {
            params.insert("quantity", quantity.as_str());
        }
        if let Some(ref tag) = order_params.tag {
            params.insert("tag", tag.as_str());
        }

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::PlaceMFOrder,
            &[],
            None,
            Some(params),
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get mutual fund SIPs with typed response
    /// 
    /// Returns strongly typed SIP data instead of JsonValue.
    /// 
    /// # Arguments
    /// 
    /// * `sip_id` - Optional SIP ID. If None, returns all SIPs
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<SIP>>` for all SIPs or `KiteResult<SIP>` for specific SIP
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
    /// let all_sips = client.mf_sips_typed(None).await?;
    /// for sip in all_sips {
    ///     println!("SIP ID: {}, Status: {:?}", sip.sip_id, sip.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips_typed(&self, sip_id: Option<&str>) -> KiteResult<Vec<SIP>> {
        let resp = if let Some(sip_id) = sip_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPInfo,
                &[sip_id],
                None,
                None,
            ).await?
        } else {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPs,
                &[],
                None,
                None,
            ).await?
        };

        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get single mutual fund SIP with typed response
    /// 
    /// Returns a single strongly typed SIP.
    /// 
    /// # Arguments
    /// 
    /// * `sip_id` - The SIP ID to fetch
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<SIP>` containing the typed SIP data
    pub async fn mf_sip_typed(&self, sip_id: &str) -> KiteResult<SIP> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::SIPInfo,
            &[sip_id],
            None,
            None,
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Place a mutual fund SIP with typed response
    /// 
    /// Creates a new SIP and returns typed response.
    /// 
    /// # Arguments
    /// 
    /// * `sip_params` - Typed SIP parameters
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<SIPResponse>` containing SIP creation confirmation
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::mutual_funds::{SIPParams, SIPFrequency};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let sip_params = SIPParams {
    ///     trading_symbol: "INF846K01DP8".to_string(),
    ///     amount: 1000.0,
    ///     instalments: Some(12),
    ///     frequency: SIPFrequency::Monthly,
    ///     initial_amount: Some(5000.0),
    ///     tag: Some("retirement_sip".to_string()),
    /// };
    /// 
    /// let response = client.place_mf_sip_typed(&sip_params).await?;
    /// println!("SIP created with ID: {}", response.sip_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_sip_typed(&self, sip_params: &SIPParams) -> KiteResult<SIPResponse> {
        // Create all string conversions upfront to avoid lifetime issues
        let amount_str = sip_params.amount.to_string();
        let instalments_str = sip_params.instalments.map(|i| i.to_string());
        let frequency_str = sip_params.frequency.to_string(); // Convert enum to string using Display trait
        let initial_amount_str = sip_params.initial_amount.map(|a| a.to_string());
        
        let mut params = HashMap::new();
        params.insert("tradingsymbol", sip_params.trading_symbol.as_str());
        params.insert("amount", amount_str.as_str());
        params.insert("frequency", frequency_str.as_str());
        
        if let Some(ref instalments) = instalments_str {
            params.insert("instalments", instalments.as_str());
        }
        if let Some(ref initial_amount) = initial_amount_str {
            params.insert("initial_amount", initial_amount.as_str());
        }
        if let Some(ref tag) = sip_params.tag {
            params.insert("tag", tag.as_str());
        }

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::PlaceSIP,
            &[],
            None,
            Some(params),
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get mutual fund holdings with typed response
    /// 
    /// Returns strongly typed MF holdings data instead of JsonValue.
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<MFHolding>>` containing typed holdings data
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
    /// let holdings = client.mf_holdings_typed().await?;
    /// for holding in holdings {
    ///     println!("Fund: {}, Units: {}, Current Value: {}", 
    ///         holding.trading_symbol, holding.quantity, holding.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings_typed(&self) -> KiteResult<Vec<MFHolding>> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::MFHoldings,
            &[],
            None,
            None,
        ).await?;
        
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
