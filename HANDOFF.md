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

## Round 7, continued (18 October 2026): the faceted ground and every model in tab 10's look (T24, T25)

- **The ground (R7.18):** one faceted surface. `scene::surface` computes corner heights every half cell once, in `map::generate` (`Terrain::surface`); `scene::ground_at` interpolates them, so units, trees, runes and marks stand on the drawn ground. `scene::ground` stretches the `Tri` mesh onto each facet, blends colours from the cells round each corner, turns steep faces to rock, and lays one water sheet over the sunken riverbed. Drawing only: no rule reads the surface.
- **Cliffs (R7.13):** a corner where high ground meets low ground (neither a ramp) stays high, so a plateau drops within half a cell; `scene::cliff_rocks` lines each cliff's foot with faceted boulders. Numbers in `data/presentation.json` `map_look.ground`, colours `cliff_rock`, `cliff_rock_dark`.
- **Every model in tab 10's look (R7.12, R7.15):** `Models::facet_all_but` facets every model but those in `map_look.keep_smooth` (empty); glows stay smooth; missiles and effect solids are faceted and plain (`marks::SOLID`). Nothing is outlined; the outline pass is still in the renderer, empty. Every unit stands on a soft shadow. The light is tab 10's late afternoon (`light_dir`, `light`, `ambient`, `ground`, `fog`, `fog_range`, `sky`).
- **Smaller creeps (T25, R7.16):** lane creeps drawn at 85% (`creep_scale_pct`, `scene::drawn_scale`); collision unchanged.
- **Journal:** tab 11 Part 5 shows the built look before and after (`journal/art5/b_*`); the "before" pictures came from build `e98191f` in a throwaway worktree. The branch `mockup/smooth-ground` is superseded and can be deleted.
- **Checked:** every core test passes, including `tests/marks.rs` (all 41 abilities cast), `tests/ground.rs` and `tests/map_look.rs`; `scripts/break_tests.py "round 7b"` caught 9 of 9 after two test fixes (R7.14). Frame rate 60 a second at 5 s and 90 s in a 5v5 (`MEASUREMENTS.md`). Five-runner test (`runs/fifth`, 6 minutes, read live by a separate agent, `runs/fifth/ANALYSIS.md`): all five in the match at the end, 30 ticks a second, 119 checksum ticks compared over the whole log with no disagreement, no rollbacks, the dropped browser back in 2.2 s. One runner logged a 404, also seen in round 2's fourth run; the page sets no favicon, which is the likely request.


Each row of `SECOND-ORDER.md` with status *person*, by number:

## Round 7, continued (18 October 2026): a match behind the title (T26)

- `crates/core/src/backdrop.rs`: ten computer players play behind the title screen; the camera follows the hero with the most enemies near, looks again every 8 s, cuts beyond 2,200 units and stands 700 west so the fight shows right of the menu. Its own world; `Game::with_frame` draws it only on the title, once it has caught up to 92 s. The title's gradient (`.menu.title-screen`) fades to clear on the right. Reduced motion (`act("motion", "reduce")`, from the browser) makes it a still (R7.21).
- Tests: `tests/backdrop.rs` (four), break rows "round 7c" (six, all caught).

## Round 7, continued (18 October 2026): names in the lobby, typing, gold on death, selling (T27–T30)

- **Typing (T28, R7.22):** text boxes are updated in place by the page's sync; the one being typed in is never written over. `tests/browser/interactions.py` `typing` types a name and a room code (it failed with the fix taken out).
- **Names in the lobby (T27, R7.23):** a name box in the lobby; `Msg::Name` from a joiner; `PROTOCOL` 6.
- **Gold on death (T29, R7.24):** `death_gold_per_level` 0.
- **Selling (T30, R7.26):** with the shop open, sellable items show their price.
- **The title backdrop on slow machines (R7.25):** five slow frames in a row and it stops.

## Round 7, continued (19 October 2026): a Sell button (T31)

- With the shop open, a click on a carried item picks it (`Game::shop_slot`, `act("shop.pick_slot")`) and `ui::sell_panel` shows it with a Sell button (R7.27). Published as `4626ab8`; the live browser check passed.

## Round 7, continued (19 October 2026): two heroes for review (T32)

- **Review:** `notes/LITERATURE.md` section 7, Troll Warlord and Goblin Shredder as of DotA 6.73 (Shredder was added in 6.73, inside the pin).
- **Rules:** ten abilities in `crates/core/src/ability.rs` (`berserker_rage`, `whirling_axes` and `whirling_axes_melee`, `fervor`, `battle_trance`; `whirling_death`, `timber_chain`, `reactive_armor`, `chakram` and `return_chakram`), with `alt` slots (R7.33), `Tag` codes `TAG_BERSERK`, `TAG_BLIND`, `TAG_BLADE_OUT`, `TAG_ATTR_PCT`, a moving `Zone` (`to`, `done`), `stats::missile` for the melee form, `ability::attacked` when an attack begins, and attribute cuts that keep the health share when they end. Tests: `tests/heroes2.rs` (13); break rows "round 7g" (18, all caught after one test fix).
- **Preview heroes (R7.34):** `hackle`, `chipper` in `data/rules.json` `preview_heroes`; `Rules::roster` is the choosable count; the host refuses a pick past it.
- **Look:** mockups `journal/mockups/heroes2.js` ported to `data/models.json` (only the two added); portraits and icons in `tools/art/`; effects and shots in `data/presentation.json` and `crates/core/src/marks.rs` (`axe`, `claw` shots; `axes`, `chakram`, `pull` zones; miss and attribute-cut marks). The cast stage takes `,close` and handles alt abilities; `tools/castgif.py` makes GIFs from it.
- **Journal:** tab 12 (`journal/mockups/tab12.py`, `journal/art6/`).
- **Not published:** nothing a player sees changed. Before these heroes go live, `PROTOCOL` must rise (the world's shape changed).

## Round 7, continued (19 October 2026): the two heroes live (T33)

- The person said "deploy the new characters". `hackle` and `chipper` moved from `preview_heroes` to `heroes` (the roster is twelve); `PROTOCOL` is 7 in `crates/core/src/online.rs` and `web/net/config.js`. Names, looks and the 20/30/40 drain shipped as proposed (R7.28, R7.29).
- Computer players: builds in `data/ai.json`; `ai::switch_form` turns Berserker Rage and calls the Chakram back; `useful_cast` throws Timber Chain only with a tree past the foe (R7.37). Tests: three more in `tests/heroes2.rs`; break rows "round 7h".
- Checks: see the round's checks below.

## Round 7, continued (19 October 2026): the heroes screen's inspector, and creeps that end a match (T34, T35)

- **Heroes screen (T34):** its cards did nothing (R7.41). Each now opens the inspector (`crates/core/src/showcase.rs`, `ui::heroes`): the hero drawn in the game's renderer, a drag turns the camera, the wheel zooms; buttons for the attack and each ability (and the second forms) play a demonstration on loop beside its description, mana, cooldown and range (R7.42, R7.43). The page sends a button's release (`game.release`). Tests: `tests/showcase.rs` (6); break rows "heroes screen" (11, all caught after one row was repointed, R7.45).
- **Creeps (T35, a subagent's work, merged):** a creep on its lane's last leg with nothing in reach goes for the nearest enemy building it may hurt, so middle-lane creeps alone take both base towers and the Sphere (`crates/core/src/lanes.rs` `siege_target`, R7.50, R7.51). Creep blocking measured and working (R7.52, R7.53). A fallen building still blocks the ground (R7.54, open).
- **Break list:** four rows named renamed tests and one had rotted (R7.40); `scripts/break_tests.py` now reports a row whose test runs nothing. 131 of 131 caught.
- **Checks and live:** `./scripts/count-tests.sh` 184 passed, 0 failed, 4 ignored; `scripts/break_tests.py` 131 of 131 caught; the online smoke test (`tests/online/smoke.py --rejoin`, version 7) met through the relays, rejoined, and the checksums agreed at tick 1800; `./scripts/publish.sh` ran `./scripts/check.sh` (**BUILDER CHECK PASSED**, Chromium, Firefox, WebKit) and published build `e40edd8`; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in all three engines on the live site.

## Round 7, continued (20 October 2026): Shiver, after Ancient Apparition (T36)

- **Research:** a research agent read playdota, the changelogs and 2010–2011 guides; `notes/LITERATURE.md` section 8. 6.72/6.73 differs from Dota 2 in Cold Feet (740, four hits), Chilling Touch (an allied buff, heroes only, −15% attack speed) and Ice Blast (225 + 50 a second, the Fountain clears frostbite). Built as 6.72/6.73 (R7.56); the unsourced parts are the agent's (R7.57, R7.58, R7.63).
- **The hero:** `shiver` in `data/units.json`, `abilities.json` (cold_feet, ice_vortex, chilling_touch, ice_blast, ice_blast_release as Hailstar's alt while the tracer flies), `rules.json` (the thirteenth on the roster), `strings.json`, `ai.json`. Rules in `crates/core/src/ability.rs`; frostbite (`ability::no_heal`) stops regeneration, heals, lifesteal, the healing ward and the relic. Online version 8.
- **Look:** the mockup `journal/mockups/heroes3.js` → `tools/art2_models.json` → `data/models.json` (`shoot.py export`, `tools/modelport.py`); the model floats (`hover`, R7.62). Portrait and five icons in `tools/art/`. A snowflake shot (`flake`), zones `frozen`, `vortex`, `hail`, and marks for Frost Talons' charges and frostbite in `crates/core/src/marks.rs`.
- **Computer players and the heroes screen:** `ai::switch_form` releases Hailstar over an enemy hero; it casts only at heroes within 900 (R7.59, open). The cast stage releases Hailstar by itself, so the heroes screen and the journal's animations show the burst.
- **Journal tab 13** (`journal/mockups/tab13.py`): the review, the mockup beside the game, every ability cast in the game, the shots, three questions.
- **Tests:** `tests/shiver.rs` (11); break rows "shiver:" (21, all caught once the edge test checked both sides). Hackle and Chipper's tests no longer assume they are the roster's last two (R7.64).
- **Deployed** on 21 October at the person's request (below).
- **Checks (20 October 2026):** `./scripts/count-tests.sh` 195 passed, 0 failed, 4 ignored; `./scripts/check.sh` **BUILDER CHECK PASSED** (Chromium, Firefox, WebKit); `scripts/break_tests.py shiver` 21 of 21 caught after `cargo fmt` (one row repointed at the reflowed text). The other 131 rows were not re-run this time; `cargo fmt` changed only the new code. **Live:** `./scripts/publish.sh` ran `./scripts/check.sh` again (**BUILDER CHECK PASSED**) and published build `9cd7ec0`; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in Chromium, Firefox and WebKit on the live site; the online smoke test on the live site (`BASE=… tests/online/smoke.py --rejoin`, version 8) met through the relays, rejoined, and the checksums agreed at tick 1800 (one relay, relay.damus.io, answered 503; the others carried it).

## Round 7, continued (21 October 2026): Safari froze after a click or two (T37)

- **Cause:** under the pointer lock, which brings Safari's "press Esc" banner, the page sends each mouse move on to the element under the drawn cursor. That move bubbled back to the page's own handler and was sent on again until the stack ran out. In Safari it ran out inside a call into the core, which then refused every call, so the match froze. Chrome and Firefox only stuttered. Present since T4 (R7.65).
- **Fix:** `web/app.js` `wireInput` takes only the mouse's own moves.
- **Check:** `tests/browser/interactions.py` `locked_moves` tells the page the body holds the lock and moves the mouse; with the fix taken out it fails in all three engines (R7.66). Safari itself with a real pointer lock is the person's check.
- **Checks and live (21 October 2026):** with the fix taken out the new check failed in Chromium, Firefox and WebKit; `./scripts/publish.sh` ran `./scripts/check.sh` (**BUILDER CHECK PASSED**, Chromium, Firefox, WebKit) and published build `147c1c0`; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in all three engines on the live site, the new check included. No core change, so the cargo tests and break list were not re-run.

## Round 8 (24 October 2026): Tallow, Skipjack and the arena (T38)

- **Research:** two research agents; `notes/LITERATURE.md` section 9 (Vengeful Spirit and Mirana as of 6.72/6.73) and section 10 (SMITE 2's Arena, and SMITE 1's where SMITE 2 copied it). Where the sources disagree, the agent chose (R8.2, R8.3, R8.4, R8.8).
- **Mockups first:** `journal/mockups/heroes4.js`, sheets in `journal/art8/models/`; then `shoot.py export` and `tools/modelport.py` added `hero_tallow` and `hero_skipjack` to `data/models.json` with no drift in the other models.
- **The heroes:** `tallow` and `skipjack` in `data/units.json`, eight abilities in `data/abilities.json`, the roster's fourteenth and fifteenth in `data/rules.json`, strings, computer builds. Rules in `crates/core/src/ability.rs` (`terror_wave`, `command`, `nether_swap`, `starfall`, `elune_arrow`, `leap`, `moonlight`; Magic Missile reuses `assassinate`). An idle invisible unit no longer attacks on its own (R8.6). New shots `ember` and `star`, the `dusk` wave and the `stars` zone in `crates/core/src/marks.rs`. Portraits and icons in `tools/art/`.
- **The arena:** `crates/core/src/arena.rs` (tickets, portals, waves turn about, brutes, buff camps, the pace); its map is `data/map.json`'s `arena`, written by `tools/arena_map.py`, swapped into `rules.map` for an arena match (`Game::use_map`, R8.12). The mode switch goes Normal, All Random All Mid, Arena, in the custom match and the online lobby (R8.15); the top bar shows tickets. Two bugs found by playing computer players on it: the middle-lane seat stayed home (R8.13) and the shop was out of reach (R8.14).
- **Online:** version 9 (the roster changed); the lobby and start messages carry the arena; `three_browsers_on_the_arena_choose_any_hero_and_play_one_match` in `crates/core/src/online/tests.rs`.
- **Tests:** `tests/heroes3.rs` (10), `tests/arena.rs` (9), the online test (1); break rows "round 8:" and "arena:" (34). Shiver's and the heroes screen's tests count fifteen heroes.
- **Journal tab 14** (`journal/mockups/tab14.py`).
- **Checks (24 October 2026):** `./scripts/count-tests.sh` 215 passed, 0 failed, 5 ignored; `./scripts/check.sh` **BUILDER CHECK PASSED** (Chromium, Firefox, WebKit); `scripts/break_tests.py` on the 34 new rows, the rows the test changes touched and the six repointed rows: every one caught (R8.18); every row's text in `tests/breaks.json` matches once. The other older rows were not re-run. The online smoke test with `--arena` met through the relays and both browsers played one arena match. **Live:** `./scripts/publish.sh` ran `./scripts/check.sh` again (**BUILDER CHECK PASSED**) and published build `cd74532`; `BUILDER_ORIGIN=https://sgilson7.github.io/pots tests/browser/check.py` passed in Chromium, Firefox and WebKit on the live site; on the live site `tests/online/smoke.py --arena --rejoin` (version 9) met through the relays, played on the arena, rejoined the dropped browser, and the checksums agreed at tick 1800 (relay.damus.io answered 503 and rate-limited again; the others carried it).

## Open for the person

Each row of `SECOND-ORDER.md` with status *person*, by number:

- **R8.1**: Tallow's and Skipjack's names, looks and ability names (Ember Dart, Dusk Wave, Lamplight, Moth Switch; Star Shower, Comet Arrow, Lily Leap, Pond Mist). Keep, or change any?
- **R7.55**: Shiver's name, look and ability names (Frozen Ground, Snow Whirl, Frost Talons, Hailstar, Hailfall). Keep, or change any?
- **R7.44**: open each hero on the heroes screen and play each ability: does every demonstration read clearly?
- **R7.2**: selling is instant on a right-click and pays half. Ask first, or give a full refund for a few seconds after buying?
- **R7.9**: next for the look: the forest's outline, or a darker ground? (Shadows under heroes and creeps came with T24.)
- **R7.3**: play All Random All Mid at the new pace (level 3, 1.5× experience, 2× gold per second) and say whether it is too fast or too slow.
- **R1.2**: the names and looks of heroes, items and creeps. Are they far enough from their DotA sources?
- **R1.10**: one free courier per side from the start (DotA sold it). Keep?
- **R1.12**: lane creep armor type, Unarmored (playdota) or Medium (Dota 2 port). It changes ranged-creep damage 2×.
- **R1.16**: if the host's browser leaves, the online match ends. A host hand-over is not built.
- **R1.18**: the 143 icons; a few are plain at 48 px.
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
