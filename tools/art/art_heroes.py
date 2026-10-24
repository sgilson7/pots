"""Hero portraits: web/art/heroes/<hero id>.png, a head-and-shoulders bust on
a round badge in the hero's element (tools/art/icons.py). Each bust shows the
look tools/modelgen.py gives the hero's model."""
import math

from icons import M, hero_fig

C = 1.0


def shoulders(fig, fill, w=0.72, top=0.62):
    """The upper body: a wide rounded mound rising from below the field."""
    fig.blob([(C - w, 0.0), (C - w, top - 0.12), (C - w + 0.18, top), (C + w - 0.18, top),
              (C + w, top - 0.12), (C + w, 0.0)], fill, tension=0.4)


def eyes(fig, y, dx=0.1, x=C, r=(0.035, 0.05), fill="ink"):
    if fill == "ink":   # the outline already widens an ink dot by half the ink width
        r = (max(r[0] - 0.02, 0.008), max(r[1] - 0.02, 0.012))
    fig.ellipse(x - dx, y, r[0], r[1], fill)
    fig.ellipse(x + dx, y, r[0], r[1], fill)


def ironvow(rng):
    fig = hero_fig("ironvow")
    with M.clipped(fig):
        shoulders(fig, "steel", top=0.6)
        fig.rect(0.86, 0.0, 1.14, 0.56, "life")
        for s in (-1, 1):
            fig.ellipse(C + s * 0.52, 0.52, 0.3, 0.2, "steeldark", rot=s * 12)
            fig.ellipse(C + s * 0.52, 0.56, 0.24, 0.13, "steel", rot=s * 12)
        # crest plume
        fig.blob([(0.92, 1.42), (0.96, 1.72), (1.12, 1.82), (1.32, 1.72), (1.2, 1.62), (1.1, 1.44)], "life", tension=0.5)
        # great helm
        fig.blob([(0.7, 0.62), (0.68, 1.06), (0.78, 1.36), (1.0, 1.46), (1.22, 1.36), (1.32, 1.06), (1.3, 0.62)],
                 "steel", tension=0.4)
        fig.rect(0.74, 1.0, 1.26, 1.1, "ink")
        fig.rect(0.965, 0.68, 1.035, 1.0, "ink")
        fig.stroke([(1.0, 1.44), (1.0, 1.12)], width=M.THIN, role="steeldark")
        fig.rect(0.7, 0.6, 1.3, 0.7, "gold", r=0.03)
        for x in (0.8, 0.88, 1.12, 1.2):
            fig.circle(x, 0.84, 0.022, "ink")
        M.shine(fig, 0.82, 1.28, 0.07, 0.035, rot=40)
    return fig


def rime(rng):
    fig = hero_fig("rime")
    with M.clipped(fig):
        shoulders(fig, "cloth", top=0.58)
        # hood: a tall rounded point falling to the shoulders
        fig.blob([(0.52, 0.42), (0.56, 0.94), (0.72, 1.36), (0.98, 1.66), (1.16, 1.58), (1.36, 1.3),
                  (1.48, 0.9), (1.48, 0.42)], "mana", tension=0.5)
        fig.ellipse(C, 0.94, 0.26, 0.3, "mana!60!night")
        fig.ellipse(C, 0.9, 0.23, 0.27, "skin")
        eyes(fig, 0.94, dx=0.085, r=(0.025, 0.035))
        fig.stroke([(0.95, 0.8), (1.05, 0.8)], width=M.THIN)
        # fur collar
        with fig.union("snow"):
            for i in range(7):
                fig.circle(0.58 + i * 0.14, 0.56 + (0.03 if i % 2 else 0), 0.11)
        M.crystal(fig, 1.0, 1.3, 0.2, "ice")
    return fig


def cinder(rng):
    fig = hero_fig("cinder")
    with M.clipped(fig):
        # flame hair behind the head
        M.flame(fig, 1.0, 0.72, 1.0, outer="ember", inner="flame", core=True)
        M.flame(fig, 0.74, 0.86, 0.6, outer="flame", inner="flamehot")
        M.flame(fig, 1.26, 0.86, 0.6, outer="flame", inner="flamehot")
        shoulders(fig, "life", w=0.6, top=0.5)
        fig.rect(0.86, 0.38, 1.14, 0.62, "skin")
        fig.ellipse(C, 0.96, 0.24, 0.28, "skin")
        # fringe of flame across the brow
        fig.blob([(0.76, 1.04), (0.84, 1.24), (0.98, 1.26), (1.1, 1.22), (1.24, 1.08), (1.1, 1.12),
                  (1.0, 1.08), (0.88, 1.14)], "flame", tension=0.5)
        eyes(fig, 0.98, dx=0.09)
        fig.stroke([(0.94, 0.82), (1.0, 0.8), (1.06, 0.82)], width=M.THIN)
        fig.rect(0.72, 0.42, 1.28, 0.5, "gold", r=0.03)
        for s in (-1, 1):
            M.flame(fig, C + s * 0.5, 0.36, 0.34, outer="flame", inner="flamehot")
    return fig


def thistle(rng):
    fig = hero_fig("thistle")
    with M.clipped(fig):
        # quiver over the left shoulder with frost-tipped arrows
        with fig.frame(0.5, 0.5, rot=24):
            fig.rect(-0.1, 0.0, 0.1, 0.62, "leather", r=0.04)
            for dx, tip in ((-0.05, 0.98), (0.05, 0.92)):
                fig.rect(dx - 0.02, 0.6, dx + 0.02, tip - 0.06, "wood")
                fig.poly([(dx - 0.06, tip - 0.08), (dx + 0.06, tip - 0.08), (dx, tip + 0.04)], "ice")
        shoulders(fig, "moss", top=0.56)
        # deep pointed hood
        fig.blob([(0.56, 0.44), (0.58, 0.9), (0.72, 1.3), (1.0, 1.66), (1.24, 1.36), (1.4, 1.0),
                  (1.44, 0.44)], "leaf", tension=0.5)
        fig.blob([(0.74, 0.8), (0.78, 1.08), (1.0, 1.24), (1.22, 1.08), (1.26, 0.8), (1.0, 0.64)], "shade", tension=0.6)
        fig.blob([(0.8, 0.84), (0.82, 0.96), (1.18, 0.96), (1.2, 0.84), (1.0, 0.66)], "skin", tension=0.6)
        eyes(fig, 0.98, dx=0.1, r=(0.05, 0.03), fill="ice")
        fig.stroke([(0.72, 1.3), (1.0, 1.62)], width=M.THIN, role="moss")
        M.leaf(fig, 1.2, 0.5, 0.26, rot=-30)
    return fig


def gristle(rng):
    fig = hero_fig("gristle")
    with M.clipped(fig):
        # a huge pale body; the head small on top
        fig.blob([(0.12, 0.0), (0.18, 0.58), (0.46, 0.92), (1.0, 1.02), (1.54, 0.92), (1.82, 0.58),
                  (1.88, 0.0)], "skinpale", tension=0.5)
        fig.rect(0.68, 0.0, 1.32, 0.42, "cloth!70!leather", r=0.04)
        fig.rect(0.76, 0.42, 0.84, 0.86, "leather")
        fig.rect(1.16, 0.42, 1.24, 0.86, "leather")
        # stitches across the chest
        fig.stroke([(0.34, 0.52), (0.62, 0.74)], width=1.6)
        for t in (0.2, 0.5, 0.8):
            x, y = 0.34 + 0.28 * t, 0.52 + 0.22 * t
            fig.stroke([(x - 0.04, y + 0.05), (x + 0.04, y - 0.05)], width=1.4)
        fig.stroke([(1.4, 0.7), (1.64, 0.48)], width=1.6)
        for t in (0.25, 0.75):
            x, y = 1.4 + 0.24 * t, 0.7 - 0.22 * t
            fig.stroke([(x - 0.04, y - 0.05), (x + 0.04, y + 0.05)], width=1.4)
        fig.circle(C, 1.18, 0.24, "skinpale")
        eyes(fig, 1.2, dx=0.08, r=(0.03, 0.03))
        fig.stroke([(0.9, 1.06), (1.0, 1.1), (1.1, 1.06)], width=M.THIN)
        fig.stroke([(0.86, 1.36), (1.06, 1.3)], width=1.4)
        fig.stroke([(0.92, 1.38), (0.92, 1.29)], width=1.2)
        fig.stroke([(1.0, 1.36), (1.0, 1.27)], width=1.2)
        # the hook on its chain over the right shoulder
        M.chain(fig, [(1.5, 1.05), (1.56, 0.62)], r=0.05)
        M.hook(fig, 1.6, 0.42, 0.45, rot=-10)
        fig.blob([(1.02, 0.3), (1.08, 0.36), (1.12, 0.24), (1.06, 0.18)], "blood", tension=0.6)
    return fig


def tectus(rng):
    fig = hero_fig("tectus")
    with M.clipped(fig):
        shoulders(fig, "moss", w=0.74, top=0.56)
        for s in (-1, 1):
            M.rock(fig, C + s * 0.5, 0.5, 0.42)
        # horns
        for s in (-1, 1):
            with fig.frame(C, 1.0, flip=(s < 0)):
                fig.blob([(0.2, 0.14), (0.42, 0.18), (0.6, 0.34), (0.66, 0.58), (0.54, 0.4), (0.36, 0.3),
                          (0.2, 0.28)], "bone", tension=0.5)
        # stone head and muzzle
        fig.blob([(0.72, 0.68), (0.7, 1.1), (0.82, 1.3), (1.18, 1.3), (1.3, 1.1), (1.28, 0.68)], "stone", tension=0.4)
        fig.ellipse(C, 0.78, 0.22, 0.15, "stone!60!skin")
        fig.ellipse(0.92, 0.78, 0.035, 0.03, "ink")
        fig.ellipse(1.08, 0.78, 0.035, 0.03, "ink")
        fig.stroke([(0.8, 1.12), (0.94, 1.04)], width=1.6)
        fig.stroke([(1.2, 1.12), (1.06, 1.04)], width=1.6)
        eyes(fig, 1.0, dx=0.11, r=(0.035, 0.035), fill="flamehot")
        fig.stroke([(1.18, 1.28), (1.12, 1.18), (1.18, 1.1)], width=M.THIN, role="stonedark")
    return fig


def kestrel(rng):
    fig = hero_fig("kestrel")
    with M.clipped(fig):
        shoulders(fig, "flame!70!ember", top=0.54)
        fig.poly([(0.7, 0.54), (1.0, 0.0), (1.08, 0.0), (0.82, 0.54)], "leather")
        # plume from the top of the mask
        fig.blob([(0.96, 1.34), (0.84, 1.6), (0.74, 1.76), (0.92, 1.68), (1.06, 1.42)], "blood", tension=0.5)
        fig.rect(0.9, 0.5, 1.1, 0.7, "skin")
        # the wide white mask
        fig.blob([(0.5, 0.96), (0.56, 1.2), (0.8, 1.38), (1.2, 1.38), (1.44, 1.2), (1.5, 0.96),
                  (1.36, 0.7), (1.0, 0.6), (0.64, 0.7)], "snow", tension=0.45)
        fig.rect(0.6, 1.0, 1.4, 1.08, "ink", r=0.03)
        fig.stroke([(1.0, 1.36), (1.0, 1.12)], width=M.THIN)
        for x in (0.82, 1.18):
            fig.stroke([(x - 0.06, 0.82), (x + 0.06, 0.82)], width=1.6, role="blood")
        for i in range(4):
            fig.stroke([(0.86 + i * 0.09, 0.72), (0.86 + i * 0.09, 0.66)], width=1.2)
    return fig


def hexwright(rng):
    fig = hero_fig("hexwright")
    with M.clipped(fig):
        # bone spikes rising from the shoulders
        for s in (-1, 1):
            for dx, h, w in ((0.42, 0.62, 0.06), (0.6, 0.48, 0.05)):
                x = C + s * dx
                fig.poly([(x - w, 0.5), (x + s * 0.1, 0.5 + h), (x + w, 0.5)], "bone")
        shoulders(fig, "arcane!80!night", top=0.58)
        # pointed cowl and a gaunt face
        fig.blob([(0.66, 0.6), (0.68, 1.1), (0.84, 1.42), (1.0, 1.7), (1.16, 1.42), (1.32, 1.1), (1.34, 0.6)],
                 "violet!70!arcane", tension=0.45)
        fig.blob([(0.82, 1.06), (0.86, 1.24), (1.0, 1.32), (1.14, 1.24), (1.18, 1.06), (1.08, 0.7),
                  (1.0, 0.6), (0.92, 0.7)], "skingrey", tension=0.5)
        fig.poly([(0.86, 1.08), (0.98, 1.04), (0.9, 1.0)], "violet")
        fig.poly([(1.14, 1.08), (1.02, 1.04), (1.1, 1.0)], "violet")
        fig.stroke([(0.9, 0.9), (0.96, 0.82)], width=M.THIN)
        fig.stroke([(1.1, 0.9), (1.04, 0.82)], width=M.THIN)
        fig.stroke([(0.94, 0.72), (1.06, 0.72)], width=M.THIN)
        M.sparkle(fig, 1.42, 1.3, 0.1, "violet")
    return fig


def longwick(rng):
    fig = hero_fig("longwick")
    with M.clipped(fig):
        shoulders(fig, "leather", top=0.48)
        fig.circle(C, 0.88, 0.28, "skin")
        # the big white beard
        fig.blob([(0.7, 0.86), (0.72, 0.56), (0.86, 0.3), (1.0, 0.22), (1.14, 0.3), (1.28, 0.56), (1.3, 0.86),
                  (1.12, 0.8), (1.0, 0.84), (0.88, 0.8)], "snow", tension=0.5)
        fig.ellipse(C, 0.86, 0.08, 0.07, "life!60!skin")
        fig.stroke([(0.86, 0.72), (0.92, 0.7)], width=M.THIN)
        fig.stroke([(1.14, 0.72), (1.08, 0.7)], width=M.THIN)
        # goggles
        fig.rect(0.7, 0.98, 1.3, 1.06, "leather")
        for x in (0.88, 1.12):
            fig.circle(x, 1.02, 0.11, "gold")
            fig.circle(x, 1.02, 0.07, "icedeep")
            M.shine(fig, x - 0.03, 1.05, 0.025, 0.015)
        # big hat: brim and a tall slouched crown
        fig.blob([(0.78, 1.16), (0.8, 1.42), (0.96, 1.56), (1.12, 1.54), (1.24, 1.44), (1.22, 1.16)],
                 "leather", tension=0.5)
        fig.ellipse(C, 1.16, 0.56, 0.11, "wooddark")
        fig.rect(0.79, 1.2, 1.21, 1.28, "life")
    return fig


def tempest(rng):
    fig = hero_fig("tempest")
    with M.clipped(fig):
        shoulders(fig, "mana", top=0.52)
        fig.poly([(0.84, 0.52), (1.0, 0.0), (1.16, 0.52)], "cloud")
        fig.circle(C, 1.04, 0.26, "skin")
        # the long beard, falling to the chest
        fig.blob([(0.76, 1.0), (0.76, 0.74), (0.86, 0.5), (1.0, 0.28), (1.14, 0.5), (1.24, 0.74), (1.24, 1.0),
                  (1.12, 0.9), (1.0, 0.94), (0.88, 0.9)], "cloud", tension=0.5)
        fig.blob([(0.86, 0.96), (0.94, 1.0), (1.0, 0.94), (1.06, 1.0), (1.14, 0.96), (1.06, 0.9), (0.94, 0.9)],
                 "cloud", tension=0.6)
        fig.stroke([(0.86, 1.12), (0.96, 1.09)], width=1.6, role="ink")
        fig.stroke([(1.14, 1.12), (1.04, 1.09)], width=1.6, role="ink")
        eyes(fig, 1.06, dx=0.09, r=(0.03, 0.025), fill="spark")
        # the cloud crown
        M.cloud(fig, C, 1.26, 0.7, "storm!60!steel")
        M.bolt(fig, 1.36, 1.5, 0.3, rot=-20)
        M.bolt(fig, 0.6, 0.62, 0.32, rot=15)
        M.bolt(fig, 1.42, 0.62, 0.32, rot=-15)
    return fig


def hackle(rng):
    """Hackle (round 7, after Troll Warlord): a boar brawler, bristle crest,
    tusks, a hatchet over each shoulder."""
    fig = hero_fig("hackle")
    hide = "violet!35!skin"
    with M.clipped(fig):
        shoulders(fig, "leaf!70!moss", top=0.56)
        for i in range(5):
            x = 0.7 + i * 0.15
            fig.poly([(x - 0.04, 0.5), (x + 0.04, 0.5), (x, 0.38)], "bone")
        # a hatchet up behind each shoulder
        M.axe(fig, 0.42, 0.86, 0.8, rot=25)
        M.axe(fig, 1.58, 0.86, 0.8, rot=-25)
        # ears and the bristle crest
        for sgn in (-1, 1):
            fig.poly([(C + sgn * 0.26, 1.24), (C + sgn * 0.46, 1.5), (C + sgn * 0.38, 1.16)], hide)
        for i in range(5):
            x = 0.8 + i * 0.1
            fig.poly([(x - 0.06, 1.3), (x + 0.06, 1.3), (x, 1.62 - abs(i - 2) * 0.06)], "flame")
        # the head, snout and tusks
        fig.circle(C, 1.0, 0.36, hide)
        fig.ellipse(C, 0.86, 0.2, 0.14, "violet!20!skin")
        fig.ellipse(0.94, 0.86, 0.03, 0.04, "ink")
        fig.ellipse(1.06, 0.86, 0.03, 0.04, "ink")
        for sgn in (-1, 1):
            fig.poly([(C + sgn * 0.14, 0.8), (C + sgn * 0.2, 0.8), (C + sgn * 0.28, 1.0)], "bone")
            fig.rect(C + sgn * 0.2 - 0.07, 1.05, C + sgn * 0.2 + 0.07, 1.08, "flame")
        eyes(fig, 1.12, dx=0.13)
    return fig


def chipper(rng):
    """Chipper (round 7, after Timbersaw): a beaver lumberjack, hard hat,
    front teeth, the saw blade on his chest and a chain claw."""
    fig = hero_fig("chipper")
    with M.clipped(fig):
        shoulders(fig, "wood", top=0.58)
        fig.rect(0.3, 0.3, 1.7, 0.36, "wooddark")
        # the saw blade on the chest
        teeth = []
        for i in range(24):
            a = math.radians(i * 15)
            r = 0.26 if i % 2 == 0 else 0.2
            teeth.append((C + r * math.cos(a), 0.3 + r * math.sin(a)))
        fig.poly(teeth, "steel")
        fig.circle(C, 0.3, 0.06, "gold")
        # the chain claw over the left shoulder
        M.chain(fig, [(0.34, 0.6), (0.4, 1.0)], r=0.05)
        M.hook(fig, 0.42, 1.12, 0.36, rot=170)
        # round ears, the head, muzzle and two big teeth
        for sgn in (-1, 1):
            fig.circle(C + sgn * 0.3, 1.22, 0.09, "wood!80!leather")
        fig.circle(C, 0.98, 0.34, "wood!80!leather")
        fig.ellipse(C, 0.84, 0.2, 0.13, "skin!70!wood")
        fig.ellipse(C, 0.92, 0.05, 0.035, "ink")
        fig.rect(0.95, 0.66, 0.995, 0.78, "snow")
        fig.rect(1.005, 0.66, 1.05, 0.78, "snow")
        eyes(fig, 1.04, dx=0.12)
        # the hard hat
        fig.ellipse(C, 1.24, 0.42, 0.06, "gold")
        fig.blob([(0.7, 1.24), (0.74, 1.42), (1.0, 1.5), (1.26, 1.42), (1.3, 1.24)], "gold", tension=0.4)
    return fig


def shiver(rng):
    """Shiver (round 7, after Ancient Apparition): an old snow owl, a round
    head with a pale facial disc, two wide frost eyes, icicle ear tufts,
    brows and a mantle in the side's colour, folded wings of ice."""
    fig = hero_fig("shiver")
    plume = "icedeep!22!snow"
    with M.clipped(fig):
        # folded wings behind the body, icicle tips at the bottom
        for sgn in (-1, 1):
            fig.blob([(C + sgn * 0.3, 0.0), (C + sgn * 0.78, 0.1), (C + sgn * 0.8, 0.5), (C + sgn * 0.5, 0.66)], "icedeep!70!snow", tension=0.4)
            for i in range(3):
                x = C + sgn * (0.48 + i * 0.1)
                fig.poly([(x - 0.04, 0.16), (x + 0.04, 0.16), (x, -0.02)], "icedeep")
        shoulders(fig, plume, w=0.5, top=0.6)
        # chevron feathers on the breast
        for row in range(2):
            for i in range(3):
                x, y = 0.8 + i * 0.2 + row * 0.1, 0.18 + row * 0.16
                fig.poly([(x - 0.05, y + 0.04), (x, y - 0.02), (x + 0.05, y + 0.04)], "icedeep")
        # the mantle
        fig.ellipse(C, 0.6, 0.42, 0.08, "gold")
        # icicle ear tufts, raking out
        for sgn in (-1, 1):
            fig.poly([(C + sgn * 0.16, 1.3), (C + sgn * 0.34, 1.2), (C + sgn * 0.56, 1.72)], "ice")
            fig.poly([(C + sgn * 0.3, 1.24), (C + sgn * 0.4, 1.18), (C + sgn * 0.62, 1.48)], "snow")
        # the head and its facial disc
        fig.circle(C, 0.98, 0.46, plume)
        with fig.union("snow"):
            fig.ellipse(C - 0.19, 0.98, 0.21, 0.24)
            fig.ellipse(C + 0.19, 0.98, 0.21, 0.24)
        for sgn in (-1, 1):
            fig.circle(C + sgn * 0.19, 1.0, 0.1, "ice")
            fig.circle(C + sgn * 0.17, 1.0, 0.015, "ink")
            fig.poly([(C + sgn * 0.06, 1.2), (C + sgn * 0.34, 1.28), (C + sgn * 0.32, 1.2)], "gold")
        # the beak
        fig.poly([(C - 0.06, 0.9), (C + 0.06, 0.9), (C, 0.76)], "stonedark")
    return fig


def tallow(rng):
    """Tallow (round 8, after Vengeful Spirit): a lantern moth, a round
    fuzzy head with two big dark eyes, feathered antennae, a ruff in the
    side's colour and dusk wings with amber eye spots."""
    fig = hero_fig("tallow")
    fuzz = "cloth!70!snow"
    with M.clipped(fig):
        # the wings, spread behind
        for sgn in (-1, 1):
            fig.blob([(C + sgn * 0.2, 0.5), (C + sgn * 0.86, 0.36), (C + sgn * 0.96, 0.92), (C + sgn * 0.62, 1.22), (C + sgn * 0.3, 0.96)],
                     "violet!60!night", tension=0.45)
            fig.blob([(C + sgn * 0.3, 0.6), (C + sgn * 0.78, 0.5), (C + sgn * 0.84, 0.88), (C + sgn * 0.58, 1.08), (C + sgn * 0.36, 0.9)],
                     "violet", tension=0.45)
            fig.circle(C + sgn * 0.66, 0.8, 0.12, "gold")
            fig.circle(C + sgn * 0.66, 0.8, 0.06, "flamehot")
        shoulders(fig, fuzz, w=0.44, top=0.6)
        for y in (0.16, 0.32):
            fig.ellipse(C, y, 0.4, 0.035, "violet!50!cloth")
        # the ruff
        fig.ellipse(C, 0.6, 0.38, 0.1, "gold")
        # feathered antennae
        for sgn in (-1, 1):
            fig.stroke([(C + sgn * 0.12, 1.32), (C + sgn * 0.26, 1.6), (C + sgn * 0.4, 1.74)], width=M.THIN, smooth=True)
            for i in range(3):
                x, y = C + sgn * (0.2 + i * 0.07), 1.5 + i * 0.08
                fig.ellipse(x + sgn * 0.05, y, 0.06, 0.02, "violet", rot=sgn * 30)
        # the head and its eyes
        fig.circle(C, 1.0, 0.36, fuzz)
        for sgn in (-1, 1):
            fig.ellipse(C + sgn * 0.15, 1.02, 0.12, 0.15, "night")
            fig.circle(C + sgn * 0.12, 1.08, 0.035, "flamehot")
        fig.stroke([(C - 0.06, 0.82), (C, 0.79), (C + 0.06, 0.82)], width=M.THIN, smooth=True)
    return fig


def skipjack(rng):
    """Skipjack (round 8, after Mirana): a pond frog, a wide head with its
    eyes on top, a long smile, a hood of night cloth sewn with stars and a
    reed bow over one shoulder."""
    fig = hero_fig("skipjack")
    frog, belly = "leaf!80!snow", "venom!30!snow"
    with M.clipped(fig):
        # the bow behind, over the right shoulder
        fig.stroke([(C + 0.62, 0.1), (C + 0.82, 0.8), (C + 0.6, 1.5)], width=3.2, role="moss", smooth=True)
        fig.stroke([(C + 0.62, 0.1), (C + 0.6, 1.5)], width=M.THIN, role="cloth")
        # the hood and cape
        fig.blob([(C - 0.7, 0.0), (C - 0.62, 0.8), (C - 0.36, 1.36), (C, 1.5), (C + 0.36, 1.36), (C + 0.62, 0.8), (C + 0.7, 0.0)], "night", tension=0.45)
        for x, y in ((C - 0.42, 1.0), (C + 0.44, 0.9), (C - 0.3, 1.3), (C + 0.26, 1.34), (C - 0.5, 0.5)):
            M.star(fig, x, y, 0.05, "spark")
        shoulders(fig, frog, w=0.46, top=0.5)
        fig.ellipse(C, 0.26, 0.26, 0.22, belly)
        fig.ellipse(C, 0.52, 0.42, 0.06, "gold")
        # the wide head, the eyes up on top
        fig.ellipse(C, 0.86, 0.42, 0.3, frog)
        fig.ellipse(C, 0.76, 0.34, 0.14, belly)
        for sgn in (-1, 1):
            fig.circle(C + sgn * 0.2, 1.12, 0.14, frog)
            fig.circle(C + sgn * 0.2, 1.14, 0.1, "snow")
            fig.circle(C + sgn * 0.21, 1.14, 0.03, "ink")
        fig.stroke([(C - 0.24, 0.86), (C, 0.8), (C + 0.24, 0.86)], width=M.THIN, smooth=True)
    return fig


PICTURES = {f"heroes/{f.__name__}": f for f in
            (ironvow, rime, cinder, thistle, gristle, tectus, kestrel, hexwright, longwick, tempest, hackle, chipper, shiver,
             tallow, skipjack)}
