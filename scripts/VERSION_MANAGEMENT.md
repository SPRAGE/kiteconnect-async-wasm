# Version Management Scripts

This directory contains scripts to help manage version bumps and releases.

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

### Manual Release Process
```bash
# 1. Bump version
./scripts/bump-version.sh patch

# 2. Update changelog
./scripts/update-changelog.sh

# 3. Create release
./scripts/create-release.sh
```

## Automated Release Process

The automated release process is triggered by pushing a git tag:

```bash
# After bumping version and committing changes
git tag v0.1.1
git push origin v0.1.1
```

This will automatically:
1. Run all tests
2. Verify version consistency
3. Publish to crates.io
4. Create GitHub release
