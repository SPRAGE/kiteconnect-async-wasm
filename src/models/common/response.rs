/*!
Response wrapper types for KiteConnect API responses.

All KiteConnect API responses follow a standard format:
```json
{
    "status": "success" | "error",
    "data": { ... } | null,
    "message": "string",
    "error_type": "string" (optional)
}
```
*/

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::errors::{KiteError, KiteResult};

/// Standard KiteConnect API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiteResponse<T> {
    /// Response status ("success" or "error")
    pub status: String,
    
    /// Response data (None for error responses)
    pub data: Option<T>,
    
    /// Response message
    #[serde(default)]
    pub message: String,
    
    /// Error type (for error responses)
    #[serde(default)]
    pub error_type: Option<String>,
}

impl<T> KiteResponse<T> {
    /// Create a new success response
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data: Some(data),
            message: String::new(),
            error_type: None,
        }
    }

    /// Create a new error response
    pub fn error(message: impl Into<String>, error_type: Option<String>) -> Self {
        Self {
            status: "error".to_string(),
            data: None,
            message: message.into(),
            error_type,
        }
    }

    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        self.status == "success"
    }

    /// Check if the response is an error
    pub fn is_error(&self) -> bool {
        self.status == "error"
    }

    /// Extract the data or return an error
    pub fn into_result(self) -> KiteResult<T> {
        match self.status.as_str() {
            "success" => self.data.ok_or_else(|| {
                KiteError::general("Success response missing data")
            }),
            "error" => Err(KiteError::api_error_with_type(
                self.status,
                self.message,
                self.error_type.unwrap_or_default(),
            )),
            _ => Err(KiteError::general(format!(
                "Unknown response status: {}", 
                self.status
            ))),
        }
    }
}

/// Raw response for backward compatibility and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    /// Response status
    pub status: String,
    
    /// Raw JSON data
    pub data: Option<JsonValue>,
    
    /// Response message
    #[serde(default)]
    pub message: String,
    
    /// Error type
    #[serde(default)]
    pub error_type: Option<String>,
}

impl From<RawResponse> for KiteResponse<JsonValue> {
    fn from(raw: RawResponse) -> Self {
        Self {
            status: raw.status,
            data: raw.data,
            message: raw.message,
            error_type: raw.error_type,
        }
    }
}

/// Status enum for type-safe status handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
}

impl Status {
    pub fn is_success(self) -> bool {
        matches!(self, Status::Success)
    }

    pub fn is_error(self) -> bool {
        matches!(self, Status::Error)
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "success" => Status::Success,
            "error" => Status::Error,
            _ => Status::Error, // Default to error for unknown status
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}
