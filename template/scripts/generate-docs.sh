#!/usr/bin/env zsh

# Generate comprehensive documentation for a Rust library
CRATE_NAME=${CRATE_NAME:-your-crate-name}
echo "🚀 Generating $CRATE_NAME documentation..."

# Clean previous documentation
echo "🧹 Cleaning previous documentation..."
rm -rf doc

# Generate documentation with comprehensive options
echo "📚 Building documentation..."
echo "   - Including private items and dependencies"
echo "   - Generating source code links"
echo "   - Including all examples"

cargo doc \
    --no-deps \
    --document-private-items \
    --examples \
    --all-features

# Check if documentation was generated successfully in target/doc
if [ -f "target/doc/${CRATE_NAME}/index.html" ]; then
    echo "📁 Copying documentation to root doc/ folder..."
    # Create doc directory if it doesn't exist
    mkdir -p doc
    # Copy all generated documentation to root doc folder
    cp -r target/doc/* doc/
    
    # Generate additional documentation files
    echo "📝 Generating additional documentation..."
    
    # Create a comprehensive README for the docs
    cat > doc/README.md << 'EOF'
# Library Documentation

This documentation provides comprehensive information about the KiteConnect Rust library.

## Navigation

- **[Main Library Documentation](${CRATE_NAME}/index.html)** - Core documentation
- **[API Reference](${CRATE_NAME}/connect/index.html)** - Detailed API methods
- **[Source Code](src/${CRATE_NAME}/)** - Browse source

## Quick Links

### Getting Started
- [Installation](${CRATE_NAME}/index.html#quick-start)
- [Authentication Flow](${CRATE_NAME}/connect/index.html#authentication-flow)
- [Basic Usage Examples](${CRATE_NAME}/index.html#basic-usage)

### API Categories
- **Portfolio**: Holdings, Positions, Margins
- **Orders**: Place, Modify, Cancel orders
- **Market Data**: Instruments, Quotes, Historical data
- **Mutual Funds**: MF orders and instruments

### Examples
- [Basic Example](${CRATE_NAME}/index.html#basic-usage)
- [Error Handling](${CRATE_NAME}/index.html#error-handling)
- [Concurrent Operations](${CRATE_NAME}/index.html#thread-safety)

## Features

- 🚀 **Async/Await** - Modern async patterns with tokio
- 🌐 **WASM Support** - Run in browsers with WebAssembly
- 🔄 **Cross-Platform** - Native and Web targets
- 🛡️ **Type Safe** - Leverages Rust's type system
- ⚡ **High Performance** - Connection pooling and efficient HTTP client

## Platform Support

- **Native**: Full API support with CSV parsing
- **WASM**: All APIs supported (raw CSV for client-side parsing)

Generated with ❤️ using `cargo doc`
EOF

    # Create a quick reference guide
    cat > doc/QUICKREF.md << 'EOF'
# Quick Reference Guide

## Authentication
```rust
let mut client = KiteConnect::new("api_key", "");
let login_url = client.login_url();
// User completes login...
let session = client.generate_session("request_token", "api_secret").await?;
```

## Portfolio Operations
```rust
let holdings = client.holdings().await?;
let positions = client.positions().await?;
let margins = client.margins(None).await?;
```

## Order Management
```rust
let orders = client.orders().await?;
let trades = client.trades().await?;
```

## Market Data
```rust
let instruments = client.instruments().await?;
let trigger_range = client.trigger_range("NSE", "RELIANCE").await?;
```

## Error Handling
```rust
match client.holdings().await {
    Ok(holdings) => println!("Success: {:?}", holdings),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Concurrent Operations
```rust
let (holdings, positions) = tokio::try_join!(
    client.holdings(),
    client.positions()
)?;
```
EOF
    
    echo "✅ Documentation generated successfully!"
    echo "📖 Main documentation: doc/${CRATE_NAME}/index.html"
    echo "📚 Quick reference: doc/QUICKREF.md"
    echo "📋 Documentation guide: doc/README.md"
    echo "🌐 Open in browser: file://$(pwd)/doc/${CRATE_NAME}/index.html"
    echo ""
    echo "📊 Documentation statistics:"
    echo "   - Total files: $(find doc -name "*.html" | wc -l)"
    echo "   - Main modules: $(find doc/${CRATE_NAME} -maxdepth 1 -name "*.html" | wc -l)"
    echo "   - Source files: $(find doc/src -name "*.html" | wc -l)"
else
    echo "❌ Documentation generation failed!"
    echo "💡 Try running: cargo doc --help"
    exit 1
fi
