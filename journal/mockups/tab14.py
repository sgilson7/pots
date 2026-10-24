"""Writes journal tab 14 (round 8, triage T38: two added heroes, after
Vengeful Spirit and Mirana, and the arena after SMITE 2's Arena) into
../pots-journal.html.

    cargo run -q -p pots_core --example tips > journal/art8/tips.json
    python3 tab14.py

Pictures: ../art8/models/ (the mockups, `ART=art8 LOOK=ape shoot.py models
hero_tallow hero_skipjack`), ../art8/game/ (the game's own model viewer,
`tools/shots.py`), ../art8/abilities/ (each ability cast in the game,
`tools/castgif.py`), ../art8/icons/ (the TikZ icons), ../art8/arena/ (the
arena, `tools/shots.py` and the lobby). The ability text is the core's own
tip. Fails if a picture is missing. Replaces the tab if it is already there.
"""
import html, json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
J = HERE.parent / "pots-journal.html"
ART = HERE.parent / "art8"
e = lambda s: html.escape(str(s), quote=True)
TIPS = {t["id"]: t for t in json.loads((ART / "tips.json").read_text())}
missing = []


def need(path):
    if not (ART / path).exists():
        missing.append(path)
    return f"art8/{path}"


def pic(path, cap):
    src = need(path)
    return (f'<button class="zoom" data-full="{src}" data-cap="{e(cap)}">'
            f'<img loading="lazy" src="{src}" alt="{e(cap)}"></button>')


REQUEST = ("ok, now please add in pots renditions of wc3 dota vengeful spirit and mirana, make the mockups first, make sure "
           "they style match the game and are far from actual dota characters, abilities should be the same as usual, add to "
           "the heroes page, make sure the online still works, add a circular map like arena from smite 2 for online play "
           "where you can select any champion, with the ticket system, small jungle on the sides, etc")

# what you see and how you can tell, by ability (the rule text is the core's tip)
SEE = {
    "magic_missile": ("Tallow throws a glowing ember between two fluttering dusk wings, trailing sparks, straight at one enemy.",
                      "It always reaches its target. A flash and a ring where it lands; the enemy stands stunned."),
    "wave_of_terror": ("A row of dusk motes rolls out along the ground, rising and falling out of step.",
                       "Every enemy it passes loses health and armor: the armor stays down for 20 seconds, so it marks a target for the fight to come."),
    "command_aura": ("Nothing to see: an aura. Allies near Tallow, creeps too, hit harder.",
                     "The damage on an ally's tip shows the bonus while it stands within 900."),
    "nether_swap": ("A flash of dusk where Tallow stood and where the other hero stood, and the two have traded places.",
                    "Only heroes, ally or enemy; it works on a hero immune to spells and breaks its channel. Trees round both spots fall."),
    "starfall": ("A ring opens round Skipjack and stars drop toward it; when they land, a flash and a ring the size of the area.",
                 "A beam of light marks the second star on one enemy close to Skipjack."),
    "elune_arrow": ("A big four-pointed star flies out along the ground with a long pale tail.",
                    "It stops at the first enemy it meets, creeps too. The farther it flew, the longer the stun: half a second close, five seconds from 1500."),
    "leap": ("Skipjack bounds forward the way it faces and lands in a ring of light.",
             "Allied heroes inside the ring when it lands, Skipjack too, run and strike faster for 10 seconds."),
    "moonlight_shadow": ("A ring and a rising light round Skipjack; every allied hero, anywhere on the map, fades.",
                         "To its own side a hidden hero is drawn see-through. An attack or a spell shows it again until it waits out the fade."),
}

HEROES = [
    {"id": "tallow", "name": "Tallow", "after": "Vengeful Spirit (Shendelzare)",
     "abilities": ["magic_missile", "wave_of_terror", "command_aura", "nether_swap"],
     "look": "A lantern moth, round and fuzzy, with two pairs of dusk wings marked with an eye spot in its side's colour, feathered antennae, big dark eyes with an amber shine, a banded belly and a ruff in its side's colour. It holds an ember in one hand to throw. No wings of a harpy, no spirit of vengeance.",
     "stats": "Agility 27 + 2.35, strength 16 + 2.3, intelligence 15 + 1.75; 454 health, 195 mana; ranged 400 at 1.7 s base attack time, an ember shot flying 1500 a second; 295 move speed; 1800 sight by day, 800 by night."},
    {"id": "skipjack", "name": "Skipjack", "after": "Priestess of the Moon (Mirana)",
     "abilities": ["starfall", "elune_arrow", "leap", "moonlight_shadow"],
     "look": "A pond frog with its eyes on top of its head, a long smile, a pale belly and long folded legs for the leap. It wears a hood of night cloth sewn with stars, trimmed in its side's colour, carries a reed bow and a quiver of star-tipped arrows. No priestess and no great cat to ride.",
     "stats": "Agility 20 + 2.75, strength 17 + 1.85, intelligence 17 + 1.65; 473 health, 221 mana; ranged 600 at 1.7 s base attack time, a star shot flying 900 a second; 300 move speed; 1800 sight by day, 1200 by night."},
]

VERSION = [
    ["Vengeful Spirit", "No balance change in 6.72–6.73c. Wave of Terror only takes armor since 6.71 (it took attack damage before). No death aura or illusions: those came in 6.81 and later.", "[PD-VS-6.72], [CL-6.71]"],
    ["Wave of Terror", "Takes health, not damage, so magic resistance does not cut it; it can still kill and last-hit. 20 s of armor loss, 15 s cooldown.", "[PD-VS-6.72]"],
    ["Nether Swap", "Heroes only, ally or enemy, through spell immunity; trees fall round both spots; 600/900/1200 range.", "[PD-VS-6.72]"],
    ["Mirana", "No balance change in 6.72–6.73c. 6.73 fixed Elune's Arrow damaging but not stunning invisible units (built as 6.73).", "[PD-POM-6.72], [CL-6.73]"],
    ["Elune's Arrow", "Flat damage (distance damage came in 6.78); a stun of 0.5 s for each 150 flown, up to 5 s at 1500; it hits creeps.", "[PD-POM-6.72], [G-Competitive]"],
    ["Leap", "No charges in 6.72; the buff to allies on landing, not at the cast (6.79).", "[PD-POM-6.72], [D2W-POMCL]"],
    ["Moonlight Shadow", "175 mana, 11 s on every allied hero; attacking or casting shows a hero until the fade passes again; hidden heroes do not attack on their own.", "[PD-POM-6.72]"],
]

UNVERIFIED = [
    "Speeds and delays no DotA source gives: Magic Missile 1250, Wave of Terror 2000, Starfall's 0.57 s, Elune's Arrow 857, Leap 2000 (Dota 2's).",
    "Leap's distances: 630/720/780/870 (older playdota, the Dota 2 wiki) against late-2010 playdota's 630/690/780/840.",
    "Not built: the vision Wave of Terror and Elune's Arrow give, and Leap's instant of invisibility that makes shots already in flight miss.",
]

ARENA_NUMBERS = [
    ["Tickets", "500 a side; at none, the side loses", "SMITE 1 and SMITE 2 (500 again since February 2026)"],
    ["A hero dies", "−5", "both"],
    ["A hero kills a creep", "−1 (a creep's kill costs nothing); none once the side is down to its last 10", "SMITE 1's last-ten rule"],
    ["A creep walks into the portal", "−1", "both"],
    ["A brute walks into the portal", "−15 (after SMITE's Minotaur)", "both"],
    ["Waves", "every 20 s: 3 melee, 3 ranged, 1 siege; by the top lane, then the bottom, turn about", "SMITE 1"],
    ["Brutes", "one for each side every 3 minutes", "SMITE 2 (since July 2025)"],
    ["Camps", "three in each side's jungle; back 3 minutes after cleared; the hero who clears one gets its buff for 2 minutes", "SMITE 2"],
    ["Buffs", "red: +20% damage; blue: +3 mana a second; purple: +15% attack speed", "the agent's, in DotA's terms (R8.9)"],
    ["Pace", "level 3, 1,000 gold, half again the experience and twice the gold a second, half the usual respawn wait", "the agent's (R8.9)"],
    ["Towers", "none; the Sphere is the portal and nothing can hurt it; the fountain still guards the base", "both"],
]

STEPS = [
    ("Mockups", "Both heroes drawn in the round-2 style on the shared chibi body, before any code: Part 2.", "Done"),
    ("Rules", "All eight abilities in the core with their 6.72/6.73 numbers; 10 tests worked out by hand, break rows for each.", "Done"),
    ("On the roster", "The fourteenth and fifteenth heroes: the pick screen, the deal, the computer players and online rooms include them.", "Done"),
    ("Look", "Both models in the game from the mockups; portraits and eight icons; an ember shot and a star shot; the dusk wave, falling stars, the comet arrow.", "Done"),
    ("Heroes screen", "Both cards open the inspector; each ability plays on loop.", "Done"),
    ("The arena", "A second map, swapped in for an arena match; tickets, portals, waves turn about, brutes, buff camps; the HUD shows tickets; 9 tests.", "Done"),
    ("Online", "The host's mode switch goes Normal, All Random All Mid, Arena; every player chooses any hero; online version 9; tested with three browsers on the arena.", "Done"),
]

QUESTIONS = [
    ["R8.1", "The names and looks: <b>Tallow</b>, a lantern moth (Ember Dart, Dusk Wave, Lamplight, Moth Switch), and <b>Skipjack</b>, a star frog (Star Shower, Comet Arrow, Lily Leap, Pond Mist). Keep, or change any?"],
    ["R8.9", "The arena's pace and buffs are mine, in DotA's terms (1,000 gold, level 3, red/blue/purple as damage, mana and attack speed). Too fast, too slow, or other buffs?"],
    ["R8.10", "The arena's size and shape: an oval with the bases 7,000 apart and a pocket of three camps on each side. Want columns or stealth grass in the middle, as SMITE 2 has?"],
    ["R8.11", "The brute borrows the melee creep's model at 1¾ times the size. Want a model of its own?"],
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


def look_pair(h):
    return f'''<figure class="gpair"><div class="gpair-row"><div><span class="chip">mockup</span>{pic(f"models/hero_{h["id"]}.jpg", h["name"] + ": mockup")}</div><div><span class="chip acc">in the game</span>{pic(f"game/{h["id"]}.jpg", h["name"] + ": in the game")}</div></div>
  <figcaption><b>{e(h["name"])}</b> (abilities of {e(h["after"])}). {e(h["look"])}</figcaption></figure>'''


looks = "".join(look_pair(h) for h in HEROES) + f'''
  <figure class="gpair"><div>{pic("game/lineup.jpg", "Tallow and Skipjack beside Shiver, Thistle and Rime, in the game")}</div><figcaption><b>Beside the roster.</b> Tallow and Skipjack with Shiver, Thistle (the other archer) and Rime, in the game's model viewer.</figcaption></figure>'''

PANEL = f'''<!-- ======================================================= TAB 14 -->
<div class="panel-tab" role="tabpanel" id="p-round8" aria-labelledby="t-round8" hidden>
<header style="display:grid;gap:12px">
  <div class="eyebrow">Entry 14 · 24 October 2026 · two added heroes and the arena (triage T38)</div>
  <h2>Tallow, Skipjack and the arena</h2>
  <p class="lede">Two heroes that play as DotA's Vengeful Spirit and Mirana, with names and looks of their own, and a new mode after SMITE 2's Arena: a small round map, tickets instead of towers, a small jungle on each side, any hero. Part 1 is the literature review. Part 2 is the mockups beside the game. Part 3 is every ability, cast in the game. Part 4 is the arena. Part 5 is what is built and four questions for you. Both heroes are on the roster and the arena is in online play and the custom match. Click any picture to see it larger.</p>
</header>

<section>
  <div class="section-head"><div class="eyebrow">The request</div><h2>In your words</h2></div>
  <blockquote>{e(REQUEST)}</blockquote>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 1</div><h2>How Shendelzare and Mirana played in DotA</h2></div>
  <p>From playdota's hero pages read through the Wayback Machine (2010–2012), IceFrog's changelogs from 6.71 to 6.73c, the Dota 2 Wiki's DotA-era pages and seven 2009–2011 playdota guides; every number with its source in <code>notes/LITERATURE.md</code>, section 9. Dota 2 changed both: Vengeful Spirit gained a death aura and illusions, Mirana's Leap charges and her arrow damage by distance.</p>
  <div class="table-scroll"><table>
    <thead><tr><th>What</th><th>As of 6.72/6.73</th><th>Sources</th></tr></thead>
    <tbody>{table([[f"<b>{e(a)}</b>", e(b), f"<code>{e(c)}</code>"] for a, b, c in VERSION])}</tbody>
  </table></div>
  <h3 class="sub">Not settled by any source, and what the game uses</h3>
  <ul>{"".join(f"<li>{e(u)}</li>" for u in UNVERIFIED)}</ul>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 2</div><h2>The mockups, then the game</h2></div>
  <p>Drawn first, in the round-2 style the other thirteen share: round, faceted, an outline that reads from the raised camera. Then built into the game from the same shapes. Neither borrows from Blizzard or Valve: a moth instead of a winged spirit, a frog instead of a priestess on a cat.</p>
  {looks}
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 3</div><h2>Every ability, cast in the game</h2></div>
  <p>Each animation is the game itself: its cast stage puts the hero at full levels beside an ally, three enemy creeps and an enemy hero, and the frames are the following ticks (<code>tools/castgif.py</code>). The rule under each is the game's own tip.</p>
  {"".join(hero_block(h) for h in HEROES)}
  <div class="table-scroll"><table>
    <thead><tr><th>Shot</th><th>How it looks</th></tr></thead>
    <tbody>{table([
        ["<b>Tallow's attack</b>", "A small glowing ember between two fluttering dusk wings, 1500 a second, trailing sparks."],
        ["<b>Skipjack's attack</b>", "A four-pointed star turning as it flies, 900 a second, in a pale halo with a long tail."],
        ["<b>Ember Dart</b>", "A big ember with wings, the same family as Tallow's attack."],
        ["<b>Comet Arrow</b>", "A big star, the same family as Skipjack's attack, with a longer tail."],
        ["<b>Dusk Wave</b>", "Five dusk motes across its way, rising and falling out of step."],
    ])}</tbody>
  </table></div>
</section>

<section>
  <div class="section-head"><div class="eyebrow">Part 4</div><h2>The arena</h2></div>
  <p>After SMITE 2's Arena, which copied SMITE 1's (<code>notes/LITERATURE.md</code>, section 10, from the SMITE wikis, every SMITE 2 patch note and a 2018 guide). A small oval on its own map with a base at each end and no towers. Each side's tickets fall when its heroes die, when heroes kill its creeps and when enemy creeps walk into its portal; the side that runs out loses. Matches between computer players run 16 to 20 minutes, about SMITE's own.</p>
  <figure class="gpair"><div class="gpair-row"><div><span class="chip">from above</span>{pic("arena/overview.jpg", "The arena from above")}</div><div><span class="chip acc">in a match</span>{pic("arena/fight.jpg", "A fight in the arena")}</div></div>
  <figcaption><b>The map.</b> The Sunward base at the south-west, the Moonward at the north-east, each with its fountain, shop and Sphere (the portal). Two lanes curve round the open middle; waves take one, then the other. On each side, a pocket of jungle holds three camps: red, purple and blue. Everything else is forest.</figcaption></figure>
  <figure class="gpair"><div class="gpair-row"><div><span class="chip">lobby</span>{pic("arena/lobby.jpg", "The online lobby with the arena chosen")}</div><div><span class="chip acc">tickets</span>{pic("arena/hud.jpg", "The top bar showing each side's tickets")}</div></div>
  <figcaption><b>Online.</b> The host's mode switch goes Normal, All Random All Mid, Arena; everyone chooses any hero. In the match the top bar shows each side's tickets where the kills would stand.</figcaption></figure>
  <div class="table-scroll"><table>
    <thead><tr><th>Rule</th><th>In the game</th><th>From</th></tr></thead>
    <tbody>{table([[f"<b>{e(a)}</b>", e(b), e(c)] for a, b, c in ARENA_NUMBERS])}</tbody>
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
<footer>Mockups: <code>journal/mockups/heroes4.js</code> (three.js r160, MIT), pictures from <code>ART=art8 LOOK=ape shoot.py</code>. In-game pictures: <code>tools/shots.py</code> and <code>tools/castgif.py</code> on the build of 24 October 2026. The arena's map: <code>tools/arena_map.py</code>. Icons: <code>tools/art/</code> (TikZ). Tips: <code>cargo run -p pots_core --example tips</code>. This tab is written by <code>journal/mockups/tab14.py</code>.</footer>
</div>
'''

if missing:
    sys.exit("missing:\n  " + "\n  ".join(missing))

s = J.read_text()
s = re.sub(r'\n    <button role="tab" id="t-round8".*?</button>', "", s)
s = s.replace("    <!--TABS-->", '    <button role="tab" id="t-round8" aria-controls="p-round8" aria-selected="false" tabindex="-1"><small>14 · 24 Oct</small><span>Tallow, Skipjack, the arena</span></button>\n    <!--TABS-->', 1)
s = re.sub(r"<!-- =+ TAB 14 -->.*?<!--/TAB 14-->\n", "", s, flags=re.S)
s = s.replace("<!--PANELS-->", PANEL + "<!--/TAB 14-->\n<!--PANELS-->", 1)
s = re.sub(r"last entry \d+ October 2026", "last entry 24 October 2026", s, count=1)
for need_ in ('id="t-round8"', 'id="p-round8"', "last entry 24 October 2026"):
    assert s.count(need_) == 1, need_
J.write_text(s)
print("tab 14 written")
