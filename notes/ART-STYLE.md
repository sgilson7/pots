# The round-2 art style

Triage row T1 (16 October 2026). The request, word for word, is in `TRIAGE.md`. This note is the analysis behind journal tab 8. It covers the two references, the current look measured, the rules of the new look, and what the game must gain to draw it. The mockups are in `journal/mockups/` and the pictures in `journal/art2/`. This round does not change the game.

## The two references

**Chumbles** (Long Time Comin', `crates/core/src/draw/cat.rs`). One model, about 75 parts and 145 to 168 instances a pose, every part an ellipsoid (`Mesh::Sphere` stretched between two points by `span`), and every part outlined by an inverted hull in the ink colour. The head is drawn 1.3 times a real cat's "so his face reads". State is carried by the pose and the face: ears back or pricked, pupils slit or round, fur and tail bristled by scaling the tufts. Lessons taken: overlapping ellipsoids make a soft silhouette with no corners; one ink colour round everything holds the shapes together; an oversized head and a face that changes are what make a model read as a character.

The request says "voxel, sphere inspired". Chumbles is not built from voxels (Long Time Comin's arenas are boxes, Chumbles is spheres). This note reads the request as chunky and rounded, built from spheres, and uses no voxels. If a voxel look was meant, that is a different style; see the question in tab 8.

**Mage Tower Battle's mockups** (`MageTowerBattle/journal/mockups/mockup.js`, entry 1 of its journal). Its comment says "characters, built from spheres (after Chumbles, not cubes)". Bodies are stacked spheres (a robe is a squashed ball under a ball), the head is a big sphere about 40 % of the height, and the face is two dark dot eyes, a white catch-light in each, and pink blush beneath. Hats are a cone on a flattened brim. The light is a warm key with soft PCF shadows, a hemisphere fill, and ACES tone mapping. The palette is pastel but saturated (plum, mint, ember, gold, sky), and the panels are cream with a thick plum outline. It has no outline on the models; Chumbles has one.

## The current look, measured

From `data/models.json` (written by `tools/modelgen.py`) and `data/shaders/scene.frag`:

| Measure | Round 1 | Round 2 mockups |
| --- | --- | --- |
| Parts that are cubes or wedges | 278 of 546 (237 cubes, 41 wedges) | none |
| Hero parts that are cubes or wedges | 137 of 240 | none |
| A hero's head, as a share of its height | 18 to 28 % | about 40 % |
| Parts a hero | 22 to 26 | 24 to 38 |
| Light | three hard bands (`bands` 45 / 0 / 85 / 70), no shadow | soft key, sky fill, soft shadow |
| Faces | two dark cubes or spheres, no catch-light | dot eyes with catch-light, blush |
| Ability visuals | a coloured octahedron for a missile, a tinted disc for a zone | an animation for each of the 41 |

The blockiness the person names has three causes, and the cubes are only the first. The others are the banded light, which turns every curve into flat facets, and the proportions: thin limbs and small heads read as soldiers, not toys.

## Rules of the new look

1. **Nothing alive has a corner.** Bodies, limbs, heads, hats and weapons are spheres, capsules, cylinders, cones and torus arcs. Buildings use drums, domes, cones and rounded stone courses.
2. **Chibi proportions.** For a hero, the head is about 40 % of its height (radius 0.33 m on a 1.5 m hero), the torso about 30 % and the legs about 20 %. Hands and feet are oversized balls. Creeps are about 1.0 to 1.1 m tall, simpler, and plainly not heroes. Towers are about 4 m, the Spheres about 5 m, and the Warden about 3 m.
3. **Faces carry the style.** Every creature with a face gets two dot eyes set low and wide, a white catch-light in each, and blush. Masks and visors keep the eye dots or a slit. Glowing eyes mark the sinister ones (Hexwright, the Moonward caster).
4. **Each hero keeps its round-1 cue.** Ironvow's blade taller than he is; Rime's hood and crystal staff; Cinder's flame hair; Thistle's bow and frost-tipped quiver; Gristle's belly, hook and cleaver; Tectus's horns and totem club; Kestrel's mask and topknot; Hexwright's horned hood and claw staff; Longwick's goggles, nose and long rifle; Tempest's beard and cloud crown. These are what a player reads from the raised camera.
5. **One thin warm outline.** Every solid part has a plum (#2b2140) inverted-hull outline 0.022 m thick, about 1.5 % of a hero's height. Glows and see-through effects have none.
6. **Soft light.** A warm key from high front-right with soft shadows, a sky-blue hemisphere fill from above, a warm bounce from the ground, and filmic tone mapping. Never hard bands.
7. **Pastel but saturated, and team colour on every unit.** Sunward is gold, cream and sand; Moonward is violet, lilac and slate. Every unit also stands on a soft ring in its side's colour. Colour is never the only cue: each rune also has its own shape, and each side's creeps their own silhouette.
8. **Every ability in four beats.** A cast pose on the caster (arm raised, swing, crouch); a travel or warning phase (a projectile in flight, a ring on the ground before a delayed blast lands, an aim line); an impact (a flash on whoever is hit, squash and stretch, a burst); and a mark on whoever is affected, for as long as it lasts. The marks are fixed across the game: circling yellow stars for a stun, frost on the feet and a blue tint for a slow, a crossed-out bubble above the head for a silence, a puff of smoke and a critter for a hex, a coloured ring and motes for a buff or an aura. Effects are chunky and rounded, coloured by element: frost blue, fire orange, Ironvow's gold, rot green, earth teal, arcane pink, storm yellow and blue.

## Readability at the game's camera

In round 1's match screenshot (`journal/img/arc_hud.jpg`, 1600 × 900), a hero is about 75 pixels tall. At 40 % of that, the head is about 30 pixels and an eye about 4 pixels: the eyes and blush read, but a catch-light is a pixel or two and only shows when zoomed in. The game looks down at 56°, which shows the top of a big head more than its face. The ability pictures look down at about 42° and the model pictures at about 20°. Lowering the game's camera, or tilting heads back a few degrees in the walk and idle poses, is a choice for the person (SECOND-ORDER R2.5).

## Porting to the game (the next round, not this one)

The mockups are drawn by three.js so the style could be chosen before the renderer changes (R2.2). The joints match the game's (`body`, `head`, `arm_l`, `arm_r`, `leg_l`, `leg_r`), and every part is a mesh the game has or one of two it would gain, so each model ports to `tools/modelgen.py` as data. What the game must gain:

- **Meshes.** A Capsule (a cylinder with hemispherical ends; or two spheres and a cylinder per limb), torus arcs at a few fixed sweeps (90°, 140°, 180°, 360°) for horns, bows, hooks and rims, and a finer Sphere for heads (16 × 24 instead of 10 × 16, so a big head's outline is smooth).
- **The shader.** Replace the three bands with soft wrap lighting plus a hemisphere fill, with the numbers as uniforms from the core as now (`data/presentation.json`). A soft round contact shadow under each unit is a tint-layer disc and costs nothing; a real shadow map needs a depth pass in the page and a light matrix from the core, and can come later.
- **Effects from the core.** The core would note each ability's beats as effect records (cast began, missile out, landed, proc, status applied), each with its ability, source, target, point and tick. The scene would draw each one as a pure function of ticks since it began, as the mockups do with `update(t)`, with sizes, colours and durations in a new `data/fx.json`. Status marks (stun stars, slow frost, silence bubble, hex) would be drawn each frame from the unit's modifiers, which are already in the world, so rollback replays them for free. Every frame stays a function of the world and the tick, so online browsers agree.
- **Tests.** Following rule 6 of `CLAUDE.md`, there would be a draw-list test for each mark (a stunned unit draws stars, a silenced one its bubble), each with its break row in `tests/breaks.json`.
- **Budget.** Parts per hero rise from 22 to 26 now to 24 to 38. Each part is an instance, so the instance count rises by about half for heroes, while creeps (the most numerous) stay near today's count. Round 1 measured 60 frames a second with ten heroes and full waves (`MEASUREMENTS.md`). That should be measured again after the port, not assumed.

## What the mockups do not show

They show no fog, no health bars and no HUD. The ability pictures are on a small patch of lane, not the real map. Large areas are drawn smaller to fit the frame, and long durations (channels of 6 to 10 seconds, auras) are shortened into a few seconds; the description under each says which. The lighting is three.js's, which is better than the game's today. The pictures show the style's aim, not what the game draws now.
