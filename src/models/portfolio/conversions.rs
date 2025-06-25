use crate::models::common::{Exchange, Product, TransactionType};
use serde::{Deserialize, Serialize};

/// Portfolio conversion types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversionType {
    /// Convert from CNC to MIS
    Position,
    /// Convert from MIS to CNC
    Holding,
}

/// Portfolio conversion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRequest {
    /// Exchange
    pub exchange: Exchange,

    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Transaction type
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionType,

    /// Quantity to convert
    pub quantity: u32,

    /// From product type
    #[serde(rename = "from_product")]
    pub from_product: Product,

    /// To product type
    #[serde(rename = "to_product")]
    pub to_product: Product,
}

/// Conversion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResponse {
    /// Status of the conversion
    pub status: String,

    /// Message from the conversion operation
    pub message: Option<String>,
}

/// Bulk conversion request for multiple instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkConversionRequest {
    /// List of conversion requests
    pub conversions: Vec<ConversionRequest>,
}

/// Bulk conversion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkConversionResponse {
    /// Status of the bulk conversion
    pub status: String,

    /// Individual conversion results
    pub results: Vec<ConversionResult>,

    /// Overall message
    pub message: Option<String>,
}

/// Individual conversion result in bulk operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,

    /// Exchange
    pub exchange: Exchange,

    /// Status of this specific conversion
    pub status: String,

    /// Message for this conversion
    pub message: Option<String>,

    /// Error details if conversion failed
    pub error: Option<String>,
}

impl ConversionRequest {
    /// Create a new conversion request
    pub fn new(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
        from_product: Product,
        to_product: Product,
    ) -> Self {
        Self {
            exchange,
            trading_symbol,
            transaction_type,
            quantity,
            from_product,
            to_product,
        }
    }

    /// Create a request to convert CNC to MIS (position conversion)
    pub fn cnc_to_mis(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            quantity,
            Product::CNC,
            Product::MIS,
        )
    }

    /// Create a request to convert MIS to CNC (holding conversion)
    pub fn mis_to_cnc(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            quantity,
            Product::MIS,
            Product::CNC,
        )
    }

    /// Create a request to convert NRML to MIS
    pub fn nrml_to_mis(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            quantity,
            Product::NRML,
            Product::MIS,
        )
    }

    /// Create a request to convert MIS to NRML
    pub fn mis_to_nrml(
        exchange: Exchange,
        trading_symbol: String,
        transaction_type: TransactionType,
        quantity: u32,
    ) -> Self {
        Self::new(
            exchange,
            trading_symbol,
            transaction_type,
            quantity,
            Product::MIS,
            Product::NRML,
        )
    }

    /// Get the conversion type based on products
    pub fn conversion_type(&self) -> ConversionType {
        match (&self.from_product, &self.to_product) {
            (Product::CNC, Product::MIS) => ConversionType::Position,
            (Product::MIS, Product::CNC) => ConversionType::Holding,
            (Product::NRML, Product::MIS) => ConversionType::Position,
            (Product::MIS, Product::NRML) => ConversionType::Position,
            _ => ConversionType::Position, // Default to position conversion
        }
    }

    /// Check if this is a valid conversion
    pub fn is_valid_conversion(&self) -> bool {
        // Define valid conversion pairs
        matches!(
            (&self.from_product, &self.to_product),
            (Product::CNC, Product::MIS)
                | (Product::MIS, Product::CNC)
                | (Product::NRML, Product::MIS)
                | (Product::MIS, Product::NRML)
        )
    }

    /// Validate the conversion request
    pub fn validate(&self) -> Result<(), String> {
        if self.trading_symbol.is_empty() {
            return Err("Trading symbol cannot be empty".to_string());
        }

        if self.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }

        if !self.is_valid_conversion() {
            return Err(format!(
                "Invalid conversion from {:?} to {:?}",
                self.from_product, self.to_product
            ));
        }

        Ok(())
    }
}

impl BulkConversionRequest {
    /// Create a new bulk conversion request
    pub fn new() -> Self {
        Self {
            conversions: Vec::new(),
        }
    }

    /// Add a conversion to the bulk request
    pub fn add_conversion(mut self, conversion: ConversionRequest) -> Self {
        self.conversions.push(conversion);
        self
    }

    /// Add multiple conversions to the bulk request
    pub fn add_conversions(mut self, conversions: Vec<ConversionRequest>) -> Self {
        self.conversions.extend(conversions);
        self
    }

    /// Validate all conversions in the bulk request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.conversions.is_empty() {
            errors.push("Bulk conversion request cannot be empty".to_string());
        }

        for (index, conversion) in self.conversions.iter().enumerate() {
            if let Err(error) = conversion.validate() {
                errors.push(format!("Conversion {}: {}", index + 1, error));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get the total number of conversions
    pub fn count(&self) -> usize {
        self.conversions.len()
    }

    /// Check if the bulk request is empty
    pub fn is_empty(&self) -> bool {
        self.conversions.is_empty()
    }
}

impl Default for BulkConversionRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ConversionResponse {
    /// Check if the conversion was successful
    pub fn is_success(&self) -> bool {
        self.status.to_lowercase() == "success"
    }

    /// Check if the conversion failed
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

impl BulkConversionResponse {
    /// Check if all conversions were successful
    pub fn is_all_success(&self) -> bool {
        self.results.iter().all(|r| r.is_success())
    }

    /// Get the number of successful conversions
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_success()).count()
    }

    /// Get the number of failed conversions
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_failure()).count()
    }

    /// Get successful conversion results
    pub fn successful_conversions(&self) -> Vec<&ConversionResult> {
        self.results.iter().filter(|r| r.is_success()).collect()
    }

    /// Get failed conversion results
    pub fn failed_conversions(&self) -> Vec<&ConversionResult> {
        self.results.iter().filter(|r| r.is_failure()).collect()
    }
}

impl ConversionResult {
    /// Check if this conversion was successful
    pub fn is_success(&self) -> bool {
        self.status.to_lowercase() == "success"
    }

    /// Check if this conversion failed
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Get the error message if conversion failed
    pub fn error_message(&self) -> Option<&str> {
        self.error.as_deref().or(self.message.as_deref())
    }
}
