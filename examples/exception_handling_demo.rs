//! Demonstration of KiteConnect Exception Handling
//! 
//! This example shows how the new exception handling system
//! maps official KiteConnect API error types to specific exception variants.

use kiteconnect_async_wasm::models::common::KiteError;

fn main() {
    println!("=== KiteConnect Exception Handling Demo ===\n");

    // Demonstrate official API exception types
    demonstrate_official_exceptions();

    // Demonstrate HTTP status code mapping
    demonstrate_http_status_mapping();

    // Demonstrate error classification methods
    demonstrate_error_classification();
}

fn demonstrate_official_exceptions() {
    println!("📋 Official KiteConnect API Exception Types:");

    let exceptions = vec![
        ("TokenException", "Session expired or invalidated", KiteError::token_exception("Session has expired")),
        ("UserException", "User account related errors", KiteError::user_exception("Account suspended")),
        ("OrderException", "Order placement/management errors", KiteError::order_exception("Order placement failed")),
        ("InputException", "Invalid parameters", KiteError::input_exception("Missing required field: symbol")),
        ("MarginException", "Insufficient funds", KiteError::margin_exception("Insufficient margin for order")),
        ("HoldingException", "Insufficient holdings", KiteError::holding_exception("Not enough shares to sell")),
        ("NetworkException", "API communication issues", KiteError::network_exception("Unable to connect to OMS")),
        ("DataException", "Internal system errors", KiteError::data_exception("Unable to parse OMS response")),
        ("GeneralException", "Unclassified errors", KiteError::general_exception("Unknown server error")),
    ];

    for (name, description, error) in exceptions {
        println!("  • {}: {} → {:?}", name, description, error);
    }
    println!();
}

fn demonstrate_http_status_mapping() {
    println!("🌐 HTTP Status Code Mapping:");

    let status_mappings = vec![
        (400, "Missing or bad request parameters", "InputException"),
        (403, "Session expired or invalidate", "TokenException"),
        (404, "Request resource was not found", "ResourceNotFound"),
        (405, "Request method not allowed", "MethodNotAllowed"),
        (410, "Resource gone permanently", "ResourceGone"),
        (429, "Too many requests (rate limiting)", "RateLimited"),
        (500, "Something unexpected went wrong", "GeneralException"),
        (502, "Backend OMS down", "NetworkException"),
        (503, "Service unavailable; API is down", "NetworkException"),
        (504, "Gateway timeout; API unreachable", "NetworkException"),
    ];

    for (status_code, description, expected_type) in status_mappings {
        let error = KiteError::from_api_response(
            status_code,
            status_code.to_string(),
            description,
            None
        );
        println!("  • HTTP {}: {} → {:?}", status_code, expected_type, error);
    }
    println!();
}

fn demonstrate_error_classification() {
    println!("🔍 Error Classification Methods:");

    let errors = vec![
        ("Token expired", KiteError::token_exception("Session expired")),
        ("Invalid input", KiteError::input_exception("Bad parameter")),
        ("Network issue", KiteError::network_exception("Connection failed")),
        ("Server error", KiteError::general_exception("Internal error")),
    ];

    for (description, error) in errors {
        println!("  • {}: ", description);
        println!("    - Requires re-auth: {}", error.requires_reauth());
        println!("    - Is client error: {}", error.is_client_error());
        println!("    - Is server error: {}", error.is_server_error());
        println!("    - Is retryable: {}", error.is_retryable());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_parsing() {
        // Test error_type mapping
        let error = KiteError::from_api_response(
            400,
            "400",
            "Invalid trading symbol",
            Some("InputException".to_string())
        );
        
        match error {
            KiteError::InputException(msg) => {
                assert_eq!(msg, "Invalid trading symbol");
            }
            _ => panic!("Expected InputException")
        }
    }

    #[test]
    fn test_status_code_fallback() {
        // Test fallback to status code mapping when error_type is missing
        let error = KiteError::from_api_response(
            403,
            "403",
            "Unauthorized",
            None
        );
        
        match error {
            KiteError::TokenException(msg) => {
                assert_eq!(msg, "Unauthorized");
            }
            _ => panic!("Expected TokenException")
        }
    }

    #[test]
    fn test_error_classification() {
        let token_error = KiteError::token_exception("Session expired");
        assert!(token_error.requires_reauth());
        assert!(token_error.is_client_error());
        assert!(!token_error.is_server_error());
        assert!(!token_error.is_retryable());

        let network_error = KiteError::network_exception("Connection failed");
        assert!(!network_error.requires_reauth());
        assert!(!network_error.is_client_error());
        assert!(network_error.is_server_error());
        assert!(network_error.is_retryable());
    }

    #[test]
    fn test_network_exception_mapping() {
        // Test that 503 maps to NetworkException (not Api with ServiceUnavailable)
        let error = KiteError::from_api_response(
            503,
            "503",
            "Service unavailable",
            None
        );
        
        // Check properties before moving the error
        assert!(error.is_retryable()); // Should be retryable
        assert!(error.is_server_error()); // Should be server error
        
        match error {
            KiteError::NetworkException(msg) => {
                assert_eq!(msg, "Service unavailable");
            }
            _ => panic!("Expected NetworkException for 503 status")
        }
    }
}
