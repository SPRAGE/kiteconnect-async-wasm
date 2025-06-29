#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use serde_json::json;

    #[test]
    fn test_candle_deserialization_array_format() {
        // Test with 6 elements (no OI)
        let json_data = json!(["2024-12-20T09:15:00+0530", 100.5, 105.0, 99.5, 104.0, 1000]);

        let candle: Candle = serde_json::from_value(json_data).unwrap();

        assert_eq!(candle.open, 100.5);
        assert_eq!(candle.high, 105.0);
        assert_eq!(candle.low, 99.5);
        assert_eq!(candle.close, 104.0);
        assert_eq!(candle.volume, 1000);
        assert_eq!(candle.oi, None);
    }

    #[test]
    fn test_candle_deserialization_with_oi() {
        // Test with 7 elements (with OI)
        let json_data = json!([
            "2024-12-20T09:15:00+0530",
            100.5,
            105.0,
            99.5,
            104.0,
            1000,
            500
        ]);

        let candle: Candle = serde_json::from_value(json_data).unwrap();

        assert_eq!(candle.open, 100.5);
        assert_eq!(candle.high, 105.0);
        assert_eq!(candle.low, 99.5);
        assert_eq!(candle.close, 104.0);
        assert_eq!(candle.volume, 1000);
        assert_eq!(candle.oi, Some(500));
    }

    #[test]
    fn test_candle_deserialization_timestamp() {
        // Test with Unix timestamp
        let json_data = json!([
            1703048100, // Unix timestamp for 2023-12-20 09:15:00 UTC
            100.5, 105.0, 99.5, 104.0, 1000
        ]);

        let candle: Candle = serde_json::from_value(json_data).unwrap();

        assert_eq!(candle.open, 100.5);
        assert_eq!(candle.high, 105.0);
        assert_eq!(candle.low, 99.5);
        assert_eq!(candle.close, 104.0);
        assert_eq!(candle.volume, 1000);
        assert_eq!(candle.oi, None);
    }

    #[test]
    fn test_date_parsing_formats() {
        // Test IST timezone format
        let ist_date = "2024-12-20T09:15:00+0530";
        let json_data = json!([ist_date, 100.0, 101.0, 99.0, 100.5, 1000]);
        let candle: Candle = serde_json::from_value(json_data).unwrap();

        // Should be converted to UTC (IST is UTC+5:30, so 09:15 IST = 03:45 UTC)
        let expected_utc =
            DateTime::parse_from_str("2024-12-20T03:45:00+0000", "%Y-%m-%dT%H:%M:%S%z")
                .unwrap()
                .with_timezone(&Utc);

        assert_eq!(candle.date, expected_utc);
    }

    #[test]
    fn test_historical_data_response() {
        // Test a complete historical data response
        let json_data = json!({
            "data": {
                "candles": [
                    ["2024-12-20T09:15:00+0530", 100.5, 105.0, 99.5, 104.0, 1000],
                    ["2024-12-20T09:20:00+0530", 104.0, 106.0, 103.0, 105.5, 1200, 600]
                ]
            }
        });

        // Test that we can parse multiple candles
        if let Some(candles_data) = json_data["data"]["candles"].as_array() {
            let candles: Vec<Candle> = candles_data
                .iter()
                .map(|c| serde_json::from_value(c.clone()).unwrap())
                .collect();

            assert_eq!(candles.len(), 2);
            assert_eq!(candles[0].oi, None);
            assert_eq!(candles[1].oi, Some(600));
        }
    }
}
