# BetterCraft: build the Windows app, install it for the current user and open it.
# Installs to %LOCALAPPDATA%\Programs\BetterCraft Launcher with a Start menu shortcut.
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path -Parent $PSScriptRoot)

$cargo = if (Get-Command cargo -ErrorAction SilentlyContinue) { 'cargo' } else { Join-Path $HOME '.cargo\bin\cargo.exe' }
& $cargo build --release -p artcraft-launcher
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$exe = Join-Path (Get-Location) 'target\release\artcraft-launcher.exe'
$dest = Join-Path $env:LOCALAPPDATA 'Programs\BetterCraft Launcher'
New-Item -ItemType Directory -Force -Path $dest | Out-Null
Copy-Item -LiteralPath $exe -Destination (Join-Path $dest 'artcraft-launcher.exe') -Force

# Start menu shortcut, in the user's own Start menu (no admin needed).
$ws = New-Object -ComObject WScript.Shell
$menu = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'
$lnk = $ws.CreateShortcut((Join-Path $menu 'BetterCraft Launcher.lnk'))
$lnk.TargetPath = Join-Path $dest 'artcraft-launcher.exe'
$lnk.WorkingDirectory = $dest
$lnk.IconLocation = (Join-Path $dest 'artcraft-launcher.exe') + ',0'
$lnk.Save()

Write-Output "Installed $($dest)\artcraft-launcher.exe"
Start-Process (Join-Path $dest 'artcraft-launcher.exe')
