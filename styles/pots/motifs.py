"""Motifs for the Protection of the Spheres style pack: the icon badge and a
kit of small subjects (blades, stars, flames, bolts, gems ...). Every helper
takes an artlib.Fig and role names, never colour values.

Coordinates: an icon is 2 x 2 cm with its centre at (1, 1). The badge's
outer edge has radius R_OUT and its field (the tinted middle) radius R_IN.
Subjects are drawn about their own origin, about 1 unit tall, and placed
with `with fig.frame(x, y, s=..., rot=...)`; most helpers take (x, y, s)
and do that themselves.

    import style
    m = style.load().motifs()
    fig = Fig("icon")
    m.badge(fig, "frost")            # round badge, frost rim
    m.sword(fig, 1, 1, 0.8, rot=-45)
"""
import math
from contextlib import contextmanager

C = 1.0          # centre of a 2 cm icon
R_OUT = 0.9      # badge outer radius (keeps 6 px clear of every edge)
R_IN = 0.74      # field radius
SQ_OUT = (0.12, 1.88, 0.32)   # rounded square: low, high, corner radius
SQ_IN = (0.27, 1.73, 0.2)
THIN = 1.2       # pt, interior detail lines


def field_of(rim):
    """The badge's field colour: the rim role washed toward paper."""
    return f"{rim}!34!paper"


# ---- the badge --------------------------------------------------------------

def badge(fig, rim, shape="round", field=None):
    """The badge behind every icon: a rim in `rim` and a pale field. shape is
    "round" (abilities, heroes) or "square" (items, a rounded square)."""
    field = field or field_of(rim)
    if shape == "round":
        fig.circle(C, C, R_OUT, rim)
        fig.circle(C, C, R_IN, field)
    else:
        lo, hi, r = SQ_OUT
        fig.rect(lo, lo, hi, hi, rim, r=r)
        lo, hi, r = SQ_IN
        fig.rect(lo, lo, hi, hi, field, r=r)
    # a small flat glint on the rim, top left
    a0, a1 = 118, 152
    if shape == "round":
        rr = (R_OUT + R_IN) / 2
        pts = [(C + rr * math.cos(math.radians(a)), C + rr * math.sin(math.radians(a)))
               for a in range(a0, a1 + 1, 4)]
    else:
        y = (SQ_OUT[1] + SQ_IN[1]) / 2
        pts = [(0.62, y), (0.95, y)]
    fig.stroke(pts, width=2.0, role=f"{rim}!45!paper")


def _inner_path(fig, shape):
    from artlib import f
    if shape == "round":
        X, Y = fig.pt(C, C)
        return f"({f(X)},{f(Y)}) circle ({f(R_IN * fig.k())})"
    lo, hi, r = SQ_IN
    return f"[rounded corners={f(r)}cm] ({f(lo)},{f(lo)}) rectangle ({f(hi)},{f(hi)})"


@contextmanager
def clipped(fig, shape="round", rim=None):
    """Draw the block clipped to the badge's field; afterwards the field's
    edge is inked again so the clipped shapes meet the rim cleanly. Used for
    busts and backdrops that should not spill over the rim."""
    fig.lines.append("\\begin{scope}")
    fig.lines.append(f"\\clip {_inner_path(fig, shape)};")
    try:
        yield fig
    finally:
        fig.lines.append("\\end{scope}")
        fig.lines.append(f"\\draw[inked] {_inner_path(fig, shape)};")


# ---- small helpers ------------------------------------------------------------

def _ring_pts(x, y, r, n, start=90.0):
    return [(x + r * math.cos(math.radians(start + 360.0 * i / n)),
             y + r * math.sin(math.radians(start + 360.0 * i / n))) for i in range(n)]


def star(fig, x, y, r, fill="spark", points=5, inner=0.48, rot=0.0):
    """A pointed star (a stun star, a sparkle)."""
    pts = []
    for i in range(points * 2):
        rr = r if i % 2 == 0 else r * inner
        a = math.radians(90 + rot + 180.0 * i / points)
        pts.append((x + rr * math.cos(a), y + rr * math.sin(a)))
    fig.poly(pts, fill)


def stun_stars(fig, x, y, s=1.0, fill="spark"):
    """Three small stars in a ring: the stun mark."""
    with fig.frame(x, y, s=s):
        fig.ellipse(0, 0, 0.42, 0.13, "spark!40!paper")
        star(fig, -0.36, 0.02, 0.16, fill, rot=-12)
        star(fig, 0.0, 0.12, 0.2, fill)
        star(fig, 0.36, 0.02, 0.16, fill, rot=12)


def sparkle(fig, x, y, r, fill="snow"):
    """A four-pointed glint."""
    star(fig, x, y, r, fill, points=4, inner=0.32)


def burst(fig, x, y, r, fill="flamehot", n=9, inner=0.62, rot=0.0):
    """A spiky impact burst."""
    star(fig, x, y, r, fill, points=n, inner=inner, rot=rot)


def shine(fig, x, y, rx=0.08, ry=0.04, rot=30.0, fill="snow"):
    """A small flat highlight."""
    fig.ellipse(x, y, rx, ry, fill, rot=rot)


def motion(fig, lines, width=2.0, role="ink"):
    """Speed lines: a list of ((x0, y0), (x1, y1)) strokes."""
    for a, b in lines:
        fig.stroke([a, b], width=width, role=role)


def trail(fig, pts, fill, w0=0.02, w1=0.16):
    """A tapering ribbon along pts (thin at the start, wide at the end): a
    blink trail, a gust, a comet tail."""
    left, right = [], []
    n = len(pts)
    for i, (px, py) in enumerate(pts):
        qx, qy = pts[min(i + 1, n - 1)]
        rx, ry = pts[max(i - 1, 0)]
        dx, dy = qx - rx, qy - ry
        L = math.hypot(dx, dy) or 1.0
        nx, ny = -dy / L, dx / L
        w = w0 + (w1 - w0) * i / (n - 1)
        left.append((px + nx * w, py + ny * w))
        right.append((px - nx * w, py - ny * w))
    fig.blob(left + right[::-1], fill, tension=0.4)


# ---- elements ------------------------------------------------------------------

def flame(fig, x, y, s=1.0, outer="flame", inner="flamehot", core=True):
    """A flame about 1 unit tall, base at (x, y)."""
    with fig.frame(x, y, s=s):
        fig.blob([(0, 0), (0.32, 0.12), (0.4, 0.42), (0.26, 0.64), (0.3, 0.86), (0.08, 0.72),
                  (0.0, 1.0), (-0.14, 0.66), (-0.3, 0.74), (-0.38, 0.42), (-0.3, 0.12)], outer, tension=0.55)
        if core:
            fig.blob([(0, 0.06), (0.18, 0.18), (0.2, 0.38), (0.06, 0.6), (-0.1, 0.42), (-0.18, 0.2)], inner, tension=0.6)


def bolt(fig, x, y, s=1.0, fill="spark", rot=0.0):
    """A lightning bolt about 1 unit tall, centred on (x, y)."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.poly([(0.12, 0.5), (-0.22, 0.02), (-0.02, 0.02), (-0.16, -0.5), (0.24, 0.06),
                  (0.04, 0.06), (0.22, 0.5)], fill)


def snowflake(fig, x, y, r, role="snow", width=2.4):
    """A six-armed snowflake of ink-backed strokes."""
    for back, w in (("ink", width + 2.6), (role, width)):
        for i in range(6):
            a = math.radians(90 + 60 * i)
            ex, ey = x + r * math.cos(a), y + r * math.sin(a)
            fig.stroke([(x, y), (ex, ey)], width=w, role=back)
            mx, my = x + 0.58 * r * math.cos(a), y + 0.58 * r * math.sin(a)
            for t in (-1, 1):
                b = a + t * math.radians(40)
                fig.stroke([(mx, my), (mx + 0.3 * r * math.cos(b), my + 0.3 * r * math.sin(b))], width=w, role=back)


def crystal(fig, x, y, s=1.0, fill="ice", rot=0.0, facet="snow"):
    """A long ice shard about 1 unit tall, centred on (x, y)."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.poly([(0, 0.5), (0.16, 0.2), (0.12, -0.38), (0, -0.5), (-0.12, -0.38), (-0.16, 0.2)], fill)
        fig.poly([(0, 0.5), (0.16, 0.2), (0.02, 0.1)], facet)


def drop(fig, x, y, s=1.0, fill="mana"):
    """A droplet about 1 unit tall, centred on (x, y)."""
    with fig.frame(x, y, s=s):
        fig.blob([(0, 0.5), (0.18, 0.12), (0.3, -0.16), (0.2, -0.42), (0, -0.5), (-0.2, -0.42),
                  (-0.3, -0.16), (-0.18, 0.12)], fill, tension=0.6)
        shine(fig, -0.12, -0.18, 0.05, 0.1, rot=-15)


def cloud(fig, x, y, s=1.0, fill="cloud"):
    """A puffy cloud, about 1.2 units wide, base at (x, y)."""
    with fig.frame(x, y, s=s):
        with fig.union(fill):
            for dx, dy, r in ((-0.38, 0.16, 0.22), (-0.1, 0.32, 0.3), (0.24, 0.26, 0.26), (0.46, 0.12, 0.18)):
                fig.circle(dx, dy, r)
            fig.rect(-0.56, -0.02, 0.62, 0.14, r=0.06)


def swirl(fig, x, y, r, role="ink", width=2.2, turns=1.6, start=0.0):
    """A spiral line (wind, dizziness, mana)."""
    pts = []
    n = int(28 * turns)
    for i in range(n + 1):
        t = i / n
        a = math.radians(start + 360 * turns * t)
        rr = r * (0.15 + 0.85 * t)
        pts.append((x + rr * math.cos(a), y + rr * math.sin(a)))
    fig.stroke(pts, width=width, role=role, smooth=True)


def ripple(fig, x, y, rx, ry, role="ink", width=2.0, a0=200, a1=340):
    """An arc of an ellipse: a shock ring, a ground ripple."""
    pts = [(x + rx * math.cos(math.radians(a)), y + ry * math.sin(math.radians(a)))
           for a in range(int(a0), int(a1) + 1, 8)]
    fig.stroke(pts, width=width, role=role, smooth=True)


def heart(fig, x, y, s=1.0, fill="life"):
    """A heart about 1 unit wide, centred on (x, y)."""
    with fig.frame(x, y, s=s):
        fig.blob([(0, -0.46), (0.36, -0.08), (0.48, 0.2), (0.3, 0.42), (0.1, 0.36), (0, 0.22),
                  (-0.1, 0.36), (-0.3, 0.42), (-0.48, 0.2), (-0.36, -0.08)], fill, tension=0.5)
        shine(fig, -0.26, 0.2, 0.08, 0.05, rot=40)


def plus(fig, x, y, s=1.0, fill="heal"):
    """A plus sign (healing)."""
    with fig.frame(x, y, s=s):
        fig.poly([(-0.12, 0.4), (0.12, 0.4), (0.12, 0.12), (0.4, 0.12), (0.4, -0.12), (0.12, -0.12),
                  (0.12, -0.4), (-0.12, -0.4), (-0.12, -0.12), (-0.4, -0.12), (-0.4, 0.12), (-0.12, 0.12)], fill)


def eye(fig, x, y, s=1.0, iris="mana", white="snow"):
    """An open eye about 1 unit wide."""
    with fig.frame(x, y, s=s):
        fig.blob([(-0.5, 0), (-0.2, 0.24), (0.2, 0.24), (0.5, 0), (0.2, -0.24), (-0.2, -0.24)], white, tension=0.5)
        fig.circle(0, 0, 0.19, iris)
        fig.circle(0, 0, 0.08, "ink")
        shine(fig, -0.06, 0.07, 0.04, 0.03)


# ---- objects ---------------------------------------------------------------------

def gem(fig, x, y, s=1.0, fill="mana", facet="snow"):
    """A cut gem about 1 unit wide, centred on (x, y)."""
    with fig.frame(x, y, s=s):
        fig.poly([(-0.5, 0.12), (-0.28, 0.4), (0.28, 0.4), (0.5, 0.12), (0, -0.48)], fill)
        fig.stroke([(-0.5, 0.12), (0.5, 0.12)], width=THIN)
        fig.stroke([(-0.16, 0.12), (0, -0.48), (0.16, 0.12)], width=THIN)
        fig.poly([(-0.28, 0.4), (-0.12, 0.4), (-0.24, 0.16), (-0.42, 0.16)], facet)


def orb(fig, x, y, r, fill="mana", glint="snow"):
    """A round orb with a glint."""
    fig.circle(x, y, r, fill)
    fig.ellipse(x - 0.36 * r, y + 0.36 * r, 0.24 * r, 0.13 * r, glint, rot=40)


def ring(fig, x, y, s=1.0, band="gold", stone="life", tilt=0.0):
    """A finger ring seen at a slant, with a set stone on top."""
    with fig.frame(x, y, s=s, rot=tilt):
        fig.ellipse(0, -0.08, 0.42, 0.34, band)
        fig.ellipse(0, -0.1, 0.27, 0.2, "shade")
        if stone:
            fig.poly([(-0.16, 0.26), (0.16, 0.26), (0.2, 0.4), (-0.2, 0.4)], "golddark")
            fig.circle(0, 0.46, 0.14, stone)
            shine(fig, -0.04, 0.5, 0.04, 0.025)


def potion(fig, x, y, s=1.0, liquid="life", glass="snow!70!ice"):
    """A round-bellied flask with a cork."""
    with fig.frame(x, y, s=s):
        fig.rect(-0.1, 0.18, 0.1, 0.42, glass)
        fig.rect(-0.12, 0.38, 0.12, 0.5, "wood", r=0.03)
        fig.circle(0, -0.12, 0.36, glass)
        fig.blob([(-0.33, -0.12), (0.33, -0.12), (0.26, -0.34), (0, -0.46), (-0.26, -0.34)], liquid, tension=0.5)
        shine(fig, -0.16, 0.04, 0.06, 0.1, rot=-20)


def sword(fig, x, y, s=1.0, rot=0.0, blade="steel", hilt="gold", grip="leather", length=1.0):
    """A straight sword, point up before rot, about 1.25 units long."""
    L = 0.85 * length
    with fig.frame(x, y, s=s, rot=rot):
        fig.poly([(-0.08, -0.2), (0.08, -0.2), (0.08, L - 0.12), (0, L), (-0.08, L - 0.12)], blade)
        fig.stroke([(0, -0.16), (0, L - 0.14)], width=THIN, role="steeldark")
        fig.rect(-0.25, -0.27, 0.25, -0.18, hilt, r=0.03)
        fig.rect(-0.05, -0.5, 0.05, -0.27, grip)
        fig.circle(0, -0.54, 0.07, hilt)


def dagger(fig, x, y, s=1.0, rot=0.0, blade="steel", hilt="golddark", grip="leather"):
    """A short leaf-bladed knife, point up before rot."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.blob([(0, 0.55), (0.1, 0.25), (0.08, -0.05), (-0.08, -0.05), (-0.1, 0.25)], blade, tension=0.5)
        fig.rect(-0.18, -0.12, 0.18, -0.04, hilt, r=0.03)
        fig.rect(-0.045, -0.36, 0.045, -0.12, grip)
        fig.circle(0, -0.4, 0.06, hilt)


def axe(fig, x, y, s=1.0, rot=0.0, head="steel", haft="wood"):
    """A one-bladed axe, haft up the y axis, head to the right."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.05, -0.55, 0.05, 0.5, haft, r=0.03)
        edge = [(0.2 + 0.32 * math.cos(math.radians(a)), 0.18 + 0.4 * math.sin(math.radians(a)))
                for a in range(-70, 71, 14)]
        fig.poly([(0.0, 0.08)] + edge + [(0.0, 0.32)], head)
        fig.stroke([(0.04, 0.12), (0.04, 0.28)], width=THIN, role="steeldark")


def hammer(fig, x, y, s=1.0, rot=0.0, head="steel", haft="wood", band="gold"):
    """A war hammer, haft along y, head across the top."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.05, -0.55, 0.05, 0.3, haft, r=0.03)
        fig.rect(-0.34, 0.2, 0.34, 0.52, head, r=0.05)
        fig.rect(-0.08, 0.2, 0.08, 0.52, band)
        fig.rect(-0.07, -0.58, 0.07, -0.48, band, r=0.02)


def mace(fig, x, y, s=1.0, rot=0.0, head="iron", haft="wood"):
    """A flanged mace."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.05, -0.55, 0.05, 0.2, haft, r=0.03)
        star(fig, 0, 0.32, 0.3, "steeldark", points=6, inner=0.7)
        fig.circle(0, 0.32, 0.19, head)


def arrow(fig, x0, y0, x1, y1, shaft="wood", head="steel", fletch="life", width=None):
    """An arrow from (x0, y0) (fletching) to (x1, y1) (point)."""
    dx, dy = x1 - x0, y1 - y0
    L = math.hypot(dx, dy)
    ang = math.degrees(math.atan2(dy, dx))
    with fig.frame(x0, y0, s=L, rot=ang):
        fig.rect(0.05, -0.025, 0.86, 0.025, shaft)
        fig.poly([(0.8, -0.07), (1.0, 0), (0.8, 0.07)], head)
        fig.poly([(0.0, 0.0), (0.06, 0.09), (0.22, 0.09), (0.18, 0.0)], fletch)
        fig.poly([(0.0, 0.0), (0.06, -0.09), (0.22, -0.09), (0.18, 0.0)], fletch)


def bow(fig, x, y, s=1.0, rot=0.0, wood="wood", string="cloth"):
    """A recurve bow, string on the left, limbs up and down."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.stroke([(-0.12, 0.56), (-0.12, -0.56)], width=1.4, role="ink")
        fig.blob([(-0.12, 0.6), (0.08, 0.5), (0.2, 0.2), (0.22, 0), (0.2, -0.2), (0.08, -0.5), (-0.12, -0.6),
                  (0.02, -0.46), (0.12, -0.2), (0.14, 0), (0.12, 0.2), (0.02, 0.46)], wood, tension=0.4)
        fig.rect(0.13, -0.09, 0.25, 0.09, "leather", r=0.03)


def shield(fig, x, y, s=1.0, fill="steel", trim="golddark", boss="gold"):
    """A heater shield about 1 unit tall."""
    with fig.frame(x, y, s=s):
        fig.blob([(-0.42, 0.42), (0.42, 0.42), (0.4, 0.0), (0.24, -0.3), (0, -0.5), (-0.24, -0.3), (-0.4, 0.0)], trim, tension=0.3)
        fig.blob([(-0.32, 0.33), (0.32, 0.33), (0.3, 0.0), (0.18, -0.24), (0, -0.39), (-0.18, -0.24), (-0.3, 0.0)], fill, tension=0.3)
        if boss:
            fig.circle(0, 0.02, 0.1, boss)


def round_shield(fig, x, y, r, fill="wood", rim="steeldark", boss="steel"):
    """A round shield with a boss."""
    fig.circle(x, y, r, rim)
    fig.circle(x, y, r * 0.8, fill)
    fig.stroke([(x - 0.8 * r, y), (x + 0.8 * r, y)], width=THIN, role="wooddark")
    fig.stroke([(x, y - 0.8 * r), (x, y + 0.8 * r)], width=THIN, role="wooddark")
    fig.circle(x, y, r * 0.28, boss)


def helm(fig, x, y, s=1.0, fill="steel", crest="life", visor=True):
    """A closed great-helm, front view, about 1 unit tall."""
    with fig.frame(x, y, s=s):
        if crest:
            fig.blob([(-0.08, 0.36), (0.0, 0.66), (0.12, 0.6), (0.08, 0.36)], crest, tension=0.4)
        fig.blob([(-0.34, -0.42), (-0.38, 0.06), (-0.24, 0.34), (0, 0.42), (0.24, 0.34), (0.38, 0.06), (0.34, -0.42)], fill, tension=0.4)
        if visor:
            fig.rect(-0.27, -0.04, 0.27, 0.06, "ink")
            fig.rect(-0.04, -0.34, 0.04, -0.04, "ink")
        fig.stroke([(0, 0.4), (0, 0.08)], width=THIN, role="steeldark")


def boot(fig, x, y, s=1.0, fill="leather", sole="wooddark", cuff="cloth", flip=False):
    """A boot in side view, toe to the right, about 1 unit tall, centred."""
    with fig.frame(x, y, s=s, flip=flip):
        fig.blob([(-0.26, 0.48), (0.08, 0.48), (0.1, -0.06), (0.4, -0.16), (0.46, -0.36), (-0.28, -0.36), (-0.3, 0.0)], fill, tension=0.3)
        fig.rect(-0.32, -0.46, 0.48, -0.34, sole, r=0.04)
        if cuff:
            fig.rect(-0.3, 0.38, 0.12, 0.54, cuff, r=0.04)


def wing(fig, x, y, s=1.0, fill="snow", flip=False, rot=0.0):
    """A small feathered wing reaching up and left (right if flip)."""
    with fig.frame(x, y, s=s, flip=flip, rot=rot):
        fig.blob([(0, 0), (-0.2, 0.22), (-0.52, 0.42), (-0.48, 0.24), (-0.6, 0.2), (-0.48, 0.06), (-0.54, -0.02),
                  (-0.3, -0.06)], fill, tension=0.35)
        fig.stroke([(-0.06, 0.04), (-0.4, 0.2)], width=THIN)


def staff(fig, x, y, s=1.0, rot=0.0, shaft="wood", top="mana", topr=0.16):
    """A straight staff with an orb at the top."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.045, -0.6, 0.045, 0.36, shaft, r=0.03)
        fig.poly([(-0.13, 0.32), (0.13, 0.32), (0.07, 0.42), (-0.07, 0.42)], "golddark")
        orb(fig, 0, 0.42 + topr * 0.9, topr, top)


def scroll(fig, x, y, s=1.0, fill="cloth", tie="life"):
    """A rolled scroll, lying across."""
    with fig.frame(x, y, s=s):
        fig.rect(-0.4, -0.2, 0.4, 0.2, fill, r=0.04)
        fig.ellipse(-0.42, 0, 0.08, 0.22, "cloth!80!wood")
        fig.ellipse(0.42, 0, 0.08, 0.22, "cloth!80!wood")
        fig.rect(-0.06, -0.21, 0.06, 0.21, tie)


def chain(fig, pts, fill="steeldark", r=0.07):
    """A chain of links along the polyline pts, alternately round and flat."""
    links = []
    for (ax, ay), (bx, by) in zip(pts, pts[1:]):
        L = math.hypot(bx - ax, by - ay)
        n = max(1, int(L / (1.6 * r)))
        ang = math.degrees(math.atan2(by - ay, bx - ax))
        for i in range(n):
            t = (i + 0.5) / n
            links.append((ax + (bx - ax) * t, ay + (by - ay) * t, ang, len(links) % 2))
    for cx, cy, ang, odd in links:
        if odd:
            fig.ellipse(cx, cy, r * 1.1, r * 0.35, fill, rot=ang)
        else:
            fig.ellipse(cx, cy, r * 1.1, r * 0.7, fill, rot=ang)
            fig.ellipse(cx, cy, r * 0.55, r * 0.25, "shade", rot=ang)


def hook(fig, x, y, s=1.0, rot=0.0, fill="steel"):
    """A butcher's hook: an eye at the top, a barbed curve below."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.circle(0, 0.44, 0.1, fill)
        fig.circle(0, 0.44, 0.04, "shade")
        fig.blob([(-0.05, 0.36), (0.05, 0.36), (0.06, -0.1), (0.0, -0.3), (-0.2, -0.36), (-0.34, -0.22),
                  (-0.36, -0.02), (-0.24, 0.06), (-0.26, -0.06), (-0.22, -0.2), (-0.1, -0.22), (-0.05, -0.1)], fill, tension=0.35)


def skull(fig, x, y, s=1.0, fill="bone"):
    """A small skull, front view."""
    with fig.frame(x, y, s=s):
        fig.blob([(-0.36, 0.06), (-0.3, 0.36), (0, 0.48), (0.3, 0.36), (0.36, 0.06), (0.2, -0.14),
                  (0.18, -0.34), (-0.18, -0.34), (-0.2, -0.14)], fill, tension=0.5)
        fig.ellipse(-0.14, 0.06, 0.1, 0.12, "ink")
        fig.ellipse(0.14, 0.06, 0.1, 0.12, "ink")
        fig.poly([(0, -0.06), (0.05, -0.16), (-0.05, -0.16)], "ink")
        for t in (-0.09, 0, 0.09):
            fig.stroke([(t, -0.24), (t, -0.34)], width=THIN)


def cracks(fig, pts, width=2.6, role="ink"):
    """A zigzag crack along pts."""
    fig.stroke(pts, width=width, role=role)


def rock(fig, x, y, s=1.0, fill="stone", rng=None, seed_shift=0.0):
    """A lumpy boulder about 1 unit wide."""
    jit = [0.0] * 8
    if rng is not None:
        jit = [rng.uniform(-0.06, 0.06) for _ in range(8)]
    with fig.frame(x, y, s=s):
        pts = [(-0.5 + jit[0], -0.3), (-0.42 + jit[1], 0.1), (-0.18, 0.36 + jit[2]), (0.16, 0.4 + jit[3]),
               (0.44 + jit[4], 0.14), (0.5, -0.3 + jit[5])]
        fig.poly(pts, fill)
        fig.stroke([(-0.18, 0.36 + jit[2]), (-0.04, 0.0), (0.44 + jit[4], 0.14)], width=THIN)


def leaf(fig, x, y, s=1.0, rot=0.0, fill="leaf"):
    """A pointed leaf, tip up before rot."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.blob([(0, 0.5), (0.2, 0.16), (0.16, -0.2), (0, -0.42), (-0.16, -0.2), (-0.2, 0.16)], fill, tension=0.5)
        fig.stroke([(0, -0.42), (0, 0.38)], width=THIN)


def band(fig, x, y, s=1.0, fill="gold", stud="life", rot=0.0):
    """A wrist cuff or bracer seen from the front: a short wide cylinder."""
    with fig.frame(x, y, s=s, rot=rot):
        fig.rect(-0.42, -0.3, 0.42, 0.3, fill, r=0.06)
        fig.ellipse(0, 0.3, 0.42, 0.1, fill + "!80!snow")
        fig.ellipse(0, 0.3, 0.32, 0.06, "shade")
        fig.stroke([(-0.42, -0.12), (0.42, -0.12)], width=THIN)
        if stud:
            gem(fig, 0, 0.04, 0.32, stud)
