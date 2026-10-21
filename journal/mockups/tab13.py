"""Writes journal tab 13 (round 7, triage T36: a third added hero, after
Ancient Apparition) into ../pots-journal.html.

    cargo run -q -p pots_core --example tips > journal/art7/tips.json
    python3 tab13.py

Pictures: ../art7/models/ (the mockup, `ART=art7 LOOK=ape shoot.py models
hero_shiver`), ../art7/game/ (the game's own model viewer, `tools/shots.py`),
../art7/abilities/ (each ability cast in the game, `tools/castgif.py`),
../art7/icons/ (the TikZ icons). The ability text is the core's own tip.
Fails if a picture is missing. Replaces the tab if it is already there.
"""
import html, json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art7"
e = lambda s: html.escape(str(s), quote=True)
TIPS = {t["id"]: t for t in json.loads((ART / "tips.json").read_text())}
missing = []


def need(path):
    if not (ART / path).exists():
        missing.append(path)
    return f"art7/{path}"


def pic(path, cap):
    src = need(path)
    return (f'<button class="zoom" data-full="{src}" data-cap="{e(cap)}">'
            f'<img loading="lazy" src="{src}" alt="{e(cap)}"></button>')


REQUEST = ("Ok, now I want to add a new character to the game; ancient apparition from dota 2. do an analysis and mock up of "
           "ancient apparation from the original dota in the house style in the log, then get them working in pots, add them to "
           "the heroes screen, make sure they have animations, new projectile for them, etc")

# what you see and how you can tell, by ability (the rule text is the core's tip)
SEE = {
    "cold_feet": ("Frost spreads on the ground where the enemy stood, a thin cold line runs to its feet, and ice spikes grow round the spot as the seconds pass. A faint ring shows how far it must go to break free.",
                  "If the enemy is still inside the ring after 4 seconds, it is frozen in a block of ice. Walking out of the ring breaks the ice and stops the hits."),
    "ice_vortex": ("A pale disc with three arms of snow turning inward. Enemies in it get the frost ring of a slow.",
                   "The vortex itself is the area: inside it, spells hurt more. It lasts 12 seconds and does not stack with a second one."),
    "chilling_touch": ("A frost ring opens round the point, and each allied hero in it gets ice shards circling its chest, one for each charge.",
                       "A shard goes with each attack on an enemy hero, and a frost flash marks the extra hit. Attacks on creeps keep the shards."),
    "ice_blast": ("A small star of hail flies off across the map. When it is released, a chunk of ice falls from Shiver to the star, a ring on the ground showing where it will burst, then bursts in a ring of ice spikes.",
                  "The longer the star flew, the wider the ring. Frostbitten enemies have a snowflake over their heads and frost at their feet. Enemies never see the star."),
    "ice_blast_release": ("The star stops where it is and the hail falls on it.",
                          "Once the hail is falling, the button is Hailstar again, cooling down from the first cast."),
}

HERO = {"id": "shiver", "name": "Shiver", "after": "Ancient Apparition (Kaldr)",
        "abilities": ["cold_feet", "ice_vortex", "chilling_touch", "ice_blast", "ice_blast_release"],
        "look": "An old snow owl woken from a glacier. It floats on a tail of frost mist with no feet on the ground, and has a round head with a pale facial disc and two wide frost eyes. Its ear tufts are icicles, its brows and mantle are in its side's colour, and its wings are icicle feathers. It holds a snowflake in one wing. Not a skeleton and not a spirit of the dead.",
        "stats": "Intelligence 25 + 2.6, agility 20 + 2.2, strength 18 + 1.4; 492 health, 325 mana; ranged 600 at 1.7 s base attack time, a snowflake shot flying 1250 a second; 295 move speed; 1800 sight by day, 1400 by night."}

VERSION = [
    ["Ancient Apparition", "Added in 6.65; in 6.72/6.73 only his armor changed (6.72) and a few bugs were fixed. The playdota page reads the same from June 2011 to January 2012. Built as of 6.72/6.73.", "[PD-AA-6.72], [CL-6.72], [D2W-AACL]"],
    ["Chilling Touch", "The old buff on allied heroes: 2–5 attacks with 40–70 more magic damage, on hits against heroes only, the ally attacking 15% slower. Dota 2's orb on Kaldr's own attacks came in 7.20.", "[PD-AA-6.72], [D2W-AACL]"],
    ["Ice Blast", "Radius 225 + 50 a second of the tracer's flight, at most 1000; no Aghanim's Scepter (6.78); the Fountain clears frostbite (until 6.75).", "[PD-AA-6.72], [CL-6.75]"],
]

UNVERIFIED = [
    "The hail's speed: at least 750 a second, landing within about 2 s (guides); built as min(distance ÷ 750, 2 s).",
    "An unreleased tracer: here it waits at the map's edge. If Shiver dies, it is released (as in the Dota 2 port).",
    "Frostbite's tick: here smooth, as every damage over time in the game is.",
    "Cold Feet's hits come every 0.85 s here, against the source's 0.8, 1.6, 2.5 and 3.4 s.",
    "Turn rate (0.6, the port's) and acquisition range (800).",
    "Ice Vortex's vision and the tracer's vision are not built: sight in the game comes from units.",
]

STEPS = [
    ("Rules", "All four abilities and Release in the core with their 6.72/6.73 numbers; 11 tests with values worked out by hand, 21 break rows, all caught.", "Done"),
    ("On the roster", "The thirteenth hero: the pick screen, the deal, the computer players and online rooms include it (online version 8).", "Done"),
    ("Look", "The model in the game, floating and bobbing, from the mockup; a portrait and five icons; a snowflake shot; the frost patch, the snow vortex, the hail star, the falling hail and its burst as zones; ice shards for Frost Talons' charges and a snowflake over frostbitten heads.", "Done"),
    ("Heroes screen", "Shiver's card opens the inspector; each ability plays on loop, Hailstar released by itself once its star is over the creeps.", "Done"),
    ("Computer players", "Cast Frozen Ground, Snow Whirl and Hailstar at heroes within 900, Frost Talons round itself before a fight, and Release once an enemy hero stands where the burst would land.", "Done"),
]

QUESTIONS = [
    ["R7.55", "The name and look: <b>Shiver</b>, a snow owl, and its ability names: Frozen Ground, Snow Whirl, Frost Talons, Hailstar, Hailfall. Keep, or change any?"],
    ["R7.59", "Should a computer Shiver fire Hailstar across the map at a far-off fight? Today it casts only at heroes within 900, like every other spell."],
    ["R7.44", "Open Shiver on the heroes screen and play each ability: does every one read clearly?"],
]


def ability_card(aid):
    t = TIPS[aid]["tip"]
    see, tell = SEE[aid]
    lines = " · ".join(e(x) for x in t.get("lines", []))
    gif = need(f"abilities/{aid}.gif")
    return f'''<article class="acard">
  <header><img class="aicon" src="{need(f"icons/{aid}.png")}" alt="" width="44" height="44"><div><h3>{e(t["title"])}</h3><div class="ameta">{e(t.get("sub", ""))}</div></div></header>
  <button class="zoom gif" data-full="{need(f"abilities/{aid}.jpg")}" data-cap="{e(t["title"])}: in the game"><img loading="lazy" src="{gif}" alt="{e(t["title"])} cast in the game: {e(see)}" width="480" height="270"></button>
  <p><b>What you see.</b> {e(see)}</p>
  <p><b>How you can tell.</b> {e(tell)}</p>
  <p class="rule">{e(t["text"])} <span>{lines}</span></p>
</article>'''


def table(rows):
    return "".join(f"<tr>{''.join(f'<td>{c}</td>' for c in r)}</tr>" for r in rows)


h = HERO
cards = "".join(ability_card(a) for a in h["abilities"])
hero_block = f'''<div class="hgroup"><div class="hhead"><img class="aicon big" src="{need(f"icons/hero_{h["id"]}.png")}" alt="" width="56" height="56"><h3>{e(h["name"])}</h3></div>
  <p class="lede" style="font-size:15px">Plays as {e(h["after"])}. {e(h["stats"])}</p>
  <div class="agrid">{cards}</div></div>'''

looks = f'''<figure class="gpair"><div class="gpair-row"><div><span class="chip">mockup</span>{pic("models/hero_shiver.jpg", "Shiver: mockup")}</div><div><span class="chip acc">in the game</span>{pic("game/shiver.jpg", "Shiver: in the game")}</div></div>
  <figcaption><b>Shiver</b> (abilities of {e(h["after"])}). {e(h["look"])}</figcaption></figure>
  <figure class="gpair"><div>{pic("game/lineup.jpg", "Shiver beside Rime, Hackle, Chipper and a Sunward creep, in the game")}</div><figcaption><b>Beside the roster.</b> Shiver with Rime (the other frost hero), Hackle, Chipper and a Sunward creep, in the game's model viewer. Shiver is the palest and the only one off the ground.</figcaption></figure>'''

PANEL = f'''<!-- ======================================================= TAB 13 -->
<div class="panel-tab" role="tabpanel" id="p-shiver" aria-labelledby="t-shiver" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 13 · 20 October 2026 · a third added hero (triage T36)</div>
  <h2>Shiver</h2>
  <p class="lede">A hero that plays as DotA's Ancient Apparition, with a name and a look of its own. Part 1 is the literature review. Part 2 is its look. Part 3 is every ability, cast in the game itself. Part 4 is its attack and shots. Part 5 is what is built and three questions for you. Unlike tab 12's pair, Shiver is already on the roster: you can choose it, play against it and open it on the heroes screen. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 1</div><h2>How Kaldr played in DotA</h2></div>
  <p>From IceFrog's changelogs and playdota's hero page read through the Wayback Machine, the Dota 2 Wiki's DotA-era changelog, and 2010–2011 player guides; every number with its source in <code>notes/LITERATURE.md</code>, section 8. Several of Dota 2's numbers are later: Cold Feet's 715, its hits each second, Chilling Touch as an orb, Ice Blast's 275 base radius.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>As of 6.72/6.73</th><th>Sources</th></tr></thead>
    <tbody>{table([[f"<b>{e(a)}</b>", e(b), f"<code>{e(c)}</code>"] for a, b, c in VERSION])}</tbody>
  </table></div>
  <div class="grid2">
    <div class="card"><h3>As players described him</h3><ul>
      <li>A lane bully and ganker who also supports: "Cold Feet ... is the crux of your laning stage."</li>
      <li>Cold Feet with Ice Vortex: "if the enemy hesitates to run for even a second, it will activate."</li>
      <li>Max Cold Feet first, Ice Vortex beside it, Ice Blast at 6, 11 and 16, Chilling Touch last.</li></ul></div>
    <div class="card"><h3>Ice Blast in their words</h3><ul>
      <li>Hit as many heroes as possible, with Ice Vortex first (its 1500 range makes that easy).</li>
      <li>"As long as the carry is hit with the DoT, that is usually enough, because now they can't life steal, or be healed."</li>
      <li>A global finisher, timed by watching the tracer on the minimap.</li></ul></div>
  </div>
  <h3 class="sub">Not settled by any source, and what the game uses</h3>
  <ul>{"".join(f"<li>{e(u)}</li>" for u in UNVERIFIED)}</ul>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 2</div><h2>Its look</h2></div>
  <p>Not a skeletal frost spirit, and nothing borrowed from Blizzard or Valve. It is round and faceted like the other twelve, with an outline that reads from the raised camera: the owl's head and ear tufts, two wide wings, and no legs. In the game it floats, its body rising and falling every two seconds and its wings beating a little.</p>
  {looks}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 3</div><h2>Every ability, cast in the game</h2></div>
  <p>Each animation is the game itself: its cast stage puts Shiver at full levels beside an ally, three enemy creeps and an enemy hero, and the frames are the following ticks (<code>tools/castgif.py</code>). The rule under each is the game's own tip. Hailstar's button turns into Hailfall (Release) while the star flies, so Shiver shows five. The cast stage releases the star by itself once it is over the creeps.</p>
  {hero_block}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 4</div><h2>Attack and shots</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>How it looks</th><th>Seen in</th></tr></thead>
    <tbody>{table([
        ["<b>Shiver's attack</b>", "A snowflake of six ice arms turning as it flies, 1250 a second, in a cold halo with a trail of frost mist; the wing that holds it swings forward to throw (0.45 s wind-up, 0.3 s follow-through).", "Snow Whirl's animation, and the heroes screen's Attack"],
        ["<b>Hailstar's star</b>", "A small snowflake with a bright core and a short streak back toward Shiver, 1500 a second, drawn only for Shiver's side.", "Hailstar"],
        ["<b>The hail</b>", "A tumbling chunk of faceted ice with a trail, falling from where Shiver stood to the star, a ring on the ground at the size of the burst to come.", "Hailstar, Hailfall"],
        ["<b>The burst</b>", "A ring of ice opening to the burst's radius and eight ice spikes thrown up and sinking back.", "Hailstar, Hailfall"],
    ])}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 5</div><h2>What is built</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>Step</th><th>What</th><th>State</th></tr></thead>
    <tbody>{table([[e(a), b, e(c)] for a, b, c in STEPS])}</tbody>
  </table></div>
  <h3 class="sub">Yours to decide, by number</h3>
  <div class="table-scroll"><table>
    <thead><tr><th>#</th><th>Question</th></tr></thead>
    <tbody>{table(QUESTIONS)}</tbody>
  </table></div>
</section>
<footer>Mockup: <code>journal/mockups/heroes3.js</code> (three.js r160, MIT), pictures from <code>ART=art7 LOOK=ape shoot.py</code>. In-game pictures: <code>tools/shots.py</code> and <code>tools/castgif.py</code> on the build of 20 October 2026. Icons: <code>tools/art/</code> (TikZ). Tips: <code>cargo run -p pots_core --example tips</code>. This tab is written by <code>journal/mockups/tab13.py</code>.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r'\n    <button role="tab" id="t-shiver".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-shiver" aria-controls="p-shiver" aria-selected="false" tabindex="-1"><small>13 · 20 Oct</small><span>Shiver</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 13 -->.*?<!--/TAB 13-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 13-->\n<!--PANELS-->", 1)
s = re.sub(r"last entry \d+ October 2026", "last entry 20 October 2026", s, count=1)
for need_ in ('id="t-shiver"', 'id="p-shiver"', "last entry 20 October 2026"):
    assert s.count(need_) == 1, need_
J.write_text(s)
print("tab 13 written")
