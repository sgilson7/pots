"""Ability icons: web/art/abilities/<ability id>.png, one round badge per
ability in its hero's element (tools/art/icons.py), with one subject that
shows what the ability does."""
import math

from icons import M, ability_fig, ability_ids

THIN = M.THIN


# ---- local helpers ---------------------------------------------------------------

def arc_pts(x, y, r, a0, a1, step=6):
    n = max(2, int(abs(a1 - a0) / step))
    return [(x + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)),
             y + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def crescent(fig, x, y, r0, r1, a0, a1, fill, step=6):
    """A filled band between radii r0 and r1 from angle a0 to a1."""
    outer = arc_pts(x, y, r1, a0, a1, step)
    inner = arc_pts(x, y, r0, a1, a0, step)
    fig.poly(outer + inner, fill)


def slash(fig, x0, y0, x1, y1, w=0.07, fill="snow"):
    """A thin pointed slash mark from (x0, y0) to (x1, y1)."""
    dx, dy = x1 - x0, y1 - y0
    L = math.hypot(dx, dy)
    nx, ny = -dy / L * w, dx / L * w
    mx, my = (x0 + x1) / 2, (y0 + y1) / 2
    fig.poly([(x0, y0), (mx + nx, my + ny), (x1, y1), (mx - nx, my - ny)], fill)


def spike(fig, x, y, w, h, fill="stone", lean=0.0):
    fig.poly([(x - w / 2, y), (x + lean, y + h), (x + w / 2, y)], fill)


def beam(fig, x0, y0, x1, y1, w, fill):
    dx, dy = x1 - x0, y1 - y0
    L = math.hypot(dx, dy)
    nx, ny = -dy / L * w, dx / L * w
    fig.poly([(x0 + nx * 0.3, y0 + ny * 0.3), (x1 + nx, y1 + ny), (x1 - nx, y1 - ny), (x0 - nx * 0.3, y0 - ny * 0.3)], fill)


def fist(fig, x, y, s=1.0, fill="skin", rot=0.0):
    """A clenched fist, knuckles up, wrist at the bottom."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.2, -0.55, 0.2, -0.15, fill, r=0.06)
        fig.rect(-0.34, -0.25, 0.32, 0.22, fill, r=0.12)
        for i in range(4):
            fig.rect(-0.34 + i * 0.165, 0.08, -0.34 + (i + 1) * 0.165, 0.34, fill, r=0.07)
        fig.blob([(0.18, -0.2), (0.42, -0.1), (0.4, 0.08), (0.18, 0.04)], fill, tension=0.4)


def zig(fig, pts, fill="spark", w=0.06):
    """A jagged lightning ribbon along pts."""
    left, right = [], []
    for i, (px, py) in enumerate(pts):
        qx, qy = pts[min(i + 1, len(pts) - 1)]
        rx, ry = pts[max(i - 1, 0)]
        dx, dy = qx - rx, qy - ry
        L = math.hypot(dx, dy) or 1
        ww = w * (1 - 0.6 * i / (len(pts) - 1))
        left.append((px - dy / L * ww, py + dx / L * ww))
        right.append((px + dy / L * ww, py - dx / L * ww))
    fig.poly(left + right[::-1], fill)


def totem(fig, x, y, s=1.0, wood="wood", face="wooddark", top=None):
    with fig.frame(x, y, s=s):
        fig.rect(-0.2, -0.55, 0.2, 0.3, wood, r=0.05)
        fig.rect(-0.26, -0.6, 0.26, -0.48, face, r=0.03)
        fig.ellipse(-0.08, 0.08, 0.05, 0.06, "ink")
        fig.ellipse(0.08, 0.08, 0.05, 0.06, "ink")
        fig.rect(-0.1, -0.16, 0.1, -0.06, "ink")
        fig.stroke([(-0.2, -0.3), (0.2, -0.3)], width=THIN)
        if top:
            top(fig)


def cleaver(fig, x, y, s=1.0, rot=0.0):
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.06, -0.62, 0.06, -0.12, "wooddark", r=0.03)
        fig.poly([(-0.14, -0.14), (0.4, -0.14), (0.44, 0.5), (-0.14, 0.42)], "steel")
        fig.circle(0.28, 0.32, 0.05, "shade")
        fig.stroke([(-0.08, -0.08), (0.36, -0.08)], width=THIN, role="steeldark")


def rifle(fig, x, y, s=1.0, rot=0.0):
    """A long rifle pointing right."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.poly([(-0.62, -0.14), (-0.62, 0.02), (-0.2, 0.06), (-0.16, -0.04), (-0.36, -0.16)], "wood")
        fig.rect(-0.22, -0.02, 0.66, 0.06, "iron")
        fig.rect(-0.12, 0.06, 0.18, 0.14, "steeldark", r=0.02)
        fig.rect(0.6, -0.04, 0.68, 0.08, "golddark")


def target(fig, x, y, r):
    fig.circle(x, y, r, "snow")
    fig.circle(x, y, r * 0.68, "life")
    fig.circle(x, y, r * 0.38, "snow")
    fig.circle(x, y, r * 0.14, "life")


# ---- Ironvow (steel) -------------------------------------------------------------

def storm_bolt(rng):
    fig = ability_fig("storm_bolt")
    M.motion(fig, [((0.3, 1.2), (0.58, 1.06)), ((0.28, 0.92), (0.56, 0.84)), ((0.38, 0.62), (0.62, 0.64))], width=2.6)
    M.hammer(fig, 1.08, 0.86, 1.45, rot=-50)
    M.stun_stars(fig, 1.24, 1.54, 0.75)
    return fig


def great_cleave(rng):
    fig = ability_fig("great_cleave")
    crescent(fig, 0.98, 0.82, 0.48, 0.66, 20, 165, "snow")
    for a in (40, 90, 140):
        x, y = 0.98 + 0.66 * math.cos(math.radians(a)), 0.82 + 0.66 * math.sin(math.radians(a))
        M.burst(fig, x, y, 0.13, "life", n=7)
    M.sword(fig, 1.0, 0.98, 1.05, rot=-35, length=1.15)
    return fig


def warcry(rng):
    fig = ability_fig("warcry")
    for r in (0.3, 0.46, 0.62):
        M.ripple(fig, 1.02, 1.1, r, r, width=3.0, a0=-45, a1=45)
    # a curved war horn, mouth to the right
    fig.blob([(0.3, 0.74), (0.5, 0.66), (0.76, 0.76), (0.98, 0.9), (1.08, 0.82), (1.12, 1.1), (1.06, 1.38),
              (0.98, 1.3), (0.74, 1.1), (0.5, 0.92), (0.34, 0.86)], "bone", tension=0.4)
    fig.ellipse(1.06, 1.1, 0.1, 0.28, "golddark")
    fig.ellipse(1.07, 1.1, 0.05, 0.18, "shade")
    for x in (0.52, 0.74):
        fig.stroke([(x, 0.7 + (x - 0.52) * 0.4), (x + 0.02, 0.93 + (x - 0.52) * 0.8)], width=2.2, role="gold")
    M.shield(fig, 0.66, 0.5, 0.5)
    return fig


def gods_strength(rng):
    fig = ability_fig("gods_strength")
    M.flame(fig, 0.66, 0.38, 0.95, outer="ember", inner="flame", core=False)
    M.flame(fig, 1.34, 0.38, 0.95, outer="ember", inner="flame", core=False)
    M.flame(fig, 1.0, 0.36, 1.3)
    fist(fig, 1.0, 1.0, 1.15, fill="steel")
    M.sparkle(fig, 1.42, 1.44, 0.14)
    return fig


# ---- Rime (frost) ------------------------------------------------------------------

def crystal_nova(rng):
    fig = ability_fig("crystal_nova")
    for i in range(8):
        a = 90 + 45 * i
        x, y = 1 + 0.42 * math.cos(math.radians(a)), 1 + 0.42 * math.sin(math.radians(a))
        M.crystal(fig, x, y, 0.62, "ice", rot=a - 90)
    M.burst(fig, 1, 1, 0.3, "snow", n=8, inner=0.5)
    fig.circle(1, 1, 0.1, "icedeep")
    return fig


def frostbite(rng):
    fig = ability_fig("frostbite")
    fig.poly([(0.52, 0.42), (1.48, 0.42), (1.56, 1.2), (1.3, 1.6), (0.72, 1.62), (0.44, 1.2)], "ice")
    M.chain(fig, [(0.48, 0.92), (1.52, 0.92)], fill="iron", r=0.1)
    fig.circle(1.0, 1.08, 0.2, "steeldark")
    fig.rect(0.94, 0.86, 1.06, 1.02, "iron")
    fig.poly([(0.72, 1.62), (0.86, 1.62), (0.6, 1.3)], "snow")
    M.sparkle(fig, 1.32, 1.36, 0.12)
    return fig


def brilliance_aura(rng):
    fig = ability_fig("brilliance_aura")
    M.burst(fig, 1, 1, 0.7, "snow", n=12, inner=0.7)
    M.orb(fig, 1, 1, 0.42, "mana")
    M.sparkle(fig, 1.0, 1.0, 0.2, "ice")
    return fig


def freezing_field(rng):
    fig = ability_fig("freezing_field")
    M.ripple(fig, 1, 0.7, 0.6, 0.18, role="icedeep", width=3.0, a0=0, a1=360)
    fig.ellipse(1, 0.7, 0.18, 0.08, "snow")
    for i, (x, y, s) in enumerate(((0.56, 1.16, 0.55), (0.9, 1.38, 0.6), (1.26, 1.24, 0.6), (1.5, 0.98, 0.5),
                                   (0.66, 0.82, 0.45), (1.2, 0.74, 0.45))):
        M.crystal(fig, x, y, s, "ice", rot=200 + rng.uniform(-8, 8))
    return fig


# ---- Cinder (fire) ----------------------------------------------------------------

def dragon_slave(rng):
    fig = ability_fig("dragon_slave")
    fig.blob([(0.3, 0.62), (0.6, 0.52), (1.0, 0.48), (1.48, 0.5), (1.66, 0.72), (1.6, 1.08), (1.42, 1.44),
              (1.3, 1.14), (1.14, 1.32), (1.0, 1.04), (0.84, 1.18), (0.7, 0.92), (0.5, 0.92)], "ember", tension=0.45)
    fig.blob([(0.5, 0.66), (1.0, 0.6), (1.46, 0.62), (1.52, 0.86), (1.4, 1.12), (1.26, 0.92), (1.1, 1.04),
              (0.96, 0.84), (0.76, 0.86)], "flame", tension=0.45)
    fig.blob([(0.96, 0.7), (1.38, 0.7), (1.36, 0.86), (1.2, 0.8), (1.06, 0.86)], "flamehot", tension=0.45)
    M.motion(fig, [((0.26, 0.98), (0.52, 1.04)), ((0.3, 1.2), (0.6, 1.3))], width=2.4)
    return fig


def light_strike_array(rng):
    fig = ability_fig("light_strike_array")
    fig.ellipse(1, 0.56, 0.6, 0.2, "ember")
    fig.ellipse(1, 0.56, 0.42, 0.12, "flamehot")
    fig.poly([(0.74, 0.58), (0.8, 1.66), (1.2, 1.66), (1.26, 0.58)], "flame")
    fig.poly([(0.88, 0.6), (0.92, 1.66), (1.08, 1.66), (1.12, 0.6)], "flamehot")
    M.stun_stars(fig, 1.0, 1.36, 0.7)
    return fig


def fiery_soul(rng):
    fig = ability_fig("fiery_soul")
    M.flame(fig, 0.62, 0.64, 0.55)
    M.flame(fig, 1.38, 0.64, 0.55)
    M.flame(fig, 1.0, 0.3, 1.25)
    M.motion(fig, [((0.4, 1.36), (0.66, 1.46)), ((1.34, 1.46), (1.6, 1.36))], width=2.4)
    return fig


def laguna_blade(rng):
    fig = ability_fig("laguna_blade")
    beam(fig, 1.5, 1.6, 0.78, 0.6, 0.24, "ember")
    beam(fig, 1.46, 1.56, 0.8, 0.62, 0.12, "flamehot")
    M.burst(fig, 0.74, 0.54, 0.34, "flame", n=9)
    M.burst(fig, 0.74, 0.54, 0.18, "flamehot", n=7)
    return fig


# ---- Thistle (frost) --------------------------------------------------------------

def frost_arrows(rng):
    fig = ability_fig("frost_arrows")
    M.arrow(fig, 0.36, 0.42, 1.52, 1.58, fletch="snow", head="ice")
    M.crystal(fig, 1.5, 1.5, 0.48, "ice", rot=-45)
    M.snowflake(fig, 0.62, 1.34, 0.18, role="ice", width=2.0)
    M.snowflake(fig, 1.38, 0.64, 0.14, role="ice", width=2.0)
    return fig


def silence(rng):
    fig = ability_fig("silence")
    # a closed mouth with a finger across it: hush
    fig.blob([(0.52, 1.0), (0.78, 1.16), (1.0, 1.1), (1.22, 1.16), (1.48, 1.0), (1.22, 0.86), (0.78, 0.86)], "life", tension=0.4)
    fig.stroke([(0.54, 1.0), (1.46, 1.0)], width=2.0)
    fig.rect(0.9, 0.42, 1.1, 1.36, "skin", r=0.1)
    fig.stroke([(0.92, 1.18), (1.08, 1.18)], width=THIN)
    for r in (0.66,):
        M.ripple(fig, 1, 1, r, r, width=2.4, a0=110, a1=160)
        M.ripple(fig, 1, 1, r, r, width=2.4, a0=20, a1=70)
    fig.stroke([(0.5, 1.5), (1.5, 0.5)], width=3.4, role="ink")
    return fig


def trueshot_aura(rng):
    fig = ability_fig("trueshot_aura")
    M.eye(fig, 1.0, 1.14, 1.3, iris="leaf")
    M.arrow(fig, 0.3, 0.6, 1.64, 0.6, fletch="snow")
    return fig


def marksmanship(rng):
    fig = ability_fig("marksmanship")
    M.bow(fig, 1.08, 1.0, 1.35, wood="wood")
    M.arrow(fig, 0.42, 1.0, 1.66, 1.0, fletch="leaf")
    fig.stroke([(0.92, 1.76), (0.46, 1.0), (0.92, 0.24)], width=1.4)
    return fig


# ---- Gristle (rot) ----------------------------------------------------------------

def meat_hook(rng):
    fig = ability_fig("meat_hook")
    M.chain(fig, [(0.36, 1.58), (0.62, 1.4), (0.9, 1.26)], fill="steeldark", r=0.1)
    M.hook(fig, 1.1, 0.96, 1.5, rot=-25)
    return fig


def rot(rng):
    fig = ability_fig("rot")
    for (x, y, s) in ((0.66, 0.6, 0.8), (1.34, 0.62, 0.8), (1.0, 0.86, 1.2)):
        M.cloud(fig, x, y, s, "venom")
    for i in range(5):
        x = 0.5 + i * 0.25 + rng.uniform(-0.03, 0.03)
        M.swirl(fig, x, 1.34 + rng.uniform(-0.05, 0.08), 0.1, role="rot", width=2.0, turns=1.2)
    fig.circle(0.84, 1.08, 0.05, "rot")
    fig.circle(1.2, 1.0, 0.06, "rot")
    return fig


def flesh_heap(rng):
    fig = ability_fig("flesh_heap")
    fig.blob([(0.4, 0.5), (0.36, 0.86), (0.6, 1.24), (0.9, 1.5), (1.2, 1.44), (1.5, 1.1), (1.64, 0.7), (1.58, 0.5)],
             "skinpale", tension=0.5)
    fig.stroke([(0.62, 1.2), (0.9, 0.9), (1.0, 0.56)], width=2.0, role="blood", smooth=True)
    for t in range(4):
        x, y = 0.66 + t * 0.11, 1.15 - t * 0.17
        fig.stroke([(x - 0.07, y - 0.05), (x + 0.07, y + 0.05)], width=1.6)
    fig.stroke([(1.24, 1.3), (1.4, 0.8)], width=2.0, role="blood")
    for t in range(3):
        x, y = 1.27 + t * 0.06, 1.2 - t * 0.17
        fig.stroke([(x - 0.07, y), (x + 0.07, y + 0.02)], width=1.6)
    M.shield(fig, 1.38, 0.66, 0.52, fill="arcane!40!paper", trim="arcane", boss=None)
    return fig


def dismember(rng):
    fig = ability_fig("dismember")
    fig.blob([(0.36, 0.5), (0.5, 0.8), (0.86, 0.9), (1.12, 0.72), (1.1, 0.46), (0.7, 0.36)], "life", tension=0.5)
    fig.ellipse(0.98, 0.64, 0.12, 0.1, "bone")
    fig.stroke([(0.6, 0.48), (0.66, 0.74)], width=1.6, role="snow")
    cleaver(fig, 1.24, 1.12, 1.05, rot=-30)
    for x in (0.6, 0.82):
        M.drop(fig, x, 1.12, 0.22, "blood")
    return fig


# ---- Tectus (earth) ----------------------------------------------------------------

def fissure(rng):
    fig = ability_fig("fissure")
    fig.poly([(0.3, 0.5), (1.7, 0.86), (1.62, 1.08), (0.32, 0.76)], "stonedark")
    pts = [(0.38, 0.62)]
    for i in range(1, 7):
        t = i / 6
        pts.append((0.38 + 1.22 * t, 0.62 + 0.34 * t + (0.07 if i % 2 else -0.06)))
    M.cracks(fig, pts, width=3.6, role="ember")
    for i, (x, y, s) in enumerate(((0.56, 0.96, 0.42), (0.98, 1.12, 0.52), (1.4, 1.3, 0.42), (1.12, 0.52, 0.34))):
        M.rock(fig, x, y, s, "stone", rng=rng)
    return fig


def enchant_totem(rng):
    fig = ability_fig("enchant_totem")
    M.burst(fig, 1.0, 1.26, 0.5, "flamehot", n=10, inner=0.7)
    with fig.frame(1.0, 1.0, s=1.0, rot=-20):
        fig.rect(-0.06, -0.62, 0.06, 0.1, "wood", r=0.03)
        fig.rect(-0.26, 0.04, 0.26, 0.5, "stone", r=0.06)
        fig.blob([(-0.26, 0.4), (-0.46, 0.5), (-0.52, 0.72), (-0.4, 0.58), (-0.24, 0.5)], "bone", tension=0.4)
        fig.blob([(0.26, 0.4), (0.46, 0.5), (0.52, 0.72), (0.4, 0.58), (0.24, 0.5)], "bone", tension=0.4)
        fig.stroke([(-0.2, 0.2), (0.2, 0.34)], width=THIN)
    return fig


def aftershock(rng):
    fig = ability_fig("aftershock")
    for r in (0.7, 0.5, 0.3):
        M.ripple(fig, 1, 0.74, r, r * 0.38, width=3.0, a0=0, a1=360)
    fig.ellipse(1, 0.74, 0.12, 0.05, "stonedark")
    for x, y, s in ((0.6, 1.22, 0.5), (1.4, 1.26, 0.44)):
        M.rock(fig, x, y, s, "stone", rng=rng)
    M.stun_stars(fig, 1.0, 1.5, 0.6)
    return fig


def echo_slam(rng):
    fig = ability_fig("echo_slam")
    for r in (0.66, 0.5, 0.34):
        M.ripple(fig, 1, 0.84, r, r, role="stonedark", width=3.4, a0=0, a1=360)
    M.burst(fig, 1, 0.66, 0.3, "flamehot", n=8)
    fist(fig, 1.0, 1.04, 0.9, fill="stone", rot=180)
    return fig


# ---- Kestrel (blood) -------------------------------------------------------------

def blade_fury(rng):
    fig = ability_fig("blade_fury")
    for a in (10, 130, 250):
        crescent(fig, 1, 1, 0.52, 0.7, a, a + 85, "snow")
    M.swirl(fig, 1, 1, 0.36, role="blood", width=3.0, turns=1.2)
    M.sword(fig, 1.0, 1.0, 0.95, rot=-60, length=1.1)
    return fig


def healing_ward(rng):
    fig = ability_fig("healing_ward")
    M.ripple(fig, 1, 0.48, 0.56, 0.14, role="heal", width=3.0, a0=0, a1=360)
    totem(fig, 1.0, 0.98, 1.0)
    fig.blob([(0.8, 1.32), (1.0, 1.5), (1.2, 1.32)], "life", tension=0.5)
    M.plus(fig, 1.48, 1.36, 0.6, "heal")
    M.plus(fig, 0.5, 1.18, 0.4, "heal")
    return fig


def blade_dance(rng):
    fig = ability_fig("blade_dance")
    M.sword(fig, 0.92, 0.9, 1.15, rot=-40, length=1.1)
    M.burst(fig, 1.44, 1.42, 0.3, "flamehot", n=9, inner=0.45)
    M.burst(fig, 1.44, 1.42, 0.15, "snow", n=5, inner=0.45)
    return fig


def omnislash(rng):
    fig = ability_fig("omnislash")
    for (x0, y0, x1, y1) in ((0.36, 1.3, 1.5, 0.38), (0.6, 1.62, 1.66, 0.84), (0.34, 0.86, 1.02, 0.34)):
        slash(fig, x0, y0, x1, y1, 0.1, "snow")
    M.sword(fig, 1.0, 1.06, 0.8, rot=-50, length=1.0)
    M.sparkle(fig, 1.52, 1.44, 0.12, "snow")
    return fig


# ---- Hexwright (arcane) ----------------------------------------------------------

def earth_spike(rng):
    fig = ability_fig("earth_spike")
    fig.poly([(0.3, 0.44), (1.7, 0.7), (1.66, 0.8), (0.3, 0.56)], "stonedark")
    for i, (x, y, h) in enumerate(((0.48, 0.5, 0.42), (0.8, 0.56, 0.62), (1.12, 0.62, 0.86), (1.44, 0.68, 0.72))):
        spike(fig, x, y, 0.26, h, "bone" if i % 2 else "stone", lean=rng.uniform(-0.04, 0.06))
    M.stun_stars(fig, 1.22, 1.6, 0.5)
    return fig


def hex(rng):
    fig = ability_fig("hex")
    toad = "venom"
    fig.ellipse(1.0, 0.82, 0.56, 0.36, toad)
    fig.ellipse(0.56, 0.6, 0.18, 0.1, toad, rot=20)
    fig.ellipse(1.44, 0.6, 0.18, 0.1, toad, rot=-20)
    fig.ellipse(1.0, 0.72, 0.32, 0.18, "venom!40!paper")
    for x in (0.7, 1.3):
        fig.circle(x, 1.16, 0.17, toad)
        fig.circle(x, 1.18, 0.1, "snow")
        fig.circle(x + 0.02, 1.17, 0.05, "ink")
    fig.stroke([(0.74, 0.92), (1.0, 0.86), (1.26, 0.92)], width=1.8, smooth=True)
    M.swirl(fig, 1.0, 1.52, 0.16, role="arcane", width=2.4)
    M.sparkle(fig, 0.5, 1.46, 0.1, "violet")
    M.sparkle(fig, 1.52, 1.44, 0.1, "violet")
    return fig


def mana_drain(rng):
    fig = ability_fig("mana_drain")
    pts = []
    for i in range(13):
        t = i / 12
        pts.append((0.42 + 1.1 * t, 0.6 + 0.8 * t + 0.08 * math.sin(t * math.pi * 3)))
    M.trail(fig, pts, "mana", w0=0.14, w1=0.04)
    M.orb(fig, 1.5, 1.42, 0.2, "violet")
    M.orb(fig, 0.46, 0.58, 0.26, "mana")
    for t in (0.3, 0.55, 0.8):
        x, y = pts[int(t * 12)]
        fig.circle(x, y, 0.04, "snow")
    return fig


def finger_of_death(rng):
    fig = ability_fig("finger_of_death")
    beam(fig, 0.98, 1.08, 1.62, 0.5, 0.34, "life")
    beam(fig, 1.0, 1.06, 1.6, 0.52, 0.16, "violet!35!snow")
    M.burst(fig, 1.48, 0.6, 0.3, "life", n=8)
    M.burst(fig, 1.48, 0.6, 0.14, "snow", n=6)
    with fig.frame(0.66, 1.34, s=1.25, rot=-40):
        fig.rect(-0.2, -0.13, 0.14, 0.2, "skingrey", r=0.06)
        fig.poly([(0.1, 0.13), (0.42, 0.09), (0.52, 0.04), (0.42, -0.0), (0.1, 0.02)], "skingrey")
        fig.poly([(0.1, -0.13), (0.22, -0.2), (0.1, -0.24)], "bone")
        fig.rect(-0.36, -0.12, -0.18, 0.2, "arcane", r=0.03)
    return fig


# ---- Longwick (steel) -------------------------------------------------------------

def shrapnel(rng):
    fig = ability_fig("shrapnel")
    fig.ellipse(1, 0.5, 0.6, 0.16, "stonedark")
    for i in range(5):
        x = 0.5 + i * 0.25 + rng.uniform(-0.03, 0.03)
        y = 1.0 + (0.38 if i % 2 else 0.12) + rng.uniform(-0.04, 0.04)
        with fig.frame(x, y, s=1.0, rot=-20):
            fig.poly([(-0.05, 0.05), (0, 0.34), (0.05, 0.05)], "steel!50!paper")
            fig.circle(0, 0, 0.1, "iron")
    for x in (0.66, 1.04, 1.38):
        M.burst(fig, x, 0.56, 0.14, "flamehot", n=6)
    return fig


def headshot(rng):
    fig = ability_fig("headshot")
    M.helm(fig, 1.12, 0.84, 1.15, fill="steel", crest=None)
    M.burst(fig, 0.66, 1.06, 0.18, "flamehot", n=8)
    with fig.frame(0.52, 1.14, s=1.0, rot=-25):
        fig.ellipse(0, 0, 0.12, 0.07, "golddark")
    M.motion(fig, [((0.26, 1.26), (0.42, 1.2))], width=2.2)
    M.stun_stars(fig, 1.12, 1.52, 0.6)
    return fig


def take_aim(rng):
    fig = ability_fig("take_aim")
    with fig.frame(1.0, 1.0, s=1.0, rot=30):
        fig.rect(-0.66, -0.1, -0.18, 0.1, "golddark", r=0.03)
        fig.rect(-0.2, -0.13, 0.22, 0.13, "gold", r=0.03)
        fig.rect(0.2, -0.17, 0.58, 0.17, "golddark", r=0.03)
        fig.ellipse(0.6, 0, 0.06, 0.17, "icedeep")
        fig.stroke([(-0.2, 0.0), (0.2, 0.0)], width=THIN)
    M.motion(fig, [((1.56, 1.06), (1.7, 1.24)), ((1.36, 1.5), (1.5, 1.66))], width=2.4)
    return fig


def assassinate(rng):
    fig = ability_fig("assassinate")
    target(fig, 1.24, 1.2, 0.5)
    M.trail(fig, [(0.3, 0.34), (0.5, 0.56), (0.72, 0.76), (0.98, 0.98)], "snow", w0=0.01, w1=0.09)
    with fig.frame(1.06, 1.04, s=1.5, rot=42):
        fig.blob([(-0.1, -0.05), (0.06, -0.05), (0.13, 0), (0.06, 0.05), (-0.1, 0.05)], "golddark", tension=0.3)
    M.burst(fig, 1.24, 1.2, 0.12, "flamehot", n=7)
    return fig


# ---- Tempest (storm) ------------------------------------------------------------

def arc_lightning(rng):
    fig = ability_fig("arc_lightning")
    zig(fig, [(0.38, 1.46), (0.66, 1.24), (0.58, 1.12), (0.92, 0.92)], w=0.16)
    zig(fig, [(0.92, 0.92), (1.18, 1.18), (1.22, 1.02), (1.56, 1.26)], w=0.13)
    zig(fig, [(0.92, 0.92), (1.12, 0.7), (1.0, 0.62), (1.26, 0.4)], w=0.12)
    M.burst(fig, 0.92, 0.92, 0.2, "snow", n=7)
    return fig


def lightning_bolt(rng):
    fig = ability_fig("lightning_bolt")
    M.bolt(fig, 1.02, 0.84, 1.1, fill="spark")
    M.cloud(fig, 1.0, 1.2, 1.05, "night!30!cloud")
    M.stun_stars(fig, 1.0, 0.44, 0.55)
    return fig


def static_field(rng):
    fig = ability_fig("static_field")
    M.orb(fig, 1, 1, 0.4, "storm!60!mana")
    for i in range(6):
        a = math.radians(30 + 60 * i + rng.uniform(-6, 6))
        p0 = (1 + 0.42 * math.cos(a), 1 + 0.42 * math.sin(a))
        mid = (1 + 0.55 * math.cos(a + 0.2), 1 + 0.55 * math.sin(a + 0.2))
        p1 = (1 + 0.7 * math.cos(a - 0.05), 1 + 0.7 * math.sin(a - 0.05))
        fig.stroke([p0, mid, p1], width=4.6, role="ink")
        fig.stroke([p0, mid, p1], width=2.2, role="spark")
    M.bolt(fig, 1, 1, 0.55, "spark")
    return fig


def thundergods_wrath(rng):
    fig = ability_fig("thundergods_wrath")
    for x, s in ((0.58, 0.62), (1.42, 0.62), (1.0, 0.8)):
        M.bolt(fig, x, 0.74, s, "spark")
    M.cloud(fig, 1.0, 1.08, 1.1, "night!40!cloud")
    M.sparkle(fig, 1.3, 1.5, 0.08, "spark")
    return fig


# ---- the Warden (earth) -----------------------------------------------------------

def warden_bash(rng):
    fig = ability_fig("warden_bash")
    M.burst(fig, 0.82, 0.66, 0.3, "flamehot", n=8)
    M.mace(fig, 1.12, 1.0, 1.25, rot=40)
    M.stun_stars(fig, 0.86, 1.46, 0.65)
    return fig


# ---- Hackle (blood), round 7 ---------------------------------------------------

def saw_pts(x, y, r0, r1, teeth=12, rot=0.0):
    pts = []
    for i in range(teeth * 2):
        a = math.radians(rot + i * 180.0 / teeth)
        r = r1 if i % 2 == 0 else r0
        pts.append((x + r * math.cos(a), y + r * math.sin(a)))
    return pts


def berserker_rage(rng):
    fig = ability_fig("berserker_rage")
    M.burst(fig, 1.0, 1.0, 0.62, "ember", n=11)
    # an upturned tusk across the burst
    fig.blob([(0.56, 0.62), (0.7, 0.6), (1.0, 0.86), (1.32, 1.3), (1.4, 1.5), (1.26, 1.38), (0.92, 1.0), (0.62, 0.76)], "bone", tension=0.4)
    M.axe(fig, 1.32, 0.66, 0.7, rot=-35)
    return fig


def whirling_axes(rng):
    fig = ability_fig("whirling_axes")
    # five hatchets fanning out from the bottom, each with a short trail
    for ang in (50, 70, 90, 110, 130):
        a = math.radians(ang)
        x, y = 1.0 + 0.82 * math.cos(a), 0.3 + 1.08 * math.sin(a)
        bx, by = 1.0 + 0.4 * math.cos(a), 0.3 + 0.5 * math.sin(a)
        fig.stroke([(bx, by), (x - 0.12 * math.cos(a), y - 0.12 * math.sin(a))], width=1.6, role="snow")
        M.axe(fig, x, y, 0.34, rot=ang - 90)
    return fig


def whirling_axes_melee(rng):
    fig = ability_fig("whirling_axes_melee")
    M.ripple(fig, 1.0, 1.0, 0.62, 0.62, role="snow", width=2.6, a0=0, a1=360)
    M.axe(fig, 0.46, 1.0, 0.5, rot=90)
    M.axe(fig, 1.54, 1.0, 0.5, rot=-90)
    M.swirl(fig, 1.0, 1.0, 0.24, role="ink", width=2.2, turns=1.2)
    return fig


def fervor(rng):
    fig = ability_fig("fervor")
    target(fig, 1.12, 0.84, 0.42)
    for i in range(3):
        y = 0.62 + i * 0.3
        fig.poly([(0.32, y), (0.58, y + 0.16), (0.32, y + 0.32), (0.4, y + 0.16)], "flame")
    return fig


def battle_trance(rng):
    fig = ability_fig("battle_trance")
    for r in (0.5, 0.66, 0.82):
        M.ripple(fig, 1.0, 0.86, r, r * 0.8, role="ink", width=2.4, a0=20, a1=160)
    fig.ellipse(1.0, 0.72, 0.42, 0.14, "leather")
    fig.rect(0.58, 0.44, 1.42, 0.72, "flame")
    fig.ellipse(1.0, 0.44, 0.42, 0.14, "leather")
    fig.ellipse(1.0, 0.72, 0.36, 0.1, "skin")
    for x in (0.86, 1.14):
        fig.stroke([(x, 0.72), (x, 0.44)], width=1.6, role="golddark")
    fig.stroke([(0.5, 1.2), (0.86, 0.8)], width=3.0, role="wooddark")
    return fig


# ---- Chipper (earth), round 7 ---------------------------------------------------

def whirling_death(rng):
    fig = ability_fig("whirling_death")
    fig.poly(saw_pts(1.0, 1.0, 0.42, 0.56, teeth=14), "steel")
    fig.circle(1.0, 1.0, 0.14, "gold")
    for a in (30, 150, 270):
        x, y = 1.0 + 0.72 * math.cos(math.radians(a)), 1.0 + 0.72 * math.sin(math.radians(a))
        fig.poly([(x - 0.12, y - 0.03), (x + 0.12, y + 0.03), (x + 0.1, y + 0.08), (x - 0.13, y + 0.02)], "wood")
    M.swirl(fig, 1.0, 1.0, 0.3, role="ink", width=2.0, turns=1.0)
    return fig


def timber_chain(rng):
    fig = ability_fig("timber_chain")
    # a trunk on the right, the claw shut on it, the chain back to the left
    fig.rect(1.28, 0.3, 1.56, 1.7, "wood")
    for y in (0.6, 1.0, 1.4):
        fig.stroke([(1.32, y), (1.5, y + 0.06)], width=1.4, role="wooddark")
    M.chain(fig, [(0.34, 0.64), (1.1, 1.02)], r=0.06)
    for dy in (-0.14, 0.0, 0.14):
        fig.poly([(1.1, 1.02 + dy), (1.32, 1.08 + dy * 1.4), (1.12, 1.1 + dy)], "steel")
    return fig


def reactive_armor(rng):
    fig = ability_fig("reactive_armor")
    for i, (r, fill) in enumerate(((0.66, "wooddark"), (0.52, "wood"), (0.38, "wood!70!leaf"))):
        fig.blob([(1.0 - r, 0.5), (1.0 - r * 0.8, 0.5 + r * 1.1), (1.0, 0.5 + r * 1.5), (1.0 + r * 0.8, 0.5 + r * 1.1), (1.0 + r, 0.5)], fill, tension=0.4)
    M.plus(fig, 1.0, 0.86, 0.45, fill="heal")
    return fig


def chakram(rng):
    fig = ability_fig("chakram")
    fig.poly(saw_pts(1.12, 1.08, 0.36, 0.5, teeth=12, rot=8), "steel")
    fig.circle(1.12, 1.08, 0.12, "gold")
    M.motion(fig, [((0.28, 0.66), (0.62, 0.84)), ((0.3, 0.98), (0.6, 1.04)), ((0.36, 1.3), (0.64, 1.22))], width=2.6)
    return fig


def return_chakram(rng):
    fig = ability_fig("return_chakram")
    fig.poly(saw_pts(1.24, 1.16, 0.26, 0.36, teeth=10), "steel")
    fig.circle(1.24, 1.16, 0.09, "gold")
    # a curved arrow coming home, lower left
    pts = arc_pts(1.0, 1.0, 0.56, 200, 320)
    M.trail(fig, pts, "snow", w0=0.03, w1=0.12)
    fig.poly([(0.38, 0.74), (0.6, 0.62), (0.56, 0.9)], "snow")
    return fig


# ---- Shiver (frost), round 7 ---------------------------------------------------

def cold_feet(rng):
    fig = ability_fig("cold_feet")
    # a frosted patch of ground with ice spikes round two feet held fast
    fig.ellipse(1.0, 0.62, 0.66, 0.24, "ice")
    fig.ellipse(1.0, 0.62, 0.46, 0.15, "snow")
    for x, h in ((0.42, 0.34), (0.62, 0.48), (1.38, 0.48), (1.58, 0.34)):
        M.crystal(fig, x, 0.62 + h / 2, h, "icedeep")
    for x in (0.84, 1.16):
        fig.rect(x - 0.11, 0.62, x + 0.11, 1.02, "leather", r=0.04)
        fig.ellipse(x, 0.64, 0.15, 0.08, "wooddark")
    fig.ellipse(1.0, 0.66, 0.38, 0.1, "ice!70!snow")
    M.snowflake(fig, 1.0, 1.42, 0.22, role="snow", width=2.0)
    return fig


def ice_vortex(rng):
    fig = ability_fig("ice_vortex")
    fig.ellipse(1.0, 0.92, 0.66, 0.42, "ice")
    for r, start in ((0.58, 0), (0.42, 120), (0.26, 240)):
        M.swirl(fig, 1.0, 0.92, r, role="snow", width=2.6, turns=1.1, start=start)
    for x, y in ((0.52, 1.42), (1.46, 1.36), (1.2, 0.42)):
        M.sparkle(fig, x, y, 0.1, "snow")
    return fig


def chilling_touch(rng):
    fig = ability_fig("chilling_touch")
    # three talons of ice reaching down, frost streaming off them
    for i, x in enumerate((0.66, 1.0, 1.34)):
        fig.poly([(x - 0.1, 1.5), (x + 0.1, 1.5), (x + 0.04, 0.82), (x - 0.06 + (i - 1) * 0.04, 0.5)], "ice")
        fig.poly([(x - 0.1, 1.5), (x - 0.02, 1.5), (x - 0.06 + (i - 1) * 0.04, 0.5)], "snow")
    fig.rect(0.5, 1.46, 1.5, 1.66, "icedeep", r=0.08)
    M.sparkle(fig, 0.48, 0.66, 0.12, "snow")
    M.sparkle(fig, 1.56, 0.86, 0.1, "snow")
    return fig


def ice_blast(rng):
    fig = ability_fig("ice_blast")
    # a star of hail streaking down from the upper left
    pts = [(0.3 + 0.07 * i, 1.6 - 0.07 * i) for i in range(10)]
    M.trail(fig, pts, "ice", w0=0.01, w1=0.14)
    M.burst(fig, 1.12, 0.86, 0.42, "ice", n=8, inner=0.45)
    M.burst(fig, 1.12, 0.86, 0.26, "snow", n=8, inner=0.5, rot=22)
    fig.circle(1.12, 0.86, 0.08, "icedeep")
    return fig


def ice_blast_release(rng):
    fig = ability_fig("ice_blast_release")
    # the hail bursting on the ground: shards thrown up from a ring
    fig.ellipse(1.0, 0.6, 0.66, 0.2, "ice")
    for a in (-60, -30, 0, 30, 60):
        x = 1.0 + 0.5 * math.sin(math.radians(a))
        M.crystal(fig, x, 0.98 - abs(a) / 300, 0.5 - abs(a) / 300, "ice", rot=-a)
    M.burst(fig, 1.0, 0.7, 0.28, "snow", n=8, inner=0.5)
    M.ripple(fig, 1.0, 0.6, 0.78, 0.26, role="snow", width=2.4, a0=190, a1=350)
    return fig


_FUNCS = {f.__name__: f for f in (
    storm_bolt, great_cleave, warcry, gods_strength,
    crystal_nova, frostbite, brilliance_aura, freezing_field,
    dragon_slave, light_strike_array, fiery_soul, laguna_blade,
    frost_arrows, silence, trueshot_aura, marksmanship,
    meat_hook, rot, flesh_heap, dismember,
    fissure, enchant_totem, aftershock, echo_slam,
    blade_fury, healing_ward, blade_dance, omnislash,
    earth_spike, hex, mana_drain, finger_of_death,
    shrapnel, headshot, take_aim, assassinate,
    arc_lightning, lightning_bolt, static_field, thundergods_wrath,
    warden_bash,
    berserker_rage, whirling_axes, whirling_axes_melee, fervor, battle_trance,
    whirling_death, timber_chain, reactive_armor, chakram, return_chakram,
    cold_feet, ice_vortex, chilling_touch, ice_blast, ice_blast_release)}

_missing = set(ability_ids()) ^ set(_FUNCS)
if _missing:
    raise SystemExit(f"art_abilities: ability icons out of step with data/abilities.json: {sorted(_missing)}")

PICTURES = {f"abilities/{k}": v for k, v in _FUNCS.items()}
