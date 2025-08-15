//! Margin and Charges Calculation Example
//!
//! Demonstrates usage of official margin and virtual contract note (charges) APIs.
//!
//! Usage:
//!   export KITE_API_KEY="your_key"
//!   export KITE_ACCESS_TOKEN="your_token"
//!   cargo run --example margin_calculation_example --features=native

use kiteconnect_async_wasm::connect::KiteConnect;
use kiteconnect_async_wasm::models::common::{Exchange, OrderType, Product, TransactionType, Variety};
use kiteconnect_async_wasm::models::orders::{OrderChargesRequest, OrderMarginRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Credentials
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    let client = KiteConnect::new(&api_key, &access_token);

    // Sample order for RELIANCE on NSE
    let orders = vec![OrderMarginRequest {
        exchange: Exchange::NSE,
        trading_symbol: "RELIANCE".into(),
        transaction_type: TransactionType::BUY,
        variety: Variety::Regular,
        product: Product::CNC,
        order_type: OrderType::MARKET,
        quantity: 1,
        price: None,
        trigger_price: None,
    }];

    println!("\n== Margins for single order ==");
    let margin_items = client
        .calculate_order_margins_typed(&orders, true)
        .await?;
    for (i, item) in margin_items.iter().enumerate() {
        println!(
            "#{} {}@{} total margin: {:.2} (span: {:.2}, exposure: {:.2})",
            i + 1,
            item.trading_symbol,
            item.exchange,
            item.total,
            item.span,
            item.exposure
        );
    }

    println!("\n== Basket margins (consider positions) ==");
    let basket = client
        .calculate_basket_margins_typed(&orders, true, true)
        .await?;
    println!(
        "Initial total: {:.2}, Final total: {:.2}",
        basket.initial.total, basket.final_.total
    );

    // Virtual contract note (charges) requires average_price per order fill
    let charges_orders = vec![OrderChargesRequest {
        order_id: "demo-1".into(),
        exchange: Exchange::NSE,
        trading_symbol: "RELIANCE".into(),
        transaction_type: TransactionType::BUY,
        variety: Variety::Regular,
        product: Product::CNC,
        order_type: OrderType::MARKET,
        quantity: 1,
        average_price: 2500.0, // replace with realistic executed average
    }];

    println!("\n== Virtual contract note (charges) ==");
    let charges = client
        .calculate_order_charges_typed(&charges_orders)
        .await?;
    for c in charges {
        println!(
            "{} {} qty {} price {:.2} total charges: {:.2} (brokerage: {:.2}, gst: {:.2})",
            c.exchange,
            c.trading_symbol,
            c.quantity,
            c.price,
            c.charges.total,
            c.charges.brokerage,
            c.charges.gst.total
        );
    }

    Ok(())
}
