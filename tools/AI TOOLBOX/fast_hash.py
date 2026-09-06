import struct, json

path = r"E:\beta hsr\StarRail_4.5.52_OS\StarRail_Data\Persistent\DesignData\Windows\en\07a2562ffa38532eab0921374f4c443b.bytes"
with open(path, "rb") as f:
    textmap = f.read()

target_pos = textmap.find(b"Survival of the Fittest (I)")
header_bytes = textmap[target_pos-20:target_pos]
print("header_bytes:", list(header_bytes))

with open(r"E:\beta hsr\himeko-nova-sr\himeko-nova-sr\resources\ChallengeMazeConfig.json", "r", encoding="utf-8") as f:
    data = json.load(f)

# Put all hashes into a dict
hash_to_stage = {}
for c in data.get("challenge_config", []):
    h32 = c.get("Name", {}).get("Hash")
    h64 = c.get("Name", {}).get("Hash64")
    if h32: hash_to_stage[h32] = c
    if h64: hash_to_stage[h64] = c

# Check possible 4-byte and 8-byte ints in header_bytes
for offset in range(len(header_bytes) - 4):
    val32 = struct.unpack("<i", header_bytes[offset:offset+4])[0]
    if val32 in hash_to_stage:
        c = hash_to_stage[val32]
        print("FOUND MATCH 32-bit!", val32, "Stage:", c.get("ID"), "Group:", c.get("GroupID"))

for offset in range(len(header_bytes) - 8):
    val64 = struct.unpack("<Q", header_bytes[offset:offset+8])[0]
    if val64 in hash_to_stage:
        c = hash_to_stage[val64]
        print("FOUND MATCH 64-bit!", val64, "Stage:", c.get("ID"), "Group:", c.get("GroupID"))
