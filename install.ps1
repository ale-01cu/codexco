$ErrorActionPreference = 'Stop'

Write-Host "Installing codexco..." -ForegroundColor Cyan

$repo = "ale-01cu/codexco"
$installDir = "$HOME\.codexco\bin"
$binName = "codexco.exe"
$targetExe = Join-Path $installDir $binName
$archiveName = "codexco-x86_64-pc-windows-msvc.zip"

# Obtener última release
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing
    $asset = $release.assets | Where-Object { $_.name -eq $archiveName }
    if (-not $asset) {
        throw "No asset found matching $archiveName in latest release."
    }
    $downloadUrl = $asset.browser_download_url
} catch {
    Write-Host "Warning: Could not fetch latest release asset via API ($_.Exception.Message). Falling back to direct URL..." -ForegroundColor Yellow
    $downloadUrl = "https://github.com/$repo/releases/latest/download/$archiveName"
}

# Preparar directorio de instalación
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

$tempZip = Join-Path ([System.IO.Path]::GetTempPath()) "codexco-install.zip"
Write-Host "Downloading $downloadUrl..." -ForegroundColor Gray
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing

Write-Host "Extracting to $installDir..." -ForegroundColor Gray
Expand-Archive -Path $tempZip -DestinationPath $installDir -Force
Remove-Item -Path $tempZip -Force

# Asegurar que esté en el PATH del usuario
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($userPath -split ';' -notcontains $installDir) {
    Write-Host "Adding $installDir to User PATH..." -ForegroundColor Green
    [Environment]::SetEnvironmentVariable('Path', "$userPath;$installDir", 'User')
    $env:Path = "$env:Path;$installDir"
}

Write-Host "`n✓ codexco successfully installed to $targetExe!" -ForegroundColor Green
Write-Host "Restart your terminal or run 'codexco' directly.`n" -ForegroundColor Cyan
