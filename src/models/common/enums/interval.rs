/*!
Interval types for historical market data with dual string/integer serialization support.

This module provides the `Interval` enum for specifying time intervals in historical data requests.
The v1.0.3 enhancement adds dual serde support, accepting both string and integer formats during
deserialization while always serializing as strings for API consistency.

# Dual Serde Support (v1.0.3)

The `Interval` enum now supports flexible input formats:
- **Strings**: "minute", "day", "5minute", etc. (human-readable)
- **Integers**: 1, 0, 3, etc. (compact, legacy support)

It always serializes as strings for API compatibility and human readability.

# Examples

## Basic Usage

```rust
use kiteconnect_async_wasm::models::common::Interval;

let interval = Interval::Day;
assert_eq!(interval.to_string(), "day");
```

## Deserialization Flexibility

```rust
use kiteconnect_async_wasm::models::common::Interval;

// From strings (human-readable)
let from_string: Interval = serde_json::from_str("\"day\"").unwrap();
let from_minute: Interval = serde_json::from_str("\"5minute\"").unwrap();

// From integers (compact)
let from_int: Interval = serde_json::from_str("0").unwrap();  // Day
let from_five: Interval = serde_json::from_str("3").unwrap(); // FiveMinute

assert_eq!(from_string, from_int);
assert_eq!(from_minute, from_five);
```

## Serialization Consistency

```rust
use kiteconnect_async_wasm::models::common::Interval;

// Always serializes as strings
assert_eq!(serde_json::to_string(&Interval::Day).unwrap(), "\"day\"");
assert_eq!(serde_json::to_string(&Interval::FiveMinute).unwrap(), "\"5minute\"");
```
*/

/// Interval types for historical data with dual serialization support
///
/// This enum represents time intervals for historical market data requests.
/// Each variant corresponds to a specific time period for OHLCV candles.
///
/// # Supported Intervals
///
/// - **Day**: Daily candles (most common for long-term analysis)
/// - **Minute**: 1-minute candles (highest resolution intraday)
/// - **Multi-minute**: 3, 5, 10, 15, 30, 60 minute candles (various intraday resolutions)
///
/// # Data Availability
///
/// - **Daily data**: Several years of history available
/// - **Intraday data**: Limited to recent periods (varies by broker)
/// - **Higher frequency**: More data points but faster rate limit consumption
///
/// # Rate Limiting Considerations
///
/// Higher frequency intervals generate more data points and may consume rate limits faster.
/// Consider using appropriate intervals for your use case:
/// - Backtesting: Daily or hourly data
/// - Real-time monitoring: 1-5 minute data
/// - Pattern analysis: 15-30 minute data
///
/// # Integer Mapping
///
/// For legacy compatibility, intervals map to integers:
/// - Day = 0
/// - Minute = 1  
/// - ThreeMinute = 2
/// - FiveMinute = 3
/// - TenMinute = 4
/// - FifteenMinute = 5
/// - ThirtyMinute = 6
/// - SixtyMinute = 7
///
/// # Examples
///
/// ```rust
/// use kiteconnect_async_wasm::models::common::Interval;
///
/// // Different ways to create intervals
/// let daily = Interval::Day;
/// let intraday = Interval::FiveMinute;
///
/// // Convert to string for display
/// println!("Daily interval: {}", daily);      // "day"
/// println!("Intraday: {}", intraday);         // "5minute"
///
/// // Check properties
/// assert!(daily.is_daily());
/// assert!(intraday.is_intraday());
/// assert_eq!(intraday.minutes(), 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Interval {
    /// Daily interval (1 day candles)
    ///
    /// Most commonly used for long-term analysis, backtesting, and trend identification.
    /// Provides good data coverage with minimal API rate limit impact.
    Day = 0,

    /// 1-minute interval (highest resolution)
    ///
    /// Highest available resolution for intraday analysis. Useful for:
    /// - Scalping strategies
    /// - Real-time monitoring  
    /// - High-frequency pattern analysis
    ///
    /// Note: Consumes rate limits quickly due to large data volumes.
    Minute = 1,

    /// 3-minute interval
    ///
    /// Good balance between resolution and data volume for short-term strategies.
    ThreeMinute = 2,

    /// 5-minute interval
    ///
    /// Popular choice for intraday trading strategies. Provides good detail
    /// while maintaining manageable data volumes.
    FiveMinute = 3,

    /// 10-minute interval
    ///
    /// Suitable for medium-term intraday analysis with reduced noise.
    TenMinute = 4,

    /// 15-minute interval
    ///
    /// Common choice for swing trading and pattern recognition.
    /// Good balance of detail and broader market perspective.
    FifteenMinute = 5,

    /// 30-minute interval
    ///
    /// Used for identifying medium-term trends within trading sessions.
    /// Filters out short-term noise while maintaining intraday perspective.
    ThirtyMinute = 6,

    /// 60-minute (1-hour) interval
    ///
    /// Bridge between intraday and daily analysis. Useful for:
    /// - Multi-session analysis
    /// - Identifying major support/resistance levels
    /// - Trend confirmation
    SixtyMinute = 7,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Day => write!(f, "day"),
            Interval::Minute => write!(f, "minute"),
            Interval::ThreeMinute => write!(f, "3minute"),
            Interval::FiveMinute => write!(f, "5minute"),
            Interval::TenMinute => write!(f, "10minute"),
            Interval::FifteenMinute => write!(f, "15minute"),
            Interval::ThirtyMinute => write!(f, "30minute"),
            Interval::SixtyMinute => write!(f, "60minute"),
        }
    }
}

// Custom serde implementation that supports both string and integer formats
impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Try to serialize as string first (for compatibility)
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IntervalVisitor;

        impl<'de> serde::de::Visitor<'de> for IntervalVisitor {
            type Value = Interval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or integer representing an interval")
            }

            fn visit_str<E>(self, value: &str) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "minute" => Ok(Interval::Minute),
                    "3minute" => Ok(Interval::ThreeMinute),
                    "5minute" => Ok(Interval::FiveMinute),
                    "10minute" => Ok(Interval::TenMinute),
                    "15minute" => Ok(Interval::FifteenMinute),
                    "30minute" => Ok(Interval::ThirtyMinute),
                    "60minute" => Ok(Interval::SixtyMinute),
                    "day" => Ok(Interval::Day),
                    _ => Err(serde::de::Error::unknown_variant(
                        value,
                        &[
                            "minute", "3minute", "5minute", "10minute", "15minute", "30minute",
                            "60minute", "day",
                        ],
                    )),
                }
            }

            fn visit_i8<E>(self, value: i8) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0 => Ok(Interval::Day),
                    1 => Ok(Interval::Minute),
                    2 => Ok(Interval::ThreeMinute),
                    3 => Ok(Interval::FiveMinute),
                    4 => Ok(Interval::TenMinute),
                    5 => Ok(Interval::FifteenMinute),
                    6 => Ok(Interval::ThirtyMinute),
                    7 => Ok(Interval::SixtyMinute),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Signed(value as i64),
                        &"an integer between 0 and 7",
                    )),
                }
            }

            fn visit_u8<E>(self, value: u8) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                self.visit_i8(value as i8)
            }

            fn visit_i32<E>(self, value: i32) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                if (0..=7).contains(&value) {
                    self.visit_i8(value as i8)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Signed(value as i64),
                        &"an integer between 0 and 7",
                    ))
                }
            }

            fn visit_u32<E>(self, value: u32) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                if value <= 7 {
                    self.visit_i8(value as i8)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value as u64),
                        &"an integer between 0 and 7",
                    ))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                if (0..=7).contains(&value) {
                    self.visit_i8(value as i8)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Signed(value),
                        &"an integer between 0 and 7",
                    ))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Interval, E>
            where
                E: serde::de::Error,
            {
                if value <= 7 {
                    self.visit_i8(value as i8)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value),
                        &"an integer between 0 and 7",
                    ))
                }
            }
        }

        deserializer.deserialize_any(IntervalVisitor)
    }
}

impl Interval {
    /// Get the integer representation of the interval
    pub fn as_i8(self) -> i8 {
        self as i8
    }

    /// Create an interval from its integer representation
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            0 => Some(Interval::Day),
            1 => Some(Interval::Minute),
            2 => Some(Interval::ThreeMinute),
            3 => Some(Interval::FiveMinute),
            4 => Some(Interval::TenMinute),
            5 => Some(Interval::FifteenMinute),
            6 => Some(Interval::ThirtyMinute),
            7 => Some(Interval::SixtyMinute),
            _ => None,
        }
    }

    /// All available intervals
    pub fn all() -> Vec<Self> {
        vec![
            Interval::Minute,
            Interval::ThreeMinute,
            Interval::FiveMinute,
            Interval::TenMinute,
            Interval::FifteenMinute,
            Interval::ThirtyMinute,
            Interval::SixtyMinute,
            Interval::Day,
        ]
    }
}
