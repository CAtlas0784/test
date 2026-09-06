import json

with open(r"E:\beta hsr\himeko-nova-sr\himeko-nova-sr\resources\ActivityConfig.json", "r", encoding="utf-8") as f:
    data = json.load(f)

for item in data.get("activity_config", []):
    aid = item.get("ActivityID")
    panel = item.get("ActivityPanelID")
    modules = item.get("ActivityModuleIDList")
    print(f"ActivityID: {aid}, Panel: {panel}, Modules: {modules}")
