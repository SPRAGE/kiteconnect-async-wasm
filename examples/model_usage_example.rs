//! Example demonstrating the use of KiteConnect models
//! 
//! This example shows how to use the various model types
//! that are available in the kiteconnect-async-wasm library.

use kiteconnect_async_wasm::model::*;
use chrono::Utc;

fn main() {
    println!("KiteConnect Models Example");
    println!("==========================");

    // Example: User Session Model
    println!("\n1. User Session Model:");
    let user_session = UserSession {
        user_id: "USER123".to_string(),
        user_name: "John Doe".to_string(),
        user_shortname: "John".to_string(),
        avatar_url: Some("https://example.com/avatar.png".to_string()),
        user_type: "individual".to_string(),
        email: "john@example.com".to_string(),
        broker: "ZERODHA".to_string(),
        meta: UserMeta { demat_consent: "consent".to_string() },
        exchanges: vec![exchanges::NSE.to_string(), exchanges::BSE.to_string()],
        products: vec![products::CNC.to_string(), products::MIS.to_string()],
        order_types: vec![order_types::MARKET.to_string(), order_types::LIMIT.to_string()],
        api_key: "api_key".to_string(),
        access_token: "access_token".to_string(),
        public_token: "public_token".to_string(),
        refresh_token: Some("refresh_token".to_string()),
        login_time: chrono::Utc::now() - chrono::Duration::hours(2), // 2 hours ago
    };
    println!("User: {} ({})", user_session.user_name, user_session.user_id);

    // Example: Portfolio models
    println!("\n2. Portfolio Models:");
    let holding = Holding {
        tradingsymbol: "RELIANCE".to_string(),
        exchange: exchanges::NSE.to_string(),
        instrument_token: 738561,
        isin: "INE002A01018".to_string(),
        product: products::CNC.to_string(),
        price: 2550.0,
        used_quantity: 0,
        quantity: 100,
        t1_quantity: 0,
        realised_quantity: 100,
        authorised_quantity: 100,
        authorised_date: Utc::now(),
        opening_quantity: 100,
        collateral_quantity: 0,
        collateral_type: "".to_string(),
        discrepancy: false,
        average_price: 2500.0,
        last_price: 2550.0,
        close_price: 2520.0,
        pnl: 5000.0,
        day_change: 30.0,
        day_change_percentage: 1.19,
        mtf: MTFHolding {
            quantity: 0,
            used_quantity: 0,
            average_price: 0.0,
            value: 0.0,
            initial_margin: 0.0,
        },
    };
    println!("Holding: {} x {} @ ₹{}", holding.quantity, holding.tradingsymbol, holding.average_price);

    // Example: Order models
    println!("\n3. Order Models:");
    let order = Order {
        account_id: "USER123".to_string(),
        placed_by: "USER123".to_string(),
        order_id: "240115000001".to_string(),
        exchange_order_id: "NSE240115000001".to_string(),
        parent_order_id: None,
        status: "COMPLETE".to_string(),
        status_message: "Order executed".to_string(),
        status_message_raw: "Order executed".to_string(),
        order_timestamp: Utc::now(),
        exchange_update_timestamp: Utc::now(),
        exchange_timestamp: Utc::now(),
        variety: varieties::REGULAR.to_string(),
        modified: false,
        meta: std::collections::HashMap::new(),
        exchange: exchanges::NSE.to_string(),
        trading_symbol: "SBIN".to_string(),
        instrument_token: 779521,
        order_type: order_types::LIMIT.to_string(),
        transaction_type: transaction_types::BUY.to_string(),
        validity: validities::DAY.to_string(),
        validity_ttl: 0,
        product: products::CNC.to_string(),
        quantity: 50.0,
        disclosed_quantity: 0.0,
        price: 500.0,
        trigger_price: 0.0,
        average_price: 500.0,
        filled_quantity: 50.0,
        pending_quantity: 0.0,
        cancelled_quantity: 0.0,
        auction_number: None,
        tag: Some("demo_order".to_string()),
        tags: Some(vec!["demo".to_string()]),
    };
    println!("Order: {} {} x {} @ ₹{}", 
             order.transaction_type, 
             order.quantity, 
             order.trading_symbol, 
             order.price);

    // Example: Market Data models
    println!("\n4. Market Data Models:");
    let instrument = Instrument {
        instrument_token: 738561,
        exchange_token: 2885,
        tradingsymbol: "RELIANCE".to_string(),
        name: "Reliance Industries Limited".to_string(),
        last_price: 2550.0,
        expiry: None,
        strike_price: 0.0,
        tick_size: 0.05,
        lot_size: 1.0,
        instrument_type: "EQ".to_string(),
        segment: "NSE".to_string(),
        exchange: "NSE".to_string(),
    };
    println!("Instrument: {} - {} (Token: {})", 
             instrument.tradingsymbol, 
             instrument.name, 
             instrument.instrument_token);

    // Example: Constants
    println!("\n5. Constants:");
    println!("Order Type LIMIT: {}", order_types::LIMIT);
    println!("Transaction Type BUY: {}", transaction_types::BUY);
    println!("Product Type CNC: {}", products::CNC);
    println!("Exchange NSE: {}", exchanges::NSE);
    println!("Validity DAY: {}", validities::DAY);

    // Example: Mutual Fund models
    println!("\n6. Mutual Fund Models:");
    let mf_order = MFOrder {
        order_id: "MF240115000001".to_string(),
        exchange_order_id: "".to_string(),
        tradingsymbol: "INF090I01239".to_string(),
        status: "COMPLETE".to_string(),
        status_message: "Order executed".to_string(),
        folio: "123456789".to_string(),
        fund: "HDFC Equity Fund".to_string(),
        order_timestamp: Utc::now(),
        exchange_timestamp: Utc::now(),
        settlement_id: "20240115".to_string(),
        transaction_type: transaction_types::BUY.to_string(),
        variety: varieties::REGULAR.to_string(),
        purchase_type: "FRESH".to_string(),
        quantity: 0.0,
        amount: 5000.0,
        last_price: 125.50,
        average_price: 125.50,
        placed_by: "USER123".to_string(),
        tag: "sip".to_string(),
    };
    println!("MF Order: {} - ₹{} - {}", mf_order.fund, mf_order.amount, mf_order.status);

    println!("\n✅ All models are working correctly!");
    println!("\nYou can now use these models in your application:");
    println!("use kiteconnect_async_wasm::model::{{Holding, Order, Instrument}};");
    println!("use kiteconnect_async_wasm::model::{{transaction_types, order_types, products}};");
    println!("use kiteconnect_async_wasm::model::MFOrder;");
}
