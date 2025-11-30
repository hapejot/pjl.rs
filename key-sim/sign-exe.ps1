#!/usr/bin/env pwsh
# Post-build script to sign the executable
# Usage: .\sign-exe.ps1 [certificate-file] [password]

param(
    [string]$CertFile = "code-signing-cert.pfx",
    [string]$Password = "",
    [string]$ExePath = "target\release\key-sim.exe"
)

# Check if signtool is available
$signtool = Get-Command "signtool.exe" -ErrorAction SilentlyContinue
if (-not $signtool) {
    Write-Error "signtool.exe not found. Please install Windows SDK."
    Write-Host "Download from: https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/"
    exit 1
}

# Check if executable exists
if (-not (Test-Path $ExePath)) {
    Write-Error "Executable not found: $ExePath"
    Write-Host "Run 'cargo build --release' first."
    exit 1
}

# Check if certificate exists
if (-not (Test-Path $CertFile)) {
    Write-Error "Certificate file not found: $CertFile"
    Write-Host "Create a certificate first using sign-setup.ps1"
    exit 1
}

# Sign the executable
Write-Host "Signing $ExePath with certificate $CertFile..."

if ($Password) {
    & signtool sign /f $CertFile /p $Password /t http://timestamp.digicert.com /v $ExePath
} else {
    & signtool sign /f $CertFile /t http://timestamp.digicert.com /v $ExePath
}

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Successfully signed $ExePath" -ForegroundColor Green
    
    # Verify the signature
    Write-Host "Verifying signature..."
    & signtool verify /pa /v $ExePath
} else {
    Write-Error "❌ Failed to sign executable"
    exit 1
}