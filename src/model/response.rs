//! # Response Models
//! 
//! This module contains response data models for the KiteConnect API.
//! 
//! The response structure follows the official KiteConnect API v3 specification:
//! - Successful responses have `status: "success"` and `data` containing the payload
//! - Error responses have `status: "error"`, `message`, and `error_type` fields
//! - All responses are JSON with `application/json` content-type

use serde::{Deserialize, Serialize};
use super::errors::{KiteExceptionType, KiteError, KiteResult};

/// Standard KiteConnect API response wrapper for successful requests
/// 
/// According to the official docs, successful responses always have:
/// - `status`: "success" 
/// - `data`: the actual response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiteResponse<T> {
    /// Response status - always "success" for successful requests
    pub status: String,
    /// Response data containing the actual payload
    pub data: T,
}

/// KiteConnect API error response
/// 
/// Error responses contain:
/// - `status`: "error"
/// - `message`: textual description of the error  
/// - `error_type`: name of the exception
/// - `data`: optional additional payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiteErrorResponse {
    /// Response status - always "error" for error responses
    pub status: String,
    /// Error message description
    pub message: String,
    /// Type/name of the exception
    pub error_type: KiteExceptionType,
    /// Optional additional error data
    pub data: Option<serde_json::Value>,
}

impl KiteErrorResponse {
    /// Convert this error response to a KiteError with additional context
    pub fn to_kite_error(&self) -> KiteError {
        KiteError::new(self.error_type.clone(), self.message.clone())
            .with_data(self.data.clone().unwrap_or_default())
    }

    /// Convert this error response to a KiteError with HTTP status code
    pub fn to_kite_error_with_status(&self, status_code: u16) -> KiteError {
        self.to_kite_error().with_status_code(status_code)
    }

    /// Check if this error requires re-authentication
    pub fn requires_reauth(&self) -> bool {
        matches!(self.error_type, KiteExceptionType::TokenException)
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self.error_type, 
            KiteExceptionType::NetworkException |
            KiteExceptionType::DataException
        )
    }

    /// Get a user-friendly error category
    pub fn error_category(&self) -> &'static str {
        match self.error_type {
            KiteExceptionType::TokenException => "Authentication",
            KiteExceptionType::UserException => "Account",
            KiteExceptionType::OrderException => "Order",
            KiteExceptionType::InputException => "Input Validation",
            KiteExceptionType::MarginException => "Insufficient Funds",
            KiteExceptionType::HoldingException => "Insufficient Holdings",
            KiteExceptionType::NetworkException => "Network",
            KiteExceptionType::DataException => "System",
            KiteExceptionType::GeneralException => "General",
            KiteExceptionType::Unknown(_) => "Unknown",
        }
    }
}

impl From<KiteError> for KiteErrorResponse {
    /// Convert a KiteError to a KiteErrorResponse
    fn from(error: KiteError) -> Self {
        Self {
            status: "error".to_string(),
            message: error.message,
            error_type: error.exception_type,
            data: error.data,
        }
    }
}

/// Generic API response that can be either success or error
/// 
/// This enum allows handling both successful and error responses
/// in a type-safe manner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    /// Successful response with data
    Success(KiteResponse<T>),
    /// Error response
    Error(KiteErrorResponse),
}

impl<T> ApiResponse<T> {
    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        matches!(self, ApiResponse::Success(_))
    }
    
    /// Check if the response is an error
    pub fn is_error(&self) -> bool {
        matches!(self, ApiResponse::Error(_))
    }
    
    /// Extract the data from a successful response
    pub fn data(self) -> Result<T, KiteErrorResponse> {
        match self {
            ApiResponse::Success(response) => Ok(response.data),
            ApiResponse::Error(error) => Err(error),
        }
    }
    
    /// Get reference to the data if response is successful
    pub fn data_ref(&self) -> Result<&T, &KiteErrorResponse> {
        match self {
            ApiResponse::Success(response) => Ok(&response.data),
            ApiResponse::Error(error) => Err(error),
        }
    }
    
    /// Convert to a KiteResult, extracting data or converting error
    pub fn into_result(self) -> KiteResult<T> {
        match self {
            ApiResponse::Success(response) => Ok(response.data),
            ApiResponse::Error(error) => Err(error.to_kite_error()),
        }
    }

    /// Convert to a KiteResult with HTTP status code context
    pub fn into_result_with_status(self, status_code: u16) -> KiteResult<T> {
        match self {
            ApiResponse::Success(response) => Ok(response.data),
            ApiResponse::Error(error) => Err(error.to_kite_error_with_status(status_code)),
        }
    }

    /// Create a successful ApiResponse
    pub fn success(data: T) -> Self {
        ApiResponse::Success(KiteResponse {
            status: "success".to_string(),
            data,
        })
    }

    /// Create an error ApiResponse from a KiteError
    pub fn error(error: KiteError) -> Self {
        ApiResponse::Error(error.into())
    }

    /// Create an error ApiResponse from exception type and message
    pub fn error_from_parts(exception_type: KiteExceptionType, message: String) -> Self {
        let error = KiteError::new(exception_type, message);
        ApiResponse::Error(error.into())
    }

    /// Get error details if this is an error response
    pub fn error_details(&self) -> Option<(KiteExceptionType, &str)> {
        match self {
            ApiResponse::Error(error) => Some((error.error_type.clone(), &error.message)),
            ApiResponse::Success(_) => None,
        }
    }

    /// Check if this response indicates session expiry
    pub fn is_session_expired(&self) -> bool {
        match self {
            ApiResponse::Error(error) => error.requires_reauth(),
            ApiResponse::Success(_) => false,
        }
    }

    /// Check if this response indicates a retryable error
    pub fn is_retryable(&self) -> bool {
        match self {
            ApiResponse::Error(error) => error.is_retryable(),
            ApiResponse::Success(_) => false,
        }
    }
}

/// Helper functions for creating responses from HTTP data
impl<T> ApiResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    /// Parse an HTTP response into an ApiResponse
    /// 
    /// This method attempts to deserialize the response body as either a success
    /// or error response based on the HTTP status code and JSON structure.
    pub fn from_http_response(
        status_code: u16,
        body: &str,
    ) -> Result<Self, serde_json::Error> {
        // First try to deserialize as a regular API response
        match serde_json::from_str::<ApiResponse<T>>(body) {
            Ok(response) => {
                // If it's an error response, add the HTTP status code context
                if let ApiResponse::Error(_) = &response {
                    match response {
                        ApiResponse::Error(error_resp) => {
                            // We can't modify the error_resp directly since it's moved,
                            // so we'll create a new one with enhanced error context
                            let enhanced_error = error_resp.to_kite_error_with_status(status_code);
                            Ok(ApiResponse::Error(enhanced_error.into()))
                        }
                        _ => Ok(response),
                    }
                } else {
                    Ok(response)
                }
            }
            Err(e) => {
                // If deserialization fails, try to create an appropriate error response
                if status_code >= 400 {
                    let error_type = match status_code {
                        400 => KiteExceptionType::InputException,
                        403 => KiteExceptionType::TokenException,
                        404 => KiteExceptionType::GeneralException,
                        429 => KiteExceptionType::NetworkException,
                        500..=599 => KiteExceptionType::DataException,
                        _ => KiteExceptionType::GeneralException,
                    };
                    
                    let error = KiteError::new(
                        error_type,
                        format!("HTTP {} - Failed to parse response: {}", status_code, e)
                    ).with_status_code(status_code);
                    
                    Ok(ApiResponse::Error(error.into()))
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Create an ApiResponse from a raw HTTP error
    pub fn from_http_error(status_code: u16, message: String) -> Self {
        let error_type = match status_code {
            400 => KiteExceptionType::InputException,
            403 => KiteExceptionType::TokenException,
            404 => KiteExceptionType::GeneralException,
            405 => KiteExceptionType::InputException,
            410 => KiteExceptionType::GeneralException,
            429 => KiteExceptionType::NetworkException,
            500 => KiteExceptionType::GeneralException,
            502 => KiteExceptionType::NetworkException,
            503 => KiteExceptionType::DataException,
            504 => KiteExceptionType::NetworkException,
            _ => KiteExceptionType::GeneralException,
        };

        let error = KiteError::new(error_type, message).with_status_code(status_code);
        ApiResponse::Error(error.into())
    }
}

/// Simple success response for operations that don't return data
/// 
/// Used for operations like placing orders, cancelling orders, etc.
/// where only confirmation of success is needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Success status - always "success"
    pub status: String,
}

/// List response wrapper for endpoints that return arrays
/// 
/// Many KiteConnect endpoints return arrays of data (orders, positions, etc.)
/// This type provides a consistent way to handle such responses
pub type ListResponse<T> = KiteResponse<Vec<T>>;
