import os, re, glob

maps = {
    0: "Azeroth",
    1: "Kalimdor",
    13: "test",
    25: "ScottTest",
    29: "Test",
    30: "PVPZone01",
    33: "Shadowfang",
    34: "StormwindJail",
    35: "StormwindPrison",
    36: "DeadminesInstance",
    37: "PVPZone02",
    42: "Collin",
    43: "WailingCaverns",
    44: "Monastery",
    47: "RazorfenKraulInstance",
    48: "Blackfathom",
    70: "Uldaman",
    90: "GnomeragonInstance",
    109: "SunkenTemple",
    129: "RazorfenDowns",
    169: "EmeraldDream",
    189: "MonasteryInstances",
    209: "TanarisInstance",
    229: "BlackRockSpire",
    230: "BlackrockDepths",
    249: "OnyxiaLairInstance",
    269: "CavernsOfTime",
    289: "SchoolofNecromancy",
    309: "Zul'gurub",
    329: "Stratholme",
    349: "Mauradon",
    369: "DeeprunTram",
    389: "OrgrimmarInstance",
    409: "MoltenCore",
    429: "DireMaul",
    449: "AlliancePVPBarracks",
    450: "HordePVPBarracks",
    451: "development",
    469: "BlackwingLair",
    489: "PVPZone03",
    509: "AhnQiraj",
    529: "PVPZone04",
    531: "AhnQirajTemple",
    533: "Stratholme Raid",
}

map_list = {}
for fn in glob.glob("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map*.obj"):
    filename = os.path.basename(fn)
    mch = re.match("^map(\d{3})(\d{2})(\d{2}).obj$", filename)
    if mch == None:
        continue

    map_id = int(mch.group(1))
    if map_id in maps:
        if map_id not in map_list:
            map_list[map_id] = []
        map_list[map_id].append(fn)

for map_id, files in map_list.items():
    print(maps[map_id])
    if os.path.exists("foop.falkvbo"):
        os.unlink("foop.falkvbo")
    assert os.system("cargo run --release " + " ".join(files)) == 0
    os.rename("foop.falkvbo", maps[map_id] + ".falkvbo")

