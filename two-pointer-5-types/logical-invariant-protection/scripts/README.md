# Auto-Pull Scripts

This directory contains scripts for automatically pulling changes from the remote repository every 10 minutes.

## Available Scripts

1. **[auto-pull.sh](auto-pull.sh)** - Bash script for Linux/macOS
2. **[auto-pull.bat](auto-pull.bat)** - Batch script for Windows Command Prompt
3. **[auto-pull.ps1](auto-pull.ps1)** - PowerShell script for Windows PowerShell

## Usage

### Linux/macOS

Make the script executable and run it:

```bash
chmod +x auto-pull.sh
./auto-pull.sh
```

### Windows Command Prompt

```batch
auto-pull.bat
```

### Windows PowerShell

```powershell
powershell -ExecutionPolicy Bypass -File auto-pull.ps1
```

## Features

All scripts provide the following features:

- Automatically pulls changes from the remote repository every 10 minutes
- Checks for uncommitted changes before pulling
- Uses fast-forward merge to avoid conflicts
- Provides colored output for better visibility
- Handles errors gracefully
- Can be stopped with Ctrl+C

## Configuration

You can modify the following variables in each script:

- `INTERVAL` - Time between pulls in seconds (default: 600 seconds = 10 minutes)
- `BRANCH` - Branch to pull from (default: main)
- `REMOTE` - Remote repository name (default: origin)

## Running in Background

### Linux/macOS with tmux

```bash
# Start a new tmux session
tmux new-session -d -s autopull

# Run the auto-pull script in the session
tmux send-keys -t autopull './auto-pull.sh' Enter

# Attach to the session (optional)
tmux attach -t autopull
```

### Linux/macOS with screen

```bash
# Start a new screen session
screen -dmS autopull

# Run the auto-pull script in the session
screen -S autopull -X stuff "./auto-pull.sh$(printf \\r)"
```

### Windows with Task Scheduler

1. Open Task Scheduler
2. Create a new task
3. Set trigger to daily, repeating every 10 minutes
4. Set action to start a program:
   - Program: `powershell.exe`
   - Arguments: `-ExecutionPolicy Bypass -File "FULL_PATH_TO_SCRIPT\auto-pull.ps1"`

## Security Notes

- Ensure your Git credentials are properly configured
- Use SSH keys for secure authentication with GitHub
- Run scripts in a secure network environment
- Monitor script output for any unusual activity

## Troubleshooting

If you encounter issues:

1. **Permission Denied**: Make sure your Git credentials are properly configured
2. **Merge Conflicts**: The scripts use fast-forward only to avoid conflicts
3. **Network Issues**: Ensure stable network connectivity
4. **Path Issues**: Make sure you're running the script from the correct directory

For more information about auto-sync features, see the main [AUTO_SYNC.md](../AUTO_SYNC.md) documentation.