#!/bin/bash
# Auto Pull Script
# This script automatically pulls changes from the remote repository every 10 minutes

# Configuration
REPO_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INTERVAL=600  # 10 minutes in seconds
BRANCH="main"
REMOTE="origin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[$(date)]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[$(date)]${NC} $1"
}

print_error() {
    echo -e "${RED}[$(date)]${NC} $1"
}

# Function to check if we're in a git repository
check_git_repo() {
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Error: Not a git repository"
        exit 1
    fi
}

# Function to check git status
check_git_status() {
    local status=$(git status --porcelain)
    if [ -n "$status" ]; then
        print_warning "Warning: You have uncommitted changes"
        echo "$status"
    fi
}

# Function to pull changes
pull_changes() {
    print_status "Pulling latest changes from $REMOTE/$BRANCH..."
    
    # Fetch latest changes
    if ! git fetch $REMOTE $BRANCH; then
        print_error "Failed to fetch from $REMOTE/$BRANCH"
        return 1
    fi
    
    # Check if we're behind
    LOCAL=$(git rev-parse HEAD)
    REMOTE_COMMIT=$(git rev-parse $REMOTE/$BRANCH)
    
    if [ "$LOCAL" != "$REMOTE_COMMIT" ]; then
        # Try fast-forward merge
        if git merge $REMOTE/$BRANCH --ff-only; then
            print_status "Successfully pulled latest changes"
        else
            print_error "Failed to fast-forward merge. You may have local changes."
            return 1
        fi
    else
        print_status "Already up to date"
    fi
    
    return 0
}

# Function to run tests (optional)
run_tests() {
    print_status "Running tests..."
    if command -v cargo &> /dev/null; then
        if cargo test; then
            print_status "All tests passed"
        else
            print_error "Some tests failed"
            return 1
        fi
    else
        print_warning "Cargo not found, skipping tests"
    fi
}

# Main function
main() {
    # Change to repository directory
    cd "$REPO_PATH"
    
    # Check if we're in a git repository
    check_git_repo
    
    print_status "Auto Pull Script Started"
    print_status "Repository: $REPO_PATH"
    print_status "Remote: $REMOTE"
    print_status "Branch: $BRANCH"
    print_status "Interval: $INTERVAL seconds ($(($INTERVAL / 60)) minutes)"
    print_status "Press Ctrl+C to stop"
    
    # Initial check
    check_git_status
    
    # Main loop
    while true; do
        print_status "Checking for updates..."
        
        if pull_changes; then
            # Optionally run tests after successful pull
            # run_tests
            true
        fi
        
        print_status "Sleeping for $((INTERVAL / 60)) minutes..."
        sleep $INTERVAL
    done
}

# Handle Ctrl+C
trap 'print_status "Stopping Auto Pull Script..."; exit 0' INT

# Run main function
main