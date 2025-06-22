use serde_json::Value as JsonValue;
use anyhow::Result;
use chrono::Utc;

// WASM platform imports for CSV parsing
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use csv_core::{Reader, ReadFieldResult};

use crate::model::{Instrument, MFInstrument};

/// Parse CSV data using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn parse_csv_with_core(csv_data: &str) -> Result<JsonValue> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut headers: Vec<String> = Vec::new();
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                }
                break;
            }
            ReadFieldResult::OutputFull => {
                field.extend_from_slice(&output[..output_written]);
                // Continue reading with same input
            }
            ReadFieldResult::Field { record_end } => {
                field.extend_from_slice(&output[..output_written]);
                let field_str = String::from_utf8_lossy(&field).to_string();
                current_record.push(field_str);
                field.clear();
                
                if record_end {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to JSON format
    let mut result = Vec::new();
    for record in records {
        let mut obj = serde_json::Map::new();
        for (i, field_value) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.clone(), JsonValue::String(field_value.clone()));
            }
        }
        result.push(JsonValue::Object(obj));
    }
    
    Ok(JsonValue::Array(result))
}

/// Parse CSV data into Instrument structs using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn parse_csv_to_instruments(csv_data: &str) -> Result<Vec<Instrument>> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() && !is_first_row {
                    records.push(current_record.clone());
                }
                break;
            }
            ReadFieldResult::OutputFull => {
                field.extend_from_slice(&output[..output_written]);
                // Continue reading with same input
            }
            ReadFieldResult::Field { record_end } => {
                field.extend_from_slice(&output[..output_written]);
                let field_str = String::from_utf8_lossy(&field).to_string();
                current_record.push(field_str);
                field.clear();
                
                if record_end {
                    if is_first_row {
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to Instrument structs
    let mut result = Vec::new();
    for record in records {
        if record.len() >= 12 {
            let instrument = Instrument {
                instrument_token: record[0].parse().unwrap_or(0),
                exchange_token: record[1].parse().unwrap_or(0),
                tradingsymbol: record[2].clone(),
                name: record[3].clone(),
                last_price: record[4].parse().unwrap_or(0.0),
                expiry: if record[5].is_empty() { 
                    None 
                } else { 
                    record[5].parse().ok() 
                },
                strike_price: record[6].parse().unwrap_or(0.0),
                tick_size: record[7].parse().unwrap_or(0.0),
                lot_size: record[8].parse().unwrap_or(0.0),
                instrument_type: record[9].clone(),
                segment: record[10].clone(),
                exchange: record[11].clone(),
            };
            result.push(instrument);
        }
    }
    
    Ok(result)
}

/// Parse CSV data into MFInstrument structs using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn parse_csv_to_mf_instruments(csv_data: &str) -> Result<Vec<MFInstrument>> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() && !is_first_row {
                    records.push(current_record.clone());
                }
                break;
            }
            ReadFieldResult::OutputFull => {
                field.extend_from_slice(&output[..output_written]);
                // Continue reading with same input
            }
            ReadFieldResult::Field { record_end } => {
                field.extend_from_slice(&output[..output_written]);
                let field_str = String::from_utf8_lossy(&field).to_string();
                current_record.push(field_str);
                field.clear();
                
                if record_end {
                    if is_first_row {
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to MFInstrument structs
    let mut result = Vec::new();
    for record in records {
        if record.len() >= 16 {
            let mf_instrument = MFInstrument {
                tradingsymbol: record[0].clone(),
                name: record[1].clone(),
                last_price: record[2].parse().unwrap_or(0.0),
                amc: record[3].clone(),
                purchase_allowed: record[4].parse().unwrap_or(false),
                redemption_allowed: record[5].parse().unwrap_or(false),
                minimum_purchase_amount: record[6].parse().unwrap_or(0.0),
                purchase_amount_multiplier: record[7].parse().unwrap_or(0.0),
                minimum_additional_purchase_amount: record[8].parse().unwrap_or(0.0),
                minimum_redemption_quantity: record[9].parse().unwrap_or(0.0),
                redemption_quantity_multiplier: record[10].parse().unwrap_or(0.0),
                dividend_type: record[11].clone(),
                scheme_type: record[12].clone(),
                plan: record[13].clone(),
                settlement_type: record[14].clone(),
                last_price_date: record[15].parse().unwrap_or_else(|_| Utc::now()),
            };
            result.push(mf_instrument);
        }
    }
    
    Ok(result)
}
