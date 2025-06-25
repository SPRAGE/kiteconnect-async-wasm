//! # Portfolio Module
//!
//! This module contains portfolio-related methods for the KiteConnect API.

use crate::connect::endpoints::KiteEndpoint;
use anyhow::Result;
use serde_json::Value as JsonValue;
// Import typed models for dual API support
use crate::models::auth::MarginData;
use crate::models::common::KiteResult;
use crate::models::portfolio::{ConversionRequest, Holding, Position};

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===

    /// Retrieves account balance and margin details
    ///
    /// Returns margin information for trading segments including available cash,
    /// used margins, and available margins for different product types.
    ///
    /// # Arguments
    ///
    /// * `segment` - Optional trading segment ("equity" or "commodity"). If None, returns all segments
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing margin data with fields like:
    /// - `available` - Available margin for trading
    /// - `utilised` - Currently utilized margin
    /// - `net` - Net available margin
    /// - `enabled` - Whether the segment is enabled
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
    /// // Get margins for all segments
    /// let all_margins = client.margins(None).await?;
    /// println!("All margins: {:?}", all_margins);
    ///
    /// // Get margins for specific segment
    /// let equity_margins = client.margins(Some("equity".to_string())).await?;
    /// println!("Equity available margin: {}",
    ///     equity_margins["data"]["available"]["live_balance"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins(&self, segment: Option<String>) -> Result<JsonValue> {
        if let Some(segment) = segment {
            let resp = self
                .send_request_with_rate_limiting_and_retry(
                    KiteEndpoint::MarginsSegment,
                    &[&segment],
                    None,
                    None,
                )
                .await
                .map_err(|e| anyhow::anyhow!("Get margins failed: {:?}", e))?;
            self.raise_or_return_json(resp).await
        } else {
            let resp = self
                .send_request_with_rate_limiting_and_retry(KiteEndpoint::Margins, &[], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Get margins failed: {:?}", e))?;
            self.raise_or_return_json(resp).await
        }
    }

    /// Get user profile details
    pub async fn profile(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Profile, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get profile failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieves the user's holdings (stocks held in demat account)
    ///
    /// Holdings represent stocks that are held in the user's demat account.
    /// This includes information about quantity, average price, current market value,
    /// profit/loss, and more.
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing holdings data with fields like:
    /// - `tradingsymbol` - Trading symbol of the instrument
    /// - `quantity` - Total quantity held
    /// - `average_price` - Average buying price
    /// - `last_price` - Current market price
    /// - `pnl` - Profit and loss
    /// - `product` - Product type (CNC, MIS, etc.)
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
    /// let holdings = client.holdings().await?;
    /// println!("Holdings: {:?}", holdings);
    ///
    /// // Access specific fields
    /// if let Some(data) = holdings["data"].as_array() {
    ///     for holding in data {
    ///         println!("Symbol: {}, Quantity: {}",
    ///             holding["tradingsymbol"], holding["quantity"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Holdings, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get holdings failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieves the user's positions (open positions for the day)
    ///
    /// Positions represent open trading positions for the current trading day.
    /// This includes both intraday and carry-forward positions with details about
    /// profit/loss, margin requirements, and position status.
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing positions data with fields like:
    /// - `tradingsymbol` - Trading symbol of the instrument
    /// - `quantity` - Net position quantity
    /// - `buy_quantity` - Total buy quantity
    /// - `sell_quantity` - Total sell quantity
    /// - `average_price` - Average position price
    /// - `pnl` - Realized and unrealized P&L
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
    /// let positions = client.positions().await?;
    /// println!("Positions: {:?}", positions);
    ///
    /// // Check for open positions
    /// if let Some(day_positions) = positions["data"]["day"].as_array() {
    ///     for position in day_positions {
    ///         if position["quantity"].as_i64().unwrap_or(0) != 0 {
    ///             println!("Open position: {} qty {}",
    ///                 position["tradingsymbol"], position["quantity"]);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Positions, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get positions failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===

    /// Get user margins with typed response
    ///
    /// Returns strongly typed margin data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `segment` - Optional trading segment ("equity" or "commodity")
    ///
    /// # Returns
    ///
    /// A `KiteResult<MarginData>` containing typed margin information
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
    /// let margins = client.margins_typed(None).await?;
    /// println!("Available equity margin: {}", margins.equity.unwrap().available.cash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins_typed(&self, segment: Option<&str>) -> KiteResult<MarginData> {
        if let Some(segment) = segment {
            let resp = self
                .send_request_with_rate_limiting_and_retry(
                    KiteEndpoint::MarginsSegment,
                    &[segment],
                    None,
                    None,
                )
                .await?;
            let json_response = self.raise_or_return_json_typed(resp).await?;
            self.parse_response(json_response)
        } else {
            let resp = self
                .send_request_with_rate_limiting_and_retry(KiteEndpoint::Margins, &[], None, None)
                .await?;
            let json_response = self.raise_or_return_json_typed(resp).await?;
            self.parse_response(json_response)
        }
    }

    /// Get user holdings with typed response
    ///
    /// Returns a vector of strongly typed holding objects instead of JsonValue.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Holding>>` containing typed holdings data
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
    /// let holdings = client.holdings_typed().await?;
    /// for holding in holdings {
    ///     println!("Symbol: {}, Quantity: {}, P&L: {}",
    ///         holding.trading_symbol, holding.quantity, holding.pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings_typed(&self) -> KiteResult<Vec<Holding>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Holdings, &[], None, None)
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the "data" field and parse as Vec<Holding>
        if let Some(data) = json_response.get("data") {
            self.parse_response(data.clone())
        } else {
            // If no "data" field, try parsing the entire response
            self.parse_response(json_response)
        }
    }

    /// Get user positions with typed response
    ///
    /// Returns structured position data instead of JsonValue.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Position>>` containing typed positions data
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
    /// let positions = client.positions_typed().await?;
    /// for position in &positions {
    ///     if position.quantity != 0 {
    ///         println!("Open position: {} qty {}",
    ///             position.trading_symbol, position.quantity);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions_typed(&self) -> KiteResult<Vec<Position>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Positions, &[], None, None)
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // KiteConnect returns positions in nested structure: { "data": { "day": [...], "net": [...] } }
        // We'll flatten both day and net positions into a single vector
        let mut all_positions = Vec::new();

        if let Some(data) = json_response.get("data") {
            if let Some(day_positions) = data.get("day").and_then(|v| v.as_array()) {
                for pos_json in day_positions {
                    if let Ok(position) = self.parse_response::<Position>(pos_json.clone()) {
                        all_positions.push(position);
                    }
                }
            }

            if let Some(net_positions) = data.get("net").and_then(|v| v.as_array()) {
                for pos_json in net_positions {
                    if let Ok(position) = self.parse_response::<Position>(pos_json.clone()) {
                        all_positions.push(position);
                    }
                }
            }
        }

        Ok(all_positions)
    }

    /// Convert positions between product types (typed)
    ///
    /// Converts a position from one product type to another (e.g., MIS to CNC).
    ///
    /// # Arguments
    ///
    /// * `request` - Conversion request details
    ///
    /// # Returns
    ///
    /// A `KiteResult<bool>` indicating success
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::portfolio::ConversionRequest;
    /// use kiteconnect_async_wasm::models::common::{Exchange, Product, TransactionType};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let conversion = ConversionRequest {
    ///     exchange: Exchange::NSE,
    ///     trading_symbol: "RELIANCE".to_string(),
    ///     transaction_type: TransactionType::BUY,
    ///     quantity: 10,
    ///     from_product: Product::MIS,
    ///     to_product: Product::CNC,
    /// };
    ///
    /// let success = client.convert_position_typed(&conversion).await?;
    /// println!("Conversion successful: {}", success);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_position_typed(&self, request: &ConversionRequest) -> KiteResult<bool> {
        let mut params = std::collections::HashMap::new();
        let exchange_str = request.exchange.to_string();
        let transaction_str = request.transaction_type.to_string();
        let quantity_str = request.quantity.to_string();
        let from_product_str = request.from_product.to_string();
        let to_product_str = request.to_product.to_string();

        params.insert("exchange", exchange_str.as_str());
        params.insert("tradingsymbol", request.trading_symbol.as_str());
        params.insert("transaction_type", transaction_str.as_str());
        params.insert("quantity", quantity_str.as_str());
        params.insert("old_product", from_product_str.as_str());
        params.insert("new_product", to_product_str.as_str());

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ConvertPosition,
                &[],
                None,
                Some(params),
            )
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Check if conversion was successful
        Ok(json_response.get("status").and_then(|v| v.as_str()) == Some("success"))
    }
}
