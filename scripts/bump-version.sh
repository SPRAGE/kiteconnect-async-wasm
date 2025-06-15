#!/usr/bin/env bash

# Script to bump version numbers in Cargo.toml
# Usage: ./bump-version.sh [patch|minor|major|VERSION]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Get current version from Cargo.toml
current_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "Current version: $current_version"

# Parse version components
IFS='.' read -r major minor patch <<< "$current_version"

# Determine new version
case "$1" in
    "patch")
        new_patch=$((patch + 1))
        new_version="$major.$minor.$new_patch"
        ;;
    "minor")
        new_minor=$((minor + 1))
        new_version="$major.$new_minor.0"
        ;;
    "major")
        new_major=$((major + 1))
        new_version="$new_major.0.0"
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
        else
            print_error "Invalid version format. Use semantic versioning (e.g., 1.2.3)"
            exit 1
        fi
        ;;
esac

print_info "New version: $new_version"

# Confirm the change
read -p "Do you want to bump version from $current_version to $new_version? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Version bump cancelled"
    exit 0
fi

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
    exit 1
fi

# Commit the changes
print_info "Committing changes..."
git add Cargo.toml Cargo.lock
if git diff --staged --quiet README.md 2>/dev/null; then
    git add README.md
fi

git commit -m "Bump version to $new_version"

# Create tag
read -p "Create git tag v$new_version? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git tag "v$new_version"
    print_info "Created tag v$new_version"
    
    print_info "To trigger release, push the tag:"
    print_info "git push origin v$new_version"
else
    print_info "Tag not created. You can create it manually later with:"
    print_info "git tag v$new_version"
fi

print_info "Version bump complete!"
print_info "Don't forget to update CHANGELOG.md and push your changes"
