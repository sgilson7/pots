"""Item icons, part A: the consumables, basics, attributes, armaments, arcane
and secret tabs. Each is web/art/items/<id>.png on a rounded-square badge in
its shop tab's rim colour (tools/art/icons.py)."""
import math

from icons import M, item_fig

THIN = M.THIN


# ---- local helpers ---------------------------------------------------------------

def sprig(fig, x, y, s, rng, leaf="leaf"):
    """A sapling twig with leaves (tango)."""
    with fig.frame(x, y, s=s):
        fig.stroke([(0, -0.55), (0.02, -0.1), (-0.06, 0.3), (0.0, 0.55)], width=5.0, role="ink", smooth=True)
        fig.stroke([(0, -0.55), (0.02, -0.1), (-0.06, 0.3), (0.0, 0.55)], width=2.6, role="wood", smooth=True)
        for (lx, ly, r) in ((0.0, 0.5, 0), (-0.25, 0.22, 50), (0.24, 0.05, -55), (-0.22, -0.2, 60)):
            M.leaf(fig, lx, ly, 0.5, rot=r + rng.uniform(-6, 6), fill=leaf)


def ward(fig, x, y, s, iris, cap):
    """A ward: an eye on a carved stick."""
    with fig.frame(x, y, s=s):
        fig.rect(-0.06, -0.62, 0.06, 0.05, "wood", r=0.03)
        fig.poly([(-0.2, -0.62), (0.2, -0.62), (0.1, -0.5), (-0.1, -0.5)], "wooddark")
        fig.circle(0, 0.25, 0.36, cap)
        M.eye(fig, 0, 0.25, 0.62, iris=iris)


def flask(fig, x, y, s, liquid, glass="ice!50!snow"):
    """A tall bottle."""
    with fig.frame(x, y, s=s):
        fig.rect(-0.09, 0.22, 0.09, 0.48, glass)
        fig.rect(-0.12, 0.44, 0.12, 0.56, "wood", r=0.03)
        fig.blob([(-0.09, 0.24), (0.09, 0.24), (0.3, 0.04), (0.32, -0.36), (0.22, -0.5), (-0.22, -0.5),
                  (-0.32, -0.36), (-0.3, 0.04)], glass, tension=0.4)
        fig.blob([(-0.31, -0.12), (0.31, -0.12), (0.3, -0.38), (0.2, -0.47), (-0.2, -0.47), (-0.3, -0.38)],
                 liquid, tension=0.4)
        M.shine(fig, -0.18, 0.02, 0.05, 0.12, rot=-10)


def gauntlet(fig, x, y, s, fill="steel", cuff="golddark", flip=False):
    """An armoured glove, fingers up."""
    with fig.frame(x, y, s=s, flip=flip):
        fig.rect(-0.26, -0.6, 0.24, -0.26, cuff, r=0.05)
        fig.blob([(-0.26, -0.28), (-0.3, 0.08), (-0.2, 0.3), (0.2, 0.3), (0.26, 0.0), (0.24, -0.28)], fill, tension=0.3)
        for i, fx in enumerate((-0.18, -0.06, 0.06, 0.18)):
            top = 0.56 - abs(i - 1.5) * 0.06
            fig.rect(fx - 0.055, 0.2, fx + 0.055, top, fill, r=0.05)
        fig.blob([(0.22, -0.05), (0.42, 0.1), (0.46, 0.24), (0.36, 0.26), (0.2, 0.12)], fill, tension=0.4)
        fig.stroke([(-0.26, -0.12), (0.24, -0.12)], width=THIN, role="steeldark")


def slipper(fig, x, y, s, fill, trim):
    """A soft pointed shoe, toe to the right."""
    with fig.frame(x, y, s=s):
        fig.blob([(-0.5, -0.2), (-0.48, 0.12), (-0.24, 0.2), (0.0, 0.06), (0.3, 0.02), (0.56, 0.24),
                  (0.52, -0.06), (0.36, -0.22)], fill, tension=0.4)
        fig.ellipse(-0.28, 0.14, 0.2, 0.06, "shade")
        fig.stroke([(-0.48, -0.1), (0.36, -0.14)], width=THIN, role=trim)


def cloak(fig, x, y, s, fill, trim, clasp="gold"):
    """A shawl or mantle draped over shoulders."""
    with fig.frame(x, y, s=s):
        fig.blob([(-0.22, 0.44), (0.22, 0.44), (0.5, 0.2), (0.56, -0.5), (0.2, -0.36), (0, -0.48),
                  (-0.2, -0.36), (-0.56, -0.5), (-0.5, 0.2)], fill, tension=0.3)
        fig.blob([(-0.22, 0.44), (0.22, 0.44), (0.2, 0.3), (-0.2, 0.3)], trim, tension=0.2)
        fig.stroke([(-0.3, 0.2), (-0.36, -0.36)], width=THIN)
        fig.stroke([(0.3, 0.2), (0.36, -0.36)], width=THIN)
        fig.circle(0, 0.28, 0.08, clasp)


def robe(fig, x, y, s, fill, trim):
    """A long robe with sleeves."""
    with fig.frame(x, y, s=s):
        fig.poly([(-0.2, 0.46), (-0.54, 0.2), (-0.46, 0.0), (-0.28, 0.1), (-0.36, -0.56), (0.36, -0.56),
                  (0.28, 0.1), (0.46, 0.0), (0.54, 0.2), (0.2, 0.46)], fill)
        fig.poly([(-0.2, 0.46), (0, 0.18), (0.2, 0.46), (0.1, 0.46), (0, 0.32), (-0.1, 0.46)], trim)
        fig.poly([(-0.06, 0.24), (0.06, 0.24), (0.06, -0.56), (-0.06, -0.56)], trim)
        fig.rect(-0.3, -0.1, 0.3, 0.0, "golddark")


def belt(fig, x, y, s, fill, buckle):
    with fig.frame(x, y, s=s):
        fig.blob([(-0.6, 0.1), (0, 0.2), (0.6, 0.1), (0.6, -0.14), (0, -0.04), (-0.6, -0.14)], fill, tension=0.3)
        fig.rect(-0.22, -0.2, 0.22, 0.26, buckle, r=0.05)
        fig.rect(-0.12, -0.1, 0.12, 0.16, "shade", r=0.03)
        fig.rect(-0.02, -0.06, 0.24, 0.0, "steel")


def sash(fig, x, y, s, fill, knot):
    with fig.frame(x, y, s=s):
        fig.blob([(-0.6, 0.18), (0.0, 0.08), (0.6, 0.18), (0.6, -0.06), (0.0, -0.14), (-0.6, -0.06)], fill, tension=0.3)
        fig.poly([(0.0, -0.04), (0.14, -0.6), (0.26, -0.5), (0.08, -0.04)], fill)
        fig.poly([(0.0, -0.04), (-0.2, -0.56), (-0.06, -0.6), (0.06, -0.04)], fill)
        fig.circle(0.03, 0.02, 0.12, knot)


def wand(fig, x, y, s, rot, fill="wood"):
    with fig.frame(x, y, s=s, rot=rot):
        fig.blob([(-0.05, -0.6), (0.05, -0.6), (0.07, 0.0), (0.04, 0.5), (-0.04, 0.5), (-0.07, 0.0)], fill, tension=0.3)


def crescent(fig, x, y, r, fill="flamehot", rot=0.0):
    """A crescent moon opening to the right: the unit circle less a circle of
    radius 0.8 centred 0.35 to the right (they cross at 46.4 and 64.9 deg)."""
    pts = []
    for i in range(19):
        a = math.radians(46.4 + (313.6 - 46.4) * i / 18)
        pts.append((r * math.cos(a), r * math.sin(a)))
    for i in range(15):
        a = math.radians(295.1 - (295.1 - 64.9) * i / 14)
        pts.append((0.35 * r + 0.8 * r * math.cos(a), 0.8 * r * math.sin(a)))
    with fig.frame(x, y, s=1.0, rot=rot):
        fig.poly(pts, fill)


def mask(fig, x, y, s, fill, eyes="ink"):
    with fig.frame(x, y, s=s):
        fig.blob([(-0.42, 0.3), (0, 0.42), (0.42, 0.3), (0.4, -0.1), (0.2, -0.4), (0, -0.5), (-0.2, -0.4),
                  (-0.4, -0.1)], fill, tension=0.5)
        fig.poly([(-0.32, 0.12), (-0.08, 0.06), (-0.12, -0.04), (-0.3, 0.0)], eyes)
        fig.poly([(0.32, 0.12), (0.08, 0.06), (0.12, -0.04), (0.3, 0.0)], eyes)
        fig.ellipse(0, -0.28, 0.12, 0.06, eyes)


# ---- consumables ---------------------------------------------------------------

def tango(rng):
    fig = item_fig("tango")
    sprig(fig, 1.0, 1.0, 1.25, rng)
    return fig


def salve(rng):
    fig = item_fig("salve")
    with fig.frame(1.0, 0.9, s=1.3):
        fig.ellipse(0, -0.12, 0.5, 0.2, "wooddark")
        fig.rect(-0.5, -0.12, 0.5, 0.14, "wood")
        fig.ellipse(0, 0.14, 0.5, 0.2, "heal")
        fig.ellipse(0, 0.16, 0.36, 0.12, "heal!60!snow")
    M.plus(fig, 1.0, 1.42, 0.7, "life")
    return fig


def clarity(rng):
    fig = item_fig("clarity")
    flask(fig, 1.0, 0.98, 1.35, "mana")
    M.sparkle(fig, 1.5, 1.45, 0.12)
    return fig


def tp(rng):
    fig = item_fig("tp")
    with fig.frame(1.0, 0.95, s=1.3):
        fig.blob([(-0.36, -0.5), (-0.42, 0.1), (-0.28, 0.42), (0.0, 0.52), (0.3, 0.4), (0.4, 0.06), (0.36, -0.5)],
                 "stone", tension=0.4)
        fig.rect(-0.46, -0.58, 0.46, -0.46, "stonedark", r=0.04)
        M.swirl(fig, 0.0, 0.02, 0.26, role="icedeep", width=3.2, turns=1.7)
    return fig


def observer(rng):
    fig = item_fig("observer")
    ward(fig, 1.0, 1.0, 1.25, "flamehot", "gold")
    return fig


def sentry(rng):
    fig = item_fig("sentry")
    ward(fig, 1.0, 1.0, 1.25, "icedeep", "mana")
    return fig


def bottle(rng):
    fig = item_fig("bottle")
    flask(fig, 0.92, 0.95, 1.3, "mana!50!life")
    M.swirl(fig, 1.42, 1.38, 0.2, role="icedeep", width=2.6, turns=1.4)
    return fig


# ---- basics -----------------------------------------------------------------------

def branch(rng):
    fig = item_fig("branch")
    with fig.frame(1.0, 1.0, s=1.35, rot=-40):
        fig.blob([(-0.06, -0.6), (0.06, -0.6), (0.06, 0.55), (-0.06, 0.55)], "wood", tension=0.2)
        fig.poly([(0.04, 0.0), (0.3, 0.3), (0.26, 0.36), (0.0, 0.1)], "wood")
    M.leaf(fig, 1.42, 1.32, 0.5, rot=-60)
    return fig


def gauntlets(rng):
    fig = item_fig("gauntlets")
    gauntlet(fig, 1.0, 1.0, 1.3, "life!70!steeldark", "golddark")
    return fig


def slippers(rng):
    fig = item_fig("slippers")
    slipper(fig, 1.0, 1.0, 1.3, "heal!70!leaf", "snow")
    return fig


def mantle(rng):
    fig = item_fig("mantle")
    cloak(fig, 1.0, 0.98, 1.3, "mana", "icedeep")
    return fig


def circlet(rng):
    fig = item_fig("circlet")
    with fig.frame(1.0, 1.0, s=1.3):
        fig.ellipse(0, -0.05, 0.52, 0.32, "gold")
        fig.ellipse(0, -0.02, 0.4, 0.2, "shade")
        fig.poly([(-0.16, 0.18), (0, 0.48), (0.16, 0.18)], "gold")
        fig.circle(0, 0.25, 0.09, "life")
    return fig


def ring_protection(rng):
    fig = item_fig("ring_protection")
    M.ring(fig, 1.0, 0.92, 1.35, band="steel", stone=None)
    M.shield(fig, 1.0, 1.42, 0.4, fill="mana", trim="steeldark", boss=None)
    return fig


def stout_shield(rng):
    fig = item_fig("stout_shield")
    with fig.frame(1.0, 1.0, s=1.3):
        fig.rect(-0.42, -0.48, 0.42, 0.48, "wood", r=0.12)
        for xx in (-0.14, 0.14):
            fig.stroke([(xx, -0.46), (xx, 0.46)], width=THIN, role="wooddark")
        fig.rect(-0.42, 0.18, 0.42, 0.28, "steeldark")
        fig.rect(-0.42, -0.28, 0.42, -0.18, "steeldark")
    return fig


def gloves(rng):
    fig = item_fig("gloves")
    gauntlet(fig, 1.05, 0.98, 1.2, "flame!70!leather", "leather")
    M.motion(fig, [((0.38, 1.25), (0.62, 1.25)), ((0.34, 1.05), (0.6, 1.05)), ((0.38, 0.85), (0.62, 0.85))], width=2.4)
    return fig


def boots(rng):
    fig = item_fig("boots")
    M.boot(fig, 1.0, 1.0, 1.35)
    return fig


def ring_regen(rng):
    fig = item_fig("ring_regen")
    M.ring(fig, 1.0, 0.92, 1.35, band="gold", stone="heal")
    return fig


def sobi(rng):
    fig = item_fig("sobi")
    mask(fig, 1.0, 1.0, 1.35, "cloth")
    fig.circle(1.0, 1.36, 0.07, "mana")
    return fig


def magic_stick(rng):
    fig = item_fig("magic_stick")
    wand(fig, 0.95, 0.95, 1.3, -35)
    for (sx, sy, r) in ((1.38, 1.42, 0.13), (1.1, 1.56, 0.08), (1.54, 1.15, 0.08)):
        M.sparkle(fig, sx, sy, r + rng.uniform(-0.01, 0.01), "spark")
    return fig


# ---- attributes ------------------------------------------------------------------

def belt_item(rng):
    fig = item_fig("belt")
    belt(fig, 1.0, 1.0, 1.25, "life!60!leather", "gold")
    return fig


def elvenskin(rng):
    fig = item_fig("elvenskin")
    sash(fig, 1.0, 1.08, 1.25, "leaf", "golddark")
    return fig


def robe_item(rng):
    fig = item_fig("robe")
    robe(fig, 1.0, 1.0, 1.3, "violet", "mana")
    return fig


def ogre_club(rng):
    fig = item_fig("ogre_club")
    with fig.frame(1.0, 1.0, s=1.35, rot=-35):
        fig.blob([(-0.06, -0.58), (0.06, -0.58), (0.2, 0.2), (0.24, 0.5), (0.0, 0.6), (-0.24, 0.5),
                  (-0.2, 0.2)], "wood", tension=0.4)
        for (sx, sy) in ((-0.12, 0.3), (0.1, 0.4), (0.06, 0.12)):
            fig.poly([(sx - 0.05, sy), (sx, sy + 0.1), (sx + 0.05, sy)], "steel")
    return fig


def alacrity(rng):
    fig = item_fig("alacrity")
    M.dagger(fig, 1.0, 1.0, 1.35, rot=-45, blade="heal!40!steel")
    M.motion(fig, [((0.38, 0.84), (0.56, 0.66)), ((0.52, 0.48), (0.64, 0.36))], width=2.4)
    return fig


def wizardry(rng):
    fig = item_fig("wizardry")
    M.staff(fig, 1.0, 0.95, 1.25, rot=-30, top="mana", topr=0.2)
    return fig


def ultimate_orb(rng):
    fig = item_fig("ultimate_orb")
    M.burst(fig, 1.0, 1.0, 0.7, "gold", n=12, inner=0.78)
    M.orb(fig, 1.0, 1.0, 0.48, "flamehot!60!snow")
    return fig


# ---- armaments ------------------------------------------------------------------

def blades(rng):
    fig = item_fig("blades")
    M.dagger(fig, 1.0, 1.0, 1.3, rot=35)
    M.dagger(fig, 1.0, 1.0, 1.3, rot=-35, blade="steel!80!ice")
    return fig


def quarterstaff(rng):
    fig = item_fig("quarterstaff")
    with fig.frame(1.0, 1.0, s=1.0, rot=-45):
        fig.rect(-0.07, -0.86, 0.07, 0.86, "wood", r=0.05)
        for yy in (-0.62, 0.62):
            fig.rect(-0.09, yy - 0.08, 0.09, yy + 0.08, "steeldark", r=0.02)
    return fig


def chainmail(rng):
    fig = item_fig("chainmail")
    with fig.frame(1.0, 0.98, s=1.3):
        fig.poly([(-0.2, 0.46), (-0.52, 0.3), (-0.5, 0.0), (-0.32, 0.04), (-0.32, -0.52), (0.32, -0.52),
                  (0.32, 0.04), (0.5, 0.0), (0.52, 0.3), (0.2, 0.46), (0, 0.32)], "steel")
        for row in range(4):
            yy = 0.16 - row * 0.18
            for col in range(4):
                xx = -0.21 + col * 0.14
                fig.ellipse(xx, yy, 0.06, 0.05, "steeldark")
    return fig


def helm_iron_will(rng):
    fig = item_fig("helm_iron_will")
    M.helm(fig, 1.0, 0.94, 1.35, fill="steeldark!60!steel", crest="gold")
    return fig


def broadsword(rng):
    fig = item_fig("broadsword")
    M.sword(fig, 1.0, 1.0, 1.15, rot=-45)
    return fig


def claymore(rng):
    fig = item_fig("claymore")
    with fig.frame(1.0, 0.96, s=1.05, rot=-40):
        fig.poly([(-0.12, -0.26), (0.12, -0.26), (0.12, 0.66), (0, 0.84), (-0.12, 0.66)], "steel")
        fig.stroke([(0, -0.2), (0, 0.64)], width=THIN, role="steeldark")
        fig.poly([(-0.36, -0.36), (-0.3, -0.26), (0.3, -0.26), (0.36, -0.36), (0.3, -0.3), (-0.3, -0.3)], "golddark")
        fig.rect(-0.36, -0.34, 0.36, -0.26, "golddark", r=0.03)
        fig.rect(-0.05, -0.7, 0.05, -0.34, "leather")
        fig.circle(0, -0.74, 0.07, "golddark")
    return fig


def platemail(rng):
    fig = item_fig("platemail")
    with fig.frame(1.0, 0.98, s=1.3):
        fig.ellipse(-0.36, 0.3, 0.2, 0.15, "steeldark")
        fig.ellipse(0.36, 0.3, 0.2, 0.15, "steeldark")
        fig.blob([(-0.3, 0.4), (0, 0.3), (0.3, 0.4), (0.32, -0.1), (0.22, -0.5), (-0.22, -0.5), (-0.32, -0.1)],
                 "steel", tension=0.3)
        fig.stroke([(0, 0.3), (0, -0.48)], width=THIN, role="steeldark")
        fig.stroke([(-0.3, -0.2), (0.3, -0.2)], width=THIN, role="steeldark")
        fig.ellipse(0, 0.12, 0.08, 0.08, "gold")
    return fig


def javelin(rng):
    fig = item_fig("javelin")
    M.arrow(fig, 0.42, 0.42, 1.62, 1.62, fletch="flame")
    return fig


def mithril(rng):
    fig = item_fig("mithril")
    M.hammer(fig, 1.0, 0.98, 1.3, rot=30, head="icedeep!40!steel")
    return fig


def mask_death(rng):
    fig = item_fig("mask_death")
    mask(fig, 1.0, 1.02, 1.35, "bone", eyes="life")
    M.drop(fig, 1.32, 0.56, 0.32, "life")
    return fig


# ---- arcane -----------------------------------------------------------------------

def void_stone(rng):
    fig = item_fig("void_stone")
    with fig.frame(1.0, 1.0, s=1.3):
        fig.blob([(-0.4, -0.2), (-0.36, 0.24), (0, 0.44), (0.38, 0.24), (0.42, -0.2), (0, -0.42)], "night", tension=0.5)
    M.swirl(fig, 1.0, 1.0, 0.3, role="violet", width=3.0, turns=1.6)
    return fig


def ring_health(rng):
    fig = item_fig("ring_health")
    M.ring(fig, 1.0, 0.92, 1.35, band="heal!60!gold", stone=None)
    M.heart(fig, 1.0, 1.48, 0.42)
    return fig


def vitality(rng):
    fig = item_fig("vitality")
    M.gem(fig, 1.0, 1.0, 1.25, "life")
    return fig


def energy(rng):
    fig = item_fig("energy")
    M.gem(fig, 1.0, 1.0, 1.25, "mana")
    return fig


def point_booster(rng):
    fig = item_fig("point_booster")
    M.gem(fig, 0.72, 1.08, 0.82, "life")
    M.gem(fig, 1.28, 0.92, 0.82, "mana")
    return fig


def blink(rng):
    fig = item_fig("blink")
    M.trail(fig, [(0.34, 0.5), (0.5, 0.56), (0.66, 0.72), (0.8, 0.94), (0.98, 1.06)], "violet", w0=0.02, w1=0.2)
    M.dagger(fig, 1.16, 1.18, 1.15, rot=-45, blade="steel", hilt="violet")
    M.sparkle(fig, 0.44, 0.8, 0.12, "violet!40!snow")
    M.sparkle(fig, 0.66, 0.4, 0.08, "violet!40!snow")
    return fig


# ---- secret -------------------------------------------------------------------------

def hyperstone(rng):
    fig = item_fig("hyperstone")
    with fig.frame(1.0, 1.0, s=1.25):
        fig.poly([(0, 0.52), (0.32, 0.0), (0, -0.52), (-0.32, 0.0)], "spark")
        fig.poly([(0, 0.52), (0.12, 0.0), (0, -0.52), (-0.32, 0.0)], "flamehot")
    M.ripple(fig, 1.0, 1.0, 0.62, 0.22, role="ink", width=4.4, a0=200, a1=340)
    M.ripple(fig, 1.0, 1.0, 0.62, 0.22, role="spark", width=2.2, a0=200, a1=340)
    return fig


def demon_edge(rng):
    fig = item_fig("demon_edge")
    with fig.frame(1.0, 1.0, s=1.0, rot=-45):
        fig.poly([(-0.1, -0.24), (0.16, -0.24), (0.24, 0.3), (0.12, 0.84), (-0.04, 0.5), (-0.14, 0.2)], "night")
        fig.poly([(0.16, -0.24), (0.24, 0.3), (0.12, 0.84), (0.14, 0.3)], "life")
        fig.rect(-0.3, -0.34, 0.34, -0.24, "iron", r=0.03)
        fig.rect(-0.05, -0.68, 0.07, -0.34, "leather")
        fig.circle(0.01, -0.72, 0.07, "life")
    return fig


def eaglesong(rng):
    fig = item_fig("eaglesong")
    with fig.frame(1.0, 1.0, s=1.35, rot=-35):
        fig.blob([(0, 0.6), (0.18, 0.36), (0.2, -0.1), (0.04, -0.4), (-0.04, -0.4), (-0.2, -0.1), (-0.16, 0.36)],
                 "snow", tension=0.5)
        fig.stroke([(0, -0.6), (0, 0.5)], width=THIN * 1.6)
        for t in (-0.05, 0.12, 0.28):
            fig.stroke([(0, t), (0.16, t + 0.12)], width=THIN)
            fig.stroke([(0, t), (-0.16, t + 0.12)], width=THIN)
        fig.blob([(0, 0.6), (0.12, 0.44), (-0.1, 0.44)], "golddark", tension=0.3)
    return fig


def reaver(rng):
    fig = item_fig("reaver")
    with fig.frame(1.0, 1.0, s=1.2, rot=-20):
        fig.rect(-0.06, -0.62, 0.06, 0.56, "wooddark", r=0.03)
        edge = [(0.16 + 0.42 * math.cos(math.radians(a)), 0.2 + 0.46 * math.sin(math.radians(a))) for a in range(-75, 76, 15)]
        fig.poly([(0.0, 0.02)] + edge + [(0.0, 0.38)], "steeldark")
        edge2 = [(-0.12 - 0.3 * math.cos(math.radians(a)), 0.2 + 0.3 * math.sin(math.radians(a))) for a in range(-70, 71, 20)]
        fig.poly([(0.0, 0.08)] + edge2 + [(0.0, 0.32)], "steeldark")
        fig.circle(0, 0.2, 0.08, "life")
    return fig


def mystic(rng):
    fig = item_fig("mystic")
    with fig.frame(1.0, 1.0, s=1.3, rot=-30):
        fig.rect(-0.045, -0.62, 0.045, 0.24, "wood!60!snow", r=0.03)
        crescent(fig, 0.0, 0.48, 0.32, "spark", rot=90)
    M.sparkle(fig, 1.48, 1.42, 0.1, "snow")
    return fig


def relic(rng):
    fig = item_fig("relic")
    with fig.frame(1.0, 0.98, s=1.3):
        fig.poly([(-0.34, -0.52), (0.34, -0.52), (0.24, -0.38), (-0.24, -0.38)], "golddark")
        fig.blob([(-0.2, -0.38), (0.2, -0.38), (0.22, 0.0), (0.12, 0.12), (-0.12, 0.12), (-0.22, 0.0)], "gold", tension=0.4)
        M.flame(fig, 0.0, 0.06, 0.5)
    return fig


def talisman(rng):
    fig = item_fig("talisman")
    fig.stroke([(0.56, 1.66), (1.0, 1.42), (1.44, 1.66)], width=1.8, role="ink", smooth=True)
    fig.circle(1.0, 0.92, 0.5, "gold")
    fig.circle(1.0, 0.92, 0.38, "night")
    M.flame(fig, 1.0, 0.6, 0.62, outer="storm", inner="snow")
    fig.circle(1.0, 1.42, 0.07, "golddark")
    return fig


_ALL = [tango, salve, clarity, tp, observer, sentry, bottle,
        branch, gauntlets, slippers, mantle, circlet, ring_protection, stout_shield, gloves, boots, ring_regen,
        sobi, magic_stick,
        ogre_club, alacrity, wizardry, ultimate_orb, elvenskin,
        blades, quarterstaff, chainmail, helm_iron_will, broadsword, claymore, platemail, javelin, mithril, mask_death,
        void_stone, ring_health, vitality, energy, point_booster, blink,
        hyperstone, demon_edge, eaglesong, reaver, mystic, relic, talisman]

PICTURES = {f"items/{fn.__name__}": fn for fn in _ALL}
PICTURES["items/belt"] = belt_item
PICTURES["items/robe"] = robe_item
