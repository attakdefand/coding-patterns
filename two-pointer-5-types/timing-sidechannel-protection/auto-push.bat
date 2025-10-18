@echo off
:: Auto-push script for timing-sidechannel-protection project

echo === Timing Side-Channel Protection Auto-Push Script ===

:: Check if there are changes to commit
git status --porcelain | findstr /R "." >nul
if %errorlevel% == 0 (
    echo Changes detected. Adding and committing...
    
    :: Add all changes
    git add .
    
    :: Commit with timestamp
    for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
    set "YY=%dt:~2,2%" & set "YYYY=%dt:~0,4%" & set "MM=%dt:~4,2%" & set "DD=%dt:~6,2%"
    set "HH=%dt:~8,2%" & set "Min=%dt:~10,2%" & set "Sec=%dt:~12,2%"
    set "timestamp=%YYYY%-%MM%-%DD% %HH%:%Min%:%Sec%"
    
    git commit -m "Auto-commit: %timestamp%"
    
    :: Push to remote repository
    echo Pushing to GitHub...
    git push origin main
    
    if %errorlevel% == 0 (
        echo Successfully pushed to GitHub!
    ) else (
        echo Failed to push to GitHub. Please check your connection and credentials.
    )
) else (
    echo No changes to commit.
)

echo === Auto-push completed ===