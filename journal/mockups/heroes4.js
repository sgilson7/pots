// Two heroes proposed in round 8 (journal tab 14), in the round-2 style on
// the shared chibi body.
//
// Tallow (Vengeful Spirit's abilities). Not a winged harpy and not Valve's
// look: a lantern moth, round and fuzzy, with two pairs of wide dusk wings
// marked with an eye spot in its side's colour, feathered antennae, and an
// ember held in one hand to throw.
//
// Skipjack (Mirana's abilities). Not a moon priestess and no great cat: a
// pond frog with a reed bow, a hood of night cloth sewn with stars, a
// quiver of star-tipped arrows and long folded legs for the leap.
import { C, glow, shiny, ball, orb, limb, cone, gem, hoop, joint, halo, cyl } from "./kit.js";
import { chibi } from "./heroes.js";
import { rig } from "./kit.js";

const D = Math.PI / 180;
const TEAM = { sun: C.sun, moon: C.moon };
const TEAM_DEEP = { sun: C.sunDeep, moon: C.moonDeep };

function finishHero(c, team, extra = {}) {
  const r = rig(c.root, c.parts);
  return { g: c.root, rig: r, at: c.at, team, ...extra };
}

// ------------------------------------------------------------------ 14
const FUZZ = 0xf3e2c0;
const DUSK = 0x8a6f9e;
const DUSK2 = 0xb79bc4;
const AMBER = 0xffb347;

/** One moth wing: a wide rounded vane with a darker rim and an eye spot.
 * `s` is the side (-1 left, 1 right); `big` the upper wing. */
function mothWing(s, big, team) {
  const w = joint(0, 0, 0);
  const [rx, ry] = big ? [0.6, 0.42] : [0.4, 0.28];
  const y = big ? 0.3 : -0.12;
  const tilt = big ? 28 : -22;
  w.add(ball(rx, ry, 0.03, s * rx * 0.95, y, -0.02, DUSK, { rot: [0, 0, s * tilt] }));
  w.add(ball(rx * 0.82, ry * 0.8, 0.034, s * rx * 0.98, y + (big ? 0.02 : -0.01), 0, DUSK2, { ink: false, rot: [0, 0, s * tilt] }));
  // the eye spot: the side's colour round an amber glow
  const ex = s * rx * (big ? 1.18 : 1.1), ey = y + (big ? 0.07 : -0.04);
  w.add(ball(rx * 0.3, rx * 0.3, 0.038, ex, ey, 0.01, TEAM[team], { ink: false }));
  w.add(ball(rx * 0.15, rx * 0.15, 0.042, ex, ey, 0.02, glow(AMBER, 1.3), { ink: false }));
  return w;
}

/** Tallow (after Vengeful Spirit): a lantern moth that throws embers,
 * sends a wave of dusk ahead of it and trades places with a foe. */
export function tallow(team = "sun") {
  const c = chibi({ skin: FUZZ, torso: [0.32, 0.34, 0.3], torsoColor: FUZZ, sleeve: DUSK, legs: 0x5d4a6e, legLen: 0.2, hands: 0x5d4a6e, headR: 0.34, face: false });
  const { head, armR, armL, body } = c.parts;
  const hr = c.hr;
  // a fuzzy ruff round the neck in the side's colour, and a banded belly
  body.add(hoop(0.26, 0.08, 0, 0.58, 0.02, TEAM[team], { rot: [90, 0, 0] }));
  for (let i = 0; i < 3; i++) body.add(hoop(0.3 - i * 0.04, 0.025, 0, 0.36 - i * 0.12, 0, DUSK2, { rot: [90, 0, 0], ink: false }));
  // two pairs of wings, spread from the back
  const wings = joint(0, 0.46, -0.2);
  wings.rotation.set(-12 * D, 0, 0);
  for (const s of [-1, 1]) {
    wings.add(mothWing(s, true, team));
    wings.add(mothWing(s, false, team));
  }
  body.add(wings);
  c.at.wings = wings;
  // the head: round and fuzzy, two big dark eyes with an amber shine
  head.add(orb(hr * 1.02, 0, hr * 0.95, 0, FUZZ));
  for (const s of [-1, 1]) {
    head.add(ball(hr * 0.36, hr * 0.42, hr * 0.26, s * hr * 0.46, hr * 0.98, hr * 0.7, 0x2b2140));
    head.add(orb(hr * 0.11, s * hr * 0.4, hr * 1.12, hr * 0.94, glow(AMBER, 1.4), { ink: false }));
    // feathered antennae: a curved stalk with fronds
    const a = joint(s * hr * 0.28, hr * 1.75, hr * 0.2);
    a.rotation.set(-25 * D, 0, -s * 22 * D);
    a.add(limb([0, 0, 0], [0, hr * 0.9, 0], 0.018, 0x5d4a6e));
    for (let i = 0; i < 4; i++) {
      a.add(ball(0.07 - i * 0.01, 0.02, 0.012, s * 0.05, hr * (0.3 + i * 0.17), 0, DUSK2, { rot: [0, 0, s * 30] }));
    }
    head.add(a);
  }
  // a small smiling mouth under the eyes
  head.add(hoop(hr * 0.12, hr * 0.025, 0, hr * 0.62, hr * 0.98, 0x2b2140, { arc: 180, rot: [0, 0, 180], ink: false }));
  // arms forward; the right holds an ember to throw
  armL.rotation.set(-20 * D, 0, -18 * D);
  armR.rotation.set(-40 * D, 0, 18 * D);
  const ember = joint(0.02, -0.32, 0.1);
  ember.add(orb(0.07, 0, 0, 0, glow(AMBER, 1.8), { ink: false }));
  ember.add(orb(0.04, 0, 0, 0, glow(C.white, 2), { ink: false }));
  ember.add(halo(AMBER, 0.45, 0.7));
  armR.add(ember);
  c.at.ember = ember;
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 15
const FROG = 0x7cc66a;
const FROG2 = 0xc9e8a0;
const NIGHT = 0x2f3a6e;
const STAR = 0xfff2a8;

/** Skipjack (after Mirana): a pond frog with a reed bow, who calls stars
 * down, shoots a long arrow that stuns more the farther it flew, leaps
 * forward and hides its allies in pond mist. */
export function skipjack(team = "sun") {
  const c = chibi({ skin: FROG, torso: [0.34, 0.3, 0.3], torsoColor: FROG, sleeve: FROG, legs: false, legLen: 0.16, hands: FROG, headR: 0.36, face: false });
  const { head, armR, armL, body, legL, legR } = c.parts;
  const hr = c.hr;
  // a pale belly
  body.add(ball(0.25, 0.22, 0.14, 0, 0.26, 0.2, FROG2, { ink: false }));
  // folded frog legs: a thick thigh and a long foot, crouched
  for (const [s, leg] of [[-1, legL], [1, legR]]) {
    leg.add(ball(0.13, 0.11, 0.18, s * 0.06, -0.04, -0.02, FROG, { rot: [-20, 0, 0] }));
    leg.add(ball(0.07, 0.04, 0.18, s * 0.1, -0.15, 0.1, FROG));
    for (const t of [-1, 0, 1]) leg.add(orb(0.03, s * 0.1 + t * 0.045, -0.16, 0.27, FROG2));
  }
  // a hood and short cape of night cloth sewn with stars, trimmed in the side's colour
  body.add(cone(0.36, 0.42, 0, 0.12, -0.08, NIGHT, { rot: [-8, 0, 0] }));
  body.add(hoop(0.28, 0.045, 0, 0.56, 0.02, TEAM[team], { rot: [90, 0, 0] }));
  for (const [x, y] of [[-0.18, 0.3], [0.14, 0.22], [0.02, 0.42], [-0.06, 0.16], [0.2, 0.4]]) {
    body.add(gem(0.025, 0.04, 0.02, x, y, -0.36, glow(STAR, 1.3), { ink: false }));
  }
  // a quiver on the back: star-tipped arrows
  const quiver = joint(0.14, 0.4, -0.3);
  quiver.rotation.set(0, 0, -20 * D);
  quiver.add(cyl(0.07, 0.36, 0, 0, 0, C.wood));
  for (const x of [-0.03, 0.02, 0.05]) {
    quiver.add(limb([x, 0.16, 0], [x, 0.3, 0], 0.012, C.woodDark));
    quiver.add(gem(0.035, 0.05, 0.02, x, 0.34, 0, glow(STAR, 1.2), { ink: false }));
  }
  body.add(quiver);
  // the head: wide, flat-topped, eyes up on top, a long smile
  head.add(ball(hr * 1.12, hr * 0.82, hr * 0.98, 0, hr * 0.8, 0, FROG));
  head.add(ball(hr * 0.98, hr * 0.38, hr * 0.8, 0, hr * 0.5, hr * 0.18, FROG2, { ink: false }));
  for (const s of [-1, 1]) {
    head.add(orb(hr * 0.36, s * hr * 0.55, hr * 1.42, hr * 0.28, FROG));
    head.add(orb(hr * 0.27, s * hr * 0.58, hr * 1.48, hr * 0.48, C.white, { ink: false }));
    head.add(ball(hr * 0.13, hr * 0.17, hr * 0.06, s * hr * 0.6, hr * 1.5, hr * 0.74, 0x241a30, { ink: false }));
    head.add(orb(hr * 0.05, s * hr * 0.55, hr * 1.58, hr * 0.78, C.white, { ink: false }));
  }
  head.add(hoop(hr * 0.62, hr * 0.03, 0, hr * 0.78, hr * 0.62, 0x2b2140, { arc: 150, rot: [-10, 0, 195], ink: false }));
  // the hood: night cloth over the back of the head, between the eyes, sewn with stars
  head.add(ball(hr * 1.2, hr * 0.78, hr * 0.9, 0, hr * 1.12, -hr * 0.36, NIGHT));
  head.add(ball(hr * 1.22, hr * 0.12, hr * 0.5, 0, hr * 0.62, -hr * 0.3, TEAM[team]));
  for (const [x, y, z] of [[-0.5, 1.45, -0.2], [0.35, 1.5, -0.3], [0, 1.62, -0.6], [-0.2, 1.3, -0.95], [0.6, 1.2, -0.7]]) {
    head.add(gem(hr * 0.07, hr * 0.1, hr * 0.04, x * hr, y * hr, z * hr, glow(STAR, 1.4), { ink: false }));
  }
  // its peak falls behind, tipped with a star
  head.add(cone(hr * 0.42, hr * 0.8, 0, hr * 1.0, -hr * 0.9, NIGHT, { rot: [-115, 0, 0] }));
  head.add(gem(hr * 0.12, hr * 0.16, hr * 0.06, 0, hr * 0.62, -hr * 1.65, glow(STAR, 1.5), { ink: false }));
  // the reed bow in the left hand, drawn back by the right
  const bow = joint(0, -0.3, 0.06);
  bow.add(hoop(0.42, 0.022, 0, 0, 0, 0x8fae55, { arc: 150, rot: [0, 90, 105] }));
  bow.add(limb([0, 0.4, -0.12], [0, -0.4, -0.12], 0.006, C.white, { ink: false }));
  for (const y of [-0.3, 0, 0.3]) bow.add(hoop(0.04, 0.012, 0, y, 0.04, TEAM_DEEP[team], { rot: [90, 0, 0], ink: false }));
  armL.add(bow);
  armL.rotation.set(-70 * D, 0, -10 * D);
  armR.rotation.set(-60 * D, 0, 30 * D);
  const arrow = joint(0, -0.3, 0.1);
  arrow.add(limb([0, 0, 0], [0, 0, 0.36], 0.012, C.woodDark));
  arrow.add(gem(0.04, 0.05, 0.03, 0, 0, 0.4, glow(STAR, 1.5), { ink: false, rot: [90, 0, 0] }));
  armR.add(arrow);
  c.at.arrow = arrow;
  return finishHero(c, team);
}

export const MODELS = { hero_tallow: tallow, hero_skipjack: skipjack };

export const NOTES = {
  hero_tallow: "A lantern moth, round and fuzzy: two pairs of dusk wings with an eye spot in its side's colour, feathered antennae, big dark eyes and an ember in one hand to throw.",
  hero_skipjack: "A pond frog with eyes on top of its head, long folded legs, a hood of night cloth sewn with stars, a reed bow and a quiver of star-tipped arrows.",
};
