@echo off
echo 启动 Tauri 开发服务器...
echo.

REM 将 Cargo 添加到 PATH
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

REM 验证 Cargo
cargo --version
if errorlevel 1 (
    echo 错误: 找不到 Cargo
    pause
    exit /b 1
)

echo.
echo 正在启动...
pnpm tauri dev


