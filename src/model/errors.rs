//! # Error and Exception Models
//! 
//! This module contains error types and exception models for the KiteConnect API.
//! 
//! The KiteConnect API returns structured error responses with specific exception types
//! that help identify the nature of the error. In addition to standard HTTP status codes,
//! the API includes an `error_type` field that specifies the exact exception.

use serde::{Deserialize, Serialize};
use std::fmt;

/// KiteConnect API exception types
/// 
/// These correspond to the specific exception types returned by the API
/// in the `error_type` field of error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KiteExceptionType {
    /// Token expiry or invalidation (HTTP 403)
    /// 
    /// Indicates that the authenticated session has expired or been invalidated.
    /// This can be caused by:
    /// - User logging out
    /// - Natural session expiry
    /// - User logging into another Kite instance
    /// 
    /// When this occurs, clear the user's session and re-initiate login.
    #[serde(rename = "TokenException")]
    TokenException,

    /// User account related errors
    /// 
    /// Represents errors related to the user's account status, permissions,
    /// or account-specific restrictions.
    #[serde(rename = "UserException")]
    UserException,

    /// Order related errors
    /// 
    /// Represents errors in order operations such as:
    /// - Order placement failures
    /// - Corrupt order fetch operations
    /// - Invalid order parameters
    #[serde(rename = "OrderException")]
    OrderException,

    /// Input validation errors (HTTP 400)
    /// 
    /// Represents errors due to:
    /// - Missing required fields
    /// - Bad values for parameters
    /// - Invalid request format
    #[serde(rename = "InputException")]
    InputException,

    /// Insufficient funds error
    /// 
    /// Represents errors when there are insufficient funds
    /// required for order placement.
    #[serde(rename = "MarginException")]
    MarginException,

    /// Insufficient holdings error
    /// 
    /// Represents errors when there are insufficient holdings
    /// available to place a sell order for the specified instrument.
    #[serde(rename = "HoldingException")]
    HoldingException,

    /// Network communication error
    /// 
    /// Represents network errors where the API was unable to
    /// communicate with the OMS (Order Management System).
    #[serde(rename = "NetworkException")]
    NetworkException,

    /// Internal system error
    /// 
    /// Represents an internal system error where the API was unable
    /// to understand the response from the OMS.
    #[serde(rename = "DataException")]
    DataException,

    /// Unclassified error
    /// 
    /// Represents an unclassified error. This should only happen rarely
    /// and indicates an unexpected system condition.
    #[serde(rename = "GeneralException")]
    GeneralException,

    /// Unknown exception type
    /// 
    /// Used for exception types not explicitly defined above.
    /// Contains the original string value.
    #[serde(untagged)]
    Unknown(String),
}

impl fmt::Display for KiteExceptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KiteExceptionType::TokenException => write!(f, "TokenException"),
            KiteExceptionType::UserException => write!(f, "UserException"),
            KiteExceptionType::OrderException => write!(f, "OrderException"),
            KiteExceptionType::InputException => write!(f, "InputException"),
            KiteExceptionType::MarginException => write!(f, "MarginException"),
            KiteExceptionType::HoldingException => write!(f, "HoldingException"),
            KiteExceptionType::NetworkException => write!(f, "NetworkException"),
            KiteExceptionType::DataException => write!(f, "DataException"),
            KiteExceptionType::GeneralException => write!(f, "GeneralException"),
            KiteExceptionType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// HTTP status codes commonly returned by the KiteConnect API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum KiteHttpStatus {
    /// Missing or bad request parameters or values
    BadRequest = 400,
    
    /// Session expired or invalidated. Must re-login
    Forbidden = 403,
    
    /// Requested resource was not found
    NotFound = 404,
    
    /// Request method (GET, POST etc.) is not allowed on the endpoint
    MethodNotAllowed = 405,
    
    /// The requested resource is gone permanently
    Gone = 410,
    
    /// Too many requests to the API (rate limiting)
    TooManyRequests = 429,
    
    /// Something unexpected went wrong
    InternalServerError = 500,
    
    /// The backend OMS is down and the API is unable to communicate with it
    BadGateway = 502,
    
    /// Service unavailable; the API is down
    ServiceUnavailable = 503,
    
    /// Gateway timeout; the API is unreachable
    GatewayTimeout = 504,
}

impl fmt::Display for KiteHttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", *self as u16, self.reason_phrase())
    }
}

impl KiteHttpStatus {
    /// Get the reason phrase for the HTTP status code
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            KiteHttpStatus::BadRequest => "Bad Request",
            KiteHttpStatus::Forbidden => "Forbidden",
            KiteHttpStatus::NotFound => "Not Found",
            KiteHttpStatus::MethodNotAllowed => "Method Not Allowed",
            KiteHttpStatus::Gone => "Gone",
            KiteHttpStatus::TooManyRequests => "Too Many Requests",
            KiteHttpStatus::InternalServerError => "Internal Server Error",
            KiteHttpStatus::BadGateway => "Bad Gateway",
            KiteHttpStatus::ServiceUnavailable => "Service Unavailable",
            KiteHttpStatus::GatewayTimeout => "Gateway Timeout",
        }
    }

    /// Check if the status code indicates a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(self, 
            KiteHttpStatus::BadRequest |
            KiteHttpStatus::Forbidden |
            KiteHttpStatus::NotFound |
            KiteHttpStatus::MethodNotAllowed |
            KiteHttpStatus::Gone |
            KiteHttpStatus::TooManyRequests
        )
    }

    /// Check if the status code indicates a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self,
            KiteHttpStatus::InternalServerError |
            KiteHttpStatus::BadGateway |
            KiteHttpStatus::ServiceUnavailable |
            KiteHttpStatus::GatewayTimeout
        )
    }

    /// Check if the error requires re-authentication
    pub fn requires_reauth(&self) -> bool {
        matches!(self, KiteHttpStatus::Forbidden)
    }

    /// Check if the error is due to rate limiting
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, KiteHttpStatus::TooManyRequests)
    }
}

/// API rate limit information and constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Endpoint category
    pub endpoint: RateLimitEndpoint,
    /// Requests per second allowed
    pub requests_per_second: u32,
    /// Additional constraints
    pub constraints: Option<RateLimitConstraints>,
}

/// Rate limit categories for different endpoints
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RateLimitEndpoint {
    /// Quote endpoints - 1 req/second
    #[serde(rename = "quote")]
    Quote,
    
    /// Historical candle endpoints - 3 req/second
    #[serde(rename = "historical_candle")]
    HistoricalCandle,
    
    /// Order placement endpoints - 10 req/second
    #[serde(rename = "order_placement")]
    OrderPlacement,
    
    /// All other endpoints - 10 req/second
    #[serde(rename = "other")]
    Other,
}

impl RateLimitEndpoint {
    /// Get the default rate limit for this endpoint type
    pub fn default_rate_limit(&self) -> u32 {
        match self {
            RateLimitEndpoint::Quote => 1,
            RateLimitEndpoint::HistoricalCandle => 3,
            RateLimitEndpoint::OrderPlacement => 10,
            RateLimitEndpoint::Other => 10,
        }
    }
}

/// Additional rate limit constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConstraints {
    /// Maximum orders per minute (200)
    pub max_orders_per_minute: Option<u32>,
    
    /// Maximum orders per second (10)
    pub max_orders_per_second: Option<u32>,
    
    /// Maximum orders per day (3000)
    pub max_orders_per_day: Option<u32>,
    
    /// Maximum order modifications per order (25)
    pub max_modifications_per_order: Option<u32>,
}

impl Default for RateLimitConstraints {
    fn default() -> Self {
        Self {
            max_orders_per_minute: Some(200),
            max_orders_per_second: Some(10),
            max_orders_per_day: Some(3000),
            max_modifications_per_order: Some(25),
        }
    }
}

/// Enhanced error response with additional context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiteError {
    /// HTTP status code
    pub status_code: Option<u16>,
    
    /// Exception type
    pub exception_type: KiteExceptionType,
    
    /// Error message
    pub message: String,
    
    /// Additional error data
    pub data: Option<serde_json::Value>,
    
    /// Whether this error requires re-authentication
    pub requires_reauth: bool,
    
    /// Whether this error is due to rate limiting
    pub is_rate_limited: bool,
    
    /// Whether this is a retryable error
    pub is_retryable: bool,
}

impl KiteError {
    /// Create a new KiteError from basic components
    pub fn new(exception_type: KiteExceptionType, message: String) -> Self {
        let requires_reauth = matches!(exception_type, KiteExceptionType::TokenException);
        let is_rate_limited = false; // This would be determined by HTTP status
        let is_retryable = matches!(exception_type, 
            KiteExceptionType::NetworkException |
            KiteExceptionType::DataException
        );

        Self {
            status_code: None,
            exception_type,
            message,
            data: None,
            requires_reauth,
            is_rate_limited,
            is_retryable,
        }
    }

    /// Create a KiteError with HTTP status code
    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        
        // Update flags based on status code
        if status_code == 403 {
            self.requires_reauth = true;
        }
        if status_code == 429 {
            self.is_rate_limited = true;
        }
        if status_code >= 500 && status_code < 600 {
            self.is_retryable = true;
        }
        
        self
    }

    /// Add additional error data
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Check if this error indicates the session has expired
    pub fn is_session_expired(&self) -> bool {
        self.requires_reauth
    }

    /// Get a user-friendly error category
    pub fn error_category(&self) -> &'static str {
        match self.exception_type {
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

impl fmt::Display for KiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", 
            self.error_category(),
            self.exception_type,
            self.message
        )
    }
}

impl std::error::Error for KiteError {}

/// Result type for KiteConnect API operations
pub type KiteResult<T> = Result<T, KiteError>;
