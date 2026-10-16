# Licences

Every file in this repository that is not code written for it: its source, its author, its terms. The code is under the repository's licence.

## Assets

| Path | Source | Terms |
| --- | --- | --- |
| `web/art/` | drawn by `tools/art/art.py` from TikZ in this repository (style pack `styles/pots/`) | this repository's licence |
| `data/models.json` | written by `tools/modelgen.py` in this repository; every model is original | this repository's licence |
| `journal/mockups/vendor/three.module.js` | three.js r160 (three.js authors), pinned by `journal/mockups/vendor/SHA256SUMS`; used only by the journal's art mockups, never by the game | MIT |
| `journal/mockups/*.js`, `journal/art2/` | the round-2 art mockups and their pictures, drawn in this repository; every model is original | this repository's licence |
| hero, item, ability and unit names in `data/strings.json` | written for this game; none is a Blizzard or IceFrog name | this repository's licence |

## Vendored code

| Path | Project and version | Author | Licence | Checksum |
| --- | --- | --- | --- | --- |
| `web/net/vendor/trystero-nostr-0.25.4.js` | Trystero 0.25.4, Nostr strategy, bundled by esm.sh | Dan Motzenbecker | MIT | `6bfce15d72a64384cc66c2693917994e1b900f4f98c9b7a2d54e4e86f5202906` |

## Code taken from other repositories

| From | What | Licence |
| --- | --- | --- |
| [Builder](https://github.com/sgilson7/builder-setting) `template/` and kits (canvas, realtime, online) | the project's skeleton, `rng.rs`, `draw.rs`, `render2d.js`, `online_fence.rs`, the checks and scripts | MIT (Sam Gilson) |
| Gatecrasher (`sgilson7/gatecrasher`) | `fx.rs`, `geom.rs`, `mesh.rs`, `render.rs` (portal passes removed), `gl.js` (simplified) | MIT (Sam Gilson) |
| Galactic Survivors (`sgilson7/galactic-survivors`) | `web/net/config.js`, `web/net/rtc.js`; the rollback design | MIT (Sam Gilson) |

The game's rules and numbers follow DotA Allstars (by IceFrog and earlier authors) as described in public sources listed in `notes/LITERATURE.md`. No file, model, icon, sound or text from Warcraft III or DotA is included.

A file with no licence, a likeness, a recording or any third-party picture is a worklist row in `SECOND-ORDER.md` for the person to decide, and stays out of git until they do.
