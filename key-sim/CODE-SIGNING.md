# Code Signing for key-sim

This document explains how to sign your `key-sim.exe` to avoid Windows security warnings.

## Quick Start (Self-Signed Certificate)

1. **Create a self-signed certificate** (run as Administrator):
   ```powershell
   .\sign-setup.ps1
   ```

2. **Build and sign your executable**:
   ```powershell
   cargo build --release
   .\sign-exe.ps1
   ```

## Detailed Steps

### Option 1: Self-Signed Certificate (Free)

**Pros:** Free, works for personal use
**Cons:** Still shows "Unknown publisher" warning, requires manual trust

1. Run PowerShell as Administrator
2. Execute the certificate creation script:
   ```powershell
   .\sign-setup.ps1
   ```
3. Install the certificate in Trusted Root (to reduce warnings):
   ```powershell
   # Import to Trusted Root Certification Authorities
   Import-Certificate -FilePath "code-signing-cert.pfx" -CertStoreLocation "Cert:\LocalMachine\Root"
   ```

### Option 2: Commercial Certificate (Recommended)

**Pros:** No warnings, trusted by all Windows systems
**Cons:** Costs $100-600/year

1. Purchase a code signing certificate from:
   - [Sectigo](https://sectigo.com/ssl-certificates-tls/code-signing) (~$100-200/year)
   - [SSL.com](https://www.ssl.com/certificates/code-signing/) (~$100-200/year)
   - [DigiCert](https://www.digicert.com/code-signing/) (~$400-600/year)

2. Follow their verification process (usually requires business verification)

3. Download the certificate and use it with the signing script

### Option 3: EV Code Signing Certificate (Best)

**Pros:** No SmartScreen warnings, immediate trust
**Cons:** Most expensive ($300-800/year), requires hardware token

Extended Validation certificates provide the highest level of trust and skip Windows SmartScreen warnings entirely.

## Signing Process

### Manual Signing
```powershell
# Build the release version
cargo build --release

# Sign with your certificate
signtool sign /f your-certificate.pfx /p your-password /t http://timestamp.digicert.com target\release\key-sim.exe

# Verify the signature
signtool verify /pa target\release\key-sim.exe
```

### Automated Signing
Use the provided script:
```powershell
.\sign-exe.ps1 [certificate-file] [password]
```

## Prerequisites

### Install Windows SDK (for signtool)
Download and install from: https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/

The signtool.exe is typically installed to:
`C:\Program Files (x86)\Windows Kits\10\bin\{version}\x64\signtool.exe`

### Alternative: Install via Visual Studio
If you have Visual Studio installed, signtool is already available.

## Troubleshooting

### "signtool not found"
Add Windows SDK bin directory to your PATH:
```powershell
$env:PATH += ";C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64"
```

### Certificate Issues
- Ensure certificate is not expired
- Check that certificate includes "Code Signing" in Enhanced Key Usage
- Verify certificate is installed in correct store

### Still Getting Warnings
- Self-signed certificates will always show publisher warnings
- SmartScreen may still flag new executables (commercial certs help)
- EV certificates provide immediate trust without SmartScreen delays

## Best Practices

1. **Always timestamp your signatures** - prevents expiration issues
2. **Use strong passwords** for certificate files
3. **Store certificates securely** - never commit to version control
4. **Test signatures** on clean systems before distribution
5. **Consider CI/CD integration** for automatic signing

## CI/CD Integration

For automated builds, store your certificate securely:

### GitHub Actions Example
```yaml
- name: Sign executable
  run: |
    echo "${{ secrets.CODE_SIGNING_CERT }}" | base64 -d > cert.pfx
    signtool sign /f cert.pfx /p "${{ secrets.CERT_PASSWORD }}" /t http://timestamp.digicert.com target/release/key-sim.exe
  shell: pwsh
```

### Security Notes
- Never store certificates or passwords in plain text
- Use secure secret management in CI/CD
- Consider using Azure Key Vault or similar for certificate storage