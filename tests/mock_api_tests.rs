use kiteconnect_async_wasm::connect::{KiteConnect, KiteConnectConfig};
use mockito::{Matcher, Server};

// Helper: create a client pointed at a given base_url
fn mock_client(base_url: String) -> KiteConnect {
    let mut cfg = KiteConnectConfig {
        base_url,
        enable_rate_limiting: false,
        ..Default::default()
    };
    // Keep retries small to make failures fast in tests
    cfg.retry_config.max_retries = 0;
    let mut client = KiteConnect::new_with_config("test_key", cfg);
    client.set_access_token("test_token");
    client
}

#[tokio::test]
async fn mock_end_to_end_basic() {
    // Start isolated mock server
    let mut server = Server::new_async().await;

    // Load mock payloads from the repo-synced mocks/ folder
    let instruments_csv = include_str!("../mocks/instruments.csv");
    let mf_instruments_csv = include_str!("../mocks/mf_instruments.csv");
    let holdings_json = include_str!("../mocks/holdings.json");
    let positions_json = include_str!("../mocks/positions.json");
    let orders_json = include_str!("../mocks/orders.json");
    let trades_json = include_str!("../mocks/trades.json");
    let trigger_range_json = include_str!("../mocks/trigger_range.json");

    // Serve /instruments as gzip to cover gzip path
    let gz_instruments_body: Vec<u8> = {
        use flate2::write::GzEncoder;
        use std::io::Write;
        let mut enc = GzEncoder::new(Vec::new(), flate2::Compression::default());
        enc.write_all(instruments_csv.as_bytes()).unwrap();
        enc.finish().unwrap()
    };

    let _m_instruments = server
        .mock("GET", "/instruments")
        .with_header("content-type", "text/csv; charset=utf-8")
        .with_header("content-encoding", "gzip")
        .with_body(gz_instruments_body)
        .create_async()
        .await;

    // Serve /instruments/NSE (optional extra route to ensure path segments work)
    let _m_instruments_nse = server
        .mock("GET", "/instruments/NSE")
        .with_header("content-type", "text/csv; charset=utf-8")
        .with_body(instruments_csv)
        .create_async()
        .await;

    // Serve /mf/instruments (plain csv)
    let _m_mf_instruments = server
        .mock("GET", "/mf/instruments")
        .with_header("content-type", "text/csv; charset=utf-8")
        .with_body(mf_instruments_csv)
        .create_async()
        .await;

    // Portfolio endpoints
    let _m_holdings = server
        .mock("GET", "/portfolio/holdings")
        .with_header("content-type", "application/json")
        .with_body(holdings_json)
        .create_async()
        .await;

    let _m_positions = server
        .mock("GET", "/portfolio/positions")
        .with_header("content-type", "application/json")
        .with_body(positions_json)
        .create_async()
        .await;

    // Orders + trades
    let _m_orders = server
        .mock("GET", "/orders")
        .with_header("content-type", "application/json")
        .with_body(orders_json)
        .create_async()
        .await;

    let _m_trades = server
        .mock("GET", "/trades")
        .with_header("content-type", "application/json")
        .with_body(trades_json)
        .create_async()
        .await;

    // Trigger range endpoint: allow any instrument query, match path segment for transaction type
    let _m_trigger_range_buy = server
        .mock("GET", Matcher::Regex(r"^/instruments/trigger_range/BUY$".into()))
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(trigger_range_json)
        .create_async()
        .await;

    let client = mock_client(server.url());

    // Call instruments() (legacy JSON)
    let instruments = client
        .instruments(None)
        .await
        .expect("instruments() should parse CSV");
    assert!(instruments.as_array().map(|a| !a.is_empty()).unwrap_or(false));

    // Call typed instruments_typed()
    let instruments_typed = client
        .instruments_typed(None)
        .await
        .expect("instruments_typed() should parse CSV into models");
    assert!(!instruments_typed.is_empty());

    // MF instruments legacy + typed
    let mf_json = client
        .mf_instruments()
        .await
        .expect("mf_instruments() ok");
    assert!(mf_json.as_array().map(|a| !a.is_empty()).unwrap_or(false));

    let mf_typed = client
        .mf_instruments_typed()
        .await
        .expect("mf_instruments_typed() ok");
    assert!(!mf_typed.is_empty());

    // Portfolio endpoints
    let _h = client.holdings().await.expect("holdings() ok");
    let _p = client.positions().await.expect("positions() ok");

    // Orders/trades
    let _o = client.orders().await.expect("orders() ok");
    let _t = client.trades().await.expect("trades() ok");

    // Trigger range
    let _tr = client
        .trigger_range("BUY", vec!["NSE:INFY"]) // query is mocked with Any matcher
        .await
        .expect("trigger_range() ok");
}

#[tokio::test]
async fn mock_quotes_historical_and_margins() {
    use chrono::NaiveDateTime;
    use kiteconnect_async_wasm::models::common::{Exchange, Interval, OrderType, Product, TransactionType, Variety};
    use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
    use kiteconnect_async_wasm::models::orders::OrderMarginRequest;
    use serde_json::json;

    let mut server = Server::new_async().await;

    // Quotes: /quote
    let quote_payload = json!({
        "status": "success",
        "data": [
            {
                "instrument_token": 738561,
                "tradingsymbol": "RELIANCE",
                "exchange": "NSE",
                "timestamp": "2024-12-20T03:45:00Z",
                "last_price": 2500.5,
                "last_quantity": 10,
                "last_trade_time": "2024-12-20T03:45:00Z",
                "average_price": 2490.0,
                "volume": 100000,
                "buy_quantity": 50000,
                "sell_quantity": 50000,
                "oi": 123456,
                "oi_day_high": 130000,
                "oi_day_low": 120000,
                "net_change": 5.5,
                "lower_circuit_limit": 2000.0,
                "upper_circuit_limit": 3000.0,
                "ohlc": { "open": 2480.0, "high": 2520.0, "low": 2470.0, "close": 2495.0 },
                "depth": {
                    "buy": [ { "price": 2500.0, "quantity": 100, "orders": 10 } ],
                    "sell": [ { "price": 2501.0, "quantity": 100, "orders": 8 } ]
                }
            }
        ]
    })
    .to_string();

    let _m_quote = server
        .mock("GET", "/quote")
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(quote_payload)
        .create_async()
        .await;

    // OHLC: /quote/ohlc
    let ohlc_payload = json!({
        "status": "success",
        "data": [ { "open": 2480.0, "high": 2520.0, "low": 2470.0, "close": 2500.5 } ]
    })
    .to_string();

    let _m_ohlc = server
        .mock("GET", "/quote/ohlc")
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(ohlc_payload)
        .create_async()
        .await;

    // LTP: /quote/ltp
    let ltp_payload = json!({
        "status": "success",
        "data": [ { "instrument_token": 738561, "last_price": 2500.5 } ]
    })
    .to_string();

    let _m_ltp = server
        .mock("GET", "/quote/ltp")
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(ltp_payload)
        .create_async()
        .await;

    // Historical: /instruments/historical/{token}/{interval}
    let historical_payload = json!({
        "status": "success",
        "data": { "candles": [["2024-12-20T09:15:00+0530", 2480.0, 2520.0, 2470.0, 2500.5, 100000]] }
    })
    .to_string();

    let _m_hist = server
        .mock(
            "GET",
            Matcher::Regex(r"^/instruments/historical/738561/(minute|day|3minute|5minute|10minute|15minute|30minute|60minute)$".into()),
        )
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(historical_payload)
        .create_async()
        .await;

    // Margins (legacy): /user/margins
    let _m_margins = server
        .mock("GET", "/user/margins")
        .with_header("content-type", "application/json")
        .with_body(include_str!("../mocks/margins.json"))
        .create_async()
        .await;

    // Order margins (typed): POST /margins/orders
    let order_margins_payload = json!({
        "status": "success",
        "data": [
            {
                "type": "equity",
                "tradingsymbol": "RELIANCE",
                "exchange": "NSE",
                "span": 0.0,
                "exposure": 0.0,
                "option_premium": 0.0,
                "additional": 0.0,
                "bo": 0.0,
                "cash": 2500.5,
                "var": 0.0,
                "pnl": { "realised": 0.0, "unrealised": 0.0 },
                "leverage": 1.0,
                "charges": {
                    "total": 10.0,
                    "transaction_tax": 2.0,
                    "transaction_tax_type": "stt",
                    "exchange_turnover_charge": 1.5,
                    "sebi_turnover_charge": 0.1,
                    "brokerage": 5.0,
                    "stamp_duty": 1.4,
                    "gst": {
                        "igst": 0.0,
                        "cgst": 0.0,
                        "sgst": 0.0,
                        "total": 0.0
                    }
                },
                "total": 2510.5
            }
        ]
    })
    .to_string();

    let _m_order_margins = server
        .mock("POST", "/margins/orders")
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(order_margins_payload)
        .create_async()
        .await;

    // Basket margins (typed): POST /margins/basket
    let order_result = json!({
        "type": "equity",
        "tradingsymbol": "RELIANCE",
        "exchange": "NSE",
        "span": 0.0,
        "exposure": 0.0,
        "option_premium": 0.0,
        "additional": 0.0,
        "bo": 0.0,
        "cash": 2500.5,
        "var": 0.0,
        "pnl": { "realised": 0.0, "unrealised": 0.0 },
        "leverage": 1.0,
        "charges": {
            "total": 10.0,
            "transaction_tax": 2.0,
            "transaction_tax_type": "stt",
            "exchange_turnover_charge": 1.5,
            "sebi_turnover_charge": 0.1,
            "brokerage": 5.0,
            "stamp_duty": 1.4,
            "gst": {
                "igst": 0.0,
                "cgst": 0.0,
                "sgst": 0.0,
                "total": 0.0
            }
        },
        "total": 2510.5
    });
    let charges = order_result["charges"].clone();
    let basket_margins_payload = json!({
        "status": "success",
        "data": {
            "initial": order_result,
            "final_": {
                "type": "equity",
                "tradingsymbol": "RELIANCE",
                "exchange": "NSE",
                "span": 0.0,
                "exposure": 0.0,
                "option_premium": 0.0,
                "additional": 0.0,
                "bo": 0.0,
                "cash": 2500.5,
                "var": 0.0,
                "pnl": { "realised": 0.0, "unrealised": 0.0 },
                "leverage": 1.0,
                "charges": charges,
                "total": 2510.5
            },
            "orders": [
                {
                    "type": "equity",
                    "tradingsymbol": "RELIANCE",
                    "exchange": "NSE",
                    "span": 0.0,
                    "exposure": 0.0,
                    "option_premium": 0.0,
                    "additional": 0.0,
                    "bo": 0.0,
                    "cash": 2500.5,
                    "var": 0.0,
                    "pnl": { "realised": 0.0, "unrealised": 0.0 },
                    "leverage": 1.0,
                    "charges": {
                        "total": 10.0,
                        "transaction_tax": 2.0,
                        "transaction_tax_type": "stt",
                        "exchange_turnover_charge": 1.5,
                        "sebi_turnover_charge": 0.1,
                        "brokerage": 5.0,
                        "stamp_duty": 1.4,
                        "gst": {
                            "igst": 0.0,
                            "cgst": 0.0,
                            "sgst": 0.0,
                            "total": 0.0
                        }
                    },
                    "total": 2510.5
                }
            ],
            "charges": {
                "total": 10.0,
                "transaction_tax": 2.0,
                "transaction_tax_type": "stt",
                "exchange_turnover_charge": 1.5,
                "sebi_turnover_charge": 0.1,
                "brokerage": 5.0,
                "stamp_duty": 1.4,
                "gst": {
                    "igst": 0.0,
                    "cgst": 0.0,
                    "sgst": 0.0,
                    "total": 0.0
                }
            }
        }
    })
    .to_string();

    let _m_basket_margins = server
        .mock("POST", "/margins/basket")
        .match_query(Matcher::Any)
        .with_header("content-type", "application/json")
        .with_body(basket_margins_payload)
        .create_async()
        .await;

    // Create client
    let client = mock_client(server.url());

    // Quotes typed
    let quotes = client
        .quote_typed(vec!["NSE:RELIANCE"]) // query ignored by mock
        .await
        .expect("quote_typed ok");
    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].trading_symbol, "RELIANCE");

    // OHLC typed
    let ohlc = client
        .ohlc_typed(vec!["NSE:RELIANCE"]) // query ignored by mock
        .await
        .expect("ohlc_typed ok");
    assert_eq!(ohlc.len(), 1);

    // LTP typed
    let ltp = client
        .ltp_typed(vec!["NSE:RELIANCE"]) // query ignored by mock
        .await
        .expect("ltp_typed ok");
    assert_eq!(ltp.len(), 1);

    // Historical typed
    let req = HistoricalDataRequest::new(
        738561,
        NaiveDateTime::parse_from_str("2024-12-20 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        NaiveDateTime::parse_from_str("2024-12-20 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        Interval::Day,
    );
    let hist = client
        .historical_data_typed(req)
        .await
        .expect("historical_data_typed ok");
    assert_eq!(hist.candles.len(), 1);

    // Margins (legacy): /user/margins
    let margins = client.margins(None).await.expect("margins() ok");
    assert!(margins["data"].is_object());

    // Order margins (typed)
    let order_req = OrderMarginRequest {
        exchange: Exchange::NSE,
        trading_symbol: "RELIANCE".into(),
        transaction_type: TransactionType::BUY,
        variety: Variety::Regular,
        product: Product::CNC,
        order_type: OrderType::LIMIT,
        quantity: 1,
        price: Some(2500.5),
        trigger_price: None,
    };
    let order_margins = client
        .calculate_order_margins_typed(&[order_req.clone()], true)
        .await
        .expect("order margins typed ok");
    assert_eq!(order_margins.len(), 1);

    // Basket margins (typed)
    let basket_margins = client
        .calculate_basket_margins_typed(&[order_req], false, true)
        .await
        .expect("basket margins typed ok");
    assert!(!basket_margins.orders.is_empty());
}
