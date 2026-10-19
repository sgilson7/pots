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
- Checks: BUILDER CHECK PASSED (Chromium, Firefox, WebKit, with a right-click buy in each); break tests 34 of 34; the online smoke test rejoined with checksums agreeing; 60 frames a second in a 5v5.
- Person rows: **R3.4** (should abilities that go off at once show their radius while the key is held?) and **R3.10** (try creep blocking by hand).

## Round 4 (17 October 2026): All Random All Mid and the base cannon (T13, T14)

Planned with mockups (journal tab 9); the person answered R4.1 to R4.6 (middle lane of the usual map, one reroll, the cannon and relic numbers as planned, 853 gold, no teleports); then built.

- **Rules:** `crates/core/src/aram.rs` with the numbers in `data/rules.json` `aram`. `World::new_mode` leaves the side lanes unbuilt; `lanes.rs` skips their waves, the camps, the boss and runes; the base towers count built lanes only. `aram::shop_of` sells the far shops' goods at the base and no teleports.
- **The deal:** `aram::deal(seed, heroes, rerolled)`, one shuffle a side (R4.9). Arcade: `setup.mode`, `custom`, `reroll`; the seed changes each match (`Game::arcade_seed`). Online: the host deals in `deal_seats` from the match seed; `online.mode`, `online.reroll`, `Msg::Reroll`; the mode rides in `Lobby` and `Start`; `PROTOCOL` 3.
- **The cannon:** `Cmd::Cannon(point)`, `Act::Flight`, `aram::throw` and `fly`; F or the HUD button aims it (`Aim::Cannon`); the minimap shows the band; the camera follows the throw. Drawn in `overlay::aram` and `scene::units`; models `cannon_sun`, `cannon_moon`, `relic` ported from the mockups.
- **Relics:** `aram::clock`; drawn with a ring that fills while one is away.
- **Bots:** `ai::aram`: mid only, the cannon when healed at the base, a relic below half health.

## Round 5 (17 October 2026): stale data after a deploy, the bots in All Random All Mid, computer players off (T15–T17)

- **Stale data (T15, R5.1):** the page's data files were fetched without the build stamp, so a browser could mix the new module with old data after a deploy. `scripts/build.sh` now stamps them and fails if it cannot. The person's WebKit error was not reproduced; R5.5 asks for its text.
- **Bots in the lane (T16, R5.2):** in All Random All Mid, computer players with no creeps of their own in the lane wait where the cannon lands, not on their outer tower.
- **Computer players off (T17, R5.3):** a switch in the host's lobby, both modes; open seats stay empty. `PROTOCOL` 4, so every browser must reload.
- The page probe now reports where the computer players stand (`bots`).

## Round 6 (17 October 2026): a style study after Mage Tower's newer look (T18)

Mockups only; **the game is unchanged**. Journal tab 10 (`journal/mockups/tab10.py`) traces Mage Tower Battle's look from 1.5.0 (the one this game copied) to 1.7.1, and shows all 53 models and two lanes as the game draws them now and as mockups in the new look. The mockup kit draws the new look with `?look=ape` (`journal/mockups/kit.js`: facets, no outline, sky and ground light with a low sun, haze, disc shadows, ground dressed at three sizes); `ART=art4 LOOK=ape shoot.py` makes its pictures into `journal/art4/`. The game's pictures are `tools/shots.py` into `journal/art4/game/`.

## Round 6, continued (17 October 2026): the map in the newer look (T19)

The person chose tab 10's look for every map asset and the current look for characters. `data/presentation.json` `map_look` lists the map's models; `Models::facet` swaps their parts to the new faceted meshes (`Mesh::Facet`, `FacetCyl`, `FacetCone`, `FacetTaper`: 80 faces, 8 sides, flat normals) and drops their outline, glowing parts excepted. `scene::shadow` puts two see-through discs under each; `scene::dress` dresses open ground. Light, haze and camera are unchanged (R6.9). Tests: `tests/map_look.rs`, `mesh.rs`'s facet test.

## Round 7 (18 October 2026): selling, the lane's ground, the All Random All Mid pace (T20–T22)

The session before this one stopped after T19 was published (build `82d435b` live); this round started from this file.

- **Selling (T20, R7.1):** the rule was there from round 1 but no control reached it. Near a shop a right-click on a carried item sells it for half its whole price; the tip says so and the shop's empty panel says how. `item::sell_value` decides both. Its border lights on hover (`.slot.sellable`).
- **The lane's ground (T21):** `scene::ground` now shades the lane's dirt from darker to lighter across patches of four cells, blended cell to cell (`dirt_at`); a cell at the lane's edge blends into the grass and the grass beside it into the lane (`edge_blend`); grass creeps onto the edge and flagstones lie down the middle (`road`). Numbers in `data/presentation.json` `map_look.lane`, colours `lane_dark`, `lane_light`, `lane_stone` in `data/models.json`.
- **All Random All Mid pace (T22, R7.3):** heroes start at level 3 with three points and full health, earn one and a half times the experience and twice the gold each second (`aram::start_level`, `aram::xp`, `aram::periodic_gold`; numbers in `data/rules.json` `aram`). `PROTOCOL` 5.
- Break tests: 9 of 9 round-7 breaks caught, after two misses fixed the test (R7.4).
- **Look study (T23):** journal tab 11 (`journal/mockups/tab11.py`, pictures in `journal/art5/`) compares the game with Dota 2, League and stylised games and recommends a faceted ground. The "after" pictures come from the game's own renderer on the local branch `mockup/smooth-ground` (`map_look.smooth` in `data/presentation.json`; `scene::ground` stretches the `Tri` mesh onto each facet; the page takes `?seed=` there only). The branch is not pushed or published. Its known gap: the high-ground edge blends away (R7.10). Journal artifact version 9.
- **Checked and live:** `./scripts/check.sh` BUILDER CHECK PASSED before the publish of `d2d1e4d` (Chromium, Firefox, WebKit); `./scripts/count-tests.sh` 144 passed, 0 failed, 4 ignored; the online smoke test (`tests/online/smoke.py --rejoin`) met through the relays, rejoined, and the checksums agreed; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in all three engines on build `d2d1e4d`. The scholar site was not changed: its tile and page did not need it.


Recorded at the end of this round; the counts come from the commands, not from memory.

- `./scripts/check.sh`: **BUILDER CHECK PASSED** (format, clippy, every core and boundary test, release build, import audit, art checks, notebook form, browser check in Chromium, Firefox and WebKit), before the publish of `67ed47e`.
- Live: `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py`: **BROWSER CHECK PASSED in chromium, firefox, webkit**, on build `67ed47e`.
- `./scripts/count-tests.sh`: 96 passed, 0 failed, 4 ignored (the ignored are diagnostics in `diag_bots.rs`).
- `python3 scripts/break_tests.py`: 14 of 14 breaks caught.
- Online: in-process tests (five browsers over a lagged, lossy wire; a fast wire with a drop keeps to the clock; a drop and rejoin; a silent browser; a return before the drop is noticed) pass. On the live site, two browsers met through the relays and a dropped one rejoined, three times in three. The five-runner test's second run: five browsers, ten minutes, 30 ticks a second, a drop and a rejoin, no checksum disagreement over 200 ticks (`notes/ONLINE-RUN.md`).

## Open for the person

Each row of `SECOND-ORDER.md` with status *person*, by number:

- **R7.2**: selling is instant on a right-click and pays half. Ask first, or give a full refund for a few seconds after buying?
- **R7.8**: build the faceted ground (journal tab 11)?
- **R7.9**: after the ground, which next: the forest's outline, shadows under heroes and creeps, or a darker ground?
- **R7.3**: play All Random All Mid at the new pace (level 3, 1.5× experience, 2× gold per second) and say whether it is too fast or too slow.
- **R1.2**: the names and looks of heroes, items and creeps. Are they far enough from their DotA sources?
- **R1.10**: one free courier per side from the start (DotA sold it). Keep?
- **R1.12**: lane creep armor type, Unarmored (playdota) or Medium (Dota 2 port). It changes ranged-creep damage 2×.
- **R1.16**: if the host's browser leaves, the online match ends. A host hand-over is not built.
- **R1.18**: the 143 icons; a few are plain at 48 px.
- **R6.2**: the new look's time of day: late afternoon as drawn, dusk, or a mood that turns with the day.
- **R6.8**: which models count as map assets (faceted) and which as characters: the list `map_look.faceted` in `data/presentation.json`. Wards, runes, the relic and Fissure's rock went with the map.
- **R5.5**: the WebKit error: its exact words, the browser (Safari on a Mac or an iPhone) and the screen it came on.
- **R4.18**: play a custom match in All Random All Mid: reroll once, press F at the fountain and click the lane, take a relic when hurt. Does it feel right?
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
