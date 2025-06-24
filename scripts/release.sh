#!/usr/bin/env bash

# Script to create development branches and push to GitHub for PR creation
# This script creates a development branch, runs tests, and pushes to origin
# Usage: ./release.sh [version] or just ./release.sh to use Cargo.toml version

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
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

print_branch() {
    echo -e "${CYAN}[BRANCH]${NC} $1"
}

print_pr() {
    echo -e "${PURPLE}[PR]${NC} $1"
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

# Accept version as parameter or get from Cargo.toml
if [ $# -eq 1 ]; then
    target_version="$1"
else
    target_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
fi

print_info "Target version: $target_version"

# Verify the version follows semantic versioning
if [[ ! $target_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format: $target_version"
    print_error "Expected semantic versioning format (e.g., 1.2.3)"
    exit 1
fi

# Create development branch name
dev_branch="v${target_version}-dev"

# Check if development branch already exists
if git show-ref --verify --quiet refs/heads/"$dev_branch"; then
    print_warning "Development branch '$dev_branch' already exists locally"
    read -p "Do you want to switch to it and continue? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git checkout "$dev_branch"
        current_branch="$dev_branch"
    else
        print_info "Release preparation cancelled"
        exit 0
    fi
else
    # Create new development branch
    print_branch "Creating development branch: $dev_branch"
    git checkout -b "$dev_branch"
    current_branch="$dev_branch"
fi

print_info "Current branch: $current_branch ✓"

print_release "🚀 Preparing release development branch for version $target_version"
echo

# Update version in Cargo.toml if it doesn't match
current_cargo_version=$(grep "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$current_cargo_version" != "$target_version" ]; then
    print_info "Updating Cargo.toml version from $current_cargo_version to $target_version"
    sed -i "s/^version = \".*\"/version = \"$target_version\"/" Cargo.toml
    
    # Update Cargo.lock
    print_info "Updating Cargo.lock..."
    cargo check
    
    # Commit version change
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to $target_version

Prepare for release $target_version
    
- Updated version in Cargo.toml
- Updated Cargo.lock
- Ready for release testing and review"
fi

# Run tests to ensure everything is working
print_info "Running comprehensive tests..."
if ! cargo test --all-features; then
    print_error "Tests failed! Cannot proceed with release preparation."
    print_error "Please fix the failing tests before creating the release."
    exit 1
fi
print_info "All tests passed ✓"

# Check if we can build successfully
print_info "Verifying build..."
if ! cargo build --release --all-features; then
    print_error "Build failed! Cannot proceed with release preparation."
    exit 1
fi
print_info "Build successful ✓"

# Verify we can publish (dry run)
print_info "Verifying package can be published (dry run)..."
if ! cargo publish --dry-run; then
    print_error "Package validation failed! Please fix issues before release."
    exit 1
fi
print_info "Package validation successful ✓"

echo
print_release "📋 Release Preparation Summary:"
print_info "Development Branch: $dev_branch"
print_info "Target Version: $target_version"
print_info "All tests: PASSED ✓"
print_info "Build verification: PASSED ✓"
print_info "Package validation: PASSED ✓"
echo

# Check if remote branch exists
if git ls-remote --heads origin "$dev_branch" | grep -q "$dev_branch"; then
    print_warning "Remote branch 'origin/$dev_branch' already exists"
    read -p "Do you want to force push to update it? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        push_args="--force"
    else
        print_info "Skipping push to avoid overwriting remote branch"
        print_info "You can manually push later with: git push origin $dev_branch"
        exit 0
    fi
else
    push_args=""
fi

# Push development branch to GitHub
print_branch "Pushing development branch to GitHub..."
if git push $push_args -u origin "$dev_branch"; then
    print_info "Development branch pushed to GitHub ✓"
else
    print_error "Failed to push development branch to GitHub"
    exit 1
fi

echo
print_pr "🔄 Next Steps - Create Pull Request:"
print_info "1. Go to GitHub and create a Pull Request:"
print_info "   From: $dev_branch"
print_info "   To: main (or master)"
echo
print_info "2. PR Title suggestion:"
print_info "   'Release v$target_version'"
echo
print_info "3. PR Description template:"
echo "---"
echo "## Release v$target_version"
echo ""
echo "### Changes"
echo "- [ ] All tests passing"
echo "- [ ] Version bumped to $target_version"
echo "- [ ] CHANGELOG.md updated (if applicable)"
echo "- [ ] Documentation updated (if needed)"
echo ""
echo "### Release Checklist"
echo "- [x] Tests pass locally"
echo "- [x] Build succeeds"
echo "- [x] Package validation passes"
echo "- [ ] Code review completed"
echo "- [ ] Ready for merge to main"
echo ""
echo "### Post-Merge"
echo "After merging this PR to main:"
echo "1. GitHub Actions will automatically create a git tag v$target_version"
echo "2. GitHub Actions will automatically publish to crates.io"
echo "3. GitHub Actions will automatically create a GitHub release"
echo "---"
echo
print_info "4. After creating the PR, reviewers should:"
print_info "   - Review all code changes"
print_info "   - Verify tests pass in CI"
print_info "   - Confirm version number is correct"
print_info "   - Check CHANGELOG.md is updated"
echo
print_info "5. After PR approval and merge to main:"
print_info "   - GitHub Actions will automatically handle the release"
print_info "   - No manual intervention needed for publishing"
echo

# Offer to open GitHub in browser (if available)
if command -v xdg-open >/dev/null 2>&1 || command -v open >/dev/null 2>&1; then
    read -p "Open GitHub to create PR? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Try to get the GitHub repository URL
        github_url=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github\.com:/https:\/\/github.com\//')
        if [[ $github_url =~ ^https://github\.com/ ]]; then
            pr_url="${github_url}/compare/main...${dev_branch}"
            if command -v xdg-open >/dev/null 2>&1; then
                xdg-open "$pr_url"
            elif command -v open >/dev/null 2>&1; then
                open "$pr_url"
            fi
            print_info "Opening GitHub PR creation page..."
        else
            print_warning "Could not determine GitHub URL"
        fi
    fi
fi

echo
print_release "🎉 Release Preparation Complete!"
print_info "Development branch '$dev_branch' is ready for PR creation"
print_info "Once the PR is merged to main, GitHub Actions will handle the rest!"
