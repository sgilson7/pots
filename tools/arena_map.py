"""Writes the arena (round 8, after SMITE 2's Arena) into data/map.json's
"arena": a round field on the map's diagonal with a base at each end, two
lanes round the middle, and a small jungle on each side with three camps.

Points are worked out along the diagonal from the Sunward base (south-west)
to the Moonward (north-east): t along it, s across it (positive to the
north-west). The Moonward side is the Sunward's mirror through the centre,
as on the main map.

    python3 tools/arena_map.py
"""
import json
import math
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
MAP = ROOT / "data" / "map.json"

SIZE = 8192
C = SIZE / 2
R2 = math.sqrt(0.5)


def pt(t, s):
    """The map point t along the diagonal and s across it."""
    return [round(C + t * R2 - s * R2), round(C + t * R2 + s * R2)]


def mirror(p):
    return [SIZE - p[0], SIZE - p[1]]


# the field: round clearings down the diagonal, widest in the middle
HALF = 3300
field = []
for t in range(-2600, 2601, 325):
    r = round(1750 * math.sqrt(max(0.0, 1 - (t / HALF) ** 2)))
    field.append([pt(t, 0), max(r, 900)])
# each base's floor
bases = [[pt(-3500, 0), 1050]]
# the side jungles: three camps in a pocket on each side, a path through
# the pocket, and two ways in from the field
POCKET = 2550
camps_at = [(-1150, "red"), (0, "purple"), (1150, "blue")]
pockets = []
paths = []
for side in (1, -1):
    for t, _ in camps_at:
        pockets.append([pt(side * t, side * POCKET), 540])
    paths.append([[pt(side * -1500, side * POCKET), pt(side * 1500, side * POCKET)], 240])
    for t in (-1150, 1150):
        paths.append([[pt(side * t, side * 1500), pt(side * t, side * (POCKET - 300))], 260])

GROUPS = {
    "red": [["golem", "golem_small", "golem_small"]],
    "purple": [["shaman", "hornet", "hornet"]],
    "blue": [["mossback", "mossback"]],
}
camps = []
for side in (1, -1):
    for t, buff in camps_at:
        camps.append({"at": pt(side * t, side * POCKET), "groups": GROUPS[buff], "buff": buff})


def lane(s):
    """A Sunward lane round the middle on side s, to the Moonward portal."""
    return [pt(-3050, 550 * s), pt(-2650, 650 * s), pt(-1500, 1150 * s), pt(0, 1300 * s), pt(1500, 1150 * s),
            pt(2650, 650 * s), pt(3500, 0)]


def side(team):
    sfx = "sun" if team == 0 else "moon"
    f = (lambda p: p) if team == 0 else mirror
    top, bot = lane(1), lane(-1)
    if team == 1:
        # the Moonward lanes are the Sunward's turned round: its top lane is
        # the one on the north-west, like the Sunward's
        top, bot = [mirror(p) for p in lane(-1)], [mirror(p) for p in lane(1)]
    base = [pt(-5200, -1500), pt(-2950, -1500), pt(-2650, 0), pt(-2950, 1500), pt(-5200, 1500)]
    return {
        "fountain": f(pt(-4300, 0)),
        "spawn": f(pt(-3850, 0)),
        "lanes": {"top": top, "bot": bot},
        "buildings": [
            {"unit": f"sphere_{sfx}", "at": f(pt(-3500, 0)), "lane": "base", "facing": 45 if team == 0 else 225},
            {"unit": f"fountain_{sfx}", "at": f(pt(-4300, 0)), "lane": "base", "facing": 45 if team == 0 else 225},
        ],
        "creep_spawn": {"top": f(pt(-3050, 500 if team == 0 else -500)), "bot": f(pt(-3050, -500 if team == 0 else 500))},
        "creeps": [f"creep_melee_{sfx}", f"creep_ranged_{sfx}", f"creep_siege_{sfx}"],
        "base": [f(p) for p in base],
    }


arena = {
    "size": SIZE,
    "tree_cell": 128,
    "path_cell": 64,
    "seed": 8801,
    "sides": [side(0), side(1)],
    "river": [],
    "river_width": 0,
    "paths": paths,
    "clearings": field + bases + [[mirror(c[0]), c[1]] for c in bases] + pockets,
    "lane_width": 420,
    "ramps": [[pt(-2750, 700), 380], [pt(-2750, -700), 380], [mirror(pt(-2750, 700)), 380], [mirror(pt(-2750, -700)), 380]],
    "camps": camps,
    "runes": [],
    "boss": [round(C), round(C)],
    # within the shop's reach (600) of where heroes appear, as on the main map
    "shops": [["home0", pt(-3900, 420)], ["home1", mirror(pt(-3900, 420))]],
    "gaps": 6,
}

data = json.loads(MAP.read_text())
data["arena"] = arena
MAP.write_text(json.dumps(data, indent=1) + "\n")
print("wrote the arena:", len(arena["clearings"]), "clearings,", len(camps), "camps")
