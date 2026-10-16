// The ten heroes in the round-2 style. Each keeps its round-1 silhouette
// cue (the blade, the hood and staff, the flame hair, the bow, the hook and
// belly, the horns and totem, the mask, the claw staff, the long rifle, the
// cloud crown) on a chibi body: a head about 40 % of the height, stubby
// limbs, dot eyes with a catch-light, blush.
import { THREE, C, mat, glow, shiny, light, veil, ball, orb, limb, cyl, cone, gem, hoop, group, joint, face, rig, halo } from "./kit.js";

const D = Math.PI / 180;
const TEAM = { sun: C.sun, moon: C.moon };

/** The shared chibi body. Returns { root, rig, at } where `at` holds named
 * anchors (Object3Ds) effects can read world positions from. */
export function chibi(o = {}) {
  const skin = o.skin ?? C.skin;
  const T = o.torso ?? [0.34, 0.32, 0.3];
  const hr = o.headR ?? 0.33;
  const legLen = o.legLen ?? 0.2;
  const hipY = legLen + 0.12;
  const root = new THREE.Group();
  const at = {};

  // legs: short capsules and round feet
  const leg = (s) => {
    const j = joint(s * T[0] * 0.42, hipY, 0);
    if (o.legs !== false) {
      j.add(limb([0, 0, 0], [0, -legLen, 0], o.legR ?? 0.085, o.legs ?? C.leather));
      j.add(ball(0.1, 0.07, 0.13, 0, -legLen - 0.05, 0.035, o.boots ?? o.legs ?? C.leather));
    }
    return j;
  };
  const legL = leg(-1), legR = leg(1);

  // body: the torso, and the head and arms on it
  const body = joint(0, hipY, 0);
  body.add(ball(T[0], T[1], T[2], 0, T[1] * 0.9, 0, o.torsoColor ?? C.cloth));
  if (o.belt) body.add(ball(T[0] * 1.02, 0.05, T[2] * 1.02, 0, T[1] * 0.38, 0, o.belt));
  const headY = T[1] * 1.72;
  const head = joint(0, headY, 0);
  if (o.head !== false) head.add(orb(hr, 0, hr * 0.95, 0, skin));
  if (o.face !== false) face(head, 0, hr * 0.95, 0, hr, o.faceOpts ?? {});
  at.headTop = joint(0, hr * 1.95, 0); head.add(at.headTop);
  at.face = joint(0, hr * 0.95, hr); head.add(at.face);
  body.add(head);

  const arm = (s) => {
    const j = joint(s * (T[0] + 0.02), T[1] * 1.42, 0.02);
    const sleeve = o.sleeve ?? o.torsoColor ?? C.cloth;
    j.add(limb([0, 0, 0], [0, -0.25, 0.02], o.armR ?? 0.07, sleeve));
    j.add(orb(o.handR ?? 0.085, 0, -0.3, 0.03, o.hands ?? skin));
    const hand = joint(0, -0.3, 0.03);
    j.add(hand);
    j.userData.hand = hand;
    return j;
  };
  const armL = arm(-1), armR = arm(1);
  body.add(armL, armR);
  at.handR = armR.userData.hand;
  at.handL = armL.userData.hand;
  at.chest = joint(0, T[1] * 1.0, T[2]); body.add(at.chest);
  at.feet = joint(0, 0.02, 0); root.add(at.feet);

  root.add(legL, legR, body);
  const r = { root, body, head, armL, armR, legL, legR };
  return { root, parts: r, at, hr, T, headY, hipY };
}

function finishHero(c, team, extra = {}) {
  const r = rig(c.root, c.parts);
  return { g: c.root, rig: r, at: c.at, team, ...extra };
}

// ------------------------------------------------------------------ 1
/** Ironvow (after Sven): a round little knight with a blade taller than he is. */
export function ironvow(team = "sun") {
  const c = chibi({ skin: C.skin2, torso: [0.4, 0.36, 0.36], torsoColor: C.steel, sleeve: C.steel, legs: C.iron, boots: C.steelDark, hands: C.steelDark, belt: C.leather, headR: 0.33 });
  const { head, armR, armL, body } = c.parts;
  const hr = c.hr;
  // the helm: a steel dome over the back of the head, a brow ridge, cheek guards
  head.add(ball(hr * 1.1, hr * 1.02, hr * 1.06, 0, hr * 1.08, -hr * 0.18, shiny(C.steel)));
  head.add(ball(hr * 0.98, hr * 0.16, hr * 0.5, 0, hr * 1.42, hr * 0.62, shiny(C.steelDark)));
  for (const s of [-1, 1]) head.add(ball(hr * 0.22, hr * 0.4, hr * 0.28, s * hr * 0.86, hr * 0.7, hr * 0.42, shiny(C.steel)));
  // the crest: a plume of team-coloured balls arcing front to back
  for (let i = 0; i < 6; i++) {
    const a = (-30 + i * 34) * D;
    head.add(orb(0.085 - i * 0.004, 0, hr * 1.1 + Math.cos(a) * hr * 0.95, -Math.sin(a) * hr * 0.95, TEAM[team]));
  }
  // pauldrons and a cape
  for (const s of [-1, 1]) body.add(ball(0.2, 0.16, 0.2, s * 0.4, 0.52, 0, shiny(C.steel)));
  for (const s of [-1, 1]) body.add(ball(0.21, 0.04, 0.21, s * 0.4, 0.44, 0, C.gold));
  body.add(ball(0.36, 0.42, 0.08, 0, 0.32, -0.32, TEAM[team]));
  body.add(orb(0.07, 0, 0.36, 0.35, C.gold));
  // the greatsword over his shoulder, its hilt in his right hand
  const sw = joint(0, -0.3, 0.03);
  sw.add(limb([0, -0.08, 0], [0, 0.12, 0], 0.035, C.leather));
  sw.add(limb([-0.17, 0.13, 0], [0.17, 0.13, 0], 0.04, C.gold));
  sw.add(orb(0.055, 0, -0.12, 0, C.gold));
  sw.add(limb([0, 0.2, 0], [0, 1.25, 0], 0.095, shiny(C.steel), { flat: 0.3 }));
  sw.add(limb([0, 0.2, 0], [0, 1.1, 0], 0.018, C.steelDark, { ink: false }));
  sw.rotation.set(-5 * D, 0, -28 * D);
  armR.add(sw);
  armR.rotation.set(-30 * D, 0, 12 * D);
  const tip = joint(0, 1.25, 0); sw.add(tip);
  c.at.blade = tip;
  c.at.sword = sw;
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 2
/** Rime (after Crystal Maiden): a frost hermit in a fur-rimmed hood with a crystal staff. */
export function rime(team = "sun") {
  const c = chibi({ skin: C.skin, torso: [0.36, 0.3, 0.32], torsoColor: 0xbfe4ff, sleeve: 0xe9f6ff, legs: false, headR: 0.34, faceOpts: { mouth: true } });
  const { head, armR, body, legL, legR } = c.parts;
  const hr = c.hr;
  // a bell robe to the ground in place of legs
  body.add(ball(0.44, 0.36, 0.42, 0, 0.02, 0, 0x8fc9f2));
  body.add(ball(0.46, 0.06, 0.44, 0, -0.26, 0, C.white));
  // the hood: a big soft dome behind the head and a fur rim round the face
  head.add(ball(hr * 1.18, hr * 1.14, hr * 1.12, 0, hr * 1.02, -hr * 0.22, 0x6fb3e8));
  head.add(hoop(hr * 0.98, hr * 0.2, 0, hr * 0.98, hr * 0.32, C.white, { rot: [0, 0, 0] }));
  head.add(cone(hr * 0.42, hr * 0.7, 0, hr * 1.85, -hr * 0.55, 0x6fb3e8, { rot: [-60, 0, 0] }));
  head.add(orb(hr * 0.16, 0, hr * 1.98, -hr * 1.05, C.white));
  // silver hair peeking out
  for (const s of [-1, 1]) head.add(ball(hr * 0.2, hr * 0.32, hr * 0.18, s * hr * 0.62, hr * 0.62, hr * 0.42, 0xe8f0ff));
  // the staff: a long pole with a glowing crystal at the top
  const st = joint(0, -0.3, 0.03);
  st.add(limb([0, -0.55, 0], [0, 0.9, 0], 0.03, C.wood));
  st.add(hoop(0.09, 0.025, 0, 0.92, 0, C.white));
  st.add(gem(0.1, 0.17, 0.1, 0, 1.08, 0, glow(C.frost, 0.9)));
  st.add(halo(C.frost, 0.6, 0.5));
  st.children[st.children.length - 1].position.set(0, 1.08, 0);
  armR.add(st);
  armR.rotation.set(-25 * D, 0, -8 * D);
  const tip = joint(0, 1.08, 0); st.add(tip);
  c.at.tip = tip;
  c.root.remove(legL, legR);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 3
/** Cinder (after Lina): a fire dancer with flame hair and burning hands. */
export function cinder(team = "sun") {
  const c = chibi({ skin: C.skin2, torso: [0.3, 0.3, 0.28], torsoColor: 0xe0434a, sleeve: C.skin2, legs: 0xb8323a, boots: C.gold, belt: C.gold, headR: 0.33, faceOpts: { mouth: true }, hands: glow(C.flame, 1.2) });
  const { head, body, armL, armR } = c.parts;
  const hr = c.hr;
  // a flared skirt
  body.add(ball(0.38, 0.2, 0.36, 0, 0.06, 0, 0xe0434a));
  body.add(ball(0.39, 0.04, 0.37, 0, -0.06, 0, C.gold));
  // the hair: a cap of red with flames rising off the back and top
  head.add(ball(hr * 1.06, hr * 0.9, hr * 1.04, 0, hr * 1.2, -hr * 0.12, 0xd8392c));
  // [x, y, z] on the cap in head radii, base radius, height, lean back, lean out
  const flames = [
    [0, 1.85, -0.2, 0.42, 1.9, -35, 0], [-0.5, 1.65, -0.3, 0.32, 1.5, -45, 30], [0.5, 1.65, -0.3, 0.32, 1.5, -45, -30],
    [-0.3, 1.45, -0.75, 0.34, 1.7, -75, 12], [0.3, 1.45, -0.75, 0.34, 1.7, -75, -12], [0, 1.1, -1.0, 0.32, 1.6, -105, 0],
    [-0.75, 1.2, -0.45, 0.24, 1.1, -60, 55], [0.75, 1.2, -0.45, 0.24, 1.1, -60, -55],
  ];
  for (const [x, y, z, r, h, lx, lz] of flames) {
    head.add(cone(hr * r, hr * h, x * hr, y * hr, z * hr, glow(C.fire, 0.45), { rot: [lx, 0, lz] }));
    head.add(cone(hr * r * 0.55, hr * h * 0.72, x * hr, y * hr + 0.01, z * hr, glow(C.flame, 0.7), { ink: false, rot: [lx, 0, lz] }));
  }
  head.add(halo(C.fire, 1.0, 0.35));
  head.children[head.children.length - 1].position.set(0, hr * 1.8, -hr * 0.3);
  // fire on both hands
  for (const a of [armL, armR]) {
    const h = halo(C.fire, 0.45, 0.8);
    h.position.set(0, -0.3, 0.03);
    a.add(h);
  }
  // a trailing sash
  body.add(limb([0.2, 0.36, -0.2], [0.42, 0.05, -0.42], 0.05, C.gold, { flat: 0.35 }));
  armR.rotation.set(-15 * D, 0, 12 * D);
  armL.rotation.set(-15 * D, 0, -12 * D);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 4
/** Thistle (after Drow Ranger): a hooded archer with a tall bow and frost-tipped arrows. */
export function thistle(team = "sun") {
  const c = chibi({ skin: 0xd9d6f2, torso: [0.3, 0.3, 0.28], torsoColor: 0x4f9e7a, sleeve: 0x3f7f63, legs: 0x5a4636, boots: 0x3d2f26, belt: C.leather, headR: 0.33 });
  const { head, body, armL, armR } = c.parts;
  const hr = c.hr;
  // the hood, peaked and falling back like a leaf
  head.add(ball(hr * 1.12, hr * 1.08, hr * 1.1, 0, hr * 1.04, -hr * 0.2, 0x3f8f6a));
  head.add(cone(hr * 0.5, hr * 1.0, 0, hr * 1.6, -hr * 0.55, 0x3f8f6a, { rot: [-70, 0, 0] }));
  for (const s of [-1, 1]) head.add(ball(hr * 0.24, hr * 0.42, hr * 0.2, s * hr * 0.66, hr * 0.68, hr * 0.42, 0xeef2ff));
  // a short cape
  body.add(ball(0.32, 0.36, 0.08, 0, 0.32, -0.28, 0x2f6f52));
  // the quiver on her back, frost tips showing
  body.add(cyl(0.08, 0.42, -0.12, 0.6, -0.34, C.leather, { rot: [20, 0, 25] }));
  for (let i = 0; i < 3; i++) body.add(gem(0.035, 0.07, 0.035, -0.2 - i * 0.03, 0.86 + i * 0.02, -0.42 + i * 0.03, glow(C.frost, 1.1)));
  // the bow in her left hand: a tall arc and its string
  const bow = joint(0, -0.3, 0.06);
  const R = 0.5;
  const arc = hoop(R, 0.032, 0, 0, -R, C.wood, { arc: 140 });
  arc.rotation.set(0, 90 * D, 110 * D);
  bow.add(arc);
  bow.add(limb([0, 0.94 * R, -0.66 * R], [0, -0.94 * R, -0.66 * R], 0.007, C.white, { ink: false }));
  bow.add(orb(0.045, 0, 0, 0, C.leather));
  armL.add(bow);
  armL.rotation.set(-55 * D, 0, -12 * D);
  c.at.bow = bow;
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 5
/** Gristle (after Pudge): a stitched round butcher with a hook on a chain and a cleaver. */
export function gristle(team = "sun") {
  const c = chibi({ skin: 0xd8e0c2, torso: [0.56, 0.48, 0.5], torsoColor: 0xd8e0c2, sleeve: 0xd8e0c2, legs: 0x6b5b4b, boots: 0x4a3d33, headR: 0.26, legLen: 0.16, legR: 0.12, armR: 0.11, handR: 0.12, faceOpts: { size: 0.17, sep: 0.4 } });
  const { head, body, armL, armR } = c.parts;
  const hr = c.hr;
  // lift the head and arms onto the big belly
  head.position.y = 0.78; armL.position.set(-0.56, 0.62, 0.02); armR.position.set(0.56, 0.62, 0.02);
  // apron and stitches
  body.add(ball(0.42, 0.36, 0.2, 0, 0.32, 0.34, 0xece3d0));
  // a seam down his left flank, crossed by stitches
  body.add(limb([0.5, 0.25, 0.12], [0.46, 0.75, 0.16], 0.011, C.ink, { ink: false }));
  for (let i = 0; i < 4; i++) body.add(limb([0.45, 0.33 + i * 0.12, 0.2], [0.53, 0.35 + i * 0.12, 0.08], 0.011, C.ink, { ink: false }));
  body.add(ball(0.12, 0.08, 0.1, -0.34, 0.66, 0.34, 0xc2cba9));
  // a patch of stitching on the head, and a little cook's band
  head.add(limb([-hr * 0.5, hr * 1.55, hr * 0.5], [hr * 0.2, hr * 1.7, hr * 0.62], 0.01, C.ink, { ink: false }));
  head.add(cyl(hr * 0.86, hr * 0.3, 0, hr * 1.62, 0, 0xece3d0));
  // the hook on a chain in his right hand
  const hk = joint(0, -0.3, 0.03);
  for (let i = 0; i < 4; i++) hk.add(hoop(0.04, 0.012, 0, -0.06 - i * 0.07, 0, C.iron, { rot: [0, i % 2 ? 90 : 0, 0] }));
  hk.add(hoop(0.2, 0.04, 0, -0.52, 0, shiny(C.steel), { arc: 230, rot: [0, 90, 190] }));
  hk.add(cone(0.055, 0.16, 0, -0.42, 0.2, shiny(C.steel), { rot: [50, 0, 0] }));
  armR.add(hk);
  c.at.hook = joint(0, -0.5, 0.03); hk.add(c.at.hook);
  // the cleaver in his left
  const cl = joint(0, -0.3, 0.03);
  cl.add(limb([0, 0, 0], [0, 0, 0.18], 0.025, C.woodDark));
  cl.add(ball(0.025, 0.12, 0.13, 0, -0.06, 0.32, shiny(C.steel)));
  armL.add(cl);
  armL.rotation.set(-20 * D, 0, -8 * D);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 6
/** Tectus (after Earthshaker): a stone ox-calf with curled horns and a totem club. */
export function tectus(team = "sun") {
  const c = chibi({ skin: 0x9a8fa8, torso: [0.42, 0.38, 0.36], torsoColor: 0x8f86a0, sleeve: 0x8f86a0, legs: 0x7a7088, boots: 0x5e566b, headR: 0.32, legR: 0.1, armR: 0.09, handR: 0.11, faceOpts: { sep: 0.4, h: 0.12 } });
  const { head, body, armR } = c.parts;
  const hr = c.hr;
  // stone plates and moss
  body.add(ball(0.3, 0.12, 0.26, 0.05, 0.72, -0.08, C.moss));
  body.add(ball(0.18, 0.14, 0.12, -0.2, 0.4, 0.3, 0xa79dbb));
  body.add(ball(0.12, 0.1, 0.1, 0.22, 0.25, 0.3, 0xa79dbb));
  // an ox face: a wide muzzle, nostrils, a gold ring
  head.add(ball(hr * 0.62, hr * 0.42, hr * 0.42, 0, hr * 0.55, hr * 0.78, 0xc9b9a4));
  for (const s of [-1, 1]) head.add(orb(hr * 0.07, s * hr * 0.2, hr * 0.6, hr * 1.18, C.eye, { ink: false }));
  head.add(hoop(hr * 0.16, hr * 0.03, 0, hr * 0.36, hr * 1.14, shiny(C.gold), { rot: [80, 0, 0] }));
  // curled horns and little ears
  for (const s of [-1, 1]) {
    const h = hoop(hr * 0.42, hr * 0.12, s * hr * 0.9, hr * 1.42, -hr * 0.05, C.bone, { arc: 150 });
    h.rotation.set(0, 0, s > 0 ? -20 * D : 200 * D);
    head.add(h);
    head.add(ball(hr * 0.24, hr * 0.12, hr * 0.14, s * hr * 1.0, hr * 1.02, 0, 0x8f86a0));
  }
  head.add(ball(hr * 0.5, hr * 0.15, hr * 0.4, 0, hr * 1.75, -hr * 0.1, C.moss));
  // the totem club: a log with a stone head and a glowing band
  const tc = joint(0, -0.3, 0.03);
  tc.add(limb([0, -0.08, 0], [0, 0.65, 0], 0.05, C.wood));
  tc.add(ball(0.17, 0.24, 0.17, 0, 0.82, 0, C.stone));
  tc.add(hoop(0.17, 0.03, 0, 0.82, 0, glow(0x7fe3c2, 1.2), { rot: [90, 0, 0] }));
  tc.rotation.set(20 * D, 0, 10 * D);
  armR.add(tc);
  armR.rotation.set(-30 * D, 0, 0);
  c.at.club = joint(0, 0.82, 0); tc.add(c.at.club);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 7
/** Kestrel (after Juggernaut): a masked bladedancer with a tall topknot and a curved sword. */
export function kestrel(team = "sun") {
  const c = chibi({ skin: C.skin3, torso: [0.32, 0.3, 0.28], torsoColor: 0xd9783a, sleeve: C.skin3, legs: 0xf0dcc0, boots: C.leather, belt: 0x8a2f2f, headR: 0.33, face: false, legR: 0.1 });
  const { head, body, armR, legL, legR } = c.parts;
  const hr = c.hr;
  // baggy trousers
  for (const l of [legL, legR]) l.add(ball(0.13, 0.12, 0.13, 0, -0.06, 0, 0xf0dcc0));
  // the mask: a round white face with slit eyes and red stripes
  head.add(ball(hr * 0.94, hr * 1.0, hr * 0.5, 0, hr * 0.92, hr * 0.62, C.bone));
  for (const s of [-1, 1]) {
    head.add(ball(hr * 0.16, hr * 0.06, hr * 0.05, s * hr * 0.36, hr * 1.02, hr * 1.1, C.eye, { ink: false }));
    head.add(limb([s * hr * 0.62, hr * 1.4, hr * 0.94], [s * hr * 0.66, hr * 0.5, hr * 0.94], hr * 0.05, 0xd2463c, { ink: false }));
  }
  head.add(limb([0, hr * 1.6, hr * 1.02], [0, hr * 1.2, hr * 1.1], hr * 0.05, 0xd2463c, { ink: false }));
  // the topknot: a tall plume of orange puffs
  for (let i = 0; i < 5; i++) head.add(orb(hr * (0.3 - i * 0.035), 0, hr * (1.85 + i * 0.28), -hr * (0.1 + i * 0.12), i % 2 ? 0xf28a3c : 0xe86a2c));
  head.add(cyl(hr * 0.2, hr * 0.12, 0, hr * 1.82, -hr * 0.08, 0x8a2f2f));
  // a sash over one shoulder
  body.add(limb([-0.3, 0.62, 0.2], [0.3, 0.12, 0.22], 0.06, 0x8a2f2f, { flat: 0.5 }));
  // the curved sword
  const sw = joint(0, -0.3, 0.03);
  sw.add(limb([0, -0.05, 0], [0, 0.14, 0], 0.03, 0x8a2f2f));
  sw.add(ball(0.09, 0.025, 0.09, 0, 0.15, 0, C.gold));
  const blade = hoop(1.1, 0.04, -1.1, 0.16, 0, shiny(C.steel), { arc: 42 });
  blade.scale.z = 0.35;
  sw.add(blade);
  sw.rotation.set(-20 * D, 0, -35 * D);
  armR.add(sw);
  armR.rotation.set(-35 * D, 0, 18 * D);
  c.at.blade = joint(0.1, 0.9, 0); sw.add(c.at.blade);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 8
/** Hexwright (after Lion): a slight warlock in a horned hood with a clawed staff. */
export function hexwright(team = "sun") {
  const c = chibi({ skin: 0xb9a6c9, torso: [0.28, 0.34, 0.26], torsoColor: 0x5a3f86, sleeve: 0x4a3270, legs: false, headR: 0.32, faceOpts: { glowEye: true, eye: 0xff7ad9, blush: false, size: 0.13, tall: 0.7 } });
  const { head, body, armR, armL, legL, legR } = c.parts;
  const hr = c.hr;
  c.root.remove(legL, legR);
  // a long robe with a ragged hem
  body.add(cone(0.36, 0.62, 0, -0.32, 0, 0x5a3f86));
  for (let i = 0; i < 7; i++) {
    const a = (i / 7) * Math.PI * 2;
    const f = cone(0.07, 0.16, Math.cos(a) * 0.3, -0.42, Math.sin(a) * 0.3, 0x4a3270, { rot: [180, 0, 0] });
    body.add(f);
  }
  body.add(ball(0.06, 0.3, 0.04, 0, 0.3, 0.26, C.lilac));
  // a hood with two swept horns
  head.add(ball(hr * 1.14, hr * 1.12, hr * 1.1, 0, hr * 1.02, -hr * 0.2, 0x3e2a5e));
  for (const s of [-1, 1]) {
    const h = hoop(hr * 0.75, hr * 0.13, s * hr * 0.6, hr * 1.55, -hr * 0.6, C.bone, { arc: 95 });
    h.rotation.set(0, s * 90 * D, s > 0 ? 20 * D : 160 * D);
    head.add(h);
  }
  // the clawed staff: three bone claws round a pink light
  const st = joint(0, -0.3, 0.03);
  st.add(limb([0, -0.5, 0], [0, 0.85, 0], 0.03, 0x4a3a32));
  for (let i = 0; i < 3; i++) {
    const a = (i / 3) * Math.PI * 2;
    st.add(limb([Math.cos(a) * 0.04, 0.85, Math.sin(a) * 0.04], [Math.cos(a) * 0.14, 1.08, Math.sin(a) * 0.14], 0.022, C.bone));
  }
  st.add(orb(0.08, 0, 1.0, 0, glow(0xff7ad9, 1.6), { ink: false }));
  const hh = halo(0xff7ad9, 0.6, 0.6); hh.position.set(0, 1.0, 0); st.add(hh);
  armR.add(st);
  armR.rotation.set(-25 * D, 0, 0);
  armL.rotation.set(-10 * D, 0, -15 * D);
  c.at.tip = joint(0, 1.0, 0); st.add(c.at.tip);
  // he floats a hand's breadth off the ground
  c.root.position.y = 0;
  for (const k of [body]) k.position.y += 0.12;
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 9
/** Longwick (after Sniper): a gnome with a big nose, goggles, a beard and a rifle twice his height. */
export function longwick(team = "sun") {
  const c = chibi({ skin: C.skin2, torso: [0.3, 0.27, 0.27], torsoColor: 0x7a8f4e, sleeve: 0x7a8f4e, legs: 0x5a4636, boots: C.leather, belt: C.leather, headR: 0.32, legLen: 0.14 });
  const { head, body, armR, armL } = c.parts;
  const hr = c.hr;
  // a big round nose, a white beard, bushy brows
  head.add(orb(hr * 0.26, 0, hr * 0.78, hr * 1.02, 0xf0a985));
  for (let i = 0; i < 7; i++) {
    const a = (-60 + i * 20) * D;
    head.add(orb(hr * 0.24, Math.sin(a) * hr * 0.7, hr * 0.42 - Math.cos(a) * hr * 0.18, Math.cos(a) * hr * 0.72, C.white));
  }
  head.add(orb(hr * 0.26, 0, hr * 0.18, hr * 0.78, C.white));
  // a peaked cap and goggles pushed up on it
  head.add(ball(hr * 1.04, hr * 0.62, hr * 1.04, 0, hr * 1.38, -hr * 0.04, 0x8a6a3a));
  head.add(ball(hr * 0.8, hr * 0.08, hr * 0.55, 0, hr * 1.18, hr * 0.62, 0x6e5230));
  for (const s of [-1, 1]) {
    head.add(hoop(hr * 0.2, hr * 0.06, s * hr * 0.26, hr * 1.58, hr * 0.78, shiny(C.gold), { rot: [-20, 0, 0] }));
    head.add(orb(hr * 0.17, s * hr * 0.26, hr * 1.58, hr * 0.76, veil(C.spark, 0.7), { ink: false }));
  }
  head.add(hoop(hr * 1.0, hr * 0.05, 0, hr * 1.5, 0, C.leather, { rot: [90, 0, 0] }));
  // the rifle: a long barrel, a scope, a wooden stock, held across
  const rf = joint(0, -0.3, 0.03);
  rf.add(limb([0, 0, -0.2], [0, 0, 1.25], 0.035, C.iron));
  rf.add(ball(0.06, 0.09, 0.32, 0, -0.03, -0.32, C.wood));
  rf.add(cyl(0.045, 0.32, 0, 0.09, 0.2, C.iron, { rot: [90, 0, 0] }));
  rf.add(orb(0.05, 0, 0.09, 0.37, veil(C.spark, 0.8), { ink: false }));
  rf.add(cyl(0.05, 0.06, 0, 0, 1.26, C.iron, { rot: [90, 0, 0] }));
  rf.rotation.set(-30 * D, 35 * D, 0);
  armR.add(rf);
  armR.rotation.set(-25 * D, 0, 15 * D);
  armL.rotation.set(-45 * D, 0, -20 * D);
  c.at.muzzle = joint(0, 0, 1.3); rf.add(c.at.muzzle);
  c.root.scale.setScalar(0.86);
  return finishHero(c, team);
}

// ------------------------------------------------------------------ 10
/** Tempest (after Zeus): an old storm caller with a huge beard and a crown of cloud. */
export function tempest(team = "sun") {
  const c = chibi({ skin: C.skin, torso: [0.34, 0.32, 0.3], torsoColor: 0x4f7fd6, sleeve: 0x4f7fd6, legs: false, headR: 0.33, faceOpts: { h: 0.1 }, hands: C.skin });
  const { head, body, armR, armL, legL, legR } = c.parts;
  const hr = c.hr;
  c.root.remove(legL, legR);
  body.add(ball(0.42, 0.36, 0.4, 0, 0.0, 0, 0x3f68b8));
  body.add(ball(0.44, 0.06, 0.42, 0, -0.28, 0, C.gold));
  // the beard, falling over his chest
  for (let i = 0; i < 9; i++) {
    const a = (-70 + i * 17.5) * D;
    head.add(orb(hr * 0.27, Math.sin(a) * hr * 0.72, hr * 0.4 - Math.cos(a) * hr * 0.2, Math.cos(a) * hr * 0.66, C.white));
  }
  for (let i = 0; i < 4; i++) head.add(orb(hr * (0.3 - i * 0.04), 0, hr * (0.05 - i * 0.28), hr * (0.82 - i * 0.04), C.white));
  for (const s of [-1, 1]) head.add(ball(hr * 0.22, hr * 0.1, hr * 0.12, s * hr * 0.38, hr * 1.25, hr * 0.84, C.white));
  // the cloud crown: puffs in a ring over his head, a spark in the middle
  const crown = joint(0, hr * 2.1, 0);
  for (let i = 0; i < 7; i++) {
    const a = (i / 7) * Math.PI * 2;
    crown.add(orb(hr * (0.24 + (i % 2) * 0.06), Math.cos(a) * hr * 0.62, (i % 2) * hr * 0.06, Math.sin(a) * hr * 0.62, 0xf2f6ff));
  }
  crown.add(gem(hr * 0.12, hr * 0.26, hr * 0.12, 0, hr * 0.2, 0, glow(C.bolt, 1.6)));
  head.add(crown);
  c.at.crown = crown;
  // crackling hands
  for (const a of [armL, armR]) {
    const h = halo(C.spark, 0.4, 0.75);
    h.position.set(0, -0.3, 0.03);
    a.add(h);
  }
  armR.rotation.set(-20 * D, 0, 10 * D);
  armL.rotation.set(-20 * D, 0, -10 * D);
  return finishHero(c, team);
}

export const HEROES = {
  hero_ironvow: ironvow, hero_rime: rime, hero_cinder: cinder, hero_thistle: thistle, hero_gristle: gristle,
  hero_tectus: tectus, hero_kestrel: kestrel, hero_hexwright: hexwright, hero_longwick: longwick, hero_tempest: tempest,
};

export const NOTES = {
  hero_ironvow: "A round little knight in a steel dome helm with a crest in his side's colour, big pauldrons and a cape; the greatsword over his shoulder is taller than he is.",
  hero_rime: "A frost hermit in a bell-shaped robe and a deep hood rimmed with white fur, silver hair at her cheeks; the crystal on her staff glows.",
  hero_cinder: "A fire dancer in a red dress and gold sash, flames rising off the back of her red hair, both hands alight.",
  hero_thistle: "A hooded archer with a peaked leaf hood and pale skin; the tall bow in her left hand and the frost-tipped arrows in her quiver are her outline.",
  hero_gristle: "A huge round butcher, stitched down one side, in an apron and a cook's band, with a hook on a chain in one hand and a cleaver in the other.",
  hero_tectus: "A stone ox-calf with a wide muzzle and a gold nose ring, curled horns and moss on his back, carrying a totem club with a glowing band.",
  hero_kestrel: "A bladedancer behind a round white mask with slit eyes and red stripes, a tall orange topknot, baggy trousers and a curved sword.",
  hero_hexwright: "A slight warlock in a long ragged violet robe and a horned hood, with glowing pink eyes and a staff of three bone claws round a pink light.",
  hero_longwick: "A gnome with a big round nose, a white beard, goggles on his cap and a long rifle nearly twice his height.",
  hero_tempest: "An old storm caller with a beard down his chest, bushy brows and a crown of cloud puffs round a spark, his hands crackling blue.",
};
