# Version Management Scripts

This directory contains scripts to help manage version bumps and releases.

## Versioning Strategy

This project uses a dual approach for version management:
- **Version Branches**: Each version gets its own branch (e.g., `v0.1.7`, `v0.1.8`) for easy rollback and version-specific maintenance
- **Version Tags**: Traditional git tags (e.g., `v0.1.7`) for release automation and referencing

## Usage

### Automatic Version Bump
```bash
# Bump patch version (0.1.0 -> 0.1.1)
./scripts/bump-version.sh patch

# Bump minor version (0.1.1 -> 0.2.0)
./scripts/bump-version.sh minor

# Bump major version (0.2.0 -> 1.0.0)
./scripts/bump-version.sh major

# Set specific version
./scripts/bump-version.sh 0.1.5
```

The script will:
1. Update version in `Cargo.toml` and `Cargo.lock`
2. Optionally update README.md version references
3. Run tests to ensure everything works
4. Commit the changes
5. Offer to create a version branch (e.g., `v0.1.8`)
6. Offer to create a version tag (e.g., `v0.1.8`)

### Manual Release Process
```bash
# 1. Bump version
./scripts/bump-version.sh patch

# 2. Update changelog
./scripts/update-changelog.sh

# 3. Create release
./scripts/create-release.sh
```

### Working with Version Branches

To revert to a specific version:
```bash
# Check out a specific version branch
git checkout v0.1.6

# Create a hotfix branch from a version branch
git checkout v0.1.6
git checkout -b hotfix/v0.1.6-security-fix

# List all version branches
git branch -r | grep "origin/v"
```

## Automated Release Process

The automated release process is triggered by pushing a git tag:

```bash
# After bumping version and committing changes
git tag v0.1.1
git push origin v0.1.1

# Optionally push the version branch too
git push origin v0.1.1
```

This will automatically:
1. Run all tests
2. Verify version consistency
3. Publish to crates.io
4. Create GitHub release

## Benefits of Version Branches

- **Easy Rollback**: Simply checkout the version branch to return to that exact state
- **Hotfix Support**: Create hotfix branches from specific version branches
- **Version Isolation**: Each version is preserved in its own branch
- **Better Git History**: Clear separation between development and version snapshots
