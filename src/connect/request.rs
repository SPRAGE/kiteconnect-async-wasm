use anyhow::Result;
use std::collections::HashMap;

/// Async trait for handling HTTP requests across different platforms
pub trait RequestHandler {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response>;
}
