"""Writes journal tab 10 (round 6, triage T18: how Mage Tower Battle's look
grew into its latest version, and every current model before and after in
that look) into ../pots-journal.html.

    python3 tab10.py

Pictures: ../art4/game/ (the game as it draws now, `tools/shots.py`),
../art4/models/ and ../art4/scenes/ (the mockups, `ART=art4 LOOK=ape
shoot.py models|lineups|scene`), ../art4/mtb/ (Mage Tower Battle's own
journal pictures, copied). Fails if a picture is missing. Replaces the tab
if it is already there.
"""
import html, json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art4"
e = lambda s: html.escape(str(s), quote=True)
missing = []
STR = json.loads((ROOT / "data" / "strings.json").read_text())

MODELS = json.loads((ROOT / "data" / "models.json").read_text())
IDS = sorted(MODELS["models"] if "models" in MODELS else MODELS)
SIDE = {"sun": "Sunward", "moon": "Moonward"}
OWN = {
    "tower": "tower", "barracks_melee": "melee barracks", "barracks_ranged": "ranged barracks", "fountain": "fountain",
    "sphere": "Sphere", "cannon": "cannon", "tree": "tree", "creep_melee": "melee creep", "creep_ranged": "ranged creep", "creep_siege": "siege creep",
}


def name(mid):
    if mid.startswith("hero_"):
        return STR["heroes"][mid[5:]]["name"]
    if mid in STR["units"]:
        return STR["units"][mid]
    base, _, side = mid.rpartition("_")
    if side in SIDE and base in OWN:
        return f"{SIDE[side]} {OWN[base]}"
    if mid.startswith("rune_"):
        return f"Rune: {mid[5:]}"
    return mid.replace("_", " ").capitalize()


GROUPS = [
    ("Heroes", [m for m in IDS if m.startswith("hero_")]),
    ("Creeps", [m for m in IDS if m.startswith("creep_")]),
    ("Buildings", [m for m in IDS if m.split("_")[0] in ("tower", "barracks", "fountain", "sphere", "shop", "cannon")]),
    ("The forest", [m for m in IDS if m in ("imp", "hound", "mossback", "golem_small", "golem", "hornet", "shaman", "drake_young", "drake", "warden", "critter", "tree_sun", "tree_moon")]),
    ("Small things", [m for m in IDS if m.startswith(("rune_", "relic", "courier", "healing_ward", "observer_ward", "sentry_ward", "fissure_rock"))]),
]
placed = {m for _, ms in GROUPS for m in ms}
if placed != set(IDS):
    missing.append(f"models in no group: {sorted(set(IDS) - placed)}")
for m in IDS + ["heroes_1", "heroes_2"]:
    for d in ("game", "models"):
        if not (ART / d / f"{m}.jpg").exists():
            missing.append(f"{d}/{m}.jpg")
for p in ["game/lane", "scenes/lane", "scenes/lane_game"] + [f"mtb/{n}" for n in ("r8b-floor", "r8g-floor", "r8b-library", "r8g-library", "g-battle-one", "r8g-battle", "r8b-lab", "r8-lab")]:
    if not (ART / f"{p}.jpg").exists():
        missing.append(f"{p}.jpg")


def pic(path, cap, alt=None):
    return (f'<button class="zoom" data-full="art4/{path}.jpg" data-cap="{e(cap)}">'
            f'<img loading="lazy" src="art4/{path}.jpg" alt="{e(alt or cap)}"></button>')


def pair(a, b, la, lb, cap, title):
    return f'''<figure class="ba">
  <div class="ba-row"><div><span class="chip">{e(la)}</span>{pic(a, f"{title}: {la}")}</div><div><span class="chip acc">{e(lb)}</span>{pic(b, f"{title}: {lb}")}</div></div>
  <figcaption><b>{e(title)}.</b> {cap}</figcaption>
</figure>'''


def model_pair(mid):
    n = name(mid)
    return f'''<figure class="bam">
  <div class="ba-row">{pic(f"game/{mid}", f"{n}: the game now")}{pic(f"models/{mid}", f"{n}: mockup, the new look")}</div>
  <figcaption><b>{e(n)}</b> <code>{e(mid)}</code></figcaption>
</figure>'''


def table(rows):
    return "".join(f"<tr>{''.join(f'<td>{c}</td>' for c in r)}</tr>" for r in rows)


REQUEST = "now do a pass over how mage tower's overall art style and design was improved into the latest version, then do before and after mockups of current assets in the new style"

TIMELINE = [
    ["1.0 to 1.5 (rounds 1 to 6)", "Smooth balls and cylinders with a black outline, under three flat bands of light. People were stacked balls with no arms that slid when they walked. Every station was one shape on a bare floor.",
     "This is the look Protection of the Spheres took for its round 2 (with Chumbles), and the look the game has now."],
    ["1.6.0 (round 7, T34 to T37)", "The Ape Race look: faceted shapes, sky and ground light with a wrapping sun, haze in the place's colour, glow as added light, soft disc shadows, no outline. Jointed people with shoulders, hands and feet. A dressed room. The camera comes down behind the archmage at a station. A dark see-through HUD.",
     "Asked for as \"that level of fidelity\": an analysis and mockups first (entry 8), then built once answered."],
    ["1.7.0 (round 8)", "Readable spell tiles and new screens drawn to their mockups; the battle field keeps 1.6.0's light and facets.", "A playtester's points; the look itself unchanged."],
    ["1.7.1 (round 9, T46, T47)", "The close camera keeps the overview's heading and eases down over three quarters of a second, so walking stays easy to steer. The title screen turns slowly inside the tower while the archmage walks the stations.",
     "Tuning the camera the new look brought in."],
]

PARTS = [
    ["Shape", "Smooth spheres and cylinders, 24 to 28 sides", "Few sides and flat faces; each face catches the light on its own, so a rock reads as six shades", "<code>mesh.rs</code>: flat normals, corners nudged by a seeded chance"],
    ["Outline", "A black ink pass: every shape drawn twice", "None. Light on the facets tells shapes apart, and dropping the second pass let a room of sixty students fit under the 16,000-instance cap (R7.2)", "ink pass removed from core, page and shader"],
    ["Light", "Three flat bands", "A colour from the sky above, the ground's colour from below, and a sun whose light wraps part way round each shape", "<code>scene.frag</code>; colours sent as uniforms with each view"],
    ["Haze", "None: near and far equally sharp", "Fog in the place's own colour closes in with distance and turns the far half into a backdrop", "<code>render::Mood</code> per viewport"],
    ["Shadows", "None", "A soft dark disc under each thing; the renderer still casts none (R7.1)", "<code>room.rs</code> <code>shadow</code>"],
    ["Glow", "None", "Added light, softest at a shape's outline; a smooth halo mesh is the one exception to flat faces (R7.5)", "the glow pass"],
    ["People", "Stacked balls, no arms", "Shoulders, hands, feet, head, a bent hat tip; arms turn about the shoulder; a held staff stays upright (R7.7)", "<code>models.json</code> pivots, <code>grip_r</code>"],
    ["Dressing", "Each station one shape", "Things at three sizes, each repeated with its own shade, size and tilt: stone blocks, timber, windows and light shafts, full shelves, book piles, candles", "<code>room.rs</code>"],
    ["Palette", "One purple for everything", "A few related colours for the place, plus one bright colour for what matters to play (the invasion book's red beam)", "<code>palette.json</code>"],
    ["Camera", "High over the room", "Comes down behind the archmage at a station (1.6.0), then eased so the keys keep their directions (1.7.1)", "<code>room::closeness</code>, <code>Prep::dip</code>"],
    ["HUD", "Pale cards", "Dark see-through panels, one gold button for the main action", "<code>web/style.css</code>; the contrast check still passes"],
]

LESSONS = [
    ["R7.1", "Real shadows were not possible in the game's renderer, so the mockup's were replaced with soft discs. Here the team ring already sits under every unit and can become that disc."],
    ["R7.2", "Dropping the outline halved the draw work. The busiest match screen here would gain the same."],
    ["R7.5", "A faceted glow fades in flat patches, so glows stay smooth. Here: the Sphere's light, runes, the relic and every ability's glow."],
    ["R7.6", "Nudging corners on one shared mesh made every copy the same wedge. The nudge was halved and half the copies turned. Here: trees and rocks share meshes too."],
    ["R9.1, R9.2", "A camera that turns breaks steering. This game's camera never turns, so only Mage Tower's lighter haze and dressing carry over, not its close camera."],
]

POTS = [
    ["Faceted shapes", "No: smooth spheres and capsules", "every model, by mesh setting"],
    ["No outline", "No: the plum ink pass on every model", "the shader's ink pass"],
    ["Sky and ground light, wrapping sun", "Mostly: round 3's shader already wraps the light and mixes a sky fill with a ground bounce", "kept, retuned to a lower sun"],
    ["Haze in the place's colour", "Partly: there is fog, but pale blue and far off", "colour and distances"],
    ["Soft disc shadows", "Partly: a team ring under each unit, no dark disc under buildings or trees", "a disc under everything"],
    ["Glow as light", "Partly: glow-style parts are drawn unlit, not added", "the Sphere, runes, abilities"],
    ["Ground at three sizes", "No: flat grass and a path, trees only", "boulders, tufts, flowers and pebbles"],
    ["One palette for the place", "Mostly: the sides' gold and violet are the bright colours", "kept"],
    ["Jointed people", "Yes: heroes and creeps already have shoulders and hips (round 2)", "kept"],
]

STEPS = [
    ("The light", "Remove the ink pass from the shader, the core's draw list and the page; send the sky, ground and sun colours and the haze as uniforms, as now.", "Small"),
    ("Facets", "Build the sphere, capsule, cylinder and cone meshes with few sides and flat normals; keep a smooth halo mesh for glows.", "Small"),
    ("Shadows", "A soft dark disc under every unit, building and tree, from the unit's size.", "Small"),
    ("The ground", "Seeded dressing at three sizes along the lanes and in the forest, from the map's seed, so every browser draws the same.", "Medium"),
    ("Tuning", "Colours, haze distances and the sun's angle into <code>data/presentation.json</code>; the browser check's pictures retaken.", "Small"),
]

QUESTIONS = [
    ["R6.1", "Build this look into the game: all of it, some parts (which?), or not yet?"],
    ["R6.2", "Time of day: late afternoon as drawn, or Mage Tower's dusk, or a mood that turns with the game's day and night?"],
    ["R6.3", "Faces: heads faceted as drawn, or kept smooth so the eyes read better at match distance?"],
]

CSS = """/*NEWLOOK*/
.ba { margin: 0; display: grid; gap: 8px; }
.ba-row { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
.ba-row > div { display: grid; gap: 6px; align-content: start; }
.ba-row .chip { justify-self: start; }
.ba img, .bam img { width: 100%; height: auto; display: block; border-radius: 8px; }
.ba figcaption, .bam figcaption { font-size: 14.5px; }
.bagrid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 380px), 1fr)); gap: 18px; }
.bam { margin: 0; display: grid; gap: 6px; }
.bam figcaption code { color: var(--muted); font-size: 12.5px; }
.ba-key { display: flex; flex-wrap: wrap; gap: 8px 16px; font-size: 14px; color: var(--muted); }
.timeline td:first-child { white-space: nowrap; font-weight: 600; }
/*NEWLOOK-END*/
"""

def gallery():
    out = []
    for title, ms in GROUPS:
        out.append(f'<h3 class="sub">{e(title)} ({len(ms)})</h3><div class="bagrid">{"".join(model_pair(m) for m in ms)}</div>')
    return "\n".join(out)


PANEL = f'''<!-- ======================================================= TAB 10 -->
<div class="panel-tab" role="tabpanel" id="p-newlook" aria-labelledby="t-newlook" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 10 · 17 October 2026 · a style study (triage T18); the game itself has not changed</div>
  <h2>Mage Tower's newer look, applied to every model here</h2>
  <p class="lede">Part 1 traces how Mage Tower Battle's look changed from the version this game copied to its latest version. Part 2 measures where this game stands against those changes. Part 3 shows every current model twice: as the game draws it now, and as a mockup in the new look. Part 4 is what building it would take, with three questions for you. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 1</div><h2>How Mage Tower's look changed</h2></div>
  <p>Read from Mage Tower Battle's triage, notebook, handoff, shader and design journal (entries 8 and 9). The big change came in one round, 1.6.0, when its person asked for Ape Race's level of fidelity inside the tower. Its agent took Ape Race's look apart first, mocked the tower up in it, and built it once the questions were answered.</p>
  <div class="table-scroll"><table class="timeline">
    <thead><tr><th>Version</th><th>The look</th><th>Why</th></tr></thead>
    <tbody>{table([[e(a), e(b), e(c)] for a, b, c in TIMELINE])}</tbody>
  </table></div>
  <h3 class="sub">Before and after, in Mage Tower's own pictures</h3>
  <div class="grid2">
    {pair("mtb/r8b-floor", "mtb/r8g-floor", "1.5.0", "1.6.0", "Stone, timber posts, windows with shafts of light, full shelves, a chandelier and a rug, from the same spot. The outline is gone and the faces of every shape take the light differently.", "The tower's floor")}
    {pair("mtb/r8b-library", "mtb/r8g-library", "1.5.0", "1.6.0", "Empty shelves and books lying flat became full shelves of books of mixed heights, piles on the floor and a candle, seen from just behind the archmage.", "The library")}
    {pair("mtb/g-battle-one", "mtb/r8g-battle", "1.5.0", "1.6.0", "Flat green squares became faceted tiles in many shades over an earth slab, with tufts, flowers and stones; the same light and no outlines.", "The battle field")}
    {pair("mtb/r8b-lab", "mtb/r8-lab", "1.5.0", "mockup", "The mockup that set the target: crystals with thin beams, a cauldron on a fire, a wall of jars, haze and a low camera.", "The laboratory")}
  </div>
  <h3 class="sub">What changed, part by part</h3>
  <div class="table-scroll"><table>
    <thead><tr><th>Part</th><th>1.5.0</th><th>1.6.0 and later</th><th>Where</th></tr></thead>
    <tbody>{table(PARTS)}</tbody>
  </table></div>
  <h3 class="sub">What Mage Tower learned building it</h3>
  <div class="table-scroll"><table>
    <thead><tr><th>Its row</th><th>The lesson, and what it means here</th></tr></thead>
    <tbody>{table([[e(a), e(b)] for a, b in LESSONS])}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 2</div><h2>Where this game stands</h2></div>
  <p>This game took Mage Tower's 1.5.0 look in its round 2, so it has most of what 1.6.0 left behind. Round 3 already moved its light part of the way.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>Mage Tower 1.6.0 has</th><th>This game now</th><th>In the mockups below</th></tr></thead>
    <tbody>{table(POTS)}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 3</div><h2>Every model, before and after</h2></div>
  <p class="ba-key"><span><span class="chip">left</span> the game now: its own renderer and model viewer (<code>?tile=lineup:…</code>)</span><span><span class="chip acc">right</span> mockup in the new look: the same parts, drawn with three.js</span></p>
  <div class="grid2">
    {pair("game/lane", "scenes/lane_game", "the game now", "mockup", "Left, a match 75 seconds in, at the game's own camera. Right, a lane at the clash of two waves from the same pitch: faceted ground rising toward the trees, stones, tufts and flowers off the path, trees each in their own shade, and the far trees going into the haze.", "A lane")}
    {pair("game/heroes_1", "models/heroes_1", "the game now", "mockup", "Ironvow, Rime, Cinder, Thistle and Gristle. Every round ball is now 80 faces, every column 8 sides, with no outline.", "Heroes, first five")}
    {pair("game/heroes_2", "models/heroes_2", "the game now", "mockup", "Tectus, Kestrel, Hexwright, Longwick and Tempest.", "Heroes, second five")}
    <figure class="ba">{pic("scenes/lane", "A lane in the new look, from the mockups' own angle")}<figcaption><b>The same lane from closer and lower</b>, where the facets and the dressing show most.</figcaption></figure>
  </div>
  {gallery()}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 4</div><h2>What building it would take</h2></div>
  <p>Like Mage Tower's, none of it needs a new library, a shadow map or a post-processing pass, and none of it touches the rules, so online play is unaffected.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>Step</th><th>What</th><th>Size</th></tr></thead>
    <tbody>{table([[e(a), b, e(c)] for a, b, c in STEPS])}</tbody>
  </table></div>
  <h3 class="sub">Yours to decide, by number</h3>
  <div class="table-scroll"><table>
    <thead><tr><th>#</th><th>Question</th></tr></thead>
    <tbody>{table(QUESTIONS)}</tbody>
  </table></div>
</section>
<footer>Mockups: <code>journal/mockups/kit.js</code> with <code>?look=ape</code> (three.js r160, MIT), pictures from <code>ART=art4 LOOK=ape journal/mockups/shoot.py</code>. The game's pictures from <code>tools/shots.py</code> on the build of 17 October 2026. Mage Tower Battle's pictures are from its own design journal. This tab written by <code>journal/mockups/tab10.py</code>. The mockups are labelled as such: the game draws its models with its own renderer.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r"/\*NEWLOOK\*/.*?/\*NEWLOOK-END\*/\n", "", s, flags=re.S)
s = s.replace("</style>", CSS + "</style>", 1)
s = re.sub(r'\n    <button role="tab" id="t-newlook".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-newlook" aria-controls="p-newlook" aria-selected="false" tabindex="-1"><small>10 · 17 Oct</small><span>The newer look</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 10 -->.*?<!--/TAB 10-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 10-->\n<!--PANELS-->", 1)
s = s.replace("last entry 16 October 2026", "last entry 17 October 2026")
for need in ('id="t-newlook"', 'id="p-newlook"', "/*NEWLOOK*/"):
    assert s.count(need) == 1, need
J.write_text(s)
print("tab 10 written:", len(IDS), "models")
