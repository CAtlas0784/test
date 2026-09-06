import json, struct

with open(r"E:\beta hsr\himeko-nova-sr\himeko-nova-sr\resources\ChallengeMazeConfig.json", "r", encoding="utf-8") as f:
    data = json.load(f)

path = r"E:\beta hsr\StarRail_4.5.52_OS\StarRail_Data\Persistent\DesignData\Windows\en\07a2562ffa38532eab0921374f4c443b.bytes"
with open(path, "rb") as f:
    textmap = f.read()

target_pos = textmap.find(b"Survival of the Fittest (I)")
print("target_pos for (I):", hex(target_pos))

# In HSR TextMap, entries are stored with Hash64 or Hash32.
# Let's check all entries in challenge_config
for c in data.get("challenge_config", []):
    name_obj = c.get("Name", {})
    h32 = name_obj.get("Hash")
    h64 = name_obj.get("Hash64")
    for val, fmt in [(h32, "<i"), (h64, "<Q")]:
        if val is not None:
            try:
                b = struct.pack(fmt, val)
                pos = textmap.find(b)
                if pos != -1 and abs(pos - target_pos) < 100:
                    print(f"EXACT MATCH! Stage {c.get('ID')}, Group {c.get('GroupID')}, Floor {c.get('Floor')}, val={val}")
            except:
                pass
