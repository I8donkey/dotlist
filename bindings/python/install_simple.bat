# Python Binding Installation Guide
# ==================================
# Simple 3-step manual installation

echo Step 1: Install Maturin (build tool)
pip install maturin

echo.
echo Step 2: Build the wheel package
cd /d "%~dp0..\.."
maturin build --release --features python -o python_bindings

echo.
echo Step 3: Install the generated wheel
for %%f in (python_bindings\list_lang-*.whl) do (
    echo Installing: %%f
    pip install "%%f" --force-reinstall
)

echo.
echo Done! Test with:
echo   python -c "from list_lang import PyListData; print('OK')"

pause
