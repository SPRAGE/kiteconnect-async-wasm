#!/usr/bin/env bash

# Script to create releases and publish to crates.io
# This script should ONLY be run on the main branch after merging development branches
# Usage: ./release.sh

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

print_publish() {
    echo -e "${CYAN}[PUBLISH]${NC} $1"
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
    print_error "Release script must be run on the main/master branch!"
    print_error "Current branch: $current_branch"
    print_error "Please merge your development branch to main first."
    exit 1
fi

print_info "Current branch: $current_branch ✓"

# Get current version from Cargo.toml
current_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "Current version: $current_version"

# Check if tag already exists
if git tag -l | grep -q "^v$current_version$"; then
    print_error "Tag v$current_version already exists!"
    print_error "This version has already been released."
    print_info "Available tags:"
    git tag -l | sort -V
    exit 1
fi

# Verify the version follows semantic versioning
if [[ ! $current_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format in Cargo.toml: $current_version"
    print_error "Expected semantic versioning format (e.g., 1.2.3)"
    exit 1
fi

print_release "🚀 Preparing to release version $current_version"
echo

# Run tests to ensure everything is working
print_info "Running comprehensive tests..."
if ! cargo test --all-features; then
    print_error "Tests failed! Cannot proceed with release."
    exit 1
fi
print_info "All tests passed ✓"

# Check if we can build successfully
print_info "Verifying build..."
if ! cargo build --release --all-features; then
    print_error "Build failed! Cannot proceed with release."
    exit 1
fi
print_info "Build successful ✓"

# Verify we can publish (dry run)
print_info "Verifying package can be published (dry run)..."
if ! cargo publish --dry-run; then
    print_error "Package validation failed! Cannot proceed with release."
    exit 1
fi
print_info "Package validation successful ✓"

echo
print_release "📋 Release Summary:"
print_info "Version: $current_version"
print_info "Branch: $current_branch"
print_info "Tag to create: v$current_version"
print_info "Will publish to crates.io: YES"
echo

# Final confirmation
read -p "Proceed with release? This will create a git tag and publish to crates.io (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Release cancelled"
    exit 0
fi

# Create git tag
print_release "Creating git tag v$current_version..."
git tag -a "v$current_version" -m "Release version $current_version

Changelog:
- See CHANGELOG.md for detailed changes

Published to crates.io: $(date)"

print_info "Git tag v$current_version created ✓"

# Push tag to trigger CI/CD if configured
read -p "Push tag to origin? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "Pushing tag to origin..."
    git push origin "v$current_version"
    print_info "Tag pushed to origin ✓"
fi

# Publish to crates.io
print_publish "Publishing to crates.io..."
if cargo publish; then
    print_publish "Successfully published v$current_version to crates.io! 🎉"
else
    print_error "Failed to publish to crates.io!"
    print_warning "The git tag has been created, but crates.io publication failed."
    print_info "You can retry publication with: cargo publish"
    exit 1
fi

echo
print_release "🎉 Release Complete!"
print_info "Version $current_version has been:"
print_info "✓ Tagged in git (v$current_version)"
print_info "✓ Published to crates.io"
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "✓ Pushed to origin"
fi
echo

print_info "Next steps:"
print_info "1. Update CHANGELOG.md with release notes (if not already done)"
print_info "2. Consider creating a GitHub release with release notes"
print_info "3. Announce the new version to users"
echo

print_warning "Remember: This release is now immutable on crates.io!"
print_warning "If issues are found, you'll need to publish a new patch version."
