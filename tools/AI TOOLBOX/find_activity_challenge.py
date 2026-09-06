import json

with open(r"E:\beta hsr\StarRail_4.5.52_OS\DUMP\methods2.json", "r", encoding="utf-8", errors="ignore") as f:
    methods2 = json.load(f)

print("Searching methods connecting Activity and ChallengeGroup...")
matches = []
for k in methods2:
    if ("Activity" in k and "Challenge" in k) or ("Schedule" in k and "Group" in k):
        matches.append(k)

for m in sorted(matches):
    print(m)
