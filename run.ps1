$CONFIG_FILE = "config.toml"
$EXECUTABLE = "target\debug\axon.exe"

function Stop-With-Wait {
    param($Message, $Color = "Red")
    Write-Host "`n$Message" -ForegroundColor $Color
    Write-Host "Apasă orice tastă pentru a închide..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit
}

Clear-Host
Write-Host "==> [AXON] Inițiere procedură de lansare..." -ForegroundColor Cyan

if (-not (Test-Path $CONFIG_FILE)) { Stop-With-Wait " config.toml lipsește!" }

Write-Host "==> [BUILD] Compilare..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) { Stop-With-Wait " Eroare la compilare!" }

Write-Host "==> [RUN] Pornire Motor Axon..." -ForegroundColor Green
$env:RUST_BACKTRACE = 1
$env:RUST_LOG = "debug"

try {
    & $EXECUTABLE
} catch {
    Stop-With-Wait " Crash la execuție."
}

Stop-With-Wait "==> Proces terminat." "Cyan"
