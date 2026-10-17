"""Writes journal tab 9 (round 4: All Random All Mid and the base cannon,
triage T13 and T14) into ../pots-journal.html.

    python3 tab9.py

Every number in the tab is worked out here from data/ (the map, the
heroes' speeds), and the script fails if PLAN.md's round-4 section does not
quote the same numbers. The map is drawn from data/map.json. Pictures come
from ../art3/ (`ART=art3 shoot.py models|abilities ...`). Replaces the tab
if it is already there; fails if a picture or a description is missing.
"""
import html, json, math, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art3"
data = lambda n: json.loads((ROOT / "data" / n).read_text())
e = lambda s: html.escape(str(s), quote=True)
missing = []

# ------------------------------------------------------------ the numbers
MAP = data("map.json")
UNITS = {u["id"]: u for u in data("units.json")["units"]}
RULES = data("rules.json")
S = MAP["size"]
sun = MAP["sides"][0]
moon = MAP["sides"][1]
speeds = sorted(u["speed"] for u in UNITS.values() if u["kind"] == "hero")
avg = sum(speeds) / len(speeds)
B = {(b["unit"], b.get("lane")): b["at"] for b in sun["buildings"]}
spawn, entry = sun["spawn"], sun["lanes"]["mid"][0]
t1, t3 = B[("tower1_sun", "mid")], B[("tower3_sun", "mid")]
centre = [S // 2, S // 2]
mirror = lambda p: [S - p[0], S - p[1]]
d = math.dist
walk_t1 = d(spawn, entry) + d(entry, t1)
walk_c = d(spawn, entry) + d(entry, centre)
t1_t1 = d(t1, mirror(t1))
fountain_range = RULES["tables"]["fountain_range"]
# the plan's proposed cannon: 1 s to reach, 0.5 s to climb in, 2 s in the air
ARAM = RULES["aram"]
CLIMB, FLIGHT, REACH_PAST = ARAM["climb"] / 100, ARAM["flight"] / 100, ARAM["reach_past"]
by_cannon = 1.0 + CLIMB + FLIGHT
N = {
    "walk_t1": f"{walk_t1:,.0f}", "walk_t1_s": f"{walk_t1 / avg:.1f} s", "fast": f"{walk_t1 / speeds[-1]:.1f} s", "slow": f"{walk_t1 / speeds[0]:.1f} s",
    "walk_c": f"{walk_c:,.0f}", "walk_c_s": f"{walk_c / avg:.1f} s", "t1_t1": f"{t1_t1:,.0f}", "t1_t1_s": f"{t1_t1 / avg:.1f} s",
    "avg": f"{avg:.1f}", "cannon": f"about {by_cannon:.1f} s", "range": f"{fountain_range}",
}
plan = (ROOT / "PLAN.md").read_text()
r4 = plan[plan.index("## Round 4"):]
for k in ("walk_t1", "walk_t1_s", "fast", "slow", "walk_c", "walk_c_s", "t1_t1", "t1_t1_s", "avg", "cannon", "range"):
    if N[k] not in r4:
        missing.append(f"PLAN.md round 4 does not say {N[k]} ({k})")

# ------------------------------------------------------------ the map
W = 560
k = W / S
X = lambda p: round(p[0] * k, 1)
Y = lambda p: round((S - p[1]) * k, 1)
pt = lambda p: f"{X(p)},{Y(p)}"
lane_dir = [(t1[0] - t3[0]), (t1[1] - t3[1])]
ln = math.hypot(*lane_dir)
ux, uy = lane_dir[0] / ln, lane_dir[1] / ln
half = MAP["lane_width"] / 2


def band(a, b):
    """The cannon's reach: the lane from `a` to `b`, the lane's width across."""
    px, py = -uy * half, ux * half
    return " ".join(pt(p) for p in ([a[0] + px, a[1] + py], [b[0] + px, b[1] + py], [b[0] - px, b[1] - py], [a[0] - px, a[1] - py]))


reach_end = [t1[0] + ux * REACH_PAST, t1[1] + uy * REACH_PAST]
# where the game puts them (data/rules.json "aram")
cannon_at = ARAM["cannon"]
relics = ARAM["relics"]


def svg_map():
    out = [f'<svg viewBox="0 0 {W} {W}" role="img" aria-labelledby="aram-map-t" class="aram-map"><title id="aram-map-t">The map in All Random All Mid: only the middle lane, with a cannon and a landing band at each base and two relics a side</title>',
           f'<rect width="{W}" height="{W}" rx="10" class="m-ground"/>']
    for side, cls in ((sun, "sun"), (moon, "moon")):
        out.append(f'<polygon points="{" ".join(pt(p) for p in side["base"])}" class="m-base {cls}"/>')
    river = " ".join(pt(p) for p in MAP["river"])
    out.append(f'<polyline points="{river}" class="m-river" stroke-width="{MAP["river_width"] * k:.1f}"/>')
    for name in ("top", "bot"):
        out.append(f'<polyline points="{" ".join(pt(p) for p in sun["lanes"][name])}" class="m-lane off" stroke-width="{MAP["lane_width"] * k:.1f}"/>')
    out.append(f'<polyline points="{" ".join(pt(p) for p in sun["lanes"]["mid"])}" class="m-lane" stroke-width="{MAP["lane_width"] * k:.1f}"/>')
    for c in MAP["camps"]:
        for p in (c["at"], mirror(c["at"])):
            out.append(f'<g class="m-off"><line x1="{X(p) - 5}" y1="{Y(p) - 5}" x2="{X(p) + 5}" y2="{Y(p) + 5}"/><line x1="{X(p) - 5}" y1="{Y(p) + 5}" x2="{X(p) + 5}" y2="{Y(p) - 5}"/></g>')
    for side, cls, flip in ((sun, "sun", False), (moon, "moon", True)):
        for b in side["buildings"]:
            p = b["at"]
            kind = b["unit"].split("_")[0]
            r = 9 if kind == "sphere" else 6 if kind in ("fountain",) else 4.5 if kind.startswith("tower") else 4
            on = b.get("lane") in ("mid", "base", None)
            out.append(f'<circle cx="{X(p)}" cy="{Y(p)}" r="{r}" class="m-b {cls}{"" if on else " off"}"/>')
        base_t1 = mirror(t1) if flip else t1
        base_t3 = mirror(t3) if flip else t3
        end = mirror(reach_end) if flip else reach_end
        out.append(f'<polygon points="{band(base_t3, end)}" class="m-reach {cls}"/>')
        can = mirror(cannon_at) if flip else cannon_at
        land = mirror(t1) if flip else t1
        mx, my = (X(can) + X(land)) / 2, (Y(can) + Y(land)) / 2 - 60
        out.append(f'<path d="M{X(can)},{Y(can)} Q{mx:.1f},{my:.1f} {X(land)},{Y(land)}" class="m-arc {cls}"/>')
        out.append(f'<rect x="{X(can) - 6}" y="{Y(can) - 6}" width="12" height="12" rx="3" class="m-cannon {cls}"/>')
        for q in relics:
            q = mirror(q) if flip else q
            out.append(f'<circle cx="{X(q)}" cy="{Y(q)}" r="5" class="m-relic"/>')
    for s in MAP["shops"]:
        p = s[1]
        out.append(f'<rect x="{X(p) - 3.5}" y="{Y(p) - 3.5}" width="7" height="7" class="m-shop{"" if s[0].startswith("home") else " off"}"/>')
    out.append("</svg>")
    return "".join(out)


LEGEND = [("m-lane-k", "The middle lane: the only lane with creeps"), ("m-off-k", "Side lanes, their buildings, the camps and the side and secret shops: not built"),
          ("m-cannon-k", "Each side's cannon, beside its spawn"), ("m-reach-k", f"Where its throw can land: the lane from the base tower to {REACH_PAST} past the outer tower"),
          ("m-relic-k", "Health relics, proposed: two a side between the outer tower and the river")]

# ------------------------------------------------------------ the pictures
sys.path.insert(0, str(HERE))
notes = {}
src = (HERE / "aram.js").read_text()
for m in re.finditer(r'NOTES\.(\w+) = \{\s*see: "((?:[^"\\]|\\.)*)",\s*tell: "((?:[^"\\]|\\.)*)"', src):
    notes[m.group(1)] = {"see": m.group(2), "tell": m.group(3)}
mnotes = dict(re.findall(r'^  (\w+): "((?:[^"\\]|\\.)*)",$', src[src.index("MODEL_NOTES"):src.index("// ===", src.index("MODEL_NOTES"))], re.M))
MODELS = [("cannon_sun", "Sunward cannon"), ("cannon_moon", "Moonward cannon"), ("relic", "Health relic")]
ANIMS = [("cannon_launch", "The cannon fired", "T14"), ("cannon_aim", "Aiming the cannon", "T14"), ("relic_pickup", "A health relic taken", "T13"),
         ("random_reveal", "The random draw", "T13"), ("aram_mid", "All mid, five a side", "T13")]
for mid, _ in MODELS:
    if not (ART / "models" / f"{mid}.jpg").exists():
        missing.append(f"picture of {mid}")
    if not mnotes.get(mid):
        missing.append(f"description of {mid}")
for aid, _, _ in ANIMS:
    for ext in ("gif", "jpg"):
        if not (ART / "abilities" / f"{aid}.{ext}").exists():
            missing.append(f"{ext} of {aid}")
    if not notes.get(aid, {}).get("see"):
        missing.append(f"description of {aid}")


def model_card(mid, name):
    n = mnotes.get(mid, "")
    return (f'<figure class="mcard"><button class="zoom" data-full="art3/models/{mid}.jpg" data-cap="{e(name)}: {e(n)}">'
            f'<img loading="lazy" src="art3/models/{mid}.jpg" alt="{e(name)}: {e(n)}"></button>'
            f'<figcaption><b>{e(name)}</b> <code>{e(mid)}</code><span>{e(n)}</span></figcaption></figure>')


def anim_card(aid, title, row):
    n = notes.get(aid, {})
    tell = f'<p><b>How you can tell.</b> {e(n["tell"])}</p>' if n.get("tell") else ""
    return f'''<article class="acard">
  <header><div><h3>{e(title)}</h3><div class="ameta"><span class="chip acc">{e(row)}</span> mockup · <code>{e(aid)}</code></div></div></header>
  <button class="zoom gif" data-full="art3/abilities/{aid}.jpg" data-cap="{e(title)}"><img loading="lazy" src="art3/abilities/{aid}.gif" alt="{e(title)}, animated: {e(n.get("see", ""))}" width="480" height="270"></button>
  <p><b>What you see.</b> {e(n.get("see", ""))}</p>
  {tell}
</article>'''


def table(rows):
    return "".join(f"<tr>{''.join(f'<td>{c}</td>' for c in r)}</tr>" for r in rows)


REQUEST = "now, plan for an all random all mid mode, make a design journal tab with mock ups; i want a cannon like in league to come back to the middle of lane faster."
RULES_ROWS = [
    ["Heroes", "Drawn for every player from the match seed, none twice, bots too", "One reroll each before the start (R4.2)"],
    ["Lanes", "The middle lane only; the side lanes' towers and barracks are not built", "The usual map's middle lane (R4.1)"],
    ["Base towers", "Fall once every built lane's base-level tower has fallen", "Counts the middle lane only (R4.8)"],
    ["Forest", "No neutral camps, no boss, no runes", ""],
    ["Shops", "The home shop and the courier; side and secret shops closed", "Teleport scrolls still sold? (R4.6)"],
    ["Starting gold", "853", "603 in a normal match (R4.5)"],
    ["Cannon", f"From within the fountain's range ({fountain_range}); {CLIMB:g} s to climb in, {FLIGHT:g} s in the air; lands in the band", "Once each visit to the base (R4.3)"],
    ["Health relics", "Two a side; 15 % of health and mana; back after 60 s", "Proposed (R4.4)"],
    ["Online", "The host picks the mode in the lobby; the seed carries the draw", "PROTOCOL 2 → 3"],
]
QUESTIONS = [
    ["R4.1", "All Mid on the usual map's middle lane (planned), or a small map of its own, as League's ARAM has?"],
    ["R4.2", "All Random: no reroll, one reroll each (planned), or teammates may swap?"],
    ["R4.3", f"The cannon: {CLIMB:g} s to climb in, {FLIGHT:g} s in the air, lands up to {REACH_PAST} past the outer tower, once each visit to the base. The cannon you meant?"],
    ["R4.4", "Health relics in the lane (two a side, 15 %, back after 60 s): keep, change or drop?"],
    ["R4.5", "Starting gold in All Random: 603 or 853?"],
    ["R4.6", "Teleport scrolls in this mode: sell them or not?"],
]
STEPS = [
    ("The mode as data", "A <code>modes</code> table in <code>data/rules.json</code>; a mode choice on the custom-match screen and in the lobby; middle-lane waves only; side buildings, camps, boss, runes and far shops left out; the base-tower rule counting built lanes."),
    ("All Random", "The draw from the seed, the reveal on the pick screen, the reroll."),
    ("The cannon", "A new order and a flight act in the core, so rollback and online carry them; the band, the landing ring, the arc and the landing in the scene."),
    ("Relics", "Pick-up, heal, return; the empty stones and their timer."),
    ("Computer players", "Mid only, the cannon after each respawn, a relic below half health."),
    ("Online", "The mode in the start message; PROTOCOL 3; the five-runner test."),
    ("Models", "The cannon and relic from these mockups into <code>data/models.json</code>, as round 2's were."),
]
TESTS = [
    "An <code>aram</code> match spawns creeps in the middle lane only and builds no side-lane tower, camp, boss or rune.",
    "With the middle lane's three towers and two barracks down, the base towers can be hurt; a normal match's rule is unchanged.",
    "Ten heroes drawn from one seed are ten different heroes and the same in every browser; a reroll never gives a hero someone has.",
    f"A throw lands at the chosen point after exactly {CLIMB:g} s + {FLIGHT:g} s of ticks; a point past the band lands at its end; from outside the fountain's range it is refused; a second throw before returning is refused; a hero in flight takes no damage.",
    "A relic heals once, is gone for 60 s and returns; either side can take it.",
    "Bots in an <code>aram</code> match each use the cannon at least once and a Sphere falls within the usual limit.",
    "Each test gets a row in <code>tests/breaks.json</code> and must fail with its rule broken.",
]

CSS = """/*ARAM*/
.aram-map { width: 100%; max-width: 560px; height: auto; display: block; }
.m-ground { fill: #7fb86a; }
.m-base.sun { fill: #f2b33d; fill-opacity: .28; } .m-base.moon { fill: #7a5cd6; fill-opacity: .28; }
.m-river { fill: none; stroke: #7fd3f0; stroke-linecap: round; stroke-linejoin: round; }
.m-lane { fill: none; stroke: #e9cf9e; stroke-linecap: round; stroke-linejoin: round; }
.m-lane.off { stroke: #5d7a55; stroke-opacity: .8; }
.m-off line { stroke: #3d5a37; stroke-width: 2; }
.m-b.sun { fill: #f2b33d; stroke: #2b2140; stroke-width: 1.2; } .m-b.moon { fill: #9a83ea; stroke: #2b2140; stroke-width: 1.2; }
.m-b.off { fill: none; stroke: #3d5a37; stroke-dasharray: 2 2; }
.m-reach.sun { fill: #ffd36a; fill-opacity: .55; } .m-reach.moon { fill: #c3b2ff; fill-opacity: .6; }
.m-arc { fill: none; stroke-width: 2; stroke-dasharray: 3 4; } .m-arc.sun { stroke: #b06b10; } .m-arc.moon { stroke: #4f3a9e; }
.m-cannon.sun { fill: #d9812b; stroke: #2b2140; stroke-width: 1.5; } .m-cannon.moon { fill: #4f3a9e; stroke: #2b2140; stroke-width: 1.5; }
.m-relic { fill: #3ee06a; stroke: #1f5a30; stroke-width: 1.5; }
.m-shop { fill: #fff1d6; stroke: #2b2140; } .m-shop.off { fill: none; stroke: #3d5a37; stroke-dasharray: 2 2; }
.maprow { display: grid; grid-template-columns: minmax(0, 560px) minmax(0, 1fr); gap: 22px; align-items: start; }
.legend { display: grid; gap: 10px; font-size: 14.5px; margin: 0; padding: 0; list-style: none; }
.legend li { display: flex; gap: 10px; align-items: center; }
.legend li.src { display: block; color: var(--muted); font-size: 13.5px; }
.legend i { flex: none; width: 22px; height: 12px; border-radius: 3px; border: 1px solid #2b2140; }
.m-lane-k { background: #e9cf9e; } .m-off-k { background: transparent; border-style: dashed !important; } .m-cannon-k { background: #d9812b; }
.m-reach-k { background: #ffd36a; } .m-relic-k { background: #3ee06a; border-radius: 50% !important; width: 12px !important; }
.nums td:nth-child(2) { font-family: var(--font-mono); white-space: nowrap; }
@media (max-width: 860px) { .maprow { grid-template-columns: minmax(0, 1fr); } }
/*ARAM-END*/
"""

PANEL = f'''<!-- ======================================================= TAB 9 -->
<div class="panel-tab" role="tabpanel" id="p-aram" aria-labelledby="t-aram" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 9 · 16 October 2026 · All Random All Mid (triage T13, T14)</div>
  <h2>One lane, a random hero, and a cannon home to the fight</h2>
  <p class="lede">A plan for a new mode and mockups of what it adds: every player gets a random hero, all ten fight in the middle lane, and a cannon at each base throws a hero back out to the lane. Nothing here is built yet; the plan waits for your answers to the six questions at the end. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Why a cannon</div><h2>The walk back, measured</h2></div>
  <p>From the map and the ten heroes' speeds (average {N["avg"]} map units a second):</p>
  <div class="table-scroll"><table class="nums">
    <thead><tr><th>From the spawn</th><th>Distance and time</th></tr></thead>
    <tbody>{table([
      ["to the outer middle tower, on foot", f'{N["walk_t1"]} units, {N["walk_t1_s"]} ({N["fast"]} to {N["slow"]})'],
      ["to the centre of the map, on foot", f'{N["walk_c"]} units, {N["walk_c_s"]}'],
      ["to the outer middle tower, by cannon", f'{N["cannon"]} (1 s to the cannon, {CLIMB:g} s to climb in, {FLIGHT:g} s in the air)'],
      ["between the two outer middle towers", f'{N["t1_t1"]} units, {N["t1_t1_s"]}'],
    ])}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockup</div><h2>The map, middle lane only</h2></div>
  <div class="maprow">
    <div>{svg_map()}</div>
    <ul class="legend">{"".join(f'<li><i class="{c}"></i>{e(t)}</li>' for c, t in LEGEND)}
      <li class="src">Drawn from <code>data/map.json</code>: the bases, river, lanes, buildings, camps and shops are where the game has them. The cannons, bands and relics are the plan's.</li></ul>
  </div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockups</div><h2>New models</h2></div>
  <div class="mgrid">{"".join(model_card(m, n) for m, n in MODELS)}</div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockups</div><h2>How it moves</h2></div>
  <p>Distances are drawn shorter than they are: the real throw crosses several thousand map units.</p>
  <div class="agrid">{"".join(anim_card(*a) for a in ANIMS)}</div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">The plan</div><h2>The mode's rules</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>In All Random All Mid</th><th>Note</th></tr></thead>
    <tbody>{table(RULES_ROWS)}</tbody>
  </table></div>
  <div class="grid2">
    <div class="card"><h3>Order of work</h3><ol>{"".join(f"<li><b>{e(t)}.</b> {d}</li>" for t, d in STEPS)}</ol></div>
    <div class="card"><h3>How each part is tested</h3><ul>{"".join(f"<li>{t}</li>" for t in TESTS)}</ul></div>
  </div>
  <p>The full plan, with a reason and a test for each decision, is round 4 of <code>PLAN.md</code>.</p>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Yours to decide</div><h2>Open questions, by number</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>#</th><th>Question</th></tr></thead>
    <tbody>{table(QUESTIONS)}</tbody>
  </table></div>
</section>
<footer>Mockups in <code>journal/mockups/aram.js</code> (three.js r160, MIT), pictures made with <code>ART=art3 journal/mockups/shoot.py</code>, this tab written by <code>journal/mockups/tab9.py</code>. Labelled mockups: the game draws its own models with its own renderer.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r"/\*ARAM\*/.*?/\*ARAM-END\*/\n", "", s, flags=re.S)
s = s.replace("</style>", CSS + "</style>", 1)
s = re.sub(r'\n    <button role="tab" id="t-aram".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-aram" aria-controls="p-aram" aria-selected="false" tabindex="-1"><small>9 · 16 Oct</small><span>All Random All Mid</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 9 -->.*?<!--/TAB 9-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 9-->\n<!--PANELS-->", 1)
# the picture viewer belongs outside every panel (it once sat inside tab 8's)
DLG = '<dialog class="lightbox" id="lightbox" aria-label="Larger picture"><img alt=""><p></p><button type="button" data-close>Close</button></dialog>'
if s.count(DLG) == 1 and s.index(DLG) < s.index("<!--PANELS-->"):
    s = s.replace(DLG + "\n", "").replace(DLG, "")
    i = s.index("<script>\n/*ART2-JS*/")
    s = s[:i] + DLG + "\n" + s[i:]
for need in ('id="t-aram"', 'id="p-aram"', "/*ARAM*/", 'id="lightbox"'):
    assert s.count(need) == 1, need
J.write_text(s)
print("tab 9 written:", N)
