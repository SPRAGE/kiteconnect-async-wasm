/*!
Custom error types for KiteConnect operations using `thiserror`.

Provides comprehensive error handling with proper error chaining and context.
*/

use thiserror::Error;

/// Main error type for all KiteConnect operations
#[derive(Debug, Error)]
pub enum KiteError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parsing failed
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    // === Official KiteConnect API Exception Types ===
    /// Session expired or invalidated (403 header)
    /// User should clear session and re-initiate login
    #[error("Token exception: {0}")]
    TokenException(String),

    /// User account related errors
    #[error("User exception: {0}")]
    UserException(String),

    /// Order related errors (placement failures, corrupt fetch, etc.)
    #[error("Order exception: {0}")]
    OrderException(String),

    /// Missing required fields, bad parameter values
    #[error("Input exception: {0}")]
    InputException(String),

    /// Insufficient funds required for order placement
    #[error("Margin exception: {0}")]
    MarginException(String),

    /// Insufficient holdings available to place sell order
    #[error("Holding exception: {0}")]
    HoldingException(String),

    /// Network error - API unable to communicate with OMS
    #[error("Network exception: {0}")]
    NetworkException(String),

    /// Internal system error - API unable to understand OMS response
    #[error("Data exception: {0}")]
    DataException(String),

    /// Unclassified error (should be rare)
    #[error("General exception: {0}")]
    GeneralException(String),

    // === Generic fallback errors ===
    /// Generic API error when error_type is not recognized
    #[error("API error: {status} - {message}")]
    Api {
        status: String,
        message: String,
        error_type: Option<String>,
    },

    /// Authentication failed (generic)
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Invalid parameter provided (generic)
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// CSV parsing failed (for instruments data)
    #[cfg(feature = "native")]
    #[error("CSV parsing failed: {0}")]
    CsvParsing(#[from] csv::Error),

    /// Date/time parsing failed
    #[error("Date/time parsing failed: {0}")]
    DateTimeParsing(#[from] chrono::ParseError),

    /// URL parsing failed
    #[error("URL parsing failed: {0}")]
    UrlParsing(#[from] url::ParseError),

    /// General error with custom message
    #[error("KiteConnect error: {0}")]
    General(String),

    /// Backward compatibility with anyhow errors
    #[error("Legacy error: {0}")]
    Legacy(#[from] anyhow::Error),
}

/// Result type alias for KiteConnect operations
pub type KiteResult<T> = Result<T, KiteError>;

impl KiteError {
    /// Create a new API error from response
    /// Maps official KiteConnect error_type to specific exception types
    pub fn from_api_response(
        status_code: u16,
        status: impl Into<String>,
        message: impl Into<String>,
        error_type: Option<String>,
    ) -> Self {
        let message = message.into();

        // First, map based on error_type from API response
        if let Some(error_type) = error_type.as_ref() {
            return match error_type.as_str() {
                "TokenException" => Self::TokenException(message),
                "UserException" => Self::UserException(message),
                "OrderException" => Self::OrderException(message),
                "InputException" => Self::InputException(message),
                "MarginException" => Self::MarginException(message),
                "HoldingException" => Self::HoldingException(message),
                "NetworkException" => Self::NetworkException(message),
                "DataException" => Self::DataException(message),
                "GeneralException" => Self::GeneralException(message),
                _ => Self::Api {
                    status: status.into(),
                    message,
                    error_type: Some(error_type.clone()),
                },
            };
        }

        // Fallback: map based on HTTP status code
        match status_code {
            400 => Self::InputException(message), // Missing or bad request parameters or values
            403 => Self::TokenException(message), // Session expired or invalidate. Must relogin
            404 => Self::Api {
                status: status.into(),
                message,
                error_type: Some("ResourceNotFound".to_string()), // Request resource was not found
            },
            405 => Self::Api {
                status: status.into(),
                message,
                error_type: Some("MethodNotAllowed".to_string()), // Request method (GET, POST etc.) is not allowed on the requested endpoint
            },
            410 => Self::Api {
                status: status.into(),
                message,
                error_type: Some("ResourceGone".to_string()), // The requested resource is gone permanently
            },
            429 => Self::Api {
                status: status.into(),
                message,
                error_type: Some("RateLimited".to_string()), // Too many requests to the API (rate limiting)
            },
            500 => Self::GeneralException(message), // Something unexpected went wrong
            502 => Self::NetworkException(message), // The backend OMS is down and the API is unable to communicate with it
            503 => Self::NetworkException(message), // Service unavailable; the API is down
            504 => Self::NetworkException(message), // Gateway timeout; the API is unreachable
            _ => Self::Api {
                status: status.into(),
                message,
                error_type,
            },
        }
    }

    /// Create a new API error (legacy method for backward compatibility)
    pub fn api_error(status: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Api {
            status: status.into(),
            message: message.into(),
            error_type: None,
        }
    }

    /// Create a new API error with error type (legacy method for backward compatibility)
    pub fn api_error_with_type(
        status: impl Into<String>,
        message: impl Into<String>,
        error_type: impl Into<String>,
    ) -> Self {
        Self::Api {
            status: status.into(),
            message: message.into(),
            error_type: Some(error_type.into()),
        }
    }

    /// Create a new authentication error
    pub fn auth_error(message: impl Into<String>) -> Self {
        Self::Authentication(message.into())
    }

    /// Create a new invalid parameter error
    pub fn invalid_param(message: impl Into<String>) -> Self {
        Self::InvalidParameter(message.into())
    }

    /// Create a new general error
    pub fn general(message: impl Into<String>) -> Self {
        Self::General(message.into())
    }

    // === Official KiteConnect Exception Constructors ===

    /// Create a new TokenException
    pub fn token_exception(message: impl Into<String>) -> Self {
        Self::TokenException(message.into())
    }

    /// Create a new UserException
    pub fn user_exception(message: impl Into<String>) -> Self {
        Self::UserException(message.into())
    }

    /// Create a new OrderException
    pub fn order_exception(message: impl Into<String>) -> Self {
        Self::OrderException(message.into())
    }

    /// Create a new InputException
    pub fn input_exception(message: impl Into<String>) -> Self {
        Self::InputException(message.into())
    }

    /// Create a new MarginException
    pub fn margin_exception(message: impl Into<String>) -> Self {
        Self::MarginException(message.into())
    }

    /// Create a new HoldingException
    pub fn holding_exception(message: impl Into<String>) -> Self {
        Self::HoldingException(message.into())
    }

    /// Create a new NetworkException
    pub fn network_exception(message: impl Into<String>) -> Self {
        Self::NetworkException(message.into())
    }

    /// Create a new DataException
    pub fn data_exception(message: impl Into<String>) -> Self {
        Self::DataException(message.into())
    }

    /// Create a new GeneralException
    pub fn general_exception(message: impl Into<String>) -> Self {
        Self::GeneralException(message.into())
    }

    /// Check if this error requires re-authentication
    pub fn requires_reauth(&self) -> bool {
        matches!(self, Self::TokenException(_) | Self::Authentication(_))
    }

    /// Check if this is a client-side error (4xx)
    pub fn is_client_error(&self) -> bool {
        match self {
            Self::TokenException(_) | Self::InputException(_) | Self::InvalidParameter(_) => true,
            Self::Api { status, .. } => status.starts_with('4'),
            _ => false,
        }
    }

    /// Check if this is a server-side error (5xx)
    pub fn is_server_error(&self) -> bool {
        match self {
            Self::NetworkException(_) | Self::DataException(_) | Self::GeneralException(_) => true,
            Self::Api { status, .. } => status.starts_with('5'),
            _ => false,
        }
    }

    /// Check if this error can be retried
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::NetworkException(_) | Self::Http(_) => true, // Includes 502, 503, 504 network errors
            Self::Api { status, .. } => matches!(status.as_str(), "429"), // Only rate limiting is retryable for API errors
            _ => false,
        }
    }
}
