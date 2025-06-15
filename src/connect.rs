use serde_json::Value as JsonValue;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};

// Conditional imports for different targets
#[cfg(not(target_arch = "wasm32"))]
use {csv::ReaderBuilder, sha2::{Sha256, Digest}};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::Uint8Array,
    wasm_bindgen_futures::JsFuture,
    web_sys::window,
};

#[cfg(not(test))]
const URL: &str = "https://api.kite.trade";

#[cfg(test)]
const URL: &str = "http://127.0.0.1:1234";

/// Async trait for handling HTTP requests
trait RequestHandler {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response>;
}

/// Main KiteConnect struct for interacting with Kite Connect API
#[derive(Clone, Debug)]
pub struct KiteConnect {
    api_key: String,
    access_token: String,
    session_expiry_hook: Option<fn() -> ()>,
    client: reqwest::Client,
}

impl Default for KiteConnect {
    fn default() -> Self {
        KiteConnect {
            api_key: "<API-KEY>".to_string(),
            access_token: "<ACCESS-TOKEN>".to_string(),
            session_expiry_hook: None,
            client: reqwest::Client::new(),
        }
    }
}

impl KiteConnect {
    /// Constructs url for the given path and query params
    pub(crate) fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
        let url: &str = &format!("{}/{}", URL, &path[1..]);
        let mut url = reqwest::Url::parse(url).unwrap();

        if let Some(data) = param {
            url.query_pairs_mut().extend_pairs(data.iter());
        }
        url
    }

    /// Constructor
    pub fn new(api_key: &str, access_token: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            access_token: access_token.to_string(),
            client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    /// Helper method to raise or return json response for async responses
    async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Sets an expiry hook method for this instance
    pub fn set_session_expiry_hook(&mut self, method: fn() -> ()) {
        self.session_expiry_hook = Some(method);
    }

    /// Gets the session expiry hook
    pub fn session_expiry_hook(&self) -> Option<fn() -> ()> {
        self.session_expiry_hook
    }

    /// Sets an access token for this instance
    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }

    /// Gets the access token for this instance
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Returns the login url
    pub fn login_url(&self) -> String {
        format!("https://kite.trade/connect/login?api_key={}&v3", self.api_key)
    }

    /// Compute checksum for authentication - different implementations for native vs WASM
    #[cfg(not(target_arch = "wasm32"))]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    #[cfg(target_arch = "wasm32")]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        // WASM implementation using Web Crypto API
        let window = window().ok_or_else(|| anyhow!("No window object"))?;
        let crypto = window.crypto().map_err(|_| anyhow!("No crypto object"))?;
        let subtle = crypto.subtle();

        let data = Uint8Array::from(input.as_bytes());
        let digest_promise = subtle
            .digest_with_str_and_u8_array("SHA-256", &data.to_vec())
            .map_err(|_| anyhow!("Failed to create digest"))?;

        let digest_result = JsFuture::from(digest_promise)
            .await
            .map_err(|_| anyhow!("Failed to compute hash"))?;

        let digest_array = Uint8Array::new(&digest_result);
        let digest_vec: Vec<u8> = digest_array.to_vec();
        Ok(hex::encode(digest_vec))
    }

    /// Request for access token
    pub async fn generate_session(
        &mut self,
        request_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        // Create a hex digest from api key, request token, api secret
        let input = format!("{}{}{}", self.api_key, request_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let api_key: &str = &self.api_key.clone();
        let mut data = HashMap::new();
        data.insert("api_key", api_key);
        data.insert("request_token", request_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/token", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["data"]["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the access token
    pub async fn invalidate_access_token(&self, access_token: &str) -> Result<reqwest::Response> {
        let url = self.build_url("/session/token", None);
        let mut data = HashMap::new();
        data.insert("access_token", access_token);

        self.send_request(url, "DELETE", Some(data)).await
    }

    /// Request for new access token
    pub async fn renew_access_token(
        &mut self,
        access_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        // Create a hex digest from api key, request token, api secret
        let input = format!("{}{}{}", self.api_key, access_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let api_key: &str = &self.api_key.clone();
        let mut data = HashMap::new();
        data.insert("api_key", api_key);
        data.insert("access_token", access_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/refresh_token", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the refresh token
    pub async fn invalidate_refresh_token(&self, refresh_token: &str) -> Result<reqwest::Response> {
        let url = self.build_url("/session/refresh_token", None);
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);

        self.send_request(url, "DELETE", Some(data)).await
    }

    /// Return the account balance and cash margin details for a particular segment
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

    /// Get all holdings
    pub async fn holdings(&self) -> Result<JsonValue> {
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all positions
    pub async fn positions(&self) -> Result<JsonValue> {
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Place an order
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
    ) -> Result<JsonValue> {
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
        self.raise_or_return_json(resp).await
    }

    /// Modify an open order
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
    ) -> Result<JsonValue> {
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
        self.raise_or_return_json(resp).await
    }

    /// Cancel an order
    pub async fn cancel_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        if let Some(parent_order_id) = parent_order_id {
            params.insert("parent_order_id", parent_order_id);
        }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "DELETE", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Exit a BO/CO order
    pub async fn exit_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<JsonValue> {
        self.cancel_order(order_id, variety, parent_order_id).await
    }

    /// Get a list of orders
    pub async fn orders(&self) -> Result<JsonValue> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get the list of order history
    pub async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
        let params = vec![("order_id", order_id)];
        let url = self.build_url("/orders", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades
    pub async fn trades(&self) -> Result<JsonValue> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades for a specific order
    pub async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Modify an open position product type
    pub async fn convert_position(
        &self,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        position_type: &str,
        quantity: &str,
        old_product: &str,
        new_product: &str,
    ) -> Result<JsonValue> {
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
        self.raise_or_return_json(resp).await
    }

    /// Get all mutual fund orders or individual order info
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(order_id) = order_id {
            self.build_url(&format!("/mf/orders/{}", order_id), None)
        } else {
            self.build_url("/mf/orders", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get the trigger range for a list of instruments
    pub async fn trigger_range(
        &self,
        transaction_type: &str,
        instruments: Vec<&str>,
    ) -> Result<JsonValue> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("transaction_type", transaction_type));
        
        for instrument in instruments {
            params.push(("instruments", instrument));
        }

        let url = self.build_url("/instruments/trigger_range", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get instruments list
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();
            
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }
        
        Ok(JsonValue::Array(result))
    }

    /// Get instruments list (WASM version - returns raw CSV as string)
    #[cfg(target_arch = "wasm32")]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // For WASM, return the raw CSV data as a string
        // Users can parse it client-side using JS CSV libraries
        Ok(JsonValue::String(body))
    }

    /// Get mutual fund instruments list
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();
            
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }
        
        Ok(JsonValue::Array(result))
    }

    /// Get mutual fund instruments list (WASM version - returns raw CSV as string)
    #[cfg(target_arch = "wasm32")]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // For WASM, return the raw CSV data as a string
        // Users can parse it client-side using JS CSV libraries
        Ok(JsonValue::String(body))
    }
}

/// Implement the async request handler for KiteConnect struct
impl RequestHandler for KiteConnect {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response> {
        let mut headers = HeaderMap::new();
        headers.insert("XKiteVersion", "3".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("token {}:{}", self.api_key, self.access_token)
                .parse()
                .unwrap(),
        );
        headers.insert(USER_AGENT, "Rust".parse().unwrap());

        let response = match method {
            "GET" => self.client.get(url).headers(headers).send().await?,
            "POST" => self.client.post(url).headers(headers).form(&data).send().await?,
            "DELETE" => self.client.delete(url).headers(headers).json(&data).send().await?,
            "PUT" => self.client.put(url).headers(headers).form(&data).send().await?,
            _ => return Err(anyhow!("Unknown method!")),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

    #[tokio::test]
    async fn test_build_url() {
        let kiteconnect = KiteConnect::new("key", "token");
        let url = kiteconnect.build_url("/my-holdings", None);
        assert_eq!(url.as_str(), format!("{}/my-holdings", URL).as_str());

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("one", "1"));
        let url = kiteconnect.build_url("/my-holdings", Some(params));
        assert_eq!(url.as_str(), format!("{}/my-holdings?one=1", URL).as_str());
    }

    #[tokio::test]
    async fn test_set_access_token() {
        let mut kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.access_token(), "token");
        kiteconnect.set_access_token("my_token");
        assert_eq!(kiteconnect.access_token(), "my_token");
    }

    #[tokio::test]
    async fn test_session_expiry_hook() {
        let mut kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.session_expiry_hook(), None);

        fn mock_hook() { 
            println!("Session expired");
        }

        kiteconnect.set_session_expiry_hook(mock_hook);
        assert_ne!(kiteconnect.session_expiry_hook(), None);
    }

    #[tokio::test]
    async fn test_login_url() {
        let kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.login_url(), "https://kite.trade/connect/login?api_key=key&v3");
    }

    #[tokio::test]
    async fn test_margins() {
        // Create a new mock server
        let mut server = Server::new_async().await;
        
        // Create KiteConnect instance that uses the mock server URL
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock("GET", Matcher::Regex(r"^/user/margins".to_string()))
            .with_body_from_file("mocks/margins.json")
            .create_async()
            .await;
        let _mock2 = server.mock("GET", Matcher::Regex(r"^/user/margins/commodity".to_string()))
            .with_body_from_file("mocks/margins.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.margins(None).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
        let data: JsonValue = kiteconnect.margins(Some("commodity".to_string())).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_holdings() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock = server.mock("GET", Matcher::Regex(r"^/portfolio/holdings".to_string()))
            .with_body_from_file("mocks/holdings.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.holdings().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_positions() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock = server.mock("GET", Matcher::Regex(r"^/portfolio/positions".to_string()))
            .with_body_from_file("mocks/positions.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.positions().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_order_trades() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders/171229000724687/trades".to_string())
        )
        .with_body_from_file("mocks/order_trades.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.order_trades("171229000724687").await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_orders() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders".to_string())
        )
        .with_body_from_file("mocks/orders.json")
        .with_status(200)
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.orders().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_order_history() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders".to_string())
        )
        .with_body_from_file("mocks/order_info.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.order_history("171229000724687").await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_trades() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock("GET", Matcher::Regex(r"^/trades".to_string()))
            .with_body_from_file("mocks/trades.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.trades().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_mf_orders() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock(
            "GET", Matcher::Regex(r"^/mf/orders$".to_string())
        )
        .with_body_from_file("mocks/mf_orders.json")
        .create_async()
        .await;

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/mf/orders".to_string())
        )
        .with_body_from_file("mocks/mf_orders_info.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.mf_orders(None).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
        let data: JsonValue = kiteconnect.mf_orders(Some("171229000724687")).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_trigger_range() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/instruments/trigger_range".to_string())
        )
        .with_body_from_file("mocks/trigger_range.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.trigger_range("BUY", vec!["NSE:INFY", "NSE:RELIANCE"]).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_instruments() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/instruments".to_string())
        )
        .with_body_from_file("mocks/instruments.csv")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.instruments(None).await.unwrap();
        println!("{:?}", data);
        assert_eq!(data[0]["instrument_token"].as_str(), Some("408065"));
    }

    #[tokio::test]
    async fn test_mf_instruments() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/mf/instruments".to_string())
        )
        .with_body_from_file("mocks/mf_instruments.csv")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.mf_instruments().await.unwrap();
        println!("{:?}", data);
        assert_eq!(data[0]["tradingsymbol"].as_str(), Some("INF846K01DP8"));
    }

    // Helper struct to override the URL for testing
    #[derive(Clone, Debug)]
    struct TestKiteConnect {
        api_key: String,
        access_token: String,
        client: reqwest::Client,
        base_url: String,
    }

    impl TestKiteConnect {
        fn new(api_key: &str, access_token: &str, base_url: &str) -> Self {
            Self {
                api_key: api_key.to_string(),
                access_token: access_token.to_string(),
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            }
        }

        fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
            let url: &str = &format!("{}/{}", self.base_url, &path[1..]);
            let mut url = reqwest::Url::parse(url).unwrap();

            if let Some(data) = param {
                url.query_pairs_mut().extend_pairs(data.iter());
            }
            url
        }

        async fn send_request(
            &self,
            url: reqwest::Url,
            method: &str,
            data: Option<HashMap<&str, &str>>,
        ) -> Result<reqwest::Response> {
            let mut headers = HeaderMap::new();
            headers.insert("XKiteVersion", "3".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("token {}:{}", self.api_key, self.access_token)
                    .parse()
                    .unwrap(),
            );
            headers.insert(USER_AGENT, "Rust".parse().unwrap());

            let response = match method {
                "GET" => self.client.get(url).headers(headers).send().await?,
                "POST" => self.client.post(url).headers(headers).form(&data).send().await?,
                "DELETE" => self.client.delete(url).headers(headers).json(&data).send().await?,
                "PUT" => self.client.put(url).headers(headers).form(&data).send().await?,
                _ => return Err(anyhow!("Unknown method!")),
            };

            Ok(response)
        }

        async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
            if resp.status().is_success() {
                let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
                Ok(jsn)
            } else {
                let error_text = resp.text().await?;
                Err(anyhow!(error_text))
            }
        }

        async fn holdings(&self) -> Result<JsonValue> {
            let url = self.build_url("/portfolio/holdings", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn positions(&self) -> Result<JsonValue> {
            let url = self.build_url("/portfolio/positions", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn orders(&self) -> Result<JsonValue> {
            let url = self.build_url("/orders", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn margins(&self, segment: Option<String>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(segment) = segment {
                self.build_url(&format!("/user/margins/{}", segment), None)
            } else {
                self.build_url("/user/margins", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
            let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
            let params = vec![("order_id", order_id)];
            let url = self.build_url("/orders", Some(params));
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn trades(&self) -> Result<JsonValue> {
            let url = self.build_url("/trades", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(order_id) = order_id {
                self.build_url(&format!("/mf/orders/{}", order_id), None)
            } else {
                self.build_url("/mf/orders", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn trigger_range(
            &self,
            transaction_type: &str,
            instruments: Vec<&str>,
        ) -> Result<JsonValue> {
            let mut params: Vec<(&str, &str)> = Vec::new();
            params.push(("transaction_type", transaction_type));
            
            for instrument in instruments {
                params.push(("instruments", instrument));
            }

            let url = self.build_url("/instruments/trigger_range", Some(params));
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(exchange) = exchange {
                self.build_url(&format!("/instruments/{}", exchange), None)
            } else {
                self.build_url("/instruments", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            let body = resp.text().await?;
            
            // Parse CSV response
            #[cfg(not(target_arch = "wasm32"))]
            {
                use csv::ReaderBuilder;
                let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
                let mut result = Vec::new();
                
                let headers = rdr.headers()?.clone();
                for record in rdr.records() {
                    let record = record?;
                    let mut obj = serde_json::Map::new();
                    
                    for (i, field) in record.iter().enumerate() {
                        if let Some(header) = headers.get(i) {
                            obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                        }
                    }
                    result.push(JsonValue::Object(obj));
                }
                
                Ok(JsonValue::Array(result))
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                Ok(JsonValue::String(body))
            }
        }

        async fn mf_instruments(&self) -> Result<JsonValue> {
            let url = self.build_url("/mf/instruments", None);
            let resp = self.send_request(url, "GET", None).await?;
            let body = resp.text().await?;
            
            // Parse CSV response
            #[cfg(not(target_arch = "wasm32"))]
            {
                use csv::ReaderBuilder;
                let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
                let mut result = Vec::new();
                
                let headers = rdr.headers()?.clone();
                for record in rdr.records() {
                    let record = record?;
                    let mut obj = serde_json::Map::new();
                    
                    for (i, field) in record.iter().enumerate() {
                        if let Some(header) = headers.get(i) {
                            obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                        }
                    }
                    result.push(JsonValue::Object(obj));
                }
                
                Ok(JsonValue::Array(result))
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                Ok(JsonValue::String(body))
            }
        }
    }
}
