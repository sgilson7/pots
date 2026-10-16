#!/usr/bin/env python3
"""Writes data/models.json from the round-2 mockups (T1, T3).

    (cd journal/mockups && ../../.venv/bin/python shoot.py export)
    python3 tools/modelport.py

The mockups in journal/mockups/ are the source of every model's look; the
exporter writes each model's rest pose to tools/art2_models.json, and this
script turns those parts into the game's: a sphere, cone, cylinder or gem
becomes one part; a capsule becomes a cylinder and two spheres; a torus
arc becomes a chain of short cylinders with round ends. Each part keeps
its joint, so the game's poses move it, and its outline, glow and team
colour. Metres become hundredths of a scene unit at 200 a metre (a hero
stands 300 tall, as in round 1), and no model is taller than its round-1
self, so buildings and trees keep their place on the map.

The palette, the team roles and the effect colours already in
data/models.json are kept; every new colour is added as `c<hex>`, and a
part whose colour differs between the Sunward and Moonward builds becomes
a team role `t<sunhex><moonhex>`.
"""
import json, math, pathlib, sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
SRC = ROOT / "tools" / "art2_models.json"
DST = ROOT / "data" / "models.json"
K = 200  # hundredths of a scene unit per mockup metre
TAPER = 0.8  # the taper mesh's top radius over its bottom (crates/core/src/mesh.rs)
GLOW = 4
# heads tip back this many degrees, so a face shows from the game's 56-degree camera (SECOND-ORDER R2.10)
HEAD_TILT = 12
# the map's colours in the round-2 palette (journal/mockups/kit.js C.grass, C.dirt, C.water)
GROUND = {"grass_sun": [143, 209, 111], "grass_moon": [126, 196, 140], "lane": [217, 176, 124], "path": [200, 154, 102],
          "water": [127, 211, 240], "stone_sun": [233, 207, 158], "stone_moon": [150, 156, 184],
          # ability colours (journal/mockups/kit.js C: frost, fire, gold, rot, earth, arcane pink, storm)
          "fx_frost": [120, 205, 255], "fx_fire": [255, 123, 58], "fx_gold": [245, 196, 81], "fx_rot": [143, 191, 90],
          "fx_earth": [127, 227, 194], "fx_hex": [255, 122, 217], "fx_storm": [154, 216, 255], "fx_holy": [255, 242, 122]}  # the glow style, drawn unlit (data/shaders/scene.frag via uGlow)

# round 1's heights (tools/modelgen.py at 67ed47e): no model grows past them
ROUND1 = {"tower_sun": 600, "tower_moon": 600, "barracks_melee_sun": 320, "barracks_ranged_sun": 320, "barracks_melee_moon": 320,
          "barracks_ranged_moon": 320, "sphere_sun": 700, "sphere_moon": 700, "fountain_sun": 300, "fountain_moon": 300, "shop": 260,
          "tree_sun": 520, "tree_moon": 480, "warden": 460, "drake": 320, "golem": 320}


def mat(e):
    """three.js column-major elements -> row lists."""
    return [[e[c * 4 + r] for c in range(4)] for r in range(4)]


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(4)) for j in range(4)] for i in range(4)]


def T(x, y, z):
    return [[1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1]]


def S(x, y, z):
    return [[x, 0, 0, 0], [0, y, 0, 0], [0, 0, z, 0], [0, 0, 0, 1]]


def rot_to(axis):
    """A rotation taking +y to the unit vector `axis`."""
    x, y, z = axis
    if y > 0.999999:
        return T(0, 0, 0)
    if y < -0.999999:
        return [[1, 0, 0, 0], [0, -1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]
    # axis-angle about (z, 0, -x) normalised
    ax, az = z, -x
    n = math.hypot(ax, az)
    ax, az = ax / n, az / n
    c, s = y, math.sqrt(max(0, 1 - y * y))
    t = 1 - c
    return [[t * ax * ax + c, -s * az, t * ax * az, 0],
            [s * az, c, -s * ax, 0],
            [t * ax * az, s * ax, t * az * az + c, 0],
            [0, 0, 0, 1]]


# the mockups face +z; the game faces +x: turn 90 degrees about y
TURN = [[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]]


def decompose(m):
    """Translation, YXZ Euler degrees and scale of an affine with no shear;
    also the worst error of rebuilding it, as a check."""
    cols = [[m[r][c] for r in range(3)] for c in range(3)]
    sc = [math.sqrt(sum(v * v for v in col)) for col in cols]
    R = [[cols[c][r] / sc[c] if sc[c] else 0 for c in range(3)] for r in range(3)]
    det = (R[0][0] * (R[1][1] * R[2][2] - R[1][2] * R[2][1]) - R[0][1] * (R[1][0] * R[2][2] - R[1][2] * R[2][0])
           + R[0][2] * (R[1][0] * R[2][1] - R[1][1] * R[2][0]))
    if det < 0:
        sc[0] = -sc[0]
        R = [[-R[r][0], R[r][1], R[r][2]] for r in range(3)]
    # R = Ry * Rx * Rz  (three.js order 'YXZ')
    x = math.asin(max(-1, min(1, -R[1][2])))
    if abs(R[1][2]) < 0.9999999:
        y = math.atan2(R[0][2], R[2][2])
        z = math.atan2(R[1][0], R[1][1])
    else:
        y = math.atan2(-R[2][0], R[0][0])
        z = 0
    deg = [round(math.degrees(v)) for v in (x, y, z)]
    # rebuild with whole degrees to measure the shear and rounding left
    rx, ry, rz = (math.radians(d) for d in deg)
    Ry = [[math.cos(ry), 0, math.sin(ry)], [0, 1, 0], [-math.sin(ry), 0, math.cos(ry)]]
    Rx = [[1, 0, 0], [0, math.cos(rx), -math.sin(rx)], [0, math.sin(rx), math.cos(rx)]]
    Rz = [[math.cos(rz), -math.sin(rz), 0], [math.sin(rz), math.cos(rz), 0], [0, 0, 1]]
    m3 = lambda a, b: [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    Rb = m3(m3(Ry, Rx), Rz)
    err = max(abs(Rb[r][c] * sc[c] - m[r][c]) for r in range(3) for c in range(3))
    return [m[0][3], m[1][3], m[2][3]], deg, sc, err


def pieces(p):
    """A mockup part as (game mesh, local matrix on a unit game mesh) pairs."""
    g, q = p["geo"], p["params"]
    if g == "SphereGeometry":
        r = q.get("radius", 1)
        return [("sphere", S(2 * r, 2 * r, 2 * r))]
    if g == "IcosahedronGeometry":
        r = q.get("radius", 1)
        return [("blob", S(2 * r, 2 * r, 2 * r))]
    if g == "OctahedronGeometry":
        r = q.get("radius", 1)
        return [("octa", S(2 * r, 2 * r, 2 * r))]
    if g == "ConeGeometry":
        r, h = q.get("radius", 1), q.get("height", 1)
        return [("cone", S(2 * r, h, 2 * r))]
    if g == "CylinderGeometry":
        rt, rb, h = q["radiusTop"], q["radiusBottom"], q["height"]
        if rt < 1e-6:
            return [("cone", S(2 * rb, h, 2 * rb))]
        if abs(rt - rb) < 1e-6:
            return [("cyl", S(2 * rt, h, 2 * rt))]
        ratio = rt / rb
        if ratio < 1 and abs(ratio - TAPER) < 0.12:
            return [("taper", S(2 * rb, h, 2 * rb))]
        if ratio > 1 and abs(1 / ratio - TAPER) < 0.12:
            return [("taper", mul([[1, 0, 0, 0], [0, -1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]], S(2 * rt, h, 2 * rt)))]
        r = (rt + rb) / 2
        return [("cyl", S(2 * r, h, 2 * r))]
    if g == "CapsuleGeometry":
        r, L = q["radius"], q["length"]
        out = [("sphere", mul(T(0, L / 2, 0), S(2 * r, 2 * r, 2 * r))), ("sphere", mul(T(0, -L / 2, 0), S(2 * r, 2 * r, 2 * r)))]
        if L > 1e-3:
            out.insert(0, ("cyl", S(2 * r, L, 2 * r)))
        return out
    if g == "TorusGeometry":
        R, r, arc = q["radius"], q["tube"], q.get("arc", 2 * math.pi)
        n = max(2, math.ceil(arc / math.radians(16)))
        out = []
        da = arc / n
        chord = 2 * R * math.sin(da / 2)
        for i in range(n):
            am = (i + 0.5) * da
            pos = (R * math.cos(am), R * math.sin(am), 0)
            tan = (-math.sin(am), math.cos(am), 0)
            out.append(("cyl", mul(mul(T(*pos), rot_to(tan)), S(2 * r, chord * 1.06, 2 * r))))
        if arc < 2 * math.pi - 1e-3:
            for a in (0, arc):
                out.append(("sphere", mul(T(R * math.cos(a), R * math.sin(a), 0), S(2 * r, 2 * r, 2 * r))))
        return out
    return []


def hexrgb(h):
    return [int(h[1:3], 16), int(h[3:5], 16), int(h[5:7], 16)]


def main():
    src = json.loads(SRC.read_text())
    old = json.loads(DST.read_text())
    palette, teams = dict(old["palette"]), [dict(t) for t in old["teams"]]
    palette.update(GROUND)
    models, worst, skipped = {}, 0.0, 0
    for mid, both in sorted(src.items()):
        sun, moon = both["sun"], both["moon"]
        same = len(sun["parts"]) == len(moon["parts"]) and all(a["geo"] == b["geo"] for a, b in zip(sun["parts"], moon["parts"]))
        # every part placed in game space, in metres, before the model's scale
        built = []
        for i, p in enumerate(sun["parts"]):
            color = "c" + p["color"][1:]
            if same and moon["parts"][i]["color"] != p["color"]:
                role = "t" + p["color"][1:] + moon["parts"][i]["color"][1:]
                if role not in teams[0]:
                    teams[0][role] = hexrgb(p["color"])
                    teams[1][role] = hexrgb(moon["parts"][i]["color"])
                    teams[2][role] = hexrgb(p["color"])
                color = role
            else:
                palette.setdefault(color, hexrgb(p["color"]))
            M = mul(TURN, mat(p["m"]))
            if p["joint"] == "head" and "head" in sun["pivots"]:
                hp = mul(TURN, T(*sun["pivots"]["head"]))
                px, py = hp[0][3], hp[1][3]
                c, s_ = math.cos(math.radians(HEAD_TILT)), math.sin(math.radians(HEAD_TILT))
                tilt = [[c, -s_, 0, 0], [s_, c, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
                M = mul(mul(mul(T(px, py, 0), tilt), T(-px, -py, 0)), M)
            ps = pieces(p)
            if not ps:
                skipped += 1
            for mesh, local in ps:
                built.append((mesh, mul(M, local), color, p))
        top = max(m[1][3] + abs(m[1][1]) / 2 + abs(m[1][0]) / 2 + abs(m[1][2]) / 2 for _, m, _, _ in built)
        k = K
        if mid in ROUND1 and top * K > ROUND1[mid]:
            k = ROUND1[mid] / top
        parts = []
        for mesh, m, color, p in built:
            at, deg, sc, err = decompose(m)
            worst = max(worst, err / max(1e-6, max(abs(v) for v in sc)))
            joint = p["joint"]
            if mid.startswith("rune_") or (mid.startswith("sphere_") and p["geo"] == "TorusGeometry"):
                joint = "spin"
            d = {"mesh": mesh, "at": [round(v * k) for v in at], "size": [max(1, round(abs(v) * k)) for v in sc], "color": color}
            if any(deg):
                d["rot"] = deg
            if joint != "body":
                d["joint"] = joint
            if p["glow"]:
                d["style"] = GLOW
            if not p["ink"] or p["glow"]:
                d["plain"] = True
            parts.append(d)
        pivots = {}
        for name, v in sun["pivots"].items():
            g = mul(TURN, T(*v))
            pivots[name] = [round(g[0][3] * k), round(g[1][3] * k), round(g[2][3] * k)]
        models[mid] = {"height": round(top * k), "parts": parts, "pivots": {j: pivots[j] for j in sorted(pivots) if j != "body"},
                       "_note": "round 2: from journal/mockups (tools/modelport.py)"}
    missing = sorted(set(old["models"]) - set(models))
    if missing:
        sys.exit(f"models in data/models.json with no mockup: {missing}")
    out = {"palette": dict(sorted(palette.items())), "teams": [dict(sorted(t.items())) for t in teams], "models": models}
    DST.write_text(json.dumps(out, indent=1) + "\n")
    n = sum(len(m["parts"]) for m in models.values())
    print(f"wrote {len(models)} models, {n} parts; worst rebuild error {worst:.4f}; skipped {skipped} parts")


if __name__ == "__main__":
    main()
