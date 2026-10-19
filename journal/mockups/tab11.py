"""Writes journal tab 11 (round 7, triage T20 to T22, and the person's ask
for the one change that would make the game look most professional) into
../pots-journal.html.

    python3 tab11.py

Pictures: ../art5/, every one from the game's own renderer through
`tools/shots.py` with a fixed seed (7), so each before and after is the
same second of the same match. The "after" pictures come from the branch
`mockup/smooth-ground`, where `data/presentation.json` `map_look.smooth`
turns on the faceted ground; the "before" ones from the same branch with
it off. Fails if a picture is missing. Replaces the tab if it is there.
"""
import html, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art5"
e = lambda s: html.escape(str(s), quote=True)

NAMES = ["river", "lane", "base", "road"]
BUILT_NAMES = ["b_cliff", "b_river", "b_lane", "b_heroes1", "b_heroes2", "b_lineup"]
missing = [f"{n}_{k}.jpg" for n in NAMES + BUILT_NAMES for k in ("before", "after") if not (ART / f"{n}_{k}.jpg").exists()]
missing += [f"{n}.jpg" for n in ("b_fight", "b_casts", "b_title") if not (ART / f"{n}.jpg").exists()]


def pic(path, cap):
    return (f'<button class="zoom" data-full="art5/{path}.jpg" data-cap="{e(cap)}">'
            f'<img loading="lazy" src="art5/{path}.jpg" alt="{e(cap)}"></button>')


def pair(n, la, lb, title, cap):
    return f'''<figure class="gpair">
  <div class="gpair-row"><div><span class="chip">{e(la)}</span>{pic(f"{n}_before", f"{title}: {la}")}</div><div><span class="chip acc">{e(lb)}</span>{pic(f"{n}_after", f"{title}: {lb}")}</div></div>
  <figcaption><b>{e(title)}.</b> {cap}</figcaption>
</figure>'''


def table(rows):
    return "".join(f"<tr>{''.join(f'<td>{c}</td>' for c in r)}</tr>" for r in rows)


REQUEST = ("add the ability to sell to the game, more graphics for the ground in the lane instead of one just mono color strip, "
           "and the ARAM mode should start you at level 3, have increased XP generation, increased gold per second, to match the speed "
           "and gameplay of like league of legends aram but a little slower since we're in dota. [...] I want you to do an analysis over "
           "the game compared to how actual dota and nice games look, and recommend the biggest change we could do to make this game look "
           "more professional and stylized in the current cutesy style, rendered in before and after mockups")

BUILT = [
    ["Selling (T20)", "Near a shop, right-click an item you carry to sell it for half its whole price. Its tip says what it sells for, and the shop's empty panel says how. The rule was in the game from round 1, and the computer players used it, but no control reached it."],
    ["The lane's ground (T21)", "The dirt changes shade in soft patches a few cells across. Each edge blends into the grass on both sides, grass creeps onto the edge, and worn flagstones lie down the middle. The pair below shows it."],
    ["All Random All Mid's pace (T22)", "Heroes start at level 3 with three points to spend and full health. They earn one and a half times the experience and twice the gold each second. League's ARAM pays about 2.7 times its usual gold and starts at level 3, so this is a little slower, as asked. The numbers are in <code>data/rules.json</code>; tell me if it plays too fast or too slow (R7.3)."],
]

GAPS = [
    ["1", "<b>The ground</b>",
     "Dota 2 paints its ground as layers that blend into each other, with sculpted cliffs and a shoreline on the river. League's maps are painted by hand, with every edge curved. Stylised low-poly games such as <i>Bad North</i>, <i>Islanders</i> and <i>Townscaper</i> build their ground as one faceted surface.",
     "A block for every 64-unit cell. Every edge between grass, lane, river, base and fog is a staircase of squares. When only the ground changes, 25 to 51% of a frame changes (measured on the pictures below).",
     "Medium"],
    ["2", "<b>The forest's outline</b>",
     "Tree lines are clumps of different heights with a ragged edge and a dark floor beneath, so the forest reads as a mass.",
     "One tree per cell, in rows, within a tenth of each other's size. From the game's camera the rows show.",
     "Medium"],
    ["3", "<b>Contact with the ground</b>",
     "Every unit and tree casts a shadow, so nothing looks pasted on.",
     "Map objects stand on a soft disc (T19). Heroes and creeps have only their team ring.",
     "Small"],
    ["4", "<b>Ground a step darker than the units</b>",
     "The ground is less bright and less saturated than the heroes, so a hero reads at a glance, cute palette or not.",
     "The lane's tan is as bright as most heroes. The palette suits the look; the ground needs to step back.",
     "Small"],
    ["5", "<b>The fog's edge</b>",
     "A soft gradient between seen and unseen ground.",
     "Unseen cells darken as squares.",
     "Comes with 1"],
    ["6", "<b>Things in the lane</b>",
     "Ruins, banners, lamps and walls at the towers mark where you are.",
     "The lane is bare apart from this round's stones.",
     "Medium"],
]

HOW = [
    "Corners every half cell (32 map units). Each corner takes its height and colour from the cells around it, so the edge between two kinds of ground is a slope and a gradient.",
    "Height blends over a narrower span than colour, so a cliff stays a short, steep face while its colours blend.",
    "Each corner gets a small seeded nudge in height and shade, the same in every browser, so the facets catch the light unevenly, as the faceted trees and towers do.",
    "Each triangle is lit by its own slope and turns to rock where it is steeper than about 35°.",
    "The forest floor is darker under standing trees.",
    "The water is one flat sheet over a sunken riverbed. Where the banks slope down into it, the shoreline follows the slope.",
]

STEPS = [
    ("The ground surface", "The prototype on the branch <code>mockup/smooth-ground</code>: <code>scene::ground</code> stretches the core's one-triangle mesh onto each facet. No new page code, shader or library.", "Medium (exists as a prototype)"),
    ("Cliffs", "A rock edge where high ground meets low ground, so the step reads. The prototype blends it away at the Moonward base (third pair).", "Small to medium"),
    ("Standing on it", "Units, trees and buildings stand on the blended height. Heights are drawing only, so the rules, pathing and online play do not change.", "Small"),
    ("Cost", "Shapes in a frame rise from 22,223 to 37,796 on the river picture and from 16,881 to 29,222 at the lane, but each new shape is one triangle where a block was twelve. Build the surface once, and again only when the camera crosses a cell or a tree falls; measure a 5v5's frame rate (60 a second now).", "Small"),
    ("Checks", "Tests with hand-worked heights and break rows; the browser check's pictures retaken.", "Small"),
]

BUILT2 = [
    ("b_cliff", "The Sunward base's east cliff", "The plateau keeps its edge and drops within half a cell; the face turns to rock and boulders line its foot. Before: a step of blocks."),
    ("b_river", "The river at the middle of the map", "One water sheet over a sunken bed; the banks slope into it, so the shoreline follows the slope."),
    ("b_lane", "The middle lane", "The lane's patches, edges and flagstones on the faceted surface, under the lower, warmer sun and the haze."),
    ("b_heroes1", "Heroes, first five", "Every round part faceted, no outline; glowing parts stay smooth. The game's own model viewer."),
    ("b_heroes2", "Heroes, second five", "Tectus, Kestrel, Hexwright, Longwick and Tempest."),
    ("b_lineup", "Creeps, a tower and the shop", "Characters and buildings now share one look. (The model viewer draws every model at one scale; in a match lane creeps are drawn at 85%.)"),
]

QUESTIONS = [
    ["R7.8", "Build the faceted ground into the game, as in the pairs above? <b>Answered:</b> \"build the faceted ground with the rock cliff edges, also update all the models in the game to use the mage tower visuals\". Built; see Part 5."],
    ["R7.9", "After it, which next: the forest's outline (2) or a darker ground (4)? Shadows under heroes and creeps (3) came with Part 5."],
    ["R7.2", "Selling is instant on a right-click and pays half. Should it ask first, or give the full price back for a few seconds after buying?"],
    ["R7.3", "Play All Random All Mid at the new pace. Too fast, too slow, or right?"],
]

CSS = """/*GROUND*/
.gaps td:first-child { font-weight: 700; color: var(--muted); }
.gaps td:nth-child(2) { white-space: nowrap; }
.how { display: grid; gap: 6px; padding-left: 20px; margin: 0; }
.gpair { margin: 0 0 22px; display: grid; gap: 8px; }
.gpair-row { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
.gpair-row > div { display: grid; gap: 6px; align-content: start; }
.gpair-row .chip { justify-self: start; }
.gpair img { width: 100%; height: auto; display: block; border-radius: 8px; }
.gpair figcaption { font-size: 14.5px; max-width: 72ch; }
@media (max-width: 640px) { .gpair-row { grid-template-columns: minmax(0, 1fr); } }
/*GROUND-END*/
"""

PANEL = f'''<!-- ======================================================= TAB 11 -->
<div class="panel-tab" role="tabpanel" id="p-ground" aria-labelledby="t-ground" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 11 · 18 October 2026 · selling, the lane's ground, All Random All Mid's pace (T20 to T22), a look study (T23), then the look built (T24, T25)</div>
  <h2>One surface for the ground</h2>
  <p class="lede">Part 1 is what was built and published this round. Part 2 compares the game with Dota 2, League of Legends and well-made stylised games, and ranks what separates them. Part 3 recommends the biggest change, a faceted ground, and shows it before and after in the game's own renderer. Part 4 says what building it would take, with questions for you. Part 5 is what was built once you answered: the faceted ground, and every model in tab 10's look. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 1 · built and live</div><h2>This round in the game</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>How it works</th></tr></thead>
    <tbody>{table([[f"<b>{e(a)}</b>", b] for a, b in BUILT])}</tbody>
  </table></div>
  {pair("road", "before", "built", "The lane at the Sunward outer tower", "The same second of the same match. Before: one tan per cell with a staircase edge. Built: soft patches of darker and lighter dirt, edges blended both ways, grass creeping in, flagstones down the middle.")}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 2</div><h2>Where the game stands</h2></div>
  <p>Ranked by how much of the screen each gap covers and how much it says "unfinished". The descriptions of other games are in words, as in tab 10, because this repository is public.</p>
  <div class="table-scroll"><table class="gaps">
    <thead><tr><th>#</th><th>Gap</th><th>Polished games</th><th>This game now</th><th>Cost</th></tr></thead>
    <tbody>{table(GAPS)}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 3 · the recommendation</div><h2>A faceted ground</h2></div>
  <p>The biggest change is to draw the ground as one faceted surface instead of a block per cell. The ground is the largest thing on screen, and its staircase edges are the plainest sign of an unfinished game. The change fixes gaps 1 and 5 at once and makes room for 4. It also finishes the look you chose for the map in T19: the trees, towers and rocks are already faceted with no outline, and the ground is the one map piece still drawn as blocks. The characters keep their look.</p>
  <h3 class="sub">How the prototype draws it</h3>
  <ol class="how">{"".join(f"<li>{h}</li>" for h in HOW)}</ol>
  <p>Every picture below is the game's own renderer, not a separate mockup: the same seed and the same second, with only the ground changed.</p>
  <div>
    {pair("river", "blocks", "faceted", "The river at 95 seconds", "The river's square edges become a shoreline. The banks slope into the water, the fog's edge is soft, and the lane curves into the ford. 51% of the frame changes.")}
    {pair("lane", "blocks", "faceted", "The Sunward outer tower at 20 seconds", "The lane's staircase edge becomes a curve, and the forest floor darkens under the trees. 25% of the frame changes.")}
    {pair("base", "blocks", "faceted (needs work)", "The edge of the Moonward base", "The work still to do. The base's raised edge reads in blocks, but the prototype blends it into a slope you cannot see. High ground matters to play, so the built version needs a rock edge here (Part 4).")}
  </div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 4</div><h2>What building it would take</h2></div>
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
<section>
  <div class="section-head"><div class="eyebrow">Part 5 · built (T24, T25)</div><h2>The faceted ground, and every model in tab 10's look</h2></div>
  <p>You asked for the faceted ground with rock cliff edges, and every model in tab 10's look. In the game now: the ground is one faceted surface whose cliffs keep their edge and turn to rock, with boulders along their foot. Every model, heroes and creeps included, is faceted and drawn without an outline, standing on a soft shadow; glowing parts stay smooth. The light is tab 10's late afternoon: a lower, warmer sun from the Sunward corner, a cool sky fill, a green bounce from the grass and haze the colour of the far sky. Lane creeps are drawn at 85% of their size (T25). Every ability still shows: all 41 are cast in the tests, and no shape anywhere is outlined. Each pair is the game before and after, from the same spot; matches pick their own seed, so the pairs are fixed map spots and the model viewer.</p>
  <div>{"".join(pair(n, "before", "built", t, c) for n, t, c in BUILT2)}</div>
  <figure class="gpair"><div>{pic("b_fight", "A lane fight in the built game")}</div><figcaption><b>In a match.</b> A hero by the Sunward outer tower, the base cliff with its boulders behind.</figcaption></figure>
  <figure class="gpair"><div>{pic("b_casts", "Four abilities in the built game")}</div><figcaption><b>Abilities in the new look.</b> Vow Hammer, Rift Stomp's faceted rocks, Carrion Cloud and Winter's Ring, from the game's cast stage.</figcaption></figure>
</section>
<section>
  <div class="section-head"><div class="eyebrow">Part 6 · built (T26 to T30)</div><h2>A match behind the title, and four fixes</h2></div>
  <p>As in Ape Race and Mage Tower Battle, the title screen has the game moving behind it: ten computer players play a match of their own, and the camera follows the busiest fight, framing it right of the menu and cutting to the next one every few seconds. It starts as the first creep waves meet. With reduced motion set in the browser it shows a still, and a machine too slow to draw it keeps the plain title.</p>
  <figure class="gpair"><div>{pic("b_title", "The title screen with a match behind it")}</div><figcaption><b>The title screen.</b> A fight at the river, six seconds after the page opens.</figcaption></figure>
  <div class="table-scroll"><table>
    <thead><tr><th>Ask</th><th>Now</th></tr></thead>
    <tbody>
      <tr><td><b>Typing in the boxes</b> (T28)</td><td>Every key stays. Each key used to rebuild the box empty and take the focus away, so only the first letter stayed.</td></tr>
      <tr><td><b>Your name in the lobby</b> (T27)</td><td>A name box in the lobby until the match starts; every browser in the room sees the new name.</td></tr>
      <tr><td><b>Gold on death</b> (T29)</td><td>None is lost.</td></tr>
      <tr><td><b>How to sell</b> (T30)</td><td>Near a shop, right-click an item you carry. With the shop open, each item shows what it sells for.</td></tr>
    </tbody>
  </table></div>
</section>
<footer>Pictures from <code>tools/shots.py</code> with <code>seed=7</code>. The "faceted" ones come from the branch <code>mockup/smooth-ground</code> (not published), with <code>map_look.smooth</code> on. The share of a frame that changes counts the pixels that differ between the two pictures, leaving out the minimap. Shape counts come from <code>cargo run --release -p pots_core --example count</code> on that branch. This tab is written by <code>journal/mockups/tab11.py</code>.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r"/\*GROUND\*/.*?/\*GROUND-END\*/\n", "", s, flags=re.S)
s = s.replace("</style>", CSS + "</style>", 1)
s = re.sub(r'\n    <button role="tab" id="t-ground".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-ground" aria-controls="p-ground" aria-selected="false" tabindex="-1"><small>11 · 18 Oct</small><span>One surface for the ground</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 11 -->.*?<!--/TAB 11-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 11-->\n<!--PANELS-->", 1)
s = s.replace("last entry 17 October 2026", "last entry 18 October 2026")
for need in ('id="t-ground"', 'id="p-ground"', "/*GROUND*/", "last entry 18 October 2026"):
    assert s.count(need) == 1, need
J.write_text(s)
print("tab 11 written")
