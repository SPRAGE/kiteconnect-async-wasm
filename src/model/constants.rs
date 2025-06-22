//! # KiteConnect Constants
//! 
//! This module contains all the constants used in the KiteConnect API,
//! matching the Go implementation.

/// Kite Connect API product constants
pub mod products {
    /// Bracket Order
    pub const BO: &str = "BO";
    /// Cover Order
    pub const CO: &str = "CO";
    /// Margin Intraday Squareoff
    pub const MIS: &str = "MIS";
    /// Cash and Carry (delivery)
    pub const CNC: &str = "CNC";
    /// Normal (carry forward)
    pub const NRML: &str = "NRML";
    /// Margin Trade Funding
    pub const MTF: &str = "MTF";
}

/// Order variety constants
pub mod varieties {
    /// Regular order
    pub const REGULAR: &str = "regular";
    /// After Market Order
    pub const AMO: &str = "amo";
    /// Bracket Order
    pub const BO: &str = "bo";
    /// Cover Order
    pub const CO: &str = "co";
    /// Iceberg Order
    pub const ICEBERG: &str = "iceberg";
    /// Auction order
    pub const AUCTION: &str = "auction";
}

/// Order type constants
pub mod order_types {
    /// Market order
    pub const MARKET: &str = "MARKET";
    /// Limit order
    pub const LIMIT: &str = "LIMIT";
    /// Stop loss order
    pub const SL: &str = "SL";
    /// Stop loss market order
    pub const SLM: &str = "SL-M";
}

/// Order validity constants
pub mod validities {
    /// Day order (valid for the trading day)
    pub const DAY: &str = "DAY";
    /// Immediate or Cancel
    pub const IOC: &str = "IOC";
    /// Time Till Live
    pub const TTL: &str = "TTL";
}

/// Position type constants
pub mod position_types {
    /// Day position
    pub const DAY: &str = "day";
    /// Overnight position
    pub const OVERNIGHT: &str = "overnight";
}

/// Transaction type constants
pub mod transaction_types {
    /// Buy transaction
    pub const BUY: &str = "BUY";
    /// Sell transaction
    pub const SELL: &str = "SELL";
}

/// Exchange constants
pub mod exchanges {
    /// National Stock Exchange
    pub const NSE: &str = "NSE";
    /// Bombay Stock Exchange
    pub const BSE: &str = "BSE";
    /// Multi Commodity Exchange
    pub const MCX: &str = "MCX";
    /// NSE Futures and Options
    pub const NFO: &str = "NFO";
    /// BSE Futures and Options
    pub const BFO: &str = "BFO";
    /// Currency Derivatives Segment
    pub const CDS: &str = "CDS";
    /// BSE Currency Derivatives
    pub const BCD: &str = "BCD";
}

/// Margin segment constants
pub mod margin_segments {
    /// Equity segment
    pub const EQUITY: &str = "equity";
    /// Commodity segment
    pub const COMMODITY: &str = "commodity";
}

/// Order status constants
pub mod order_statuses {
    /// Order completed
    pub const COMPLETE: &str = "COMPLETE";
    /// Order rejected
    pub const REJECTED: &str = "REJECTED";
    /// Order cancelled
    pub const CANCELLED: &str = "CANCELLED";
    /// Order open/pending
    pub const OPEN: &str = "OPEN";
    /// Order pending trigger
    pub const TRIGGER_PENDING: &str = "TRIGGER PENDING";
}

/// Holdings authorization type constants
pub mod holdings_auth_types {
    /// Mutual Fund holdings
    pub const MF: &str = "mf";
    /// Equity holdings
    pub const EQUITY: &str = "equity";
}

/// Holdings authorization transfer type constants
pub mod holdings_auth_transfer_types {
    /// Pre-trade authorization
    pub const PRE_TRADE: &str = "pre";
    /// Post-trade authorization
    pub const POST_TRADE: &str = "post";
    /// Off-market authorization
    pub const OFF_MARKET: &str = "off";
    /// Gift authorization
    pub const GIFT: &str = "gift";
}

/// GTT (Good Till Triggered) type constants
pub mod gtt_types {
    /// Single trigger GTT
    pub const SINGLE: &str = "single";
    /// One Cancels Other (OCO) GTT
    pub const OCO: &str = "two-leg";
}

/// Mutual Fund transaction type constants
pub mod mf_transaction_types {
    /// Purchase transaction
    pub const BUY: &str = "BUY";
    /// Redemption transaction
    pub const SELL: &str = "SELL";
}

/// API endpoints constants
pub mod endpoints {
    // User endpoints
    pub const USER_SESSION: &str = "/session/token";
    pub const USER_SESSION_INVALIDATE: &str = "/session/token";
    pub const USER_SESSION_RENEW: &str = "/session/refresh_token";
    pub const USER_PROFILE: &str = "/user/profile";
    pub const FULL_USER_PROFILE: &str = "/user/profile/full";
    pub const USER_MARGINS: &str = "/user/margins";
    pub const USER_MARGINS_SEGMENT: &str = "/user/margins/{segment}";

    // Orders endpoints
    pub const GET_ORDERS: &str = "/orders";
    pub const GET_TRADES: &str = "/trades";
    pub const GET_ORDER_HISTORY: &str = "/orders/{order_id}";
    pub const GET_ORDER_TRADES: &str = "/orders/{order_id}/trades";
    pub const PLACE_ORDER: &str = "/orders/{variety}";
    pub const MODIFY_ORDER: &str = "/orders/{variety}/{order_id}";
    pub const CANCEL_ORDER: &str = "/orders/{variety}/{order_id}";

    // Portfolio endpoints
    pub const GET_POSITIONS: &str = "/portfolio/positions";
    pub const GET_HOLDINGS: &str = "/portfolio/holdings";
    pub const INIT_HOLDINGS_AUTH: &str = "/portfolio/holdings/authorise";
    pub const AUCTION_INSTRUMENTS: &str = "/portfolio/holdings/auctions";
    pub const CONVERT_POSITION: &str = "/portfolio/positions";

    // Margin endpoints
    pub const ORDER_MARGINS: &str = "/margins/orders";
    pub const BASKET_MARGINS: &str = "/margins/basket";
    pub const ORDER_CHARGES: &str = "/charges/orders";

    // Mutual Fund endpoints
    pub const GET_MF_ORDERS: &str = "/mf/orders";
    pub const GET_MF_ORDER_INFO: &str = "/mf/orders/{order_id}";
    pub const PLACE_MF_ORDER: &str = "/mf/orders";
    pub const CANCEL_MF_ORDER: &str = "/mf/orders/{order_id}";
    pub const GET_MF_SIPS: &str = "/mf/sips";
    pub const GET_MF_SIP_INFO: &str = "/mf/sips/{sip_id}";
    pub const PLACE_MF_SIP: &str = "/mf/sips";
    pub const MODIFY_MF_SIP: &str = "/mf/sips/{sip_id}";
    pub const CANCEL_MF_SIP: &str = "/mf/sips/{sip_id}";
    pub const GET_MF_HOLDINGS: &str = "/mf/holdings";
    pub const GET_MF_HOLDING_INFO: &str = "/mf/holdings/{isin}";
    pub const GET_ALLOTTED_ISINS: &str = "/mf/allotments";

    // GTT endpoints
    pub const PLACE_GTT: &str = "/gtt/triggers";
    pub const GET_GTTS: &str = "/gtt/triggers";
    pub const GET_GTT: &str = "/gtt/triggers/{trigger_id}";
    pub const MODIFY_GTT: &str = "/gtt/triggers/{trigger_id}";
    pub const DELETE_GTT: &str = "/gtt/triggers/{trigger_id}";

    // Market data endpoints
    pub const GET_INSTRUMENTS: &str = "/instruments";
    pub const GET_MF_INSTRUMENTS: &str = "/mf/instruments";
    pub const GET_INSTRUMENTS_EXCHANGE: &str = "/instruments/{exchange}";
    pub const GET_HISTORICAL: &str = "/instruments/historical/{instrument_token}/{interval}";
    pub const GET_TRIGGER_RANGE: &str = "/instruments/{exchange}/{tradingsymbol}/trigger_range";
    pub const GET_QUOTE: &str = "/quote";
    pub const GET_LTP: &str = "/quote/ltp";
    pub const GET_OHLC: &str = "/quote/ohlc";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_constants() {
        assert_eq!(products::BO, "BO");
        assert_eq!(products::CO, "CO");
        assert_eq!(products::MIS, "MIS");
        assert_eq!(products::CNC, "CNC");
        assert_eq!(products::NRML, "NRML");
        assert_eq!(products::MTF, "MTF");
    }

    #[test]
    fn test_order_type_constants() {
        assert_eq!(order_types::MARKET, "MARKET");
        assert_eq!(order_types::LIMIT, "LIMIT");
        assert_eq!(order_types::SL, "SL");
        assert_eq!(order_types::SLM, "SL-M");
    }

    #[test]
    fn test_exchange_constants() {
        assert_eq!(exchanges::NSE, "NSE");
        assert_eq!(exchanges::BSE, "BSE");
        assert_eq!(exchanges::MCX, "MCX");
        assert_eq!(exchanges::NFO, "NFO");
    }

    #[test]
    fn test_transaction_type_constants() {
        assert_eq!(transaction_types::BUY, "BUY");
        assert_eq!(transaction_types::SELL, "SELL");
    }

    #[test]
    fn test_validity_constants() {
        assert_eq!(validities::DAY, "DAY");
        assert_eq!(validities::IOC, "IOC");
        assert_eq!(validities::TTL, "TTL");
    }
}
