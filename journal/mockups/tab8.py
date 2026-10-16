"""Writes journal tab 8 (the round-2 art style) into ../pots-journal.html.

    python3 tab8.py

Reads the pictures in ../art2/, the descriptions the mockups export
(../art2/notes.json, from `shoot.py notes`), the ability tips the core
prints (../art2/tips.json, from `cargo run -p pots_core --example tips`),
and the names in data/. Copies the ability and hero icons into
../art2/icons/. Replaces the tab if it is already there. Fails if a model
or an ability has no picture or no description.
"""
import html, json, pathlib, re, shutil, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art2"

data = lambda n: json.loads((ROOT / "data" / n).read_text())
strings = data("strings.json")
units = data("units.json")
units = units.get("units", units)
models = data("models.json")
models = models.get("models", models)
notes = json.loads((ART / "notes.json").read_text())
tips = {t["id"]: t for t in json.loads((ART / "tips.json").read_text())}
abilities = {a["id"]: a for a in (lambda d: d.get("abilities", d))(data("abilities.json"))}
e = lambda s: html.escape(str(s), quote=True)
missing = []

# ------------------------------------------------------------ names
name = {}
for u in units:
    m = u.get("model")
    if m and m not in name:
        n = strings["units"].get(u["id"], {})
        n = n.get("name") if isinstance(n, dict) else n
        if u["id"] in strings.get("heroes", {}):
            n = strings["heroes"][u["id"]]["name"]
        if n:
            name[m] = re.sub(r" Outer Tower$", " Tower", n)
for m, n in {"critter": "Hexed critter", "tree_sun": "Sunward pine", "tree_moon": "Moonward frost tree", "rune_haste": "Haste rune",
             "rune_double": "Double-damage rune", "rune_illusion": "Illusion rune", "rune_invis": "Invisibility rune", "rune_regen": "Regeneration rune"}.items():
    name.setdefault(m, n)

GROUPS = [
    ("Heroes", "Ten heroes, each with its round-1 cue", [k for k in models if k.startswith("hero_")]),
    ("Lane creeps", "Smaller than a hero, simpler, and one look a side", ["creep_melee_sun", "creep_ranged_sun", "creep_siege_sun", "creep_melee_moon", "creep_ranged_moon", "creep_siege_moon"]),
    ("Buildings", "Drums, domes and cones in place of boxes", ["tower_sun", "tower_moon", "barracks_melee_sun", "barracks_ranged_sun", "barracks_melee_moon", "barracks_ranged_moon", "sphere_sun", "sphere_moon", "fountain_sun", "fountain_moon", "shop"]),
    ("The forest", "Neutral camps, the ancients and the Warden", ["imp", "hound", "mossback", "golem_small", "golem", "hornet", "shaman", "drake_young", "drake", "warden"]),
    ("Small things", "Wards, the totem, the courier, the rift stone, the critter, trees and runes", ["observer_ward", "sentry_ward", "healing_ward", "courier", "fissure_rock", "critter", "tree_sun", "tree_moon", "rune_haste", "rune_double", "rune_illusion", "rune_invis", "rune_regen"]),
]
assert sorted(sum((g[2] for g in GROUPS), [])) == sorted(models), "every model is in one group"


def model_card(mid):
    img = ART / "models" / f"{mid}.jpg"
    if not img.exists():
        missing.append(f"picture of {mid}")
    note = notes["models"].get(mid)
    if not note:
        missing.append(f"description of {mid}")
    return (f'<figure class="mcard"><button class="zoom" data-full="art2/models/{mid}.jpg" data-cap="{e(name.get(mid, mid))}: {e(note or "")}">'
            f'<img loading="lazy" src="art2/models/{mid}.jpg" alt="{e(name.get(mid, mid))} in the new style: {e(note or "")}"></button>'
            f'<figcaption><b>{e(name.get(mid, mid))}</b> <code>{e(mid)}</code><span>{e(note or "")}</span></figcaption></figure>')


KEY = {"KeyQ": "Q", "KeyW": "W", "KeyE": "E", "KeyR": "R"}
KIND = {"unit": "single target", "point": "aimed at the ground", "none": "no target", "passive": "passive", "toggle": "toggle"}
icons = ART / "icons"
icons.mkdir(exist_ok=True)


def ability_card(aid, hero_name):
    t = tips[aid]
    tip = t["tip"]
    n = notes["abilities"].get(aid, {})
    for k in ("see", "tell"):
        if not n.get(k):
            missing.append(f"{k} note of {aid}")
    for ext in ("gif", "jpg"):
        if not (ART / "abilities" / f"{aid}.{ext}").exists():
            missing.append(f"{ext} of {aid}")
    src = ROOT / "web" / tip["icon"]
    if src.exists():
        shutil.copyfile(src, icons / src.name)
    a = abilities[aid]
    kind = KIND.get(a["target"], a["target"])
    if a.get("ultimate"):
        kind = "ultimate, " + kind
    key = KEY.get(t["key"], "")
    lines = " · ".join(e(l) for l in tip.get("lines", []))
    loop = f'{n.get("T", 0):.1f}'.rstrip("0").rstrip(".")
    return f'''<article class="acard">
  <header><img class="aicon" src="art2/icons/{e(src.name)}" alt="" width="44" height="44"><div><h3>{e(tip["title"])}</h3><div class="ameta">{f'<span class="chip acc">{key}</span> ' if key else ''}{e(hero_name)} · {e(kind)}</div></div></header>
  <button class="zoom gif" data-full="art2/abilities/{aid}.jpg" data-cap="{e(tip["title"])}: the most readable moment"><img loading="lazy" src="art2/abilities/{aid}.gif" alt="{e(tip["title"])} animated: {e(n.get("see", ""))}" width="480" height="270"></button>
  <p><b>What you see.</b> {e(n.get("see", ""))}</p>
  <p><b>How you can tell.</b> {e(n.get("tell", ""))}</p>
  <p class="rule">{e(tip.get("text", ""))}{f" <span>{lines}</span>" if lines else ""} <span>Loop {loop} s.</span></p>
</article>'''


def heroes_with_abilities():
    out = []
    for u in units:
        if u.get("abilities") and u["id"] in strings["heroes"]:
            out.append((u["id"], strings["heroes"][u["id"]]["name"], u["abilities"]))
    out.append(("warden", "The Warden", ["warden_bash"]))
    return out


def ability_sections():
    parts = []
    for hid, hname, abs_ in heroes_with_abilities():
        src = ROOT / "web" / "art" / "heroes" / f"{hid}.png"
        portrait = ""
        if src.exists():
            shutil.copyfile(src, icons / f"hero_{hid}.png")
            portrait = f'<img class="aicon big" src="art2/icons/hero_{hid}.png" alt="" width="56" height="56">'
        cards = "\n".join(ability_card(a, hname) for a in abs_)
        parts.append(f'<div class="hgroup"><div class="hhead">{portrait}<h3>{e(hname)}</h3></div><div class="agrid">{cards}</div></div>')
    return "\n".join(parts)


covered = sum((h[2] for h in heroes_with_abilities()), [])
assert sorted(covered) == sorted(abilities), f"abilities not covered: {set(abilities) ^ set(covered)}"

CSS = """/*ART2*/
.mgrid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 220px), 1fr)); gap: 14px; }
.mcard { background: var(--surface); border: 1px solid var(--rule); border-radius: 12px; padding: 8px; gap: 8px; align-content: start; min-width: 0; }
.mcard img, .acard .gif img { border: 0; border-radius: 8px; }
.mcard figcaption { display: grid; gap: 3px; font-size: 13.5px; padding: 0 4px 4px; }
.mcard figcaption b { color: var(--fg); font-size: 14.5px; }
.mcard figcaption code { justify-self: start; font-size: 11px; }
button.zoom { all: unset; cursor: zoom-in; display: block; border-radius: 8px; }
button.zoom:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.agrid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 330px), 1fr)); gap: 16px; }
.acard { background: var(--surface); border: 1px solid var(--rule); border-radius: 12px; padding: 14px; display: grid; gap: 10px; align-content: start; min-width: 0; font-size: 14.5px; line-height: 1.5; }
.acard header { display: flex; gap: 12px; align-items: center; }
.acard h3 { font-size: 19px; }
.acard .gif img { width: 100%; height: auto; display: block; background: #b8dca0; }
.ameta { font-family: var(--font-mono); font-size: 11.5px; letter-spacing: .03em; color: var(--muted); }
.aicon { border-radius: 10px; border: 1px solid var(--rule); background: var(--bg); flex: none; }
.aicon.big { border-radius: 50%; }
.acard .rule { font-size: 13px; color: var(--muted); border-top: 1px solid var(--rule); padding-top: 8px; }
.acard .rule span { white-space: nowrap; }
.hgroup { display: grid; gap: 14px; }
.hhead { display: flex; gap: 12px; align-items: center; }
.hhead h3 { font-size: 24px; }
.ba { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.ba figure img { background: #9fd4ff; }
dialog.lightbox { border: 0; padding: 0; background: transparent; max-width: min(96vw, 1400px); }
dialog.lightbox::backdrop { background: rgba(10, 12, 14, .82); }
dialog.lightbox img { display: block; max-width: 100%; max-height: 82vh; border-radius: 10px; }
dialog.lightbox p { color: #f1f1ee; font-size: 14.5px; margin: 10px 2px 0; max-width: 90ch; }
dialog.lightbox button { position: absolute; top: 8px; right: 8px; font: inherit; font-size: 14px; background: rgba(20,20,24,.75); color: #fff; border: 0; border-radius: 999px; padding: 6px 12px; cursor: pointer; }
@media (max-width: 760px) { .ba { grid-template-columns: minmax(0, 1fr); } }
/*ART2-END*/
"""

def table(rows):
    return "".join(f"<tr>{''.join(f'<td>{c}</td>' for c in r)}</tr>" for r in rows)


groups_html = "\n".join(
    f'<div class="hgroup"><div class="hhead"><h3>{e(t)}</h3></div><p class="lede" style="font-size:15px">{e(sub)}</p><div class="mgrid">{"".join(model_card(m) for m in ids)}</div></div>'
    for t, sub, ids in GROUPS)

REQUEST = "I'd like to make the game more cutsy / better art style, everything is too blocky for my liking. take a look at the voxel, sphere inspired design of chumbles in long time comin, and the work currently being done on mage tower battle in the journal for inspirations of how to redesign the characters, specifically the 3d models for everything. make a new tab for these changes and make a mock up of every 3d model with the new art style, with a screenshot for each one, and then animations for every ability in the game so you can tell when they are occurring, also in the art style, with a screen shot / gif for each one and a description. do the artsttyle update analysis, make the mockups, then create the design journal tab for my review"

PANEL = f'''<!-- ======================================================= TAB 8 -->
<div class="panel-tab" role="tabpanel" id="p-art" aria-labelledby="t-art" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 8 · 16 October 2026 · the new look (triage T1)</div>
  <h2>Rounder, softer, and every ability visible</h2>
  <p class="lede">A redesign of every model in a round, chibi style after Chumbles and Mage Tower Battle, and an animation for each of the 41 abilities so a player can tell when one happens. These are mockups drawn with three.js from the same kinds of shape the game draws; the game itself has not changed yet. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Before and after</div><h2>The same ten heroes</h2></div>
  <div class="ba">
    <figure><img loading="lazy" src="img/heroes_a.jpg" alt="Round one: five hero models built from boxes on a flat tan ground"><figcaption>Round 1, drawn by the game: Ironvow, Rime, Cinder, Thistle, Gristle.</figcaption></figure>
    <figure><button class="zoom" data-full="art2/models/heroes_1.jpg" data-cap="Round 2 mockup: Ironvow, Rime, Cinder, Thistle, Gristle"><img loading="lazy" src="art2/models/heroes_1.jpg" alt="Round two: the same five heroes as round chibi figures with big heads, dot eyes and blush, on a grassy lane with soft shadows"></button><figcaption>Round 2 mockup: the same five.</figcaption></figure>
    <figure><img loading="lazy" src="img/heroes_b.jpg" alt="Round one: the other five heroes built from boxes"><figcaption>Round 1: Tectus, Kestrel, Hexwright, Longwick, Tempest.</figcaption></figure>
    <figure><button class="zoom" data-full="art2/models/heroes_2.jpg" data-cap="Round 2 mockup: Tectus, Kestrel, Hexwright, Longwick, Tempest"><img loading="lazy" src="art2/models/heroes_2.jpg" alt="Round two: the other five heroes as round chibi figures"></button><figcaption>Round 2 mockup: the same five.</figcaption></figure>
  </div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Analysis</div><h2>Where the look comes from</h2></div>
  <div class="grid2">
    <div class="card"><h3>Chumbles, in Long Time Comin'</h3><ul>
      <li>About 75 parts, every one an ellipsoid stretched between two points, and every one outlined in one ink colour.</li>
      <li>The head is drawn 1.3 times a real cat's "so his face reads".</li>
      <li>State is in the pose and the face: ears, pupils, fur bristled by scaling the tufts.</li>
      <li>Not voxels: the boxes in that game are its arenas. Taken here as "chunky and rounded" (R2.7).</li></ul>
      <div class="src">LongTimeComin/crates/core/src/draw/cat.rs</div></div>
    <div class="card"><h3>Mage Tower Battle's mockups</h3><ul>
      <li>"Characters, built from spheres (after Chumbles, not cubes)": a robe is a squashed ball under a ball.</li>
      <li>A head about 40 % of the height; two dark dot eyes with white catch-lights; pink blush.</li>
      <li>A warm key light with soft shadows, a sky fill, filmic tone mapping.</li>
      <li>Pastel but saturated colours; no outline on the models.</li></ul>
      <div class="src">MageTowerBattle/journal/mockups/mockup.js</div></div>
  </div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Analysis</div><h2>Why the game looks blocky now</h2></div>
  <p>The cubes are only one of three causes. The light is the second: the shader lights every surface in three hard bands with no shadow, so even a sphere reads as facets. The proportions are the third: small heads and thin limbs read as soldiers, not toys.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>Measure</th><th>Round 1, in the game</th><th>Round 2 mockups</th></tr></thead>
    <tbody>{table([
      ["Parts that are cubes or wedges", "278 of 546", "none"],
      ["Hero parts that are cubes or wedges", "137 of 240", "none"],
      ["A hero's head, share of its height", "18 to 28 %", "about 40 %"],
      ["Parts a hero", "22 to 26", "24 to 38"],
      ["Light", "three hard bands, no shadow", "soft key, sky fill, soft shadow"],
      ["Faces", "two dark dots or cubes", "dot eyes with a catch-light, blush"],
      ["Ability visuals", "a coloured octahedron for a missile, a tinted disc for a zone; nothing else", "an animation for each of the 41"],
    ])}</tbody>
  </table></div>
  <p>Counted from <code>data/models.json</code> and <code>data/shaders/scene.frag</code>; the full analysis is in <code>notes/ART-STYLE.md</code>.</p>
</section>

<section>
  <div class="section-head"><div class="eyebrow">The new look</div><h2>Eight rules</h2></div>
  <ol class="steps">
    <li><div><h3>Nothing alive has a corner</h3><p>Spheres, capsules, cylinders, cones and torus arcs only. Buildings are drums, domes, cones and rounded stone.</p></div></li>
    <li><div><h3>Chibi proportions</h3><p>A hero's head is about 40 % of its height, the torso 30 %, the legs 20 %, with oversized hands and feet. Creeps are smaller and simpler; buildings far bigger.</p></div></li>
    <li><div><h3>Faces carry the style</h3><p>Two dot eyes set low and wide, a catch-light in each, blush beneath. Glowing eyes mark the sinister ones.</p></div></li>
    <li><div><h3>Each hero keeps its round-1 cue</h3><p>The blade, the hood and staff, the flame hair, the bow, the belly and hook, the horns and totem, the mask, the claw staff, the long rifle, the cloud crown.</p></div></li>
    <li><div><h3>One thin warm outline</h3><p>A plum outline about 1.5 % of a hero's height round every solid part, as on Chumbles. Glows have none.</p></div></li>
    <li><div><h3>Soft light</h3><p>A warm key with soft shadows, a sky-blue fill, a warm bounce from the ground. Never hard bands.</p></div></li>
    <li><div><h3>Team colour on every unit</h3><p>Sunward gold and cream, Moonward violet and slate, and a soft ring in the side's colour under every unit. Colour is never the only cue.</p></div></li>
    <li><div><h3>Every ability in four beats</h3><p>A cast pose; a flight or a warning on the ground; an impact with a flash; then a mark on whoever is affected for as long as it lasts. The marks are the same across the game: stars for a stun, frost and a blue tint for a slow, a crossed bubble for a silence, a coloured ring for a buff.</p></div></li>
  </ol>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockups</div><h2>Every model, {len(models)} of them</h2></div>
  <p>One picture each, on a small grass stand, seen from a little above. Each caption says what the model is and the outline a player knows it by.</p>
  {groups_html}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockups</div><h2>In a lane</h2></div>
  <p>Two creep waves meeting between the towers, with two heroes a side. The first picture uses the mockups' angle; the second uses the game's own camera pitch of 56°, from which the faces are seen more from above (R2.5).</p>
  <figure><button class="zoom" data-full="art2/scenes/lane.jpg" data-cap="A lane in the new style, from the mockups' angle"><img loading="lazy" src="art2/scenes/lane.jpg" alt="A lane in the new style: two waves of round creeps fighting between a sandstone tower and a slate tower, pines and frost trees on either side, two heroes each side"></button><figcaption>From about 42°.</figcaption></figure>
  <figure><button class="zoom" data-full="art2/scenes/lane_game.jpg" data-cap="The same lane from the game's camera pitch"><img loading="lazy" src="art2/scenes/lane_game.jpg" alt="The same lane seen from higher, as the game's camera sees it"></button><figcaption>From the game's 56°.</figcaption></figure>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Mockups</div><h2>Every ability, {len(abilities)} of them</h2></div>
  <p>Each animation loops and shows the four beats. The rule under each is the game's own tip, printed by the core (<code>cargo run -p pots_core --example tips</code>). Click an animation for its sharpest frame. Big areas are drawn smaller to fit, and long effects are shortened; the description says where.</p>
  {ability_sections()}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Next round</div><h2>What the game needs to draw this</h2></div>
  <div class="grid2">
    <div class="card"><h3>Shapes and light</h3><ul>
      <li>New meshes: a capsule, torus arcs at a few fixed sweeps, and a finer sphere for heads.</li>
      <li>Soft wrap lighting and a sky fill in place of the three bands, with the numbers kept in <code>data/presentation.json</code>.</li>
      <li>A soft contact shadow under each unit first; a real shadow map later.</li>
      <li>Every model rewritten in <code>tools/modelgen.py</code>. The joints already match the mockups' rigs.</li></ul></div>
    <div class="card"><h3>Effects from the core</h3><ul>
      <li>The core records each ability's beats (cast, missile, landing, proc, status) with its tick.</li>
      <li>The scene draws each one as a function of ticks since it began, as the mockups do, with sizes and colours in a new <code>data/fx.json</code>.</li>
      <li>Status marks come from each unit's modifiers every frame, so rollback replays them and online browsers agree.</li>
      <li>A draw-list test for each mark, each with its break.</li></ul></div>
  </div>
  <p>Heroes rise from 22–26 parts to 24–38 and creeps stay near their count; the frame rate should be measured again after the port, not assumed.</p>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Yours to decide</div><h2>Open questions, by number</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>#</th><th>Question</th></tr></thead>
    <tbody>{table([
      ["R2.6", "Does this style suit the game? Name, by id, any model or animation to redo."],
      ["R2.7", "By “voxel” did you mean this rounded look, or a look built from small cubes?"],
      ["R2.5", "Should the game's camera come down from 56° so faces show, or stay where it is?"],
    ])}</tbody>
  </table></div>
</section>
<footer>Mockups in <code>journal/mockups/</code> (three.js r160, MIT), pictures made with <code>journal/mockups/shoot.py</code>, this tab written by <code>journal/mockups/tab8.py</code>. Labelled mockups: the game draws its own models with its own renderer.</footer>
<dialog class="lightbox" id="lightbox" aria-label="Larger picture"><img alt=""><p></p><button type="button" data-close>Close</button></dialog>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r"/\*ART2\*/.*?/\*ART2-END\*/\n", "", s, flags=re.S)
s = s.replace("</style>", CSS + "</style>", 1)
s = re.sub(r'\n    <button role="tab" id="t-art".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-art" aria-controls="p-art" aria-selected="false" tabindex="-1"><small>8 · 16 Oct</small><span>The new look</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 8 -->.*?<!--/TAB 8-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 8-->\n<!--PANELS-->", 1)
LIGHTBOX = """<script>
/*ART2-JS*/
(function () {
  var box = document.getElementById('lightbox');
  if (!box) return;
  document.addEventListener('click', function (ev) {
    var b = ev.target.closest('button.zoom');
    if (b) { box.querySelector('img').src = b.dataset.full; box.querySelector('img').alt = b.dataset.cap || ''; box.querySelector('p').textContent = b.dataset.cap || ''; box.showModal(); return; }
    if (ev.target.closest('[data-close]') || ev.target === box) box.close();
  });
})();
/*ART2-JS-END*/
</script>
"""
s = re.sub(r"<script>\n/\*ART2-JS\*/.*?/\*ART2-JS-END\*/\n</script>\n", "", s, flags=re.S)
i = s.rindex("<script>")
s = s[:i] + LIGHTBOX + s[i:]
for need in ('id="t-art"', 'id="p-art"', "/*ART2*/", "/*ART2-JS*/"):
    assert s.count(need) == 1, need
J.write_text(s)
print("tab 8 written:", len(models), "models,", len(abilities), "abilities")
