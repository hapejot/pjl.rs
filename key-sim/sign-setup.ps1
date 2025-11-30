# Self-signed certificate setup for code signing
# Run this script as Administrator

# Create a self-signed certificate
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=YourName Code Signing" -KeyUsage DigitalSignature -FriendlyName "Code Signing Certificate" -CertStoreLocation "Cert:\CurrentUser\My" -KeyLength 2048

# Export the certificate to install it in Trusted Root (requires admin)
$password = ConvertTo-SecureString -String "your_password_here" -Force -AsPlainText
Export-PfxCertificate -Cert $cert -FilePath ".\code-signing-cert.pfx" -Password $password

Write-Host "Certificate created. Thumbprint: $($cert.Thumbprint)"
Write-Host "To sign your executable, use:"
Write-Host "signtool sign /f code-signing-cert.pfx /p your_password_here /t http://timestamp.digicert.com target\release\key-sim.exe"