use anyhow::Result;
use std::collections::HashMap;
use std::future::Future;

/// Async trait for handling HTTP requests across different platforms
pub trait RequestHandler {
    fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> impl Future<Output = Result<reqwest::Response>> + Send;
}
