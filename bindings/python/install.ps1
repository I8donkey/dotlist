# .list Python Binding Installer (PowerShell)
# Run: powershell -ExecutionPolicy Bypass -File install.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  .list Python Binding Installer" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check Python
Write-Host "[1/4] Checking Python..." -ForegroundColor Yellow
try {
    $pythonVersion = python --version 2>&1
    Write-Host "[OK] $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] Python not found. Please install Python 3.8+" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Check pip
Write-Host "`n[2/4] Checking pip..." -ForegroundColor Yellow
try {
    pip --version | Out-Null
    Write-Host "[OK] pip detected" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] pip not found" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Install maturin
Write-Host "`n[3/4] Installing Maturin..." -ForegroundColor Yellow
pip install maturin 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) {
    Write-Host "[OK] Maturin installed successfully" -ForegroundColor Green
} else {
    Write-Host "[INFO] Maturin may already installed, continuing..." -ForegroundColor DarkGray
}

# Build and install
Write-Host "`n[4/4] Compiling and installing..." -ForegroundColor Yellow
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Join-Path $scriptPath "..\.."

Push-Location $projectPath
maturin build --release --features python -o python_bindings 2>&1 | Tee-Object -Variable buildOutput
Pop-Location

if ($LASTEXITCODE -eq 0) {
    # Find the wheel file
    $wheelFile = Get-ChildItem -Path "python_bindings\list_lang-*.whl" | Select-Object -First 1
    
    if ($wheelFile) {
        # Install the wheel
        Write-Host "`nInstalling wheel package..." -ForegroundColor Yellow
        pip install $wheelFile.FullName --force-reinstall 2>&1 | Out-Null
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Host "========================================" -ForegroundColor Green
            Write-Host "  Installation Complete!" -ForegroundColor Green
            Write-Host "========================================" -ForegroundColor Green
            Write-Host ""
            Write-Host "You can now use list_lang in Python!" -ForegroundColor White
            Write-Host ""
            Write-Host "Quick test:" -ForegroundColor Cyan
            Write-Host '  python -c "from list_lang import PyListData; print(''OK'')"' -ForegroundColor Gray
            Write-Host ""
            Write-Host "Run examples:" -ForegroundColor Cyan
            Write-Host "  python bindings\python\quick_start.py" -ForegroundColor Gray
            Write-Host "  python bindings\python\test_bindings.py" -ForegroundColor Gray
            Write-Host ""
        } else {
            Write-Host "[WARN] Wheel installation may have issues" -ForegroundColor Yellow
        }
    } else {
        Write-Host "[ERROR] No wheel file found" -ForegroundColor Red
    }
} else {
    Write-Host "[ERROR] Compilation failed" -ForegroundColor Red
    Write-Host $buildOutput -ForegroundColor Red
}

Read-Host "`nPress Enter to exit"
