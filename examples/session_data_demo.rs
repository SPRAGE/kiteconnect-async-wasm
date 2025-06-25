use kiteconnect_async_wasm::models::auth::SessionData;
use serde_json::json;

fn main() {
    println!("=== KiteConnect SessionData Model Demo ===\n");

    // Example session response from KiteConnect API
    let session_json = json!({
        "user_id": "XZ0123",
        "user_name": "John Doe",
        "user_shortname": "johndoe",
        "email": "john.doe@example.com",
        "user_type": "individual",
        "broker": "ZERODHA",
        "exchanges": ["NSE", "BSE", "NFO", "CDS"],
        "products": ["CNC", "MIS", "NRML"],
        "order_types": ["MARKET", "LIMIT", "SL", "SL-M"],
        "api_key": "abc123xyz",
        "access_token": "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0",
        "public_token": "pub_token_example",
        "refresh_token": "refresh_token_example",
        "login_time": "2024-01-15 09:30:00",
        "meta": {
            "demat_consent": "consent"
        },
        "avatar_url": "https://example.com/avatar/johndoe.png"
    });

    // Deserialize JSON to SessionData struct
    let session: SessionData =
        serde_json::from_value(session_json).expect("Failed to deserialize session data");

    println!("📊 Session Information:");
    println!("   User ID: {}", session.user_id);
    println!("   User Name: {}", session.user_name);
    println!("   User Short Name: {}", session.user_shortname);
    println!("   Email: {}", session.email);
    println!("   User Type: {}", session.user_type);
    println!("   Broker: {}", session.broker);
    println!("   Login Time: {}", session.login_time);

    if let Some(avatar_url) = &session.avatar_url {
        println!("   Avatar URL: {}", avatar_url);
    }

    println!("\n🏦 Trading Permissions:");
    println!("   Exchanges: {}", session.exchanges.join(", "));
    println!("   Products: {}", session.products.join(", "));
    println!("   Order Types: {}", session.order_types.join(", "));

    println!("\n🔑 Authentication Tokens:");
    println!("   API Key: {}", session.api_key);
    println!(
        "   Access Token: {}...{}",
        &session.access_token[..8],
        &session.access_token[session.access_token.len() - 8..]
    );

    if !session.public_token.is_empty() {
        println!("   Public Token: {}...", &session.public_token[..8]);
    }

    if !session.refresh_token.is_empty() {
        println!("   Refresh Token: {}...", &session.refresh_token[..8]);
    }

    println!("\n🔍 Session Validation:");
    println!("   Is Valid: {}", session.is_valid());
    println!("   Has NSE Access: {}", session.has_exchange("NSE"));
    println!("   Has MCX Access: {}", session.has_exchange("MCX"));
    println!("   Has CNC Product: {}", session.has_product("CNC"));
    println!("   Has BO Product: {}", session.has_product("BO"));
    println!("   Has LIMIT Orders: {}", session.has_order_type("LIMIT"));
    println!("   Has GTT Orders: {}", session.has_order_type("GTT"));

    if let Some(meta) = &session.meta {
        println!("\n📋 Metadata:");
        println!("   Demat Consent: {}", meta.demat_consent);
    }

    println!("\n✅ SessionData model successfully handles all KiteConnect session fields!");

    // Demonstrate serialization back to JSON
    let serialized =
        serde_json::to_string_pretty(&session).expect("Failed to serialize session data");

    println!("\n📤 Serialized Session Data:");
    println!("{}", serialized);
}
