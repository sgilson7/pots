# Protection of the Spheres

A browser game that plays like the Warcraft III map Defense of the Ancients (DotA Allstars, version 6.72/6.73), with heroes, items and art of its own. Two sides of five heroes push three lanes of creeps toward the other side's Sphere; the side whose Sphere breaks loses.

Play it at <https://sgilson7.github.io/pots/>.

- **Arcade:** ten missions that teach the game, from last-hitting alone in one lane to a full five against five, and a custom match where the size of each side and the skill of each side's computer players are set apart.
- **Online:** up to ten players by room code, with rollback netcode and rejoining a match after a dropped connection. Empty seats are played by the computer.

Ten heroes, each carrying the abilities and numbers of a DotA hero under an original name and look: Ironvow (after Sven), Rime (Crystal Maiden), Cinder (Lina), Thistle (Drow Ranger), Gristle (Pudge), Tectus (Earthshaker), Kestrel (Juggernaut), Hexwright (Lion), Longwick (Sniper) and Tempest (Zeus).

## How to play

Right-click the ground to walk, and an enemy to attack it. Q, W, E and R use abilities; a targeted ability waits for a click. Press O, then an ability key, to learn it. B opens the shop, Tab the score, Space centres the camera on your hero, A then a click attack-moves. Gold comes only from the killing blow, so land the last hit on enemy creeps, and deny your own below half health.

## What the game copies from DotA

Every number the game copies has a source in `notes/LITERATURE.md`: archived pages of playdota.com (IceFrog's site, captured at 6.72/6.73), IceFrog's changelogs from 6.48 to 6.79c, and player threads. The rules include DotA's attack-type and armor-type table, the armor formula, dice-rolled damage, pseudo-random bashes and crits, attack points and backswing, last hits and denies, the gold and experience formulas, creep waves and upgrades, mega creeps, neutral camps that refill on the minute, runes, the boss and its shield, the tower invulnerability chain, backdoor protection and the glyph, day and night, trees that block sight and paths, and high ground. Where the game simplifies, `SECOND-ORDER.md` says so, row by row.

## How it is built

A [Builder](https://github.com/sgilson7/builder-setting) project. A Rust core holds every rule and performs no input or output; a WebAssembly shim moves values and decides nothing; the page draws what the core returns.

| Path | Holds |
| --- | --- |
| `crates/core` | the match (`world.rs`, `combat.rs`, `orders.rs`, `lanes.rs`, `ability.rs`, `item.rs`, `ai.rs`), the screens and HUD (`game.rs`, `ui.rs`), the 3D scene (`scene.rs`, `models.rs`), online play (`online.rs`) |
| `crates/shim` | the `wasm-bindgen` functions the page calls |
| `web/` | the page: WebGL renderer, the view model's DOM, input, the online transport in `web/net/` |
| `data/` | every number and word: units, abilities, items, map, behavior trees, models, strings |
| `tools/` | generators for the data (`unitgen.py`, `abilitygen.py`, `itemgen.py`, `mapgen.py`, `modelgen.py`), the TikZ icon pipeline, screenshots |
| `tests/browser/` | the browser check; `tests/online/` the online smoke test and the five-runner test |

```bash
./scripts/setup.sh              # once: Python venv with Playwright and three browsers
cargo test --workspace          # every rule, in seconds
./scripts/build.sh              # release WebAssembly build into dist/
python3 scripts/serve.py        # http://127.0.0.1:8000/
./scripts/check.sh              # everything, before publishing
.venv/bin/python tests/online/runners.py runs/out 10   # five browsers play ten minutes online
```

Online play introduces browsers through public Nostr relays this project does not run, then connects them directly. Nothing loads from another site until a player hosts or joins.
