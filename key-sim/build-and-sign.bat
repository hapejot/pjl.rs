@echo off
echo Building and signing key-sim...

echo.
echo Step 1: Building release version...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Failed to build. Exiting.
    pause
    exit /b 1
)

echo.
echo Step 2: Signing executable...
if exist "code-signing-cert.pfx" (
    set /p password="Enter certificate password: "
    signtool sign /f code-signing-cert.pfx /p %password% /t http://timestamp.digicert.com target\release\key-sim.exe
    if %ERRORLEVEL% equ 0 (
        echo.
        echo ✅ Successfully signed key-sim.exe
        echo.
        echo Verifying signature...
        signtool verify /pa target\release\key-sim.exe
    ) else (
        echo ❌ Failed to sign executable
    )
) else (
    echo Certificate file 'code-signing-cert.pfx' not found.
    echo Please create one first using sign-setup.ps1
)

echo.
pause