//! # Utility Functions
//!
//! This module contains utility functions and platform-specific implementations
//! used throughout the KiteConnect library. It provides cross-platform abstractions
//! for HTTP requests, CSV parsing, and other common operations.
//!
//! ## Platform Support
//!
//! The utilities in this module are designed to work across different platforms:
//! - **Native**: Full functionality with optimized implementations
//! - **WASM**: Browser-compatible implementations using Web APIs
//!
//! ## Key Features
//!
//! - **Cross-platform HTTP handling**: Abstract interface for HTTP requests
//! - **CSV parsing**: Platform-specific CSV parsing (native: `csv`, WASM: `csv-core`)
//! - **URL management**: Centralized API endpoint configuration
//! - **Error handling**: Consistent error patterns across platforms
//!
//! ## Example
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::utils::parse_csv_with_core;
//!
//! # #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let csv_data = "header1,header2\nvalue1,value2\n";
//! let parsed = parse_csv_with_core(csv_data)?;
//! println!("Parsed CSV: {:?}", parsed);
//! # Ok(())
//! # }
//! ```

use anyhow::Result;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use serde_json::Value as JsonValue;
use std::collections::HashMap;

// WASM platform imports
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use web_sys::window;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use js_sys::Uint8Array;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen_futures::JsFuture;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use csv_core::{ReadFieldResult, Reader};

/// Base URL for KiteConnect API in production
#[cfg(not(test))]
pub const URL: &str = "https://api.kite.trade";

/// Base URL for KiteConnect API in test environment
///
/// Used during testing to point to a local mock server for reliable,
/// offline testing without making actual API calls.
#[cfg(test)]
pub const URL: &str = "http://127.0.0.1:1234";

/// Async trait for handling HTTP requests across different platforms
///
/// This trait provides a platform-agnostic interface for making HTTP requests.
/// Implementations handle the specifics of each platform (native vs WASM)
/// while providing a consistent API for the rest of the library.
///
/// # Platform Implementations
///
/// - **Native**: Uses `reqwest` for full HTTP client functionality
/// - **WASM**: Uses `fetch` API for browser-compatible requests
///
/// # Example
///
/// ```rust,no_run
/// use kiteconnect_async_wasm::connect::utils::RequestHandler;
/// use std::collections::HashMap;
///
/// # struct MyClient;
/// # impl RequestHandler for MyClient {
/// #     async fn send_request(
/// #         &self,
/// #         url: reqwest::Url,
/// #         method: &str,
/// #         data: Option<HashMap<&str, &str>>,
/// #     ) -> anyhow::Result<reqwest::Response> {
/// #         unimplemented!()
/// #     }
/// # }
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = MyClient;
/// let url = reqwest::Url::parse("https://api.example.com/data")?;
/// let mut params = HashMap::new();
/// params.insert("key", "value");
///
/// let response = client.send_request(url, "GET", Some(params)).await?;
/// # Ok(())
/// # }
/// ```
pub trait RequestHandler {
    /// Send an HTTP request with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `url` - The complete URL to send the request to
    /// * `method` - HTTP method ("GET", "POST", "PUT", "DELETE")
    /// * `data` - Optional form data to include in the request
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The network request fails
    /// - The URL is malformed
    /// - Authentication is required but missing
    /// - The server returns an error status
    fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<reqwest::Response>> + Send;
}

/// Parse CSV data using csv-core for WASM compatibility
///
/// This function provides CSV parsing capability in WASM environments where
/// the standard `csv` crate is not available. It uses `csv-core` which is
/// a no-std implementation suitable for WebAssembly.
///
/// # Arguments
///
/// * `csv_data` - Raw CSV data as a string
///
/// # Returns
///
/// A `Result` containing the parsed CSV data as a JSON array of objects,
/// where each object represents a row with column headers as keys.
///
/// # Errors
///
/// Returns an error if:
/// - The CSV data is malformed
/// - Memory allocation fails during parsing
/// - JSON serialization fails
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
/// use kiteconnect_async_wasm::connect::utils::parse_csv_with_core;
///
/// # #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let csv_data = r#"symbol,exchange,price
/// RELIANCE,NSE,2500.00
/// TCS,NSE,3200.50"#;
///
/// let parsed = parse_csv_with_core(csv_data)?;
///
/// // parsed is a JSON array:
/// // [
/// //   {"symbol": "RELIANCE", "exchange": "NSE", "price": "2500.00"},
/// //   {"symbol": "TCS", "exchange": "NSE", "price": "3200.50"}
/// // ]
/// # Ok(())
/// # }
/// ```
///
/// # Platform Availability
///
/// This function is only available on WASM targets. On native platforms,
/// use the standard `csv` crate which provides better performance and features.
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn parse_csv_with_core(csv_data: &str) -> Result<JsonValue> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();

    let mut headers: Vec<String> = Vec::new();
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;

    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];

        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                }
                break;
            }
            ReadFieldResult::OutputFull => {
                field.extend_from_slice(&output[..output_written]);
                // Continue reading with same input
            }
            ReadFieldResult::Field { record_end } => {
                field.extend_from_slice(&output[..output_written]);
                let field_str = String::from_utf8_lossy(&field).to_string();
                current_record.push(field_str);
                field.clear();

                if record_end {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
            ReadFieldResult::Record => {
                // This case should not happen based on the API, but we handle it for completeness
                continue;
            }
        }
    }

    // Convert to JSON format
    let mut result: Vec<JsonValue> = Vec::new();
    for record in records {
        let mut obj = serde_json::Map::new();
        for (i, value) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.clone(), JsonValue::String(value.clone()));
            }
        }
        result.push(JsonValue::Object(obj));
    }

    Ok(JsonValue::Array(result))
}
