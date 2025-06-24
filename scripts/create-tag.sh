#!/usr/bin/env bash

# Script to create release tags on main branch after PR merge
# This script should ONLY be run on the main branch after merging development branches
# Usage: ./create-tag.sh [version] or just ./create-tag.sh to use Cargo.toml version

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

print_release() {
    echo -e "${BLUE}[RELEASE]${NC} $1"
}

print_tag() {
    echo -e "${CYAN}[TAG]${NC} $1"
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

# Ensure we're on the main/master branch
if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
    print_error "Tag creation script must be run on the main/master branch!"
    print_error "Current branch: $current_branch"
    print_error "Please merge your development branch to main first."
    exit 1
fi

print_info "Current branch: $current_branch ✓"

# Accept version as parameter or get from Cargo.toml
if [ $# -eq 1 ]; then
    target_version="$1"
else
    target_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
fi

print_info "Target version: $target_version"

# Check if tag already exists
if git tag -l | grep -q "^v$target_version$"; then
    print_error "Tag v$target_version already exists!"
    print_error "This version has already been released."
    print_info "Available tags:"
    git tag -l | sort -V
    exit 1
fi

# Verify the version follows semantic versioning
if [[ ! $target_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format: $target_version"
    print_error "Expected semantic versioning format (e.g., 1.2.3)"
    exit 1
fi

print_release "🏷️  Preparing to create release tag for version $target_version"
echo

# Run tests to ensure everything is working
print_info "Running comprehensive tests..."
if ! cargo test --all-features; then
    print_error "Tests failed! Cannot proceed with tag creation."
    exit 1
fi
print_info "All tests passed ✓"

# Check if we can build successfully
print_info "Verifying build..."
if ! cargo build --release --all-features; then
    print_error "Build failed! Cannot proceed with tag creation."
    exit 1
fi
print_info "Build successful ✓"

# Verify we can publish (dry run)
print_info "Verifying package can be published (dry run)..."
if ! cargo publish --dry-run; then
    print_error "Package validation failed! Cannot proceed with tag creation."
    exit 1
fi
print_info "Package validation successful ✓"

echo
print_release "📋 Tag Creation Summary:"
print_info "Version: $target_version"
print_info "Branch: $current_branch"
print_info "Tag to create: v$target_version"
print_info "Will trigger GitHub Actions: YES"
print_info "Will publish to crates.io via GitHub Actions: YES"
echo

# Final confirmation
read -p "Proceed with tag creation? This will trigger automatic publishing (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Tag creation cancelled"
    exit 0
fi

# Create git tag
print_tag "Creating git tag v$target_version..."
git tag -a "v$target_version" -m "Release version $target_version

Changelog:
- See CHANGELOG.md for detailed changes

This tag will trigger automated publishing to crates.io via GitHub Actions.
Created: $(date)"

print_info "Git tag v$target_version created ✓"

# Push tag to trigger GitHub Actions
print_tag "Pushing tag to origin to trigger GitHub Actions..."
if git push origin "v$target_version"; then
    print_info "Tag pushed to origin ✓"
    print_info "GitHub Actions workflow triggered ✓"
else
    print_error "Failed to push tag to origin!"
    print_warning "The git tag has been created locally, but not pushed."
    print_info "You can retry pushing with: git push origin v$target_version"
    exit 1
fi

echo
print_release "🎉 Tag Creation Complete!"
print_info "Version $target_version has been:"
print_info "✓ Tagged in git (v$target_version)"
print_info "✓ Pushed to origin"
print_info "✓ GitHub Actions workflow triggered"
echo

print_info "GitHub Actions will now automatically:"
print_info "1. Run comprehensive tests"
print_info "2. Verify version consistency"
print_info "3. Publish to crates.io"
print_info "4. Create GitHub release with release notes"
echo

print_info "Monitor the GitHub Actions workflow at:"
# Try to get the GitHub repository URL
github_url=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github\.com:/https:\/\/github.com\//')
if [[ $github_url =~ ^https://github\.com/ ]]; then
    actions_url="${github_url}/actions"
    print_info "$actions_url"
else
    print_info "Check your repository's Actions tab on GitHub"
fi
echo

print_warning "Remember: Once published to crates.io, this release is immutable!"
print_warning "If issues are found, you'll need to publish a new patch version."
