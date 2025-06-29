# Environment Variables for KiteConnect Examples

All KiteConnect examples now use environment variables for API credentials to improve security. Here's how to set them up:

## Required Environment Variables

### For most examples:
```bash
export KITE_API_KEY=your_actual_api_key
export KITE_ACCESS_TOKEN=your_actual_access_token
```

### For authentication examples (that need API secret):
```bash
export KITE_API_KEY=your_actual_api_key
export KITE_API_SECRET=your_actual_api_secret
export KITE_ACCESS_TOKEN=your_actual_access_token  # Optional, set after login
```

## Setting Up Credentials

### 1. Get your credentials from KiteConnect developer portal
- **API Key**: Available in your app settings
- **API Secret**: Available in your app settings (keep this secure!)
- **Access Token**: Generated after user login (temporary)

### 2. Set environment variables

#### On Linux/macOS:
```bash
# Add to your ~/.bashrc or ~/.zshrc for persistence
export KITE_API_KEY="your_api_key_here"
export KITE_API_SECRET="your_api_secret_here"
export KITE_ACCESS_TOKEN="your_access_token_here"
```

#### On Windows (PowerShell):
```powershell
$env:KITE_API_KEY="your_api_key_here"
$env:KITE_API_SECRET="your_api_secret_here"
$env:KITE_ACCESS_TOKEN="your_access_token_here"
```

#### Using a .env file (if supported):
Create a `.env` file in the project root:
```env
KITE_API_KEY=your_api_key_here
KITE_API_SECRET=your_api_secret_here
KITE_ACCESS_TOKEN=your_access_token_here
```

## Running Examples

Once environment variables are set, you can run any example:

```bash
# Simple historical data example
cargo run --example simple_256265_example

# Comprehensive historical data example
cargo run --example historical_data_256265_example

# Typed historical data example
cargo run --example historical_data_typed_example

# Authentication flow example
cargo run --example connect_sample

# And many more...
cargo run --example <example_name>
```

## Security Best Practices

1. **Never commit credentials** to version control
2. **Use environment variables** instead of hardcoded strings
3. **Keep API secrets secure** - they're like passwords
4. **Rotate access tokens** regularly
5. **Use different credentials** for development and production

## Troubleshooting

### Error: "KITE_API_KEY environment variable not set"
- Make sure you've set the environment variable in your current shell
- Check spelling: it's `KITE_API_KEY` not `KITE_APIKEY`
- Restart your terminal after setting variables in config files

### Error: "KITE_ACCESS_TOKEN environment variable not set"  
- For examples that need user authentication, you may need to run the authentication flow first
- Some examples work with just API key and secret for demonstration purposes

## Example File Updates

All example files have been updated with the pattern:

```rust
use std::env;

/// Get API credentials from environment variables
fn get_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    let api_key = env::var("KITE_API_KEY")
        .map_err(|_| "KITE_API_KEY environment variable not set")?;
    let access_token = env::var("KITE_ACCESS_TOKEN")
        .map_err(|_| "KITE_ACCESS_TOKEN environment variable not set")?;
    Ok((api_key, access_token))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (api_key, access_token) = get_credentials()?;
    let client = KiteConnect::new(&api_key, &access_token);
    // ... rest of example
}
```

This pattern ensures:
- Secure credential handling
- Clear error messages when credentials are missing
- Consistent behavior across all examples
