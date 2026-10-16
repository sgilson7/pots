// Ability animations, heroes 6 to 10 (Tectus, Kestrel, Hexwright,
// Longwick, Tempest) and the Warden. Same contract as fx_a.js: each entry
// is { T seconds, still, fps, setup(st) -> { update(t), look?, view?, dist? } },
// every frame a pure function of t, t = 0 and t = T alike so the GIF loops.
// NOTES says what a player sees and which cue tells them.
import { THREE, C, mat, glow, shiny, light, veil, ball, orb, limb, cyl, cone, gem, hoop, group, unit, wpos, glowBall, halo, shock,
  stunStars, areaDecal, puff, fade, own, flash, lightning, groundRing, disc, starMesh, motes, tint,
  resetRig, idle, castPose, swing, hop, walk, span, ease, easeOut, easeIn, window_, lerp, rng, clamp01 } from "./kit.js";

const D = Math.PI / 180;
const V = (x, y, z) => new THREE.Vector3(x, y, z);
const UP = V(0, 1, 0);


// Saturated see-through materials with normal blending: the kit's additive
// light() turns every colour white over the pale grass.
function lit(color, opacity = 0.8) {
  return new THREE.MeshBasicMaterial({ color, transparent: true, opacity, depthWrite: false, toneMapped: false, side: THREE.DoubleSide });
}
const paler = (c, k = 0.55) => new THREE.Color(c).lerp(new THREE.Color(0xffffff), k).getHex();
function ringN(color, w = 0.12, opacity = 0.9) {
  const m = new THREE.Mesh(new THREE.RingGeometry(1 - w, 1, 64), lit(color, opacity));
  m.rotation.x = -Math.PI / 2;
  m.position.y = 0.03;
  m.castShadow = false;
  return m;
}
function groundRingN(r0, r1, material, y = 0.02) {
  const m = new THREE.Mesh(new THREE.RingGeometry(r0, r1, 64), material);
  m.rotation.x = -Math.PI / 2;
  m.position.y = y;
  return m;
}
function areaN(r, color, opacity = 0.3) {
  const g = group(disc(r, 0, 0.015, 0, lit(color, opacity)), groundRingN(r * 0.95, r, lit(color, 0.95), 0.018));
  g.traverse((c) => { c.castShadow = false; });
  return g;
}
/** A jagged bolt with a pale core in its own colour, seeded. */
function boltN(a, b, color = C.bolt, seed = 1, k = 7, jit = 0.25, w = 0.05) {
  const R = rng(seed);
  const pts = [a.clone()];
  for (let i = 1; i < k; i++) {
    const p = a.clone().lerp(b, i / k);
    p.x += (R() - 0.5) * jit * 2; p.y += (R() - 0.5) * jit; p.z += (R() - 0.5) * jit * 2;
    pts.push(p);
  }
  pts.push(b.clone());
  const geo = new THREE.CylinderGeometry(1, 1, 1, 8);
  const core = lit(paler(color, 0.7), 1), outer = lit(color, 0.75);
  const g = new THREE.Group();
  for (let i = 0; i < pts.length - 1; i++) {
    const a1 = pts[i], b1 = pts[i + 1];
    for (const [m, ww] of [[core, w * 0.45], [outer, w]]) {
      const c = new THREE.Mesh(geo, m);
      c.scale.set(ww, a1.distanceTo(b1) + ww, ww);
      c.position.copy(a1).lerp(b1, 0.5);
      c.quaternion.setFromUnitVectors(UP, b1.clone().sub(a1).normalize());
      c.castShadow = false;
      g.add(c);
    }
  }
  return g;
}

export const ABILITIES = {};
export const NOTES = {};

// ------------------------------------------------------------ helpers
/** An enemy lane creep, or a Moonward hero while the creeps are not built yet. */
function enemy(st, x, z, ry = 300, id = "creep_melee_moon") {
  try { return prep(unit(st, id, "moon", x, z, ry)); } catch (e) { return prep(unit(st, "hero_hexwright", "moon", x, z, ry)); }
}
/** A hero on a side. */
function hero(st, id, team, x, z, ry) { return prep(unit(st, id, team, x, z, ry)); }
/** Records a unit's height and rest pose so every frame can start from it. */
function prep(m) {
  const b = new THREE.Box3().setFromObject(m.g);
  m.h = Math.max(0.6, b.max.y - b.min.y);
  m.home = m.holder.position.clone();
  m.homeRy = m.holder.rotation.y;
  m.g.userData.p0 = m.g.position.clone();
  m.g.userData.r0 = m.g.rotation.clone();
  m.g.userData.s0 = m.g.scale.clone();
  return m;
}
/** Back to rest, then a gentle idle. */
function rest(m, t, k = 1) {
  m.holder.position.copy(m.home);
  m.holder.rotation.y = m.homeRy;
  m.holder.visible = true;
  m.g.position.copy(m.g.userData.p0);
  m.g.rotation.copy(m.g.userData.r0);
  m.g.scale.copy(m.g.userData.s0);
  if (m.rig) { resetRig(m.rig); idle(m.rig, t, k); }
  flash(m, 0);
}
const top = (m) => wpos(m.holder).add(V(0, m.h + 0.1, 0));
const chest = (m) => wpos(m.holder).add(V(0, m.h * 0.5, 0));
/** Squash (s < 1) or stretch (s > 1) a unit about its feet. */
function sq(m, s) { const b = m.g.userData.s0; m.g.scale.set(b.x / Math.sqrt(s), b.y * s, b.z / Math.sqrt(s)); }
/** Stun stars over a unit while `on`, with a dazed wobble. */
function stars(st, n = 1) {
  const out = [];
  for (let i = 0; i < n; i++) { const s = stunStars(0.3); st.scene.add(s); out.push(s); }
  return out;
}
function dazed(m, s, t, on) {
  s.visible = on;
  if (!on) return;
  s.position.copy(top(m));
  s.userData.update(t);
  m.g.rotation.z = m.g.userData.r0.z + Math.sin(t * 7) * 4 * D;
}
/** A beam whose ends are set every frame. */
function liveBeam(color, w = 0.08) {
  const geo = new THREE.CylinderGeometry(1, 1, 1, 12);
  const core = new THREE.Mesh(geo, lit(paler(color, 0.65), 1));
  const outer = new THREE.Mesh(geo, lit(color, 0.7));
  const g = group(core, outer);
  g.userData.w = w;
  g.traverse((c) => { c.castShadow = false; });
  return g;
}
function setBeam(g, a, b, w = g.userData.w) {
  const len = Math.max(0.001, a.distanceTo(b));
  g.position.copy(a).lerp(b, 0.5);
  g.quaternion.setFromUnitVectors(UP, b.clone().sub(a).normalize());
  g.children[0].scale.set(w * 0.4, len, w * 0.4);
  g.children[1].scale.set(w, len, w);
}
/** A glowing slash: a flat arc of light. */
function slashArc(color, R = 0.7, arc = 150) {
  const m = new THREE.Mesh(new THREE.TorusGeometry(R, 0.05, 6, 32, (arc * Math.PI) / 180), lit(color, 0.95));
  const core = new THREE.Mesh(new THREE.TorusGeometry(R, 0.02, 6, 32, (arc * Math.PI) / 180), lit(C.white, 1));
  const g = group(m, core);
  g.traverse((c) => { c.castShadow = false; });
  return g;
}
/** Tries a world.js model, falling back to a stand-in. */
function model(st, id, team, fallback) {
  try { const m = st.build(id, team); return m.g ?? m; } catch (e) { return fallback(); }
}
/** A ground crack: a dark strip with a glowing core. */
function crack(len, w, color) {
  const g = group(
    ball(len / 2, 0.012, w / 2, len / 2, 0.01, 0, mat(0x5a4f48), { ink: false }),
    ball(len / 2 * 0.95, 0.014, w / 5, len / 2, 0.014, 0, lit(color, 0.9), { ink: false }),
  );
  g.traverse((c) => { c.castShadow = false; });
  return g;
}
/** Dust puffs that billow out and fade: k 0..1 through their life. */
function dust(st, n, seed, color = 0xe9dcc6) {
  const ps = [];
  for (let i = 0; i < n; i++) { const p = own(puff(color, 0.22, 4, seed + i, 0.7)); st.scene.add(p); ps.push(p); }
  return ps;
}
function billow(p, at, k, rise = 0.5) {
  p.visible = k > 0 && k < 1;
  if (!p.visible) return;
  p.position.copy(at).add(V(0, rise * easeOut(k), 0));
  p.scale.setScalar(0.5 + easeOut(k) * 1.1);
  fade(p, 1 - k); p.visible = p.visible && (k > 0 && k < 1);
}
/** Little rising crosses: healing. */
function plus(color) {
  const g = group(limb([-0.07, 0, 0], [0.07, 0, 0], 0.03, glow(color, 1.2), { ink: false }), limb([0, -0.07, 0], [0, 0.07, 0], 0.03, glow(color, 1.2), { ink: false }));
  g.traverse((c) => { c.castShadow = false; });
  return g;
}

const EARTH = 0x5fd6b0, STONE = 0xa89c8c, STEEL = 0xdfe8f2, ORANGE = 0xff9a3c, HEX = 0xff6fd0, HEXV = 0xb06cff;
const BRASS = 0xf2c25a, SMOKE = 0xb8b0a6, ZAP = 0x8fd8ff;

// ============================================================ Tectus
// ------------------------------------------------------------ Q Rift Stomp
ABILITIES.fissure = {
  T: 3.6, still: 1.05, fps: 12,
  setup(st) {
    const tc = hero(st, "hero_tectus", "sun", -4, 0.2, 75);
    const e1 = enemy(st, -0.6, 0.3, 290), e2 = enemy(st, 1.8, -0.4, 280);
    const L = 7.2;
    const cr = crack(L, 0.5, EARTH);
    cr.position.set(-3.5, 0, 0);
    st.scene.add(cr);
    const rocks = [];
    const R = rng(4);
    for (let i = 0; i < 9; i++) {
      const r = model(st, "fissure_rock", "neutral", () => group(gem(0.3, 0.42, 0.28, 0, 0.3, 0, mat(STONE), { ico: true }), gem(0.16, 0.22, 0.15, 0.22, 0.16, 0.12, mat(0x8f877c), { ico: true })));
      r.userData.x = -3.2 + (i + 0.5) * (L / 9);
      r.userData.z = (R() - 0.5) * 0.3;
      r.userData.ry = R() * 6;
      r.userData.s = 0.8 + R() * 0.4;
      st.scene.add(r);
      rocks.push(r);
    }
    const ds = dust(st, 6, 30);
    const ss = stars(st, 2);
    return {
      look: [-0.6, 0.3, 0], dist: 10.5,
      update(t) {
        rest(tc, t); rest(e1, t); rest(e2, t);
        // the stomp: club up and down on the line's start
        const u = span(t, 0.1, 0.65);
        swing(tc.rig, u);
        if (t > 0.45 && t < 0.75) sq(tc, 1 - 0.12 * Math.sin(span(t, 0.45, 0.75) * Math.PI));
        // the crack runs out along the line, then the rocks rise in order
        const run = span(t, 0.6, 1.0);
        cr.scale.set(Math.max(0.001, run), 1, 1);
        fade(cr, run > 0 ? 1 - span(t, 3.0, 3.4) : 0);
        rocks.forEach((r, i) => {
          const at = 0.62 + (i / 9) * 0.38;
          const up = span(t, at, at + 0.2), down = span(t, 3.0, 3.4);
          const k = up < 1 ? easeOut(up) * 1.15 : 1 + Math.sin(span(t, at + 0.2, at + 0.35) * Math.PI) * -0.15;
          r.visible = up > 0 && down < 1;
          r.position.set(r.userData.x, (k - 1) * 0.6 - down * 0.7, r.userData.z);
          r.rotation.y = r.userData.ry;
          r.scale.setScalar(r.userData.s * Math.max(0.01, Math.min(1, k)));
        });
        ds.forEach((p, i) => billow(p, V(-2.8 + i * 1.2, 0.1, 0.5 - (i % 2)), span(t, 0.7 + i * 0.06, 1.8 + i * 0.06)));
        // both enemies on the line: hit, then stunned
        [e1, e2].forEach((e, i) => {
          const hitAt = 0.62 + ((e.home.x + 3.2) / L) * 0.38;
          const h = t - hitAt;
          if (h > 0 && h < 0.25) { flash(e, 1 - h * 4); e.holder.position.y = Math.sin(h * 4 * Math.PI) * 0.25; }
          dazed(e, ss[i], t, h > 0 && t < 2.8);
        });
      },
    };
  },
};
NOTES.fissure = {
  see: "Tectus brings his club down and a glowing crack runs straight out from his feet; stone blocks shove up out of it one after another. Enemies on the line hop as it reaches them and sit dazed with stars over their heads.",
  tell: "The crack draws the line before the rocks rise, the rocks stay as a wall for the blocking time, and the stars mark who is stunned. The line is about 7 m here against 12 in the game (shrunk to fit); the rocks' 8 s are cut to about 2.",
};

// ------------------------------------------------------------ W Horn Charge
ABILITIES.enchant_totem = {
  T: 3.2, still: 1.25, fps: 12,
  setup(st) {
    const tc = hero(st, "hero_tectus", "sun", -1.4, 0.3, 75);
    const e = enemy(st, 0.6, 0.1, 280);
    const glowHalo = halo(EARTH, 1.4, 0.9);
    const swirl = motes(10, EARTH, 0.16, 8);
    st.scene.add(glowHalo, swirl);
    const ring = ringN(EARTH, 0.18);
    st.scene.add(ring);
    const ss = stars(st, 1);
    return {
      look: [-0.3, 0.6, 0], dist: 9,
      update(t) {
        rest(tc, t); rest(e, t);
        // he lifts the club in both hands and it fills with light
        const up = window_(t, 0.2, 1.35, 0.15);
        castPose(tc.rig, up, "R", "both");
        const charged = span(t, 0.35, 0.8) * (t < 2.0 ? 1 : 0);
        const club = wpos(tc.at.club);
        glowHalo.position.copy(club);
        glowHalo.scale.setScalar(0.3 + charged * (1.2 + Math.sin(t * 12) * 0.12));
        fade(glowHalo, charged);
        swirl.children.forEach((s, i) => {
          const a = s.userData.r[0] * 6.28 + t * 4;
          s.position.copy(club).add(V(Math.cos(a) * 0.35, (s.userData.r[1] - 0.5) * 0.4, Math.sin(a) * 0.35));
          s.material.opacity = 0.9 * charged;
        });
        swirl.visible = charged > 0;
        // the empowered blow
        const sw = span(t, 1.4, 2.0);
        if (sw > 0 && sw < 1) swing(tc.rig, sw);
        const hit = span(t, 1.75, 2.25);
        ring.visible = hit > 0 && hit < 1;
        ring.position.set(e.home.x, 0.04, e.home.z);
        ring.scale.setScalar(0.3 + easeOut(hit) * 1.6);
        fade(ring, 1 - hit); ring.visible = ring.visible && (hit > 0 && hit < 1);
        if (t > 1.75 && t < 2.2) { flash(e, 1 - span(t, 1.75, 2.2)); sq(e, 1 - 0.3 * Math.sin(span(t, 1.75, 2.2) * Math.PI)); e.holder.position.x = e.home.x + easeOut(span(t, 1.75, 2.0)) * 0.4; }
        if (t >= 2.2 && t < 3.0) e.holder.position.x = e.home.x + 0.4 * (1 - span(t, 2.2, 3.0));
        dazed(e, ss[0], t, false);
      },
    };
  },
};
NOTES.enchant_totem = {
  see: "Tectus hoists his club over his head and it fills with swirling teal light, which stays on it. His next swing lands with a teal shockwave and the enemy is squashed and shoved back.",
  tell: "The glowing club means the charge is waiting; it goes out when the blow lands (or after 14 s, cut short here).",
};

// ------------------------------------------------------------ E Ground Tremor (passive)
ABILITIES.aftershock = {
  T: 2.8, still: 0.85, fps: 12,
  setup(st) {
    const tc = hero(st, "hero_tectus", "sun", 0, 0, 0);
    const es = [enemy(st, 1.9, 0.6, 250), enemy(st, -1.8, 0.9, 110), enemy(st, 0.6, -2.1, 180)];
    const far = enemy(st, 4.2, -0.6, 270);
    const ring = ringN(EARTH, 0.1), ring2 = ringN(STONE, 0.25);
    const zone = areaN(3.0, EARTH, 0.22);
    st.scene.add(ring, ring2, zone);
    const spark = glowBall(EARTH, 0.18);
    st.scene.add(spark);
    const ds = dust(st, 8, 50);
    const ss = stars(st, 3);
    return {
      look: [0.3, 0.3, 0], dist: 12,
      update(t) {
        rest(tc, t); es.forEach((e) => rest(e, t)); rest(far, t);
        // any spell: here a quick cast from his raised hand
        castPose(tc.rig, window_(t, 0.15, 0.55, 0.12), "L", "up");
        const sp = window_(t, 0.2, 0.55, 0.1);
        spark.visible = sp > 0.01;
        spark.position.copy(wpos(tc.at.handL));
        spark.scale.setScalar(sp);
        // the tremor: the ground jolts out from his feet
        const k = span(t, 0.55, 1.05);
        if (t > 0.55 && t < 0.8) sq(tc, 1 - 0.08 * Math.sin(span(t, 0.55, 0.8) * Math.PI));
        ring.visible = k > 0 && k < 1; ring.scale.setScalar(0.2 + easeOut(k) * 2.8); fade(ring, 1 - k * 0.7); ring.visible = ring.visible && (k > 0 && k < 1);
        const k2 = span(t, 0.6, 1.2);
        ring2.visible = k2 > 0 && k2 < 1; ring2.scale.setScalar(0.2 + easeOut(k2) * 2.6); fade(ring2, 1 - k2); ring2.visible = ring2.visible && (k2 > 0 && k2 < 1);
        fade(zone, window_(t, 0.55, 1.4, 0.1));
        ds.forEach((p, i) => { const a = (i / 8) * 6.28; billow(p, V(Math.cos(a) * 2.2, 0.05, Math.sin(a) * 2.2), span(t, 0.75, 1.7), 0.3); });
        es.forEach((e, i) => {
          const h = t - (0.55 + (e.home.length() / 3) * 0.5);
          if (h > 0 && h < 0.2) { flash(e, 1 - h * 5); e.holder.position.y = Math.sin(h * 5 * Math.PI) * 0.2; }
          dazed(e, ss[i], t, h > 0 && t < 2.2);
        });
      },
    };
  },
};
NOTES.aftershock = {
  see: "Whenever Tectus casts a spell, a teal ring jolts out from his feet with a ring of dust. Enemies near him bounce and get stars; one standing farther off is untouched.",
  tell: "It needs no button: the ring comes with every spell he casts, and its edge is the 3 m reach. The stun is short, so the stars soon go.",
};

// ------------------------------------------------------------ R Resonant Slam
ABILITIES.echo_slam = {
  T: 4.0, still: 1.3, fps: 12,
  setup(st) {
    const tc = hero(st, "hero_tectus", "sun", 0, 0, 0);
    const es = [enemy(st, 2.4, 0.4, 260), enemy(st, -2.2, 0.8, 100), enemy(st, 1.0, -2.6, 200), enemy(st, -1.0, -2.2, 160)];
    const big = ringN(EARTH, 0.08), big2 = ringN(0xffffff, 0.04);
    const zone = areaN(4.2, EARTH, 0.25);
    st.scene.add(big, big2, zone);
    const echoes = es.map(() => { const r = ringN(EARTH, 0.18); st.scene.add(r); return r; });
    // motes carry each echo to every other enemy
    const pairs = [];
    es.forEach((a, i) => es.forEach((b, j) => { if (i !== j) { const g = glowBall(EARTH, 0.1); st.scene.add(g); pairs.push({ g, a, b, d: (i * 3 + j) * 0.02 }); } }));
    const ds = dust(st, 6, 70, 0xe0d4be);
    return {
      look: [0, 0.3, -0.6], dist: 15,
      update(t) {
        rest(tc, t); es.forEach((e) => rest(e, t));
        // a hop up, then down hard with the club
        const j = span(t, 0.15, 0.8);
        tc.holder.position.y = Math.sin(Math.PI * j) * 0.7;
        castPose(tc.rig, window_(t, 0.15, 0.7, 0.12), "R", "both");
        if (t > 0.8 && t < 1.15) sq(tc, 1 - 0.25 * Math.sin(span(t, 0.8, 1.15) * Math.PI));
        // the first slam
        const k = span(t, 0.8, 1.5);
        big.visible = k > 0 && k < 1; big.scale.setScalar(0.3 + easeOut(k) * 4.0); fade(big, 1 - k * 0.6); big.visible = big.visible && (k > 0 && k < 1);
        big2.visible = big.visible; big2.scale.setScalar(0.3 + easeOut(k) * 3.9); fade(big2, 1 - k); big2.visible = big2.visible && (big.visible);
        fade(zone, window_(t, 0.8, 2.9, 0.15));
        ds.forEach((p, i) => { const a = (i / 6) * 6.28 + 0.4; billow(p, V(Math.cos(a) * 1.2, 0.05, Math.sin(a) * 1.2), span(t, 0.85, 1.8), 0.4); });
        // each enemy rings, and its echo travels to every other one
        es.forEach((e, i) => {
          const h = t - (0.85 + i * 0.05);
          if (h > 0 && h < 0.2) flash(e, 1 - h * 5);
          const r = echoes[i];
          const ek = span(t, 1.25 + i * 0.08, 1.85 + i * 0.08);
          r.visible = ek > 0 && ek < 1;
          r.position.set(e.home.x, 0.05, e.home.z);
          r.scale.setScalar(0.2 + easeOut(ek) * 1.0);
          fade(r, 1 - ek); r.visible = r.visible && (ek > 0 && ek < 1);
          // hit again once per other enemy's echo
          for (let n = 0; n < 3; n++) { const hh = t - (1.75 + n * 0.18 + i * 0.02); if (hh > 0 && hh < 0.12) flash(e, 0.8); }
          if (t > 0.85 && t < 2.4) e.g.position.y = e.g.userData.p0.y + Math.abs(Math.sin((t - 0.85) * 18)) * 0.05 * (1 - span(t, 0.85, 2.4));
        });
        pairs.forEach(({ g, a, b, d }) => {
          const u = span(t, 1.3 + d, 1.9 + d);
          g.visible = u > 0 && u < 1;
          g.position.copy(chest(a)).lerp(chest(b), ease(u));
          g.position.y += Math.sin(u * Math.PI) * 0.6;
        });
      },
    };
  },
};
NOTES.echo_slam = {
  see: "Tectus hops and slams the ground; a wide teal shockwave sweeps out. Each enemy it hits gives off its own small ring, and beads of teal light arc from each enemy to every other one, flashing them again as they land.",
  tell: "The big ring is the 5 m reach. The number of bead arcs shows the extra damage: the more enemies stand together, the more each is hit again.",
};

// ============================================================ Kestrel
// ------------------------------------------------------------ Q Whirling Edge
ABILITIES.blade_fury = {
  T: 3.6, still: 1.4, fps: 12,
  setup(st) {
    const k = hero(st, "hero_kestrel", "sun", -0.4, 0.2, 60);
    const es = [enemy(st, 1.0, 0.6, 290), enemy(st, 0.8, -1.2, 230), enemy(st, -1.8, -0.6, 120)];
    const rings = [0, 1, 2].map((i) => { const s = slashArc(i === 1 ? ORANGE : STEEL, 1.3 + i * 0.25, 230); s.rotation.x = -Math.PI / 2; st.scene.add(s); return s; });
    const funnel = own(cone(1.5, 1.6, 0, 0, 0, veil(STEEL, 0.22), { ink: false }));
    funnel.rotation.x = Math.PI;
    st.scene.add(funnel);
    const zone = areaN(2.4, ORANGE, 0.18);
    const shield = orb(0.85, 0, 0, 0, lit(0xfff3c4, 0.18), { ink: false });
    st.scene.add(zone, shield);
    const ds = dust(st, 6, 90);
    return {
      look: [0, 0.4, 0], dist: 10,
      update(t) {
        rest(k, t); es.forEach((e) => rest(e, t));
        const on = window_(t, 0.35, 3.05, 0.15);
        const spin = t < 0.35 ? 0 : t < 3.05 ? (t - 0.35) * 22 : (3.05 - 0.35) * 22;
        k.holder.rotation.y = k.homeRy + spin;
        castPose(k.rig, on * 0.6, "R", "forward");
        k.rig.armL.rotation.z -= on * 70 * D;
        k.holder.position.x = k.home.x + Math.sin(span(t, 0.35, 3.05) * Math.PI) * 0.6;
        const c = k.holder.position;
        rings.forEach((r, i) => {
          r.visible = on > 0.05;
          r.position.set(c.x, 0.35 + i * 0.25, c.z);
          r.rotation.z = -t * (14 + i * 3);
          r.scale.setScalar(0.6 + on * 0.4);
          fade(r, on); r.visible = r.visible && (on > 0.05);
        });
        funnel.visible = on > 0.05;
        funnel.position.set(c.x, 1.6, c.z);
        funnel.scale.set(1.5 * on, 1.6, 1.5 * on);
        funnel.rotation.y = t * 9;
        fade(funnel, on); funnel.visible = funnel.visible && (on > 0.05);
        zone.position.set(c.x, 0, c.z); fade(zone, on);
        shield.position.set(c.x, 0.75, c.z); fade(shield, on * (0.6 + 0.4 * Math.sin(t * 15)));
        ds.forEach((p, i) => billow(p, V(c.x + Math.cos(i) * 1.6, 0.05, c.z + Math.sin(i) * 1.6), ((t * 1.6 + i / 6) % 1) * (on > 0.3 ? 1 : 0), 0.3));
        es.forEach((e, i) => {
          if (e.home.distanceTo(c) < 2.5 && on > 0.5) flash(e, Math.max(0, Math.sin(t * 20 + i * 2)));
        });
      },
    };
  },
};
NOTES.blade_fury = {
  see: "Kestrel spins in place inside a whirl of steel and orange blade-light, drifting as he spins. A pale shimmer wraps him, and enemies inside the orange circle flicker white over and over.",
  tell: "The whirl itself is the cue: while it turns he cannot be hurt by spells (the shimmer) and everything inside the 2.5 m circle takes damage. It stops after 5 s (cut to 2.7 here).",
};

// ------------------------------------------------------------ W Mending Totem
ABILITIES.healing_ward = {
  T: 4.0, still: 1.7, fps: 12,
  setup(st) {
    const k = hero(st, "hero_kestrel", "sun", -1.2, 0.4, 70);
    const ally = hero(st, "hero_rime", "sun", 1.2, -0.6, 300);
    const foe = enemy(st, 4.0, 0.2, 280);
    const ward = model(st, "healing_ward", "sun", () => group(
      limb([0, 0, 0], [0, 0.75, 0], 0.07, C.wood), orb(0.18, 0, 0.88, 0, glow(0x8af07a, 0.9), { ink: false }),
      ball(0.16, 0.12, 0.16, 0, 0.62, 0, mat(0xf4efe6)), hoop(0.14, 0.03, 0, 0.88, 0, C.gold, { rot: [90, 0, 0] })));
    st.scene.add(ward);
    const zone = areaN(3.0, 0x8af07a, 0.28);
    const pulse = ringN(0x8af07a, 0.08);
    st.scene.add(zone, pulse);
    const crosses = [];
    for (let i = 0; i < 10; i++) { const p = plus(0x8af07a); st.scene.add(p); crosses.push(p); }
    return {
      look: [0.3, 0.5, 0], dist: 12,
      update(t) {
        rest(k, t); rest(ally, t); rest(foe, t);
        castPose(k.rig, window_(t, 0.15, 0.6, 0.1), "L", "forward");
        // the totem drops where he points, then follows him as he walks
        const drop = span(t, 0.4, 0.75);
        const walkU = span(t, 1.8, 3.4);
        if (walkU > 0 && walkU < 1) walk(k.rig, t);
        k.holder.position.x = k.home.x + ease(walkU) * 1.0;
        const wx = 0.0 + ease(span(t, 1.95, 3.55)) * 1.0;
        const land = span(t, 0.75, 0.95);
        ward.visible = drop > 0 && t < 3.75;
        ward.position.set(wx, (1 - easeIn(drop)) * 2.5 + Math.sin(land * Math.PI) * 0.12, 0.6);
        ward.scale.setScalar(t > 3.55 ? Math.max(0.01, 1 - span(t, 3.55, 3.75)) : 1);
        const on = window_(t, 0.85, 3.6, 0.1);
        zone.position.set(wx, 0, 0.6); fade(zone, on);
        const pk = ((t - 0.85) % 0.9) / 0.9;
        pulse.visible = on > 0.5; pulse.position.set(wx, 0.04, 0.6); pulse.scale.setScalar(0.3 + pk * 2.7); fade(pulse, (1 - pk) * on); pulse.visible = pulse.visible && (on > 0.5);
        crosses.forEach((p, i) => {
          const who = i % 2 ? ally : k;
          const u = ((t * 0.9 + i * 0.21) % 1);
          p.visible = on > 0.5;
          p.position.copy(wpos(who.holder)).add(V(Math.cos(i * 2.1) * 0.35, 0.3 + u * 1.4, Math.sin(i * 2.1) * 0.35));
          p.scale.setScalar(Math.sin(u * Math.PI));
        });
        // tint the healed ones a little green in time with the pulse
        tint(ally.g, 0x8af07a, on * (1 - pk) * 0.25);
      },
    };
  },
};
NOTES.healing_ward = {
  see: "Kestrel points and a little totem drops onto the ground with a bounce. A soft green circle spreads round it, pulsing outward, and green crosses rise off allies inside; when Kestrel walks on, the totem trots after him.",
  tell: "The green circle is the 5 m healing area (shown at 3 m), the pulse is each heal, and the totem's presence is the timer; it pops after 25 s (cut to about 3).",
};

// ------------------------------------------------------------ E Keen Edge (passive)
ABILITIES.blade_dance = {
  T: 3.0, still: 1.95, fps: 12,
  setup(st) {
    const k = hero(st, "hero_kestrel", "sun", -0.8, 0.3, 75);
    const e = enemy(st, 0.6, 0.0, 280);
    const small = slashArc(STEEL, 0.55, 110), big = slashArc(ORANGE, 1.1, 170);
    st.scene.add(small, big);
    const burst = starMesh(0.35, 0xffd166, 0.06);
    burst.castShadow = false;
    st.scene.add(burst);
    const ring = ringN(ORANGE, 0.15);
    st.scene.add(ring);
    return {
      look: [0, 0.6, 0], dist: 8,
      update(t) {
        rest(k, t); rest(e, t);
        // a plain strike, then the critical one
        const s1 = span(t, 0.2, 0.8), s2 = span(t, 1.3, 1.95);
        if (s1 > 0 && s1 < 1) swing(k.rig, s1);
        if (s2 > 0 && s2 < 1) { swing(k.rig, s2); sq(k, 1 + 0.12 * Math.sin(s2 * Math.PI)); }
        const c = chest(e);
        const h1 = span(t, 0.6, 0.95);
        small.visible = h1 > 0 && h1 < 1; small.position.copy(c); small.rotation.set(0, 0, 0.6 + h1 * 1.5); fade(small, 1 - h1); small.visible = small.visible && (h1 > 0 && h1 < 1);
        if (h1 > 0 && h1 < 0.5) flash(e, (1 - h1 * 2) * 0.5);
        const h2 = span(t, 1.75, 2.35);
        big.visible = h2 > 0 && h2 < 1; big.position.copy(c); big.rotation.set(0.3, 0.4, -0.5 + h2 * 2.0); big.scale.setScalar(0.7 + h2 * 0.5); fade(big, 1 - h2); big.visible = big.visible && (h2 > 0 && h2 < 1);
        burst.visible = h2 > 0 && h2 < 0.7; burst.position.copy(c).add(V(0, 0.4, 0.3)); burst.scale.setScalar(0.5 + easeOut(span(t, 1.75, 2.0)) * 1.0); burst.rotation.z = h2 * 3;
        ring.visible = h2 > 0 && h2 < 1; ring.position.set(e.home.x, 0.04, e.home.z); ring.scale.setScalar(0.3 + h2 * 1.4); fade(ring, 1 - h2); ring.visible = ring.visible && (h2 > 0 && h2 < 1);
        if (h2 > 0 && h2 < 0.6) { flash(e, 1 - h2 / 0.6); sq(e, 1 - 0.25 * Math.sin(span(t, 1.75, 2.15) * Math.PI)); e.holder.position.x = e.home.x + easeOut(span(t, 1.75, 2.0)) * 0.5 * (1 - span(t, 2.1, 2.9)); }
        else if (t >= 2.35) e.holder.position.x = e.home.x + 0.5 * (1 - span(t, 2.1, 2.9));
      },
    };
  },
};
NOTES.blade_dance = {
  see: "Kestrel's ordinary cut leaves a thin steel arc. On a critical hit he leans into the swing and a wide orange arc tears across the enemy, with a gold star burst, a ground ring and the enemy squashed and knocked back.",
  tell: "Size and colour tell a critical from a plain hit: thin steel is normal, the big orange arc with the star is double damage. It comes on its own at random.",
};

// ------------------------------------------------------------ R Thousand Cuts
ABILITIES.omnislash = {
  T: 4.2, still: 1.62, fps: 12,
  setup(st) {
    const k = hero(st, "hero_kestrel", "sun", -3.6, 0.4, 70);
    const es = [enemy(st, -0.4, 0.2, 280), hero(st, "hero_cinder", "moon", 1.6, -0.8, 300), enemy(st, 2.0, 1.0, 250)];
    const order = [0, 1, 2, 0, 1];
    const cuts = order.map((_, i) => { const a = slashArc(ORANGE, 1.0, 160), b = slashArc(i % 2 ? ORANGE : STEEL, 1.0, 160); const g = group(a, b); st.scene.add(g); return g; });
    const trail = [0, 1, 2, 3].map(() => { const h = halo(ORANGE, 1.2, 0.6); st.scene.add(h); return h; });
    const T0 = 0.6, dt = 0.4;
    const posAt = (t) => {
      if (t < T0) return k.home.clone().lerp(es[0].home, ease(span(t, 0.35, T0)) * 0.0);
      const n = Math.floor((t - T0) / dt);
      if (n >= order.length) {
        const last = es[order[order.length - 1]].home.clone().add(V(-0.7, 0, 0.2));
        return last.lerp(k.home, ease(span(t, T0 + order.length * dt, T0 + order.length * dt + 0.5)));
      }
      const from = n === 0 ? k.home : es[order[n - 1]].home.clone().add(V(-0.7, 0, 0.2));
      const to = es[order[n]].home.clone().add(V(-0.7, 0, 0.2));
      return from.clone().lerp(to, easeOut(span((t - T0) % dt, 0, 0.12)));
    };
    return {
      look: [-0.6, 0.4, 0], dist: 12.5,
      update(t) {
        rest(k, t); es.forEach((e) => rest(e, t));
        castPose(k.rig, window_(t, 0.2, 0.55, 0.1), "R", "up");
        const p = posAt(t);
        k.holder.position.copy(p);
        const n = Math.floor((t - T0) / dt), within = ((t - T0) % dt) / dt;
        const active = t > T0 && n < order.length;
        if (active) {
          k.holder.position.y = Math.sin(Math.min(1, within / 0.3) * Math.PI) * 0.35;
          const tgt = es[order[n]].home;
          k.holder.rotation.y = Math.atan2(tgt.x - p.x, tgt.z - p.z);
          swing(k.rig, clamp01(within * 1.3));
          // a blurred, half-there body between strikes
          k.g.visible = within > 0.15;
        }
        trail.forEach((h, i) => {
          h.visible = active;
          const q = posAt(t - 0.03 * (i + 1));
          h.position.copy(q).add(V(0, 0.7, 0));
          h.material.opacity = 0.5 * (1 - i / 4);
        });
        cuts.forEach((c, i) => {
          const u = span(t, T0 + i * dt + 0.1, T0 + i * dt + 0.45);
          c.visible = u > 0 && u < 1;
          c.position.copy(chest(es[order[i]]));
          c.children[0].rotation.set(0, 0, 0.8 + u * 1.2);
          c.children[1].rotation.set(0, 0, -0.8 - u * 1.2 + Math.PI);
          c.scale.setScalar(0.8 + u * 0.4);
          fade(c, 1 - u); c.visible = c.visible && (u > 0 && u < 1);
          const h = t - (T0 + i * dt + 0.1);
          if (h > 0 && h < 0.2) { flash(es[order[i]], 1 - h * 5); sq(es[order[i]], 0.85); }
        });
      },
    };
  },
};
NOTES.omnislash = {
  see: "Kestrel raises his sword and vanishes into an orange streak, blinking from enemy to enemy. Each arrival leaves a crossed slash on its target, which flashes and flinches; after the last strike he returns.",
  tell: "The orange streak jumping between nearby enemies is the whole cue, and the count of crossed slashes is the count of strikes (five here, eight at the top level), one every 0.4 s.",
};

// ============================================================ Hexwright
// ------------------------------------------------------------ Q Thornburst
ABILITIES.earth_spike = {
  T: 3.2, still: 0.95, fps: 12,
  setup(st) {
    const h = hero(st, "hero_hexwright", "sun", -3.4, 0.2, 75);
    const e1 = enemy(st, -0.8, 0.1, 290), e2 = enemy(st, 1.4, -0.2, 280);
    const spikes = [];
    for (let i = 0; i < 12; i++) {
      const s = group(cone(0.17, 0.7, 0, 0, 0, mat(0x7a6a8e)), cone(0.09, 0.42, 0.18, 0, 0.1, mat(0x9a86b0)), cone(0.08, 0.36, -0.15, 0, -0.1, mat(0x9a86b0)));
      s.userData.x = -2.9 + i * 0.5;
      st.scene.add(s);
      spikes.push(s);
    }
    const glowLine = crack(6.0, 0.3, HEX);
    glowLine.position.set(-3.0, 0, 0);
    st.scene.add(glowLine);
    const ss = stars(st, 2);
    return {
      look: [-0.6, 0.3, 0], dist: 12,
      update(t) {
        rest(h, t); rest(e1, t); rest(e2, t);
        castPose(h.rig, window_(t, 0.1, 0.5, 0.1), "R", "forward");
        fade(glowLine, window_(t, 0.35, 2.4, 0.15));
        glowLine.scale.x = Math.max(0.001, span(t, 0.35, 0.95));
        spikes.forEach((s, i) => {
          const at = 0.4 + i * 0.05;
          const u = span(t, at, at + 0.15), dn = span(t, 1.9 + i * 0.02, 2.3 + i * 0.02);
          s.visible = u > 0 && dn < 1;
          s.position.set(s.userData.x, -0.7 * (1 - easeOut(u)) - 0.75 * dn, (i % 2 - 0.5) * 0.25);
        });
        [e1, e2].forEach((e, i) => {
          const at = 0.4 + ((e.home.x + 2.9) / 0.5) * 0.05;
          const air = span(t, at, at + 0.52);
          if (air > 0 && air < 1) { e.holder.position.y = Math.sin(air * Math.PI) * 0.9; e.g.rotation.z = air * 0.6; }
          if (t > at && t < at + 0.2) flash(e, 1 - (t - at) * 5);
          dazed(e, ss[i], t, air >= 1 && t < 2.8);
        });
      },
    };
  },
};
NOTES.earth_spike = {
  see: "Hexwright thrusts his staff and a pink seam runs along the ground; stone thorns punch up along it, front to back. Enemies on the line are tossed into the air, land and sit stunned under stars.",
  tell: "The thorns rising in order show the line and its speed; being thrown up means you were caught. The line is 6 m here (7 in the game).",
};

// ------------------------------------------------------------ W Toadcurse
ABILITIES.hex = {
  T: 3.8, still: 1.6, fps: 12,
  setup(st) {
    const h = hero(st, "hero_hexwright", "sun", -2.4, 0.3, 70);
    const e = hero(st, "hero_ironvow", "moon", 1.6, 0.0, 300);
    const critter = model(st, "critter", "moon", () => group(
      ball(0.22, 0.17, 0.24, 0, 0.17, 0, mat(0x8fd16f)), orb(0.06, -0.09, 0.32, 0.12, mat(C.white)), orb(0.06, 0.09, 0.32, 0.12, mat(C.white)),
      orb(0.03, -0.09, 0.33, 0.17, mat(C.eye), { ink: false }), orb(0.03, 0.09, 0.33, 0.17, mat(C.eye), { ink: false })));
    const ch = new THREE.Group(); ch.add(critter); st.scene.add(ch);
    const bolt = glowBall(HEX, 0.2);
    st.scene.add(bolt);
    const poofs = [own(puff(0xf3c8ff, 0.4, 7, 11, 0.85)), own(puff(0xf3c8ff, 0.4, 7, 12, 0.85))];
    poofs.forEach((p) => st.scene.add(p));
    const sparks = motes(8, HEX, 0.2, 5);
    st.scene.add(sparks);
    return {
      look: [-0.3, 0.5, 0], dist: 10,
      update(t) {
        rest(h, t); rest(e, t);
        castPose(h.rig, window_(t, 0.1, 0.55, 0.1), "R", "forward");
        const fly = span(t, 0.45, 0.9);
        bolt.visible = fly > 0 && fly < 1;
        const a = wpos(h.at.tip), b = chest(e);
        bolt.position.copy(a).lerp(b, fly);
        bolt.position.y += Math.sin(fly * Math.PI) * 0.4;
        const hexed = t > 0.95 && t < 3.15;
        e.holder.visible = !hexed;
        ch.visible = hexed;
        ch.position.copy(e.home);
        const hp = (t * 2.2) % 1;
        ch.position.y = Math.abs(Math.sin(hp * Math.PI)) * 0.25;
        ch.rotation.y = -0.9 + Math.sin(t * 2) * 0.4;
        critter.scale.set(1 + (1 - Math.sin(hp * Math.PI)) * 0.15, 1 - (1 - Math.sin(hp * Math.PI)) * 0.15, 1);
        [0.95, 3.15].forEach((at, i) => billow(poofs[i], e.home.clone().add(V(0, 0.4, 0)), span(t, at - 0.05, at + 0.55), 0.3));
        sparks.visible = hexed;
        sparks.children.forEach((s, i) => {
          const an = s.userData.r[0] * 6.28 + t * 2;
          s.position.copy(ch.position).add(V(Math.cos(an) * 0.4, 0.5 + Math.sin(t * 3 + i) * 0.1, Math.sin(an) * 0.4));
          s.material.opacity = 0.6 * (0.5 + 0.5 * Math.sin(t * 6 + i));
        });
      },
    };
  },
};
NOTES.hex = {
  see: "Hexwright flicks a pink spark at an enemy hero; it bursts in a cloud of lilac smoke and the hero is gone, replaced by a little hopping critter with pink sparkles round it. Another puff, and the hero is back.",
  tell: "The critter is the status: while it hops, that hero can do nothing. The two puffs mark the start and end (3.5 s at the top level, 2.2 s here).",
};

// ------------------------------------------------------------ E Soul Siphon
ABILITIES.mana_drain = {
  T: 3.6, still: 1.6, fps: 12,
  setup(st) {
    const h = hero(st, "hero_hexwright", "sun", -2.2, 0.4, 70);
    const e = hero(st, "hero_rime", "moon", 1.8, -0.2, 300);
    const tether = liveBeam(HEXV, 0.07);
    st.scene.add(tether);
    const beads = motes(10, 0x6fb8ff, 0.22, 21);
    st.scene.add(beads);
    const well = orb(0.17, 0, 0, 0, lit(0x6fb8ff, 0.8), { ink: false });
    const wellH = halo(0x6fb8ff, 0.5, 0.5);
    st.scene.add(well, wellH);
    const cup = halo(HEXV, 1.2, 0.8);
    st.scene.add(cup);
    return {
      look: [-0.2, 0.6, 0], dist: 10,
      update(t) {
        rest(h, t); rest(e, t);
        const on = window_(t, 0.45, 3.15, 0.12);
        castPose(h.rig, Math.max(on, window_(t, 0.15, 0.5, 0.1)), "R", "forward");
        const a = wpos(h.at.tip), b = chest(e);
        // the tether sags and wobbles
        const mid = a.clone().lerp(b, 0.5).add(V(0, Math.sin(t * 6) * 0.08 - 0.15, 0));
        tether.visible = on > 0.05;
        setBeam(tether, a, b, 0.07 * on * (1 + Math.sin(t * 20) * 0.15));
        beads.visible = on > 0.05;
        beads.children.forEach((s, i) => {
          const u = (t * 0.8 + i / 10) % 1;
          const p = b.clone().lerp(a, u);
          p.add(mid.clone().sub(a.clone().lerp(b, 0.5)).multiplyScalar(Math.sin(u * Math.PI)));
          s.position.copy(p);
          s.material.opacity = 0.9 * on;
        });
        // the enemy's mana shrinks over her head; his staff glows fuller
        const drained = span(t, 0.45, 3.15);
        const r = 1 - drained * 0.75;
        well.position.copy(top(e)).add(V(0, 0.15, 0));
        well.scale.setScalar(0.17 * r);
        wellH.position.copy(well.position);
        wellH.scale.setScalar(0.3 + 0.3 * r);
        fade(well, 1 - span(t, 3.2, 3.5) * 0.0); well.visible = t < 3.3 || true;
        if (t > 3.3) { well.scale.setScalar(0.17 * lerp(r, 1, span(t, 3.3, 3.6))); }
        cup.position.copy(a);
        cup.scale.setScalar(0.4 + on * (0.5 + drained * 0.6));
        cup.material.opacity = 0.8 * on;
        tint(e.g, 0x6fb8ff, on * 0.12 * (0.5 + 0.5 * Math.sin(t * 8)));
      },
    };
  },
};
NOTES.mana_drain = {
  see: "Hexwright holds out his staff and a violet tether links it to an enemy. Blue beads stream along it from the enemy to the staff, the blue ball over the enemy's head shrinks, and the staff's light swells.",
  tell: "The tether is the channel: it holds while he stands still and the enemy stays within 8 m. The beads' direction shows which way the mana goes. It lasts 4 s (2.7 here).",
};

// ------------------------------------------------------------ R Final Verdict
ABILITIES.finger_of_death = {
  T: 3.4, still: 1.25, fps: 12,
  setup(st) {
    const h = hero(st, "hero_hexwright", "sun", -2.6, 0.4, 70);
    const e = hero(st, "hero_gristle", "moon", 2.0, -0.3, 300);
    const sigil = group(groundRingN(0.75, 0.85, lit(HEX, 0.9)), groundRingN(0.45, 0.5, lit(HEX, 0.9)), disc(0.75, 0, 0.012, 0, lit(HEXV, 0.3)));
    st.scene.add(sigil);
    const charge = halo(HEX, 1.5, 0.9);
    st.scene.add(charge);
    const ray = liveBeam(HEX, 0.22);
    st.scene.add(ray);
    const burst = own(puff(0xffb8ec, 0.5, 8, 9, 0.8));
    st.scene.add(burst);
    const ring = ringN(HEX, 0.12);
    st.scene.add(ring);
    return {
      look: [-0.2, 0.6, 0], dist: 11,
      update(t) {
        rest(h, t); rest(e, t);
        castPose(h.rig, window_(t, 0.1, 1.3, 0.12), "R", "up");
        // the warning: a sigil under the target, the staff gathering light
        const warn = window_(t, 0.3, 1.25, 0.08);
        sigil.position.copy(e.home);
        sigil.rotation.y = t * 2;
        sigil.scale.setScalar(1.6 - span(t, 0.3, 1.05) * 0.6);
        fade(sigil, warn);
        charge.position.copy(wpos(h.at.tip));
        charge.scale.setScalar(0.2 + span(t, 0.3, 1.05) * 1.4);
        charge.material.opacity = 0.9 * warn;
        // the strike
        const s = span(t, 1.05, 1.5);
        ray.visible = s > 0 && s < 1;
        setBeam(ray, wpos(h.at.tip), chest(e), 0.24 * (1 - s));
        billow(burst, chest(e), span(t, 1.05, 1.9), 0.4);
        ring.visible = s > 0 && s < 1; ring.position.set(e.home.x, 0.04, e.home.z); ring.scale.setScalar(0.4 + easeOut(s) * 2.0); fade(ring, 1 - s); ring.visible = ring.visible && (s > 0 && s < 1);
        if (t > 1.05 && t < 1.6) { flash(e, 1 - span(t, 1.05, 1.6)); sq(e, 1 - 0.3 * Math.sin(span(t, 1.05, 1.6) * Math.PI)); }
        if (t > 1.05 && t < 2.6) tint(e.g, HEX, 0.25 * (1 - span(t, 1.05, 2.6)));
      },
    };
  },
};
NOTES.finger_of_death = {
  see: "Hexwright raises his staff and its light swells while a pink sigil turns and tightens under the target. Then a thick pink beam joins staff and target, and the target is squashed flat in a puff of pink smoke.",
  tell: "The sigil under your feet is the warning that it is coming at you; the beam is the hit. In the game the delay is a quarter of a second; it is lengthened here so it can be seen.",
};

// ============================================================ Longwick
// ------------------------------------------------------------ Q Scattershot
ABILITIES.shrapnel = {
  T: 4.0, still: 1.9, fps: 12,
  setup(st) {
    const l = hero(st, "hero_longwick", "sun", -3.6, 0.6, 60);
    const es = [enemy(st, 0.8, -0.6, 270), enemy(st, 2.2, 0.4, 270)];
    const C0 = V(1.4, 0, 0);
    const warn = group(groundRingN(2.3, 2.45, lit(BRASS, 0.9)), disc(2.3, 0, 0.012, 0, lit(SMOKE, 0.18)));
    warn.position.copy(C0);
    st.scene.add(warn);
    const zone = areaN(2.45, BRASS, 0.2);
    zone.position.copy(C0);
    st.scene.add(zone);
    const shell = glowBall(BRASS, 0.12);
    st.scene.add(shell);
    const pellets = [];
    const R = rng(77);
    for (let i = 0; i < 40; i++) {
      const p = orb(0.045, 0, 0, 0, glow(BRASS, 0.9), { ink: false });
      p.userData.r = [R(), R(), R()];
      p.castShadow = false;
      st.scene.add(p);
      pellets.push(p);
    }
    const ds = dust(st, 8, 200, 0xd8cbb4);
    const slow = es.map(() => { const r = groundRingN(0.38, 0.48, lit(0x9fc6ff, 0.8)); st.scene.add(r); return r; });
    const muzzle = halo(0xffd27a, 0.9, 0.9);
    st.scene.add(muzzle);
    return {
      look: [-0.2, 0.3, 0], dist: 13,
      update(t) {
        rest(l, t); es.forEach((e) => rest(e, t));
        castPose(l.rig, window_(t, 0.1, 0.6, 0.1), "R", "up");
        const shot = span(t, 0.3, 0.42);
        muzzle.visible = shot > 0 && shot < 1;
        muzzle.position.copy(wpos(l.at.muzzle));
        muzzle.scale.setScalar(0.4 + shot);
        const up = span(t, 0.35, 1.1);
        shell.visible = up > 0 && up < 1;
        shell.position.copy(wpos(l.at.muzzle)).lerp(C0.clone().add(V(0, 5, 0)), up);
        // the warning ring before anything falls (0.8 s in the game)
        const w = window_(t, 0.4, 1.25, 0.06);
        fade(warn, w * (0.6 + 0.4 * Math.sin(t * 18)));
        const rain = window_(t, 1.25, 3.4, 0.1);
        fade(zone, rain);
        pellets.forEach((p) => {
          const [a, b, c] = p.userData.r;
          const ph = ((t - 1.25) * 1.4 + c) % 1;
          const on = t > 1.25 + c * 0.3 && t < 3.4;
          p.visible = on;
          const rr = Math.sqrt(a) * 2.2, an = b * 6.28;
          p.position.set(C0.x + Math.cos(an) * rr, 4 * (1 - ph), C0.z + Math.sin(an) * rr);
        });
        ds.forEach((p, i) => billow(p, C0.clone().add(V(Math.cos(i * 1.7) * 1.6 * ((i % 3) / 2 + 0.3), 0.05, Math.sin(i * 1.7) * 1.4)), ((t - 1.3) * 1.2 + i / 8) % 1 * (rain > 0.5 ? 1 : 0), 0.25));
        // the enemies inside are slowed: they plod and wear a blue ring
        es.forEach((e, i) => {
          const sl = rain;
          e.holder.position.x = e.home.x - (t - 0) * (0.6 - 0.45 * sl) * 0.4;
          if (e.rig) walk(e.rig, t * (1 - 0.6 * sl), 0.8);
          slow[i].position.set(e.holder.position.x, 0.025, e.home.z);
          fade(slow[i], sl);
          if (sl > 0.5) flash(e, 0.35 * Math.max(0, Math.sin(t * 9 + i)));
        });
      },
    };
  },
};
NOTES.shrapnel = {
  see: "Longwick fires a shell straight up. A pulsing brass ring marks where it will come down; a moment later brass shot rains into the circle with puffs of dust. Enemies walking through it plod, with a pale blue ring at their feet.",
  tell: "The ring appears before anything falls, so there is time to step out. Inside, the rain is the damage and the blue foot-ring is the slow. It rains 8 s in the game (2 here).",
};

// ------------------------------------------------------------ W Clean Shot (passive)
ABILITIES.headshot = {
  T: 3.0, still: 1.52, fps: 12,
  setup(st) {
    const l = hero(st, "hero_longwick", "sun", -2.6, 0.4, 75);
    const e = enemy(st, 1.8, 0.0, 280);
    const tr = [liveBeam(SMOKE, 0.025), liveBeam(BRASS, 0.06)];
    tr.forEach((b) => st.scene.add(b));
    const flashes = [halo(0xffd27a, 0.6, 0.9), halo(0xffd27a, 1.0, 0.9)];
    flashes.forEach((f) => st.scene.add(f));
    const pop = starMesh(0.25, BRASS, 0.05);
    pop.castShadow = false;
    st.scene.add(pop);
    const ss = stars(st, 1);
    return {
      look: [-0.3, 0.5, 0], dist: 10,
      update(t) {
        rest(l, t); rest(e, t);
        const m = wpos(l.at.muzzle), head = top(e).add(V(0, -0.2, 0)), body = chest(e);
        [[0.4, body, 0], [1.4, head, 1]].forEach(([at, to, i]) => {
          const u = span(t, at, at + 0.15);
          tr[i].visible = u > 0 && u < 1;
          setBeam(tr[i], m, m.clone().lerp(to, Math.min(1, u * 1.4)));
          flashes[i].visible = t > at && t < at + 0.1;
          flashes[i].position.copy(m);
          if (t > at && t < at + 0.15) l.g.position.z = l.g.userData.p0.z - 0.06;
        });
        if (t > 0.5 && t < 0.7) flash(e, 0.4);
        const h = span(t, 1.5, 1.95);
        pop.visible = h > 0 && h < 1;
        pop.position.copy(head).add(V(0, 0.2, 0.2));
        pop.scale.setScalar(0.5 + easeOut(h) * 1.2);
        pop.rotation.z = h * 2;
        if (h > 0 && h < 0.6) { flash(e, 1 - h / 0.6); e.g.rotation.x = -0.3 * Math.sin(h / 0.6 * Math.PI); e.holder.position.x = e.home.x + Math.sin(h / 0.6 * Math.PI) * 0.25; }
        dazed(e, ss[0], t, t > 1.55 && t < 2.2);
      },
    };
  },
};
NOTES.headshot = {
  see: "Longwick's normal shots leave a thin grey streak. When a shot procs, the streak is thick brass, a brass star pops at the target's head, its head snaps back and stars flicker round it for a moment.",
  tell: "The brass streak and head star mark the extra damage; the brief stars are the mini-stun, which in the game is a fifth of a second or less.",
};

// ------------------------------------------------------------ E Long Sight (passive)
ABILITIES.take_aim = {
  T: 3.4, still: 2.0, fps: 12,
  setup(st) {
    const l = hero(st, "hero_longwick", "sun", -1.6, 0.4, 80);
    const e = enemy(st, 1.75, 0.2, 280);
    const R0 = 2.4, R1 = 3.7;
    const base = groundRingN(R0 - 0.05, R0, lit(0xffffff, 0.5));
    const reach = groundRingN(0.95, 1.0, lit(BRASS, 0.95));
    const fill = disc(1, 0, 0.01, 0, lit(BRASS, 0.12));
    st.scene.add(base, reach, fill);
    const tr = liveBeam(SMOKE, 0.03);
    st.scene.add(tr);
    const mf = halo(0xffd27a, 0.7, 0.9);
    st.scene.add(mf);
    const scope = halo(0x9fe3ff, 0.5, 0.9);
    st.scene.add(scope);
    return {
      look: [-0.6, 0.3, 0.4], dist: 12.5,
      update(t) {
        rest(l, t); rest(e, t);
        const p = l.home;
        base.position.set(p.x, 0.02, p.z);
        const g = ease(span(t, 0.3, 1.3)) * (1 - span(t, 3.0, 3.35));
        const r = R0 + (R1 - R0) * g;
        reach.position.set(p.x, 0.03, p.z); reach.scale.setScalar(r);
        fill.position.set(p.x, 0.012, p.z); fill.scale.setScalar(r);
        fade(reach, 0.4 + 0.6 * window_(t, 0.2, 3.1, 0.15));
        fade(fill, window_(t, 0.2, 3.1, 0.15));
        scope.position.copy(wpos(l.at.muzzle)).add(V(-0.3, 0.12, 0));
        scope.material.opacity = 0.9 * (0.5 + 0.5 * Math.sin(t * 6));
        // he can now reach the enemy standing outside his old range
        const m = wpos(l.at.muzzle);
        [1.6, 2.4].forEach((at) => {
          const u = span(t, at, at + 0.15);
          if (u > 0 && u < 1) { tr.visible = true; setBeam(tr, m, m.clone().lerp(chest(e), Math.min(1, u * 1.4))); }
          if (t > at && t < at + 0.1) { mf.visible = true; mf.position.copy(m); }
          if (t > at + 0.12 && t < at + 0.3) flash(e, 0.6);
        });
        if (!(t > 1.6 && t < 1.75) && !(t > 2.4 && t < 2.55)) tr.visible = false;
        if (!(t > 1.6 && t < 1.7) && !(t > 2.4 && t < 2.5)) mf.visible = false;
      },
    };
  },
};
NOTES.take_aim = {
  see: "A faint white circle marks Longwick's ordinary reach; a brass circle grows out past it, and his scope glints. He then shoots an enemy standing between the two circles.",
  tell: "Nothing is cast: the brass circle is the range bonus (2.6 m at the top level, shown larger here), and it is the circle a player sees when choosing what to shoot.",
};

// ------------------------------------------------------------ R Deadeye
ABILITIES.assassinate = {
  T: 3.8, still: 1.15, fps: 12,
  setup(st) {
    const l = hero(st, "hero_longwick", "sun", -4.0, 0.6, 80);
    const e = hero(st, "hero_cinder", "moon", 3.6, -0.4, 290);
    const laser = liveBeam(0xff5a3c, 0.02);
    st.scene.add(laser);
    const ret = group(hoop(0.45, 0.045, 0, 0, 0, glow(0xff5a3c, 1.4), { ink: false }),
      limb([-0.65, 0, 0], [-0.3, 0, 0], 0.02, glow(0xff5a3c, 1.4), { ink: false }), limb([0.3, 0, 0], [0.65, 0, 0], 0.02, glow(0xff5a3c, 1.4), { ink: false }),
      limb([0, -0.65, 0], [0, -0.3, 0], 0.02, glow(0xff5a3c, 1.4), { ink: false }), limb([0, 0.3, 0], [0, 0.65, 0], 0.02, glow(0xff5a3c, 1.4), { ink: false }));
    ret.traverse((c) => { c.castShadow = false; });
    st.scene.add(ret);
    const bullet = glowBall(0xffb03a, 0.24);
    st.scene.add(bullet);
    const trail = liveBeam(0xffb03a, 0.06);
    st.scene.add(trail);
    const mf = halo(0xffd27a, 1.6, 0.95);
    st.scene.add(mf);
    const smoke = own(puff(SMOKE, 0.25, 5, 33, 0.7));
    st.scene.add(smoke);
    const boom = own(puff(0xffc48a, 0.4, 7, 34, 0.8));
    st.scene.add(boom);
    const ss = stars(st, 1);
    return {
      look: [0, 0.6, 0], dist: 13.5,
      update(t) {
        rest(l, t); rest(e, t);
        castPose(l.rig, window_(t, 0.15, 2.2, 0.12) * 0.4, "L", "forward");
        if (t > 0.15 && t < 2.2) l.g.position.y = l.g.userData.p0.y - 0.06 * window_(t, 0.15, 2.2, 0.1);
        const m = wpos(l.at.muzzle), c = chest(e);
        // the aim: a thin red line and a closing crosshair (1.7 s in the game)
        const aimU = window_(t, 0.2, 1.95, 0.08);
        laser.visible = aimU > 0.02;
        setBeam(laser, m, c, 0.02 * (0.7 + 0.3 * Math.sin(t * 25)));
        ret.visible = aimU > 0.02;
        ret.position.copy(c).add(V(0, 0, 0.35));
        ret.lookAt(st.camera.position);
        ret.rotation.z += t * 1.5;
        ret.scale.setScalar(lerp(2.0, 1.1, ease(span(t, 0.2, 1.8))));
        // the shot
        const f = span(t, 1.95, 2.25);
        mf.visible = t > 1.95 && t < 2.08; mf.position.copy(m);
        if (t > 1.95 && t < 2.15) l.g.position.z = l.g.userData.p0.z - 0.12;
        bullet.visible = f > 0 && f < 1;
        bullet.position.copy(m).lerp(c, f);
        trail.visible = f > 0 && f < 1.0;
        setBeam(trail, m.clone().lerp(c, Math.max(0, f - 0.35)), bullet.position, 0.06);
        billow(smoke, m, span(t, 1.95, 2.9), 0.4);
        billow(boom, c, span(t, 2.25, 3.0), 0.3);
        if (t > 2.25 && t < 2.7) { flash(e, 1 - span(t, 2.25, 2.7)); e.g.rotation.x = -0.4 * Math.sin(span(t, 2.25, 2.7) * Math.PI); }
        dazed(e, ss[0], t, t > 2.3 && t < 2.9);
      },
    };
  },
};
NOTES.assassinate = {
  see: "Longwick crouches over his rifle. A thin red line joins the barrel to the target and a red crosshair closes on it; then a bang, a smoke puff at the muzzle, and a glowing round streaks across and bursts on the target, which reels.",
  tell: "The red line and crosshair are the warning, and they are visible to the target for the whole 1.7 s aim; when the crosshair has closed, the shot is on its way.",
};

// ============================================================ Tempest
// ------------------------------------------------------------ Q Forked Spark
ABILITIES.arc_lightning = {
  T: 2.8, still: 0.92, fps: 12,
  setup(st) {
    const z = hero(st, "hero_tempest", "sun", -3.4, 0.6, 70);
    const es = [enemy(st, -0.4, 0.2, 280), enemy(st, 1.2, -1.0, 290), enemy(st, 2.6, 0.4, 260), enemy(st, 3.9, -0.8, 270)];
    const bolts = [];
    const lens = [];
    for (let i = 0; i < es.length; i++) {
      const a = i === 0 ? null : es[i - 1];
      bolts.push(null);
      lens.push(a);
    }
    const sparkHand = halo(ZAP, 1.0, 0.9);
    st.scene.add(sparkHand);
    const pops = es.map(() => { const h = halo(C.bolt, 1.0, 0.9); st.scene.add(h); return h; });
    let built = false;
    return {
      look: [0.2, 0.5, 0], dist: 12,
      update(t) {
        z.holder.position.copy(z.home);
        rest(z, t); es.forEach((e) => rest(e, t));
        castPose(z.rig, window_(t, 0.1, 0.65, 0.1), "R", "forward");
        if (!built) {
          // built once rest has placed everyone, so the ends are the rest pose
          es.forEach((e, i) => {
            const from = i === 0 ? wpos(z.at.handR) : chest(es[i - 1]);
            const b = boltN(from, chest(e), C.bolt, 10 + i, 7, 0.3, 0.05);
            const b2 = boltN(from, chest(e), ZAP, 20 + i, 6, 0.35, 0.09);
            const g = own(group(b, b2));
            st.scene.add(g);
            bolts[i] = g;
          });
          built = true;
        }
        sparkHand.position.copy(wpos(z.at.handR));
        sparkHand.scale.setScalar(0.3 + window_(t, 0.25, 0.75, 0.1) * 0.8);
        sparkHand.material.opacity = 0.9 * window_(t, 0.25, 0.8, 0.1);
        es.forEach((e, i) => {
          const at = 0.6 + i * 0.13;
          const u = span(t, at, at + 0.45);
          bolts[i].visible = u > 0 && u < 1;
          fade(bolts[i], 1 - u); bolts[i].visible = bolts[i].visible && (u > 0 && u < 1);
          pops[i].visible = u > 0 && u < 0.6;
          pops[i].position.copy(chest(e));
          pops[i].scale.setScalar(0.5 + u);
          if (u > 0 && u < 0.4) { flash(e, 1 - u * 2.5); e.g.position.y = e.g.userData.p0.y + Math.abs(Math.sin(u * 30)) * 0.05; }
        });
      },
    };
  },
};
NOTES.arc_lightning = {
  see: "Tempest points and a yellow-blue bolt snaps from his hand to the nearest enemy, then forks on to the next and the next, each a beat later; every enemy it touches flashes and jitters.",
  tell: "The chain of bolts shows exactly who was hit and in what order. It jumps only between enemies close together (5 m), up to fifteen times at the top level.",
};

// ------------------------------------------------------------ W Skyfall Bolt
ABILITIES.lightning_bolt = {
  T: 2.8, still: 1.12, fps: 12,
  setup(st) {
    const z = hero(st, "hero_tempest", "sun", -2.4, 0.5, 70);
    const e = hero(st, "hero_thistle", "moon", 1.6, -0.2, 300);
    const b1 = own(group(boltN(V(1.9, 9, -0.6), V(1.6, 0.9, -0.2), C.bolt, 3, 9, 0.6, 0.12), boltN(V(1.9, 9, -0.6), V(1.6, 0.9, -0.2), ZAP, 4, 8, 0.7, 0.22)));
    st.scene.add(b1);
    const sky = own(puff(0x9aa6c4, 0.7, 6, 41, 0.7));
    st.scene.add(sky);
    const scorch = disc(0.7, 1.6, 0.013, -0.2, veil(0x3a3048, 0.5));
    st.scene.add(scorch);
    const ring = ringN(C.bolt, 0.15);
    st.scene.add(ring);
    const glare = halo(C.bolt, 3.0, 0.95);
    st.scene.add(glare);
    const ss = stars(st, 1);
    const eye = group(groundRingN(2.6, 2.7, lit(ZAP, 0.6)));
    eye.position.set(1.6, 0, -0.2);
    st.scene.add(eye);
    return {
      look: [0, 0.8, 0], dist: 12,
      update(t) {
        rest(z, t); rest(e, t);
        castPose(z.rig, window_(t, 0.1, 0.7, 0.1), "R", "up");
        // a dark cloud gathers over the target, then the bolt
        const cl = window_(t, 0.25, 1.4, 0.15);
        billow(sky, V(1.8, 5.8, -0.5), 0.5 - cl * 0.4 + (t > 1.4 ? span(t, 1.4, 1.9) * 0.5 : 0), 0);
        sky.scale.setScalar(2.2 * Math.max(0.01, cl));
        const s = span(t, 0.75, 1.15);
        b1.visible = s > 0 && s < 1; fade(b1, 1 - s); b1.visible = b1.visible && (s > 0 && s < 1);
        glare.visible = s > 0 && s < 0.5; glare.position.set(1.6, 0.6, -0.2); glare.scale.setScalar(3 * (1 - s));
        ring.visible = s > 0 && s < 1; ring.position.set(1.6, 0.04, -0.2); ring.scale.setScalar(0.3 + s * 1.6); fade(ring, 1 - s); ring.visible = ring.visible && (s > 0 && s < 1);
        fade(scorch, t > 0.75 ? 1 - span(t, 1.8, 2.7) : 0);
        fade(eye, window_(t, 0.8, 2.5, 0.15) * 0.8);
        if (t > 0.75 && t < 1.2) { flash(e, 1 - span(t, 0.75, 1.2)); sq(e, 1 - 0.2 * Math.sin(span(t, 0.75, 1.2) * Math.PI)); }
        dazed(e, ss[0], t, t > 0.8 && t < 1.5);
      },
    };
  },
};
NOTES.lightning_bolt = {
  see: "Tempest throws a hand up; a grey cloud gathers over the target and a thick zigzag bolt drops out of it with a white glare, leaving a scorch mark. The target flinches and is dazed for a moment, and a faint blue circle shows the spot is now seen.",
  tell: "A bolt from the sky (not from his hand) is this spell; the brief stars are the short stun, and the blue circle is the sight it leaves behind.",
};

// ------------------------------------------------------------ E Charged Air (passive)
ABILITIES.static_field = {
  T: 2.8, still: 1.0, fps: 12,
  setup(st) {
    const z = hero(st, "hero_tempest", "sun", -0.6, 0.6, 20);
    const es = [enemy(st, 2.2, 0.2, 270), enemy(st, -3.0, -0.4, 90), enemy(st, 0.8, -2.4, 200), enemy(st, 3.4, -1.8, 250)];
    const ring = ringN(ZAP, 0.05), ring2 = ringN(C.bolt, 0.02);
    const zone = areaN(4.6, ZAP, 0.12);
    st.scene.add(ring, ring2, zone);
    const zaps = [];
    const cast = halo(ZAP, 1.0, 0.9);
    st.scene.add(cast);
    let built = false;
    return {
      look: [0, 0.3, -0.5], dist: 13.5,
      update(t) {
        rest(z, t); es.forEach((e) => rest(e, t));
        castPose(z.rig, window_(t, 0.1, 0.5, 0.1), "L", "up");
        cast.position.copy(wpos(z.at.handL)); cast.material.opacity = 0.9 * window_(t, 0.15, 0.5, 0.1);
        if (!built) {
          es.forEach((e, i) => {
            const c = chest(e), up = c.clone().add(V(0, 0.9, 0));
            const g = own(group(boltN(up, c, C.bolt, 60 + i, 4, 0.22, 0.07), boltN(up.clone().add(V(0.25, 0, 0)), c, ZAP, 70 + i, 4, 0.25, 0.09), halo(ZAP, 1.2, 0.8)));
            g.children[2].position.copy(c);
            st.scene.add(g); zaps.push(g);
          });
          built = true;
        }
        const k = span(t, 0.45, 1.05);
        ring.visible = k > 0 && k < 1; ring.position.set(z.home.x, 0.05, z.home.z); ring.scale.setScalar(0.3 + easeOut(k) * 4.3); fade(ring, 1 - k * 0.5); ring.visible = ring.visible && (k > 0 && k < 1);
        ring2.visible = ring.visible; ring2.position.copy(ring.position); ring2.scale.setScalar(0.3 + easeOut(k) * 4.25); fade(ring2, 1 - k); ring2.visible = ring2.visible && (ring.visible);
        zone.position.set(z.home.x, 0, z.home.z); fade(zone, window_(t, 0.45, 1.6, 0.12));
        es.forEach((e, i) => {
          const inside = e.home.distanceTo(z.home) < 4.6;
          const at = 0.45 + (e.home.distanceTo(z.home) / 4.6) * 0.6;
          const u = span(t, at, at + 0.35);
          zaps[i].visible = inside && u > 0 && u < 1;
          fade(zaps[i], 1 - u); zaps[i].visible = zaps[i].visible && (inside && u > 0 && u < 1);
          if (inside && u > 0 && u < 0.5) { flash(e, 1 - u * 2); e.g.position.y = e.g.userData.p0.y + Math.abs(Math.sin(u * 40)) * 0.05; }
        });
      },
    };
  },
};
NOTES.static_field = {
  see: "With each spell Tempest casts, a thin blue-and-yellow ring of static sweeps out from him, and every enemy it passes gets a little zap overhead and a jolt. One enemy outside the ring is left alone.",
  tell: "The sweeping ring is the reach (10 m in the game, drawn at 4.6 m here); a zap means that enemy lost a share of its current health, so it hurts most on healthy targets.",
};

// ------------------------------------------------------------ R Stormcall
ABILITIES.thundergods_wrath = {
  T: 3.6, still: 1.55, fps: 12,
  setup(st) {
    const z = hero(st, "hero_tempest", "sun", -4.2, 2.2, 40);
    const foes = [
      hero(st, "hero_ironvow", "moon", -1.0, -0.8, 300), hero(st, "hero_cinder", "moon", 1.4, 1.2, 300),
      hero(st, "hero_gristle", "moon", 3.6, -1.6, 290), hero(st, "hero_thistle", "moon", 5.2, 1.6, 310), hero(st, "hero_rime", "moon", -3.4, -3.0, 330),
    ];
    const creep = enemy(st, 0.6, -2.8, 300);
    const bolts = foes.map((f, i) => {
      const p = f.home;
      const g = own(group(boltN(V(p.x + 0.3, 10, p.z - 0.6), V(p.x, 1.0, p.z), C.bolt, 90 + i, 10, 0.6, 0.09), boltN(V(p.x - 0.2, 10, p.z - 0.6), V(p.x, 1.0, p.z), ZAP, 95 + i, 9, 0.7, 0.17)));
      st.scene.add(g);
      return g;
    });
    const glares = foes.map(() => { const h = halo(C.bolt, 3, 0.95); st.scene.add(h); return h; });
    const rings = foes.map(() => { const r = ringN(C.bolt, 0.15); st.scene.add(r); return r; });
    const crown = halo(C.bolt, 2.5, 0.95);
    st.scene.add(crown);
    const flashSky = new THREE.Mesh(new THREE.PlaneGeometry(80, 80), lit(0xeef4ff, 0.35));
    flashSky.position.set(0, 6, -6);
    flashSky.lookAt(0, 6, 10);
    st.scene.add(flashSky);
    return {
      look: [0.6, 0.4, -0.4], dist: 18,
      update(t) {
        rest(z, t); foes.forEach((f) => rest(f, t)); rest(creep, t);
        castPose(z.rig, window_(t, 0.1, 1.35, 0.12), "R", "both");
        // his crown flares as he calls the storm
        crown.position.copy(wpos(z.at.crown));
        const ch = window_(t, 0.25, 1.3, 0.12);
        crown.scale.setScalar(0.5 + ch * 2.5 + Math.sin(t * 20) * 0.2 * ch);
        crown.material.opacity = 0.95 * ch;
        // every enemy hero at once, wherever they are; the creep is spared
        const s = span(t, 1.15, 1.6);
        flashSky.visible = s > 0 && s < 0.5; flashSky.material.opacity = 0.35 * (1 - s * 2);
        foes.forEach((f, i) => {
          const u = span(t, 1.15 + i * 0.02, 1.65 + i * 0.02);
          bolts[i].visible = u > 0 && u < 1; fade(bolts[i], 1 - u); bolts[i].visible = bolts[i].visible && (u > 0 && u < 1);
          glares[i].visible = u > 0 && u < 0.5; glares[i].position.copy(chest(f)); glares[i].scale.setScalar(3 * (1 - u));
          rings[i].visible = u > 0 && u < 1; rings[i].position.set(f.home.x, 0.04, f.home.z); rings[i].scale.setScalar(0.3 + u * 1.5); fade(rings[i], 1 - u); rings[i].visible = rings[i].visible && (u > 0 && u < 1);
          if (u > 0 && u < 0.8) { flash(f, 1 - u / 0.8); sq(f, 1 - 0.2 * Math.sin(u / 0.8 * Math.PI)); }
        });
      },
    };
  },
};
NOTES.thundergods_wrath = {
  see: "Tempest lifts both arms and his cloud crown blazes. The sky flashes and a bolt drops on every enemy hero at the same instant, near or far; the enemy creep standing among them is not touched.",
  tell: "Bolts landing on all enemy heroes together, with no travel, is the cue: it reaches the whole map, so a hero anywhere is struck. Only heroes are hit.",
};

// ============================================================ The Warden
// ------------------------------------------------------------ Crushing Blow (passive)
ABILITIES.warden_bash = {
  T: 3.2, still: 1.95, fps: 12,
  setup(st) {
    let w;
    try { w = prep(unit(st, "warden", "neutral", 0.9, -0.6, 280, { ringR: 1.2 })); }
    catch (e) {
      const g = group(ball(1.0, 0.9, 1.0, 0, 0.95, 0, mat(0x8a7aa8)), ball(0.55, 0.45, 0.5, 0, 1.65, 0.6, mat(0x9a8ab8)),
        cone(0.18, 0.6, -0.35, 1.9, 0.4, mat(C.bone), { rot: [0, 0, 25] }), cone(0.18, 0.6, 0.35, 1.9, 0.4, mat(C.bone), { rot: [0, 0, -25] }),
        limb([0.9, 1.0, 0.3], [1.1, 0.2, 0.7], 0.25, mat(0x8a7aa8)), limb([-0.9, 1.0, 0.3], [-1.1, 0.2, 0.7], 0.25, mat(0x8a7aa8)));
      st.build = ((b) => (id, team) => (id === "warden" ? { g } : b(id, team)))(st.build);
      w = prep(unit(st, "warden", "neutral", 0.9, -0.6, 280, { ringR: 1.2 }));
    }
    const hHero = hero(st, "hero_ironvow", "sun", -1.4, 0.4, 100);
    const ring = ringN(0xd4b6ff, 0.12), ring2 = ringN(C.white, 0.05);
    st.scene.add(ring, ring2);
    const burst = starMesh(0.35, 0xffd166, 0.06);
    burst.castShadow = false;
    st.scene.add(burst);
    const ds = dust(st, 5, 300);
    const ss = stars(st, 1);
    return {
      look: [-0.2, 0.7, 0], dist: 11,
      update(t) {
        rest(w, t); rest(hHero, t);
        // two blows: a plain one, then the bash
        const lean = (a) => { const u = span(t, a, a + 0.6); return Math.sin(u * Math.PI) * (u < 0.5 ? -0.5 : 1); };
        const l1 = lean(0.2), l2 = lean(1.3);
        w.g.rotation.z = w.g.userData.r0.z + (l1 * 0.12 + l2 * 0.28);
        if (w.rig) { if (t > 0.2 && t < 0.8) swing(w.rig, span(t, 0.2, 0.8)); if (t > 1.3 && t < 1.9) swing(w.rig, span(t, 1.3, 1.9)); }
        if (t > 1.3 && t < 1.9) sq(w, 1 + 0.08 * Math.sin(span(t, 1.3, 1.9) * Math.PI));
        if (t > 0.6 && t < 0.8) flash(hHero, 0.4);
        const h = span(t, 1.62, 2.2);
        ring.visible = h > 0 && h < 1; ring.position.set(hHero.home.x, 0.04, hHero.home.z); ring.scale.setScalar(0.3 + easeOut(h) * 1.8); fade(ring, 1 - h); ring.visible = ring.visible && (h > 0 && h < 1);
        ring2.visible = ring.visible; ring2.position.copy(ring.position); ring2.scale.setScalar(0.3 + easeOut(h) * 1.7); fade(ring2, 1 - h); ring2.visible = ring2.visible && (ring.visible);
        burst.visible = h > 0 && h < 0.6; burst.position.copy(top(hHero)).add(V(0, 0.1, 0.3)); burst.scale.setScalar(0.6 + easeOut(h) * 1.0); burst.rotation.z = h * 3;
        ds.forEach((p, i) => billow(p, hHero.home.clone().add(V(Math.cos(i * 1.3) * 0.6, 0.05, Math.sin(i * 1.3) * 0.6)), span(t, 1.65, 2.4), 0.2));
        if (h > 0 && h < 0.5) { flash(hHero, 1 - h * 2); sq(hHero, 1 - 0.35 * Math.sin(h * 2 * Math.PI)); }
        dazed(hHero, ss[0], t, t > 1.75 && t < 3.0);
      },
    };
  },
};
NOTES.warden_bash = {
  see: "The Warden swats at a hero; the first blow is an ordinary hit. The second comes down harder: a lilac shockwave, a gold star burst and dust, the hero squashed flat and then dazed under stars.",
  tell: "The shockwave and the long stars mark a bash (a 15 % chance each attack); the stars stay for the full 1.65 s stun, long enough to see who is stuck.",
};
