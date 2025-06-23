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

# Variable to track if this is a major version bump
is_major_bump=false

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
            # Check if this is a major version bump
            new_major=$(echo "$1" | cut -d'.' -f1)
            if [[ $new_major -gt $major ]]; then
                is_major_bump=true
            fi
        else
            print_error "Invalid version format. Use semantic versioning (e.g., 1.2.3)"
            exit 1
        fi
        ;;
esac

print_info "New version: $new_version"

# Special handling for major version bumps
if [[ $is_major_bump == true ]]; then
    print_major "⚠️  MAJOR VERSION BUMP DETECTED ⚠️"
    print_major "This will create a new development branch that won't be merged to main until you explicitly approve it."
    echo
fi

# Confirm the change
read -p "Do you want to bump version from $current_version to $new_version? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Version bump cancelled"
    exit 0
fi

# For major version bumps, create a new development branch first
if [[ $is_major_bump == true ]]; then
    major_branch="v$new_version-dev"
    
    # Check if branch already exists
    if git show-ref --verify --quiet "refs/heads/$major_branch"; then
        print_error "Branch $major_branch already exists. Please handle it manually or delete it first."
        exit 1
    fi
    
    print_major "Creating development branch: $major_branch"
    git checkout -b "$major_branch"
    print_branch "Switched to new development branch: $major_branch"
    print_warning "📋 Remember: This branch will NOT be merged to main automatically!"
    echo
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

if [[ $is_major_bump == true ]]; then
    git commit -m "🚀 Major version bump to $new_version

This is a breaking change that introduces new APIs or removes/changes existing ones.
Development branch: $(git branch --show-current)
    
⚠️  DO NOT MERGE TO MAIN WITHOUT EXPLICIT APPROVAL ⚠️"
else
    git commit -m "Bump version to $new_version"
fi

# Handle branching and tagging differently for major vs minor/patch versions
if [[ $is_major_bump == true ]]; then
    # Major version: We're already on the dev branch, just create tag
    print_major "🎯 Major Version Workflow Complete!"
    print_branch "You are now on development branch: $(git branch --show-current)"
    echo
    print_warning "📋 Next Steps for Major Version:"
    print_info "1. Develop your breaking changes on this branch"
    print_info "2. Test thoroughly"
    print_info "3. Update documentation and CHANGELOG.md"
    print_info "4. When ready, create a pull request to merge into main"
    print_info "5. Only then create the release tag"
    echo
    print_info "To push this development branch:"
    print_info "git push -u origin $(git branch --show-current)"
    echo
    print_warning "🚨 IMPORTANT: This branch will NOT be merged to main automatically!"
    print_warning "   You must manually approve the merge when the breaking changes are ready."
else
    # Minor/Patch version: Original workflow with version branches
    # Create version branch
    read -p "Create version branch v$new_version? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Store current branch (handle both current branch and detached HEAD)
        current_branch_for_return=$(git branch --show-current 2>/dev/null || git rev-parse --short HEAD)
        
        # Create the version branch from current state
        git checkout -b "v$new_version"
        print_info "Created version branch v$new_version"
        
        # Switch back to original branch (handle both branch names and commit hashes)
        if git show-ref --verify --quiet "refs/heads/$current_branch_for_return"; then
            git checkout "$current_branch_for_return"
            print_info "Switched back to $current_branch_for_return"
        else
            print_warning "Could not switch back to $current_branch_for_return (might be detached HEAD)"
            print_info "You are now on version branch v$new_version"
        fi
    else
        print_info "Version branch not created. You can create it manually later with:"
        print_info "git checkout -b v$new_version"
    fi

    # Create tag
    read -p "Create git tag v$new_version? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag "v$new_version"
        print_info "Created tag v$new_version"
        
        print_info "To trigger release, push the tag:"
        print_info "git push origin v$new_version"
        
        if git show-ref --verify --quiet "refs/heads/v$new_version"; then
            print_info "To push the version branch:"
            print_info "git push origin v$new_version"
        fi
    else
        print_info "Tag not created. You can create it manually later with:"
        print_info "git tag v$new_version"
    fi
    
    print_info "Version bump complete!"
    print_info "Don't forget to update CHANGELOG.md and push your changes"
    if git show-ref --verify --quiet "refs/heads/v$new_version"; then
        print_info "Version branch v$new_version created and ready to push"
    fi
fi
