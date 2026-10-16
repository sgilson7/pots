#!/usr/bin/env python3
"""Writes data/models.json: every model in the game as posed parts.

Run: python3 tools/modelgen.py

Units are hundredths of a model unit; a model's feet stand at y = 0 and it
faces +x. Joints: body, head, arm_l, arm_r (the weapon arm), leg_l, leg_r,
fixed (does not move with the pose), spin (turns slowly). The scene scales
every model by data/presentation.json's unit_scale.

Every look here is original: no hero, creep or building copies a Warcraft
III or DotA model. Each hero's silhouette is chosen so it reads at a glance
from the raised camera: a giant blade, a tall hood, a long rifle, a hook.
"""
import json, os

def P(mesh, at, size, color, joint="body", rot=(0, 0, 0), style=0, plain=False):
    d = {"mesh": mesh, "at": list(at), "size": list(size), "color": color}
    if joint != "body":
        d["joint"] = joint
    if any(rot):
        d["rot"] = list(rot)
    if style:
        d["style"] = style
    if plain:
        d["plain"] = True
    return d

def humanoid(h=250, w=100, skin="skin_tan", torso="cloth_red", legs="leather", arms=None,
             head="sphere", head_size=None, boots=None, belt=None, shoulders=None):
    """A standing figure: two legs from the hip, a torso, two arms from the
    shoulders, a head. Returns (parts, pivots)."""
    arms = arms or torso
    boots = boots or legs
    hip = h * 42 // 100
    shoulder = h * 76 // 100
    leg_w = w * 26 // 100
    parts = []
    for side, z in (("leg_l", -w * 18 // 100), ("leg_r", w * 18 // 100)):
        parts.append(P("cube", (0, hip // 2 + 6, z), (leg_w, hip - 8, leg_w), legs, side))
        parts.append(P("cube", (6, 8, z), (leg_w + 10, 16, leg_w + 4), boots, side))
    torso_h = shoulder - hip + h * 6 // 100
    parts.append(P("cube", (0, hip + torso_h // 2 - 4, 0), (w * 46 // 100, torso_h, w * 64 // 100), torso))
    if belt:
        parts.append(P("cube", (0, hip + 4, 0), (w * 50 // 100, 14, w * 68 // 100), belt))
    arm_len = h * 34 // 100
    for side, z in (("arm_l", -w * 42 // 100), ("arm_r", w * 42 // 100)):
        parts.append(P("cube", (0, shoulder - arm_len // 2, z), (w * 18 // 100, arm_len, w * 18 // 100), arms, side))
        parts.append(P("sphere", (0, shoulder - arm_len, z), (w * 22 // 100, w * 22 // 100, w * 22 // 100), skin, side))
    if shoulders:
        for z in (-w * 40 // 100, w * 40 // 100):
            parts.append(P("sphere", (0, shoulder + 4, z), (w * 36 // 100, w * 24 // 100, w * 36 // 100), shoulders))
    hs = head_size or (h * 22 // 100)
    parts.append(P(head, (0, shoulder + hs // 2 + 10, 0), (hs, hs, hs), skin, "head"))
    pivots = {
        "leg_l": [0, hip, -w * 18 // 100], "leg_r": [0, hip, w * 18 // 100],
        "arm_l": [0, shoulder, -w * 42 // 100], "arm_r": [0, shoulder, w * 42 // 100],
        "head": [0, shoulder + 8, 0],
    }
    return parts, pivots, shoulder, hs

def hand_r(w, shoulder, h):
    """Where the weapon hand is, hanging down, for parts on the arm_r joint."""
    return shoulder - h * 34 // 100, w * 42 // 100

M = {}

def model(name, height, parts, pivots=None, spin=0, note=""):
    M[name] = {"height": height, "parts": parts, "pivots": pivots or {}, "spin": spin, "note": note}

# ---------------------------------------------------------------- heroes
# 1 Ironvow: a knight in heavy plate with a blade taller than he is.
p, pv, sh, hs = humanoid(h=260, w=120, skin="steel", torso="steel", legs="iron", arms="steel",
                         head="cube", shoulders="team", belt="leather")
y, z = hand_r(120, sh, 260)
p += [
    P("cube", (24, y + 40, z + 6), (14, 200, 34), "steel_bright", "arm_r", rot=(0, 0, 0)),
    P("cube", (24, y - 54, z + 6), (20, 14, 70), "gold", "arm_r"),
    P("cube", (24, y - 72, z + 6), (12, 26, 12), "leather", "arm_r"),
    P("cone", (0, sh + hs + 34, 0), (14, 40, 60), "team", "head"),
    P("cube", (-30, sh - 60, 0), (10, 150, 100), "team", style=0),
    P("cube", (8, sh + hs // 2 + 6, 0), (6, 10, hs - 10), "shadow", "head"),
]
model("hero_ironvow", 300, p, pv, note="after Sven: a plate knight with a greatsword and a crested helm")

# 2 Rime: a frost hermit in a long robe with a hood and a crystal staff.
p, pv, sh, hs = humanoid(h=250, w=96, skin="skin_light", torso="cloth_white", legs="cloth_white",
                         arms="cloth_blue", belt="frost")
y, z = hand_r(96, sh, 250)
p += [
    P("cone", (0, 70, 0), (110, 150, 120), "cloth_white"),
    P("cone", (0, sh + hs + 20, 0), (hs + 30, 90, hs + 30), "cloth_blue", "head"),
    P("cyl", (14, y + 60, z), (10, 240, 10), "wood", "arm_r"),
    P("octa", (14, y + 190, z), (34, 50, 34), "ice", "arm_r", style=4),
    P("sphere", (0, sh - 6, 0), (70, 26, 80), "fur"),
]
model("hero_rime", 300, p, pv, note="after Crystal Maiden: a hooded frost hermit with a crystal staff")

# 3 Cinder: a slim fire dancer with flame hair and a trailing sash.
p, pv, sh, hs = humanoid(h=245, w=86, skin="skin_tan", torso="cloth_red", legs="cloth_red",
                         arms="skin_tan", belt="gold")
p += [
    P("cone", (-6, sh + hs + 30, 0), (hs + 10, 70, hs + 10), "fire", "head", style=4),
    P("cone", (-26, sh + hs + 6, 0), (30, 60, 30), "ember", "head", rot=(0, 0, 40), style=4),
    P("cube", (-36, 90, 0), (8, 110, 50), "fire", rot=(0, 0, -15)),
    P("sphere", (6, sh - 70 - 34, 86 * 42 // 100), (34, 34, 34), "fire", "arm_r", style=4),
    P("sphere", (6, sh - 70 - 34, -86 * 42 // 100), (34, 34, 34), "fire", "arm_l", style=4),
]
model("hero_cinder", 300, p, pv, note="after Lina: a fire dancer with burning hands and flame hair")

# 4 Thistle: a hooded archer with a tall recurve bow and a quiver.
p, pv, sh, hs = humanoid(h=250, w=90, skin="skin_light", torso="cloth_green", legs="leather",
                         arms="cloth_green", belt="leather")
y, z = hand_r(90, sh, 250)
p += [
    P("cone", (0, sh + hs + 10, 0), (hs + 26, 70, hs + 26), "cloth_green", "head"),
    P("cube", (20, y + 30, z + 10), (8, 200, 8), "wood", "arm_r", rot=(0, 0, 8)),
    P("cube", (28, y + 120, z + 10), (8, 50, 8), "wood", "arm_r", rot=(0, 0, -30)),
    P("cube", (28, y - 60, z + 10), (8, 50, 8), "wood", "arm_r", rot=(0, 0, 30)),
    P("cyl", (-30, sh - 30, -14), (30, 100, 30), "leather", rot=(0, 0, 20)),
    P("cone", (-30, sh + 28, -14), (12, 30, 12), "frost", rot=(0, 0, 20)),
    P("cube", (-24, 110, 0), (8, 120, 70), "cloth_green"),
]
model("hero_thistle", 300, p, pv, note="after Drow Ranger: a hooded archer with frost-tipped arrows")

# 5 Gristle: a huge stitched butcher with a hook on a chain and a cleaver.
p, pv, sh, hs = humanoid(h=270, w=170, skin="skin_pale", torso="skin_pale", legs="leather",
                         arms="skin_pale", belt="leather", head_size=60)
y, z = hand_r(170, sh, 270)
p += [
    P("sphere", (8, 150, 0), (160, 170, 180), "skin_pale"),
    P("cube", (60, 150, 0), (6, 120, 8), "blood"),
    P("cube", (60, 160, 0), (6, 8, 70), "blood"),
    P("cube", (30, y + 10, z + 8), (14, 90, 50), "steel", "arm_r"),
    P("cube", (30, y - 30, z + 8), (14, 30, 16), "leather", "arm_r"),
    P("cyl", (14, sh - 120, -170 * 42 // 100), (8, 90, 8), "iron", "arm_l"),
    P("ring", (24, sh - 170, -170 * 42 // 100), (40, 10, 40), "steel_bright", "arm_l", rot=(90, 0, 0)),
    P("cube", (0, 60, 0), (150, 40, 170), "cloth_brown"),
]
model("hero_gristle", 300, p, pv, note="after Pudge: a stitched butcher with a hook and a cleaver")

# 6 Tectus: a stone ox-man with horns and a totem club.
p, pv, sh, hs = humanoid(h=265, w=150, skin="stone", torso="moss", legs="stone", arms="stone",
                         belt="wood", head="cube", head_size=64, shoulders="stone")
y, z = hand_r(150, sh, 265)
p += [
    P("cone", (0, sh + 70, -50), (18, 60, 18), "bone", "head", rot=(-50, 0, 0)),
    P("cone", (0, sh + 70, 50), (18, 60, 18), "bone", "head", rot=(50, 0, 0)),
    P("cube", (24, sh + 44, 0), (30, 26, 40), "stone", "head"),
    P("cyl", (24, y + 50, z), (20, 200, 20), "wood", "arm_r"),
    P("cube", (24, y + 150, z), (60, 70, 60), "stone", "arm_r", style=0),
    P("sphere", (24, y + 150, z), (34, 34, 70), "team", "arm_r"),
]
model("hero_tectus", 310, p, pv, note="after Earthshaker: a horned stone ox-man with a totem club")

# 7 Kestrel: a masked swordsman with a wide mask and a long curved blade.
p, pv, sh, hs = humanoid(h=250, w=100, skin="skin_tan", torso="cloth_orange", legs="cloth_brown",
                         arms="skin_tan", belt="team")
y, z = hand_r(100, sh, 250)
p += [
    P("cube", (10, sh + hs // 2 + 10, 0), (30, hs + 30, hs + 40), "mask", "head"),
    P("cube", (24, sh + hs // 2 + 18, 0), (6, 8, hs + 10), "shadow", "head"),
    P("cone", (-10, sh + hs + 40, 0), (10, 80, 10), "team", "head", rot=(0, 0, -30)),
    P("cube", (26, y + 70, z + 4), (8, 170, 14), "steel_bright", "arm_r", rot=(0, 0, -6)),
    P("cube", (26, y - 20, z + 4), (12, 30, 12), "leather", "arm_r"),
    P("cone", (0, 70, 0), (120, 100, 120), "cloth_brown"),
]
model("hero_kestrel", 300, p, pv, note="after Juggernaut: a masked bladedancer with a curved sword")

# 8 Hexwright: a gaunt warlock with spiked shoulders and a clawed staff.
p, pv, sh, hs = humanoid(h=255, w=88, skin="skin_grey", torso="cloth_purple", legs="cloth_purple",
                         arms="cloth_purple", belt="bone")
y, z = hand_r(88, sh, 255)
p += [
    P("cone", (0, sh + 30, -48), (26, 70, 26), "bone", rot=(-30, 0, 0)),
    P("cone", (0, sh + 30, 48), (26, 70, 26), "bone", rot=(30, 0, 0)),
    P("cone", (0, sh + hs + 30, 0), (hs, 60, hs), "cloth_purple", "head"),
    P("cyl", (14, y + 60, z), (10, 230, 10), "bone", "arm_r"),
    P("cone", (14, y + 190, z - 14), (12, 50, 12), "arcane", "arm_r", rot=(-30, 0, 0), style=4),
    P("cone", (14, y + 190, z + 14), (12, 50, 12), "arcane", "arm_r", rot=(30, 0, 0), style=4),
    P("cone", (0, 70, 0), (100, 130, 110), "cloth_purple"),
    P("sphere", (24, sh + hs // 2 + 14, -12), (8, 8, 8), "arcane", "head", style=4),
    P("sphere", (24, sh + hs // 2 + 14, 12), (8, 8, 8), "arcane", "head", style=4),
]
model("hero_hexwright", 300, p, pv, note="after Lion: a gaunt warlock with a clawed staff")

# 9 Longwick: a small gnome marksman with a rifle twice his height and goggles.
p, pv, sh, hs = humanoid(h=190, w=100, skin="skin_light", torso="cloth_brown", legs="leather",
                         arms="cloth_brown", belt="leather", head_size=62)
y, z = hand_r(100, sh, 190)
p += [
    P("cyl", (0, sh + hs + 20, 0), (hs + 40, 14, hs + 40), "leather", "head"),
    P("cyl", (0, sh + hs + 40, 0), (hs - 6, 40, hs - 6), "leather", "head"),
    P("cube", (24, sh + hs // 2 + 18, 0), (10, 16, hs), "brass", "head"),
    P("cube", (60, y + 30, z), (240, 14, 14), "iron", "arm_r"),
    P("cube", (-30, y + 30, z), (60, 22, 16), "wood", "arm_r"),
    P("cyl", (120, y + 50, z), (10, 30, 10), "brass", "arm_r", rot=(0, 0, 90)),
    P("sphere", (30, sh + 10, 0), (40, 30, 60), "beard"),
]
model("hero_longwick", 240, p, pv, note="after Sniper: a gnome marksman with a long rifle")

# 10 Tempest: an old storm caller with a cloud crown and lightning in both hands.
p, pv, sh, hs = humanoid(h=255, w=100, skin="skin_light", torso="cloth_blue", legs="cloth_white",
                         arms="cloth_white", belt="gold", head_size=58)
y, z = hand_r(100, sh, 255)
p += [
    P("blob", (0, sh + hs + 24, 0), (hs + 40, 40, hs + 40), "cloud", "head"),
    P("blob", (-10, sh + hs + 44, 10), (hs + 10, 34, hs + 10), "cloud", "head"),
    P("cone", (26, sh + 8, 0), (40, 70, 50), "beard", "head", rot=(0, 0, 180)),
    P("octa", (10, y - 10, z), (30, 50, 30), "lightning", "arm_r", style=4),
    P("octa", (10, y - 10, -z), (30, 50, 30), "lightning", "arm_l", style=4),
    P("cone", (0, 70, 0), (120, 140, 120), "cloth_blue"),
    P("cube", (-30, sh - 40, 0), (8, 120, 90), "cloth_white"),
]
model("hero_tempest", 300, p, pv, note="after Zeus: an old storm caller with a cloud crown")

# ---------------------------------------------------------------- creeps
p, pv, sh, hs = humanoid(h=200, w=100, skin="skin_tan", torso="team", legs="leather", arms="team", belt="leather", head="cube")
y, z = hand_r(100, sh, 200)
p += [P("cyl", (20, y + 40, z), (8, 180, 8), "wood", "arm_r"), P("cone", (20, y + 140, z), (14, 30, 14), "steel", "arm_r"),
      P("cyl", (14, sh - 60, -100 * 42 // 100 - 12), (60, 10, 60), "team", "arm_l", rot=(90, 0, 0)), P("cone", (0, sh + 70, 0), (50, 30, 50), "steel", "head")]
model("creep_melee_sun", 230, p, pv, note="Sunward spearman with a round shield")
p, pv, sh, hs = humanoid(h=190, w=90, skin="skin_tan", torso="team", legs="leather", arms="cloth_white", head="sphere")
y, z = hand_r(90, sh, 190)
p += [P("cyl", (14, y + 10, z), (40, 6, 40), "leather", "arm_r", rot=(90, 0, 0)), P("cone", (0, sh + 60, 0), (60, 40, 60), "team", "head")]
model("creep_ranged_sun", 220, p, pv, note="Sunward slinger in a pointed cap")
model("creep_siege_sun", 170, [
    P("cube", (0, 50, 0), (200, 40, 110), "wood"), P("cyl", (60, 30, -64), (60, 12, 60), "wood", rot=(90, 0, 0)),
    P("cyl", (60, 30, 64), (60, 12, 60), "wood", rot=(90, 0, 0)), P("cyl", (-60, 30, -64), (60, 12, 60), "wood", rot=(90, 0, 0)),
    P("cyl", (-60, 30, 64), (60, 12, 60), "wood", rot=(90, 0, 0)), P("cube", (0, 110, 0), (40, 100, 40), "wood", "arm_r", rot=(0, 0, -30)),
    P("cube", (10, 140, 0), (120, 16, 16), "team", "arm_r"), P("sphere", (60, 150, 0), (30, 30, 30), "stone", "arm_r")],
    {"arm_r": [0, 70, 0]}, note="Sunward ballista on four wheels")
p, pv, sh, hs = humanoid(h=200, w=110, skin="skin_grey", torso="team", legs="cloth_purple", arms="skin_grey", head="cube")
y, z = hand_r(110, sh, 200)
p += [P("cone", (24, y - 10, z), (14, 50, 14), "bone", "arm_r", rot=(0, 0, -90)), P("cone", (24, sh - 90, -110 * 42 // 100), (14, 50, 14), "bone", "arm_l", rot=(0, 0, -90)),
      P("cone", (0, sh + 70, 0), (40, 40, 40), "team", "head")]
model("creep_melee_moon", 230, p, pv, note="Moonward thrall with bone claws")
p, pv, sh, hs = humanoid(h=190, w=86, skin="skin_grey", torso="team", legs="cloth_purple", arms="team", head="sphere")
y, z = hand_r(86, sh, 190)
p += [P("octa", (18, y - 10, z), (30, 30, 30), "arcane", "arm_r", style=4), P("cone", (0, sh + 70, 0), (56, 70, 56), "team", "head")]
model("creep_ranged_moon", 230, p, pv, note="Moonward caster holding a violet spark")
model("creep_siege_moon", 170, [
    P("cube", (0, 50, 0), (200, 40, 110), "stone_dark"), P("cyl", (60, 30, -64), (60, 12, 60), "stone_dark", rot=(90, 0, 0)),
    P("cyl", (60, 30, 64), (60, 12, 60), "stone_dark", rot=(90, 0, 0)), P("cyl", (-60, 30, -64), (60, 12, 60), "stone_dark", rot=(90, 0, 0)),
    P("cyl", (-60, 30, 64), (60, 12, 60), "stone_dark", rot=(90, 0, 0)), P("cube", (-20, 110, 0), (30, 90, 30), "bone", "arm_r", rot=(0, 0, 40)),
    P("sphere", (20, 160, 0), (50, 40, 50), "team", "arm_r")],
    {"arm_r": [-20, 70, 0]}, note="Moonward catapult of bone and slate")

# ---------------------------------------------------------------- buildings
def tower(team_stone, crown):
    return [
        P("cube", (0, 30, 0), (170, 60, 170), team_stone, "fixed"),
        P("cyl", (0, 230, 0), (100, 340, 100), team_stone),
        P("cyl", (0, 410, 0), (140, 30, 140), "team"),
        P("cone", (0, 470, 0), (120, 110, 120), team_stone),
        P(crown, (0, 560, 0), (70, 70, 70), "team_glow", "spin", style=4),
        P("cube", (60, 200, 0), (10, 100, 50), "team"),
    ]
model("tower_sun", 600, tower("stone_sun_build", "sphere"), note="sandstone spire with a sun-gold orb")
model("tower_moon", 600, tower("stone_moon_build", "octa"), note="slate spire with a violet crystal")

def barracks(stone, melee):
    p = [P("cube", (0, 80, 0), (300, 160, 220), stone), P("wedge", (0, 200, 0), (300, 80, 220), "team", rot=(0, 90, 0)),
         P("cube", (151, 60, 0), (6, 110, 70), "shadow")]
    if melee:
        p += [P("cube", (0, 260, 0), (20, 120, 20), "wood"), P("cube", (0, 300, 30), (8, 60, 60), "team")]
    else:
        p += [P("cyl", (90, 210, 70), (60, 120, 60), stone), P("cone", (90, 300, 70), (70, 60, 70), "team")]
    return p
model("barracks_melee_sun", 320, barracks("stone_sun_build", True), note="Sunward barracks for footmen")
model("barracks_ranged_sun", 320, barracks("stone_sun_build", False), note="Sunward barracks for slingers")
model("barracks_melee_moon", 320, barracks("stone_moon_build", True), note="Moonward barracks for thralls")
model("barracks_ranged_moon", 320, barracks("stone_moon_build", False), note="Moonward barracks for casters")

def sphere(stone):
    return [
        P("cyl", (0, 40, 0), (520, 80, 520), stone, "fixed"),
        P("cyl", (0, 120, 0), (380, 80, 380), stone, "fixed"),
        P("cyl", (0, 230, 0), (120, 140, 120), stone, "fixed"),
        P("sphere", (0, 480, 0), (380, 380, 380), "team_glow", "spin", style=4),
        P("ring", (0, 480, 0), (480, 30, 480), "team", "spin", rot=(20, 0, 0)),
        P("ring", (0, 480, 0), (460, 30, 460), "team", "spin", rot=(-20, 0, 30)),
    ]
model("sphere_sun", 700, sphere("stone_sun_build"), note="the Sunward Sphere: a gold orb in two rings")
model("sphere_moon", 700, sphere("stone_moon_build"), note="the Moonward Sphere: a violet orb in two rings")
for t, s in (("sun", "stone_sun_build"), ("moon", "stone_moon_build")):
    model(f"fountain_{t}", 300, [
        P("cyl", (0, 30, 0), (420, 60, 420), s, "fixed"), P("cyl", (0, 62, 0), (360, 8, 360), "water", "fixed", plain=True),
        P("cyl", (0, 160, 0), (60, 220, 60), s), P("sphere", (0, 290, 0), (90, 90, 90), "team_glow", "spin", style=4)],
        note="a fountain that heals its side")
model("shop", 260, [P("cube", (0, 60, 0), (200, 120, 160), "wood"), P("wedge", (0, 160, 0), (220, 80, 180), "cloth_red", rot=(0, 90, 0)),
                    P("cube", (90, 80, 0), (20, 30, 120), "gold")], note="a market stall")

# ---------------------------------------------------------------- neutrals and the boss
def beast(body, h, w, color, head_color=None, horns=False, tail=True):
    head_color = head_color or color
    hip = h * 45 // 100
    p = [P("sphere", (0, hip + h // 6, 0), (w, h * 45 // 100, w * 60 // 100), color),
         P("sphere", (w * 55 // 100, hip + h // 4, 0), (w * 40 // 100, w * 40 // 100, w * 40 // 100), head_color, "head")]
    for side, x, z in (("leg_l", w // 3, -w // 5), ("leg_r", w // 3, w // 5), ("leg_r", -w // 3, -w // 5), ("leg_l", -w // 3, w // 5)):
        p.append(P("cube", (x, hip // 2, z), (w // 8, hip, w // 8), color, side))
    if horns:
        p += [P("cone", (w * 60 // 100, hip + h // 3, -w // 8), (10, 40, 10), "bone", "head", rot=(-30, 0, 0)),
              P("cone", (w * 60 // 100, hip + h // 3, w // 8), (10, 40, 10), "bone", "head", rot=(30, 0, 0))]
    if tail:
        p.append(P("cone", (-w * 55 // 100, hip + h // 6, 0), (16, 60, 16), color, rot=(0, 0, 100)))
    piv = {"leg_l": [0, hip, 0], "leg_r": [0, hip, 0], "head": [w * 40 // 100, hip + h // 5, 0]}
    return p, piv

p, pv = beast("hound", 150, 160, "fur_grey", horns=False)
model("hound", 170, p, pv, note="a ridge hound")
p, pv = beast("mossback", 110, 180, "moss", "stone", tail=False)
p.append(P("sphere", (0, 110, 0), (190, 90, 150), "stone_dark"))
model("mossback", 170, p, pv, note="a moss-shelled tortoise")
p, pv, sh, hs = humanoid(h=140, w=80, skin="skin_green", torso="leather", legs="leather", arms="skin_green", head_size=46)
p.append(P("cone", (0, sh + 50, 0), (40, 40, 40), "cloth_red", "head"))
model("imp", 170, p, pv, note="a bramble imp")
p, pv, sh, hs = humanoid(h=280, w=170, skin="stone", torso="stone", legs="stone", arms="stone", head="cube", head_size=50, shoulders="stone_dark")
model("golem", 320, p, pv, note="a stone golem")
p, pv, sh, hs = humanoid(h=180, w=120, skin="stone", torso="stone", legs="stone", arms="stone", head="cube", head_size=40)
model("golem_small", 220, p, pv, note="a small stone golem")
p, pv, sh, hs = humanoid(h=220, w=100, skin="skin_grey", torso="cloth_green", legs="fur", arms="skin_grey", head_size=50)
p += [P("cone", (0, sh + 70, -20), (10, 50, 10), "bone", "head", rot=(-20, 0, 0)), P("cone", (0, sh + 70, 20), (10, 50, 10), "bone", "head", rot=(20, 0, 0)),
      P("octa", (20, sh - 90, 42), (24, 24, 24), "poison", "arm_r", style=4)]
model("shaman", 260, p, pv, note="a horned hornet shaman")
model("hornet", 160, [P("sphere", (0, 120, 0), (90, 60, 60), "gold"), P("sphere", (-50, 110, 0), (60, 50, 50), "shadow"),
                      P("card", (0, 160, -40), (60, 4, 70), "wing", "arm_l", rot=(90, 0, 0), plain=True),
                      P("card", (0, 160, 40), (60, 4, 70), "wing", "arm_r", rot=(90, 0, 0), plain=True), P("cone", (-90, 110, 0), (10, 30, 10), "shadow", rot=(0, 0, 90))],
      {"arm_l": [0, 150, 0], "arm_r": [0, 150, 0]}, note="a giant hornet")
p, pv = beast("drake", 260, 300, "drake", horns=True)
p += [P("card", (0, 240, -150), (160, 4, 200), "drake_wing", "arm_l", rot=(80, 0, 0)), P("card", (0, 240, 150), (160, 4, 200), "drake_wing", "arm_r", rot=(-80, 0, 0))]
pv.update({"arm_l": [0, 220, -40], "arm_r": [0, 220, 40]})
model("drake", 320, p, pv, note="an elder drake: the ancient camp")
p, pv = beast("drake_young", 180, 200, "drake", horns=True)
model("drake_young", 220, p, pv, note="a young drake")
p, pv = beast("warden", 420, 380, "warden", "warden_head", horns=True)
p += [P("cube", (0, 320, 0), (260, 60, 200), "stone_dark"), P("cone", (-40, 360, 0), (40, 90, 40), "bone"), P("cone", (60, 360, 0), (40, 90, 40), "bone")]
model("warden", 460, p, pv, note="the Warden: the boss in the river pit (after Roshan)")
model("critter", 80, [P("sphere", (0, 40, 0), (70, 60, 60), "critter"), P("sphere", (36, 60, 0), (40, 36, 36), "critter", "head"),
                      P("cube", (20, 10, -16), (10, 20, 10), "critter", "leg_l"), P("cube", (20, 10, 16), (10, 20, 10), "critter", "leg_r")],
      {"leg_l": [20, 20, 0], "leg_r": [20, 20, 0], "head": [30, 50, 0]}, note="what a hex turns a unit into")

p, pv = beast("courier", 110, 120, "fur", "fur", horns=False)
p.append(P("cube", (-10, 120, 0), (60, 40, 70), "team"))
model("courier", 150, p, pv, note="the team courier: a pack mule")

# ---------------------------------------------------------------- wards, summons, scenery
model("observer_ward", 160, [P("cyl", (0, 60, 0), (10, 120, 10), "wood"), P("sphere", (0, 140, 0), (40, 40, 40), "eye", style=4)], note="an observer ward")
model("sentry_ward", 160, [P("cyl", (0, 60, 0), (10, 120, 10), "wood"), P("octa", (0, 140, 0), (40, 50, 40), "frost", style=4)], note="a sentry ward")
model("healing_ward", 200, [P("cyl", (0, 50, 0), (30, 100, 30), "wood"), P("sphere", (0, 120, 0), (50, 50, 50), "heal", "spin", style=4),
                            P("ring", (0, 120, 0), (80, 8, 80), "heal", "spin", rot=(30, 0, 0))], note="a healing totem")
model("fissure_rock", 120, [P("cone", (0, 50, 0), (90, 120, 90), "stone_dark"), P("cone", (20, 40, 30), (60, 80, 60), "stone")], note="a rock thrown up by the fissure")
model("tree_sun", 400, [P("cyl", (0, 60, 0), (24, 120, 24), "wood", "fixed"), P("cone", (0, 170, 0), (120, 160, 120), "leaf_sun"),
                        P("cone", (0, 260, 0), (90, 140, 90), "leaf_sun_light")], note="a Sunward pine")
model("tree_moon", 400, [P("cyl", (0, 60, 0), (24, 120, 24), "wood_dark", "fixed"), P("blob", (0, 180, 0), (130, 120, 130), "leaf_moon"),
                         P("blob", (10, 250, 0), (90, 90, 90), "leaf_moon_light")], note="a Moonward frost tree")
for k, c in (("haste", "rune_haste"), ("double", "rune_double"), ("illusion", "rune_illusion"), ("invis", "rune_invis"), ("regen", "rune_regen")):
    model(f"rune_{k}", 150, [P("octa", (0, 90, 0), (60, 90, 60), c, "spin", style=4), P("ring", (0, 20, 0), (110, 6, 110), c, "fixed", plain=True)],
          note=f"the {k} rune")

palette = {
    "skin_light": [236, 200, 170], "skin_tan": [200, 150, 110], "skin_pale": [214, 196, 176], "skin_grey": [150, 160, 170],
    "skin_red": [176, 72, 56], "skin_green": [110, 150, 80], "steel": [150, 160, 172], "steel_bright": [210, 220, 232],
    "iron": [90, 94, 104], "gold": [226, 180, 60], "brass": [196, 150, 70], "leather": [120, 80, 50], "wood": [120, 84, 52],
    "wood_dark": [70, 56, 52], "cloth_red": [180, 50, 50], "cloth_blue": [70, 110, 190], "cloth_white": [232, 236, 240],
    "cloth_green": [70, 120, 70], "cloth_purple": [100, 60, 140], "cloth_orange": [220, 120, 40], "cloth_brown": [110, 80, 60],
    "stone": [140, 136, 126], "stone_dark": [86, 84, 92], "moss": [96, 130, 70], "fire": [250, 120, 40], "ember": [255, 200, 80],
    "frost": [140, 210, 240], "ice": [190, 235, 255], "bone": [230, 220, 190], "fur": [210, 205, 195], "fur_grey": [130, 126, 120],
    "shadow": [30, 26, 34], "blood": [140, 30, 40], "mask": [235, 225, 200], "arcane": [190, 90, 255], "poison": [120, 220, 80],
    "beard": [200, 120, 60], "wing": [220, 230, 240], "drake": [80, 60, 70], "drake_wing": [120, 50, 60], "warden": [96, 78, 64],
    "warden_head": [120, 96, 80], "critter": [240, 200, 210], "cloud": [220, 226, 240], "lightning": [150, 220, 255], "eye": [255, 220, 90], "heal": [120, 240, 140],
    "stone_sun_build": [214, 190, 150], "stone_moon_build": [96, 96, 120],
    "leaf_sun": [60, 120, 56], "leaf_sun_light": [96, 150, 70], "leaf_moon": [70, 90, 120], "leaf_moon_light": [120, 150, 180],
    "grass_sun": [92, 128, 64], "grass_moon": [80, 92, 104], "lane": [156, 134, 98], "water": [60, 110, 150],
    "stone_sun": [176, 162, 132], "stone_moon": [104, 104, 120], "path": [120, 112, 84],
    "rune_haste": [255, 80, 60], "rune_double": [80, 140, 255], "rune_illusion": [255, 230, 90], "rune_invis": [200, 200, 230],
    "rune_regen": [90, 230, 110],
    "bar_back": [20, 20, 24], "bar_ally": [70, 210, 90], "bar_enemy": [220, 60, 50], "bar_neutral": [230, 200, 60], "bar_mana": [70, 120, 240],
    "select": [255, 255, 255], "shot": [255, 230, 160], "illusion": [120, 170, 255],
    "fx_frost": [140, 210, 255], "fx_fire": [255, 120, 30], "fx_earth": [170, 130, 80], "fx_blood": [170, 30, 40], "fx_rot": [120, 200, 60],
    "fx_holy": [255, 230, 150], "fx_arcane": [190, 100, 255], "fx_steel": [220, 220, 230], "fx_heal": [120, 240, 140], "fx_shadow": [60, 40, 80],
}
teams = [
    {"team": [230, 160, 40], "team_glow": [255, 210, 90], "ring": [255, 190, 60]},
    {"team": [140, 110, 230], "team_glow": [180, 150, 255], "ring": [160, 130, 255]},
    {"team": [150, 150, 150], "team_glow": [200, 200, 200], "ring": [230, 210, 120]},
]
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
with open(os.path.join(root, "data", "models.json"), "w") as f:
    json.dump({"palette": palette, "teams": teams, "models": M}, f, indent=0, separators=(",", ":"))
print("wrote data/models.json with", len(M), "models")
