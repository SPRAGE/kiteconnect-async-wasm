# 🚀 Quick Release Reference

## One-Command Release
```bash
# Bump version and trigger automated release
./scripts/bump-version.sh patch && git push origin v$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
```

## Step-by-Step Release
```bash
# 1. Bump version (patch/minor/major)
./scripts/bump-version.sh patch

# 2. Push the created tag
git push origin v0.1.1  # replace with actual version

# 3. Monitor release at:
# https://github.com/SPRAGE/kiteconnect-async-wasm/actions
```

## Version Types
- `patch`: Bug fixes (0.1.0 → 0.1.1)
- `minor`: New features (0.1.0 → 0.2.0)  
- `major`: Breaking changes (0.1.0 → 1.0.0)
- `1.2.3`: Specific version

## What Happens Automatically
✅ Version updated in Cargo.toml  
✅ Cargo.lock updated  
✅ Tests run  
✅ Git commit created  
✅ Git tag created  
✅ Push tag → GitHub Actions → crates.io publish

## Emergency Manual Publish
```bash
cargo publish --token $CARGO_REGISTRY_TOKEN
```
