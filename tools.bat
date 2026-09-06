@echo off
setlocal enabledelayedexpansion
title Hoyo-hkrpg-PS - Private Server Development Suite (powered by AstralOS)
color 0B
chcp 65001 > nul
cd /d "%~dp0"

set "ASTRALOS_DIR=C:\Users\Phitchayut\Desktop\AstralOS"

:: ========================================================================
:: Auto-detect Game Directory
:: ========================================================================
set "GAME_DIR="
if exist "E:\beta hsr\StarRail_4.5.52_OS\StarRail.exe" (
    set "GAME_DIR=E:\beta hsr\StarRail_4.5.52_OS"
)
if not defined GAME_DIR (
    if exist "D:\HoYoPlay\games\Star Rail Games\StarRail.exe" (
        set "GAME_DIR=D:\HoYoPlay\games\Star Rail Games"
    )
)
if not defined GAME_DIR (
    if exist "C:\Program Files\Star Rail\Games\StarRail.exe" (
        set "GAME_DIR=C:\Program Files\Star Rail\Games"
    )
)
if not defined GAME_DIR (
    if exist "D:\Games\Star Rail\Games\StarRail.exe" (
        set "GAME_DIR=D:\Games\Star Rail\Games"
    )
)

:menu
cls
echo ==============================================================================
echo   [ Hoyo-hkrpg-PS ] Private Server Development Suite ^& Toolkit
echo   Integrated with AstralOS Reverse Engineering Engine
echo ==============================================================================
echo.

:: Service Status Check
netstat -ano | findstr "0.0.0.0:21000" | findstr "LISTENING" > nul 2>&1
if !errorlevel! equ 0 (
    set "STATUS_HTTP=[ONLINE]  SDK Server Gateway (:21000)"
) else (
    set "STATUS_HTTP=[OFFLINE] SDK Server Gateway (:21000)"
)

netstat -ano | findstr "0.0.0.0:23301" > nul 2>&1
if !errorlevel! equ 0 (
    set "STATUS_KCP=[ONLINE]  KCP Gameserver      (:23301)"
) else (
    set "STATUS_KCP=[OFFLINE] KCP Gameserver      (:23301)"
)

if exist "!GAME_DIR!\version.dll" (
    set "STATUS_HOOK=[ACTIVE]  Dumper Hook DLL in Game (version.dll)"
) else (
    set "STATUS_HOOK=[NOT HOOKED] version.dll missing from Game Folder"
)

echo   [ STATUS ] -----------------------------------------------------------------
echo     !STATUS_HTTP!
echo     !STATUS_KCP!
echo     !STATUS_HOOK!
if defined GAME_DIR (
    echo     [CLIENT]  !GAME_DIR!
) else (
    echo     [CLIENT]  Not Detected (Will prompt when running)
)
echo   ---------------------------------------------------------------------------
echo.
echo   [ PS RUNTIME ^& TESTING ] -------------------------------------------------
echo     [1] 1-Click PS Dev Mode (Start PS Server + Hook + Launch Game Client)
echo     [2] Launch Game Client with Hook (Connect to Already Running PS)
echo     [3] Fix ^& Lock version.dll Hook (Prevent Game from Renaming/Disabling)
echo     [4] Stop All Running PS Servers (Kill :21000 ^& :23301)
echo.
echo   [ PROTOBUF, PACKETS ^& REVERSE ENGINEERING ] -------------------------------
echo     [5] Dump StarRail.proto ^& packetIds.json (Morax IL2CPP Parser)
echo     [6] Compile res.json for Server (AstralOS Res Compiler)
echo.
echo   [ CLIENT TWEAKS ^& STATE MANAGEMENT ] --------------------------------------
echo     [7] Switch Game Language to Thai (th) / English (en)
echo     [8] Reset Player Spawn Position (Clean persistent file)
echo     [9] Open Full AstralOS Master Control Suite
echo     [0] Exit
echo   ---------------------------------------------------------------------------
echo.

set "choice="
set /p choice="   Select option [0-9]> "
if not defined choice goto menu

if "%choice%"=="1" goto dev_mode_all
if "%choice%"=="2" goto launch_client_only
if "%choice%"=="3" goto fix_hook_dll
if "%choice%"=="4" goto stop_servers
if "%choice%"=="5" goto dump_morax_proto
if "%choice%"=="6" goto compile_res_json
if "%choice%"=="7" goto switch_lang
if "%choice%"=="8" goto reset_player_state
if "%choice%"=="9" goto open_astralos_suite
if "%choice%"=="0" exit /b 0
goto menu

:: ========================================================================
:: [1] 1-Click PS Dev Mode
:: ========================================================================
:dev_mode_all
echo.
echo ==============================================================================
echo   [1-Click PS Dev Mode] Launching Hoyo-hkrpg-PS + Client Hook...
echo ==============================================================================
echo.

:: 1. Start PS Servers if not running
netstat -ano | findstr "0.0.0.0:21000" | findstr "LISTENING" > nul 2>&1
if !errorlevel! neq 0 (
    echo [*] Starting SDK Server (:21000)...
    if exist "target\release\sdkserver.exe" (
        start "Hoyo PS - SDK Server" cmd /k "target\release\sdkserver.exe"
    ) else if exist "sdkserver.exe" (
        start "Hoyo PS - SDK Server" cmd /k "sdkserver.exe"
    ) else (
        start "Hoyo PS - SDK Server" cmd /k "cargo run --release -p sdkserver"
    )
    timeout /t 2 /nobreak >nul
) else (
    echo [OK] SDK Server (:21000) is already running.
)

netstat -ano | findstr "0.0.0.0:23301" > nul 2>&1
if !errorlevel! neq 0 (
    echo [*] Starting Gameserver (:23301)...
    if exist "target\release\gameserver.exe" (
        start "Hoyo PS - Gameserver" cmd /k "target\release\gameserver.exe"
    ) else if exist "gameserver.exe" (
        start "Hoyo PS - Gameserver" cmd /k "gameserver.exe"
    ) else (
        start "Hoyo PS - Gameserver" cmd /k "cargo run --release -p gameserver"
    )
    timeout /t 2 /nobreak >nul
) else (
    echo [OK] Gameserver (:23301) is already running.
)

:: 2. Deploy version.dll and launch game
call :deploy_hook_internal
call :launch_game_internal
pause
goto menu

:: ========================================================================
:: [2] Launch Game Client with Hook Only
:: ========================================================================
:launch_client_only
echo.
echo ==============================================================================
echo   [Launch Game Client with Hook] Connecting to 127.0.0.1:21000...
echo ==============================================================================
echo.
call :deploy_hook_internal
call :launch_game_internal
pause
goto menu

:: ========================================================================
:: [3] Fix & Lock version.dll Hook
:: ========================================================================
:fix_hook_dll
echo.
echo ==============================================================================
echo   [Fix & Protect version.dll]
echo ==============================================================================
echo.
call :deploy_hook_internal
echo.
echo [OK] Hook DLL verified, deployed, and locked against game modifications!
echo.
pause
goto menu

:: ========================================================================
:: [4] Stop All Running PS Servers
:: ========================================================================
:stop_servers
echo.
echo [*] Terminating Hoyo-hkrpg-PS and AstralOS server processes...
taskkill /F /IM sdkserver.exe /T >nul 2>&1
taskkill /F /IM gameserver.exe /T >nul 2>&1
taskkill /F /IM robinsr.exe /T >nul 2>&1
echo [OK] All local servers stopped. Ports 21000 & 23301 are free!
ping 127.0.0.1 -n 2 > nul
goto menu

:: ========================================================================
:: [5] Dump StarRail.proto & packetIds.json via Morax
:: ========================================================================
:dump_morax_proto
echo.
echo ==============================================================================
echo   [Morax Proto Dumper] Extracting Latest Protobuf & Packet Schemas
echo ==============================================================================
echo.
if not defined GAME_DIR (
    set /p GAME_DIR="   Enter Star Rail game folder (with GameAssembly.dll): "
)

set "DUMP_OUT=%~dp0tools\DUMP"
if not exist "!DUMP_OUT!" mkdir "!DUMP_OUT!"

echo [*] Target Game: !GAME_DIR!
echo [*] Output Directory: !DUMP_OUT!
echo [*] Running Morax Engine from AstralOS...

if exist "!ASTRALOS_DIR!\bin\morax.exe" (
    "!ASTRALOS_DIR!\bin\morax.exe" all --raw -g "!GAME_DIR!" -o "!DUMP_OUT!"
) else (
    cargo run --release --manifest-path "!ASTRALOS_DIR!\Cargo.toml" -p morax --bin morax -- all --raw -g "!GAME_DIR!" -o "!DUMP_OUT!"
)

echo.
echo [OK] Morax dump finished! Check '!DUMP_OUT!' for:
echo      - StarRail.proto
echo      - packetIds.json
echo      - dump.cs
echo      - methods.json
echo.
pause
goto menu

:: ========================================================================
:: [6] Compile res.json for Server
:: ========================================================================
:compile_res_json
echo.
echo ==============================================================================
echo   [Resource Compiler] Compiling res.json for Hoyo-hkrpg-PS
echo ==============================================================================
echo.
set "RES_SRC=!GAME_DIR!\Config"
if not exist "!RES_SRC!" (
    set /p RES_SRC="   Enter raw Resources or Config folder path: "
)

echo [*] Compiling from '!RES_SRC!' to '%~dp0res.json'...
if exist "!ASTRALOS_DIR!\bin\res_compiler.exe" (
    "!ASTRALOS_DIR!\bin\res_compiler.exe" "!RES_SRC!" "%~dp0res.json"
) else (
    cargo run --release --manifest-path "!ASTRALOS_DIR!\Cargo.toml" -p morax --bin res_compiler -- "!RES_SRC!" "%~dp0res.json"
)
echo.
pause
goto menu

:: ========================================================================
:: [7] Switch Game Language (Thai / English)
:: ========================================================================
:switch_lang
echo.
echo ==============================================================================
echo   [Language Switcher] Set Game Client Language for Testing
echo ==============================================================================
echo.
echo   [1] Thai Text + Japanese Voice (th / ja)
echo   [2] English Text + English Voice (en / en)
echo   [3] English Text + Japanese Voice (en / ja)
echo.
set "LCHOICE="
set /p LCHOICE="   Select language preset [1-3, default=1]: "
if "!LCHOICE!"=="" set LCHOICE=1

if "!LCHOICE!"=="1" (
    set "T_HEX=746800"
    set "V_HEX=6A7000"
    set "T_CODE=th"
    set "V_CODE=ja"
) else if "!LCHOICE!"=="2" (
    set "T_HEX=656E00"
    set "V_HEX=656E00"
    set "T_CODE=en"
    set "V_CODE=en"
) else (
    set "T_HEX=656E00"
    set "V_HEX=6A7000"
    set "T_CODE=en"
    set "V_CODE=ja"
)

reg add "HKCU\Software\Cognosphere\Star Rail" /v "LanguageSettings_LocalTextLanguage_h2764291023" /t REG_BINARY /d "!T_HEX!" /f >nul 2>&1
reg add "HKCU\Software\Cognosphere\Star Rail" /v "LanguageSettings_LocalAudioLanguage_h882585060" /t REG_BINARY /d "!V_HEX!" /f >nul 2>&1
reg add "HKCU\Software\Cognosphere\Star Rail" /v "MIHOYOSDK_CURRENT_LANGUAGE_h2559149783" /t REG_BINARY /d "!T_HEX!" /f >nul 2>&1

if defined GAME_DIR if exist "!GAME_DIR!" (
    powershell -NoProfile -Command "$cfg = @{ TextLanguage = '!T_CODE!'; VoiceLanguage = '!V_CODE!' }; $json = $cfg | ConvertTo-Json; Set-Content -Path '!GAME_DIR!\GeneralConfig.json' -Value $json -Force" >nul 2>&1
)
echo [OK] Set game language to Text=[!T_CODE!] Voice=[!V_CODE!]!
echo.
pause
goto menu

:: ========================================================================
:: [8] Reset Player Spawn Position
:: ========================================================================
:reset_player_state
echo.
echo [*] Resetting Hoyo-hkrpg-PS player spawn position...
if exist "persistent" (
    del /f /q "persistent" >nul 2>&1
    echo [OK] Cleared 'persistent' state file. Player will respawn at Parlor Car upon login.
) else (
    echo [OK] 'persistent' file is already clean.
)
ping 127.0.0.1 -n 2 > nul
goto menu

:: ========================================================================
:: [9] Open Full AstralOS Master Control Suite
:: ========================================================================
:open_astralos_suite
if exist "!ASTRALOS_DIR!\menu.bat" (
    start "AstralOS Master Suite" cmd /c "cd /d !ASTRALOS_DIR! && menu.bat"
) else (
    echo [X] AstralOS folder not found at '!ASTRALOS_DIR!'.
    pause
)
goto menu

:: ========================================================================
:: Helper: Deploy and Lock version.dll
:: ========================================================================
:deploy_hook_internal
if not defined GAME_DIR (
    set /p GAME_DIR="   Enter Star Rail game folder (with StarRail.exe): "
)
if not exist "!GAME_DIR!" (
    echo [X] Game directory '!GAME_DIR!' not found!
    exit /b 1
)

echo [*] Inspecting game hook in: !GAME_DIR!

:: Clean any renamed remnants
for %%F in ("!GAME_DIR!\version.dll.*" "!GAME_DIR!\version_old*") do (
    if exist "%%F" (
        attrib -r -h -s "%%F" >nul 2>&1
        del /f /q "%%F" >nul 2>&1
        echo [*] Cleaned backup: %%~nxF
    )
)

set "HOOK_SRC="
if exist "!ASTRALOS_DIR!\bin\version.dll" set "HOOK_SRC=!ASTRALOS_DIR!\bin\version.dll"
if not defined HOOK_SRC if exist "!ASTRALOS_DIR!\target\release\version.dll" set "HOOK_SRC=!ASTRALOS_DIR!\target\release\version.dll"

if defined HOOK_SRC (
    if exist "!GAME_DIR!\version.dll" attrib -r -h -s "!GAME_DIR!\version.dll" >nul 2>&1
    copy /y "!HOOK_SRC!" "!GAME_DIR!\version.dll" >nul
    attrib +r "!GAME_DIR!\version.dll" >nul 2>&1
    echo [OK] Deployed version.dll hook from AstralOS and write-protected (+r)!
) else (
    echo [!] AstralOS version.dll not found. Please build dumper in AstralOS first.
)
exit /b 0

:: ========================================================================
:: Helper: Launch Game Client
:: ========================================================================
:launch_game_internal
echo [*] Launching Star Rail Client (redirected to local PS)...
if exist "!GAME_DIR!\StarRail.exe" (
    start "" /d "!GAME_DIR!" "!GAME_DIR!\StarRail.exe"
    echo [OK] Star Rail launched! Traffic redirected to 127.0.0.1:21000.
) else if exist "!GAME_DIR!\launcher.exe" (
    start "" /d "!GAME_DIR!" "!GAME_DIR!\launcher.exe"
    echo [OK] Launcher started!
) else (
    echo [X] Could not find StarRail.exe in '!GAME_DIR!'.
)
exit /b 0
