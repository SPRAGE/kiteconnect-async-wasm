#!/usr/bin/env bash

# Script to bump version numbers in Cargo.toml
# Usage: ./bump-version.sh [patch|minor|major|VERSION]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_major() {
    echo -e "${BLUE}[MAJOR]${NC} $1"
}

print_branch() {
    echo -e "${CYAN}[BRANCH]${NC} $1"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    print_error "Working directory is not clean. Please commit or stash changes first."
    exit 1
fi

# Get current branch
current_branch=$(git branch --show-current 2>/dev/null || git rev-parse --short HEAD)

# Get current version from Cargo.toml
current_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "Current version: $current_version"
print_info "Current branch: $current_branch"

# Parse version components
IFS='.' read -r major minor patch <<< "$current_version"

# Variables to track version bump type
is_major_bump=false
is_minor_bump=false
version_bump_type=""

# Determine new version
case "$1" in
    "patch")
        new_patch=$((patch + 1))
        new_version="$major.$minor.$new_patch"
        version_bump_type="patch"
        ;;
    "minor")
        new_minor=$((minor + 1))
        new_version="$major.$new_minor.0"
        version_bump_type="minor"
        is_minor_bump=true
        ;;
    "major")
        new_major=$((major + 1))
        new_version="$new_major.0.0"
        version_bump_type="major"
        is_major_bump=true
        ;;
    "")
        print_error "Please specify version bump type: patch, minor, major, or specific version"
        print_info "Usage: $0 [patch|minor|major|VERSION]"
        exit 1
        ;;
    *)
        # Check if it's a valid semantic version
        if [[ $1 =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            new_version="$1"
            # Determine what type of bump this is
            new_major=$(echo "$1" | cut -d'.' -f1)
            new_minor=$(echo "$1" | cut -d'.' -f2)
            new_patch=$(echo "$1" | cut -d'.' -f3)
            
            if [[ $new_major -gt $major ]]; then
                is_major_bump=true
                version_bump_type="major"
            elif [[ $new_minor -gt $minor ]]; then
                is_minor_bump=true
                version_bump_type="minor"
            else
                version_bump_type="patch"
            fi
        else
            print_error "Invalid version format. Use semantic versioning (e.g., 1.2.3)"
            exit 1
        fi
        ;;
esac

print_info "New version: $new_version"
print_info "Bump type: $version_bump_type"

# All version bumps now create development branches
if [[ $is_major_bump == true ]]; then
    print_major "⚠️  MAJOR VERSION BUMP DETECTED ⚠️"
    print_major "This introduces breaking changes and will create a development branch."
elif [[ $is_minor_bump == true ]]; then
    print_warning "🔄 MINOR VERSION BUMP DETECTED"
    print_warning "This introduces new features and will create a development branch."
else
    print_info "🔧 PATCH VERSION BUMP DETECTED"
    print_info "This introduces bug fixes and will create a development branch."
fi

print_warning "📋 ALL VERSION UPDATES require manual approval before merging to main!"
echo

# Confirm the change
read -p "Do you want to bump version from $current_version to $new_version? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Version bump cancelled"
    exit 0
fi

# Create development branch for ALL version bumps
dev_branch="v$new_version-dev"

# Check if branch already exists
if git show-ref --verify --quiet "refs/heads/$dev_branch"; then
    print_error "Branch $dev_branch already exists. Please handle it manually or delete it first."
    exit 1
fi

print_info "Creating development branch: $dev_branch"
git checkout -b "$dev_branch"
print_branch "Switched to new development branch: $dev_branch"

if [[ $is_major_bump == true ]]; then
    print_major "🚨 BREAKING CHANGES: This branch contains breaking API changes!"
elif [[ $is_minor_bump == true ]]; then
    print_warning "✨ NEW FEATURES: This branch contains new features!"
else
    print_info "🔧 BUG FIXES: This branch contains bug fixes!"
fi

print_warning "📋 Remember: This branch will NOT be merged to main automatically!"
echo

# Update Cargo.toml
print_info "Updating Cargo.toml..."
sed -i "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml

# Update Cargo.lock
print_info "Updating Cargo.lock..."
cargo update --workspace

# Check if README.md contains version references and offer to update them
if grep -q "kiteconnect-async-wasm.*=.*\"$current_version\"" README.md 2>/dev/null; then
    print_warning "Found version references in README.md"
    read -p "Update README.md version references? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        sed -i "s/kiteconnect-async-wasm.*=.*\"$current_version\"/kiteconnect-async-wasm = \"$new_version\"/" README.md
        print_info "Updated README.md"
    fi
fi

# Run tests to make sure everything still works
print_info "Running tests..."
if cargo test --quiet; then
    print_info "All tests passed"
else
    print_error "Tests failed. Please fix issues before committing."
    if [[ $is_major_bump == true ]]; then
        print_warning "You can fix the tests and commit manually on the development branch."
    fi
    exit 1
fi

# Commit the changes
print_info "Committing changes..."
git add Cargo.toml Cargo.lock
if git diff --staged --quiet README.md 2>/dev/null; then
    git add README.md
fi

# Create commit message based on version bump type
if [[ $is_major_bump == true ]]; then
    commit_message="🚀 Major version bump to $new_version

This is a breaking change that introduces new APIs or removes/changes existing ones.
Development branch: $(git branch --show-current)

⚠️  BREAKING CHANGES - DO NOT MERGE TO MAIN WITHOUT EXPLICIT APPROVAL ⚠️
    
Changes include:
- Breaking API modifications
- Potential migration requirements
- Updated documentation needed"
elif [[ $is_minor_bump == true ]]; then
    commit_message="✨ Minor version bump to $new_version

This introduces new features while maintaining backward compatibility.
Development branch: $(git branch --show-current)

🔄 NEW FEATURES - DO NOT MERGE TO MAIN WITHOUT EXPLICIT APPROVAL

Changes include:
- New functionality added
- Enhanced features
- Backward compatible changes"
else
    commit_message="🔧 Patch version bump to $new_version

This introduces bug fixes and improvements.
Development branch: $(git branch --show-current)

🔧 BUG FIXES - DO NOT MERGE TO MAIN WITHOUT EXPLICIT APPROVAL

Changes include:
- Bug fixes
- Performance improvements
- Security patches"
fi

git commit -m "$commit_message"

# All version bumps now follow the same workflow: development branch only
print_info "🎯 Version Bump Workflow Complete!"
print_branch "You are now on development branch: $(git branch --show-current)"
echo

print_warning "📋 Next Steps for $version_bump_type Version $new_version:"
print_info "1. Develop and test your changes on this branch"
print_info "2. Update documentation and CHANGELOG.md"
print_info "3. Run comprehensive tests"
print_info "4. When ready, create a pull request to merge into main"
print_info "5. After merge to main, the release process will:"
print_info "   - Create the git tag"
print_info "   - Publish to crates.io"
print_info "   - Generate release notes"
echo

print_info "To push this development branch:"
print_info "git push -u origin $(git branch --show-current)"
echo

print_warning "🚨 IMPORTANT WORKFLOW NOTES:"
print_warning "   • This branch will NOT be merged to main automatically"
print_warning "   • NO tags will be created until merged to main"
print_warning "   • Crate will NOT be published until merged to main"
print_warning "   • You must manually approve the merge via pull request"
echo

if [[ $is_major_bump == true ]]; then
    print_major "🚨 BREAKING CHANGES DETECTED:"
    print_major "   • Review all API changes carefully"
    print_major "   • Update migration documentation"
    print_major "   • Consider deprecation warnings"
elif [[ $is_minor_bump == true ]]; then
    print_warning "✨ NEW FEATURES ADDED:"
    print_warning "   • Ensure backward compatibility"
    print_warning "   • Document new functionality"
    print_warning "   • Add appropriate tests"
else
    print_info "🔧 BUG FIXES APPLIED:"
    print_info "   • Verify fixes work as expected"
    print_info "   • Check for regressions"
    print_info "   • Update relevant tests"
fi
