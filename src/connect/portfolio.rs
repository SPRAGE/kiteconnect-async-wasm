use anyhow::Result;
use super::{client::KiteConnect, request::RequestHandler};

// Import model types for typed responses
use crate::model::{
    UserProfile, AllMargins, Margins, Holdings, Positions,
};

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
    /// A `Result<AllMargins>` containing margin data for all segments, or a `Result<Margins>` for a specific segment.
    /// The returned data includes:
    /// - `available` - Available margin for trading with breakdown of cash, collateral, etc.
    /// - `used` - Currently utilized margin with SPAN, exposure, M2M details
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
    /// println!("Equity available balance: {}", all_margins.equity.available.live_balance);
    /// 
    /// // Get margins for specific segment - would need separate method for specific segment
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins(&self, segment: Option<String>) -> Result<AllMargins> {
        let url: reqwest::Url = if let Some(segment) = segment {
            self.build_url(&format!("/user/margins/{}", segment), None)
        } else {
            self.build_url("/user/margins", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get user profile details
    /// 
    /// Returns basic user profile information including user ID, name, email, broker details,
    /// enabled exchanges, products, and order types.
    /// 
    /// # Returns
    /// 
    /// A `Result<UserProfile>` containing user profile data with fields like:
    /// - `user_id` - Unique user identifier
    /// - `user_name` - User's real name  
    /// - `email` - User's email address
    /// - `broker` - Broker ID
    /// - `exchanges` - List of enabled exchanges
    /// - `products` - List of enabled product types
    /// - `order_types` - List of enabled order types
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
    /// let profile = client.profile().await?;
    /// println!("User: {} ({})", profile.user_name, profile.user_id);
    /// println!("Broker: {}", profile.broker);
    /// println!("Enabled exchanges: {:?}", profile.exchanges);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn profile(&self) -> Result<UserProfile> {
        let url = self.build_url("/user/profile", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Retrieves the user's holdings (stocks held in demat account)
    /// 
    /// Holdings represent stocks that are held in the user's demat account.
    /// This includes information about quantity, average price, current market value,
    /// profit/loss, and more.
    /// 
    /// # Returns
    /// 
    /// A `Result<Holdings>` containing a vector of holdings with fields like:
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
    /// // Access holdings data directly
    /// for holding in &holdings {
    ///     println!("Symbol: {}, Quantity: {}, P&L: {}", 
    ///         holding.tradingsymbol, holding.quantity, holding.pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings(&self) -> Result<Holdings> {
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Retrieves the user's positions (open positions for the day)
    /// 
    /// Positions represent open trading positions for the current trading day.
    /// This includes both intraday and carry-forward positions with details about
    /// profit/loss, margin requirements, and position status.
    /// 
    /// # Returns
    /// 
    /// A `Result<Positions>` containing positions data with fields like:
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
    /// // Check for open day positions
    /// for position in &positions.day {
    ///     if position.quantity != 0 {
    ///         println!("Open position: {} qty {}", 
    ///             position.tradingsymbol, position.quantity);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions(&self) -> Result<Positions> {
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Retrieve margin requirements for specific trading segments
    /// 
    /// Gets margin requirements and charges for different trading segments
    /// like equity, commodity, currency, etc.
    /// 
    /// # Arguments
    /// 
    /// * `segment` - Trading segment ("equity", "commodity", "currency")
    /// 
    /// # Returns
    /// 
    /// A `Result<Margins>` containing segment-specific margin data with fields like:
    /// - `enabled` - Whether the segment is enabled for the user
    /// - `net` - Net cash balance available for trading
    /// - `available` - Available cash and margin breakdown
    /// - `used` - Utilised margin details
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
    /// let equity_margins = client.instruments_margins("equity").await?;
    /// println!("Equity margins available: {}", equity_margins.available.live_balance);
    /// println!("Equity segment enabled: {}", equity_margins.enabled);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instruments_margins(&self, segment: &str) -> Result<Margins> {
        let url = self.build_url(&format!("/margins/{}", segment), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }
}
