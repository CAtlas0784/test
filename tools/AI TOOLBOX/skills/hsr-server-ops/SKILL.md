---
name: hsr-server-ops
description: Comprehensive runbook for building, running, optimizing, and packaging the HSR Rust and C# servers.
---

# HSR Server Operations Runbook

## 1. Quick Launch & Architecture
- **Primary Server Workspace**: c:\Users\Phitchayut\Desktop\Hoyo-hkrpg-PS
- **Network Ports**:
  - 21000: SDK / Dispatch HTTP server (sdkserver.exe)
  - 23301: GameServer MhyKcp server (gameserver.exe)
- **Launch Script**: [un.bat](file:///c:/Users/Phitchayut/Desktop/Hoyo-hkrpg-PS/run.bat)
  - Automatically runs 	arget\release\sdkserver.exe and 	arget\release\gameserver.exe.
  - Falls back to 	arget\debug\ if release binaries do not exist.
  - Portable ready: no .exe files should be placed loosely in the root directory.

## 2. Compilation Profiles
- **Release Build (Production / Distributable)**:
  - Command: cargo build --release or double-click [uild.bat](file:///c:/Users/Phitchayut/Desktop/Hoyo-hkrpg-PS/build.bat)
  - Output: 	arget\release\gameserver.exe (2.56 MB), 	arget\release\sdkserver.exe (5.16 MB)
  - Full optimizations, high performance, compact binary size.
- **Debug Build (Fast iteration during active development)**:
  - Command: cargo build
  - Output: 	arget\debug\*.exe (fast compile, unoptimized).

## 3. Configuration & Scene Files
- **Hotfixes**: ersions.json (maps client version e.g. OSBETAWin4.5.52 to hotfix asset bundles).
- **Scene & Player Position**: persistent/ (contains JSON coordinates per UID).
  - Safe Express Parlor Car fallback: plane_id: 20001, floor_id: 20001001, entry_id: 2000101, pos: {x: 0, y: 0, z: 0}.
- **FreeCam System**: scripts/freecam.lua (hot-reloaded via PlayerHeartBeatScRsp.download_data CmdID 62).

## 4. Dependencies & Python Management
- Core server runs on native .exe files (zero runtime dependencies for friends).
- Any auxiliary Python tools must be recorded in [equirements.txt](file:///c:/Users/Phitchayut/Desktop/Hoyo-hkrpg-PS/requirements.txt).
