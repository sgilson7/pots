// Two heroes proposed in round 7 (journal tab 12), in the round-2 style on
// the shared chibi body: Hackle (Troll Warlord's abilities) and Chipper
// (Timbersaw's). Neither is a troll or a goblin, and neither copies a
// Blizzard or Valve look; each keeps an outline a player can read from the
// raised camera: twin hatchets and a bristle crest; a saw-blade chest, a
// chain claw and a beaver's tail.
import { C, glow, shiny, ball, orb, limb, cyl, cone, hoop, joint } from "./kit.js";
import { chibi } from "./heroes.js";
import { rig } from "./kit.js";

const D = Math.PI / 180;
const TEAM = { sun: C.sun, moon: C.moon };

function finishHero(c, team, extra = {}) {
  const r = rig(c.root, c.parts);
  return { g: c.root, rig: r, at: c.at, team, ...extra };
}

/** A hatchet: a short haft, a wide steel head with a bright edge. */
function hatchet(team) {
  const h = joint(0, -0.3, 0.03);
  h.add(limb([0, -0.1, 0], [0, 0.5, 0], 0.032, C.wood));
  h.add(hoop(0.04, 0.014, 0, -0.02, 0, C.leather));
  // the head: a wide steel crescent out to one side of the haft, a bright edge
  h.add(ball(0.05, 0.17, 0.15, 0, 0.42, 0.13, shiny(C.steel)));
  h.add(ball(0.016, 0.17, 0.05, 0, 0.42, 0.27, shiny(0xffffff)));
  h.add(orb(0.035, 0, 0.53, 0, TEAM[team]));
  // held forward and up, ready to swing or throw
  h.rotation.set(75 * D, 0, 0);
  return h;
}

// ------------------------------------------------------------------ 11
/** Hackle (after Troll Warlord): a boar brawler with a bristle crest and a
 * hatchet in each hand; thrown from afar, swung up close. */
export function hackle(team = "sun") {
  const hide = 0x8a6a8e;
  const c = chibi({ skin: hide, torso: [0.36, 0.33, 0.32], torsoColor: 0x6f8f56, sleeve: hide, legs: 0x5a4636, boots: 0x3f3128, hands: hide, belt: C.leather, headR: 0.34 });
  const { head, armR, armL, body } = c.parts;
  const hr = c.hr;
  // the snout, nostrils and two upturned tusks
  head.add(ball(hr * 0.42, hr * 0.32, hr * 0.3, 0, hr * 0.62, hr * 0.86, 0xc9909a));
  for (const s of [-1, 1]) {
    head.add(orb(hr * 0.06, s * hr * 0.14, hr * 0.66, hr * 1.14, 0x5a3a44, { ink: false }));
    head.add(cone(hr * 0.09, hr * 0.42, s * hr * 0.34, hr * 0.62, hr * 0.92, C.bone, { rot: [-20, 0, s * 25] }));
  }
  // ears, and a bristle crest in the side's colour from brow to nape
  for (const s of [-1, 1]) head.add(cone(hr * 0.2, hr * 0.42, s * hr * 0.78, hr * 1.5, 0, hide, { rot: [0, 0, s * -40] }));
  for (let i = 0; i < 6; i++) {
    const a = (-20 + i * 32) * D;
    head.add(cone(0.085, 0.3 - Math.abs(i - 2.5) * 0.025, 0, hr * 1.02 + Math.cos(a) * hr * 0.98, -Math.sin(a) * hr * 0.98, TEAM[team], { rot: [-(-20 + i * 32), 0, 0] }));
  }
  // war paint under the eyes, a bone necklace, a leather harness
  for (const s of [-1, 1]) head.add(ball(hr * 0.16, hr * 0.04, hr * 0.05, s * hr * 0.36, hr * 0.84, hr * 0.92, TEAM[team], { ink: false }));
  for (let i = 0; i < 5; i++) body.add(cone(0.03, 0.08, (i - 2) * 0.09, 0.56, 0.3 - Math.abs(i - 2) * 0.03, C.bone, { rot: [180, 0, 0] }));
  body.add(limb([-0.3, 0.6, 0.18], [0.28, 0.12, 0.26], 0.035, C.leather));
  // a hatchet in each hand, raised and ready
  const hr2 = hatchet(team);
  armR.add(hr2);
  armR.rotation.set(-25 * D, 0, 18 * D);
  const hl = hatchet(team);
  armL.add(hl);
  armL.rotation.set(-25 * D, 0, -18 * D);
  c.at.axeR = joint(0, 0.42, 0.13); hr2.add(c.at.axeR);
  c.at.axeL = joint(0, 0.42, 0.13); hl.add(c.at.axeL);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 12
/** Chipper (after Timbersaw): a beaver lumberjack in a little wooden walker,
 * a saw blade spinning on its chest, a chain claw on one arm and a blade
 * launcher on the other shoulder. */
export function chipper(team = "sun") {
  const fur = 0xa8733f;
  const c = chibi({ skin: fur, torso: [0.42, 0.36, 0.38], torsoColor: C.wood, sleeve: C.woodDark, legs: C.woodDark, boots: C.iron, hands: C.iron, belt: C.iron, headR: 0.32 });
  const { head, armR, armL, body } = c.parts;
  const hr = c.hr;
  // a beaver's muzzle, two big front teeth, round ears and a hard hat
  head.add(ball(hr * 0.46, hr * 0.32, hr * 0.32, 0, hr * 0.62, hr * 0.84, 0xd9b38a));
  head.add(orb(hr * 0.1, 0, hr * 0.78, hr * 1.12, 0x4a3226, { ink: false }));
  for (const s of [-1, 1]) {
    head.add(ball(hr * 0.08, hr * 0.14, hr * 0.04, s * hr * 0.09, hr * 0.4, hr * 1.06, C.white));
    head.add(orb(hr * 0.2, s * hr * 0.78, hr * 1.5, -hr * 0.1, fur));
  }
  head.add(ball(hr * 1.08, hr * 0.5, hr * 1.08, 0, hr * 1.62, -hr * 0.05, shiny(TEAM[team])));
  head.add(ball(hr * 1.3, hr * 0.08, hr * 1.3, 0, hr * 1.36, hr * 0.1, shiny(TEAM[team])));
  // the walker's planks and rivets, the saw blade on the chest
  for (const y of [0.18, 0.42]) body.add(ball(0.44, 0.03, 0.4, 0, y, 0, C.woodDark));
  for (const s of [-1, 1]) body.add(orb(0.03, s * 0.3, 0.3, 0.36, C.steelDark));
  const saw = joint(0, 0.34, 0.4);
  saw.add(cyl(0.22, 0.04, 0, 0, 0, shiny(C.steel), { rot: [90, 0, 0] }));
  for (let i = 0; i < 12; i++) {
    const a = (i * 30) * D;
    saw.add(cone(0.04, 0.07, Math.cos(a) * 0.24, Math.sin(a) * 0.24, 0, shiny(C.steel), { rot: [0, 0, i * 30 - 90] }));
  }
  saw.add(orb(0.06, 0, 0, 0.03, TEAM[team]));
  body.add(saw);
  c.at.saw = saw;
  // a flat paddle tail behind
  body.add(ball(0.2, 0.05, 0.34, 0, 0.08, -0.5, 0x6e4a2f, { rot: [20, 0, 0] }));
  // the left arm ends in a three-fingered chain claw on a coil of chain
  const claw = joint(0, -0.3, 0.03);
  for (const s of [-1, 0, 1]) claw.add(cone(0.035, 0.16, s * 0.05, -0.08, 0.04, shiny(C.steelDark), { rot: [180 + s * 18, 0, 0] }));
  claw.add(hoop(0.08, 0.02, 0, 0.04, 0, C.iron));
  claw.add(hoop(0.06, 0.018, 0, 0.12, 0, C.iron, { rot: [0, 90, 0] }));
  armL.add(claw);
  armL.rotation.set(-20 * D, 0, -10 * D);
  c.at.claw = joint(0, -0.16, 0.04); claw.add(c.at.claw);
  // the right shoulder carries a drum that launches the big blade
  const drum = joint(0.42, 0.62, -0.05);
  drum.add(cyl(0.12, 0.26, 0, 0, 0, C.iron, { rot: [70, 0, 0] }));
  drum.add(cyl(0.16, 0.025, 0, 0.06, 0.12, shiny(C.steel), { rot: [70, 0, 0] }));
  body.add(drum);
  c.at.launcher = joint(0, 0.06, 0.18); drum.add(c.at.launcher);
  armR.rotation.set(-10 * D, 0, 10 * D);
  return finishHero(c, team);
}

export const MODELS = { hero_hackle: hackle, hero_chipper: chipper };

export const NOTES = {
  hero_hackle: "A boar brawler with a bristle crest in his side's colour, upturned tusks, war paint and a bone necklace; a hatchet in each hand, thrown from afar or swung up close.",
  hero_chipper: "A beaver lumberjack in a little wooden walker: a hard hat in his side's colour, big front teeth, a saw blade on the chest, a chain claw on the left arm, a blade launcher on the right shoulder and a paddle tail.",
};
