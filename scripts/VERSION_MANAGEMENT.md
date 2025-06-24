# Version Management and Release Process

This document outlines the **secure version management strategy** for kiteconnect-async-wasm.

## 🔄 Secure Release Workflow

**ALL version updates require manual approval before merging to main!**

### Core Principles

1. **Development Branch Creation**: All version bumps (patch, minor, major) create development branches
2. **No Auto-Merge**: No changes are automatically merged to main
3. **Manual Review**: All changes must be reviewed and manually merged via pull requests  
4. **Release Only on Main**: Tags and crate publishing only happen after merging to main
5. **Comprehensive Validation**: Multiple safety checks before any release

### Key Benefits

- 🛡️ **Security**: No accidental releases to main branch
- 🔍 **Code Review**: All changes reviewed before merging
- 🧪 **Testing**: Comprehensive validation at every step
- 📋 **Documentation**: Required updates to CHANGELOG.md
- 🚀 **Quality**: Only thoroughly tested code reaches production

## 📋 Detailed Workflow

### 1. Development Phase

When you need to bump a version:

```bash
# For patch versions (bug fixes)
./scripts/bump-version.sh patch

# For minor versions (new features, backward compatible)  
./scripts/bump-version.sh minor

# For major versions (breaking changes)
./scripts/bump-version.sh major

# For specific version
./scripts/bump-version.sh 2.1.0

# OR use the new release preparation script
./scripts/release.sh           # Uses current Cargo.toml version
./scripts/release.sh 2.1.0     # Uses specified version
```

**What happens:**
- ✅ Creates development branch: `v{VERSION}-dev`
- ✅ Updates `Cargo.toml` and `Cargo.lock`
- ✅ Runs comprehensive tests
- ✅ Commits changes with detailed message
- ❌ **NO tags created yet**
- ❌ **NO publishing to crates.io yet**
- ❌ **NO merging to main**

### 2. Development on Branch

- **Work on your changes** on the development branch
- **Add features** (minor versions)
- **Fix bugs** (patch versions)
- **Make breaking changes** (major versions)
- **Update documentation**
- **Add/update tests**

```bash
# Push development branch for collaboration
git push -u origin v{VERSION}-dev
```

### 3. Review & Merge Phase

```bash
# Create Pull Request to merge development branch to main
# This requires manual review and approval
```

**Pull Request Requirements:**
- 🔍 **Code Review**: Team review of all changes
- 🧪 **Test Validation**: All tests must pass
- 📚 **Documentation**: CHANGELOG.md must be updated
- ✅ **Approval**: Explicit approval required

### 4. Release Phase (Main Branch Only)

After merging to main:

```bash
# Switch to main and run tag creation script
git checkout main
git pull origin main
./scripts/create-tag.sh
```

**Tag Creation Script Actions:**
- ✅ Verifies you're on main branch
- ✅ Runs comprehensive tests
- ✅ Validates package can build and publish
- ✅ Creates git tag `v{VERSION}`
- ✅ Pushes tag to origin
- 🤖 **GitHub Actions automatically triggered**

**GitHub Actions Workflow (Automated):**
- ✅ Runs tests on the tagged commit
- ✅ Verifies version consistency
- ✅ Publishes to crates.io
- ✅ Creates GitHub release with release notes

## 📊 Version Types

### Patch Version (x.y.Z)
- **Purpose**: Bug fixes, security patches, performance improvements
- **Branch**: `v{VERSION}-dev` (e.g., `v0.1.5-dev`)
- **Compatibility**: Fully backward compatible
- **Example**: `0.1.4` → `0.1.5`

### Minor Version (x.Y.0)
- **Purpose**: New features, enhancements, non-breaking changes
- **Branch**: `v{VERSION}-dev` (e.g., `v0.2.0-dev`) 
- **Compatibility**: Backward compatible, new functionality added
- **Example**: `0.1.4` → `0.2.0`

### Major Version (X.0.0)
- **Purpose**: Breaking changes, API modifications, architectural changes
- **Branch**: `v{VERSION}-dev` (e.g., `v1.0.0-dev`)
- **Compatibility**: **Breaking changes** - may require user code updates
- **Example**: `0.1.4` → `1.0.0`

## 🔧 Script Details

### bump-version.sh
**Creates development branches for ALL version types**

Features:
- ✅ Creates isolated development branch
- ✅ Updates Cargo.toml and Cargo.lock  
- ✅ Handles README.md version references
- ✅ Runs tests before committing
- ✅ Provides clear next steps
- ✅ Prevents accidental main branch changes

### release.sh  
**Prepares development branches and creates PRs**

Features:
- 🚀 **Creates or switches to development branch**: `v{VERSION}-dev`
- 🔧 **Updates version**: Automatically updates Cargo.toml if needed
- 🧪 **Comprehensive validation**: Tests, builds, publish dry-run
- 📤 **GitHub integration**: Pushes branch and provides PR creation guidance
- 🛡️ **Safety checks**: Multiple validation steps before pushing
- 🤖 **PR templates**: Provides detailed PR description templates

### create-tag.sh  
**Handles tag creation ONLY from main branch (Post-PR-merge)**

Features:
- 🔒 **Main branch enforcement**: Only works on main/master
- 🧪 **Final validation**: Tests, builds, publish dry-run
- 🏷️ **Tag creation**: Creates annotated git tags
- 📤 **Automatic trigger**: Pushes tag to trigger GitHub Actions
- 🤖 **Hands-off publishing**: GitHub Actions handles crates.io publishing

## 🚨 Important Safeguards

### Automatic Protections
- ✅ **Branch Protection**: Development branches cannot auto-merge to main
- ✅ **Main Branch Only**: Tag creation script only works on main/master branch
- ✅ **Duplicate Prevention**: Cannot create tags that already exist
- ✅ **Test Validation**: All tests must pass before release
- ✅ **Build Verification**: Package must build successfully
- ✅ **Publish Validation**: Dry-run validation before actual publishing
- 🤖 **GitHub Actions**: Automated publishing with additional safety checks

### Manual Checkpoints
- 🔍 **Code Review**: All changes reviewed in pull requests
- 🔍 **Version Approval**: Manual approval required for all version bumps
- 🔍 **Release Approval**: Manual confirmation required before publishing
- 🔍 **Documentation**: CHANGELOG.md updates required

## 📚 Complete Examples

### Example: Patch Release (Bug Fix)
```bash
# 1. Create development branch and prepare release
./scripts/release.sh patch  # Creates v0.1.5-dev branch, updates version, runs tests

# 2. Fix bugs on the development branch (if needed)
git add .
git commit -m "Fix critical API timeout issue"

# 3. Branch is automatically pushed to GitHub with PR guidance

# 4. Create PR via GitHub UI: v0.1.5-dev → main

# 5. After PR review and approval, merge to main
git checkout main
git pull origin main

# 6. Create release tag to trigger automated publishing
./scripts/create-tag.sh  # Creates tag, GitHub Actions publishes to crates.io
```

### Example: Minor Release (New Features)
```bash
# 1. Create development branch and prepare release
./scripts/release.sh minor  # Creates v0.2.0-dev branch, updates version

# 2. Add new features
git add .
git commit -m "Add new trading indicators API"

# 3. Update documentation
# Edit CHANGELOG.md, README.md, etc.

# 4. Branch is automatically pushed with PR creation guidance

# 5. Create detailed PR with feature documentation

# 6. After thorough review and approval
git checkout main
git pull origin main

# 7. Create tag to trigger automated release
./scripts/create-tag.sh  # GitHub Actions handles publishing
```

### Example: Major Release (Breaking Changes)
```bash
# 1. Create major version development branch
./scripts/release.sh major  # Creates v1.0.0-dev branch, updates version

# 2. Implement breaking changes
# - Modify APIs
# - Restructure modules  
# - Update error handling

# 3. Comprehensive testing and documentation
# - Update all documentation
# - Add migration guides
# - Update examples

# 4. Branch is automatically pushed with detailed PR guidance

# 5. Create detailed PR with breaking changes documentation
# Include:
# - Migration guide
# - Breaking changes list
# - Updated examples

# 6. After extensive review and team approval
git checkout main
git pull origin main

# 7. Create major version tag
./scripts/create-tag.sh  # GitHub Actions publishes v1.0.0 automatically
```

## 🛡️ Security & Quality Assurance

### Pre-Release Validation
- ✅ All tests pass (`cargo test --all-features`)
- ✅ Code builds successfully (`cargo build --release --all-features`)
- ✅ Package validates (`cargo publish --dry-run`)
- ✅ Documentation is updated and accurate
- ✅ CHANGELOG.md reflects all changes

### Branch Protection Strategy
- 🔒 Main branch requires pull request reviews
- 🔒 Development branches cannot directly merge to main
- 🔒 Release script enforces main branch requirement
- 🔒 Tags can only be created from main branch
- 🔒 No force pushes allowed to main

### Publication Safety
- 🔐 Manual confirmation required before tag creation
- 🔐 Comprehensive validation before tag push
- 🤖 GitHub Actions handles actual publishing with additional checks
- 🔐 Immutable releases (cannot delete from crates.io)
- 🔐 Clear audit trail via git tags and commits
- 🔐 Rollback strategy for emergency situations

## 🔄 Emergency Procedures

### Emergency Hotfix
```bash
# 1. Create hotfix from latest release tag
git checkout v0.1.4
./scripts/release.sh 0.1.5  # Creates v0.1.5-dev branch

# 2. Apply minimal fix
git add .
git commit -m "Emergency fix for security vulnerability"

# 3. Fast-track review process
# Branch is automatically pushed to GitHub

# 4. Create urgent PR with security team review

# 5. After approval, create tag immediately
git checkout main
git pull origin main
./scripts/create-tag.sh  # GitHub Actions handles emergency publishing
```

### Version Rollback
```bash
# If a release has issues, create a new patch version
# Cannot delete from crates.io, must release new version
./scripts/release.sh patch
# Fix issues, create PR, merge, then create tag for automated publishing
```

This workflow ensures maximum security, quality, and control over all releases while maintaining a clear audit trail and preventing accidental deployments.

## 🤖 GitHub Actions Integration

The new workflow leverages GitHub Actions for automated publishing, providing additional safety and consistency:

### Automated Release Pipeline

1. **Developer creates development branch**: `./scripts/release.sh`
2. **PR creation and review**: Manual process with team oversight
3. **PR merge to main**: After approval and testing
4. **Tag creation**: `./scripts/create-tag.sh` (manual trigger on main)
5. **GitHub Actions triggered**: Automatic publishing pipeline

### GitHub Actions Workflow Features

- **Comprehensive Testing**: Runs full test suite on tagged commit
- **Version Validation**: Ensures Cargo.toml version matches git tag
- **Build Verification**: Confirms package builds successfully
- **Dry Run Publishing**: Validates package before actual publish
- **Automatic crates.io Publishing**: Uses stored CARGO_REGISTRY_TOKEN
- **GitHub Release Creation**: Automatically creates GitHub release with CHANGELOG.md extraction
- **Error Handling**: Proper error reporting and rollback procedures

### Benefits of GitHub Actions Integration

- 🛡️ **Consistent Environment**: All releases built in identical CI environment
- 🤖 **Reduced Human Error**: Automated process eliminates manual publishing mistakes
- 🔍 **Audit Trail**: Complete history of release actions in GitHub Actions logs
- 🔒 **Secure Token Management**: API tokens stored securely in GitHub Secrets
- 📋 **Standardized Process**: Every release follows identical validation steps
- ⚡ **Fast Publishing**: Immediate publishing after tag creation
- 🔄 **Reliable Rollback**: Clear process for handling failed releases

### Script Responsibilities

| Script | Purpose | Triggers | Output |
|--------|---------|----------|---------|
| `bump-version.sh` | Version bumping | Manual | Development branch |
| `release.sh` | Release preparation | Manual | Development branch + GitHub push |
| `create-tag.sh` | Tag creation | Manual (post-merge) | Git tag + GitHub Actions trigger |
| GitHub Actions | Publishing | Tag push | crates.io + GitHub release |

This separation of concerns ensures that:
- Development work is isolated in branches
- All changes go through PR review
- Publishing is automated and consistent
- Manual oversight is maintained where needed
- Emergency procedures remain available

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
