//! # Portfolio Module
//! 
//! This module contains portfolio-related methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use crate::connect::utils::RequestHandler;

use crate::connect::KiteConnect;

impl KiteConnect {
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
        let url: reqwest::Url = if let Some(segment) = segment {
            self.build_url(&format!("/user/margins/{}", segment), None)
        } else {
            self.build_url("/user/margins", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get user profile details
    pub async fn profile(&self) -> Result<JsonValue> {
        let url = self.build_url("/user/profile", None);
        let resp = self.send_request(url, "GET", None).await?;
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
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
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
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }
}
