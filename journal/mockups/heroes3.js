// A hero proposed in round 7 (journal tab 13), in the round-2 style on the
// shared chibi body: Shiver (Ancient Apparition's abilities). Not a
// skeletal frost spirit and not Valve's look: an old snow owl woken from a
// glacier, floating with no feet on the ground. Its outline reads from the
// raised camera: a round owl head with icicle ear tufts, two wide wings of
// icicle feathers, and a tail of frost mist where its legs would be.
import { C, glow, shiny, ball, orb, limb, cone, gem, hoop, joint, veil, halo } from "./kit.js";
import { chibi } from "./heroes.js";
import { rig } from "./kit.js";

const D = Math.PI / 180;
const TEAM = { sun: C.sun, moon: C.moon };
const SNOW = 0xd6e6f6;
const PLUME = 0xbcdcf5;
const DEEP = 0x7fb2dc;

function finishHero(c, team, extra = {}) {
  const r = rig(c.root, c.parts);
  return { g: c.root, rig: r, at: c.at, team, ...extra };
}

/** A wing folded down the arm: four long feathers fanned back, each
 * ending in an icicle. `s` is the side (-1 left, 1 right). */
function wing(s) {
  const w = joint(0, 0, 0);
  for (let i = 0; i < 4; i++) {
    const len = 0.34 + i * 0.05;
    const back = -0.05 - i * 0.07;
    const f = ball(0.075, len, 0.035, s * 0.06, -len * 0.75, back, i % 2 ? PLUME : SNOW, { rot: [-12 - i * 9, 0, s * (8 + i * 5)] });
    w.add(f);
    w.add(cone(0.045, 0.16, s * (0.1 + i * 0.03), -len * 1.45 - 0.08, back - 0.05 - i * 0.03, shiny(C.ice), { rot: [180 - i * 9, 0, -s * (8 + i * 5)] }));
  }
  // the leading edge: a darker bar of feathers along the arm
  w.add(ball(0.06, 0.22, 0.05, s * 0.02, -0.14, 0.04, DEEP));
  return w;
}

// ------------------------------------------------------------------ 13
/** Shiver (after Ancient Apparition): an old snow owl woken from the ice,
 * floating on a tail of frost mist; it throws snowflakes and calls down a
 * star of hail from anywhere on the map. */
export function shiver(team = "sun") {
  const c = chibi({ skin: SNOW, torso: [0.34, 0.34, 0.3], torsoColor: SNOW, sleeve: PLUME, legs: false, legLen: 0.42, hands: DEEP, headR: 0.38, face: false });
  const { head, armR, armL, body, legL, legR } = c.parts;
  const hr = c.hr;
  // the breast: rows of small pale-blue scallops
  body.add(ball(0.24, 0.26, 0.12, 0, 0.32, 0.2, C.white, { ink: false }));
  for (let row = 0; row < 3; row++) {
    for (let i = -1; i <= 1; i++) {
      body.add(cone(0.04, 0.06, i * 0.1 + (row % 2) * 0.05 - 0.025, 0.2 + row * 0.11, 0.31 - row * 0.012, DEEP, { ink: false, rot: [80, 0, 180] }));
    }
  }
  // no legs: a tail of frost mist and three icicles hanging under the body
  body.add(cone(0.26, 0.42, 0, -0.36, -0.02, veil(C.frost, 0.55), { rot: [180, 0, 0] }));
  body.add(cone(0.16, 0.26, 0, -0.5, 0.02, veil(C.ice, 0.7), { rot: [180, 0, 0] }));
  for (const [x, z, h] of [[-0.14, 0.06, 0.24], [0.12, 0.08, 0.3], [0, -0.12, 0.2]]) {
    body.add(cone(0.05, h, x, -0.02, z, shiny(C.ice), { rot: [180, 0, 0] }));
  }
  // a mantle of the side's colour round the neck, clasped with a frost gem
  body.add(hoop(0.27, 0.06, 0, 0.56, 0.02, TEAM[team], { rot: [90, 0, 0] }));
  body.add(gem(0.06, 0.08, 0.04, 0, 0.5, 0.28, glow(C.frost, 0.9)));
  // the owl's head: round, a pale facial disc, two wide glowing eyes and a beak
  head.add(orb(hr, 0, hr * 0.95, 0, SNOW));
  // the facial disc: two white rounds side by side, the eyes in them
  for (const s of [-1, 1]) head.add(ball(hr * 0.46, hr * 0.5, hr * 0.2, s * hr * 0.34, hr * 0.92, hr * 0.84, C.white, { ink: false }));
  for (const s of [-1, 1]) {
    head.add(ball(hr * 0.24, hr * 0.26, hr * 0.1, s * hr * 0.34, hr * 0.98, hr * 1.0, 0x23345a, { ink: false }));
    head.add(ball(hr * 0.19, hr * 0.21, hr * 0.08, s * hr * 0.34, hr * 0.98, hr * 1.06, glow(C.frost, 1.6), { ink: false }));
    head.add(orb(hr * 0.08, s * hr * 0.32, hr * 0.98, hr * 1.13, 0x23345a, { ink: false }));
    // brows of the side's colour, swept up to the tufts
    head.add(ball(hr * 0.26, hr * 0.06, hr * 0.06, s * hr * 0.4, hr * 1.32, hr * 0.84, TEAM[team], { ink: false, rot: [0, 0, s * 18] }));
    // icicle ear tufts, raking back and out
    head.add(cone(hr * 0.2, hr * 1.0, s * hr * 0.52, hr * 1.45, 0, shiny(C.ice), { rot: [-15, 0, -s * 30] }));
    head.add(cone(hr * 0.1, hr * 0.55, s * hr * 0.74, hr * 1.36, -hr * 0.2, shiny(C.frost), { rot: [-30, 0, -s * 50] }));
  }
  head.add(cone(hr * 0.13, hr * 0.34, 0, hr * 0.72, hr * 1.02, 0x5d6b80, { rot: [110, 0, 0] }));
  // a crown of frost behind the head: three small shards
  for (const a of [-30, 0, 30]) {
    head.add(gem(hr * 0.08, hr * 0.22, hr * 0.06, Math.sin(a * D) * hr * 0.5, hr * 1.7, -hr * 0.5, shiny(C.ice), { rot: [-35, 0, -a] }));
  }
  // the wings, folded down the arms; the right one holds a snowflake to throw
  const wl = wing(-1);
  armL.add(wl);
  armL.rotation.set(-10 * D, 0, -32 * D);
  const wr = wing(1);
  armR.add(wr);
  armR.rotation.set(-10 * D, 0, 32 * D);
  const flake = joint(0.04, -0.32, 0.12);
  for (let i = 0; i < 3; i++) flake.add(gem(0.025, 0.11, 0.025, 0, 0, 0, glow(C.ice, 1.2), { rot: [0, 0, i * 60], ink: false }));
  flake.add(halo(C.frost, 0.35, 0.5));
  armR.add(flake);
  c.at.flake = flake;
  c.root.remove(legL, legR);
  return finishHero(c, team);
}

export const MODELS = { hero_shiver: shiver };

export const NOTES = {
  hero_shiver: "An old snow owl woken from a glacier, floating on a tail of frost mist: a round owl head with glowing eyes and icicle ear tufts, brows and a mantle in its side's colour, wings of icicle feathers and a snowflake in one wing to throw.",
};
