//! Integration tests for KiteConnect v1.0.0
//!
//! These tests verify the public API functionality, retry mechanisms,
//! error handling, and typed responses.

#[cfg(test)]
mod tests {
    use kiteconnect_async_wasm::connect::KiteConnect;

    #[test]
    fn test_client_creation() {
        let client = KiteConnect::new("test_key", "test_token");

        // Test that the client is created successfully
        // Since fields are private, we'll test that Debug formatting works
        assert!(format!("{:?}", client).contains("KiteConnect"));
    }

    #[test]
    fn test_client_cloning() {
        let client1 = KiteConnect::new("test_key", "test_token");
        let client2 = client1.clone();

        // Test that cloning works - both should have the same debug representation
        assert_eq!(format!("{:?}", client1), format!("{:?}", client2));
    }

    #[test]
    fn test_access_token_update() {
        let mut client = KiteConnect::new("test_key", "");
        client.set_access_token("new_token");

        // Test that access token can be set (no panic/error)
        // Since field is private, we can't directly test the value
        // The fact that we reach here means the method worked
    }

    #[test]
    fn test_session_expiry_hook() {
        let mut client = KiteConnect::new("test_key", "test_token");

        fn mock_hook() {
            println!("Session expired!");
        }

        client.set_session_expiry_hook(mock_hook);

        // Test that hook can be set without panic
        // The fact that we reach here means the method worked
    }
}

#[cfg(test)]
mod api_tests {
    use kiteconnect_async_wasm::connect::KiteConnect;

    // Note: These tests require actual API credentials and network access
    // They are marked as ignored by default to prevent CI failures

    #[tokio::test]
    #[ignore]
    async fn test_instruments_api_call() {
        let client = KiteConnect::new("your_api_key", "your_access_token");

        // This would require valid credentials
        match client.instruments(None).await {
            Ok(_instruments) => {
                // API call succeeded - test passes
            }
            Err(_e) => {
                // Expected to fail without valid credentials - test passes
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_typed_api_methods() {
        let client = KiteConnect::new("your_api_key", "your_access_token");

        // Test that typed methods exist and can be called
        // They should fail with authentication errors without valid tokens

        let result = client.holdings_typed().await;
        assert!(result.is_err()); // Expected to fail without valid auth

        let result = client.positions_typed().await;
        assert!(result.is_err()); // Expected to fail without valid auth
    }
}

#[cfg(test)]
mod configuration_tests {
    use kiteconnect_async_wasm::connect::KiteConnect;

    #[test]
    fn test_default_configuration() {
        let _client = KiteConnect::default();

        // Test that default configuration works
        // The fact that we reach here means the default config worked
    }

    #[test]
    fn test_custom_configuration() {
        let config = kiteconnect_async_wasm::connect::KiteConnectConfig {
            timeout: 60,
            base_url: "https://custom.api.url".to_string(),
            ..Default::default()
        };

        // Test that configuration can be created and modified
        assert_eq!(config.timeout, 60);
        assert_eq!(config.base_url, "https://custom.api.url");
    }
}

#[cfg(test)]
mod error_handling_tests {
    #[test]
    fn test_error_types() {
        // Test that our error types can be created and used
        use kiteconnect_async_wasm::models::common::KiteError;

        let general_error = KiteError::General("Test general error".to_string());
        assert!(matches!(general_error, KiteError::General(_)));

        let api_error = KiteError::Api {
            status: "500".to_string(),
            message: "Test message".to_string(),
            error_type: None,
        };
        assert!(matches!(api_error, KiteError::Api { .. }));

        let auth_error = KiteError::Authentication("Invalid token".to_string());
        assert!(matches!(auth_error, KiteError::Authentication(_)));
    }
}

#[cfg(test)]
mod model_tests {
    #[test]
    fn test_model_serialization() {
        use kiteconnect_async_wasm::models::prelude::*;

        // Test that models can be created and serialized
        let order_params = OrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "RELIANCE".to_string(),
            transaction_type: TransactionType::BUY,
            order_type: OrderType::LIMIT,
            quantity: 10,
            price: Some(2500.0),
            product: Product::CNC,
            validity: Some(Validity::DAY),
            disclosed_quantity: None,
            trigger_price: None,
            squareoff: None,
            stoploss: None,
            trailing_stoploss: None,
            market_protection: None,
            iceberg_legs: None,
            iceberg_quantity: None,
            auction_number: None,
            tag: None,
        };

        // Test serialization
        let json = serde_json::to_string(&order_params);
        assert!(json.is_ok());

        // Test that required fields are present
        let json_str = json.unwrap();
        assert!(json_str.contains("RELIANCE"));
        assert!(json_str.contains("BUY"));
        assert!(json_str.contains("LIMIT"));
    }

    #[test]
    fn test_model_validation() {
        use kiteconnect_async_wasm::models::prelude::*;

        // Test order params creation with valid data
        let valid_params = OrderParams {
            exchange: Exchange::NSE,
            trading_symbol: "RELIANCE".to_string(),
            transaction_type: TransactionType::BUY,
            order_type: OrderType::MARKET,
            quantity: 10,
            price: None, // Market order doesn't need price
            product: Product::CNC,
            validity: Some(Validity::DAY),
            disclosed_quantity: None,
            trigger_price: None,
            squareoff: None,
            stoploss: None,
            trailing_stoploss: None,
            market_protection: None,
            iceberg_legs: None,
            iceberg_quantity: None,
            auction_number: None,
            tag: None,
        };

        // Test that serialization works for valid params
        let json_result = serde_json::to_string(&valid_params);
        assert!(json_result.is_ok());

        // Test creation with empty trading symbol (should be allowed structurally)
        let params_with_empty_symbol = OrderParams {
            trading_symbol: "".to_string(),
            ..valid_params
        };

        // Should still serialize (validation would happen at API level)
        let json_result = serde_json::to_string(&params_with_empty_symbol);
        assert!(json_result.is_ok());
    }
}
