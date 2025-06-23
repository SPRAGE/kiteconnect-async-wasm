//! # Utility Functions
//! 
//! This module contains utility functions used throughout the KiteConnect library.

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use serde_json::Value as JsonValue;
use anyhow::Result;
use std::collections::HashMap;


// WASM platform imports  
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use web_sys::window;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use js_sys::Uint8Array;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen_futures::JsFuture;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use csv_core::{Reader, ReadFieldResult};

#[cfg(not(test))]
pub const URL: &str = "https://api.kite.trade";

#[cfg(test)]
pub const URL: &str = "http://127.0.0.1:1234";

/// Async trait for handling HTTP requests across different platforms
pub trait RequestHandler {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response>;
}

/// Parse CSV data using csv-core for WASM compatibility
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
