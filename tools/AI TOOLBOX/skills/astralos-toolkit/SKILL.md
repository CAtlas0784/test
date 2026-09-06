---
name: astralos-toolkit
description: Complete specification and operational guide for AstralOS at C:\Users\Phitchayut\Desktop\AstralOS: local server emulator (RobinSR), Morax reverse engineering suite, in-game language patcher, HDiff patcher, version.dll protector, MCP server, and menu.bat CLI operations.
---

# AstralOS Toolkit Guide

## 1. Overview & Location
- **Location**: `C:\Users\Phitchayut\Desktop\AstralOS`
- **Purpose**: All-in-one local server emulator (RobinSR), reverse engineering toolkit (Morax), in-game language patcher, delta patch applier with snapshot rollback, IL2CPP metadata/Protobuf de-obfuscation suite, and MCP server.

## 2. Master Control Script (`menu.bat`) Commands
All features can be executed non-interactively via CLI flags:

| Command Line Flag | Action / Description |
| :--- | :--- |
| `.\menu.bat --launch-all` | Starts RobinSR Server, auto-deploys `version.dll`, and opens the Desktop GUI. |
| `.\menu.bat --game` | Deploys `version.dll` (+R locked) and launches `StarRail.exe` directly. |
| `.\menu.bat --fix-dll` | Deploys and write-protects `version.dll` (`attrib +r`) in the game directory. |
| `.\menu.bat --robinsr` | Starts local RobinSR gameserver (`:23301`) and HTTP dispatch (`:21000`). |
| `.\menu.bat --desktop` | Launches AstralOS Tauri desktop application. |
| `.\menu.bat --web` | Launches React 19 web frontend. |
| `.\menu.bat --stop` | Kills running gameserver / sdkserver processes and frees ports 21000 & 23301. |
| `.\menu.bat --morax` | Runs Morax IL2CPP & Protobuf static decoder on `GameAssembly.dll` and `global-metadata.dat`. |
| `.\menu.bat --res-json` | Compiles minified `res.json` from client configs via `res_compiler`. |
| `.\menu.bat --lang` | Runs in-game language patcher (unlocks all 13 languages including Thai). |
| `.\menu.bat --hdiff` | Applies differential update patches (`.hdiff`, `.patch`, `.zip`) with rollback. |
| `.\menu.bat --reset-pos` | Resets player coordinates to safe default (Parlor Car). |
| `.\menu.bat --build-all` | Compiles all workspace crates, web, and desktop. |
| `.\menu.bat --clean` | Cleans build caches. |
| `.\menu.bat --check` | Validates environment dependencies (MSVC, Rust nightly, Node.js, WebView2). |

## 3. Core Binary Tools in `bin/`
- **`bin\hsr-mcp.exe`**: Model Context Protocol server exposing AI tools (`dump_metadata`, `find_method_rva`, `execute_lua_script`, `send_network_packet`).
- **`bin\gameserver.exe`**: KCP UDP gameserver (port 23301).
- **`bin\sdkserver.exe`**: HTTP dispatch & gateway server (port 21000).
- **`bin\version.dll`**: In-game memory dumper hook DLL.
- **`bin\launcher.exe`**: Game launcher proxy.
- **`bin\res.json`**: Compiled world/scene database (~12.8 MB, minified).
- **`bin\versions.json`**: Version routing & hotfix CDN mapping table.

## 4. Morax Reverse Engineering Suite (`crates/morax`)
- **`target\release\morax.exe`**:
  - Offline static decoder for `GameAssembly.dll` and `global-metadata.dat`.
  - Disassembles `WriteTo` bytecodes using `iced-x86`.
  - Generates: `StarRail.proto`, `packetIds.json`, `dump.cs`, `methods.json`, `il2cpp.h`.
- **`target\release\res_compiler.exe`**:
  - Compiles raw `Config/LevelOutput` and `ExcelOutput` tables into minified `res.json`.

## 5. In-Game Chat Commands (Server UID 727 / RobinSR)
- `sync`: Syncs inventory, stats, and relics between `persistent` and live game view.
- `mc <path / id>`: Switches Trailblazer path (`destruction`, `preservation`, `harmony`, `remembrance`).
- `march <path / id>`: Switches March 7th path (`preservation`, `hunt`).
- `sw on / off`: Toggles Silver Wolf global team damage & resistance debuff.
- `castorice on / off`: Toggles Castorice global buff.
- `lua <script>`: Executes an external XLua script in live client.
- `cl clear`: Clears all chat messages.

## 6. MCP Server Integration
Configured in `~/.gemini/config/mcp_config.json`:
```json
{
  "mcpServers": {
    "astralos": {
      "command": "C:\\Users\\Phitchayut\\Desktop\\AstralOS\\bin\\hsr-mcp.exe",
      "args": []
    }
  }
}
```
Available AI MCP Tools:
1. `dump_metadata`: Decrypts `global-metadata.dat` and generates `dump.cs` + headers.
2. `find_method_rva`: Resolves method signatures to RVA addresses.
3. `execute_lua_script`: Dispatches arbitrary XLua scripts to the running game client.
4. `send_network_packet`: Injects client/server network packets to test gameplay logic.

## 7. Using AstralOS for Private Server Development (`Hoyo-hkrpg-PS`)

### Primary Workflows:
1. **Testing Game Client against `Hoyo-hkrpg-PS`**:
   - Run `tools.bat` (option 1 or 2) directly inside `Hoyo-hkrpg-PS`.
   - Automatically deploys AstralOS's `version.dll` (+r write-protected) to the game directory (`E:\beta hsr\StarRail_4.5.52_OS`).
   - Launches `StarRail.exe` pre-redirected to local dispatch (`127.0.0.1:21000`) and gameserver (`127.0.0.1:23301`).

2. **Investigating Missing Packets & Infinite Loading Spinners**:
   - When the client hangs on a 3-dot spinner or fails to open a feature (e.g. MoC, PF, Lineup):
     - Check console for `[PACKET] Received cmd_id: <ID>`.
     - If the packet structure or response tags are unknown, run Morax via `tools.bat` option 5 or `bin\morax.exe all --raw -g "E:\beta hsr\StarRail_4.5.52_OS"`.
     - Morax extracts the exact `StarRail.proto` message fields and `packetIds.json` CmdId mappings from `GameAssembly.dll`.

3. **Generating Server Data (`res.json`)**:
   - When new maps, mazes, or stages are needed, run `tools.bat` option 6 (`res_compiler.exe "E:\beta hsr\StarRail_4.5.52_OS\Config" "res.json"`).

4. **Live In-Game Testing & Scripts**:
   - Use `execute_lua_script` via MCP to dispatch live scripts (e.g. `scripts\freecam.lua` or avatar unlocking) directly into the hooked client.
   - Use `tools.bat` option 7 to set client text language to Thai (`th`).
   - Use `tools.bat` option 8 to delete `persistent` and reset the player's spawn back to Parlor Car if stuck.
