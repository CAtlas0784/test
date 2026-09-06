---
name: il2cpp-enum-population
description: Extracting Protocol Command IDs, packet opcodes, and protobuf message tags from dumped IL2CPP metadata.
---
# IL2CPP Enum Population
- Parse `dump.cs` for classes extending `Google.Protobuf.IMessage`.
- Extract field numbers and `CmdID` constants for packet routing.
