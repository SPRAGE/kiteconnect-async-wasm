#!/usr/bin/env zsh

# Generate documentation for KiteConnect Rust library
echo "🚀 Generating KiteConnect documentation..."

# Clean previous documentation
echo "🧹 Cleaning previous documentation..."
rm -rf doc

# Generate documentation
echo "📚 Building documentation..."
cargo doc --no-deps --document-private-items

# Check if documentation was generated successfully in target/doc
if [ -f "target/doc/kiteconnect/index.html" ]; then
    echo "📁 Copying documentation to root doc/ folder..."
    # Create doc directory if it doesn't exist
    mkdir -p doc
    # Copy all generated documentation to root doc folder
    cp -r target/doc/* doc/
    
    echo "✅ Documentation generated successfully!"
    echo "📖 Documentation available at: doc/kiteconnect/index.html"
    echo "🌐 Open in browser: file://$(pwd)/doc/kiteconnect/index.html"
else
    echo "❌ Documentation generation failed!"
    exit 1
fi
