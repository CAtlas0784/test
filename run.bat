@echo off
cd /d "%~dp0"
echo ==============================================
echo   Starting Hoyo-hkrpg-PS Servers...
echo ==============================================

set "SDK_EXE="
if exist "sdkserver.exe" set "SDK_EXE=sdkserver.exe"
if not defined SDK_EXE if exist "target\release\sdkserver.exe" set "SDK_EXE=target\release\sdkserver.exe"
if not defined SDK_EXE if exist "target\debug\sdkserver.exe" set "SDK_EXE=target\debug\sdkserver.exe"

set "GAME_EXE="
if exist "gameserver.exe" set "GAME_EXE=gameserver.exe"
if not defined GAME_EXE if exist "target\release\gameserver.exe" set "GAME_EXE=target\release\gameserver.exe"
if not defined GAME_EXE if exist "target\debug\gameserver.exe" set "GAME_EXE=target\debug\gameserver.exe"

if not defined SDK_EXE (
    echo [!] Server executables not found (target folder was deleted or not built yet).
    echo [*] Compiling now with 'cargo build --release'...
    call cargo build --release
    if exist "target\release\sdkserver.exe" set "SDK_EXE=target\release\sdkserver.exe"
    if exist "target\release\gameserver.exe" set "GAME_EXE=target\release\gameserver.exe"
)

if not defined SDK_EXE (
    echo [X] Build failed or cargo is not installed.
    pause
    exit /b 1
)

echo [*] Launching SDK Server (%SDK_EXE%)...
start "SDK Server" cmd /k "%SDK_EXE%"

timeout /t 2 /nobreak >nul

echo [*] Launching Game Server (%GAME_EXE%)...
start "Game Server" cmd /k "%GAME_EXE%"