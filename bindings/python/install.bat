@echo off
chcp 65001 >nul 2>n1
echo ========================================
echo   .list Python Binding Installer
echo ========================================
echo.

REM Check Python
python --version >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Python not found. Please install Python 3.8+
    pause
    exit /b 1
)

echo [OK] Python detected:
python --version
echo.

REM Check pip
pip --version >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] pip not found
    pause
    exit /b 1
)

echo [OK] pip detected
echo.

REM Install maturin
echo [1/3] Installing Maturin...
pip install maturin 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [OK] Maturin installed successfully
) else (
    echo [INFO] Maturin may already installed, continuing...
)
echo.

REM Build and install
echo [2/3] Compiling Rust + Python bindings (Release mode)...
cd /d "%~dp0..\.."
maturin build --release --features python -o python_bindings
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [OK] Python bindings compiled successfully!
) else (
    echo.
    echo [ERROR] Compilation failed. Please check error messages above.
    pause
    exit /b 1
)
echo.

REM Install wheel
echo [3/3] Installing the wheel package...
for %%f in (python_bindings\list_lang-*.whl) do (
    set WHEEL_FILE=%%f
)
if defined WHEEL_FILE (
    pip install "%WHEEL_FILE%" --force-reinstall
    if %ERRORLEVEL% EQU 0 (
        echo.
        echo ========================================
        echo   Installation Complete!
        echo ========================================
        echo.
        echo   You can now use list_lang in Python!
        echo.
        echo   Quick test:
        echo     python -c "from list_lang import PyListData; print('OK')"
        echo.
        echo   Run examples:
        echo     python bindings\python\quick_start.py
        echo     python bindings\python\test_bindings.py
        echo.
    ) else (
        echo [WARN] Installation may have issues
    )
) else (
    echo [ERROR] No wheel file found in python_bindings folder
)

pause
