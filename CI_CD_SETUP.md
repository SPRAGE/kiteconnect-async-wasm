# CI/CD Setup Guide

This guide explains how to set up automated publishing to crates.io and version management for your Rust crate.

## 🚀 Automated Publishing Setup

### 1. GitHub Secrets Configuration

You need to configure the following secrets in your GitHub repository:

#### Required Secrets:
- `CARGO_REGISTRY_TOKEN`: Your crates.io API token for publishing

#### How to set up secrets:
1. Go to your repository on GitHub: https://github.com/SPRAGE/kiteconnect-async-wasm
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add the following:

**CARGO_REGISTRY_TOKEN**:
```bash
# Get your crates.io token (you already have this from earlier)
# Go to https://crates.io/settings/tokens
# Copy your existing token or create a new one
# Paste it as the secret value
```

### 2. Workflow Overview

The CI/CD pipeline consists of two main workflows:

#### CI Workflow (`.github/workflows/ci.yml`)
Triggered on every push and pull request:
- ✅ **Multi-Rust version testing** (stable, beta, nightly)
- ✅ **WASM compatibility testing**
- ✅ **Code formatting** (rustfmt)
- ✅ **Linting** (clippy)
- ✅ **Documentation generation**
- ✅ **Security audit** (cargo-audit)

#### Release Workflow (`.github/workflows/release.yml`)
Triggered when you push a version tag (e.g., `v0.1.1`):
- ✅ **Pre-release testing**
- ✅ **Version verification** (ensures tag matches Cargo.toml)
- ✅ **Dry-run publish** (safety check)
- ✅ **Automatic crates.io publishing**
- ✅ **GitHub release creation**

## 📋 Version Management

### Semantic Versioning
This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

### Automated Version Bumping

Use the provided script for version management:

```bash
# Bump patch version (0.1.0 → 0.1.1)
./scripts/bump-version.sh patch

# Bump minor version (0.1.0 → 0.2.0)
./scripts/bump-version.sh minor

# Bump major version (0.1.0 → 1.0.0)
./scripts/bump-version.sh major

# Set specific version
./scripts/bump-version.sh 1.2.3
```

The script automatically:
- ✅ Updates `Cargo.toml` version
- ✅ Updates `Cargo.lock`
- ✅ Optionally updates `README.md` version references
- ✅ Runs tests to verify changes
- ✅ Commits changes with proper message
- ✅ Optionally creates git tag

## 🎯 Release Process

### Quick Release (Recommended)
```bash
# 1. Bump version and create tag
./scripts/bump-version.sh patch  # or minor/major

# 2. Push the tag to trigger release
git push origin v0.1.1  # replace with your version
```

### Manual Release Process
```bash
# 1. Update version manually in Cargo.toml
# 2. Update CHANGELOG.md
./scripts/create-release.sh

# 3. Push tag
git push origin v0.1.1
```

### What Happens After Pushing a Tag?

1. **GitHub Actions Triggered**: The release workflow starts automatically
2. **Tests Run**: All tests must pass before publishing
3. **Version Verification**: Ensures tag version matches Cargo.toml
4. **Crates.io Publishing**: Automatically publishes to crates.io
5. **GitHub Release**: Creates a GitHub release with changelog
6. **Notifications**: You'll receive email notifications about the process

## 🔍 Monitoring Releases

### Check Release Status
- **GitHub Actions**: https://github.com/SPRAGE/kiteconnect-async-wasm/actions
- **Crates.io**: https://crates.io/crates/kiteconnect-async-wasm
- **GitHub Releases**: https://github.com/SPRAGE/kiteconnect-async-wasm/releases

### Troubleshooting Common Issues

#### 1. Version Mismatch Error
```
Version mismatch between Cargo.toml (0.1.1) and tag (0.1.0)
```
**Solution**: Ensure the tag version matches the version in `Cargo.toml`

#### 2. Crates.io Authentication Error
```
error: failed to publish to registry at https://crates.io/
```
**Solution**: Check that `CARGO_REGISTRY_TOKEN` secret is correctly set

#### 3. Tests Failing
**Solution**: Fix test failures locally before pushing tags

#### 4. Duplicate Version Error
```
error: crate version `0.1.1` is already uploaded
```
**Solution**: You cannot republish the same version. Bump to next version.

## 📊 Advanced Features

### Pre-release Versions
For beta releases, use pre-release identifiers:
```bash
./scripts/bump-version.sh 0.2.0-beta.1
git tag v0.2.0-beta.1
git push origin v0.2.0-beta.1
```

### Hotfix Releases
For urgent fixes:
```bash
# Create hotfix branch from last release tag
git checkout v0.1.0
git checkout -b hotfix/0.1.1

# Make fixes, then:
./scripts/bump-version.sh 0.1.1
git push origin v0.1.1
```

### Manual Publishing (Emergency)
If automated publishing fails:
```bash
# Publish manually (after setting CARGO_REGISTRY_TOKEN locally)
cargo publish --token $CARGO_REGISTRY_TOKEN
```

## 🛡️ Security Best Practices

1. **Token Rotation**: Regularly rotate your crates.io API token
2. **Scope Limitation**: Use tokens with minimal required permissions
3. **Branch Protection**: Enable branch protection on main branch
4. **Review Process**: Require PR reviews for version bumps
5. **Audit Dependencies**: Regular `cargo audit` runs (included in CI)

## 📈 Maintenance

### Regular Tasks
- **Monthly**: Review and update dependencies
- **Quarterly**: Rotate API tokens
- **Per Release**: Update CHANGELOG.md
- **As Needed**: Review and improve CI/CD pipeline

This setup provides a professional, automated release pipeline that ensures consistent, reliable publishing to crates.io while maintaining high code quality standards.
