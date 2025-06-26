// Example of refactored async connect.rs

use anyhow::{anyhow, Context, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use url::Url;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
use sha2::{Digest, Sha256};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, SubtleCrypto};

#[cfg(not(test))]
const URL: &str = "https://api.kite.trade";

#[cfg(test)]
const URL: &str = "http://localhost:1234"; // Mock server URL

#[async_trait::async_trait]
trait RequestHandler {
    async fn send_request(
        &self,
        url: Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response>;
}

pub struct KiteConnect {
    api_key: String,
    access_token: String,
    #[allow(dead_code)]
    session_expiry_hook: Option<fn() -> ()>,
    #[cfg(not(target_arch = "wasm32"))]
    client: reqwest::Client,
    #[cfg(target_arch = "wasm32")]
    client: reqwest::Client,
}

impl KiteConnect {
    /// Constructor
    pub fn new(api_key: &str, access_token: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            access_token: access_token.to_string(),
            session_expiry_hook: None,
            client: reqwest::Client::new(),
        }
    }

    /// Constructs url for the given path and query params
    fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> Url {
        let url_str = format!("{}{}", URL, path);
        let mut url = Url::parse(&url_str).unwrap();

        if let Some(data) = param {
            url.query_pairs_mut().extend_pairs(data.iter());
        }
        url
    }

    /// Sets an access token for this instance
    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }

    /// Returns the login url
    pub fn login_url(&self) -> String {
        format!(
            "https://kite.trade/connect/login?api_key={}&v3",
            self.api_key
        )
    }

    /// Async SHA256 hash computation (platform-specific)
    #[cfg(not(target_arch = "wasm32"))]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        // Native implementation using sha2
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    #[cfg(target_arch = "wasm32")]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        // WASM implementation using Web Crypto API
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;

        let window = window().ok_or_else(|| anyhow!("No window object"))?;
        let crypto = window.crypto().map_err(|_| anyhow!("No crypto object"))?;
        let subtle = crypto.subtle();

        let data = Uint8Array::from(input.as_bytes());
        let digest_promise = subtle
            .digest_with_str_and_u8_array("SHA-256", &data)
            .map_err(|_| anyhow!("Failed to create digest"))?;

        let digest_result = JsFuture::from(digest_promise)
            .await
            .map_err(|_| anyhow!("Failed to compute hash"))?;

        let digest_array = Uint8Array::new(&digest_result);
        let digest_vec: Vec<u8> = digest_array.to_vec();

        Ok(hex::encode(digest_vec))
    }

    /// Request for access token (now async)
    pub async fn generate_session(
        &mut self,
        request_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        // Create a hex digest from api key, request token, api secret
        let input = format!("{}{}{}", self.api_key, request_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let mut data = HashMap::new();
        data.insert("api_key", self.api_key.as_str());
        data.insert("request_token", request_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/token", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            if let Some(access_token) = jsn["access_token"].as_str() {
                self.set_access_token(access_token);
            }
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Get all holdings (now async)
    pub async fn holdings(&self) -> Result<JsonValue> {
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all positions (now async)
    pub async fn positions(&self) -> Result<JsonValue> {
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get a list of orders (now async)
    pub async fn orders(&self) -> Result<JsonValue> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Place an order (now async)
    #[allow(clippy::too_many_arguments)]
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
        params.insert("exchange", exchange);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        params.insert("quantity", quantity);

        if let Some(p) = product {
            params.insert("product", p);
        }
        if let Some(ot) = order_type {
            params.insert("order_type", ot);
        }
        if let Some(pr) = price {
            params.insert("price", pr);
        }
        if let Some(v) = validity {
            params.insert("validity", v);
        }
        if let Some(dq) = disclosed_quantity {
            params.insert("disclosed_quantity", dq);
        }
        if let Some(tp) = trigger_price {
            params.insert("trigger_price", tp);
        }
        if let Some(so) = squareoff {
            params.insert("squareoff", so);
        }
        if let Some(sl) = stoploss {
            params.insert("stoploss", sl);
        }
        if let Some(tsl) = trailing_stoploss {
            params.insert("trailing_stoploss", tsl);
        }
        if let Some(t) = tag {
            params.insert("tag", t);
        }

        let url = self.build_url(&format!("/orders/{}", variety), None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Helper method to raise or return json response
    async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }
}

#[async_trait::async_trait]
impl RequestHandler for KiteConnect {
    async fn send_request(
        &self,
        url: Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response> {
        let mut request = match method {
            "GET" => self.client.get(url),
            "POST" => {
                let mut req = self.client.post(url);
                if let Some(form_data) = data {
                    req = req.form(&form_data);
                }
                req
            }
            "PUT" => {
                let mut req = self.client.put(url);
                if let Some(form_data) = data {
                    req = req.form(&form_data);
                }
                req
            }
            "DELETE" => {
                let mut req = self.client.delete(url);
                if let Some(json_data) = data {
                    req = req.json(&json_data);
                }
                req
            }
            _ => return Err(anyhow!("Unknown HTTP method: {}", method)),
        };

        // Add headers
        request = request
            .header("X-Kite-Version", "3")
            .header(
                "Authorization",
                format!("token {}:{}", self.api_key, self.access_token),
            )
            .header("User-Agent", "Rust");

        let response = request.send().await?;
        Ok(response)
    }
}

// Usage example for both native and WASM
#[cfg(not(target_arch = "wasm32"))]
pub async fn example_usage() -> Result<()> {
    let mut kite = KiteConnect::new("your_api_key", "");

    // Generate session
    let session = kite.generate_session("request_token", "api_secret").await?;
    println!("Session: {:?}", session);

    // Get holdings
    let holdings = kite.holdings().await?;
    println!("Holdings: {:?}", holdings);

    // Place an order
    let order = kite
        .place_order(
            "regular",
            "NSE",
            "INFY",
            "BUY",
            "1",
            Some("CNC"),
            Some("LIMIT"),
            Some("1500.00"),
            Some("DAY"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    println!("Order placed: {:?}", order);

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub async fn example_usage_wasm() -> Result<()> {
    use wasm_bindgen_futures::spawn_local;

    spawn_local(async {
        let mut kite = KiteConnect::new("your_api_key", "");

        match kite.holdings().await {
            Ok(holdings) => {
                web_sys::console::log_1(&format!("Holdings: {:?}", holdings).into());
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Error: {:?}", e).into());
            }
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Example usage for native environment
    let mut kite = KiteConnect::new("your_api_key", "your_access_token");

    // Generate session
    match kite.generate_session("request_token", "api_secret").await {
        Ok(session) => println!("Session generated: {:?}", session),
        Err(e) => println!("Error generating session: {:?}", e),
    }

    // Get holdings
    match kite.holdings().await {
        Ok(holdings) => println!("Holdings: {:?}", holdings),
        Err(e) => println!("Error getting holdings: {:?}", e),
    }

    Ok(())
}
