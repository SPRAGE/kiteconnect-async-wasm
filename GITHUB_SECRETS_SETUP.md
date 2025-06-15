# GitHub Secrets Setup Guide

This document explains how to configure the required secrets for the GitHub CI/CD workflows.

## Required Secrets

### `CARGO_REGISTRY_TOKEN`

**Purpose:** Allows the release workflow to publish the crate to crates.io automatically.

**Setup Steps:**

1. **Get your crates.io API token:**
   - Go to https://crates.io/me
   - Click "Account Settings"
   - Scroll to "API Tokens" section
   - Click "New Token"
   - Name it something like `github-ci-kiteconnect-async-wasm`
   - Copy the generated token (shown only once!)

2. **Add the token to GitHub:**
   - Go to your repository: https://github.com/SPRAGE/kiteconnect-async-wasm
   - Click "Settings" tab
   - Navigate to "Secrets and variables" → "Actions"
   - Click "New repository secret"
   - Set name as: `CARGO_REGISTRY_TOKEN`
   - Paste the token value
   - Click "Add secret"

3. **Verify setup:**
   - Create a new tag and push it to trigger the release workflow
   - Check the workflow logs to ensure publication succeeds

## Workflow Behavior

### Release Workflow (`release.yml`)

**Triggers:** When you push a tag matching `v*.*.*` (e.g., `v0.1.3`)

**Steps:**
1. **Test:** Runs all tests with native features
2. **Publish:** Publishes to crates.io using the token
3. **GitHub Release:** Creates a GitHub release with changelog notes

**Example trigger:**
```bash
git tag v0.1.3
git push origin v0.1.3
```

### CI Workflow (`ci.yml`)

**Triggers:** On every push and pull request

**Steps:**
1. Tests with different feature combinations
2. Builds for multiple targets (native, WASM)
3. Validates documentation builds

## Troubleshooting

### Error: "a value is required for '--token <TOKEN>' but none was supplied"

**Cause:** The `CARGO_REGISTRY_TOKEN` secret is not configured.

**Solution:** Follow the setup steps above to add the secret.

### Error: "authentication failed"

**Cause:** The token is invalid or expired.

**Solution:** 
1. Generate a new token on crates.io
2. Update the GitHub secret with the new token

### Error: "crate already exists"

**Cause:** You're trying to publish a version that already exists.

**Solution:** 
1. Bump the version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a new tag with the updated version

## Security Notes

- **Never commit API tokens to the repository**
- **Use repository secrets for sensitive values**
- **Tokens are masked in workflow logs**
- **Only repository maintainers can view/edit secrets**
