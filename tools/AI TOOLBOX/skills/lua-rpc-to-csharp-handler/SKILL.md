---
name: lua-rpc-to-csharp-handler
description: Architecture for executing client-side Lua scripts and communicating with C# server handlers via Heartbeat data.
---
# Lua RPC to C# Handler
- Delivers Lua bytecode or raw script via `PlayerHeartBeatScRsp.download_data`.
- Client executes script in Unity Lua state.
