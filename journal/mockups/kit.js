// The style kit for the round-2 art mockups: materials, primitives, faces,
// rigs, stages and effect pieces. Not the game: the game draws with its own
// WebGL page from data/models.json (Builder D3). Every body here is built
// from the meshes the game's renderer already has (sphere, cylinder, cone,
// ring, card), so a model ports to tools/modelgen.py as data.
//
// Conventions: metres, y up, a model faces +z with its feet at y = 0.
// A hero stands about 1.5 tall; its head is about 40 % of that.
import * as THREE from "./vendor/three.module.js";
export { THREE };

// ---------------------------------------------------------------- the look
// `?look=ape` draws every model in the look Mage Tower Battle took from Ape
// Race in its round 7 (journal tab 10 here): faceted shapes with flat
// faces, no ink outline, light from the sky above and the ground below with
// a low sun, haze in the place's own colour, soft disc shadows instead of
// cast ones, and ground dressed at three sizes. Without it, round 2's look:
// smooth shapes, the plum outline, cast shadows.
export const LOOK = new URLSearchParams(globalThis.location?.search || "").get("look") || "round2";
export const APE = LOOK === "ape";

// ---------------------------------------------------------------- palette
// Pastel but saturated; the ink is a warm plum, never black.
export const C = {
  ink: 0x2b2140,
  eye: 0x241a30,
  white: 0xffffff,
  blush: 0xff8fa3,
  skin: 0xf6d2b4, skin2: 0xe8b48e, skin3: 0xc98d66, skinPale: 0xe9e2cf,
  // the sides
  sun: 0xf2b33d, sunDeep: 0xd9812b, sunCream: 0xfff1d6, sand: 0xe9cf9e,
  moon: 0x7a5cd6, moonDeep: 0x4f3a9e, lilac: 0xb7a4f2, slate: 0x6a7594,
  // materials
  steel: 0xc9d2de, steelDark: 0x8d97a8, gold: 0xf5c451, wood: 0x9a6a43, woodDark: 0x6e4a2f,
  leather: 0x8a5a3c, cloth: 0xf4efe6, stone: 0xbdb5a8, stoneDark: 0x8f877c, moss: 0x7cbf62,
  bone: 0xf1e8d4, iron: 0x5d6270,
  // elements
  frost: 0x9fe3ff, ice: 0xd4f6ff, fire: 0xff7b3a, flame: 0xffc04d, ember: 0xff4f3a,
  spark: 0x9ad8ff, bolt: 0xfff27a, poison: 0x9be36a, rot: 0x8fbf5a, arcane: 0xc58cff, blood: 0xe0435a,
  // ground
  grass: 0x8fd16f, grass2: 0x79c160, dirt: 0xd9b07c, dirt2: 0xc89a66, water: 0x7fd3f0,
  leaf: 0x5fb35a, leafDeep: 0x3f8f4f, frostLeaf: 0x9fc9e6, trunk: 0x9a6a43,
};

// ---------------------------------------------------------------- materials
const mats = new Map();
export function mat(color, o = {}) {
  const key = color + JSON.stringify(o);
  if (!mats.has(key)) mats.set(key, new THREE.MeshStandardMaterial({ color, roughness: APE ? 0.92 : 0.78, metalness: 0, flatShading: APE, ...o }));
  return mats.get(key);
}
export const shiny = (color) => mat(color, { roughness: 0.32, metalness: 0.15 });
export const glow = (color, k = 1.4) => mat(color, { emissive: color, emissiveIntensity: k, roughness: 0.4 });
// A see-through effect material: additive light, no depth write, no shadow.
export function light(color, opacity = 0.8) {
  return new THREE.MeshBasicMaterial({ color, transparent: true, opacity, blending: THREE.AdditiveBlending, depthWrite: false, toneMapped: false, side: THREE.DoubleSide });
}
// A see-through solid (ice, smoke): normal blending.
export function veil(color, opacity = 0.5, o = {}) {
  return new THREE.MeshStandardMaterial({ color, transparent: true, opacity, depthWrite: false, roughness: 0.3, ...o });
}
const INK = new THREE.MeshBasicMaterial({ color: C.ink, side: THREE.BackSide });

// ---------------------------------------------------------------- geometry
// In the new look a ball is an icosphere of 80 faces and a column has eight
// sides, so every face catches the light on its own (Ape Race's facets).
const SPH = APE ? new THREE.IcosahedronGeometry(1, 1) : new THREE.SphereGeometry(1, 28, 20);
const SPH_LO = APE ? new THREE.IcosahedronGeometry(1, 0) : new THREE.SphereGeometry(1, 12, 9);
const SIDES = APE ? 8 : 24;
const CYL = new THREE.CylinderGeometry(1, 1, 1, SIDES);
const CONE = new THREE.ConeGeometry(1, 1, SIDES);
const OCTA = new THREE.OctahedronGeometry(1, 0);
const ICO = new THREE.IcosahedronGeometry(1, 0);
const caps = new Map();
function capsule(r, len) {
  const k = r.toFixed(3) + ":" + len.toFixed(3);
  if (!caps.has(k)) caps.set(k, new THREE.CapsuleGeometry(r, Math.max(0.0001, len), APE ? 2 : 6, APE ? 7 : 16));
  return caps.get(k);
}
const tori = new Map();
function torusGeo(R, r, arc) {
  const k = [R, r, arc].map((v) => v.toFixed(3)).join(":");
  if (!tori.has(k)) tori.set(k, new THREE.TorusGeometry(R, r, APE ? 5 : 10, APE ? 16 : 36, arc));
  return tori.get(k);
}

// ---------------------------------------------------------------- the ink
// An outline is the back faces of the same shape, grown by `d` metres on
// each axis, drawn in the ink colour (Chumbles' look, after Long Time
// Comin'). `s` is the shape's size on each axis.
export const INK_D = 0.022;
function outline(m, s, d = INK_D) {
  // the new look has no outline: light on the facets tells shapes apart
  if (APE) return m;
  const o = new THREE.Mesh(m.geometry, INK);
  o.scale.set(1 + d / s[0], 1 + d / s[1], 1 + d / s[2]);
  o.castShadow = false;
  o.receiveShadow = false;
  o.userData.ink = true;
  m.add(o);
  return m;
}

function place(m, x, y, z, color, ink) {
  m.position.set(x, y, z);
  m.castShadow = true;
  m.receiveShadow = true;
  if (ink === false) return m;
  return m;
}
const asMat = (c) => (c && c.isMaterial ? c : mat(c));

// ---------------------------------------------------------------- primitives
// Every primitive returns a Mesh; `o.ink` (default true) adds the outline,
// `o.rot` is [x, y, z] in degrees.
function finish(m, o) {
  if (o.rot) m.rotation.set(...o.rot.map((d) => (d * Math.PI) / 180));
  if (o.shadow === false) m.castShadow = false;
  return m;
}
/** An ellipsoid of radii (rx, ry, rz) at (x, y, z). */
export function ball(rx, ry, rz, x, y, z, color, o = {}) {
  const m = new THREE.Mesh(o.lo ? SPH_LO : SPH, asMat(color));
  m.scale.set(rx, ry, rz);
  place(m, x, y, z);
  if (o.ink !== false) outline(m, [rx, ry, rz], o.d);
  return finish(m, o);
}
/** A sphere of radius r. */
export const orb = (r, x, y, z, color, o) => ball(r, r, r, x, y, z, color, o);
/** A capsule from a to b (arrays), radius r: limbs, blades, logs. */
export function limb(a, b, r, color, o = {}) {
  const A = new THREE.Vector3(...a), B = new THREE.Vector3(...b);
  const len = A.distanceTo(B);
  const m = new THREE.Mesh(capsule(r, len), asMat(color));
  m.position.copy(A).lerp(B, 0.5);
  m.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), B.clone().sub(A).normalize());
  m.castShadow = true;
  m.receiveShadow = true;
  if (o.ink !== false && !APE) {
    const g = new THREE.Mesh(capsule(r + (o.d ?? INK_D), len), INK);
    g.userData.ink = true;
    m.add(g);
  }
  if (o.flat) m.scale.set(1, 1, o.flat);
  return m;
}
/** An upright cylinder, radius r (or [rTop, rBottom]), height h, centred at y. */
export function cyl(r, h, x, y, z, color, o = {}) {
  const [rt, rb] = Array.isArray(r) ? r : [r, r];
  const geo = rt === rb ? CYL : new THREE.CylinderGeometry(rt, rb, 1, SIDES);
  const m = new THREE.Mesh(geo, asMat(color));
  const s = rt === rb ? [rt, h, rt] : [1, h, 1];
  m.scale.set(...s);
  place(m, x, y, z);
  if (o.ink !== false) outline(m, rt === rb ? s : [Math.max(rt, rb), h, Math.max(rt, rb)], o.d);
  return finish(m, o);
}
/** An upright cone, base radius r, height h, its base centre at y. */
export function cone(r, h, x, y, z, color, o = {}) {
  const m = new THREE.Mesh(CONE, asMat(color));
  m.scale.set(r, h, r);
  m.geometry = CONE;
  place(m, x, y, z);
  // the cone's geometry is centred; lift it so (x, y, z) is the base
  m.position.y += h / 2;
  if (o.ink !== false) outline(m, [r, h, r], o.d);
  return finish(m, o);
}
/** A gem: an octahedron of radii (rx, ry, rz). */
export function gem(rx, ry, rz, x, y, z, color, o = {}) {
  const m = new THREE.Mesh(o.ico ? ICO : OCTA, asMat(color));
  m.scale.set(rx, ry, rz);
  place(m, x, y, z);
  if (o.ink !== false) outline(m, [rx, ry, rz], o.d);
  return finish(m, o);
}
/** A torus arc in the xy plane: R the ring, r the tube, arc in degrees. */
export function hoop(R, r, x, y, z, color, o = {}) {
  const arc = ((o.arc ?? 360) * Math.PI) / 180;
  const m = new THREE.Mesh(torusGeo(R, r, arc), asMat(color));
  place(m, x, y, z);
  if (o.ink !== false && !APE) {
    const g = new THREE.Mesh(torusGeo(R, r + (o.d ?? INK_D), arc), INK);
    g.userData.ink = true;
    m.add(g);
  }
  return finish(m, o);
}
/** A flat disc on the ground (decals, shadows, rugs). */
export function disc(r, x, y, z, material) {
  const m = new THREE.Mesh(new THREE.CircleGeometry(r, 48), material);
  m.rotation.x = -Math.PI / 2;
  m.position.set(x, y, z);
  m.receiveShadow = true;
  return m;
}
/** A ring on the ground, inner r0, outer r1. */
export function groundRing(r0, r1, material, y = 0.02) {
  const m = new THREE.Mesh(new THREE.RingGeometry(r0, r1, 64), material);
  m.rotation.x = -Math.PI / 2;
  m.position.y = y;
  return m;
}

export function group(...kids) {
  const g = new THREE.Group();
  kids.flat().forEach((k) => k && g.add(k));
  return g;
}
/** A joint: a group whose origin is the pivot, so rotating it swings its parts. */
export function joint(x, y, z, ...kids) {
  const g = group(...kids);
  g.position.set(x, y, z);
  return g;
}

// ---------------------------------------------------------------- faces
// The face that makes everything read as cute: two dark dot eyes with a
// white catch-light, wide apart and low on the head, and blush beneath.
// (cx, cy, cz) is the head's centre, r its radius; the face looks along +z.
export function face(g, cx, cy, cz, r, o = {}) {
  const eye = o.eye ?? C.eye;
  const sep = o.sep ?? 0.36, h = o.h ?? 0.02, size = o.size ?? 0.14;
  for (const s of [-1, 1]) {
    const ex = cx + s * r * sep, ey = cy + r * h;
    const ez = cz + Math.sqrt(Math.max(0, 1 - sep * sep - h * h)) * r * 0.97;
    if (o.closed) {
      g.add(limb([ex - r * 0.1, ey, ez], [ex + r * 0.1, ey, ez], r * 0.03, eye, { ink: false }));
    } else {
      g.add(ball(r * size, r * size * (o.tall ?? 1.15), r * size * 0.6, ex, ey, ez, o.glowEye ? glow(eye, 1.8) : eye, { ink: false }));
      if (!o.glowEye) g.add(orb(r * size * 0.38, ex + r * size * 0.28, ey + r * size * 0.42, ez + r * size * 0.45, C.white, { ink: false }));
    }
    if (o.blush !== false) {
      const bm = veil(o.blushColor ?? C.blush, 0.55);
      g.add(ball(r * 0.13, r * 0.07, r * 0.05, cx + s * r * (sep + 0.16), cy - r * 0.2, cz + r * 0.86, bm, { ink: false }));
    }
  }
  if (o.mouth) {
    // a small smile: a short arc below the eyes
    const m = hoop(r * 0.09, r * 0.022, cx, cy - r * 0.26, cz + r * 0.95, C.eye, { arc: 180, rot: [0, 0, 180], ink: false });
    g.add(m);
  }
  return g;
}

// ---------------------------------------------------------------- rigs
// A rig is { root, body, head, armL, armR, legL, legR, hand, prop } of
// joints. Poses only rotate and move joints, so every animation reads the
// same on every hero. Missing joints are empty groups.
export function rig(root, parts = {}) {
  const empty = () => new THREE.Group();
  const r = { root, body: empty(), head: empty(), armL: empty(), armR: empty(), legL: empty(), legR: empty(), ...parts };
  r.rest = {};
  for (const k of ["body", "head", "armL", "armR", "legL", "legR"]) {
    r.rest[k] = { p: r[k].position.clone(), q: r[k].rotation.clone() };
  }
  r.rest.root = { p: root.position.clone(), q: root.rotation.clone(), s: root.scale.clone() };
  return r;
}
export function resetRig(r) {
  for (const k of ["body", "head", "armL", "armR", "legL", "legR"]) {
    r[k].position.copy(r.rest[k].p);
    r[k].rotation.copy(r.rest[k].q);
  }
}
const D = Math.PI / 180;
/** Idle: a slow bob and breath. */
export function idle(r, t, k = 1) {
  r.body.position.y = r.rest.body.p.y + Math.sin(t * 3.2) * 0.02 * k;
  r.head.rotation.z = r.rest.head.q.z + Math.sin(t * 1.7) * 3 * D * k;
  r.armL.rotation.x = r.rest.armL.q.x + Math.sin(t * 3.2) * 4 * D * k;
  r.armR.rotation.x = r.rest.armR.q.x - Math.sin(t * 3.2) * 4 * D * k;
}
/** Cast: the casting arm rises over `u` 0..1 (0 rest, 1 fully raised, forward). */
export function castPose(r, u, side = "R", how = "up") {
  const arm = side === "R" ? r.armR : r.armL;
  const sgn = side === "R" ? 1 : -1;
  if (how === "up") {
    arm.rotation.x = r.rest[side === "R" ? "armR" : "armL"].q.x - u * 150 * D;
    arm.rotation.z = r.rest[side === "R" ? "armR" : "armL"].q.z + sgn * u * 10 * D;
  } else if (how === "forward") {
    arm.rotation.x = r.rest[side === "R" ? "armR" : "armL"].q.x - u * 95 * D;
  } else if (how === "both") {
    r.armR.rotation.x = r.rest.armR.q.x - u * 150 * D;
    r.armL.rotation.x = r.rest.armL.q.x - u * 150 * D;
    r.armR.rotation.z = r.rest.armR.q.z + u * 25 * D;
    r.armL.rotation.z = r.rest.armL.q.z - u * 25 * D;
  }
  r.body.rotation.x = r.rest.body.q.x + u * 6 * D;
  r.head.rotation.x = r.rest.head.q.x - u * 8 * D;
}
/** A swing: wind up (u 0..0.5), strike (0.5..1) with the weapon arm. */
export function swing(r, u) {
  const a = u < 0.5 ? -ease(u * 2) * 120 : -120 + ease((u - 0.5) * 2) * 200;
  r.armR.rotation.x = r.rest.armR.q.x + a * D;
  r.body.rotation.y = r.rest.body.q.y + (u < 0.5 ? ease(u * 2) * 20 : 20 - ease((u - 0.5) * 2) * 45) * D;
}
/** A hop: squash on landing, stretch in the air. */
export function hop(r, u, height = 0.4) {
  const y = Math.sin(Math.PI * clamp01(u)) * height;
  r.root.position.y = r.rest.root.p.y + y;
}
/** Walk cycle at phase t (seconds). */
export function walk(r, t, k = 1) {
  const s = Math.sin(t * 9);
  r.legL.rotation.x = r.rest.legL.q.x + s * 30 * D * k;
  r.legR.rotation.x = r.rest.legR.q.x - s * 30 * D * k;
  r.armL.rotation.x = r.rest.armL.q.x - s * 25 * D * k;
  r.armR.rotation.x = r.rest.armR.q.x + s * 25 * D * k;
  r.body.position.y = r.rest.body.p.y + Math.abs(Math.cos(t * 9)) * 0.04 * k;
}
/** Squash and stretch the whole model: s > 1 tall and thin, s < 1 short and wide. */
export function squash(r, s) {
  const base = r.rest.root.s;
  r.root.scale.set(base.x / Math.sqrt(s), base.y * s, base.z / Math.sqrt(s));
}

// ---------------------------------------------------------------- timing
export const clamp01 = (x) => Math.max(0, Math.min(1, x));
export const ease = (x) => { x = clamp01(x); return x * x * (3 - 2 * x); };
export const easeOut = (x) => { x = clamp01(x); return 1 - (1 - x) * (1 - x); };
export const easeIn = (x) => { x = clamp01(x); return x * x; };
/** 0..1 across [a, b], clamped. */
export const span = (t, a, b) => clamp01((t - a) / (b - a));
/** 1 inside [a, b], with soft edges of width e. */
export const window_ = (t, a, b, e = 0.08) => Math.min(span(t, a - e, a + e), 1 - span(t, b - e, b + e));
export const lerp = (a, b, u) => a + (b - a) * u;
export function rng(seed) {
  let a = seed >>> 0;
  return () => {
    a |= 0; a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

// ---------------------------------------------------------------- effect pieces
// Each returns an Object3D the effect's update(t) moves, scales and fades.
// None casts a shadow.
function noShadow(o) { o.traverse((c) => { c.castShadow = false; }); return o; }

let SOFT;
function softTexture() {
  if (SOFT) return SOFT;
  const c = document.createElement("canvas");
  c.width = c.height = 128;
  const x = c.getContext("2d");
  const g = x.createRadialGradient(64, 64, 0, 64, 64, 64);
  g.addColorStop(0, "rgba(255,255,255,1)");
  g.addColorStop(0.35, "rgba(255,255,255,0.55)");
  g.addColorStop(1, "rgba(255,255,255,0)");
  x.fillStyle = g;
  x.fillRect(0, 0, 128, 128);
  SOFT = new THREE.CanvasTexture(c);
  return SOFT;
}
/** A soft glowing blob that always faces the camera. */
export function halo(color, size = 1, opacity = 0.9) {
  const s = new THREE.Sprite(new THREE.SpriteMaterial({ map: softTexture(), color, transparent: true, opacity, blending: THREE.AdditiveBlending, depthWrite: false, toneMapped: false }));
  s.scale.setScalar(size);
  return s;
}
/** A glowing ball with a halo: fireballs, sparks, orbs. */
export function glowBall(color, r = 0.15, core = C.white) {
  const g = group(orb(r * 0.55, 0, 0, 0, glow(core, 2), { ink: false }), orb(r, 0, 0, 0, light(color, 0.75), { ink: false }), halo(color, r * 6, 0.7));
  return noShadow(g);
}
/** A ground decal: a disc that glows (area of a spell). */
export function areaDecal(r, color, opacity = 0.35) {
  const g = group(disc(r, 0, 0.015, 0, light(color, opacity)), groundRing(r * 0.94, r, light(color, Math.min(1, opacity * 2.4)), 0.018));
  return noShadow(g);
}
/** A beam from a to b (Vector3s), radius w. */
export function beam(a, b, color, w = 0.08) {
  const len = a.distanceTo(b);
  const g = new THREE.Group();
  g.add(new THREE.Mesh(CYL, glow(C.white, 2.2)));
  g.children[0].scale.set(w * 0.45, len, w * 0.45);
  const outer = new THREE.Mesh(CYL, light(color, 0.6));
  outer.scale.set(w, len, w);
  g.add(outer);
  g.position.copy(a).lerp(b, 0.5);
  g.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), b.clone().sub(a).normalize());
  return noShadow(g);
}
/** A jagged bolt from a to b: k segments jittered by `jit`, seeded. */
export function lightning(a, b, color = C.bolt, seed = 1, k = 7, jit = 0.25, w = 0.05) {
  const R = rng(seed);
  const pts = [a.clone()];
  for (let i = 1; i < k; i++) {
    const p = a.clone().lerp(b, i / k);
    p.x += (R() - 0.5) * jit * 2; p.y += (R() - 0.5) * jit; p.z += (R() - 0.5) * jit * 2;
    pts.push(p);
  }
  pts.push(b.clone());
  const g = new THREE.Group();
  for (let i = 0; i < pts.length - 1; i++) g.add(beam(pts[i], pts[i + 1], color, w));
  return noShadow(g);
}
/** A ring that expands on the ground: a shockwave. */
export function shock(color, w = 0.12) {
  const m = new THREE.Mesh(new THREE.RingGeometry(1 - w, 1, 64), light(color, 0.9));
  m.rotation.x = -Math.PI / 2;
  m.position.y = 0.03;
  return noShadow(m);
}
/** Little stars circling a head: stunned (DotA's swirl, made cute). */
export function stunStars(r = 0.28, n = 3) {
  const g = new THREE.Group();
  for (let i = 0; i < n; i++) {
    const s = starMesh(0.07, C.bolt);
    s.userData.a = (i / n) * Math.PI * 2;
    g.add(s);
  }
  g.userData.r = r;
  g.userData.update = (t) => {
    g.children.forEach((s) => {
      const a = s.userData.a + t * 5;
      s.position.set(Math.cos(a) * r, Math.sin(a * 2) * 0.03, Math.sin(a) * r);
      s.rotation.z = t * 6;
    });
  };
  g.userData.update(0);
  return noShadow(g);
}
/** A chubby five-pointed star (a flat extruded shape). */
export function starMesh(r, color, depth = 0.04) {
  const sh = new THREE.Shape();
  for (let i = 0; i < 10; i++) {
    const a = (i / 10) * Math.PI * 2 + Math.PI / 2;
    const rr = i % 2 ? r * 0.5 : r;
    if (i === 0) sh.moveTo(Math.cos(a) * rr, Math.sin(a) * rr);
    else sh.lineTo(Math.cos(a) * rr, Math.sin(a) * rr);
  }
  const geo = new THREE.ExtrudeGeometry(sh, { depth, bevelEnabled: true, bevelSize: r * 0.15, bevelThickness: r * 0.15, bevelSegments: 2 });
  geo.center();
  const m = new THREE.Mesh(geo, glow(color, 1.3));
  return m;
}
/** Seeded particles: n soft sprites; `place(i, R)` gives each its fixed numbers. */
export function motes(n, color, size = 0.12, seed = 3) {
  const R = rng(seed);
  const g = new THREE.Group();
  for (let i = 0; i < n; i++) {
    const s = halo(color, size, 0.9);
    s.userData.r = [R(), R(), R(), R()];
    g.add(s);
  }
  return g;
}
/** A puff of smoke or cloud: a cluster of soft spheres. */
export function puff(color, r = 0.3, n = 5, seed = 5, opacity = 0.75) {
  const R = rng(seed);
  const g = new THREE.Group();
  for (let i = 0; i < n; i++) {
    const s = orb(r * (0.6 + R() * 0.5), (R() - 0.5) * r * 1.6, (R() - 0.3) * r * 0.8, (R() - 0.5) * r * 1.6, veil(color, opacity), { ink: false, lo: true });
    s.castShadow = false;
    g.add(s);
  }
  return g;
}
/** Sets every material's opacity under `o` to base × k (k 0..1). */
export function fade(o, k) {
  o.visible = k > 0.001;
  o.traverse((c) => {
    const ms = c.material ? (Array.isArray(c.material) ? c.material : [c.material]) : [];
    for (const m of ms) {
      if (m === INK) continue;
      if (m.userData.base === undefined) m.userData.base = m.opacity;
      if (!m.transparent) continue;
      m.opacity = m.userData.base * k;
    }
  });
}
/** Gives an effect object its own copies of its materials, so fading one leaves others alone. */
export function own(o) {
  o.traverse((c) => {
    if (c.material && c.material !== INK && !c.userData.ink) {
      c.material = c.material.clone();
      if (!c.material.transparent && c.material.isMeshStandardMaterial) {
        // solids fade too
        c.material.transparent = true;
      }
    }
  });
  return o;
}
/** Tints a model (e.g. frozen, hexed): mixes every standard material toward `color`. */
export function tint(o, color, k) {
  const tc = new THREE.Color(color);
  o.traverse((c) => {
    if (!c.material || c.userData.ink || !c.material.isMeshStandardMaterial) return;
    if (!c.userData.own) { c.material = c.material.clone(); c.userData.own = true; c.userData.c0 = c.material.color.clone(); c.userData.e0 = c.material.emissive.clone(); }
    c.material.color.copy(c.userData.c0).lerp(tc, k);
    c.material.emissive.copy(c.userData.e0).lerp(tc, k * 0.35);
  });
}

// ---------------------------------------------------------------- stages
export const W = 1280, H = 720;
export function makeRenderer(canvas, w = W, h = H) {
  const r = new THREE.WebGLRenderer({ canvas, antialias: true, preserveDrawingBuffer: true });
  r.setPixelRatio(1);
  r.setSize(w, h, false);
  // the new look casts no shadows: each thing stands on a soft dark disc
  r.shadowMap.enabled = !APE;
  r.shadowMap.type = THREE.PCFSoftShadowMap;
  r.toneMapping = THREE.ACESFilmicToneMapping;
  r.toneMappingExposure = 1.08;
  return r;
}
function skyTexture(top, bottom) {
  const c = document.createElement("canvas");
  c.width = 4; c.height = 256;
  const x = c.getContext("2d");
  const g = x.createLinearGradient(0, 0, 0, 256);
  g.addColorStop(0, top);
  g.addColorStop(1, bottom);
  x.fillStyle = g;
  x.fillRect(0, 0, 4, 256);
  const t = new THREE.CanvasTexture(c);
  t.colorSpace = THREE.SRGBColorSpace;
  return t;
}
// The new look's place: late afternoon over the forest. A clear sky above
// and the grass's green bounced up from below; a low warm sun from the
// Sunward corner; haze the colour of the far sky. One palette for the
// place; the sides' gold and violet stay the bright colours that matter to
// play (Ape Race's rule).
export const PLACE = {
  sky: ["#7fb6e8", "#f3d9b0"], skyLight: 0xcfe4ff, groundLight: 0x6f8f4a, sun: 0xffd9a8, haze: 0xe9d6b8,
  hemi: 0.95, key: 3.3, fill: 0.55, sunAt: [8, 5, 2.5],
};
/** A scene in the new look: no cast shadows, sky and ground light, a low sun, haze. */
export function makeApeScene({ fog = [18, 60], sunAt = PLACE.sunAt } = {}) {
  const scene = new THREE.Scene();
  scene.background = skyTexture(...PLACE.sky);
  scene.add(new THREE.HemisphereLight(PLACE.skyLight, PLACE.groundLight, PLACE.hemi));
  const sun = new THREE.DirectionalLight(PLACE.sun, PLACE.key);
  sun.position.set(...sunAt);
  scene.add(sun, sun.target);
  // the sun's light wrapping round: a weak warm light from the far side
  const wrap = new THREE.DirectionalLight(0xffc89a, PLACE.fill);
  wrap.position.set(-sunAt[0], sunAt[1] * 0.4, -sunAt[2]);
  scene.add(wrap);
  scene.fog = new THREE.Fog(PLACE.haze, fog[0], fog[1]);
  return { scene, sun };
}
/** A soft dark disc under a thing, the new look's shadow. */
export function blob(r, opacity = 0.3) {
  const m = disc(r, 0, 0.013, 0, new THREE.MeshBasicMaterial({ color: 0x1d2a14, transparent: true, opacity, depthWrite: false }));
  m.castShadow = false;
  m.renderOrder = 1;
  return m;
}
/** A scene with the round-2 light: a warm key with soft shadows, a sky fill. */
export function makeScene({ sky = ["#9fd4ff", "#e8f6ff"], shadowBox = 6, sunAt = [5, 9, 6], key = 2.3, fog } = {}) {
  if (APE) return makeApeScene({ fog: fog ?? [shadowBox * 3, shadowBox * 9] });
  const scene = new THREE.Scene();
  scene.background = skyTexture(...sky);
  scene.add(new THREE.HemisphereLight(0xe8f4ff, 0xb39a78, 1.1));
  const sun = new THREE.DirectionalLight(0xfff1dc, key);
  sun.position.set(...sunAt);
  sun.castShadow = true;
  sun.shadow.mapSize.set(2048, 2048);
  const b = shadowBox;
  Object.assign(sun.shadow.camera, { left: -b, right: b, top: b, bottom: -b, near: 0.5, far: 40 });
  sun.shadow.bias = -0.0004;
  sun.shadow.normalBias = 0.02;
  sun.shadow.radius = 4;
  scene.add(sun);
  scene.add(sun.target);
  const rim = new THREE.DirectionalLight(0xd9e4ff, 0.6);
  rim.position.set(-6, 4, -6);
  scene.add(rim);
  return { scene, sun };
}
/** Ground dressing at three sizes, seeded: boulders, tufts, flowers and
 * pebbles, each with its own shade, size and tilt, where `inside(x, z)`
 * holds over [x0, x1] × [z0, z1]. */
export function dress(g, n, x0, x1, z0, z1, seed, inside = () => true, big = 1) {
  const R = rng(seed);
  const shade = (c, k) => new THREE.Color(c).offsetHSL((R() - 0.5) * 0.03, (R() - 0.5) * 0.1, (R() - 0.5) * k).getHex();
  for (let i = 0; i < n; i++) {
    const x = x0 + R() * (x1 - x0), z = z0 + R() * (z1 - z0);
    if (!inside(x, z)) continue;
    const roll = R();
    if (roll < 0.12 * big) {
      const s = 0.22 + R() * 0.3;
      g.add(ball(s * 1.3, s * 0.8, s, x, s * 0.3, z, shade(C.stone, 0.12), { lo: true, rot: [R() * 30, R() * 360, R() * 30] }));
    } else if (roll < 0.55) {
      const c = shade(C.grass2, 0.1);
      for (let b = 0; b < 3; b++) {
        const a = R() * Math.PI * 2, l = 0.12 + R() * 0.12;
        g.add(cone(0.035, l, x + Math.cos(a) * 0.05, 0, z + Math.sin(a) * 0.05, c, { rot: [(R() - 0.5) * 40, 0, (R() - 0.5) * 40], shadow: false }));
      }
    } else if (roll < 0.72) {
      g.add(cyl(0.012, 0.16, x, 0.08, z, C.leafDeep, { shadow: false }));
      g.add(orb(0.04, x, 0.17, z, [0xfff1a8, 0xffb4c8, 0xffffff, 0xf2b33d][Math.floor(R() * 4)], { lo: true }));
    } else {
      g.add(ball(0.05 + R() * 0.05, 0.03, 0.045, x, 0.012, z, shade(C.stoneDark, 0.14), { lo: true, rot: [0, R() * 360, 0] }));
    }
  }
  return g;
}
/** A round grass diorama for one model, radius r. */
export function plinth(r = 1.6, top = C.grass, side = C.dirt) {
  const g = new THREE.Group();
  if (APE) {
    // faceted turf over an earth slab, dressed round the edge
    g.add(cyl([r * 0.96, r * 0.9], 0.34, 0, -0.2, 0, C.dirt2));
    const turf = new THREE.Mesh(new THREE.CircleGeometry(r, 18), mat(top));
    turf.rotation.x = -Math.PI / 2;
    turf.position.y = -0.02;
    g.add(turf);
    // no boulders on a model's own base: tufts, flowers and pebbles only
    dress(g, 30, -r, r, -r, r, 11, (x, z) => { const d = Math.hypot(x, z); return d > r * 0.62 && d < r * 0.93; }, 0);
    return g;
  }
  const t = cyl(r, 0.22, 0, -0.11, 0, side, { d: 0.03 });
  g.add(t);
  const lid = cyl(r * 0.985, 0.06, 0, -0.02, 0, top, { ink: false });
  g.add(lid);
  // a few tufts and pebbles round the edge, seeded
  const R = rng(11);
  for (let i = 0; i < 9; i++) {
    const a = R() * Math.PI * 2, d = r * (0.72 + R() * 0.22);
    if (R() < 0.6) g.add(ball(0.06 + R() * 0.05, 0.05, 0.06, Math.cos(a) * d, 0.02, Math.sin(a) * d, C.grass2, { lo: true }));
    else g.add(ball(0.05, 0.035, 0.045, Math.cos(a) * d, 0.02, Math.sin(a) * d, C.stone, { lo: true }));
  }
  return g;
}
/** A patch of lane for an ability: grass with a dirt path along x. */
export function lanePatch(w = 14, d = 10, seed = 21) {
  const g = new THREE.Group();
  if (APE) {
    // facets you can see: a coarse grid of grass whose corners rise and fall,
    // a worn path along x, dressed at three sizes off the path
    const R = rng(seed);
    const gr = new THREE.Mesh(new THREE.PlaneGeometry(w, d, Math.round(w * 1.4), Math.round(d * 1.4)), mat(C.grass));
    const pos = gr.geometry.attributes.position;
    for (let i = 0; i < pos.count; i++) {
      const y = pos.getY(i);
      pos.setZ(i, Math.abs(y) < 1.6 ? (R() - 0.5) * 0.04 : (R() - 0.5) * 0.22 + Math.min(0.4, (Math.abs(y) - 1.6) * 0.05));
    }
    gr.geometry.computeVertexNormals();
    gr.rotation.x = -Math.PI / 2;
    g.add(gr);
    const path = new THREE.Mesh(new THREE.PlaneGeometry(w, 2.8, Math.round(w), 3), mat(C.dirt));
    const pp = path.geometry.attributes.position;
    for (let i = 0; i < pp.count; i++) pp.setZ(i, (R() - 0.5) * 0.03);
    path.geometry.computeVertexNormals();
    path.rotation.x = -Math.PI / 2;
    path.position.y = 0.012;
    g.add(path);
    dress(g, Math.round(w * d * 0.55), -w / 2, w / 2, -d / 2, d / 2, seed + 1, (x, z) => Math.abs(z) > 1.6);
    return g;
  }
  const gr = new THREE.Mesh(new THREE.PlaneGeometry(w, d, 28, 20), mat(C.grass, { flatShading: true }));
  const pos = gr.geometry.attributes.position;
  const R = rng(seed);
  for (let i = 0; i < pos.count; i++) pos.setZ(i, (R() - 0.5) * 0.06);
  gr.geometry.computeVertexNormals();
  gr.rotation.x = -Math.PI / 2;
  gr.receiveShadow = true;
  g.add(gr);
  const path = new THREE.Mesh(new THREE.PlaneGeometry(w, 2.6), mat(C.dirt, { flatShading: true }));
  path.rotation.x = -Math.PI / 2;
  path.position.set(0, 0.008, 0);
  path.receiveShadow = true;
  g.add(path);
  for (let i = 0; i < 26; i++) {
    const x = (R() - 0.5) * w, z = (R() - 0.5) * d;
    if (Math.abs(z) < 1.5) continue;
    g.add(ball(0.08, 0.06, 0.08, x, 0.02, z, R() < 0.7 ? C.grass2 : C.stone, { lo: true }));
  }
  return g;
}
/** A camera looking at `at` from `dir` (unnormalised) at distance `dist`. */
export function aim(camera, at, dir, dist) {
  const d = new THREE.Vector3(...dir).normalize().multiplyScalar(dist);
  camera.position.set(at[0] + d.x, at[1] + d.y, at[2] + d.z);
  camera.lookAt(...at);
  camera.updateProjectionMatrix();
}
/** The game's camera angle (pitched like the match view), closer in. */
export const GAME_VIEW = [0, 0.9, 1.0];
/** The model-sheet angle: three-quarters from the front, a little above. */
export const SHEET_VIEW = [0.62, 0.42, 1];

// ---------------------------------------------------------------- units on a stage
/** A soft team ring under a unit's feet: gold for Sunward, violet for Moonward. */
export function teamRing(team, r = 0.42) {
  const col = team === "moon" ? C.moon : team === "neutral" ? 0xe0d6c0 : C.sun;
  const g = APE
    ? group(groundRing(r * 0.86, r, light(col, 0.7), 0.02), blob(r * 0.9, 0.32))
    : group(groundRing(r * 0.82, r, light(col, 0.85), 0.02), disc(r * 0.82, 0, 0.012, 0, veil(0x2b2140, 0.12)));
  g.traverse((c) => { c.castShadow = false; });
  return g;
}
/** World position of an anchor (Object3D) as a new Vector3. */
export function wpos(o) {
  o.updateWorldMatrix(true, false);
  return new THREE.Vector3().setFromMatrixPosition(o.matrixWorld);
}
/** A white flash on a hit: k 0..1. */
export const flash = (m, k) => tint(m.g ?? m, 0xffffff, k * 0.7);
/** Places a model on a stage: builds it, stands it at (x, z) facing angle
 * `ry` (degrees, 0 = +z), puts a team ring under it, adds it to the scene. */
export function unit(st, id, team, x, z, ry = 0, o = {}) {
  const m = st.build(id, team);
  const holder = new THREE.Group();
  holder.position.set(x, 0, z);
  holder.rotation.y = (ry * Math.PI) / 180;
  holder.add(m.g);
  if (o.ring !== false) holder.add(teamRing(team, o.ringR ?? 0.42));
  st.scene.add(holder);
  m.holder = holder;
  return m;
}
/** Faces a unit's holder toward a point (x, z). */
export function faceTo(m, x, z) {
  const h = m.holder;
  h.rotation.y = Math.atan2(x - h.position.x, z - h.position.z);
}
