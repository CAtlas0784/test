@echo off
cd /d "%~dp0"
echo Starting Hoyo-hkrpg-PS Servers...

if exist "sdkserver.exe" (
    start "SDK Server" cmd /k "sdkserver.exe"
) else if exist "target\release\sdkserver.exe" (
    start "SDK Server" cmd /k "target\release\sdkserver.exe"
) else (
    start "SDK Server" cmd /k "target\debug\sdkserver.exe"
)

timeout /t 2 /nobreak >nul

if exist "gameserver.exe" (
    start "Game Server" cmd /k "gameserver.exe"
) else if exist "target\release\gameserver.exe" (
    start "Game Server" cmd /k "target\release\gameserver.exe"
) else (
    start "Game Server" cmd /k "target\debug\gameserver.exe"
)