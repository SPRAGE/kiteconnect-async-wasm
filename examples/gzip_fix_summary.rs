//! # Instruments Gzip Fix Summary
//!
//! ## Problem Identified
//! 
//! The KiteConnect API returns instrument data as **gzipped CSV**, but our library
//! was trying to parse it as plain CSV text. This caused the CSV parser to fail
//! silently and return 0 instruments.
//!
//! ## Root Cause
//! 
//! From the official KiteConnect documentation:
//! > "Unlike the rest of the calls that return JSON, the instrument list API returns a
//! > gzipped CSV dump of instruments across all exchanges..."
//!
//! Our implementation was missing gzip decompression.
//!
//! ## Fix Applied
//!
//! 1. **Added flate2 dependency** for gzip decompression
//! 2. **Updated instruments method** to detect and decompress gzipped responses
//! 3. **Added debug logging** to help troubleshoot future issues
//! 4. **Created backup methods** for testing different approaches
//!
//! ## Testing the Fix
//!
//! Set your environment variables:
//! ```bash
//! export KITE_API_KEY="your_api_key"
//! export KITE_ACCESS_TOKEN="your_access_token"
//! export RUST_LOG=debug
//! ```
//!
//! Then run:
//! ```bash
//! # Test the main fix
//! cargo run --example test_gzip_fix --features=native,debug
//!
//! # Extended debugging
//! cargo run --example debug_instruments --features=native,debug
//!
//! # Simple test
//! cargo run --example simple_instruments_log --features=native
//! ```
//!
//! ## Expected Results
//!
//! After the fix, you should see:
//! - **instruments()** returns thousands of instruments (typically 80,000+)
//! - **instruments_typed()** works correctly
//! - CSV parsing succeeds with proper headers and data
//!
//! ## Technical Details
//!
//! ### Before Fix:
//! ```rust
//! let body = resp.text().await?; // This was trying to read gzipped data as text
//! let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes()); // CSV parser failed
//! ```
//!
//! ### After Fix:
//! ```rust
//! let content_encoding = resp.headers().get("content-encoding");
//! let body_text = if content_encoding.contains("gzip") {
//!     // Decompress gzipped data first
//!     let body_bytes = resp.bytes().await?;
//!     let mut decoder = flate2::read::GzDecoder::new(&body_bytes[..]);
//!     let mut decompressed = String::new();
//!     decoder.read_to_string(&mut decompressed)?;
//!     decompressed
//! } else {
//!     resp.text().await?
//! };
//! let mut rdr = ReaderBuilder::new().from_reader(body_text.as_bytes()); // Now works!
//! ```
//!
//! ## Files Modified
//!
//! - `Cargo.toml`: Added flate2 dependency
//! - `src/connect/market_data.rs`: Updated instruments methods with gzip support
//! - `examples/`: Created comprehensive debugging examples
//!
//! ## Verification Steps
//!
//! 1. **Run the test examples** to confirm instruments are now fetched
//! 2. **Check the count** - should be 80,000+ total instruments
//! 3. **Verify data quality** - instruments should have proper fields like tradingsymbol, name, exchange
//! 4. **Test exchange filtering** - NSE should return ~40,000+ instruments
//!
//! ## If the Fix Doesn't Work
//!
//! If you still get 0 instruments after this fix:
//!
//! 1. **Check API permissions** - Your API key might not have instruments access
//! 2. **Verify authentication** - Run the profile endpoint test first
//! 3. **Check response format** - API might have changed again
//! 4. **Contact Zerodha** - They might have server-side issues
//!
//! ## Performance Notes
//!
//! - The instruments dump is large (~10MB compressed, ~50MB+ uncompressed)
//! - Fetching takes 5-10 seconds depending on connection
//! - Results are cached for 1 hour by default
//! - Consider fetching once daily as recommended by Zerodha

fn main() {
    println!("This is a documentation file explaining the gzip fix for instruments.");
    println!("Please run the test examples to verify the fix works:");
    println!("  cargo run --example test_gzip_fix --features=native,debug");
}
