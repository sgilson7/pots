"""Item icons for the common, support, protect, weapons and caster shop tabs,
and the aegis (no shop): web/art/items/<item id>.png, each on a rounded-square
badge in its tab's colour (tools/art/icons.py)."""
import math

from icons import M, item_fig

THIN = M.THIN


# ---- local helpers ------------------------------------------------------------

def cuff(fig, metal, stone):
    """A wrist cuff: a wide band seen from the front, a gem set in it."""
    with fig.frame(1.0, 1.0, s=1.45):
        fig.rect(-0.44, -0.3, 0.44, 0.26, metal, r=0.07)
        fig.ellipse(0, 0.26, 0.44, 0.11, f"{metal}!70!snow")
        fig.ellipse(0, 0.26, 0.34, 0.065, "shade")
        fig.stroke([(-0.44, -0.18), (0.44, -0.18)], width=THIN)
        for x in (-0.33, 0.33):
            fig.circle(x, 0.0, 0.04, "golddark")
        M.gem(fig, 0, 0.0, 0.36, stone)


def lightning_path(fig, pts, role="spark", width=3.2):
    """A jagged bolt line with an ink edge."""
    fig.stroke(pts, width=width + 2.6, role="ink")
    fig.stroke(pts, width=width, role=role)


def curved_blade(fig, x, y, s, rot, blade, hilt="golddark", grip="leather", bend=0.18):
    """A curved sabre, point up before rot."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.blob([(-0.07, -0.18), (0.07, -0.18), (0.08 + bend * 0.5, 0.3), (bend, 0.64),
                  (bend * 0.2, 0.42), (-0.06, 0.1)], blade, tension=0.45)
        fig.rect(-0.24, -0.26, 0.24, -0.17, hilt, r=0.03)
        fig.rect(-0.05, -0.5, 0.05, -0.26, grip)
        fig.circle(0, -0.54, 0.07, hilt)


def cog(fig, x, y, r, fill="golddark", teeth=8):
    pts = []
    for i in range(teeth * 4):
        a = math.radians(360.0 * i / (teeth * 4))
        rr = r if (i % 4) in (0, 1) else r * 0.8
        pts.append((x + rr * math.cos(a), y + rr * math.sin(a)))
    fig.poly(pts, fill)


def wand(fig, x, y, s, rot, shaft="wood", tip="violet"):
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.05, -0.55, 0.05, 0.4, shaft, r=0.04)
        fig.rect(-0.07, -0.58, 0.07, -0.36, "leather", r=0.03)
        M.star(fig, 0, 0.5, 0.26, tip, points=5)


# ---- common ----------------------------------------------------------------------

def magic_wand(rng):
    fig = item_fig("magic_wand")
    wand(fig, 1.0, 0.98, 1.3, -35, shaft="wooddark", tip="violet")
    for x, y, r in ((0.55, 1.45, 0.09), (1.45, 0.62, 0.08), (0.62, 0.62, 0.07)):
        M.sparkle(fig, x, y, r, "spark")
    M.swirl(fig, 1.36, 1.38, 0.18, role="violet", width=2.4)
    return fig


def bracer(rng):
    fig = item_fig("bracer")
    cuff(fig, "ember", "life")
    return fig


def wraith_band(rng):
    fig = item_fig("wraith_band")
    cuff(fig, "moss", "heal")
    return fig


def null_talisman(rng):
    fig = item_fig("null_talisman")
    cuff(fig, "mana", "icedeep")
    return fig


def power_treads(rng):
    fig = item_fig("power_treads")
    M.boot(fig, 0.98, 1.06, 1.35, fill="ember", sole="iron", cuff="gold")
    for x in (0.62, 0.86, 1.1, 1.34):
        fig.rect(x - 0.06, 0.38, x + 0.06, 0.48, "steeldark", r=0.02)
    return fig


def phase_boots(rng):
    fig = item_fig("phase_boots")
    for i, k in enumerate((0.5, 0.25)):
        with fig.frame(-0.18 * (2 - i), 0):
            M.boot(fig, 1.0, 1.0, 1.25, fill=f"violet!{int(k * 100)}!paper", sole=f"violet!{int(k * 100)}!paper",
                   cuff=None)
    M.boot(fig, 1.08, 1.0, 1.25, fill="violet", sole="night", cuff="ice")
    return fig


def travel_boots(rng):
    fig = item_fig("travel_boots")
    M.wing(fig, 0.86, 1.32, 0.95, fill="snow", rot=-5)
    M.boot(fig, 1.0, 0.98, 1.3, fill="gold", sole="golddark", cuff="snow")
    M.wing(fig, 1.12, 1.38, 0.7, fill="snow", flip=True, rot=10)
    return fig


def oblivion_staff(rng):
    fig = item_fig("oblivion_staff")
    with fig.frame(1.0, 1.0, s=1.35, rot=-30):
        fig.rect(-0.045, -0.6, 0.045, 0.3, "night", r=0.03)
        fig.blob([(-0.2, 0.28), (0.2, 0.28), (0.24, 0.5), (0.0, 0.62), (-0.24, 0.5)], "violet", tension=0.5)
        fig.circle(0, 0.44, 0.09, "night")
        M.shine(fig, -0.12, 0.5, 0.05, 0.03)
    M.swirl(fig, 0.6, 0.6, 0.16, role="violet", width=2.2)
    return fig


def perseverance(rng):
    fig = item_fig("perseverance")
    with fig.frame(1.0, 0.96, s=1.35):
        fig.blob([(-0.42, -0.3), (-0.46, 0.06), (-0.2, 0.36), (0.16, 0.4), (0.44, 0.14), (0.42, -0.3)],
                 "stone", tension=0.4)
        fig.stroke([(-0.2, 0.36), (-0.06, 0.04), (0.44, 0.14)], width=THIN)
        M.gem(fig, -0.02, -0.12, 0.36, "heal")
    M.plus(fig, 1.48, 1.44, 0.35, "heal")
    M.drop(fig, 0.5, 1.45, 0.32, "mana")
    return fig


def midas(rng):
    fig = item_fig("midas")
    with fig.frame(0.95, 0.95, s=1.35):
        # an open palm, fingers up
        fig.blob([(-0.3, -0.5), (0.22, -0.5), (0.3, -0.1), (0.42, 0.1), (0.3, 0.12), (0.16, 0.0),
                  (0.16, 0.42), (-0.32, 0.42), (-0.34, 0.0)], "gold", tension=0.3)
        for x, top in ((-0.27, 0.6), (-0.11, 0.66), (0.05, 0.62)):
            fig.rect(x - 0.07, 0.3, x + 0.07, top, "gold", r=0.07)
        fig.stroke([(-0.3, -0.36), (0.2, -0.36)], width=THIN, role="golddark")
    fig.circle(1.4, 1.36, 0.24, "gold")
    fig.circle(1.4, 1.36, 0.15, "golddark")
    M.sparkle(fig, 1.48, 0.62, 0.1, "snow")
    return fig


# ---- support ----------------------------------------------------------------------

def buckler(rng):
    fig = item_fig("buckler")
    M.round_shield(fig, 1.0, 1.0, 0.6, fill="wood", rim="steel", boss="gold")
    for a in range(0, 360, 60):
        r = math.radians(a + 30)
        fig.circle(1.0 + 0.53 * math.cos(r), 1.0 + 0.53 * math.sin(r), 0.035, "steeldark")
    return fig


def headdress(rng):
    fig = item_fig("headdress")
    cols = ("heal", "gold", "life", "gold", "heal")
    for i, c in enumerate(cols):
        a = -40 + 20 * i
        with fig.frame(1.0, 0.82, s=1.0, rot=a):
            fig.blob([(0, 0.0), (0.12, 0.3), (0.08, 0.68), (0, 0.78), (-0.08, 0.68), (-0.12, 0.3)], c, tension=0.5)
            fig.stroke([(0, 0.05), (0, 0.7)], width=THIN)
    fig.rect(0.46, 0.5, 1.54, 0.8, "leather", r=0.08)
    for x in (0.7, 1.0, 1.3):
        M.gem(fig, x, 0.66, 0.18, "icedeep")
    return fig


def mekansm(rng):
    fig = item_fig("mekansm")
    cog(fig, 1.0, 1.0, 0.62, "steeldark", teeth=10)
    fig.circle(1.0, 1.0, 0.42, "steel")
    M.plus(fig, 1.0, 1.0, 0.85, "heal")
    return fig


# ---- protect ----------------------------------------------------------------------

def vanguard(rng):
    fig = item_fig("vanguard")
    with fig.frame(1.0, 1.0, s=1.4):
        fig.blob([(-0.42, 0.44), (0.42, 0.44), (0.42, -0.1), (0.2, -0.38), (0, -0.48), (-0.2, -0.38),
                  (-0.42, -0.1)], "golddark", tension=0.25)
        fig.blob([(-0.33, 0.35), (0.33, 0.35), (0.33, -0.08), (0.15, -0.31), (0, -0.39), (-0.15, -0.31),
                  (-0.33, -0.08)], "ember", tension=0.25)
        fig.poly([(-0.33, 0.12), (0.33, 0.12), (0.33, 0.0), (-0.33, 0.0)], "steel")
        fig.poly([(-0.06, 0.35), (0.06, 0.35), (0.06, -0.37), (-0.06, -0.37)], "steel")
    return fig


def bkb(rng):
    fig = item_fig("bkb")
    fig.circle(1.0, 1.0, 0.64, "gold!40!paper")
    fig.circle(1.0, 1.0, 0.5, "gold!20!paper")
    with fig.frame(1.0, 1.0, s=1.3, rot=-30):
        fig.rect(-0.5, -0.17, 0.5, 0.17, "night", r=0.05)
        fig.rect(-0.56, -0.2, -0.42, 0.2, "gold", r=0.03)
        fig.rect(0.42, -0.2, 0.56, 0.2, "gold", r=0.03)
        # runes as plain marks, no letters
        for x in (-0.22, 0.0, 0.22):
            fig.poly([(x, 0.09), (x + 0.07, 0.0), (x, -0.09), (x - 0.07, 0.0)], "gold")
    return fig


def heart(rng):
    fig = item_fig("heart")
    M.heart(fig, 1.0, 0.98, 1.45, "stone")
    fig.stroke([(0.78, 1.22), (0.95, 1.0), (0.86, 0.82), (1.02, 0.66)], width=THIN)
    fig.stroke([(1.2, 1.24), (1.12, 1.04), (1.28, 0.9)], width=THIN)
    fig.blob([(0.8, 1.35), (0.92, 1.3), (0.9, 1.18), (0.78, 1.2)], "moss", tension=0.5)
    M.plus(fig, 1.42, 0.55, 0.3, "heal")
    return fig


def shivas(rng):
    fig = item_fig("shivas")
    # a frost wave ring behind
    M.ripple(fig, 1.0, 0.72, 0.66, 0.24, role="icedeep", width=3.4, a0=180, a1=360)
    with fig.frame(1.0, 1.04, s=1.3):
        fig.blob([(-0.42, 0.3), (-0.2, 0.42), (0.2, 0.42), (0.42, 0.3), (0.36, 0.0), (0.3, -0.44),
                  (-0.3, -0.44), (-0.36, 0.0)], "ice", tension=0.3)
        fig.blob([(-0.14, 0.42), (0, 0.24), (0.14, 0.42)], "icedeep", tension=0.2)
        fig.stroke([(0, 0.24), (0, -0.44)], width=THIN)
        for y in (0.04, -0.2):
            fig.stroke([(-0.28, y), (0.28, y)], width=THIN, role="icedeep")
    for x in (0.42, 1.58):
        M.crystal(fig, x, 0.82, 0.38, "ice", rot=25 if x < 1 else -25)
    return fig


def assault(rng):
    fig = item_fig("assault")
    # a war banner behind
    fig.rect(1.36, 0.42, 1.42, 1.62, "wooddark")
    fig.poly([(1.42, 1.6), (1.66, 1.5), (1.42, 1.34)], "life")
    with fig.frame(0.94, 0.98, s=1.3):
        fig.blob([(-0.44, 0.32), (-0.2, 0.42), (0.2, 0.42), (0.44, 0.32), (0.36, 0.0), (0.3, -0.44),
                  (-0.3, -0.44), (-0.36, 0.0)], "steel", tension=0.3)
        fig.blob([(-0.14, 0.42), (0, 0.22), (0.14, 0.42)], "steeldark", tension=0.2)
        fig.poly([(-0.2, 0.06), (0.2, 0.06), (0.14, -0.3), (0, -0.38), (-0.14, -0.3)], "gold")
        fig.circle(-0.38, 0.36, 0.1, "golddark")
        fig.circle(0.38, 0.36, 0.1, "golddark")
    return fig


def linkens(rng):
    fig = item_fig("linkens")
    fig.ellipse(1.0, 0.5, 0.36, 0.1, "golddark")
    fig.poly([(0.78, 0.5), (1.22, 0.5), (1.12, 0.66), (0.88, 0.66)], "gold")
    M.orb(fig, 1.0, 1.1, 0.5, "icedeep")
    fig.ellipse(1.0, 1.1, 0.5, 0.16, "icedeep")
    fig.ellipse(1.18, 0.95, 0.12, 0.2, "ice", rot=30)
    M.shine(fig, 0.82, 1.3, 0.12, 0.06, rot=35)
    return fig


# ---- weapons -------------------------------------------------------------------------

def battlefury(rng):
    fig = item_fig("battlefury")
    M.ripple(fig, 1.0, 1.0, 0.68, 0.68, role="ink", width=2.0, a0=200, a1=320)
    with fig.frame(1.0, 1.0, s=1.4, rot=-25):
        fig.rect(-0.045, -0.6, 0.045, 0.52, "wooddark", r=0.03)
        for flip in (False, True):
            with fig.frame(0, 0.24, flip=flip):
                edge = [(0.2 + 0.28 * math.cos(math.radians(a)), 0.0 + 0.36 * math.sin(math.radians(a)))
                        for a in range(-70, 71, 14)]
                fig.poly([(0.0, -0.1)] + edge + [(0.0, 0.12)], "steel")
        fig.circle(0, 0.24, 0.07, "gold")
    return fig


def crystalys(rng):
    fig = item_fig("crystalys")
    M.sword(fig, 0.95, 0.98, 1.3, rot=-40, blade="steel", hilt="icedeep")
    M.crystal(fig, 1.42, 1.42, 0.5, "icedeep", rot=-30)
    M.sparkle(fig, 1.48, 0.6, 0.12, "snow")
    return fig


def daedalus(rng):
    fig = item_fig("daedalus")
    M.bow(fig, 0.86, 1.0, 1.45, rot=-40, wood="life")
    M.arrow(fig, 0.5, 0.5, 1.52, 1.52, shaft="wooddark", head="steel", fletch="gold")
    M.burst(fig, 1.48, 1.48, 0.2, "spark", n=8)
    return fig


def mkb(rng):
    fig = item_fig("mkb")
    with fig.frame(1.0, 1.0, s=1.4, rot=-40):
        fig.rect(-0.05, -0.6, 0.05, 0.6, "wood", r=0.04)
        for y in (-0.6, 0.48):
            fig.rect(-0.08, y, 0.08, y + 0.12, "gold", r=0.03)
    lightning_path(fig, [(1.38, 1.62), (1.2, 1.36), (1.4, 1.26), (1.18, 0.98)])
    lightning_path(fig, [(0.62, 1.0), (0.44, 0.78), (0.62, 0.7), (0.46, 0.44)])
    return fig


def basher(rng):
    fig = item_fig("basher")
    M.mace(fig, 1.0, 0.92, 1.35, rot=-25)
    M.stun_stars(fig, 1.08, 1.56, 0.85)
    return fig


def butterfly(rng):
    fig = item_fig("butterfly")
    for flip in (False, True):
        with fig.frame(1.0, 1.02, flip=flip):
            fig.blob([(0.02, 0.0), (0.3, 0.42), (0.56, 0.4), (0.5, 0.1), (0.3, -0.02)], "heal", tension=0.5)
            fig.blob([(0.02, -0.04), (0.36, -0.12), (0.42, -0.36), (0.18, -0.36)], "leaf", tension=0.5)
            fig.circle(0.36, 0.22, 0.06, "spark")
    M.sword(fig, 1.0, 0.92, 1.0, rot=0, blade="steel", hilt="moss")
    return fig


def desolator(rng):
    fig = item_fig("desolator")
    curved_blade(fig, 0.92, 1.0, 1.35, -35, "night", hilt="ember", grip="wooddark", bend=0.24)
    for x, y in ((1.2, 0.86), (1.4, 1.06), (1.0, 0.62)):
        M.drop(fig, x, y, 0.22, "venom")
    M.skull(fig, 0.56, 1.46, 0.36)
    return fig


def maelstrom(rng):
    fig = item_fig("maelstrom")
    M.hammer(fig, 0.94, 0.96, 1.25, rot=-30, head="icedeep", haft="wood", band="steel")
    lightning_path(fig, [(1.34, 1.06), (1.5, 0.86), (1.34, 0.78), (1.52, 0.52)])
    return fig


def mjollnir(rng):
    fig = item_fig("mjollnir")
    with fig.frame(1.0, 0.96, s=1.4, rot=-30):
        fig.rect(-0.05, -0.55, 0.05, 0.2, "wooddark", r=0.03)
        fig.rect(-0.42, 0.16, 0.42, 0.56, "steel", r=0.06)
        fig.rect(-0.42, 0.16, -0.28, 0.56, "gold")
        fig.rect(0.28, 0.16, 0.42, 0.56, "gold")
        M.bolt(fig, 0.0, 0.36, 0.34, "spark")
        fig.rect(-0.07, -0.58, 0.07, -0.48, "gold", r=0.02)
    lightning_path(fig, [(0.44, 1.2), (0.6, 1.0), (0.44, 0.92), (0.6, 0.64)])
    lightning_path(fig, [(1.6, 1.36), (1.44, 1.14), (1.6, 1.06), (1.46, 0.8)])
    return fig


def radiance(rng):
    fig = item_fig("radiance")
    M.burst(fig, 1.0, 1.0, 0.7, "flame", n=12, inner=0.72)
    fig.circle(1.0, 1.0, 0.42, "flamehot")
    fig.circle(1.0, 1.0, 0.26, "spark")
    M.shine(fig, 0.88, 1.12, 0.08, 0.05)
    return fig


def rapier(rng):
    fig = item_fig("rapier")
    with fig.frame(1.0, 1.0, s=1.45, rot=-45):
        fig.poly([(-0.035, -0.2), (0.035, -0.2), (0.02, 0.58), (0, 0.66), (-0.02, 0.58)], "ice")
        fig.blob([(-0.22, -0.18), (0, -0.32), (0.22, -0.18), (0, -0.12)], "gold", tension=0.6)
        fig.rect(-0.04, -0.46, 0.04, -0.26, "life")
        fig.circle(0, -0.5, 0.06, "gold")
    for x, y, r in ((1.46, 1.46, 0.12), (0.62, 1.3, 0.08), (1.3, 0.66, 0.07)):
        M.sparkle(fig, x, y, r, "spark")
    return fig


def helm_dominator(rng):
    fig = item_fig("helm_dominator")
    for flip in (False, True):
        with fig.frame(1.0, 1.0, flip=flip):
            fig.blob([(0.22, 0.26), (0.5, 0.36), (0.62, 0.62), (0.52, 0.5), (0.3, 0.44)], "bone", tension=0.4)
    with fig.frame(1.0, 0.96, s=1.3):
        fig.blob([(-0.34, -0.42), (-0.38, 0.06), (-0.24, 0.34), (0, 0.42), (0.24, 0.34), (0.38, 0.06), (0.34, -0.42)],
                 "iron", tension=0.4)
        fig.poly([(-0.27, -0.02), (-0.06, -0.08), (-0.27, 0.08)], "life")
        fig.poly([(0.27, -0.02), (0.06, -0.08), (0.27, 0.08)], "life")
        fig.poly([(-0.04, 0.0), (0.04, 0.0), (0.03, -0.34), (-0.03, -0.34)], "ink")
        fig.poly([(-0.06, 0.42), (0, 0.18), (0.06, 0.42)], "gold")
    return fig


def satanic(rng):
    fig = item_fig("satanic")
    M.drop(fig, 1.0, 1.02, 1.3, "life")
    with fig.frame(1.0, 0.92, s=0.9):
        fig.poly([(-0.24, 0.24), (-0.12, -0.24), (-0.02, 0.24)], "bone")
        fig.poly([(0.02, 0.24), (0.12, -0.24), (0.24, 0.24)], "bone")
    return fig


def sange(rng):
    fig = item_fig("sange")
    curved_blade(fig, 1.0, 1.0, 1.4, -40, "life", hilt="gold", bend=0.1)
    return fig


def yasha(rng):
    fig = item_fig("yasha")
    curved_blade(fig, 1.0, 1.0, 1.4, -40, "leaf", hilt="steel", bend=0.3)
    M.motion(fig, [((0.46, 0.72), (0.7, 0.6)), ((0.42, 0.92), (0.62, 0.84))], width=2.2)
    return fig


def sange_yasha(rng):
    fig = item_fig("sange_yasha")
    curved_blade(fig, 0.94, 0.96, 1.3, -35, "life", hilt="gold", bend=0.1)
    curved_blade(fig, 1.06, 0.96, 1.3, 35, "leaf", hilt="steel", bend=-0.3)
    return fig


def manta(rng):
    fig = item_fig("manta")

    def figure(x, fill, s=1.0):
        with fig.frame(x, 0.86, s=s):
            fig.circle(0, 0.5, 0.17, fill)
            fig.blob([(-0.3, -0.38), (-0.26, 0.12), (0, 0.3), (0.26, 0.12), (0.3, -0.38)], fill, tension=0.4)

    figure(0.58, "icedeep!45!paper", 0.85)
    figure(1.42, "icedeep!45!paper", 0.85)
    figure(1.0, "icedeep", 1.05)
    return fig


# ---- caster --------------------------------------------------------------------------

def dagon(rng):
    fig = item_fig("dagon")
    with fig.frame(0.7, 0.72, s=1.0, rot=-45):
        fig.rect(-0.06, -0.4, 0.06, 0.24, "wooddark", r=0.03)
        fig.poly([(-0.16, 0.2), (0.16, 0.2), (0.0, 0.5)], "ember")
    # the beam
    fig.poly([(0.92, 0.96), (1.54, 1.42), (1.6, 1.32), (0.96, 0.9)], "flamehot")
    M.burst(fig, 1.5, 1.42, 0.22, "ember", n=8)
    M.burst(fig, 0.92, 0.92, 0.13, "spark", n=6)
    return fig


def euls(rng):
    fig = item_fig("euls")
    # cyclone funnel
    for i, (y, rx) in enumerate(((1.5, 0.42), (1.28, 0.34), (1.06, 0.26), (0.86, 0.18), (0.68, 0.11))):
        M.ripple(fig, 1.14 + 0.03 * i, y, rx, 0.08, role="icedeep", width=3.0, a0=150, a1=390)
    M.staff(fig, 0.66, 0.98, 1.2, rot=-8, shaft="wood", top="icedeep", topr=0.13)
    return fig


def force_staff(rng):
    fig = item_fig("force_staff")
    M.staff(fig, 0.62, 1.0, 1.2, rot=0, shaft="wooddark", top="spark", topr=0.12)
    with fig.frame(1.18, 1.0, s=1.0):
        fig.poly([(-0.32, -0.12), (0.12, -0.12), (0.12, -0.3), (0.46, 0.0), (0.12, 0.3), (0.12, 0.12),
                  (-0.32, 0.12)], "heal")
    M.motion(fig, [((0.86, 1.36), (1.2, 1.36)), ((0.86, 0.64), (1.2, 0.64))], width=2.2)
    return fig


def orchid(rng):
    fig = item_fig("orchid")
    fig.stroke([(1.0, 0.36), (1.0, 0.96)], width=3.4, role="ink")
    fig.stroke([(1.0, 0.36), (1.0, 0.96)], width=1.6, role="leaf")
    M.leaf(fig, 0.78, 0.6, 0.45, rot=50)
    for a in range(0, 360, 72):
        with fig.frame(1.0, 1.12, rot=a):
            fig.ellipse(0, 0.26, 0.15, 0.26, "violet")
    fig.circle(1.0, 1.12, 0.12, "night")
    # hush: a closed mouth bar in the heart of the bloom
    fig.stroke([(0.94, 1.12), (1.06, 1.12)], width=1.8, role="violet")
    return fig


def sheep(rng):
    fig = item_fig("sheep")
    M.staff(fig, 0.56, 1.06, 1.2, rot=-10, shaft="wood", top="heal", topr=0.12)
    with fig.frame(1.18, 0.74, s=0.85):
        fig.blob([(-0.42, -0.18), (-0.36, 0.14), (-0.1, 0.3), (0.2, 0.3), (0.42, 0.12), (0.44, -0.18)],
                 "leaf", tension=0.5)
        for x in (-0.18, 0.18):
            fig.circle(x, 0.32, 0.12, "leaf")
            fig.circle(x, 0.34, 0.05, "ink")
        fig.stroke([(-0.14, 0.02), (0.0, -0.04), (0.14, 0.02)], width=THIN)
    M.sparkle(fig, 1.42, 1.32, 0.12, "spark")
    return fig


def aghanims(rng):
    fig = item_fig("aghanims")
    with fig.frame(1.0, 1.0, s=1.4, rot=-30):
        fig.rect(-0.05, -0.6, 0.05, 0.22, "gold", r=0.03)
        fig.rect(-0.08, -0.2, 0.08, -0.08, "golddark", r=0.02)
        fig.blob([(-0.22, 0.2), (0.0, 0.14), (0.22, 0.2), (0.18, 0.4), (0, 0.32), (-0.18, 0.4)], "golddark", tension=0.4)
        M.gem(fig, 0, 0.42, 0.4, "icedeep")
    for x, y in ((0.54, 1.5), (1.48, 0.62)):
        M.sparkle(fig, x, y, 0.1, "snow")
    return fig


# ---- none -------------------------------------------------------------------------

def aegis(rng):
    fig = item_fig("aegis")
    M.burst(fig, 1.0, 1.16, 0.62, "flamehot", n=10, inner=0.7)
    M.wing(fig, 0.78, 1.04, 1.05, fill="snow")
    M.wing(fig, 1.22, 1.04, 1.05, fill="snow", flip=True)
    M.shield(fig, 1.0, 0.96, 1.05, fill="gold", trim="golddark", boss="life")
    return fig


PICTURES = {f"items/{fn.__name__}": fn for fn in (
    magic_wand, bracer, wraith_band, null_talisman, power_treads, phase_boots, travel_boots,
    oblivion_staff, perseverance, midas,
    buckler, headdress, mekansm,
    vanguard, bkb, heart, shivas, assault, linkens,
    battlefury, crystalys, daedalus, mkb, basher, butterfly, desolator, maelstrom, mjollnir,
    radiance, rapier, helm_dominator, satanic, sange, yasha, sange_yasha, manta,
    dagon, euls, force_staff, orchid, sheep, aghanims,
    aegis,
)}
