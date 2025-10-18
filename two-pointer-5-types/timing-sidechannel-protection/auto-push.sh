#!/bin/bash

# Auto-push script for timing-sidechannel-protection project

echo "=== Timing Side-Channel Protection Auto-Push Script ==="

# Check if there are changes to commit
if [[ -n $(git status --porcelain) ]]; then
    echo "Changes detected. Adding and committing..."
    
    # Add all changes
    git add .
    
    # Commit with timestamp
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    git commit -m "Auto-commit: $TIMESTAMP"
    
    # Push to remote repository
    echo "Pushing to GitHub..."
    git push origin main
    
    if [ $? -eq 0 ]; then
        echo "Successfully pushed to GitHub!"
    else
        echo "Failed to push to GitHub. Please check your connection and credentials."
    fi
else
    echo "No changes to commit."
fi

echo "=== Auto-push completed ==="