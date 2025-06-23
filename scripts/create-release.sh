#!/bin/bash

# Script to create a release with changelog generation
# Usage: ./create-release.sh [version]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get version from Cargo.toml if not provided
if [ -z "$1" ]; then
    version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
else
    version="$1"
fi

print_info "Creating release for version $version"

# Check if tag already exists
if git tag | grep -q "^v$version$"; then
    print_error "Tag v$version already exists"
    exit 1
fi

# Generate changelog since last tag
last_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$last_tag" ]; then
    print_info "Generating changelog since $last_tag"
    changelog=$(git log "$last_tag..HEAD" --pretty=format:"- %s" --no-merges)
else
    print_info "Generating changelog since beginning"
    changelog=$(git log --pretty=format:"- %s" --no-merges)
fi

# Create/update CHANGELOG.md
if [ ! -f CHANGELOG.md ]; then
    print_info "Creating CHANGELOG.md"
    cat > CHANGELOG.md << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$version] - $(date +%Y-%m-%d)

$changelog

EOF
else
    print_info "Updating CHANGELOG.md"
    # Insert new version after [Unreleased] section
    temp_file=$(mktemp)
    awk -v version="$version" -v date="$(date +%Y-%m-%d)" -v changelog="$changelog" '
        /^## \[Unreleased\]/ {
            print $0
            print ""
            print "## [" version "] - " date
            print ""
            print changelog
            print ""
            next
        }
        { print }
    ' CHANGELOG.md > "$temp_file"
    mv "$temp_file" CHANGELOG.md
fi

# Show the generated changelog
print_info "Generated changelog:"
echo "----------------------------------------"
echo "$changelog"
echo "----------------------------------------"

# Commit changelog if it was modified
if ! git diff --quiet CHANGELOG.md; then
    read -p "Commit changelog changes? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git add CHANGELOG.md
        git commit -m "Update changelog for v$version"
    fi
fi

# Create version branch if it doesn't exist
if ! git show-ref --verify --quiet "refs/heads/v$version"; then
    read -p "Create version branch v$version? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Store current branch
        current_branch=$(git branch --show-current)
        
        # Create the version branch from current state
        git checkout -b "v$version"
        print_info "Created version branch v$version"
        
        # Switch back to original branch
        git checkout "$current_branch"
        print_info "Switched back to $current_branch"
    fi
else
    print_info "Version branch v$version already exists"
fi

# Create and push tag
print_info "Creating tag v$version"
git tag -a "v$version" -m "Release version $version"

read -p "Push tag to trigger release? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin "v$version"
    print_info "Tag pushed. Release workflow should start automatically."
    print_info "Check GitHub Actions: https://github.com/SPRAGE/kiteconnect-async-wasm/actions"
    
    # Also offer to push the version branch
    if git show-ref --verify --quiet "refs/heads/v$version"; then
        read -p "Also push version branch v$version? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git push origin "v$version"
            print_info "Version branch v$version pushed."
        else
            print_info "Version branch not pushed. Push manually with:"
            print_info "git push origin v$version"
        fi
    fi
else
    print_info "Tag created locally. Push manually with:"
    print_info "git push origin v$version"
    if git show-ref --verify --quiet "refs/heads/v$version"; then
        print_info "Version branch created locally. Push manually with:"
        print_info "git push origin v$version"
    fi
fi

print_info "Release creation complete!"
