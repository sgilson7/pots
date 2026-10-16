#!/usr/bin/env python3
"""Writes data/map.json: the Sunward half by hand, the Moonward half as its
point mirror through the centre, so the two sides are the same distance from
everything (DotA's map is close to this symmetry, not exactly in it).

Run: python3 tools/mapgen.py
The layout follows the literature review's account of DotA Allstars 6.6x:
three lanes, a diagonal river crossing mid at the centre, the Sunward base in
the south-west (Sentinel's corner) and the Moonward in the north-east.
"""
import json, os

S = 16384
def m(p):
    return [S - p[0], S - p[1]]

def ml(line):
    return [m(p) for p in line]

sun_lanes = {
    "top": [[1700, 3200], [1450, 4400], [1400, 8200], [1450, 13300], [2700, 14850], [8200, 15000], [12300, 14950], [13300, 14600], [14084, 14084]],
    "mid": [[3100, 3100], [3500, 3500], [8192, 8192], [12884, 12884], [14084, 14084]],
    "bot": [[3200, 1700], [4400, 1450], [8200, 1400], [13300, 1450], [14850, 2700], [15000, 8200], [14950, 12300], [14600, 13300], [14084, 14084]],
}
# Moonward walks the same lanes the other way and ends at the Sunward Sphere.
moon_lanes = {}
for k, line in sun_lanes.items():
    # the same ground walked back, from inside the Moonward base to the Sunward Sphere
    moon_lanes[k] = list(reversed(line[:-1])) + [[2300, 2300]]

sun_buildings = [
    # towers: tier 1 farthest out, tier 3 at the base, two tier 4 by the Sphere
    {"unit": "tower1_sun", "at": [1100, 10300], "lane": "top", "tier": 1},
    {"unit": "tower2_sun", "at": [1100, 7100], "lane": "top", "tier": 2},
    {"unit": "tower3_sun", "at": [1150, 4900], "lane": "top", "tier": 3},
    {"unit": "tower1_sun", "at": [6450, 6050], "lane": "mid", "tier": 1},
    {"unit": "tower2_sun", "at": [5250, 4850], "lane": "mid", "tier": 2},
    {"unit": "tower3_sun", "at": [4250, 3850], "lane": "mid", "tier": 3},
    {"unit": "tower1_sun", "at": [12500, 1100], "lane": "bot", "tier": 1},
    {"unit": "tower2_sun", "at": [7900, 1100], "lane": "bot", "tier": 2},
    {"unit": "tower3_sun", "at": [4900, 1150], "lane": "bot", "tier": 3},
    {"unit": "tower4_sun", "at": [2950, 2050], "lane": "base", "tier": 4},
    {"unit": "tower4_sun", "at": [2050, 2950], "lane": "base", "tier": 4},
    {"unit": "barracks_melee_sun", "at": [1100, 3900], "lane": "top"},
    {"unit": "barracks_ranged_sun", "at": [2050, 3900], "lane": "top"},
    {"unit": "barracks_melee_sun", "at": [3850, 3050], "lane": "mid"},
    {"unit": "barracks_ranged_sun", "at": [3050, 3850], "lane": "mid"},
    {"unit": "barracks_melee_sun", "at": [3900, 1100], "lane": "bot"},
    {"unit": "barracks_ranged_sun", "at": [3900, 2050], "lane": "bot"},
    {"unit": "sphere_sun", "at": [2300, 2300], "lane": "base", "facing": 45},
    {"unit": "fountain_sun", "at": [650, 650], "lane": "base", "facing": 45},
]
moon_buildings = []
for b in sun_buildings:
    c = dict(b)
    c["unit"] = b["unit"].replace("_sun", "_moon")
    c["at"] = m(b["at"])
    c["facing"] = (b.get("facing", 45) + 180) % 360
    # the mirror swaps which physical lane a tower guards
    c["lane"] = {"top": "bot", "bot": "top"}.get(b["lane"], b["lane"])
    moon_buildings.append(c)

sun_base = [[0, 0], [4300, 0], [4300, 2700], [3600, 3600], [2700, 4300], [0, 4300]]
sun_ramps = [[[1450, 4300], 420], [[4300, 1450], 420], [[3600, 3600], 420]]

river = [[200, 13300], [2600, 11400], [5600, 10000], [8192, 8192], [10784, 6384], [13784, 4984], [16184, 3084]]

sun_paths = [
    [[[1450, 6200], [3800, 7800], [5200, 8500], [6000, 9700]], 190],
    [[[5800, 5800], [8000, 5600], [9600, 7300]], 190],
    [[[4600, 1450], [6700, 3800], [8800, 3600], [10900, 3300], [12300, 5300]], 190],
    [[[8800, 1400], [8800, 3600], [8000, 5600]], 190],
    [[[3800, 7800], [3000, 9300], [2600, 11400]], 170],
]
camps_sun = [
    {"at": [3800, 7800], "groups": [["hound", "hound", "hound"], ["mossback", "mossback"]]},
    {"at": [6700, 3800], "groups": [["imp", "imp", "imp", "imp"], ["hound", "hound", "hound"]]},
    {"at": [8800, 3600], "groups": [["golem", "golem_small", "golem_small"], ["shaman", "hornet", "hornet"]]},
    {"at": [10900, 3300], "groups": [["mossback", "mossback"], ["imp", "imp", "imp", "imp"]]},
    {"at": [8000, 5600], "groups": [["golem", "golem_small", "golem_small"], ["shaman", "hornet", "hornet"]]},
    {"at": [5200, 8500], "groups": [["drake", "drake_young", "drake_young"]], "ancient": True},
]
camps = camps_sun + [dict(c, at=m(c["at"])) for c in camps_sun]

runes = [[4600, 10470], m([4600, 10470])]
boss = [10350, 7350]
shops = [
    ["home0", [1450, 850]], ["home1", m([1450, 850])],
    ["side", [2050, 10700]], ["side", m([2050, 10700])],
    ["secret", [4300, 9100]], ["secret", m([4300, 9100])],
]
clearings = [[r, 320] for r in runes] + [[boss, 520]] + [[s[1], 260] for s in shops]
paths = sun_paths + [[ml(l), w] for l, w in sun_paths] + [[[boss, [9900, 6900]], 200]]

sides = [
    {
        "fountain": [650, 650], "spawn": [1150, 1150], "lanes": sun_lanes, "buildings": sun_buildings,
        "creep_spawn": {"top": [1650, 3000], "mid": [3000, 3000], "bot": [3000, 1650]},
        "creeps": ["creep_melee_sun", "creep_ranged_sun", "creep_siege_sun"], "base": sun_base,
    },
    {
        "fountain": m([650, 650]), "spawn": m([1150, 1150]), "lanes": moon_lanes, "buildings": moon_buildings,
        "creep_spawn": {"bot": m([1650, 3000]), "mid": m([3000, 3000]), "top": m([3000, 1650])},
        "creeps": ["creep_melee_moon", "creep_ranged_moon", "creep_siege_moon"], "base": ml(sun_base),
    },
]

out = {
    "size": S, "tree_cell": 128, "path_cell": 64, "seed": 6681,
    "sides": sides, "river": river, "river_width": 360, "paths": paths, "clearings": clearings,
    "lane_width": 420, "ramps": sun_ramps + [[m(c), r] for c, r in sun_ramps],
    "camps": camps, "runes": runes, "boss": boss, "shops": shops, "gaps": 10,
}
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
with open(os.path.join(root, "data", "map.json"), "w") as f:
    json.dump(out, f, indent=1)
print("wrote data/map.json")
