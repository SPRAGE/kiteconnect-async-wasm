# Automated Releases Guide

This repository uses GitHub Actions to automatically create releases when changes are merged to the main branch.

## How It Works

The automated release workflow is simple and robust:

1. **Trigger**: Runs automatically when code is pushed to `main` or `master` branch
2. **Version Check**: Reads the current version from `Cargo.toml`
3. **Tag Check**: Checks if a git tag already exists for that version (e.g., `v1.0.1`)
4. **Release Process**: If no tag exists, creates the full release automatically

## Workflow Steps

When a tag doesn't exist for the current version:

1. **🧪 Test**: Run all tests and verify the build
2. **🏷️ Create Tag**: Create and push a git tag (e.g., `v1.0.1`)
3. **📦 Publish**: Publish to crates.io automatically
4. **📋 Release**: Create a GitHub release with auto-generated notes

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

## Creating a Release

### Simple Method (Recommended)

1. **Update Version**: Edit the version in `Cargo.toml`
   ```toml
   [package]
   name = "kiteconnect-async-wasm"
   version = "1.0.2"  # ← Change this
   ```

2. **Commit & Push**: Push your changes to main
   ```bash
   git add Cargo.toml
   git commit -m "Release v1.0.2"
   git push origin main
   ```

3. **Automatic Release**: The workflow automatically:
   - ✅ Runs tests
   - ✅ Creates tag `v1.0.2`
   - ✅ Publishes to crates.io
   - ✅ Creates GitHub release

### Using Development Branches

For safer releases, use the provided scripts:

```bash
# Create a development branch for version 1.0.2
./scripts/release.sh 1.0.2

# This will:
# 1. Create branch v1.0.2-dev
# 2. Update Cargo.toml
# 3. Commit and push
# 4. Create a pull request

# When you merge the PR → automatic release!
```

## Release Scenarios

### ✅ Normal Release
- Version in `Cargo.toml`: `1.0.1`
- Existing tags: `v1.0.0`
- **Result**: Creates `v1.0.1` tag and full release

### ⏭️ Skipped Release
- Version in `Cargo.toml`: `1.0.1`
- Existing tags: `v1.0.0`, `v1.0.1`
- **Result**: Skips release (tag already exists)

### 🔄 Re-running
- If a workflow fails, just re-run it from GitHub Actions
- Safe to run multiple times (won't create duplicate releases)

## Requirements

### GitHub Secrets

The workflow requires one secret to be set in your repository:

- `CARGO_REGISTRY_TOKEN`: Your crates.io API token

#### Setting up the Token

1. Go to [crates.io/me](https://crates.io/me)
2. Generate a new API token
3. In GitHub: Settings → Secrets and variables → Actions
4. Add secret: `CARGO_REGISTRY_TOKEN` = `your_token_here`

## Workflow Status

### Success ✅
```
🏷️ Release Summary
==================
Version: 1.0.1
Tag exists: false
✅ Release v1.0.1 created successfully!
🚀 Published to crates.io
📋 GitHub release created  
🏷️ Git tag v1.0.1 created
```

### Skipped ⏭️
```
🏷️ Release Summary
==================
Version: 1.0.1
Tag exists: true
ℹ️ Tag v1.0.1 already exists, skipping release
```

## Troubleshooting

### Workflow Not Running
- **Check**: Is your change pushed to `main` or `master`?
- **Check**: Are you on the correct branch?

### Publication Failed
- **Check**: Is `CARGO_REGISTRY_TOKEN` secret set correctly?
- **Check**: Do you have permissions to publish this crate?
- **Check**: Is the version number valid and greater than published versions?

### Tag Already Exists
- **Normal**: This prevents duplicate releases
- **Override**: Delete the tag if you need to recreate the release:
  ```bash
  git tag -d v1.0.1
  git push origin :refs/tags/v1.0.1
  ```

### Tests Failing
- The workflow will stop if tests fail
- Fix tests and push again to retry

## Manual Trigger

You can also trigger releases manually:

1. Go to **Actions** tab in GitHub
2. Select **Release** workflow
3. Click **Run workflow**
4. Choose the branch and click **Run workflow**

## Release Notes

The workflow automatically generates release notes:

1. **From CHANGELOG.md**: If it exists, extracts the section for the current version
2. **Auto-generated**: If no changelog, creates standard release notes with:
   - Installation instructions
   - Link to documentation
   - Basic version information

## Version Management

### Semantic Versioning
Follow [Semantic Versioning](https://semver.org/):
- `1.0.0` → `1.0.1`: Patch (bug fixes)
- `1.0.0` → `1.1.0`: Minor (new features)
- `1.0.0` → `2.0.0`: Major (breaking changes)

### Pre-release Versions
Use suffixes for pre-releases:
- `1.1.0-alpha.1`
- `1.1.0-beta.2`
- `1.1.0-rc.1`

## Migration from Manual Process

If you were previously creating releases manually:

1. **Remove Manual Tags**: Delete any manual tags that don't match published versions
2. **Update Cargo.toml**: Ensure the version matches your intended next release
3. **Push to Main**: The workflow will handle the rest

## Benefits

### ✅ Advantages
- **Zero Manual Work**: Just update version and push
- **Consistent Process**: Same steps every time
- **Error Prevention**: Tests must pass before release
- **No Duplicate Releases**: Tag checking prevents accidents
- **Complete Automation**: Tags, publishing, and releases all handled

### 🔒 Safety Features
- **Test First**: Won't release if tests fail
- **Duplicate Prevention**: Checks existing tags
- **Dry Run**: Tests publishing before actual publish
- **Rollback Friendly**: Easy to delete tags and retry

## Support

If you encounter issues with the automated release process:

1. Check the **Actions** tab for detailed logs
2. Verify your `CARGO_REGISTRY_TOKEN` secret
3. Ensure your version number follows semantic versioning
4. Check that tests pass locally before pushing

The automated system is designed to be safe and reliable. When in doubt, the workflow will skip rather than create an incorrect release.
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
