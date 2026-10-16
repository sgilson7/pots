# Protection of the Spheres style

The look of the game's icons: ability icons, item icons and hero portraits, each a badge 128 px square that must still read at 48 px. Every picture is TikZ written by a script in `tools/art/` and drawn in the roles of `palette.json`. Nothing here copies another game's icon art.

## What an icon looks like

- One badge per icon, drawn by `motifs.badge`: a coloured rim and a pale field (the rim colour washed toward `paper`). Abilities and hero portraits are round; items are rounded squares.
- The rim colour says what kind of thing the icon is (see below). The subject never takes its colour from the rim, so the subject still reads against the field.
- One clear subject per icon, large: it fills most of the field and may overlap the rim a little. Small secondary marks (stun stars, speed lines, a sparkle) say what the thing does: a thrown hammer gets a stun star, a dagger a blink trail, a boot a wing.
- Bold dark plum outlines (`ink`) on every filled shape, with round joins and caps. Interior detail lines are thinner (`motifs.THIN`, 1.2 pt) and also in `ink`.
- Flat fills only. A highlight is a second flat shape (`motifs.shine`, `motifs.sparkle`), never a gradient. No shadows, glow, blur or texture.
- Hero portraits are a head-and-shoulders bust clipped to the round field (`motifs.clipped`), facing the viewer or three-quarters, showing the hero's one defining feature (crested helm, hood, flame hair, mask, horns, goggles, cloud crown).

## Rim colour by kind

Abilities and hero portraits take the hero's element:

| Role | Element | Heroes |
| --- | --- | --- |
| `frost` | frost blue | Rime, Thistle |
| `fire` | fire orange | Cinder |
| `earth` | earth brown | Tectus, the Warden |
| `steelrim` | steel grey | Ironvow, Longwick |
| `blood` | blood red | Kestrel |
| `arcane` | arcane violet | Hexwright |
| `storm` | storm pale blue | Tempest |
| `rot` | rot green | Gristle |

Items take their shop tab: `consumables`, `basics`, `attributes`, `armaments`, `arcanetab` (the arcane tab), `secret`, `common`, `support`, `protect`, `weapons`, `caster`, and `relictab` for an item no shop sells. The tables mapping heroes and tabs to roles are in `tools/art/icons.py`.

## Sizes and line weights

Sizes, transparency and file caps come from `data/art.json`. Ink widths are in `ink.json`; an icon's is 2.2 pt, about 5 px at 128 px and 2 px at 48 px.

## Palette by role

| Roles | Use |
| --- | --- |
| `ink`, `shade` | outlines; holes and deep shadow shapes |
| `paper`, `snow`, `cloth`, `bone` | pale surfaces, highlights, cloth, bone |
| `steel`, `steeldark`, `iron` | blades, armour, iron |
| `gold`, `golddark` | hilts, trim, coins |
| `wood`, `wooddark`, `leather` | hafts, bows, boots |
| `skin`, `skinpale`, `skingrey` | faces and hands |
| `stone`, `stonedark`, `moss`, `leaf` | rock, earth, plants |
| `ice`, `icedeep`, `flame`, `flamehot`, `ember`, `spark` | frost, fire and lightning |
| `mana`, `life`, `heal`, `venom`, `violet`, `night`, `cloud` | magic effects |

Mixes in xcolor's form (`rot!34!paper`) are allowed when both names are roles.

## What not to do

- No words, letters or digits. The pipeline refuses TeX that could set text.
- No real person, brand or logo, and no copy of another game's icon.
- No colour value in picture code; name roles.
- No gradient, blur or soft shadow, and no filled shape without its ink outline.
- Nothing touching the picture's edge: the badge keeps 6 px clear on every side.
