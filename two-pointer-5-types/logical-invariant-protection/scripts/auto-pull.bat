@echo off
REM Auto Pull Script for Windows
REM This script automatically pulls changes from the remote repository every 10 minutes

REM Configuration
set INTERVAL=600
set BRANCH=main
set REMOTE=origin

REM Get the directory of this script
set SCRIPT_DIR=%~dp0
set REPO_PATH=%SCRIPT_DIR%..

REM Change to repository directory
cd /d "%REPO_PATH%"

echo Auto Pull Script Started
echo Repository: %REPO_PATH%
echo Remote: %REMOTE%
echo Branch: %BRANCH%
echo Interval: %INTERVAL% seconds (%INTERVAL% / 60 minutes)
echo Press Ctrl+C to stop

:loop
echo.
echo [%date% %time%] Checking for updates...

REM Check if we're in a git repository
git rev-parse --git-dir >nul 2>&1
if errorlevel 1 (
    echo [%date% %time%] Error: Not a git repository
    goto wait
)

REM Check for uncommitted changes
git status --porcelain | findstr /R /C:"^.." >nul
if not errorlevel 1 (
    echo [%date% %time%] Warning: You have uncommitted changes
)

REM Fetch latest changes
echo [%date% %time%] Fetching latest changes from %REMOTE%/%BRANCH%...
git fetch %REMOTE% %BRANCH%
if errorlevel 1 (
    echo [%date% %time%] Failed to fetch from %REMOTE%/%BRANCH%
    goto wait
)

REM Check if we're behind
for /f %%i in ('git rev-parse HEAD') do set LOCAL=%%i
for /f %%i in ('git rev-parse %REMOTE%/%BRANCH%') do set REMOTE_COMMIT=%%i

if not "%LOCAL%"=="%REMOTE_COMMIT%" (
    echo [%date% %time%] Pulling latest changes...
    git merge %REMOTE%/%BRANCH% --ff-only
    if errorlevel 1 (
        echo [%date% %time%] Failed to fast-forward merge. You may have local changes.
    ) else (
        echo [%date% %time%] Successfully pulled latest changes
    )
) else (
    echo [%date% %time%] Already up to date
)

:wait
echo [%date% %time%] Sleeping for %INTERVAL% seconds...
timeout /t %INTERVAL% /nobreak >nul
goto loop