# Automated Release Workflow

This document explains the new automated GitHub Actions release workflow for `kiteconnect-async-wasm`.

## 🚀 How It Works

### Trigger
The workflow automatically runs when:
- Code is pushed to `main` or `master` branch
- Can also be triggered manually via GitHub UI (`workflow_dispatch`)

### Smart Version Detection
The workflow intelligently detects when a release is needed by:
1. **Reading version from Cargo.toml** - Gets current version from the package metadata
2. **Comparing with previous commit** - Checks if version actually changed
3. **Checking existing tags** - Ensures the tag doesn't already exist
4. **Only proceeding if version changed** - Skips unnecessary runs

### Automated Steps

When a version change is detected, the workflow automatically:

#### 1. 🧪 **Testing & Validation**
```yaml
- Run all tests with `cargo test --all-features`
- Run documentation tests
- Verify build with `cargo build --release --all-features`
- Perform dry-run publish to validate package
```

#### 2. 🏷️ **Git Tag Creation**
```yaml
- Create annotated git tag: `v{VERSION}`
- Push tag to repository
- Include descriptive tag message
```

#### 3. 📦 **crates.io Publishing**
```yaml
- Publish package to crates.io using CARGO_REGISTRY_TOKEN
- Only publishes after all tests pass
- Uses proper error handling and validation
```

#### 4. 🎯 **GitHub Release Creation**
```yaml
- Create GitHub release with tag
- Extract release notes from CHANGELOG.md (if available)
- Generate default release notes if no changelog entry
- Include installation instructions
```

#### 5. 📊 **Summary Report**
```yaml
- Provide comprehensive summary of what happened
- Report version information and actions taken
- Clear success/skip/failure status
```

## 📋 Usage Examples

### Standard Release Process

1. **Update version in Cargo.toml**:
   ```toml
   [package]
   name = "kiteconnect-async-wasm"
   version = "1.0.2"  # <- Change this
   ```

2. **Create development branch and PR**:
   ```bash
   ./scripts/release.sh 1.0.2
   # This creates v1.0.2-dev branch and pushes to GitHub
   ```

3. **Create PR and merge to main**:
   - GitHub UI: Create PR from `v1.0.2-dev` → `main`
   - Review and approve
   - Merge to main

4. **Automatic release happens**:
   - ✅ GitHub Actions detects version change
   - ✅ Runs tests and validation
   - ✅ Creates git tag `v1.0.2`
   - ✅ Publishes to crates.io
   - ✅ Creates GitHub release

### Manual Trigger

You can also trigger the workflow manually:
1. Go to GitHub → Actions → Release workflow
2. Click "Run workflow"
3. Choose the branch (usually `main`)
4. Click "Run workflow" button

## 🔒 Safety Features

### Duplicate Prevention
- **Tag existence check** - Won't create duplicate tags
- **Version comparison** - Only runs when version actually changes
- **Comprehensive testing** - All tests must pass before release

### Error Handling
- **Token validation** - Checks for required `CARGO_REGISTRY_TOKEN`
- **Build verification** - Ensures package builds successfully
- **Publish dry-run** - Validates package before actual publish
- **Clear error messages** - Provides actionable feedback on failures

### Conditional Execution
- **Smart skipping** - Skips unnecessary steps when no release needed
- **Dependency chain** - Each step depends on previous success
- **Rollback safe** - Failed releases don't leave partial state

## 📁 Required Secrets

Ensure these GitHub secrets are configured:

### `CARGO_REGISTRY_TOKEN`
- **Required for**: Publishing to crates.io
- **How to get**: 
  1. Go to https://crates.io/me
  2. Create new token with appropriate scope
  3. Add to GitHub repository secrets
- **Scope needed**: `publish-new` and `publish-update`

### `GITHUB_TOKEN`
- **Required for**: Creating GitHub releases
- **Setup**: Automatically provided by GitHub Actions
- **No manual setup needed**

## 🐛 Troubleshooting

### Workflow doesn't trigger
- ✅ Check if version in Cargo.toml actually changed
- ✅ Ensure push is to `main` or `master` branch
- ✅ Verify workflow file syntax is correct

### Tests fail
- ✅ Run tests locally: `cargo test --all-features`
- ✅ Check build: `cargo build --release --all-features`
- ✅ Fix issues and push new commit

### Publishing fails
- ✅ Verify `CARGO_REGISTRY_TOKEN` secret is set
- ✅ Check token has correct permissions
- ✅ Ensure version doesn't already exist on crates.io
- ✅ Run `cargo publish --dry-run` locally

### Release creation fails
- ✅ Check `GITHUB_TOKEN` permissions
- ✅ Verify tag was created successfully
- ✅ Ensure release notes generation worked

## 📈 Workflow Status Examples

### Successful Release
```
🏷️  Release Summary
==================
Version: 1.0.2
Version changed: true
Tag exists: false
✅ Release v1.0.2 created successfully!
🚀 Published to crates.io
📋 GitHub release created
🏷️  Git tag v1.0.2 created
```

### Skipped (No Version Change)
```
🏷️  Release Summary
==================
Version: 1.0.1
Version changed: false
Tag exists: false
ℹ️  Version unchanged, no release needed
```

### Skipped (Tag Exists)
```
🏷️  Release Summary
==================
Version: 1.0.1
Version changed: true
Tag exists: true
ℹ️  Tag v1.0.1 already exists, skipping release
```

## 🔄 Migration from Manual Process

### Old Process (Manual)
1. Update Cargo.toml
2. Commit changes
3. Create PR and merge
4. Manually run `./scripts/create-tag.sh`
5. Wait for workflow to trigger on tag
6. Manual verification

### New Process (Automated)
1. Update Cargo.toml
2. Create PR using `./scripts/release.sh`
3. Merge PR
4. **Everything else is automatic!** ✨

### Benefits of Automation
- 🚀 **Faster releases** - No manual intervention needed
- 🛡️ **More reliable** - Consistent process every time
- 📊 **Better tracking** - Clear workflow status and logs
- 🔄 **Simpler process** - Fewer manual steps to remember
- 🧪 **Safer** - More comprehensive testing and validation

## 📚 Related Documentation

- [`scripts/VERSION_MANAGEMENT.md`](scripts/VERSION_MANAGEMENT.md) - Complete version management guide
- [`scripts/release.sh`](scripts/release.sh) - Development branch creation script
- [`.github/workflows/release.yml`](.github/workflows/release.yml) - The actual workflow file
- [`scripts/create-tag.sh`](scripts/create-tag.sh) - Manual tag creation (now rarely needed)

---

**🎉 Enjoy automated releases!** The workflow handles all the heavy lifting while maintaining safety and reliability.
