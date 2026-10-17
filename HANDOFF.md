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

The person answered "looks amazing" (R2.5 to R2.7 closed).

## Round 2, continued (16 October 2026): the look in the game, build trees, the locked cursor (T2, T3, T4)

- **The look in the game (T3).** `data/models.json` is now written by `tools/modelport.py` from the mockups (`journal/mockups/shoot.py export` writes `tools/art2_models.json` first); heads tip back 12° for the 56° camera. New `Taper` mesh, smooth 20-sided cylinders and cones, 12 × 20 spheres. Soft light in `data/shaders/scene.frag` (wrap, a gentle ramp, a sky fill and a ground bounce; numbers in `data/presentation.json` `bands`, `ambient`, `ground`). The map's colours are the mockups' pastel ones.
- **Ability effects (T3).** The rules leave `World::marks` (cast, fire, land, proc), kept four seconds and left out of the checksum. `crates/core/src/marks.rs` draws them, the missiles, the zones and every unit's status (stun stars, silence bubble, slow frost, ice for a root, buff rings, damage-over-time puffs, spinning blades, channel tethers), each ability's look in `data/presentation.json` `fx`. The `cast:<ability>,<ticks>` stage (`Game::cast_world`) shows any ability; `tests/marks.rs` casts all 41.
- **Build trees (T2).** A click in the shop picks the item and shows its tree (`item::tree`, `ui::build_panel`); the tree's Buy button buys. Picking is the page's state, never an order.
- **The locked cursor (T4).** In a match the page draws its own cursor; a click on the battlefield locks the pointer to the window and the core moves the drawn cursor (`Game::nudge`); every edge pans the camera, including the bottom and left under the HUD and the minimap (R2.15).
- **The page's DOM sync** now keeps a node whose own part did not change (R2.17).

**Checked and live (16 October 2026).** `./scripts/check.sh`: BUILDER CHECK PASSED (110 core tests passed, 0 failed, 4 ignored; browser check in Chromium, Firefox and WebKit); `scripts/break_tests.py` caught 22 of 22 before the last five rows were added. Five-runner test: run three kept every checksum in step but its dropped browser could not get back through the relays (R2.18); run four passed, rejoin in 4.5 s, 199 checksum ticks and 371 live sums with no disagreement, 30 ticks a second (`notes/ONLINE-RUN.md`). `./scripts/publish.sh` published build `bbf8133`; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in all three engines on the live site, and a live `smoke.py --rejoin` rejoined. The scholar site's page and tile were updated (commit `f42a76a` in `~/Documents/website`, unpushed under its rules), `make check`, `make packet`, `make gate`, `make deploy` (build bb39ce2d) and `make gate-live` passed. Journal artifact version 5.

Still open from round 2: R2.18 (rejoining failed in two of four five-runner runs, in the relay and WebRTC layer), R2.19 (a rejoined browser comes back three ticks behind and causes small rollbacks).

Open for the person from round 2: **R2.16** (try the locked cursor by hand: automated browsers cannot lock the pointer).

## Copy buttons (16 October 2026, T5)

The lobby's Copy code and Copy link had never worked: the page's copy function was not called (R2.20). The page now copies inside the click, the core shows "Copied.", and `tests/actions.rs` presses every button on every screen and fails on an action the core does not know. Check: BUILDER CHECK PASSED; break tests 24 of 24.

## Round 3 (16 October 2026): shots, attacks, the hand's marks, solid units (T6–T12)

- **Shots (T6):** `data/presentation.json` `shots` gives each kind of shooter a shape, colour and size (tower bolt, creep orb, siege boulder, an arrow, bullet streak, frost shard, fireball, hex orb and spark for the ranged heroes); spells can name a `missile` (Storm Bolt's hammer, Meat Hook's hook, Dragon Slave's fireball, Assassinate's streak). Drawn in `marks::draw_missile`, moving between ticks. The `volley:<ticks>` stage shows them all (`tools/shots.py`).
- **Attacks (T7):** `models::strike_angle` and `lean`, timed by the rules' attack point and backswing at the unit's attack speed (`scene::pose_of`).
- **Shop (T8):** a node's `alt` is its right-click action; shop items and tree parts carry `buy`.
- **Aim, order marker, teleport (T9–T11):** `crates/core/src/overlay.rs`. `Game::overlay()` builds the aim outline and the order marker each frame; neither reaches the world. `overlay::teleports` draws channels from the world, using `item::tp_destination`, the rule's own landing spot.
- **Solid units (T12):** `orders::clear_step` and `crowded`: a step into another unit turns aside or waits; creeps do not push heroes. `PROTOCOL` is now 2, so an older browser is told to reload rather than play on different rules.
- Person rows: **R3.4** (should abilities that go off at once show their radius while the key is held?) and **R3.10** (try creep blocking by hand).

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
- **R3.4**: abilities that go off at once (Warcry, Echo Slam, Blade Fury) show no outline. Show their radius while the key is held, or on hovering the button?
- **R3.10**: creep blocking: walk just ahead of your first wave and zigzag in its path. Does it feel like DotA's?
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
