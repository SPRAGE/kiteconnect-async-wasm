# Version Management Scripts

This directory contains scripts to help manage version bumps and releases.

## Versioning Strategy

This project uses a dual approach for version management with **special handling for major versions**:

### Minor/Patch Versions (0.x.y)
- **Version Branches**: Each version gets its own branch (e.g., `v0.1.7`, `v0.1.8`) for easy rollback and version-specific maintenance
- **Version Tags**: Traditional git tags (e.g., `v0.1.7`) for release automation and referencing
- **Direct to Main**: These versions are committed directly to the main branch

### Major Versions (x.0.0) - Breaking Changes
- **Development Branches**: Major versions create development branches (e.g., `v1.0.0-dev`, `v2.0.0-dev`)
- **Manual Merge Required**: These branches are **NOT** automatically merged to main
- **Explicit Approval**: Only merged to main when breaking changes are thoroughly tested and approved
- **No Automatic Tags**: Tags are only created after manual merge approval

## Usage

### Automatic Version Bump

#### Minor/Patch Versions
```bash
# Bump patch version (0.1.0 -> 0.1.1)
./scripts/bump-version.sh patch

# Bump minor version (0.1.1 -> 0.2.0)
./scripts/bump-version.sh minor

# Set specific minor/patch version
./scripts/bump-version.sh 0.1.5
```

These will:
1. Update version in `Cargo.toml` and `Cargo.lock`
2. Optionally update README.md version references
3. Run tests to ensure everything works
4. Commit changes to current branch
5. Offer to create a version branch (e.g., `v0.1.8`)
6. Offer to create a version tag (e.g., `v0.1.8`)

#### Major Versions (Breaking Changes)
```bash
# Bump major version (0.2.0 -> 1.0.0) - Creates development branch
./scripts/bump-version.sh major

# Set specific major version
./scripts/bump-version.sh 2.0.0
```

⚠️ **Major Version Workflow:**
1. Creates a development branch (e.g., `v1.0.0-dev`)
2. Updates version in `Cargo.toml` and `Cargo.lock`
3. Commits changes to the **development branch** (not main!)
4. **Does NOT create tags automatically**
5. **Does NOT merge to main automatically**

**Next Steps for Major Versions:**
1. Develop breaking changes on the development branch
2. Test thoroughly
3. Update documentation and CHANGELOG.md
4. Create pull request to merge into main when ready
5. Only create release tags after manual approval

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

#### Minor/Patch Version Branches
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

#### Major Version Development Branches
To work with major version development:
```bash
# Switch to major version development branch
git checkout v1.0.0-dev

# Push development branch for collaboration
git push -u origin v1.0.0-dev

# Create pull request when ready (manual process)
# GitHub/GitLab: Create PR from v1.0.0-dev -> main

# After PR approval and merge, create release tag
git checkout main
git tag v1.0.0
git push origin v1.0.0
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

## Benefits of This Strategy

### Version Branches (Minor/Patch)
- **Easy Rollback**: Simply checkout the version branch to return to that exact state
- **Hotfix Support**: Create hotfix branches from specific version branches
- **Version Isolation**: Each version is preserved in its own branch
- **Better Git History**: Clear separation between development and version snapshots

### Development Branches (Major)
- **Breaking Change Safety**: Prevents accidental deployment of breaking changes
- **Thorough Testing**: Forces explicit review and testing of major changes
- **Collaboration**: Team can work together on breaking changes before they affect main
- **Rollback Safety**: Main branch remains stable during major development
- **Release Control**: Only create releases when major changes are truly ready
