# Handoff

Stage 9, `C5 Consolidate in your own words`. A fresh session starts here, not from memory.

## Where things stand (16 October 2026, round 1)

**Protection of the Spheres** is built and published: a browser game that plays like DotA Allstars 6.72/6.73 with ten original heroes, an arcade mission ladder against computer players, and 5v5 online play with rollback netcode and rejoining. Live at <https://sgilson7.github.io/pots/>; repository <https://github.com/sgilson7/pots>. The design journal (a private claude.ai artifact) is <https://claude.ai/artifact/54ByhyyFPtmixHLiECsRXq>, one tab per round.

What exists, by part:

- **The match** (`crates/core`): map generated from a seed, A* pathing around trees, DotA's combat (attack points and backswing, dice damage, the 6.72 damage table, armor, magic resistance, pseudo-random procs), ten heroes with forty abilities, 92 items with recipes and actives, creep waves with upgrades and mega creeps, towers in order with backdoor protection and the glyph, neutral camps refilling on the minute, runes, the Warden and its shield, gold, experience, deaths, respawn, buyback, vision with fog, day and night, a courier per side.
- **Computer players** (`ai.rs`, `data/ai.json`): behavior trees after Vagrancy's pilots; five difficulty levels set apart from the trees; ally and enemy skill chosen separately in the arcade.
- **Arcade**: ten missions and a custom match (0–4 allies, 1–5 enemies, two skills).
- **Online** (`online.rs`, `web/online.js`, `web/net/`): rooms by code through public Nostr relays, a host-centred star of up to ten browsers, rollback over orders, the computer taking a dropped seat, rejoining by the room link with a seat token.
- **3D**: every model in `data/models.json` (from `tools/modelgen.py`), drawn by the WebGL renderer after Gatecrasher's; icons for every item, ability and hero drawn in TikZ with no text, all with hover tips.

## Round 2 (16 October 2026): the new look, mockups only (T1)

The person asked for a cuter, rounder style after Chumbles (Long Time Comin') and Mage Tower Battle's mockups, a mockup and picture of every model, and an animation with a picture, GIF and description for every ability, in a new journal tab. Done as mockups; **the game itself is unchanged**.

- Analysis: `notes/ART-STYLE.md` (what is blocky now, measured; the eight rules of the new look; what the renderer and core must gain to draw it).
- Mockups: `journal/mockups/` (three.js r160 vendored with SHA256SUMS; `kit.js` style kit, `heroes.js`, `world.js`, `fx_a.js`, `fx_b.js`, `scenes.js`). `shoot.py` makes every picture into `journal/art2/`; `tab8.py` writes journal tab 8 from those pictures, the mockups' NOTES, and the core's own ability tips (`cargo run -q -p pots_core --example tips > journal/art2/tips.json`; `ui::ability_tip` is now public for it).
- 50 model stills, 2 hero lineups, 2 lane scenes, 41 ability stills and GIFs. Weakest, by the agents' own reading: Thousand Cuts' still, Keen Edge's pale star, Thornburst and Rift Stomp (rocks hide the line), Steady Hand and Clear Mind (subtle), Hoarfrost Burst (whitish), Gutting Hook (the dragged unit jumps back at the loop seam), the drakes' wings.
- Next round, if the person approves: port to `tools/modelgen.py`, new meshes (capsule, torus arcs, finer sphere), soft lighting, effect records from the core and a status layer (`notes/ART-STYLE.md`, "Porting").

Checked: `./scripts/check.sh` passed after these changes (**BUILDER CHECK PASSED**, browser check in Chromium, Firefox and WebKit); `./scripts/count-tests.sh` 96 passed, 0 failed, 4 ignored. The journal tab was looked at in Chromium at 1280 and 400 px wide, with no sideways scroll.

Open for the person from round 2: **R2.5** (bring the game camera down from 56° so faces show?), **R2.6** (does the style suit; which models or animations to redo, by id), **R2.7** (did "voxel" mean this rounded look or small cubes?).

## What passed

Recorded at the end of this round; the counts come from the commands, not from memory.

- `./scripts/check.sh`: **BUILDER CHECK PASSED** (format, clippy, every core and boundary test, release build, import audit, art checks, notebook form, browser check in Chromium, Firefox and WebKit), before the publish of `67ed47e`.
- Live: `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py`: **BROWSER CHECK PASSED in chromium, firefox, webkit**, on build `67ed47e`.
- `./scripts/count-tests.sh`: 96 passed, 0 failed, 4 ignored (the ignored are diagnostics in `diag_bots.rs`).
- `python3 scripts/break_tests.py`: 14 of 14 breaks caught.
- Online: in-process tests (five browsers over a lagged, lossy wire; a fast wire with a drop keeps to the clock; a drop and rejoin; a silent browser; a return before the drop is noticed) pass. On the live site, two browsers met through the relays and a dropped one rejoined, three times in three. The five-runner test's second run: five browsers, ten minutes, 30 ticks a second, a drop and a rejoin, no checksum disagreement over 200 ticks (`notes/ONLINE-RUN.md`).

## Open for the person

Each row of `SECOND-ORDER.md` with status *person*, by number:

- **R1.2**: the names and looks of heroes, items and creeps. Are they far enough from their DotA sources?
- **R1.10**: one free courier per side from the start (DotA sold it). Keep?
- **R1.12**: lane creep armor type, Unarmored (playdota) or Medium (Dota 2 port). It changes ranged-creep damage 2×.
- **R1.16**: if the host's browser leaves, the online match ends. A host hand-over is not built.
- **R1.18**: the 143 icons; a few are plain at 48 px.
- **R1.21**: online depends on public relays and STUN, with no TURN server; two strict NATs cannot connect. Try two real networks by hand.

## Try by hand first

1. Arcade, mission 1 ("First steps"): walk to the middle lane, wait for creeps, and land last hits as their health drops. Count: the goal line says how many you have.
2. A custom match, 4 allies and 5 enemies on Normal: buy boots at the base shop (B), learn abilities (the + under each), and play to a tower.
3. Online with a second computer on another network: host, send the link, both pick, start. Close one browser mid-match and reopen its link.

## Mistakes so far, by system

- **Online:** the browsers chased each other's one-tick lead and ran at twice the clock (R1.24); silence was counted in ticks that had stopped (R1.25); a returning player's token was ignored before the drop was noticed (R1.29); the test reopened a stale address (R1.30). Each now has a test.
- **Rules:** a unit at no health regenerated before the death check (R1.15); the damage table was Warcraft III's until the review (R1.4).
- **Bots:** they barely last-hit (R1.27) until creeps preferred creeps and the farm routine was measured.
- **Checks:** a verdict that compared nothing (R1.23); two pairs of mechanisms that cover each other, so a single break was not seen (R1.24, R1.26).

## What the next session must not assume

- That two players on different networks can connect: every online test so far ran on one machine.
- That the bots play well: they last-hit, deny, use abilities and push, but a match between them lasts 40–60 minutes.
- That the art is final: the models are simple shapes; the icons are first drafts.
