"""Writes journal tab 12 (round 7, triage T32: two heroes after Troll
Warlord and Timbersaw, for review before they are fully built) into
../pots-journal.html.

    cargo run -q -p pots_core --example tips > journal/art6/tips.json
    python3 tab12.py

Pictures: ../art6/models/ (the mockups, `ART=art6 LOOK=ape shoot.py
models hero_hackle hero_chipper`), ../art6/game/ (the game's own model
viewer, `tools/shots.py`), ../art6/abilities/ (each ability cast in the
game, `tools/castgif.py`), ../art6/icons/ (the TikZ icons). The ability
text is the core's own tip. Fails if a picture is missing. Replaces the tab
if it is already there.
"""
import html, json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art6"
e = lambda s: html.escape(str(s), quote=True)
TIPS = {t["id"]: t for t in json.loads((ART / "tips.json").read_text())}
missing = []


def need(path):
    if not (ART / path).exists():
        missing.append(path)
    return f"art6/{path}"


def pic(path, cap):
    src = need(path)
    return (f'<button class="zoom" data-full="{src}" data-cap="{e(cap)}">'
            f'<img loading="lazy" src="{src}" alt="{e(cap)}"></button>')


REQUEST = ("add troll warlord as a new character, and add timbersaw as a new character, add your designs, icons, assets in game, "
           "abilities, ability animations, auto attack animations and projectile assets. make sure the characters have different names "
           "and look different, but play the exact same. first do a lit review on how those characters played in the original warcraft3 "
           "dota, then create their abilities in game, then make all the assets necessary for showing me in the design log before I have "
           "you fully implement them and test.")

# what you see and how you can tell, by ability (the rule text is the core's tip)
SEE = {
    "berserker_rage": ("A red flash at Hackle's feet and a ring in fire colours while it is on; his hatchets swing instead of fly, and a bash flashes on the target.",
                       "The ring under him is the melee form. Pressing it again sends him back to throwing."),
    "whirling_axes": ("Five hatchets tumble out in a fan, leaving short trails; every enemy one passes gets the frost ring of a slow.",
                      "The fan is the cone: wide at the end, narrow at Hackle. Each enemy is hit once, whichever axe reaches it first."),
    "whirling_axes_melee": ("Two hatchets spin out round Hackle to the edge of a pale ring and back; an enemy they touch is hit and gets grey puffs circling its eyes.",
                            "The puffs are the miss chance: that enemy misses 60% of its attacks while they show. Ranged form and melee form keep separate cooldowns, so the two can be thrown back to back."),
    "fervor": ("Hackle's attacks get faster on the same target; a ring at his feet marks the bonus.",
               "No cast and no button: it builds with each hit on one target and starts again on a new one."),
    "battle_trance": ("Drum rings roll out from Hackle and every allied hero on the map gets the attack-speed ring.",
                      "The ring at an ally's feet lasts 5 seconds; on Hackle, 10."),
    "whirling_death": ("Blades spin out to a ring round Chipper; trees inside fall, and enemy heroes get a steel arrow pointing down over their heads.",
                       "The arrow is the attribute cut. If a tree fell, the hit was pure damage."),
    "timber_chain": ("The chain claw flies out with its links trailing; it shuts on a tree and Chipper is pulled along the chain, cutting enemies on the way; the tree falls.",
                     "With no tree near its path the claw comes back and nothing happens. Stunned as it grabs, Chipper stays put."),
    "reactive_armor": ("Each attack that starts on Chipper adds to the earth-coloured ring at his feet.",
                       "The ring is the armor and regeneration; each stack fades 16 seconds after the attack that made it."),
    "chakram": ("The saw blade flies to the point, cutting trees, then spins there inside a pale circle with sparks off its rim.",
                "Inside the circle enemies take damage and slow more the more health they have lost. While the blade is out Chipper cannot attack, and his ultimate's button becomes Recall Blade."),
    "return_chakram": ("The blade flies home to Chipper along its line, hitting enemies once more on the way.",
                       "Once it is back he can attack again and the button is Buzz Blade."),
}

HEROES = [
    {"id": "hackle", "name": "Hackle", "after": "Troll Warlord (Jah'rakal)", "abilities": ["berserker_rage", "whirling_axes", "whirling_axes_melee", "fervor", "battle_trance"],
     "look": "A boar brawler: a bristle crest in his side's colour, upturned tusks, war paint, a bone necklace and a hatchet in each hand. Not a troll.",
     "stats": "Agility 21 + 2.75, strength 17 + 2.2, intelligence 13 + 1; 473 health; throws from 500 at 1.7 s base attack time, swings at 128 and 1.55 in melee form; 300 move speed."},
    {"id": "chipper", "name": "Chipper", "after": "Goblin Shredder (Rizzrak; Dota 2's Timbersaw)", "abilities": ["whirling_death", "timber_chain", "reactive_armor", "chakram", "return_chakram"],
     "look": "A beaver lumberjack in a little wooden walker: a hard hat in his side's colour, big front teeth, a saw blade on his chest, a chain claw on his left arm, a blade launcher on his right shoulder and a paddle tail. Not a goblin.",
     "stats": "Strength 22 + 2.4, agility 16 + 1.3, intelligence 21 + 1.8; 568 health; melee at 128, 1.7 s base attack time; 290 move speed."},
]

VERSION = [
    ["Goblin Shredder", "Added in 6.73, not 6.74 as first thought, so he is inside the game's 6.72/6.73 pin. Built as of 6.73 (with the 6.73b growth and range fixes).", "[CL-6.73], [PD-Shred-6.73]"],
    ["Troll Warlord", "In 6.72/6.73 his kit is Berserker Rage, Whirling Axes (ranged and melee), Fervor and Battle Trance; Whirling Axes replaced Blind in 6.72 and gained its 75 damage in 6.72c. Built as of 6.73.", "[CL-6.72], [CL-6.72c], [CL-6.73]"],
]

UNVERIFIED = [
    "Timber Chain's flight and pull speed: 1600 / 2000 / 2400 / 2800 a second (the Dota 2 port's).",
    "Thrown Whirling Axes' speed, 1500, and the melee whirl's 1.5 s out and back (one guide measured 2 s).",
    "Chakram spins in half-second ticks; what happens when Chipper runs out of mana (here, it comes home).",
    "Reactive Armor's stacks each have their own timer; Fervor counts when the hit lands.",
    "Whirling Death's 15% is of the whole attribute; Troll's melee range 128; both heroes' turn rate and size (the port's).",
    "Deny experience in melee form: one guide reported more; left out as the review recommends.",
]

STEPS = [
    ("Rules", "All ten abilities in the core with their DotA numbers; 13 tests with values worked out by hand, 18 break rows, all caught. The two heroes are preview heroes: the cast stage shows them, no pick screen, deal, computer player or room can choose them.", "Done"),
    ("Look", "Both models in the game (from the mockups, faceted like the rest), portraits and ten TikZ icons, each ability's effect, the thrown hatchet and the chain claw as shots, the orbiting hatchets, the spinning blade and the pull as zones, and marks for the miss chance and the attribute cut.", "Done"),
    ("Onto the roster", "Move them from <code>preview_heroes</code> to <code>heroes</code> in <code>data/rules.json</code>; the hero grid, the deal and online rooms then include them. <code>PROTOCOL</code> goes up.", "Small"),
    ("Computer players", "Teach the behaviour trees the form switch, the chain to a tree and the blade's recall, and when to use each; measure them in bot matches.", "Medium"),
    ("Checks", "A match each in the arcade and in All Random All Mid with them; the five-runner online test; the browser check.", "Small"),
]

QUESTIONS = [
    ["R7.28", "The names and looks: <b>Hackle</b> (boar, twin hatchets) and <b>Chipper</b> (beaver lumberjack), and their ability names: Red Tusk, Hatchet Storm, Grudge, War Drums; Splinter Spin, Grapple Claw, Thick Bark, Buzz Blade, Recall Blade. Keep, or change any?"],
    ["R7.29", "Chakram's mana drain: 20 / 30 / 40 a second (6.73, as built) or 20 / 25 / 30 (6.73c)?"],
    ["R7.30", "Build them fully (roster, computer players, tests), as Part 4 lays out?"],
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


def hero_block(h):
    cards = "".join(ability_card(a) for a in h["abilities"])
    return f'''<div class="hgroup"><div class="hhead"><img class="aicon big" src="{need(f"icons/hero_{h["id"]}.png")}" alt="" width="56" height="56"><h3>{e(h["name"])}</h3></div>
  <p class="lede" style="font-size:15px">Plays as {e(h["after"])}. {e(h["stats"])}</p>
  <div class="agrid">{cards}</div></div>'''


looks = "".join(f'''<figure class="gpair"><div class="gpair-row"><div><span class="chip">mockup</span>{pic(f"models/hero_{h["id"]}.jpg", f"{h['name']}: mockup")}</div><div><span class="chip acc">in the game</span>{pic(f"game/{h['id']}.jpg", f"{h['name']}: in the game")}</div></div>
  <figcaption><b>{e(h["name"])}</b> (abilities of {e(h["after"])}). {e(h["look"])}</figcaption></figure>''' for h in HEROES)

PANEL = f'''<!-- ======================================================= TAB 12 -->
<div class="panel-tab" role="tabpanel" id="p-twoheroes" aria-labelledby="t-twoheroes" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 12 · 19 October 2026 · two heroes for review (triage T32)</div>
  <h2>Hackle and Chipper</h2>
  <p class="lede">Two heroes that play as DotA's Troll Warlord and Goblin Shredder (Dota 2's Timbersaw), with names and looks of their own. Part 1 is the literature review. Part 2 is their looks. Part 3 is every ability, cast in the game itself. Part 4 is their attacks and shots. Part 5 is what is built, what is left, and three questions for you. They are in the game's rules and art but no one can choose them yet. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 1</div><h2>How they played in DotA</h2></div>
  <p>From IceFrog's changelogs and playdota's hero pages read through the Wayback Machine, the Dota 2 Wiki's DotA-era changelogs, and 2011–2012 player guides; every number with its source in <code>notes/LITERATURE.md</code>, section 7.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>Hero</th><th>Which version</th><th>Sources</th></tr></thead>
    <tbody>{table([[f"<b>{e(a)}</b>", e(b), f"<code>{e(c)}</code>"] for a, b, c in VERSION])}</tbody>
  </table></div>
  <div class="grid2">
    <div class="card"><h3>Troll Warlord, as players described him</h3><ul>
      <li>Whirling Axes gave him burst and a chase for the first time: axes in one form, switch, axes in the other, ult, attack.</li>
      <li>Max Whirling Axes first; Berserker Rage at one point, since more only adds stun time and speed.</li>
      <li>A late carry who reaches the attack-speed cap with Fervor and Battle Trance.</li></ul></div>
    <div class="card"><h3>Goblin Shredder, as players described him</h3><ul>
      <li>"Timber chain for moving around fast, like a blink but can be used in combat, as long as you're not stunned."</li>
      <li>Almost all his damage is pure; very low cooldowns; heavy mana use from the spinning blade.</li>
      <li>Max Timber Chain first, an early point of Reactive Armor, Chakram whenever possible.</li></ul></div>
  </div>
  <h3 class="sub">Not settled by any source, and what the game uses</h3>
  <ul>{"".join(f"<li>{e(u)}</li>" for u in UNVERIFIED)}</ul>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 2</div><h2>Their looks</h2></div>
  <p>Neither is a troll or a goblin, and neither borrows a Blizzard or Valve look. Each keeps an outline a player can read from the raised camera, in the same round, faceted style as the other ten.</p>
  {looks}
  <figure class="gpair"><div>{pic("game/lineup.jpg", "Hackle and Chipper beside Ironvow, Kestrel and a Sunward creep, in the game")}</div><figcaption><b>Beside the roster.</b> Hackle and Chipper with Ironvow, Kestrel and a Sunward creep, in the game's model viewer.</figcaption></figure>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 3</div><h2>Every ability, cast in the game</h2></div>
  <p>Each animation is the game itself: its cast stage puts the hero at full levels beside an ally, three enemy creeps and an enemy hero, and the frames are the following ticks (<code>tools/castgif.py</code>). The rule under each is the game's own tip. Whirling Axes has two forms, one for each of Hackle's stances, and Chakram's button turns into Recall Blade while the blade is out, so each hero shows five.</p>
  {"".join(hero_block(h) for h in HEROES)}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 4</div><h2>Attacks and shots</h2></div>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>How it looks</th><th>Seen in</th></tr></thead>
    <tbody>{table([
        ["<b>Hackle, thrown</b>", "A hatchet tumbling end over end with a short trail, 1200 a second, his attack's own wind-up and follow-through (0.3 s each).", "Grudge's animation"],
        ["<b>Hackle, up close</b>", "The same hatchets swung, faster (1.55 s base); a bash flashes on the target and stuns it.", "Red Tusk's animation"],
        ["<b>Chipper</b>", "A swing of the walker's arms (0.36 s wind-up, 0.64 s follow-through).", "Thick Bark's animation"],
        ["<b>Hatchet Storm's axes</b>", "Bigger tumbling hatchets in a fan.", "Hatchet Storm"],
        ["<b>Grapple Claw</b>", "Three steel prongs ahead of a line of iron links back to his arm; during the pull, the chain from Chipper to the tree.", "Grapple Claw"],
        ["<b>Buzz Blade</b>", "A flat steel saw with ten teeth and a gold hub, spinning as it flies, and in its circle when it stops.", "Buzz Blade, Recall Blade"],
    ])}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 5</div><h2>What is built, and what is left</h2></div>
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
<footer>Mockups: <code>journal/mockups/heroes2.js</code> (three.js r160, MIT), pictures from <code>ART=art6 LOOK=ape shoot.py</code>. In-game pictures: <code>tools/shots.py</code> and <code>tools/castgif.py</code> on the build of 19 October 2026. Icons: <code>tools/art/</code> (TikZ). Tips: <code>cargo run -p pots_core --example tips</code>. This tab is written by <code>journal/mockups/tab12.py</code>.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r'\n    <button role="tab" id="t-twoheroes".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-twoheroes" aria-controls="p-twoheroes" aria-selected="false" tabindex="-1"><small>12 · 19 Oct</small><span>Hackle and Chipper</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 12 -->.*?<!--/TAB 12-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 12-->\n<!--PANELS-->", 1)
s = re.sub(r"last entry \d+ October 2026", "last entry 19 October 2026", s, count=1)
for need_ in ('id="t-twoheroes"', 'id="p-twoheroes"', "last entry 19 October 2026"):
    assert s.count(need_) == 1, need_
J.write_text(s)
print("tab 12 written")
