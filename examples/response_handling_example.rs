//! Example demonstrating KiteConnect API response handling
//! 
//! This example shows how to use the response structures that match
//! the official KiteConnect API v3 specification.

use kiteconnect_async_wasm::model;
use serde_json::json;

fn main() {
    println!("KiteConnect Response Handling Examples");
    println!("=====================================\n");
    
    // Example 1: Successful response with data
    demo_successful_response();
    
    // Example 2: Error response
    demo_error_response();
    
    // Example 3: List response (like orders, positions)
    demo_list_response();
    
    // Example 4: Simple success response (like order placement confirmation)
    demo_success_response();
}

fn demo_successful_response() {
    println!("1. Successful Response Example:");
    println!("-------------------------------");
    
    // Simulate a successful API response JSON
    let json_response = json!({
        "status": "success",
        "data": {
            "user_id": "XYZ123",
            "user_name": "John Doe",
            "email": "john@example.com",
            "broker": "ZERODHA"
        }
    });
    
    // Parse into our response structure
    let response: model::ApiResponse<serde_json::Value> = 
        serde_json::from_value(json_response).unwrap();
    
    match response {
        model::ApiResponse::Success(success_resp) => {
            println!("✓ Status: {}", success_resp.status);
            println!("✓ Data: {:#}", success_resp.data);
        },
        model::ApiResponse::Error(error_resp) => {
            println!("✗ Error: {}", error_resp.message);
        }
    }
    
    println!();
}

fn demo_error_response() {
    println!("2. Error Response Example:");
    println!("--------------------------");
    
    // Simulate an error API response JSON
    let json_response = json!({
        "status": "error",
        "message": "Invalid API key",
        "error_type": "TokenException"
    });
    
    // Parse into our response structure
    let response: model::ApiResponse<serde_json::Value> = 
        serde_json::from_value(json_response).unwrap();
    
    match response {
        model::ApiResponse::Success(_) => {
            println!("✓ Request successful");
        },
        model::ApiResponse::Error(error_resp) => {
            println!("✗ Status: {}", error_resp.status);
            println!("✗ Error Type: {}", error_resp.error_type);
            println!("✗ Message: {}", error_resp.message);
        }
    }
    
    println!();
}

fn demo_list_response() {
    println!("3. List Response Example (e.g., Orders):");
    println!("----------------------------------------");
    
    // Simulate a list response (like orders)
    let json_response = json!({
        "status": "success",
        "data": [
            {
                "order_id": "ORDER001",
                "tradingsymbol": "RELIANCE",
                "quantity": 100,
                "price": 2500.50
            },
            {
                "order_id": "ORDER002", 
                "tradingsymbol": "TCS",
                "quantity": 50,
                "price": 3200.75
            }
        ]
    });
    
    // Parse as a list response
    let response: model::ListResponse<serde_json::Value> = 
        serde_json::from_value(json_response).unwrap();
    
    println!("✓ Status: {}", response.status);
    println!("✓ Number of items: {}", response.data.len());
    
    for (i, item) in response.data.iter().enumerate() {
        println!("  Item {}: {:#}", i + 1, item);
    }
    
    println!();
}

fn demo_success_response() {
    println!("4. Simple Success Response Example:");
    println!("-----------------------------------");
    
    // Simulate a simple success response (like order placement)
    let json_response = json!({
        "status": "success"
    });
    
    // Parse into success response
    let response: model::SuccessResponse = 
        serde_json::from_value(json_response).unwrap();
    
    println!("✓ Status: {}", response.status);
    println!("✓ Operation completed successfully");
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_response_methods() {
        // Test successful response
        let success_json = json!({
            "status": "success",
            "data": "test_data"
        });
        
        let success_response: model::ApiResponse<String> = 
            serde_json::from_value(success_json).unwrap();
        
        assert!(success_response.is_success());
        assert!(!success_response.is_error());
        assert_eq!(success_response.data().unwrap(), "test_data");
        
        // Test error response
        let error_json = json!({
            "status": "error",
            "message": "Test error",
            "error_type": "TestException"
        });
        
        let error_response: model::ApiResponse<String> = 
            serde_json::from_value(error_json).unwrap();
        
        assert!(!error_response.is_success());
        assert!(error_response.is_error());
        assert!(error_response.data().is_err());
    }
    
    #[test]
    fn test_kite_response_parsing() {
        let json = json!({
            "status": "success",
            "data": {
                "user_id": "TEST123",
                "email": "test@example.com"
            }
        });
        
        let response: model::KiteResponse<serde_json::Value> = 
            serde_json::from_value(json).unwrap();
        
        assert_eq!(response.status, "success");
        assert!(response.data.is_object());
    }
    
    #[test]
    fn test_error_response_parsing() {
        let json = json!({
            "status": "error",
            "message": "Invalid request",
            "error_type": "InputException",
            "data": {"details": "Additional error info"}
        });
        
        let response: model::KiteErrorResponse = 
            serde_json::from_value(json).unwrap();
        
        assert_eq!(response.status, "error");
        assert_eq!(response.message, "Invalid request");
        assert_eq!(response.error_type, "InputException");
        assert!(response.data.is_some());
    }
}
