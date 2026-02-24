# 1. Rulează build-ul și colectează erorile
Write-Host "Analizând erorile Axon..." -ForegroundColor Cyan
$buildOutput = cargo check 2>&1 | Out-String

# 2. Trimite logul către OpenClaw pentru reparații automate
if ($buildOutput -match "error") {
    Write-Host "Erori găsite. Solicit reparații..." -ForegroundColor Yellow
    openclaw agent --message "Rezolvă aceste erori de compilare pentru proiectul Axon. Concentrează-te pe alinierea structurilor din schema.rs cu utilizarea lor în patch_tree.rs și self_reflection.rs: $buildOutput"
} else {
    Write-Host "Nicio eroare detectată!" -ForegroundColor Green
}