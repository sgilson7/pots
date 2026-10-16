// Every model that is not a hero, in the round-2 style: lane creeps,
// buildings, the forest's neutrals, the Warden, wards, trees and runes.
// Same rules as heroes.js: spheres, capsules, cones, drums and rings with
// the plum ink; faces with dot eyes, a catch-light and blush; no boxes.
// Sunward is gold, cream and sand; Moonward violet, lilac and slate.
import { THREE, C, mat, glow, shiny, light, veil, ball, orb, limb, cyl, cone, gem, hoop, group, joint, face, rig, halo } from "./kit.js";
import { chibi } from "./heroes.js";

const D = Math.PI / 180;

/** Named anchors on a model: Object3Ds at fixed spots, so effects can read
 * their world positions. Every model gets the same five names. */
function anchors(root, pts) {
  const at = {};
  const all = { feet: [0, 0.02, 0], chest: [0, 0.5, 0.2], headTop: [0, 1, 0], handR: [0.3, 0.5, 0.2], top: [0, 1, 0], ...pts };
  for (const [k, p] of Object.entries(all)) {
    if (p && p.isObject3D) { at[k] = p; continue; }
    at[k] = joint(...p);
    root.add(at[k]);
  }
  return at;
}
/** A still model (no joints that move) with anchors. */
function still(g, pts, update) {
  const at = anchors(g, pts);
  const out = { g, at, rig: rig(g) };
  if (update) out.update = update;
  return out;
}

// ------------------------------------------------------------ lane creeps
const SUN = { tunic: 0xfff1d6, trim: C.sun, deep: C.sunDeep, skin: C.skin };
const MOON = { skin: 0xb7a4f2, robe: C.moon, deep: C.moonDeep, slate: C.slate };

function creepBody(o) {
  const c = chibi({ legLen: 0.16, headR: 0.32, torso: [0.3, 0.27, 0.27], ...o });
  c.root.scale.setScalar(o.scale ?? 0.72);
  return c;
}
function finishCreep(c, extra = {}) {
  const r = rig(c.root, c.parts);
  return { g: c.root, rig: r, at: c.at, ...extra };
}

/** Sunward spearman: a round gold helm, a round shield with a sun boss, a spear. */
function creepMeleeSun() {
  const c = creepBody({ skin: SUN.skin, torsoColor: SUN.tunic, sleeve: SUN.tunic, legs: 0xc89a66, boots: C.leather, belt: SUN.trim });
  const { head, armL, armR, body } = c.parts;
  const hr = c.hr;
  head.add(ball(hr * 1.06, hr * 0.78, hr * 1.04, 0, hr * 1.28, -hr * 0.06, shiny(C.gold)));
  head.add(ball(hr * 1.12, hr * 0.12, hr * 1.1, 0, hr * 1.02, 0, shiny(C.sunDeep)));
  head.add(orb(hr * 0.16, 0, hr * 2.05, 0, C.sunDeep));
  body.add(ball(0.31, 0.06, 0.28, 0, 0.46, 0, C.sunDeep));
  // the round shield on the left arm, facing out
  const sh = joint(-0.06, -0.2, 0.08);
  sh.add(cyl(0.26, 0.05, 0, 0, 0, shiny(C.gold), { rot: [0, 0, 90] }));
  sh.add(cyl(0.12, 0.06, -0.02, 0, 0, glow(C.sun, 0.35), { rot: [0, 0, 90] }));
  sh.add(hoop(0.25, 0.025, -0.03, 0, 0, C.sunDeep, { rot: [0, 90, 0] }));
  sh.rotation.y = -20 * D;
  armL.add(sh);
  // the spear, upright in the right hand
  const sp = joint(0, -0.3, 0.03);
  sp.add(limb([0, -0.35, 0], [0, 0.95, 0], 0.028, C.wood));
  sp.add(cone(0.065, 0.24, 0, 0.95, 0, shiny(C.steel)));
  sp.add(ball(0.06, 0.03, 0.06, 0, 0.94, 0, C.sun));
  armR.add(sp);
  armR.rotation.set(-25 * D, 0, 8 * D);
  return finishCreep(c);
}

/** Sunward slinger: a tall pointed orange cap, a sling and a pouch of stones. */
function creepRangedSun() {
  const c = creepBody({ skin: C.skin2, torsoColor: 0xf6dfae, sleeve: 0xf6dfae, legs: 0xc89a66, boots: C.leather, belt: C.leather });
  const { head, armR, body } = c.parts;
  const hr = c.hr;
  head.add(ball(hr * 1.04, hr * 0.42, hr * 1.04, 0, hr * 1.38, 0, C.sunDeep));
  head.add(cone(hr * 0.86, hr * 1.5, 0, hr * 1.45, -hr * 0.1, C.sun, { rot: [-14, 0, 0] }));
  head.add(orb(hr * 0.15, 0, hr * 2.92, -hr * 0.45, C.sunCream));
  body.add(ball(0.12, 0.13, 0.1, -0.28, 0.25, 0.12, C.leather));
  // the sling: a cord swinging from the hand with a stone in its cup
  const sl = joint(0, -0.3, 0.03);
  sl.add(limb([0, 0, 0], [0.05, -0.32, 0.12], 0.012, C.leather, { ink: false }));
  sl.add(ball(0.06, 0.05, 0.06, 0.05, -0.34, 0.12, C.leather));
  sl.add(orb(0.04, 0.05, -0.31, 0.12, C.stone));
  armR.add(sl);
  armR.rotation.set(-60 * D, 0, 10 * D);
  c.at.stone = joint(0.05, -0.31, 0.12); sl.add(c.at.stone);
  return finishCreep(c);
}

/** A wheel: a drum on its side with a hub, at (x, y, z). */
function wheel(r, x, y, z, rim, hub) {
  return group(cyl(r, 0.1, x, y, z, rim, { rot: [0, 0, 90] }), cyl(r * 0.35, 0.14, x, y, z, hub, { rot: [0, 0, 90] }));
}
/** A siege engine on four wheels with a face on its front. */
function cart(colBody, colWheel, hubCol) {
  const g = new THREE.Group();
  const body = joint(0, 0, 0);
  body.add(ball(0.48, 0.26, 0.62, 0, 0.42, 0, colBody));
  for (const x of [-0.5, 0.5]) for (const z of [-0.38, 0.38]) body.add(wheel(0.2, x, 0.2, z, colWheel, hubCol));
  face(body, 0, 0.42, 0.18, 0.44, { sep: 0.3, size: 0.13 });
  g.add(body);
  return { g, body };
}

/** Sunward ballista: a round wooden cart with a gold bow and a bolt laid on top. */
function creepSiegeSun() {
  const { g, body } = cart(C.wood, C.woodDark, C.gold);
  body.add(ball(0.5, 0.06, 0.64, 0, 0.6, 0, C.sun));
  const bow = hoop(0.5, 0.04, 0, 0.82, 0.12, shiny(C.gold), { arc: 140 });
  bow.rotation.set(-90 * D, 0, 20 * D);
  body.add(bow);
  body.add(limb([0, 0.8, -0.45], [0, 0.8, 0.62], 0.035, C.wood));
  body.add(cone(0.07, 0.18, 0, 0.8, 0.66, shiny(C.steel), { rot: [90, 0, 0] }));
  body.add(limb([-0.46, 0.8, 0.32], [0, 0.8, -0.3], 0.01, C.white, { ink: false }));
  body.add(limb([0.46, 0.8, 0.32], [0, 0.8, -0.3], 0.01, C.white, { ink: false }));
  body.add(orb(0.1, 0, 0.92, -0.4, C.sun));
  const at = anchors(g, { chest: [0, 0.55, 0.4], headTop: [0, 1.0, 0], handR: [0, 0.8, 0.8], top: [0, 1.0, 0] });
  return { g, at, rig: rig(g, { body }) };
}

/** Moonward thrall: a little lilac brute with horn nubs and bone claws. */
function creepMeleeMoon() {
  const c = creepBody({ skin: MOON.skin, torsoColor: 0x8d79d6, sleeve: MOON.skin, legs: 0x6a5aa8, boots: MOON.deep, hands: MOON.skin, belt: C.bone, armR: 0.08, handR: 0.1, faceOpts: { sep: 0.38 } });
  const { head, armL, armR, body } = c.parts;
  const hr = c.hr;
  for (const s of [-1, 1]) head.add(cone(hr * 0.2, hr * 0.42, s * hr * 0.5, hr * 1.62, -hr * 0.05, C.bone, { rot: [0, 0, -s * 25] }));
  head.add(ball(hr * 0.9, hr * 0.3, hr * 0.8, 0, hr * 1.5, -hr * 0.25, MOON.deep));
  body.add(ball(0.24, 0.1, 0.12, 0, 0.62, -0.22, C.bone));
  // three bone claws on each hand
  for (const a of [armL, armR]) {
    for (let i = -1; i <= 1; i++) a.add(cone(0.028, 0.16, i * 0.05, -0.36, 0.08, C.bone, { rot: [150, 0, i * 12] }));
  }
  armL.rotation.set(-30 * D, 0, -12 * D);
  armR.rotation.set(-30 * D, 0, 12 * D);
  body.rotation.x = 8 * D;
  return finishCreep(c);
}

/** Moonward caster: a hooded robe with a violet spark held up in one hand. */
function creepRangedMoon() {
  const c = creepBody({ skin: 0xd9cdf6, torsoColor: MOON.robe, sleeve: MOON.robe, legs: false, faceOpts: { glowEye: false } });
  const { head, armR, body, legL, legR } = c.parts;
  const hr = c.hr;
  c.root.remove(legL, legR);
  body.add(cone(0.36, 0.5, 0, -0.3, 0, MOON.deep));
  body.add(ball(0.37, 0.05, 0.37, 0, -0.27, 0, C.lilac));
  head.add(ball(hr * 1.12, hr * 1.1, hr * 1.1, 0, hr * 1.02, -hr * 0.2, MOON.deep));
  head.add(cone(hr * 0.5, hr * 0.9, 0, hr * 1.6, -hr * 0.5, MOON.deep, { rot: [-55, 0, 0] }));
  head.add(hoop(hr * 0.95, hr * 0.1, 0, hr * 0.97, hr * 0.3, C.lilac));
  const sp = joint(0, -0.34, 0.04);
  sp.add(orb(0.09, 0, 0, 0, glow(C.arcane, 1.6), { ink: false }));
  const h = halo(C.arcane, 0.55, 0.8); sp.add(h);
  for (let i = 0; i < 4; i++) sp.add(gem(0.025, 0.05, 0.025, Math.cos(i * 1.57) * 0.15, Math.sin(i * 1.57) * 0.15, 0, glow(C.lilac, 1.2), { ink: false }));
  armR.add(sp);
  armR.rotation.set(-130 * D, 0, 10 * D);
  c.at.spark = sp;
  c.at.handR = sp;
  return finishCreep(c, { update: (t) => { sp.rotation.z = t * 2; } });
}

/** Moonward catapult: a slate cart with a bone arm and a rock in its cup. */
function creepSiegeMoon() {
  const { g, body } = cart(C.slate, 0x4a5370, C.bone);
  body.add(ball(0.5, 0.06, 0.64, 0, 0.6, 0, C.moon));
  for (const s of [-1, 1]) body.add(cone(0.06, 0.2, s * 0.32, 0.6, -0.45, C.bone, { rot: [-30, 0, 0] }));
  const arm = joint(0, 0.68, -0.2);
  arm.add(limb([0, 0, 0], [0, 0.55, -0.35], 0.05, C.bone));
  arm.add(ball(0.15, 0.08, 0.15, 0, 0.6, -0.4, C.bone));
  arm.add(orb(0.13, 0, 0.7, -0.4, 0x5d6b8c));
  body.add(arm);
  body.add(cyl(0.12, 0.2, 0, 0.7, -0.2, C.moonDeep, { rot: [0, 0, 90] }));
  const at = anchors(g, { chest: [0, 0.55, 0.4], headTop: [0, 1.4, -0.4], handR: [0, 1.38, -0.6], top: [0, 1.4, -0.4] });
  return { g, at, rig: rig(g, { body, armR: arm }) };
}

// ------------------------------------------------------------ buildings
/** A rounded stone course: a fat ring round a drum. */
const course = (r, y, col) => hoop(r, 0.08, 0, y, 0, col, { rot: [90, 0, 0] });

/** A tower: a tapering drum spire in stacked courses, a dome, and a glowing
 * jewel on top. */
function tower(side) {
  const sun = side === "sun";
  const stone = sun ? C.sand : 0x8a93ae, dark = sun ? 0xd9b77a : 0x6a7594, cap = sun ? C.sunDeep : C.moonDeep;
  const g = new THREE.Group();
  g.add(cyl([0.95, 1.15], 0.5, 0, 0.25, 0, dark));
  g.add(cyl([0.72, 0.9], 1.4, 0, 1.2, 0, stone));
  g.add(course(0.9, 0.52, dark));
  g.add(course(0.8, 1.2, dark));
  g.add(cyl([0.62, 0.7], 1.0, 0, 2.4, 0, stone));
  g.add(course(0.72, 1.92, dark));
  // a balcony of merlons
  g.add(cyl(0.92, 0.18, 0, 2.96, 0, dark));
  for (let i = 0; i < 10; i++) {
    const a = (i / 10) * Math.PI * 2;
    g.add(ball(0.13, 0.16, 0.13, Math.cos(a) * 0.82, 3.12, Math.sin(a) * 0.82, stone));
  }
  // windows like sleepy eyes, and a door
  for (const s of [-1, 1]) g.add(ball(0.09, 0.14, 0.04, s * 0.22, 2.45, 0.6, C.eye, { ink: false }));
  g.add(ball(0.26, 0.36, 0.06, 0, 0.42, 1.0, 0x4a3628));
  g.add(ball(0.6, 0.4, 0.6, 0, 3.12, 0, cap));
  // the jewel: a sun-gold orb or a violet crystal, floating
  const jewel = joint(0, 3.85, 0);
  if (sun) {
    jewel.add(orb(0.3, 0, 0, 0, glow(C.gold, 1.1)));
    for (let i = 0; i < 8; i++) {
      const a = (i / 8) * Math.PI * 2;
      jewel.add(cone(0.06, 0.18, Math.cos(a) * 0.38, Math.sin(a) * 0.38 - 0.09, 0, glow(C.sun, 0.9), { ink: false, rot: [0, 0, (a * 180) / Math.PI - 90] }));
    }
    jewel.add(halo(C.sun, 1.8, 0.55));
  } else {
    jewel.add(gem(0.24, 0.42, 0.24, 0, 0, 0, glow(C.arcane, 1.0)));
    jewel.add(halo(C.arcane, 1.8, 0.55));
  }
  g.add(jewel);
  g.add(cone(0.1, 0.25, 0, 3.45, 0, cap));
  return still(g, { top: [0, 3.85, 0], headTop: [0, 3.85, 0], handR: [0, 3.85, 0], chest: [0, 1.5, 0.8] }, (t) => {
    jewel.position.y = 3.85 + Math.sin(t * 2) * 0.06;
    jewel.rotation.y = sun ? t * 0.6 : t * 1.2;
  });
}

/** A barracks: melee is a round dome with a shield and crossed spears;
 * ranged is a drum with a cone roof and a target. */
function barracks(side, kind) {
  const sun = side === "sun";
  const wall = sun ? C.sand : 0x8a93ae, dark = sun ? 0xd9b77a : 0x6a7594, roof = sun ? C.sunDeep : C.moonDeep, team = sun ? C.sun : C.moon, bright = sun ? C.gold : C.lilac;
  const g = new THREE.Group();
  g.add(cyl([1.25, 1.4], 0.3, 0, 0.15, 0, dark));
  if (kind === "melee") {
    g.add(ball(1.15, 1.1, 1.15, 0, 0.45, 0, wall));
    g.add(course(1.06, 0.88, dark));
    g.add(ball(0.55, 0.28, 0.55, 0, 1.5, 0, roof));
    g.add(orb(0.15, 0, 1.82, 0, bright));
    // the emblem: a round shield and two crossed spears over the door
    const em = joint(0, 1.05, 1.0);
    em.add(cyl(0.28, 0.06, 0, 0, 0, shiny(team), { rot: [90, 0, 0] }));
    em.add(orb(0.1, 0, 0, 0.04, shiny(bright)));
    for (const s of [-1, 1]) em.add(limb([s * -0.42, -0.42, -0.05], [s * 0.42, 0.42, -0.05], 0.025, C.wood));
    g.add(em);
  } else {
    g.add(cyl([1.0, 1.1], 1.1, 0, 0.85, 0, wall));
    g.add(course(1.06, 0.5, dark));
    g.add(cone(1.3, 1.1, 0, 1.4, 0, roof));
    g.add(orb(0.15, 0, 2.55, 0, bright));
    // the emblem: a target of rings with an arrow in it
    const em = joint(0, 1.05, 1.07);
    em.add(cyl(0.26, 0.05, 0, 0, 0, C.cloth, { rot: [90, 0, 0] }));
    em.add(cyl(0.17, 0.06, 0, 0, 0, team, { rot: [90, 0, 0] }));
    em.add(cyl(0.07, 0.07, 0, 0, 0, C.cloth, { rot: [90, 0, 0] }));
    em.add(limb([0.05, 0.05, 0.02], [0.3, 0.3, 0.25], 0.018, C.wood));
    g.add(em);
  }
  // an arched door and a banner on a pole
  g.add(ball(0.32, 0.45, 0.08, 0, 0.42, kind === "melee" ? 1.08 : 1.02, 0x4a3628));
  g.add(limb([1.3, 0.3, 0.55], [1.3, 2.3, 0.55], 0.03, C.wood));
  g.add(ball(0.03, 0.26, 0.2, 1.3, 2.0, 0.77, team));
  g.add(orb(0.05, 1.3, 2.32, 0.55, bright));
  return still(g, { top: [0, kind === "melee" ? 1.82 : 2.55, 0], headTop: [0, 2.0, 0], chest: [0, 0.9, 1.0] });
}

/** A Sphere: a huge glowing orb on a pedestal, held in two turning rings. */
function sphere(side) {
  const sun = side === "sun";
  const stone = sun ? C.sand : 0x8a93ae, dark = sun ? 0xd9b77a : 0x6a7594, core = sun ? C.gold : C.arcane, team = sun ? C.sun : C.moon;
  const g = new THREE.Group();
  g.add(cyl([1.7, 1.95], 0.35, 0, 0.18, 0, dark));
  g.add(cyl([1.3, 1.5], 0.4, 0, 0.55, 0, stone));
  g.add(course(1.4, 0.72, dark));
  for (let i = 0; i < 4; i++) {
    const a = (i / 4) * Math.PI * 2 + Math.PI / 4;
    g.add(limb([Math.cos(a) * 1.15, 0.7, Math.sin(a) * 1.15], [Math.cos(a) * 0.7, 1.6, Math.sin(a) * 0.7], 0.14, stone));
  }
  const orbJ = joint(0, 2.75, 0);
  orbJ.add(orb(1.1, 0, 0, 0, glow(core, 0.7)));
  orbJ.add(orb(1.22, 0, 0, 0, light(team, 0.25), { ink: false }));
  orbJ.add(halo(team, 5.5, 0.45));
  g.add(orbJ);
  const r1 = hoop(1.6, 0.09, 0, 2.75, 0, shiny(sun ? C.sunDeep : C.lilac));
  const r2 = hoop(1.85, 0.07, 0, 2.75, 0, shiny(sun ? C.gold : C.moonDeep));
  g.add(r1, r2);
  return still(g, { top: [0, 3.9, 0], headTop: [0, 3.9, 0], chest: [0, 2.75, 1.2], handR: [0, 2.75, 0] }, (t) => {
    r1.rotation.set(70 * D + Math.sin(t) * 0.1, t * 0.7, 20 * D);
    r2.rotation.set(-60 * D, -t * 0.5, -25 * D + Math.sin(t * 0.8) * 0.1);
    orbJ.position.y = 2.75 + Math.sin(t * 1.5) * 0.05;
  });
}

/** A fountain: a round basin of water round a pillar with the side's light. */
function fountain(side) {
  const sun = side === "sun";
  const stone = sun ? C.sand : 0x8a93ae, dark = sun ? 0xd9b77a : 0x6a7594, team = sun ? C.sun : C.moon, core = sun ? C.gold : C.arcane;
  const g = new THREE.Group();
  g.add(cyl([1.5, 1.6], 0.45, 0, 0.22, 0, stone));
  g.add(hoop(1.5, 0.13, 0, 0.46, 0, dark, { rot: [90, 0, 0] }));
  g.add(cyl(1.38, 0.05, 0, 0.42, 0, veil(C.water, 0.85, { roughness: 0.15 }), { ink: false }));
  g.add(cyl([0.3, 0.4], 1.0, 0, 0.9, 0, stone));
  g.add(ball(0.6, 0.18, 0.6, 0, 1.45, 0, dark));
  g.add(cyl(0.5, 0.05, 0, 1.53, 0, veil(C.water, 0.85), { ink: false }));
  const top = joint(0, 1.9, 0);
  top.add(orb(0.22, 0, 0, 0, glow(core, 1.1)));
  top.add(halo(team, 1.4, 0.6));
  g.add(top);
  // spouts of water arcing down, and sparkles
  const drops = [];
  for (let i = 0; i < 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    const d = orb(0.06, Math.cos(a) * 0.8, 1.0, Math.sin(a) * 0.8, veil(C.ice, 0.9), { ink: false });
    d.userData.a = a;
    drops.push(d);
    g.add(d);
  }
  return still(g, { top: [0, 1.9, 0], headTop: [0, 1.9, 0], chest: [0, 0.8, 1.4] }, (t) => {
    drops.forEach((d, i) => {
      const u = (t * 0.8 + i / 6) % 1;
      const r = 0.5 + u * 0.85;
      d.position.set(Math.cos(d.userData.a) * r, 1.5 + u * 0.3 - u * u * 1.2, Math.sin(d.userData.a) * r);
    });
    top.position.y = 1.9 + Math.sin(t * 2) * 0.05;
  });
}

/** The shop: a market stall with a striped round canopy and pots of goods. */
function shop() {
  const g = new THREE.Group();
  g.add(cyl([1.0, 1.05], 0.7, 0, 0.35, 0.2, C.wood));
  g.add(hoop(1.0, 0.07, 0, 0.72, 0.2, C.woodDark, { rot: [90, 0, 0] }));
  for (const [x, z] of [[-0.85, -0.3], [0.85, -0.3], [-0.85, 0.7], [0.85, 0.7]]) g.add(limb([x, 0, z], [x, 1.9, z], 0.05, C.woodDark));
  // the canopy: a dome in alternating cream and red slices
  for (let i = 0; i < 8; i++) {
    const a = (i / 8) * Math.PI * 2;
    g.add(ball(0.55, 0.32, 0.3, Math.cos(a) * 0.62, 2.05, 0.2 + Math.sin(a) * 0.62, i % 2 ? C.sunCream : 0xe0434a, { rot: [0, -(a * 180) / Math.PI, 0] }));
  }
  g.add(ball(1.05, 0.5, 1.05, 0, 2.15, 0.2, 0xe0434a));
  g.add(orb(0.15, 0, 2.72, 0.2, C.gold));
  // goods: pots, a potion, a coin pile
  g.add(ball(0.17, 0.2, 0.17, -0.45, 0.88, 0.5, 0xd98a5a));
  g.add(ball(0.12, 0.14, 0.12, -0.1, 0.84, 0.65, glow(0xe0435a, 0.5)));
  g.add(cyl(0.04, 0.08, -0.1, 1.0, 0.65, C.wood));
  g.add(gem(0.1, 0.16, 0.1, 0.25, 0.9, 0.6, glow(C.frost, 0.6)));
  for (let i = 0; i < 4; i++) g.add(cyl(0.08, 0.025, 0.55, 0.74 + i * 0.03, 0.5, shiny(C.gold)));
  // a sign hung from the front
  g.add(cyl(0.22, 0.05, 0, 1.62, 0.92, C.sunCream, { rot: [90, 0, 0] }));
  g.add(orb(0.08, 0, 1.62, 0.96, shiny(C.gold)));
  return still(g, { top: [0, 2.75, 0.2], headTop: [0, 2.75, 0.2], chest: [0, 1.0, 1.0] });
}

// ------------------------------------------------------------ the forest
/** A four-legged creature: body, head (with a face), four legs on joints.
 * Front legs are the rig's arms, hind legs its legs. */
function quad(o) {
  const root = new THREE.Group();
  const bodyR = o.body ?? [0.32, 0.26, 0.42];
  const legLen = o.legLen ?? 0.2;
  const y = legLen + bodyR[1] * 0.6;
  const body = joint(0, y, 0);
  body.add(ball(...bodyR, 0, 0, 0, o.color));
  if (o.belly) body.add(ball(bodyR[0] * 0.8, bodyR[1] * 0.7, bodyR[2] * 0.8, 0, -bodyR[1] * 0.25, 0.02, o.belly));
  const hr = o.headR ?? 0.24;
  const head = joint(0, bodyR[1] * 0.55, bodyR[2] * 0.85);
  head.add(orb(hr, 0, hr * 0.35, hr * 0.5, o.headColor ?? o.color));
  if (o.snout) head.add(ball(hr * 0.55, hr * 0.42, hr * 0.5, 0, hr * 0.12, hr * 1.35, o.snout));
  if (o.snout) head.add(orb(hr * 0.13, 0, hr * 0.32, hr * 1.82, C.eye, { ink: false }));
  face(head, 0, hr * 0.35, hr * 0.5, hr, { sep: 0.42, h: 0.12, ...(o.face ?? {}) });
  body.add(head);
  const leg = (x, z) => {
    const j = joint(x, y - bodyR[1] * 0.3, z);
    j.add(limb([0, 0, 0], [0, -(y - bodyR[1] * 0.3) + 0.06, 0], o.legR ?? 0.07, o.legColor ?? o.color));
    j.add(ball(0.08, 0.05, 0.09, 0, -(y - bodyR[1] * 0.3) + 0.04, 0.02, o.foot ?? o.legColor ?? o.color));
    return j;
  };
  const lx = bodyR[0] * 0.6, lz = bodyR[2] * 0.55;
  const armL = leg(-lx, lz), armR = leg(lx, lz), legL = leg(-lx, -lz), legR = leg(lx, -lz);
  root.add(body, armL, armR, legL, legR);
  const at = anchors(root, {});
  at.chest = joint(0, 0, bodyR[2]); body.add(at.chest);
  at.headTop = joint(0, hr * 1.4, hr * 0.5); head.add(at.headTop);
  at.top = at.headTop;
  at.handR = joint(0, hr * 0.2, hr * 1.4); head.add(at.handR);
  return { root, body, head, armL, armR, legL, legR, at, hr };
}
function finishQuad(q, extra = {}) {
  return { g: q.root, at: q.at, rig: rig(q.root, { body: q.body, head: q.head, armL: q.armL, armR: q.armR, legL: q.legL, legR: q.legR }), ...extra };
}

/** Ridge hound: a sandy wolf-pup with big ears, a ruff and a curled tail. */
function hound() {
  const q = quad({ color: 0xb08a6a, belly: 0xf1e3cf, headColor: 0xb08a6a, snout: 0xf1e3cf, body: [0.26, 0.22, 0.4], headR: 0.24 });
  for (const s of [-1, 1]) q.head.add(cone(0.09, 0.22, s * 0.15, 0.22, 0.05, 0xb08a6a, { rot: [-10, 0, -s * 15] }));
  q.body.add(ball(0.24, 0.22, 0.14, 0, 0.08, 0.3, 0xf1e3cf));
  q.body.add(limb([0, 0.1, -0.38], [0, 0.42, -0.55], 0.07, 0xb08a6a));
  q.body.add(orb(0.08, 0, 0.45, -0.55, 0xf1e3cf));
  return finishQuad(q);
}

/** Mossback: a tortoise under a high domed shell with moss and a flower. */
function mossback() {
  const q = quad({ color: 0x9cc77a, body: [0.42, 0.22, 0.48], headR: 0.2, legR: 0.09, legLen: 0.14 });
  q.body.add(ball(0.52, 0.42, 0.58, 0, 0.14, -0.02, 0x8a6a4a));
  q.body.add(hoop(0.52, 0.06, 0, 0.0, -0.02, 0x6e5232, { rot: [90, 0, 0] }));
  for (let i = 0; i < 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    q.body.add(ball(0.13, 0.06, 0.13, Math.cos(a) * 0.3, 0.42, Math.sin(a) * 0.32, 0x7a5a3a));
  }
  q.body.add(ball(0.32, 0.12, 0.34, 0.05, 0.53, -0.05, C.moss));
  for (let i = 0; i < 5; i++) {
    const a = (i / 5) * Math.PI * 2;
    q.body.add(orb(0.05, 0.12 + Math.cos(a) * 0.06, 0.66, -0.05 + Math.sin(a) * 0.06, 0xffb6c8));
  }
  q.body.add(orb(0.045, 0.12, 0.68, -0.05, C.gold));
  q.head.position.z += 0.08;
  return finishQuad(q);
}

/** A humanoid neutral on the chibi body; returns the chibi pieces. */
function biped(o) {
  const c = chibi(o);
  if (o.scale) c.root.scale.setScalar(o.scale);
  return c;
}

/** Bramble imp: a round green sprite with thorns, big ears and a leaf on top. */
function imp() {
  const c = biped({ skin: 0x9be36a, torso: [0.26, 0.22, 0.24], torsoColor: 0x6fbf4f, sleeve: 0x9be36a, legs: 0x6fbf4f, boots: 0x4f9e3a, headR: 0.34, legLen: 0.12, scale: 0.62, faceOpts: { size: 0.17, mouth: true } });
  const { head, body, armR } = c.parts;
  const hr = c.hr;
  for (const s of [-1, 1]) head.add(cone(hr * 0.22, hr * 0.7, s * hr * 0.95, hr * 1.0, 0, 0x9be36a, { rot: [0, 0, -s * 70] }));
  head.add(ball(hr * 0.3, hr * 0.06, hr * 0.5, hr * 0.1, hr * 2.0, 0, C.leaf, { rot: [0, 0, -25] }));
  head.add(limb([0, hr * 1.85, 0], [hr * 0.05, hr * 2.05, 0], 0.015, C.woodDark, { ink: false }));
  for (let i = 0; i < 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    body.add(cone(0.05, 0.14, Math.cos(a) * 0.24, 0.3 + (i % 2) * 0.12, Math.sin(a) * 0.22 - 0.02, 0x4f9e3a, { rot: [Math.sin(a) * 70, 0, -Math.cos(a) * 70] }));
  }
  const sp = joint(0, -0.3, 0.03);
  sp.add(limb([0, -0.25, 0], [0, 0.45, 0], 0.025, C.woodDark));
  for (const s of [-1, 1]) sp.add(cone(0.025, 0.1, s * 0.03, 0.42, 0, C.bone, { rot: [0, 0, -s * 20] }));
  armR.add(sp);
  armR.rotation.set(-20 * D, 0, 10 * D);
  return finishCreep(c);
}

/** A stone golem: a boulder body, mossy shoulders, small glowing eyes, big
 * fists. `big` picks the great golem. */
function golem(big) {
  const s = big ? 1 : 0.7;
  const root = new THREE.Group();
  const stone = big ? 0xa7a09a : 0xb5aea6, dark = 0x7f7870;
  const legL = joint(-0.26, 0.42, 0), legR = joint(0.26, 0.42, 0);
  for (const l of [legL, legR]) { l.add(ball(0.2, 0.26, 0.2, 0, -0.2, 0, dark)); l.add(ball(0.24, 0.12, 0.28, 0, -0.38, 0.05, stone)); }
  const body = joint(0, 0.42, 0);
  body.add(ball(0.62, 0.58, 0.52, 0, 0.55, 0, stone));
  body.add(ball(0.3, 0.2, 0.15, -0.2, 0.4, 0.42, dark));
  body.add(ball(0.18, 0.14, 0.1, 0.25, 0.75, 0.44, 0xc4bdb4));
  for (const x of [-0.5, 0.5]) body.add(ball(0.3, 0.14, 0.3, x, 1.0, 0, C.moss));
  body.add(ball(0.3, 0.12, 0.26, 0.1, 1.08, -0.1, C.moss));
  const head = joint(0, 1.05, 0.15);
  head.add(ball(0.3, 0.26, 0.28, 0, 0.12, 0.08, 0xc4bdb4));
  face(head, 0, 0.12, 0.08, 0.3, { glowEye: true, eye: 0x7fe3c2, blush: false, sep: 0.4, size: 0.15, tall: 0.8 });
  body.add(head);
  const arm = (x) => {
    const j = joint(x, 0.95, 0);
    j.add(limb([0, 0, 0], [x * 0.25, -0.5, 0.1], 0.16, dark));
    j.add(ball(0.26, 0.24, 0.26, x * 0.3, -0.65, 0.12, stone));
    const hand = joint(x * 0.3, -0.65, 0.12); j.add(hand); j.userData.hand = hand;
    return j;
  };
  const armL = arm(-1.05 * 0.62), armR = arm(1.05 * 0.62);
  body.add(armL, armR);
  root.add(legL, legR, body);
  root.scale.setScalar(s);
  const at = anchors(root, {});
  at.chest = joint(0, 0.55, 0.5); body.add(at.chest);
  at.headTop = joint(0, 0.42, 0.08); head.add(at.headTop);
  at.top = at.headTop;
  at.handR = armR.userData.hand;
  return { g: root, at, rig: rig(root, { body, head, armL, armR, legL, legR }) };
}

/** Striped wings: two flat see-through ellipses on a joint. */
function wings(w, h, y, z) {
  const j = joint(0, y, z);
  for (const s of [-1, 1]) {
    const wg = ball(w, h, 0.02, s * w * 0.95, h * 0.3, 0, veil(0xeaf6ff, 0.55), { rot: [0, s * 20, -s * 25], d: 0.012 });
    j.add(wg);
  }
  return j;
}

/** Horned hornet shaman: a striped bee-folk in a leaf cape, antennae and two
 * curled horns, with a honey-orb staff. */
function shaman() {
  const c = biped({ skin: 0xffd36a, torso: [0.3, 0.3, 0.28], torsoColor: 0xffc23a, sleeve: 0x3a2f33, legs: 0x3a2f33, boots: 0x2b2140, hands: 0xffd36a, headR: 0.33, scale: 0.9 });
  const { head, body, armR } = c.parts;
  const hr = c.hr;
  for (const y of [0.3, 0.52]) body.add(ball(0.305, 0.06, 0.285, 0, y, 0, 0x3a2f33));
  for (const s of [-1, 1]) {
    head.add(limb([s * hr * 0.3, hr * 1.7, 0], [s * hr * 0.55, hr * 2.35, hr * 0.2], 0.014, 0x3a2f33, { ink: false }));
    head.add(orb(hr * 0.1, s * hr * 0.55, hr * 2.38, hr * 0.2, 0x3a2f33));
    const h = hoop(hr * 0.35, hr * 0.08, s * hr * 0.75, hr * 1.4, -hr * 0.1, C.bone, { arc: 160 });
    h.rotation.set(0, 0, s > 0 ? -40 * D : 220 * D);
    head.add(h);
  }
  body.add(wings(0.28, 0.4, 0.75, -0.25));
  body.add(ball(0.32, 0.36, 0.06, 0, 0.35, -0.3, C.leaf));
  const st = joint(0, -0.3, 0.03);
  st.add(limb([0, -0.45, 0], [0, 0.75, 0], 0.028, C.wood));
  st.add(orb(0.11, 0, 0.85, 0, glow(0xffb02e, 0.9)));
  st.add(hoop(0.11, 0.02, 0, 0.85, 0, 0x3a2f33));
  armR.add(st);
  armR.rotation.set(-25 * D, 0, 8 * D);
  c.at.tip = joint(0, 0.85, 0); st.add(c.at.tip);
  return finishCreep(c);
}

/** Giant hornet: a round striped bug with buzzing wings and a stinger. */
function hornet() {
  const root = new THREE.Group();
  const body = joint(0, 0.75, 0);
  body.add(orb(0.22, 0, 0, 0.12, 0xffc23a));
  body.add(ball(0.28, 0.26, 0.34, 0, -0.05, -0.32, 0xffc23a));
  for (const z of [-0.2, -0.36, -0.52]) body.add(ball(0.285, 0.265, 0.05, 0, -0.05, z, 0x3a2f33));
  body.add(cone(0.06, 0.2, 0, -0.1, -0.68, 0x3a2f33, { rot: [-100, 0, 0] }));
  const head = joint(0, 0.05, 0.32);
  head.add(orb(0.22, 0, 0, 0.05, 0xffd36a));
  face(head, 0, 0, 0.05, 0.22, { size: 0.2, sep: 0.4 });
  for (const s of [-1, 1]) head.add(limb([s * 0.07, 0.18, 0.05], [s * 0.18, 0.36, 0.18], 0.012, 0x3a2f33, { ink: false }));
  body.add(head);
  const wing = wings(0.26, 0.15, 0.2, -0.15);
  body.add(wing);
  for (let i = 0; i < 3; i++) for (const s of [-1, 1]) body.add(limb([s * 0.12, -0.15, 0.05 - i * 0.12], [s * 0.2, -0.45, 0.08 - i * 0.12], 0.025, 0x3a2f33));
  root.add(body);
  root.add(ball(0.25, 0.01, 0.3, 0, 0.01, -0.1, veil(C.ink, 0.15), { ink: false }));
  const at = anchors(root, { feet: [0, 0.02, 0] });
  at.chest = joint(0, 0, 0.3); body.add(at.chest);
  at.headTop = joint(0, 0.25, 0.05); head.add(at.headTop);
  at.top = at.headTop;
  at.handR = joint(0, -0.1, -0.72); body.add(at.handR);
  return { g: root, at, rig: rig(root, { body, head }), update: (t) => {
    wing.rotation.x = Math.sin(t * 40) * 0.35;
    body.position.y = 0.75 + Math.sin(t * 3) * 0.06;
  } };
}

/** A drake: a round dragon with horns, a spiky crest, bat wings and a tail. */
function drake(young) {
  const col = young ? 0x8fd1a8 : 0x5fae8a, belly = 0xf6e7b8;
  const q = quad({ color: col, belly, body: young ? [0.34, 0.3, 0.44] : [0.62, 0.55, 0.78], headR: young ? 0.28 : 0.44, snout: col, legR: young ? 0.08 : 0.15, legLen: young ? 0.2 : 0.38, face: { sep: 0.44, h: 0.2 } });
  const s = young ? 0.6 : 1;
  for (const x of [-1, 1]) q.head.add(cone(0.06 * s + 0.02, 0.32 * s + 0.08, x * q.hr * 0.45, q.hr * 1.05, q.hr * 0.05, C.bone, { rot: [-40, 0, -x * 18] }));
  for (let i = 0; i < 5; i++) q.body.add(cone(0.07 * s + 0.02, 0.16 * s + 0.04, 0, q.body.children[0].scale.y * 0.88 - i * 0.03 * s, 0.3 * s - i * 0.18 * s, 0xf28a3c, { rot: [-25, 0, 0] }));
  // wings: a bone arm and a membrane on each side
  const wingJ = joint(0, q.body.children[0].scale.y * 0.7, -0.05);
  const ws = young ? 0.55 : 1.1;
  for (const x of [-1, 1]) {
    wingJ.add(limb([x * 0.25 * ws, 0, 0], [x * 0.75 * ws, 0.7 * ws, -0.1 * ws], 0.04 * ws + 0.01, col));
    wingJ.add(limb([x * 0.75 * ws, 0.7 * ws, -0.1 * ws], [x * 1.25 * ws, 0.35 * ws, -0.2 * ws], 0.03 * ws + 0.01, col));
    wingJ.add(orb(0.06 * ws + 0.02, x * 0.75 * ws, 0.72 * ws, -0.1 * ws, C.bone));
    for (const [cx, cy, rx, ry] of [[0.55, 0.32, 0.32, 0.34], [0.98, 0.28, 0.3, 0.26]]) {
      wingJ.add(ball(rx * ws, ry * ws, 0.025, x * cx * ws, cy * ws, -0.15 * ws, veil(0xf28a3c, 0.9), { rot: [-20, x * 25, 0], d: 0.015 }));
    }
  }
  q.body.add(wingJ);
  // the tail, tapering, with a spade tip
  const R = q.body.children[0].scale;
  for (let i = 0; i < 4; i++) q.body.add(orb((0.16 - i * 0.03) * s + 0.03, 0, -R.y * 0.2 - i * 0.05 * s, -R.z - i * 0.18 * s, col));
  q.body.add(cone(0.1 * s + 0.03, 0.15 * s + 0.05, 0, -R.y * 0.4, -R.z - 0.82 * s, 0xf28a3c, { rot: [-90, 0, 0] }));
  return finishQuad(q, { update: (t) => { wingJ.rotation.z = Math.sin(t * 2.5) * 0.08; wingJ.children.forEach((w, i) => { w.rotation.z += 0; }); } });
}

/** The Warden (after Roshan): a huge round horned beast with tusks, a
 * stone collar and a little underbite. */
function warden() {
  const root = new THREE.Group();
  const hide = 0x9a7ab0, dark = 0x6e5288, belly = 0xe8d6c0;
  const legL = joint(-0.55, 0.75, 0), legR = joint(0.55, 0.75, 0);
  for (const l of [legL, legR]) { l.add(limb([0, 0, 0], [0, -0.55, 0], 0.26, dark)); l.add(ball(0.34, 0.2, 0.4, 0, -0.62, 0.08, dark)); for (const x of [-0.15, 0, 0.15]) l.add(orb(0.07, x, -0.66, 0.44, C.bone)); }
  const body = joint(0, 0.75, 0);
  body.add(ball(1.15, 1.05, 0.95, 0, 0.9, 0, hide));
  body.add(ball(0.85, 0.75, 0.4, 0, 0.75, 0.62, belly));
  for (let i = 0; i < 5; i++) body.add(cone(0.16, 0.36, (i - 2) * 0.35, 1.6 - Math.abs(i - 2) * 0.12, -0.5, C.bone, { rot: [-35, 0, (i - 2) * -15] }));
  // a stone collar with gold studs
  body.add(hoop(0.8, 0.16, 0, 1.55, 0.15, C.stone, { rot: [80, 0, 0] }));
  for (let i = 0; i < 6; i++) { const a = (i / 6) * Math.PI * 2; body.add(orb(0.07, Math.cos(a) * 0.8, 1.55 + Math.sin(a) * 0.14, 0.15 + Math.sin(a) * 0.79, shiny(C.gold))); }
  const head = joint(0, 1.7, 0.45);
  head.add(ball(0.72, 0.6, 0.62, 0, 0.3, 0.1, hide));
  head.add(ball(0.5, 0.3, 0.32, 0, 0.05, 0.55, belly));
  face(head, 0, 0.45, 0.1, 0.66, { sep: 0.42, h: 0.05, size: 0.13 });
  for (const s of [-1, 1]) {
    head.add(cone(0.08, 0.22, s * 0.28, 0.12, 0.8, C.bone, { rot: [-15, 0, 0] }));
    head.add(limb([s * 0.5, 0.7, 0], [s * 0.95, 0.85, -0.05], 0.13, C.bone));
    head.add(limb([s * 0.95, 0.85, -0.05], [s * 1.1, 1.2, -0.05], 0.1, C.bone));
    head.add(cone(0.1, 0.25, s * 1.12, 1.2, -0.05, C.bone, { rot: [0, 0, -s * 10] }));
    head.add(ball(0.18, 0.1, 0.12, s * 0.62, 0.55, 0.15, dark, { rot: [0, 0, s * 30] }));
  }
  body.add(head);
  const arm = (x) => {
    const j = joint(x, 1.35, 0.1);
    j.add(limb([0, 0, 0], [x * 0.3, -0.8, 0.25], 0.22, dark));
    j.add(ball(0.32, 0.28, 0.34, x * 0.36, -0.95, 0.3, dark));
    for (const z of [-0.12, 0, 0.12]) j.add(cone(0.05, 0.14, x * 0.4 + z, -1.2, 0.5, C.bone, { rot: [160, 0, 0] }));
    const hand = joint(x * 0.36, -0.95, 0.3); j.add(hand); j.userData.hand = hand;
    return j;
  };
  const armL = arm(-1.1), armR = arm(1.1);
  body.add(armL, armR);
  body.add(limb([0, 0.4, -0.85], [0, 0.2, -1.5], 0.14, hide));
  body.add(orb(0.18, 0, 0.18, -1.55, dark));
  root.add(legL, legR, body);
  const at = anchors(root, {});
  at.chest = joint(0, 0.9, 0.95); body.add(at.chest);
  at.headTop = joint(0, 1.0, 0.1); head.add(at.headTop);
  at.top = at.headTop;
  at.handR = armR.userData.hand;
  return { g: root, at, rig: rig(root, { body, head, armL, armR, legL, legR }) };
}

/** The hex critter: a small green toad with a gold crown-spot, harmless. */
function critter() {
  const root = new THREE.Group();
  const body = joint(0, 0, 0);
  body.add(ball(0.24, 0.17, 0.22, 0, 0.17, 0, 0x8fd16f));
  body.add(ball(0.18, 0.11, 0.14, 0, 0.13, 0.1, 0xf6efb8));
  const head = joint(0, 0.26, 0.06);
  for (const s of [-1, 1]) {
    head.add(orb(0.08, s * 0.11, 0.05, 0.05, 0x8fd16f));
    head.add(orb(0.045, s * 0.12, 0.07, 0.11, C.eye, { ink: false }));
    head.add(orb(0.016, s * 0.12 + 0.012, 0.09, 0.15, C.white, { ink: false }));
    head.add(ball(0.04, 0.02, 0.02, s * 0.17, -0.04, 0.17, veil(C.blush, 0.6), { ink: false }));
  }
  head.add(hoop(0.06, 0.012, 0, -0.03, 0.2, C.eye, { arc: 180, rot: [0, 0, 180], ink: false }));
  body.add(head);
  const legL = joint(-0.15, 0.06, -0.05), legR = joint(0.15, 0.06, -0.05);
  for (const [l, s] of [[legL, -1], [legR, 1]]) { l.add(ball(0.1, 0.06, 0.12, s * 0.04, 0, 0, 0x79c160)); l.add(ball(0.06, 0.02, 0.07, s * 0.05, -0.04, 0.1, 0x79c160)); }
  const armL = joint(-0.1, 0.08, 0.15), armR = joint(0.1, 0.08, 0.15);
  for (const a of [armL, armR]) a.add(ball(0.04, 0.06, 0.04, 0, -0.03, 0, 0x79c160));
  root.add(body, legL, legR, armL, armR);
  const at = anchors(root, { chest: [0, 0.18, 0.2], headTop: [0, 0.4, 0.05], handR: [0.1, 0.1, 0.2], top: [0, 0.4, 0.05] });
  return { g: root, at, rig: rig(root, { body, head, armL, armR, legL, legR }) };
}

/** The courier: a small pack mule with long ears, saddlebags and a pennant. */
function courier(team = "sun") {
  const tc = team === "moon" ? C.moon : C.sun;
  const q = quad({ color: 0xb0a090, belly: 0xece2d4, snout: 0xece2d4, body: [0.26, 0.24, 0.36], headR: 0.22, legLen: 0.2, legR: 0.06, foot: 0x5a4636 });
  for (const s of [-1, 1]) q.head.add(ball(0.06, 0.2, 0.04, s * 0.12, 0.38, 0.02, 0xb0a090, { rot: [0, 0, -s * 18] }));
  q.head.add(ball(0.08, 0.06, 0.12, 0, 0.3, 0.2, 0x5a4636));
  for (const s of [-1, 1]) q.body.add(ball(0.12, 0.16, 0.18, s * 0.28, 0.02, -0.02, tc));
  q.body.add(ball(0.24, 0.06, 0.26, 0, 0.22, -0.02, C.leather));
  q.body.add(ball(0.16, 0.12, 0.14, 0, 0.32, -0.12, 0xece2d4));
  q.body.add(limb([0.1, 0.3, -0.2], [0.1, 0.8, -0.2], 0.015, C.wood));
  q.body.add(ball(0.02, 0.1, 0.14, 0.1, 0.72, -0.12, tc));
  q.body.add(limb([0, 0.05, -0.34], [0, -0.15, -0.46], 0.03, 0x5a4636));
  return finishQuad(q);
}

/** A ward on a stalk. The observer is a big round gold eye; the sentry a
 * blue flame in a little cage on three legs. */
function ward(kind) {
  const g = new THREE.Group();
  if (kind === "observer") {
    g.add(cone(0.18, 0.12, 0, 0, 0, C.woodDark));
    g.add(limb([0, 0.05, 0], [0, 0.85, 0], 0.035, C.wood));
    for (const s of [-1, 1]) g.add(ball(0.12, 0.04, 0.06, s * 0.1, 0.65, 0, C.leaf, { rot: [0, 0, -s * 30] }));
    const eye = joint(0, 1.08, 0);
    eye.add(orb(0.24, 0, 0, 0, shiny(0xfff6dc)));
    eye.add(ball(0.13, 0.13, 0.06, 0, 0, 0.2, glow(C.gold, 0.6), { ink: false }));
    eye.add(ball(0.06, 0.08, 0.04, 0, 0, 0.245, C.eye, { ink: false }));
    eye.add(orb(0.025, 0.03, 0.04, 0.27, C.white, { ink: false }));
    eye.add(hoop(0.25, 0.04, 0, 0.0, 0, C.gold, { arc: 180, rot: [0, 0, 0] }));
    eye.add(halo(C.gold, 0.9, 0.35));
    g.add(eye);
    return still(g, { top: [0, 1.35, 0], headTop: [0, 1.35, 0], chest: [0, 1.08, 0.25] }, (t) => { eye.rotation.y = Math.sin(t * 0.9) * 0.5; eye.position.y = 1.08 + Math.sin(t * 2) * 0.03; });
  }
  for (let i = 0; i < 3; i++) {
    const a = (i / 3) * Math.PI * 2;
    g.add(limb([Math.cos(a) * 0.3, 0, Math.sin(a) * 0.3], [0, 0.75, 0], 0.03, C.slate));
  }
  g.add(cyl([0.14, 0.18], 0.08, 0, 0.78, 0, C.iron));
  for (let i = 0; i < 4; i++) {
    const a = (i / 4) * Math.PI * 2;
    g.add(limb([Math.cos(a) * 0.15, 0.82, Math.sin(a) * 0.15], [Math.cos(a) * 0.08, 1.2, Math.sin(a) * 0.08], 0.015, C.iron));
  }
  g.add(cone(0.15, 0.14, 0, 1.18, 0, C.iron));
  const fl = joint(0, 0.95, 0);
  fl.add(orb(0.1, 0, -0.04, 0, glow(C.frost, 0.9), { ink: false }));
  fl.add(cone(0.09, 0.22, 0, 0.0, 0, glow(C.frost, 0.9), { ink: false }));
  fl.add(orb(0.05, 0, -0.03, 0.03, glow(C.white, 1.2), { ink: false }));
  fl.add(halo(C.frost, 0.8, 0.6));
  g.add(fl);
  return still(g, { top: [0, 1.32, 0], headTop: [0, 1.32, 0], chest: [0, 0.95, 0.15] }, (t) => { fl.scale.set(1, 1 + Math.sin(t * 9) * 0.12, 1); });
}

/** Healing totem: a short carved pole with leaves and a green light that pulses. */
function healingWard() {
  const g = new THREE.Group();
  g.add(cyl([0.22, 0.26], 0.1, 0, 0.05, 0, C.woodDark));
  g.add(cyl([0.13, 0.16], 0.8, 0, 0.5, 0, C.wood));
  for (const y of [0.3, 0.6]) g.add(hoop(0.15, 0.03, 0, y, 0, glow(C.poison, 0.8), { rot: [90, 0, 0] }));
  face(g, 0, 0.75, 0, 0.15, { sep: 0.35, mouth: true });
  for (let i = 0; i < 4; i++) {
    const a = (i / 4) * Math.PI * 2;
    g.add(ball(0.16, 0.04, 0.08, Math.cos(a) * 0.17, 0.95, Math.sin(a) * 0.17, C.leaf, { rot: [0, -(a * 180) / Math.PI, 25] }));
  }
  const top = joint(0, 1.12, 0);
  top.add(orb(0.13, 0, 0, 0, glow(C.poison, 1.2)));
  top.add(halo(C.poison, 1.0, 0.6));
  g.add(top);
  return still(g, { top: [0, 1.12, 0], headTop: [0, 1.12, 0], chest: [0, 0.6, 0.15] }, (t) => { const k = 1 + Math.sin(t * 4) * 0.12; top.scale.setScalar(k); });
}

/** A fissure rock: a cluster of rounded slate stones with a crack of light. */
function fissureRock() {
  const g = new THREE.Group();
  g.add(gem(0.42, 0.5, 0.38, 0, 0.38, 0, 0x8a93ae, { ico: true }));
  g.add(gem(0.28, 0.32, 0.26, 0.38, 0.22, 0.12, 0x9aa2ba, { ico: true }));
  g.add(gem(0.22, 0.26, 0.22, -0.34, 0.18, 0.18, 0x7a849e, { ico: true }));
  g.add(ball(0.2, 0.05, 0.18, 0, 0.82, 0, C.moss));
  return still(g, { top: [0, 0.88, 0], headTop: [0, 0.88, 0], chest: [0, 0.4, 0.35] });
}

/** A Sunward pine: three plump stacked cones on a trunk. */
function treeSun() {
  const g = new THREE.Group();
  g.add(cyl([0.14, 0.18], 0.6, 0, 0.3, 0, C.trunk));
  const tiers = [[0.95, 1.0, 0.45], [0.75, 0.9, 1.05], [0.52, 0.8, 1.6]];
  tiers.forEach(([r, h, y], i) => {
    g.add(cone(r, h, 0, y, 0, i % 2 ? C.leaf : 0x6fc163));
    g.add(ball(r * 0.95, 0.12, r * 0.95, 0, y + 0.02, 0, i % 2 ? C.leafDeep : C.leaf));
  });
  g.add(orb(0.1, 0, 2.42, 0, C.leaf));
  return still(g, { top: [0, 2.45, 0], headTop: [0, 2.45, 0], chest: [0, 1.0, 0.7] });
}
/** A Moonward frost tree: round pale-blue crowns with snow caps on a twisted trunk. */
function treeMoon() {
  const g = new THREE.Group();
  g.add(limb([0, 0, 0], [0.1, 1.0, 0], 0.13, 0x7a6a8a));
  g.add(limb([0.1, 0.8, 0], [-0.35, 1.35, 0.1], 0.07, 0x7a6a8a));
  g.add(limb([0.1, 0.9, 0], [0.4, 1.4, -0.1], 0.07, 0x7a6a8a));
  for (const [x, y, z, r] of [[0, 1.75, 0, 0.62], [-0.45, 1.4, 0.1, 0.42], [0.48, 1.45, -0.05, 0.44], [0.1, 2.2, 0, 0.4]]) {
    g.add(orb(r, x, y, z, C.frostLeaf));
    g.add(ball(r * 0.8, r * 0.35, r * 0.8, x, y + r * 0.6, z, C.white));
  }
  return still(g, { top: [0, 2.5, 0], headTop: [0, 2.5, 0], chest: [0, 1.4, 0.6] });
}

// ------------------------------------------------------------ runes
/** A rune: a floating shape over a small stone base, bobbing and turning.
 * Each has its own shape so colour is never the only cue. */
function rune(kind) {
  const g = new THREE.Group();
  g.add(cyl([0.26, 0.3], 0.08, 0, 0.04, 0, C.stone));
  const col = { haste: 0xff5a4f, double: 0x4f8cff, illusion: 0xffd23f, invis: 0xc9a6ff, regen: 0x5fd36a }[kind];
  const j = joint(0, 0.62, 0);
  if (kind === "haste") {
    // a tall thin diamond with two swept fins
    j.add(gem(0.13, 0.32, 0.13, 0, 0, 0, glow(col, 0.8)));
    for (const s of [-1, 1]) j.add(cone(0.06, 0.24, s * 0.16, -0.18, 0, glow(col, 0.5), { rot: [0, 0, -s * 120] }));
  } else if (kind === "double") {
    // twin orbs side by side
    for (const s of [-1, 1]) j.add(orb(0.15, s * 0.14, 0, 0, glow(col, 0.8)));
  } else if (kind === "illusion") {
    // three small orbs round a centre
    for (let i = 0; i < 3; i++) { const a = (i / 3) * Math.PI * 2; j.add(orb(0.1, Math.cos(a) * 0.18, Math.sin(a) * 0.18, 0, glow(col, 0.8))); }
    j.add(orb(0.05, 0, 0, 0, glow(C.white, 1.2)));
  } else if (kind === "invis") {
    // a hollow ring, half see-through
    j.add(hoop(0.2, 0.06, 0, 0, 0, veil(col, 0.6, { emissive: col, emissiveIntensity: 0.6 })));
  } else {
    // a rounded plus
    j.add(limb([0, -0.2, 0], [0, 0.2, 0], 0.08, glow(col, 0.8)));
    j.add(limb([-0.2, 0, 0], [0.2, 0, 0], 0.08, glow(col, 0.8)));
  }
  j.add(halo(col, 1.1, 0.6));
  g.add(j);
  const spin = kind === "illusion" ? 1.6 : 1.0;
  return still(g, { top: [0, 0.95, 0], headTop: [0, 0.95, 0], chest: [0, 0.62, 0.2] }, (t) => {
    j.position.y = 0.62 + Math.sin(t * 2.2) * 0.06;
    j.rotation.y = t * spin;
    if (kind === "illusion") j.rotation.z = t * 1.2;
  });
}

// ------------------------------------------------------------ the table
export const MODELS = {
  creep_melee_sun: creepMeleeSun, creep_ranged_sun: creepRangedSun, creep_siege_sun: creepSiegeSun,
  creep_melee_moon: creepMeleeMoon, creep_ranged_moon: creepRangedMoon, creep_siege_moon: creepSiegeMoon,
  tower_sun: () => tower("sun"), tower_moon: () => tower("moon"),
  barracks_melee_sun: () => barracks("sun", "melee"), barracks_ranged_sun: () => barracks("sun", "ranged"),
  barracks_melee_moon: () => barracks("moon", "melee"), barracks_ranged_moon: () => barracks("moon", "ranged"),
  sphere_sun: () => sphere("sun"), sphere_moon: () => sphere("moon"),
  fountain_sun: () => fountain("sun"), fountain_moon: () => fountain("moon"),
  shop,
  hound, mossback, imp, golem: () => golem(true), golem_small: () => golem(false), shaman, hornet,
  drake: () => drake(false), drake_young: () => drake(true), warden, critter, courier,
  observer_ward: () => ward("observer"), sentry_ward: () => ward("sentry"), healing_ward: healingWard, fissure_rock: fissureRock,
  tree_sun: treeSun, tree_moon: treeMoon,
  rune_haste: () => rune("haste"), rune_double: () => rune("double"), rune_illusion: () => rune("illusion"), rune_invis: () => rune("invis"), rune_regen: () => rune("regen"),
};

export const NOTES = {
  creep_melee_sun: "Sunward spearman: read by the gold dome helm, the round gold shield and the upright spear.",
  creep_ranged_sun: "Sunward slinger: read by the tall pointed orange cap and the sling held out.",
  creep_siege_sun: "Sunward ballista: a round wooden cart on four wheels with a gold bow and a bolt laid on top, and a face on its front.",
  creep_melee_moon: "Moonward thrall: a hunched lilac brute read by its bone horn nubs and three bone claws on each hand.",
  creep_ranged_moon: "Moonward caster: a hooded violet robe read by the glowing violet spark held over its head.",
  creep_siege_moon: "Moonward catapult: a slate cart read by its raised bone arm with a rock in the cup.",
  tower_sun: "Sunward tower: a sandstone spire of rounded courses with a floating sun-gold orb in a ring of rays.",
  tower_moon: "Moonward tower: the same spire in slate with a floating, turning violet crystal.",
  barracks_melee_sun: "Sunward melee barracks: a sandstone dome with a round shield and crossed spears over the door.",
  barracks_ranged_sun: "Sunward ranged barracks: a sandstone drum under a cone roof with a target and arrow over the door.",
  barracks_melee_moon: "Moonward melee barracks: the dome with shield and spears, in slate and violet.",
  barracks_ranged_moon: "Moonward ranged barracks: the drum, cone roof and target, in slate and violet.",
  sphere_sun: "The Sunward Sphere: a huge glowing gold orb over a four-pillar pedestal, held in two turning rings.",
  sphere_moon: "The Moonward Sphere: the same in violet, the largest and brightest thing in its base.",
  fountain_sun: "Sunward fountain: a round basin of water with arcing spouts round a pillar topped by a gold light.",
  fountain_moon: "Moonward fountain: the basin and spouts with a violet light.",
  shop: "The shop: a market stall under a round red-and-cream striped canopy, pots and potions on the counter.",
  hound: "Ridge hound: a sandy pup with big pointed ears, a cream ruff and a curled tail.",
  mossback: "Mossback: a tortoise under a high brown domed shell with a moss cap and a pink flower.",
  imp: "Bramble imp: the smallest neutral, a green sprite with wide pointed ears, thorns and a leaf on its head.",
  golem: "Stone golem: a big round boulder body with moss shoulders, huge fists and small glowing mint eyes.",
  golem_small: "Small stone golem: the same golem at about two-thirds size, so the pair reads as a set.",
  shaman: "Hornet shaman: a gold-and-black striped bee-folk with antennae, curled horns, little wings and a honey-orb staff.",
  hornet: "Giant hornet: a round striped flying bug with buzzing wings, read by its hover and stinger.",
  drake: "Elder drake: a green round dragon with horns, an orange crest, open orange wings and a spade tail.",
  drake_young: "Young drake: a paler, smaller drake with stubby wings, so it reads as the elder's young.",
  warden: "The Warden: a huge round violet beast with curled horns, tusks, a studded stone collar and spikes down its back.",
  critter: "The hex critter: a tiny green toad with a pale belly and a smile; plainly harmless.",
  courier: "The courier: a small grey mule with long ears and saddlebags in the side's colour, a pennant on its back.",
  observer_ward: "Observer ward: a big gold-irised eye on a leafy stalk that slowly looks around.",
  sentry_ward: "Sentry ward: a blue flame in a little iron cage on three slate legs.",
  healing_ward: "Healing totem: a short carved pole with a smiling face, green bands and a pulsing green light.",
  fissure_rock: "Fissure rock: a cluster of rounded slate stones with a glowing crack and a tuft of moss.",
  tree_sun: "Sunward pine: three plump stacked cones of green on a short trunk.",
  tree_moon: "Moonward frost tree: round pale-blue crowns capped with snow on a twisted lilac trunk.",
  rune_haste: "Haste rune: a red tall thin diamond with two swept fins.",
  rune_double: "Double damage rune: two blue orbs side by side.",
  rune_illusion: "Illusion rune: three gold orbs circling a white centre.",
  rune_invis: "Invisibility rune: a half see-through lilac ring.",
  rune_regen: "Regeneration rune: a rounded green plus.",
};
