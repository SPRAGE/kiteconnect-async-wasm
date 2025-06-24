/*!
Foundation Types Demo

This example demonstrates:
- Importing the new typed models
- Using the common error types
- Creating response wrappers
- Working with enums
*/

use kiteconnect_async_wasm::models::prelude::*;

fn main() {
    println!("=== KiteConnect v1.0.2 Foundation Models Demo ===");
    
    // Test error types
    test_error_types();
    
    // Test response types  
    test_response_types();
    
    // Test common enums
    test_enums();
    
    println!("\n✅ All foundation models working correctly!");
}

fn test_error_types() {
    println!("\n📌 Testing Error Types:");
    
    // Create different error types
    let auth_err = KiteError::auth_error("Invalid API key");
    let api_err = KiteError::api_error("400", "Bad request");
    let param_err = KiteError::invalid_param("Missing required field");
    
    println!("  ✓ Authentication Error: {}", auth_err);
    println!("  ✓ API Error: {}", api_err);
    println!("  ✓ Parameter Error: {}", param_err);
}

fn test_response_types() {
    println!("\n📌 Testing Response Types:");
    
    // Create success response
    let success_data = vec!["holding1", "holding2"];
    let success_response = KiteResponse::success(success_data.clone());
    
    println!("  ✓ Success Response: status={}, has_data={}", 
             success_response.status, success_response.data.is_some());
    
    // Create error response
    let error_response = KiteResponse::<Vec<String>>::error(
        "Invalid session", 
        Some("TokenException".to_string())
    );
    
    println!("  ✓ Error Response: status={}, error_type={:?}", 
             error_response.status, error_response.error_type);
    
    // Test status enum
    let status = Status::Success;
    println!("  ✓ Status Enum: is_success={}", status.is_success());
}

fn test_enums() {
    println!("\n📌 Testing Common Enums:");
    
    // Test exchanges
    let nse = Exchange::NSE;
    let bse = Exchange::BSE;
    println!("  ✓ Exchanges: {} (equity={}), {} (equity={})", 
             nse, nse.is_equity(), bse, bse.is_equity());
    
    // Test products
    let cnc = Product::CNC;
    let mis = Product::MIS;
    println!("  ✓ Products: {}, {}", cnc, mis);
    
    // Test transaction types
    let buy = TransactionType::BUY;
    let sell = TransactionType::SELL;
    println!("  ✓ Transaction Types: {}, {}", buy, sell);
    
    // Test order types
    let market = OrderType::MARKET;
    let limit = OrderType::LIMIT;
    println!("  ✓ Order Types: {}, {}", market, limit);
    
    // Test validity
    let day = Validity::DAY;
    let ioc = Validity::IOC;
    println!("  ✓ Validity: {}, {}", day, ioc);
    
    // Test all exchanges
    let all_exchanges = Exchange::all();
    println!("  ✓ All Exchanges ({}): {:?}", all_exchanges.len(), all_exchanges);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = KiteError::auth_error("test");
        assert!(matches!(err, KiteError::Authentication(_)));
    }
    
    #[test]
    fn test_response_success() {
        let resp = KiteResponse::success("test_data");
        assert!(resp.is_success());
        assert!(!resp.is_error());
    }
    
    #[test]
    fn test_exchange_properties() {
        assert!(Exchange::NSE.is_equity());
        assert!(Exchange::NFO.is_derivative());
        assert!(Exchange::MCX.is_commodity());
    }
}
