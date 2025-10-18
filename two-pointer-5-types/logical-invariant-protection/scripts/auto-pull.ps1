# Auto Pull Script for PowerShell
# This script automatically pulls changes from the remote repository every 10 minutes

# Configuration
$Interval = 600  # 10 minutes in seconds
$Branch = "main"
$Remote = "origin"

# Get the directory of this script
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoPath = Join-Path $ScriptDir ".."

# Change to repository directory
Set-Location $RepoPath

function Write-Status {
    param([string]$Message)
    Write-Host "[$(Get-Date)] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[$(Get-Date)] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[$(Get-Date)] $Message" -ForegroundColor Red
}

function Test-GitRepository {
    try {
        git rev-parse --git-dir | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Invoke-PullChanges {
    Write-Status "Pulling latest changes from $Remote/$Branch..."
    
    # Fetch latest changes
    $fetchResult = git fetch $Remote $Branch 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to fetch from $Remote/$Branch"
        Write-Error $fetchResult
        return $false
    }
    
    # Check if we're behind
    $local = git rev-parse HEAD
    $remote = git rev-parse "$Remote/$Branch"
    
    if ($local -ne $remote) {
        # Try fast-forward merge
        $mergeResult = git merge "$Remote/$Branch" --ff-only 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Status "Successfully pulled latest changes"
            return $true
        } else {
            Write-Error "Failed to fast-forward merge. You may have local changes."
            Write-Error $mergeResult
            return $false
        }
    } else {
        Write-Status "Already up to date"
        return $true
    }
}

# Main execution
Write-Status "Auto Pull Script Started"
Write-Status "Repository: $RepoPath"
Write-Status "Remote: $Remote"
Write-Status "Branch: $Branch"
Write-Status "Interval: $Interval seconds ($($Interval / 60) minutes)"
Write-Status "Press Ctrl+C to stop"

# Check if we're in a git repository
if (-not (Test-GitRepository)) {
    Write-Error "Error: Not a git repository"
    exit 1
}

# Main loop
while ($true) {
    Write-Status "Checking for updates..."
    
    if (Invoke-PullChanges) {
        # Optionally run tests after successful pull
        # Add test execution here if needed
    }
    
    Write-Status "Sleeping for $($Interval / 60) minutes..."
    Start-Sleep -Seconds $Interval
}