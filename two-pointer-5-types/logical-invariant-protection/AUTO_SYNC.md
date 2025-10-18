# Auto-Sync and Auto-Pull Configuration

This document explains how to set up automated synchronization between your local repository and the remote GitHub repository.

## GitHub Actions Workflows

We have several GitHub Actions workflows that provide automated synchronization:

### Auto Pull ([.github/workflows/auto-pull.yml](.github/workflows/auto-pull.yml))

This workflow automatically pulls changes from the main branch every 10 minutes.

**Schedule**: Every 10 minutes (`*/10 * * * *`)

**Features**:
- Fetches latest changes from the remote repository
- Performs fast-forward merge when possible
- Commits and pushes any local changes if needed

### Auto Sync ([.github/workflows/auto-sync.yml](.github/workflows/auto-sync.yml))

This workflow provides more comprehensive synchronization capabilities.

**Schedule**: Every 10 minutes (`*/10 * * * *`)

**Features**:
- Fetches all branches and tags
- Synchronizes with the remote main branch
- Runs tests when updates are detected
- Creates issues on test failures

### Auto Merge ([.github/workflows/auto-merge.yml](.github/workflows/auto-merge.yml))

This workflow automatically merges pull requests under certain conditions.

**Triggers**:
- When pull requests are opened, synchronized, or reopened
- Every 10 minutes (scheduled check)
- Manual trigger

**Features**:
- Automatically approves pull requests from trusted authors (Dependabot, GitHub Actions)
- Merges pull requests with "automerge" label when all checks pass

## Local Auto-Pull Setup

If you want to set up auto-pull functionality on your local machine, here are several options:

We've included pre-built scripts in the [scripts](scripts/) directory:
- [auto-pull.sh](scripts/auto-pull.sh) - Bash script for Linux/macOS
- [auto-pull.bat](scripts/auto-pull.bat) - Batch script for Windows
- [auto-pull.ps1](scripts/auto-pull.ps1) - PowerShell script for Windows

### Option 1: Using Pre-built Scripts

We've provided pre-built scripts for different platforms in the [scripts](scripts/) directory:

**For Linux/macOS:**
```bash
# Make the script executable
chmod +x scripts/auto-pull.sh

# Run the script
./scripts/auto-pull.sh
```

**For Windows (Command Prompt):**
```batch
scripts\auto-pull.bat
```

**For Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy Bypass -File scripts\auto-pull.ps1
```

### Option 2: Using a Simple Shell Script

Create a shell script that pulls changes periodically:

```bash
#!/bin/bash
# auto-pull.sh

REPO_PATH="/path/to/your/repository"
INTERVAL=600  # 10 minutes in seconds

cd $REPO_PATH

while true; do
    echo "Pulling latest changes at $(date)"
    git pull origin main
    echo "Sleeping for $INTERVAL seconds"
    sleep $INTERVAL
done
```

Make it executable and run it:
```bash
chmod +x auto-pull.sh
./auto-pull.sh
```

### Option 2: Using cron (Linux/macOS)

Add a cron job to automatically pull changes:

```bash
# Edit your crontab
crontab -e

# Add this line to pull every 10 minutes
*/10 * * * * cd /path/to/your/repository && git pull origin main
```

### Option 3: Using Windows Task Scheduler

1. Create a batch file `auto-pull.bat`:
```batch
@echo off
cd /d "D:\path\to\your\repository"
git pull origin main
```

2. Create a PowerShell script `auto-pull.ps1`:
```powershell
Set-Location "D:\path\to\your\repository"
git pull origin main
```

3. Set up a scheduled task in Task Scheduler:
   - Trigger: Daily, repeating every 10 minutes
   - Action: Start a program -> powershell.exe
   - Arguments: -ExecutionPolicy Bypass -File "D:\path\to\your\auto-pull.ps1"

### Option 4: Using Git Hooks

You can set up a post-merge hook that schedules the next pull:

1. Create `.git/hooks/post-merge`:
```bash
#!/bin/bash
# Schedule next pull in 10 minutes
echo "Scheduling next pull in 10 minutes"
sleep 600 && git pull origin main &
```

2. Make it executable:
```bash
chmod +x .git/hooks/post-merge
```

## Using tmux or screen for Persistent Sessions

To keep auto-pull running even when you close your terminal:

### Using tmux:
```bash
# Start a new tmux session
tmux new-session -d -s autopull

# Run the auto-pull script in the session
tmux send-keys -t autopull 'cd /path/to/your/repository && ./auto-pull.sh' Enter

# Attach to the session (optional)
tmux attach -t autopull
```

### Using screen:
```bash
# Start a new screen session
screen -dmS autopull

# Run the auto-pull script in the session
screen -S autopull -X stuff "cd /path/to/your/repository && ./auto-pull.sh$(printf \\r)"
```

## Configuration Options

### Customizing Pull Intervals

To change the pull interval, modify the cron schedule in the GitHub Actions workflows:

```yaml
# Every 5 minutes
- cron: '*/5 * * * *'

# Every 30 minutes
- cron: '*/30 * * * *'

# Every hour
- cron: '0 * * * *'
```

### Handling Merge Conflicts

The workflows are configured to use fast-forward merges only, which means they will not attempt to resolve merge conflicts. If a fast-forward is not possible, the workflow will log a message and continue.

For local setups, you might want to handle conflicts differently:

```bash
#!/bin/bash
# auto-pull-with-conflict-handling.sh

REPO_PATH="/path/to/your/repository"
INTERVAL=600

cd $REPO_PATH

while true; do
    echo "Pulling latest changes at $(date)"
    
    # Try to pull with rebase
    if ! git pull --rebase origin main; then
        echo "Merge conflicts detected, attempting to resolve..."
        # You can add conflict resolution logic here
        # For now, we'll just abort the rebase
        git rebase --abort
    fi
    
    echo "Sleeping for $INTERVAL seconds"
    sleep $INTERVAL
done
```

## Monitoring and Notifications

### GitHub Actions Notifications

All GitHub Actions workflows will send notifications through GitHub's built-in notification system. You can customize your notification settings in GitHub:

1. Go to GitHub Settings
2. Navigate to Notifications
3. Configure how you want to receive notifications for workflow runs

### Local Monitoring

For local auto-pull setups, you can add logging and notification capabilities:

```bash
#!/bin/bash
# auto-pull-with-logging.sh

REPO_PATH="/path/to/your/repository"
LOG_FILE="/path/to/autopull.log"
INTERVAL=600

cd $REPO_PATH

while true; do
    echo "$(date): Pulling latest changes" >> $LOG_FILE
    
    if git pull origin main; then
        echo "$(date): Pull successful" >> $LOG_FILE
        # Optionally send a success notification
    else
        echo "$(date): Pull failed" >> $LOG_FILE
        # Optionally send a failure notification
    fi
    
    echo "$(date): Sleeping for $INTERVAL seconds" >> $LOG_FILE
    sleep $INTERVAL
done
```

## Security Considerations

1. **Authentication**: Ensure your local Git is properly configured with credentials
2. **SSH Keys**: Use SSH keys for secure authentication with GitHub
3. **Permissions**: Make sure the auto-pull script has appropriate permissions
4. **Network Security**: Run auto-pull in a secure network environment

## Troubleshooting

### Common Issues

1. **Permission Denied**: Make sure your Git credentials are properly configured
2. **Merge Conflicts**: The workflows use fast-forward only to avoid conflicts
3. **Network Issues**: Ensure stable network connectivity
4. **Disk Space**: Monitor disk space as the repository grows

### Debugging

To debug auto-pull issues, check:

1. GitHub Actions logs in the repository's Actions tab
2. Local log files if you've set up logging
3. Git configuration: `git config --list`
4. Remote configuration: `git remote -v`

These auto-sync mechanisms ensure that your repository stays up-to-date with the latest changes from the remote repository, providing continuous synchronization without manual intervention.