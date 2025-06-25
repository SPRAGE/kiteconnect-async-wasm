/*!
Interval types for historical market data with dual string/integer serialization support.
*/

/// Interval types for historical data (supports both string and integer serialization)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Interval {
    Day = 0,
    Minute = 1,
    ThreeMinute = 2,
    FiveMinute = 3,
    TenMinute = 4,
    FifteenMinute = 5,
    ThirtyMinute = 6,
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
                    _ => Err(serde::de::Error::unknown_variant(value, &[
                        "minute", "3minute", "5minute", "10minute", 
                        "15minute", "30minute", "60minute", "day"
                    ])),
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
                        &"an integer between 0 and 7"
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
                        &"an integer between 0 and 7"
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
                        &"an integer between 0 and 7"
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
                        &"an integer between 0 and 7"
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
                        &"an integer between 0 and 7"
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
