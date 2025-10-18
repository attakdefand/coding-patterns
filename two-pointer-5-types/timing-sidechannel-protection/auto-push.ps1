# Auto-push script for timing-sidechannel-protection project

Write-Host "=== Timing Side-Channel Protection Auto-Push Script ==="

# Check if there are changes to commit
$changes = git status --porcelain
if ($changes) {
    Write-Host "Changes detected. Adding and committing..."
    
    # Add all changes
    git add .
    
    # Commit with timestamp
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    git commit -m "Auto-commit: $timestamp"
    
    # Push to remote repository
    Write-Host "Pushing to GitHub..."
    git push origin main
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Successfully pushed to GitHub!" -ForegroundColor Green
    } else {
        Write-Host "Failed to push to GitHub. Please check your connection and credentials." -ForegroundColor Red
    }
} else {
    Write-Host "No changes to commit." -ForegroundColor Yellow
}

Write-Host "=== Auto-push completed ==="