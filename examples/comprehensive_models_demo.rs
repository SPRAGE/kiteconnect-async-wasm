use kiteconnect_async_wasm::model::*;
use std::collections::HashMap;
use chrono::Utc;

fn main() {
    println!("🚀 KiteConnect Async WASM - Comprehensive Model Demo");
    println!("=====================================================\n");

    // 1. Constants demo
    println!("📊 Constants:");
    println!("  Product CNC: {}", products::CNC);
    println!("  Order Type LIMIT: {}", order_types::LIMIT);
    println!("  Exchange NSE: {}", exchanges::NSE);
    println!("  Transaction Type BUY: {}\n", transaction_types::BUY);

    // 2. User Session demo
    println!("👤 User Session:");
    let user_session = UserSession {
        user_id: "AB1234".to_string(),
        user_name: "John Doe".to_string(),
        user_shortname: "John".to_string(),
        avatar_url: Some("https://example.com/avatar.png".to_string()),
        user_type: "individual".to_string(),
        email: "john@example.com".to_string(),
        broker: "ZERODHA".to_string(),
        meta: UserMeta {
            demat_consent: "consent".to_string(),
        },
        products: vec![products::CNC.to_string(), products::MIS.to_string()],
        order_types: vec![order_types::MARKET.to_string(), order_types::LIMIT.to_string()],
        exchanges: vec![exchanges::NSE.to_string(), exchanges::BSE.to_string()],
        api_key: "test_api_key".to_string(),
        access_token: "test_access_token".to_string(),
        public_token: "test_public_token".to_string(),
        refresh_token: Some("test_refresh_token".to_string()),
        login_time: "2024-01-15 09:15:00".to_string(),
    };
    println!("  User ID: {}", user_session.user_id);
    println!("  Name: {}", user_session.user_name);
    println!("  Email: {}\n", user_session.email);

    // 3. Order demo
    println!("📋 Order Management:");
    let order = Order {
        account_id: "AB1234".to_string(),
        placed_by: "AB1234".to_string(),
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
        meta: HashMap::new(),
        exchange: exchanges::NSE.to_string(),
        trading_symbol: "RELIANCE".to_string(),
        instrument_token: 738561,
        order_type: order_types::LIMIT.to_string(),
        transaction_type: transaction_types::BUY.to_string(),
        validity: validities::DAY.to_string(),
        validity_ttl: 0,
        product: products::CNC.to_string(),
        quantity: 10.0,
        disclosed_quantity: 0.0,
        price: 2500.0,
        trigger_price: 0.0,
        average_price: 2500.0,
        filled_quantity: 10.0,
        pending_quantity: 0.0,
        cancelled_quantity: 0.0,
        auction_number: None,
        tag: Some("demo_order".to_string()),
        tags: Some(vec!["demo".to_string()]),
    };
    println!("  Order ID: {}", order.order_id);
    println!("  Symbol: {}", order.trading_symbol);
    println!("  Status: {}", order.status);
    println!("  Is Complete: {}\n", order.is_complete());

    // 4. Portfolio demo
    println!("💼 Portfolio:");
    let holding = Holding {
        tradingsymbol: "RELIANCE".to_string(),
        exchange: exchanges::NSE.to_string(),
        instrument_token: 738561,
        isin: "INE002A01018".to_string(),
        product: products::CNC.to_string(),
        price: 2500.0,
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
        average_price: 2400.0,
        last_price: 2500.0,
        close_price: 2480.0,
        pnl: 10000.0,
        day_change: 20.0,
        day_change_percentage: 0.81,
        mtf: MTFHolding {
            quantity: 0,
            used_quantity: 0,
            average_price: 0.0,
            value: 0.0,
            initial_margin: 0.0,
        },
    };
    println!("  Symbol: {}", holding.tradingsymbol);
    println!("  Quantity: {}", holding.quantity);
    println!("  Market Value: {}", holding.market_value());
    println!("  P&L: {}\n", holding.pnl);

    // 5. Market Data demo
    println!("📈 Market Data:");
    let quote_data = QuoteData {
        instrument_token: 738561,
        timestamp: Utc::now(),
        last_price: 2500.0,
        last_quantity: 50,
        last_trade_time: Utc::now(),
        average_price: 2495.0,
        volume: 1000000,
        buy_quantity: 5000,
        sell_quantity: 4500,
        ohlc: OHLC {
            instrument_token: Some(738561),
            open: 2480.0,
            high: 2510.0,
            low: 2470.0,
            close: 2500.0,
        },
        net_change: 20.0,
        oi: 0.0,
        oi_day_high: 0.0,
        oi_day_low: 0.0,
        lower_circuit_limit: 2232.0,
        upper_circuit_limit: 2728.0,
        depth: Depth {
            buy: [
                DepthItem { price: 2499.0, quantity: 100, orders: 5 },
                DepthItem { price: 2498.0, quantity: 200, orders: 8 },
                DepthItem { price: 2497.0, quantity: 150, orders: 6 },
                DepthItem { price: 2496.0, quantity: 300, orders: 12 },
                DepthItem { price: 2495.0, quantity: 250, orders: 10 },
            ],
            sell: [
                DepthItem { price: 2500.0, quantity: 120, orders: 6 },
                DepthItem { price: 2501.0, quantity: 180, orders: 7 },
                DepthItem { price: 2502.0, quantity: 160, orders: 8 },
                DepthItem { price: 2503.0, quantity: 220, orders: 9 },
                DepthItem { price: 2504.0, quantity: 200, orders: 8 },
            ],
        },
    };
    println!("  Last Price: {}", quote_data.last_price);
    println!("  Volume: {}", quote_data.volume);
    println!("  OHLC: O:{} H:{} L:{} C:{}", 
             quote_data.ohlc.open, quote_data.ohlc.high, 
             quote_data.ohlc.low, quote_data.ohlc.close);
    println!("  Has Buying Pressure: {}\n", quote_data.has_buying_pressure());

    // 6. Ticker demo
    println!("⚡ Real-time Ticker:");
    let tick = Tick {
        mode: "full".to_string(),
        instrument_token: 738561,
        is_tradable: true,
        is_index: false,
        timestamp: ticker::Time { time: Utc::now() },
        last_trade_time: ticker::Time { time: Utc::now() },
        last_price: 2500.0,
        last_traded_quantity: 50,
        total_buy_quantity: 5000,
        total_sell_quantity: 4500,
        total_buy: 1200000,
        total_sell: 1100000,
        volume_traded: 1000000,
        ohlc: OHLC {
            instrument_token: Some(738561),
            open: 2480.0,
            high: 2510.0,
            low: 2470.0,
            close: 2500.0,
        },
        oi: 0,
        oi_day_high: 0,
        oi_day_low: 0,
        net_change: 20.0,
        average_trade_price: 2495.0,
        depth: Depth {
            buy: [
                DepthItem { price: 2499.0, quantity: 100, orders: 5 },
                DepthItem { price: 2498.0, quantity: 200, orders: 8 },
                DepthItem { price: 2497.0, quantity: 150, orders: 6 },
                DepthItem { price: 2496.0, quantity: 300, orders: 12 },
                DepthItem { price: 2495.0, quantity: 250, orders: 10 },
            ],
            sell: [
                DepthItem { price: 2500.0, quantity: 120, orders: 6 },
                DepthItem { price: 2501.0, quantity: 180, orders: 7 },
                DepthItem { price: 2502.0, quantity: 160, orders: 8 },
                DepthItem { price: 2503.0, quantity: 220, orders: 9 },
                DepthItem { price: 2504.0, quantity: 200, orders: 8 },
            ],
        },
    };
    println!("  Mode: {}", tick.mode);
    println!("  Last Price: {}", tick.last_price);
    println!("  OHLC Is Bullish: {}", tick.ohlc.is_bullish());
    println!("  Spread (estimate): {:?}\n", tick.depth.spread());

    // 7. Margin Calculation demo
    println!("💰 Margin Calculation:");
    let order_margin_param = OrderMarginParam {
        exchange: exchanges::NSE.to_string(),
        tradingsymbol: "RELIANCE".to_string(),
        transaction_type: transaction_types::BUY.to_string(),
        variety: varieties::REGULAR.to_string(),
        product: products::CNC.to_string(),
        order_type: order_types::LIMIT.to_string(),
        quantity: 10.0,
        price: Some(2500.0),
        trigger_price: None,
    };
    println!("  Symbol: {}", order_margin_param.tradingsymbol);
    println!("  Quantity: {}", order_margin_param.quantity);
    println!("  Price: {:?}\n", order_margin_param.price);

    // 8. GTT demo
    println!("🎯 GTT Orders:");
    let gtt_condition = GTTCondition {
        exchange: exchanges::NSE.to_string(),
        tradingsymbol: "RELIANCE".to_string(),
        last_price: 2500.0,
        trigger_values: vec![2400.0, 2600.0],
    };
    println!("  Symbol: {}", gtt_condition.tradingsymbol);
    println!("  Current Price: {}", gtt_condition.last_price);
    println!("  Trigger Values: {:?}\n", gtt_condition.trigger_values);

    // 9. Mutual Funds demo
    println!("🏦 Mutual Funds:");
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
        placed_by: "AB1234".to_string(),
        tag: "sip".to_string(),
    };
    println!("  Fund: {}", mf_order.fund);
    println!("  Amount: {}", mf_order.amount);
    println!("  Status: {}", mf_order.status);
    println!("  Total Value: ₹{:.2}\n", mf_order.amount);

    println!("✅ All models are working correctly!");
    println!("📚 The Rust KiteConnect library now has comprehensive model coverage matching the Go implementation.");
}
