// Ability animations, heroes 1 to 5 (Ironvow, Rime, Cinder, Thistle,
// Gristle). Each entry: { T seconds, still (the frame for the picture),
// fps, setup(st) -> { update(t), look?, view?, dist? } }. Every frame is a
// pure function of t, and t = 0 and t = T show the same rest pose, so the
// GIF loops. NOTES says what a player sees and which cue tells them.
// Scale: about 1 m to 100 game units; large areas are drawn smaller.
import { THREE, C, glow, veil, orb, ball, limb, gem, hoop, cone, group, unit, wpos, glowBall, halo, shock, stunStars, areaDecal, groundRing, disc, puff, fade, own, tint,
  resetRig, idle, castPose, swing, walk, span, ease, easeOut, easeIn, window_, lerp, rng } from "./kit.js";

const D = Math.PI / 180;
/** An effect material with normal blending, so element colours keep their hue over pale grass. */
function soft(color, opacity = 0.8) {
  return new THREE.MeshBasicMaterial({ color, transparent: true, opacity, blending: THREE.NormalBlending, depthWrite: false, toneMapped: false, side: THREE.DoubleSide });
}
const V = (x = 0, y = 0, z = 0) => new THREE.Vector3(x, y, z);

/** An enemy lane creep, or a Moonward hero while the creeps are not built yet. */
export function enemy(st, x, z, ry = 180, id = "creep_melee_moon") {
  let m;
  try { m = unit(st, id, "moon", x, z, ry); } catch (e) { m = unit(st, "hero_hexwright", "moon", x, z, ry); }
  return prep(m);
}
/** An ally (Sunward), any model id. */
function ally(st, id, x, z, ry = 0) {
  let m;
  try { m = unit(st, id, "sun", x, z, ry); } catch (e) { m = unit(st, "hero_rime", "sun", x, z, ry); }
  return prep(m);
}
function hero(st, id, x, z, ry, team = "sun") { return prep(unit(st, id, team, x, z, ry)); }

// ------------------------------------------------------------ helpers
/** Remembers a unit's rest transform and height, for the helpers below. */
function prep(m) {
  m._p = m.g.position.clone(); m._s = m.g.scale.clone(); m._r = m.g.rotation.clone();
  const b = new THREE.Box3();
  m.holder.updateMatrixWorld(true);
  m.g.traverse((c) => { if (c.isMesh && !c.userData.ink && c.geometry) { c.geometry.computeBoundingBox(); b.union(c.geometry.boundingBox.clone().applyMatrix4(c.matrixWorld)); } });
  m.h = Math.max(0.6, b.max.y);
  m.ph = (m.holder.position.x * 1.7 + m.holder.position.z) % 3;
  return m;
}
/** Back to rest, then idle; `frozen` units do not idle. */
function rest(t, ms, frozen = []) {
  for (const m of ms) {
    m.g.position.copy(m._p); m.g.scale.copy(m._s); m.g.rotation.copy(m._r);
    if (m.rig) { resetRig(m.rig); if (!frozen.includes(m)) idle(m.rig, t + m.ph); }
    m.paint = null;
  }
}
/** Colours to apply this frame: the highest k wins; then commit. */
function paint(m, color, k) { if (k > 0.001 && (!m.paint || k >= m.paint[1])) m.paint = [color, k]; }
function commit(ms) { for (const m of ms) { const p = m.paint; tint(m.g, p ? p[0] : 0xffffff, p ? p[1] : 0); } }
/** A white flash that falls off from time `at` over `len` seconds. */
function hitAt(m, t, at, len = 0.25) { if (t >= at && t < at + len) paint(m, 0xffffff, 0.5 * (1 - (t - at) / len)); }
/** Squash on a hit: wide and short, back over `len`. */
function squish(m, t, at, len = 0.3, amt = 0.25) {
  if (t < at || t > at + len) return;
  const k = Math.sin(((t - at) / len) * Math.PI) * amt;
  m.g.scale.set(m._s.x * (1 + k * 0.6), m._s.y * (1 - k), m._s.z * (1 + k * 0.6));
}
const base = (m) => m.holder.position.clone();
const top = (m) => base(m).add(V(0, m.h * m.holder.scale.y, 0));
const mid = (m) => base(m).add(V(0, m.h * 0.55, 0));
const hand = (m, s = "R") => (m.at && m.at["hand" + s] ? wpos(m.at["hand" + s]) : mid(m));
/** Stun stars over a unit while `on`. */
function starsOver(st) { const s = stunStars(0.28); st.scene.add(s); return s; }
function stun(s, m, t, on, i = 0) {
  s.visible = on;
  if (!on) return;
  s.position.copy(top(m)).add(V(0, 0.08, 0));
  s.userData.update(t);
  m.g.rotation.z = m._r.z + Math.sin(t * 7 + i) * 4 * D;
}
/** A slow: frost crystals round the feet; the caller tints the unit too. */
function frostFeet(st, color = C.frost) {
  const g = group(groundRing(0.3, 0.5, soft(color, 0.7), 0.03));
  for (let i = 0; i < 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    g.add(gem(0.05, 0.1, 0.05, Math.cos(a) * 0.38, 0.06, Math.sin(a) * 0.38, veil(color === C.frost ? C.ice : color, 0.85), { ink: false }));
  }
  own(g);
  g.traverse((c) => { c.castShadow = false; });
  st.scene.add(g);
  return g;
}
function feet(o, m, k) { o.position.copy(base(m)); fade(o, k); o.scale.setScalar(0.6 + 0.4 * Math.min(1, k * 2)); }
/** A silence: a white bubble over the head with a violet "no" sign in it. */
function muteBubble(st) {
  const g = own(group(
    orb(0.16, 0, 0, 0, veil(0xffffff, 0.92), { ink: false }),
    orb(0.05, -0.1, -0.14, 0, veil(0xffffff, 0.92), { ink: false }),
    hoop(0.1, 0.022, 0, 0, 0.14, glow(C.arcane, 1.2), { ink: false }),
    limb([-0.07, -0.07, 0.15], [0.07, 0.07, 0.15], 0.02, glow(C.arcane, 1.2), { ink: false }),
  ));
  st.scene.add(g);
  return g;
}
/** A coloured buff ring on the ground under a unit, with motes rising. */
function buffRing(st, color, r = 0.55, n = 6, seed = 4) {
  const g = group(groundRing(r * 0.8, r, soft(color, 0.85), 0.03));
  const R = rng(seed);
  for (let i = 0; i < n; i++) { const h = halo(color, 0.16, 0.9); h.userData.r = [R(), R(), R()]; g.add(h); }
  g.userData.tick = (t) => {
    g.children.forEach((c) => {
      if (!c.userData.r) return;
      const [a, b, s] = c.userData.r;
      const u = (t * (0.5 + s * 0.4) + b) % 1;
      c.position.set(Math.cos(a * 6.3 + t) * r * 0.75, 0.1 + u * 1.2, Math.sin(a * 6.3 + t) * r * 0.75);
      c.material.opacity = 0.9 * Math.sin(u * Math.PI);
    });
  };
  g.traverse((c) => { c.castShadow = false; });
  st.scene.add(g);
  return g;
}
function ringOn(g, m, t, k) { g.position.copy(base(m)); g.userData.tick(t); g.visible = k > 0.001; g.children[0].material.opacity = 0.85 * k; g.children.forEach((c) => { if (c.userData.r) c.material.opacity *= k; }); }
/** A shot (arrow or attack) that flies from a to b over [t0, t0 + len]. */
function shotAt(o, a, b, t, t0, len, arc = 0.2) {
  const u = span(t, t0, t0 + len);
  o.visible = t >= t0 && u < 1;
  o.position.copy(a).lerp(b, u);
  o.position.y += Math.sin(u * Math.PI) * arc;
  return u;
}
function ready(o) { o.traverse((c) => { c.castShadow = false; }); return o; }

export const ABILITIES = {};
export const NOTES = {};

// ============================================================ Ironvow
// Q Vow Hammer: a thrown hammer that stuns everyone round the target.
ABILITIES.storm_bolt = {
  T: 3.0, still: 1.2, fps: 12,
  setup(st) {
    const h = hero(st, "hero_ironvow", -3, 0.4, 70);
    const target = enemy(st, 2.6, 0, 240);
    const side = enemy(st, 3.3, 0.9, 200);
    const all = [h, target, side];
    const hammer = ready(own(group(
      limb([0, -0.18, 0], [0, 0.12, 0], 0.035, C.leather),
      orb(0.16, 0, 0.2, 0, glow(C.sun, 0.6)),
      limb([-0.2, 0.2, 0], [0.2, 0.2, 0], 0.12, glow(C.gold, 0.5)),
      halo(C.sun, 1.1, 0.7),
    )));
    st.scene.add(hammer);
    const ring = shock(C.sun);
    const zone = areaDecal(1.3, C.sun, 0.25);
    st.scene.add(ring, zone);
    const stars = [starsOver(st), starsOver(st)];
    return {
      look: [0, 0.4, 0.2], dist: 12.5,
      update(t) {
        rest(t, all);
        const wind = span(t, 0.1, 0.45), thr = span(t, 0.45, 0.6);
        castPose(h.rig, ease(wind) - ease(thr) * 0.4, "R", "up");
        const fly = span(t, 0.55, 1.05);
        const a = hand(h), b = mid(target);
        hammer.visible = fly > 0 && fly < 1;
        hammer.position.copy(a).lerp(b, easeIn(fly));
        hammer.position.y += Math.sin(fly * Math.PI) * 0.8;
        hammer.rotation.z = -fly * 14;
        const hit = span(t, 1.05, 1.5);
        ring.position.set(b.x, 0.04, b.z);
        ring.scale.setScalar(0.2 + easeOut(hit) * 1.2);
        fade(ring, t > 1.05 && hit < 1 ? 1 - hit : 0);
        zone.position.set(b.x, 0, b.z);
        fade(zone, window_(t, 1.05, 2.6, 0.12));
        hitAt(target, t, 1.05); hitAt(side, t, 1.05);
        squish(target, t, 1.05); squish(side, t, 1.05);
        const on = t > 1.05 && t < 2.7;
        [target, side].forEach((m, i) => stun(stars[i], m, t, on, i));
        commit(all);
      },
    };
  },
};
NOTES.storm_bolt = {
  see: "Ironvow winds up and hurls a spinning gold hammer in an arc. It lands with a gold ring that opens over everyone it stuns, and little stars circle each stunned head until the stun ends.",
  tell: "The hammer is in flight for half a second, so the target can see it coming; the ring shows the area; the stars show who is stunned and for how long.",
};

// W Wide Arc: each melee attack also hits enemies round the target.
ABILITIES.great_cleave = {
  T: 2.6, still: 0.86, fps: 12,
  setup(st) {
    const h = hero(st, "hero_ironvow", -1.3, 0.2, 80);
    const t0 = enemy(st, 0.4, 0.1, 250), t1 = enemy(st, 1.0, 1.1, 230), t2 = enemy(st, 1.1, -0.9, 270);
    const all = [h, t0, t1, t2];
    const arc = ready(own(group(
      hoop(1.25, 0.12, 0, 0, 0, soft(C.gold, 0.8), { arc: 120, ink: false }),
      hoop(1.25, 0.05, 0, 0, 0, soft(0xffffff, 0.9), { arc: 120, ink: false }),
    )));
    arc.children.forEach((c) => { c.rotation.x = -Math.PI / 2; });
    st.scene.add(arc);
    const spark = halo(C.gold, 1.4, 0.9); st.scene.add(spark);
    return {
      look: [0, 0.3, 0.1], dist: 10,
      update(t) {
        rest(t, all);
        const u = span(t, 0.25, 1.05);
        swing(h.rig, u);
        const s = span(t, 0.75, 1.15);
        arc.position.set(t0.holder.position.x - 0.5, 0.45, t0.holder.position.z);
        arc.rotation.y = lerp(80, -40, easeOut(s)) * D;
        arc.scale.setScalar(0.7 + s * 0.4);
        fade(arc, s > 0 && s < 1 ? 1 - s : 0);
        spark.position.copy(mid(t0)); spark.visible = s > 0 && s < 0.5; spark.material.opacity = 0.9 * (1 - s * 2);
        hitAt(t0, t, 0.8); squish(t0, t, 0.8, 0.3, 0.3);
        hitAt(t1, t, 0.88); squish(t1, t, 0.88, 0.3, 0.18);
        hitAt(t2, t, 0.92); squish(t2, t, 0.92, 0.3, 0.18);
        commit(all);
      },
    };
  },
};
NOTES.great_cleave = {
  see: "Ironvow swings his blade at one creep and a flat gold crescent sweeps out from it, flashing the creeps on either side a moment later.",
  tell: "Wide Arc has no cast: it shows on each melee hit, and the crescent marks the cleave's reach round the target. The creeps beside it flash and squash less than the one struck, because they take a share.",
};

// E Rallying Shout: allies near Ironvow gain armour and move speed.
ABILITIES.warcry = {
  T: 3.2, still: 1.5, fps: 12,
  setup(st) {
    const h = hero(st, "hero_ironvow", 0, 0, 0);
    const a1 = ally(st, "hero_rime", -2.0, -0.5, 25), a2 = ally(st, "hero_thistle", 2.0, -0.4, -25);
    const far = ally(st, "hero_cinder", 4.6, -1.2, -30);
    const all = [h, a1, a2, far];
    const rings = [shock(C.gold, 0.08), shock(C.gold, 0.08), shock(C.gold, 0.05)];
    rings.forEach((r) => st.scene.add(r));
    const mouth = halo(C.gold, 1.2, 0.8); st.scene.add(mouth);
    const buffs = [h, a1, a2].map((_, i) => buffRing(st, C.gold, 0.55, 5, 7 + i));
    const shields = [h, a1, a2].map(() => {
      const s = ready(own(group(orb(0.12, 0, 0, 0, glow(C.gold, 0.8), { ink: false }), hoop(0.15, 0.025, 0, 0, 0, glow(0xffffff, 1.2), { ink: false }))));
      s.children[0].scale.z = 0.35;
      st.scene.add(s);
      return s;
    });
    return {
      look: [0.8, 0.5, -0.4], dist: 12,
      update(t) {
        rest(t, all);
        const up = window_(t, 0.25, 1.1, 0.15);
        castPose(h.rig, up, "R", "both");
        h.rig.head.rotation.x = -up * 15 * D;
        mouth.position.copy(top(h)).add(V(0, -0.35, 0.35));
        mouth.visible = t > 0.5 && t < 1.0;
        mouth.scale.setScalar(0.6 + Math.sin(t * 40) * 0.15);
        rings.forEach((r, i) => {
          const s = span(t, 0.5 + i * 0.12, 1.3 + i * 0.12);
          r.position.set(0, 0.05 + i * 0.01, 0);
          r.scale.setScalar(0.4 + easeOut(s) * 3.1);
          fade(r, s > 0 && s < 1 ? 1 - s * s : 0);
        });
        const k = window_(t, 0.9, 2.9, 0.15);
        [h, a1, a2].forEach((m, i) => {
          ringOn(buffs[i], m, t, k);
          const sh = shields[i];
          sh.position.copy(top(m)).add(V(0, 0.22 + Math.sin(t * 3 + i) * 0.04, 0));
          sh.rotation.y = t * 3 + i;
          fade(sh, k);
          sh.scale.setScalar(k > 0 ? 0.6 + 0.4 * Math.min(1, (t - 0.9) * 4) : 0.01);
          hitAt(m, t, 0.95 + i * 0.05, 0.3);
        });
        commit(all);
      },
    };
  },
};
NOTES.warcry = {
  see: "Ironvow throws both arms up and shouts; three gold rings run out over the ground, and each ally inside gains a gold ring at their feet and a small turning gold shield over the head.",
  tell: "The shield and ring stay for the buff's six seconds (shortened here); an ally outside the shout, on the right, gets neither. The rings are drawn at half the game's 700 reach.",
};

// R Iron Oath: raises attack damage for 25 seconds.
ABILITIES.gods_strength = {
  T: 3.6, still: 1.9, fps: 12,
  setup(st) {
    const h = hero(st, "hero_ironvow", -0.9, 0.2, 80);
    const target = enemy(st, 0.75, 0.1, 250);
    const all = [h, target];
    const aura = ready(own(puff(0xff5a3a, 0.32, 9, 8, 0.35)));
    st.scene.add(aura);
    const burst = shock(0xff7a3a, 0.15); st.scene.add(burst);
    const glowUp = halo(0xff6a3a, 2.4, 0.6); st.scene.add(glowUp);
    const hitGlow = halo(C.flame, 1.8, 0.9); st.scene.add(hitGlow);
    const embers = [];
    const R = rng(9);
    for (let i = 0; i < 10; i++) { const e = halo(C.flame, 0.14, 0.9); e.userData.r = [R(), R()]; embers.push(e); st.scene.add(e); }
    return {
      look: [0, 0.6, 0], dist: 8.5,
      update(t) {
        rest(t, all);
        const up = window_(t, 0.15, 0.95, 0.15);
        castPose(h.rig, up, "R", "both");
        const grow = window_(t, 0.6, 3.2, 0.25) * 0.2;
        h.g.scale.copy(h._s).multiplyScalar(1 + grow);
        paint(h, 0xff8a5a, window_(t, 0.6, 3.2, 0.25) * 0.28);
        const b = span(t, 0.6, 1.1);
        burst.position.set(h.holder.position.x, 0.05, h.holder.position.z);
        burst.scale.setScalar(0.3 + easeOut(b) * 2); fade(burst, b > 0 && b < 1 ? 1 - b : 0);
        glowUp.position.copy(mid(h)); glowUp.visible = b > 0 && b < 1; glowUp.material.opacity = 0.6 * (1 - b);
        const k = window_(t, 0.7, 3.2, 0.2);
        aura.position.copy(base(h)).add(V(0, 0.55, 0));
        aura.rotation.y = t * 1.5;
        aura.scale.setScalar(1.2 + grow * 2);
        fade(aura, k);
        embers.forEach((e) => {
          const [a, s] = e.userData.r;
          const u = (t * 0.8 + s) % 1;
          e.position.copy(base(h)).add(V(Math.cos(a * 6.3) * 0.5, 0.2 + u * 1.6, Math.sin(a * 6.3) * 0.5));
          e.visible = k > 0.05; e.material.opacity = 0.9 * Math.sin(u * Math.PI) * k;
        });
        const sw = span(t, 1.35, 2.15);
        if (sw > 0 && sw < 1) swing(h.rig, sw);
        hitGlow.position.copy(mid(target)); hitGlow.visible = t > 1.85 && t < 2.15; hitGlow.material.opacity = 0.9 * (1 - (t - 1.85) / 0.3);
        hitAt(target, t, 1.85, 0.35); squish(target, t, 1.85, 0.4, 0.4);
        commit(all);
      },
    };
  },
};
NOTES.gods_strength = {
  see: "Ironvow raises both arms and flares red; he grows by a fifth, a red haze and rising embers wrap him, and his next swing lands with a large burst on the creep.",
  tell: "The size, the red tint and the embers last as long as the oath (25 seconds in the game, shortened here), so anyone can see that his hits are doubled before he lands one.",
};

// ============================================================ Rime
// Q Hoarfrost Burst: a frost blast at a point, damage and a slow.
ABILITIES.crystal_nova = {
  T: 3.4, still: 1.05, fps: 12,
  setup(st) {
    const h = hero(st, "hero_rime", -3.2, 0.6, 70);
    const P = V(1.6, 0, 0);
    const es = [enemy(st, 1.1, 0.4, 230), enemy(st, 2.2, -0.3, 240), enemy(st, 1.7, 1.2, 220, "creep_ranged_moon")];
    const out = enemy(st, 4.6, 0.2, 260);
    const all = [h, ...es, out];
    const orbIn = glowBall(C.frost, 0.16); st.scene.add(orbIn);
    const zone = areaDecal(2.0, C.frost, 0.3); zone.position.copy(P); st.scene.add(zone);
    const ring = shock(C.frost, 0.1); st.scene.add(ring);
    const shards = ready(own(group()));
    const R = rng(12);
    for (let i = 0; i < 16; i++) {
      const a = (i / 16) * Math.PI * 2 + R() * 0.3, d = 0.4 + R() * 1.4;
      const s = gem(0.08, 0.28 + R() * 0.2, 0.08, Math.cos(a) * d, 0, Math.sin(a) * d, veil(C.ice, 0.9), { ink: false });
      s.rotation.set(Math.sin(a) * 0.4, 0, -Math.cos(a) * 0.4);
      shards.add(s);
    }
    shards.position.copy(P); st.scene.add(shards);
    const ff = es.map(() => frostFeet(st));
    return {
      look: [0, 0.4, 0.3], dist: 12,
      update(t) {
        rest(t, all);
        castPose(h.rig, window_(t, 0.15, 0.75, 0.12), "R", "forward");
        shotAt(orbIn, h.at.tip ? wpos(h.at.tip) : hand(h), P.clone().add(V(0, 0.3, 0)), t, 0.45, 0.35, 0.6);
        const b = span(t, 0.8, 1.35);
        ring.position.set(P.x, 0.05, P.z);
        ring.scale.setScalar(0.3 + easeOut(b) * 1.8); fade(ring, b > 0 && b < 1 ? 1 - b : 0);
        fade(zone, window_(t, 0.8, 1.9, 0.1));
        const g = span(t, 0.8, 1.0), m = span(t, 1.9, 2.4);
        shards.children.forEach((s) => { s.userData.y ??= s.scale.y; s.scale.y = s.userData.y * Math.max(0.01, easeOut(g) * (1 - m)); });
        fade(shards, t > 0.8 ? 1 - m : 0);
        es.forEach((e, i) => {
          hitAt(e, t, 0.82, 0.25); squish(e, t, 0.82);
          const k = window_(t, 1.05, 3.1, 0.15);
          paint(e, C.frost, k * 0.45);
          feet(ff[i], e, k);
        });
        commit(all);
      },
    };
  },
};
NOTES.crystal_nova = {
  see: "Rime points her staff, a frost spark lands at a spot, and a blue burst throws up a ring of ice spikes over the area. Each enemy inside flashes, turns pale blue and keeps a ring of ice crystals at its feet.",
  tell: "The pale tint and the crystals at the feet are the slow; they stay for its four or five seconds (shortened here). The enemy on the right was outside the area and has neither. The area is drawn at half the game's 400 radius.",
};

// W Ice Shackle: freezes one enemy in place and hurts it over time.
ABILITIES.frostbite = {
  T: 3.4, still: 1.4, fps: 12,
  setup(st) {
    const h = hero(st, "hero_rime", -2.2, 0.5, 70);
    const target = enemy(st, 1.4, 0.1, 230, "hero_cinder");
    const all = [h, target];
    const flake = glowBall(C.frost, 0.13); st.scene.add(flake);
    const block = ready(own(group(ball(0.5, 0.85, 0.5, 0, 0.75, 0, veil(C.ice, 0.42), { ink: false }))));
    const R = rng(5);
    for (let i = 0; i < 8; i++) {
      const a = (i / 8) * Math.PI * 2;
      block.add(gem(0.1, 0.32 + R() * 0.2, 0.1, Math.cos(a) * 0.45, 0.15 + R() * 0.3, Math.sin(a) * 0.45, veil(C.frost, 0.85), { ink: false }));
    }
    block.position.copy(base(target)); st.scene.add(block);
    const bits = ready(own(group()));
    for (let i = 0; i < 10; i++) bits.add(gem(0.06, 0.06, 0.06, 0, 0, 0, veil(C.ice, 0.9), { ink: false }));
    st.scene.add(bits);
    const pulse = halo(C.frost, 1.6, 0.6); st.scene.add(pulse);
    return {
      look: [-0.3, 0.6, 0.2], dist: 9,
      update(t) {
        rest(t, all, t > 0.85 && t < 2.6 ? [target] : []);
        castPose(h.rig, window_(t, 0.15, 0.7, 0.12), "R", "forward");
        shotAt(flake, h.at.tip ? wpos(h.at.tip) : hand(h), mid(target), t, 0.45, 0.4, 0.4);
        const on = t > 0.85 && t < 2.6;
        const grow = easeOut(span(t, 0.85, 1.05));
        block.scale.set(1, Math.max(0.01, grow), 1);
        fade(block, on ? 1 : 0);
        paint(target, C.frost, on ? 0.55 : 0);
        for (const at of [1.3, 1.8, 2.3]) if (t > at && t < at + 0.15) paint(target, 0xffffff, 0.6);
        pulse.position.copy(mid(target)); pulse.visible = on; pulse.material.opacity = 0.35 + Math.sin(t * 8) * 0.15;
        const sh = span(t, 2.6, 3.1);
        bits.children.forEach((b, i) => {
          const a = (i / 10) * Math.PI * 2;
          b.position.copy(base(target)).add(V(Math.cos(a) * (0.3 + sh * 1.2), 0.8 + sh * 0.8 - sh * sh * 2, Math.sin(a) * (0.3 + sh * 1.2)));
          b.rotation.set(sh * 8, sh * 5, 0);
        });
        fade(bits, sh > 0 && sh < 1 ? 1 - sh : 0);
        commit(all);
      },
    };
  },
};
NOTES.frostbite = {
  see: "Rime flicks a snowflake at one enemy and a shell of ice with crystal spikes closes round it. It stops dead inside, pale blue, flashing as the cold hurts it, until the ice shatters into bits.",
  tell: "The ice shell is the whole cue: while it stands the enemy cannot move or attack (1.5 to 3 seconds on heroes, 10 on creeps; about 1.7 here). A soft blue pulse inside shows the damage ticking.",
};

// E Clear Mind: every allied hero, anywhere, regains extra mana.
ABILITIES.brilliance_aura = {
  T: 3.0, still: 1.2, fps: 12,
  setup(st) {
    const h = hero(st, "hero_rime", -1.6, 0.3, 30);
    const a1 = ally(st, "hero_tempest", 0.6, -0.5, 0), a2 = ally(st, "hero_cinder", 2.6, 0.4, -20);
    const all = [h, a1, a2];
    const rings = all.map((m, i) => buffRing(st, 0x4fa8ff, 0.55, 8, 20 + i));
    const gems = all.map(() => {
      const g = ready(own(group()));
      for (let i = 0; i < 4; i++) g.add(gem(0.06, 0.11, 0.06, 0, 0, 0, glow(0x4fa8ff, 0.9), { ink: false }));
      st.scene.add(g);
      return g;
    });
    const crowns = all.map(() => { const s = halo(0x4fa8ff, 0.7, 0.8); st.scene.add(s); return s; });
    return {
      look: [0.4, 0.4, 0], dist: 9.5,
      update(t) {
        rest(t, all);
        all.forEach((m, i) => {
          ringOn(rings[i], m, t, 1);
          gems[i].children.forEach((g, j) => {
            const u = (t / 1.5 + j / 4 + i * 0.13) % 1;
            const a = j * 1.6 + t * 2;
            g.position.copy(base(m)).add(V(Math.cos(a) * 0.4 * (1 - u), 0.3 + u * (m.h + 0.1), Math.sin(a) * 0.4 * (1 - u)));
            g.rotation.y = t * 4;
          });
          crowns[i].position.copy(top(m)).add(V(0, 0.15, 0));
          crowns[i].material.opacity = 0.4 + Math.sin(t * Math.PI * 2 / 1.5 + i) * 0.25;
        });
        commit(all);
      },
    };
  },
};
NOTES.brilliance_aura = {
  see: "Every allied hero, Rime included, has a faint blue ring at their feet and small blue diamonds that spiral up into a soft sparkle over the head.",
  tell: "It never switches off and has no cast; the blue diamonds mean extra mana coming in. The game's aura covers the whole map, so the cue shows on allies far from Rime too.",
};

// R Winter's Ring: a channel; ice blasts land round Rime for four seconds.
ABILITIES.freezing_field = {
  T: 4.4, still: 2.1, fps: 12,
  setup(st) {
    const h = hero(st, "hero_rime", 0, 0, 0);
    const es = [enemy(st, 2.4, 1.0, 200), enemy(st, -2.6, 0.6, 140), enemy(st, 1.6, -2.4, 220, "creep_ranged_moon"), enemy(st, -1.4, -2.6, 170)];
    const all = [h, ...es];
    const field = ready(own(group(
      groundRing(3.1, 3.25, soft(C.frost, 0.85), 0.03),
      groundRing(0.95, 1.05, soft(C.frost, 0.6), 0.03),
      groundRing(1.05, 3.1, soft(C.frost, 0.16), 0.02),
    )));
    st.scene.add(field);
    const R = rng(31);
    const blasts = [];
    for (let i = 0; i < 26; i++) {
      const a = R() * Math.PI * 2, d = 1.2 + R() * 1.8, at = 0.6 + i * 0.13;
      const g = ready(own(group(halo(C.ice, 1.0, 0.9))));
      const r = shock(C.frost, 0.2);
      g.add(r);
      for (let k = 0; k < 4; k++) g.add(gem(0.05, 0.14, 0.05, (k - 1.5) * 0.12, 0.1, ((k * 7) % 3 - 1) * 0.1, veil(C.ice, 0.9), { ink: false }));
      g.position.set(Math.cos(a) * d, 0, Math.sin(a) * d);
      g.userData = { at, ring: r };
      blasts.push(g);
      st.scene.add(g);
    }
    const snow = [];
    for (let i = 0; i < 20; i++) { const s = halo(0xffffff, 0.12, 0.9); s.userData.r = [R(), R(), R()]; snow.push(s); st.scene.add(s); }
    const ff = es.map(() => frostFeet(st));
    return {
      look: [0, 0.2, -0.6], dist: 13,
      update(t) {
        rest(t, all);
        const ch = window_(t, 0.3, 4.1, 0.15);
        castPose(h.rig, ch, "R", "both");
        h.g.position.y = h._p.y + ch * (0.15 + Math.sin(t * 4) * 0.03);
        fade(field, window_(t, 0.45, 4.1, 0.15));
        field.rotation.y = t * 0.3;
        blasts.forEach((g) => {
          const u = span(t, g.userData.at, g.userData.at + 0.4);
          g.userData.ring.scale.setScalar(0.2 + easeOut(u) * 0.8);
          g.children.forEach((c, k) => { if (k >= 2) { c.userData.y ??= c.scale.y; c.scale.y = c.userData.y * Math.max(0.01, Math.sin(u * Math.PI)); } });
          fade(g, u > 0 && u < 1 ? 1 - u : 0);
        });
        snow.forEach((s) => {
          const [a, b, c] = s.userData.r;
          const ang = a * 6.3 + t * (0.8 + c);
          const d = 1 + b * 2.2;
          s.position.set(Math.cos(ang) * d, 0.3 + ((t * 0.4 + c) % 1) * 1.6, Math.sin(ang) * d);
          s.visible = ch > 0.05; s.material.opacity = 0.9 * ch;
        });
        es.forEach((e, i) => {
          for (const g of blasts) {
            const p = g.position, q = e.holder.position;
            if (Math.hypot(p.x - q.x, p.z - q.z) < 0.9) hitAt(e, t, g.userData.at + 0.05, 0.2);
          }
          const k = window_(t, 0.7, 4.1, 0.2);
          if (!e.paint) paint(e, C.frost, k * 0.45);
          feet(ff[i], e, k);
        });
        commit(all);
      },
    };
  },
};
NOTES.freezing_field = {
  see: "Rime lifts both arms and floats a little; a ring of frost marks the field round her while snow swirls and small ice blasts burst at random spots between the inner and outer rings. Enemies in the field turn pale blue with ice at their feet and flash when a blast lands on them.",
  tell: "The rings show where blasts can land (none land in the clear middle), and the floating pose shows she is still channelling: an interrupt ends the snow and the rings at once. Drawn at half the game's size; it lasts its full four seconds here.",
};

// ============================================================ Cinder
// Q Flame Wave: a wave of fire along a line.
ABILITIES.dragon_slave = {
  T: 3.0, still: 1.0, fps: 12,
  setup(st) {
    const h = hero(st, "hero_cinder", -4.2, 0.2, 75);
    const es = [enemy(st, -1.2, 0.5, 250), enemy(st, 0.6, -0.4, 250, "creep_ranged_moon"), enemy(st, 2.4, 0.3, 250)];
    const off = enemy(st, 0.8, 2.5, 230);
    const all = [h, ...es, off];
    const L0 = -3.7, L1 = 3.7;
    const wave = ready(own(group()));
    const R = rng(17);
    for (let i = 0; i < 18; i++) {
      const p = puff(i % 3 ? C.fire : C.flame, 0.3, 3, 40 + i, 0.75);
      p.userData.r = [R() - 0.5, R(), R()];
      p.children.forEach((c) => { const m = glow(i % 3 ? C.fire : C.flame, 0.9).clone(); m.transparent = true; m.opacity = 0.85; c.material = m; });
      wave.add(p);
    }
    const head = halo(C.fire, 2.2, 0.8); wave.add(head);
    st.scene.add(wave);
    const scorch = ready(own(group()));
    for (let i = 0; i < 12; i++) scorch.add(disc(0.75 - i * 0.025, L0 + 0.4 + i * 0.62, 0.015, 0.1, veil(0x6a3a22, 0.22)));
    st.scene.add(scorch);
    return {
      look: [-0.2, 0.3, 0.3], dist: 12.5,
      update(t) {
        rest(t, all);
        castPose(h.rig, window_(t, 0.1, 0.6, 0.1), "R", "forward");
        castPose(h.rig, window_(t, 0.1, 0.6, 0.1), "L", "forward");
        const u = span(t, 0.45, 1.35);
        const x = lerp(L0, L1, u);
        wave.visible = u > 0 && u < 1;
        wave.position.set(x, 0, 0.1);
        const w = lerp(1.3, 0.75, u);
        wave.children.forEach((p) => {
          if (!p.userData.r) return;
          const [z, y, s] = p.userData.r;
          p.position.set(-s * 0.9, 0.2 + y * 0.6 + Math.sin(t * 20 + s * 9) * 0.06, z * w * 2);
          p.scale.setScalar(0.8 + s * 0.6);
        });
        head.position.set(0, 0.5, 0);
        const sk = u > 0 ? 1 - span(t, 1.6, 2.6) : 0;
        fade(scorch, sk);
        scorch.children.forEach((d) => { d.visible = sk > 0 && d.position.x < x - 0.3; });
        es.forEach((e) => {
          const at = 0.45 + ((e.holder.position.x - L0) / (L1 - L0)) * 0.9;
          hitAt(e, t, at, 0.3); squish(e, t, at, 0.3, 0.3);
          paint(e, C.fire, window_(t, at + 0.3, at + 0.8, 0.05) * 0.4);
        });
        commit(all);
      },
    };
  },
};
NOTES.dragon_slave = {
  see: "Cinder thrusts both hands forward and a wall of flame rolls out along the ground, widest near her and narrower at its far end, leaving a scorched strip behind it. Each enemy it passes flashes and glows orange for a moment.",
  tell: "The wave and the scorch mark show the line exactly; the enemy standing off the line is untouched. Drawn at about seven tenths of its 1075 length.",
};

// W Pillar of Flame: a warning circle, then fire from the sky that stuns.
ABILITIES.light_strike_array = {
  T: 3.4, still: 1.12, fps: 12,
  setup(st) {
    const h = hero(st, "hero_cinder", -3.2, 0.6, 70);
    const P = V(1.4, 0, 0);
    const es = [enemy(st, 1.0, 0.4, 230), enemy(st, 1.9, -0.3, 240)];
    const all = [h, ...es];
    const warn = ready(own(group(groundRing(1.3, 1.4, soft(C.fire, 0.9), 0.03), disc(1.0, 0, 0.02, 0, soft(C.fire, 0.35)))));
    const fill = warn.children[1];
    warn.position.copy(P); st.scene.add(warn);
    const pillar = ready(own(group(
      new THREE.Mesh(new THREE.CylinderGeometry(1.1, 1.35, 3.2, 32, 1, true), soft(C.ember, 0.4)),
      new THREE.Mesh(new THREE.CylinderGeometry(0.55, 0.8, 3.2, 32, 1, true), soft(C.fire, 0.45)),
      halo(C.fire, 3, 0.6),
    )));
    pillar.children[0].position.y = 1.6; pillar.children[1].position.y = 1.6; pillar.children[2].position.y = 0.6;
    pillar.position.copy(P); st.scene.add(pillar);
    const ring = shock(C.flame, 0.2); st.scene.add(ring);
    const smoke = ready(own(puff(0x8a7a7a, 0.35, 6, 3, 0.5))); smoke.position.copy(P); st.scene.add(smoke);
    const stars = es.map(() => starsOver(st));
    return {
      look: [-0.6, 0.6, 0.3], dist: 11,
      update(t) {
        rest(t, all);
        castPose(h.rig, window_(t, 0.1, 0.55, 0.1), "R", "up");
        const w = span(t, 0.45, 0.95);
        fade(warn, t > 0.45 && t < 1.0 ? 1 : 0);
        fill.scale.setScalar(Math.max(0.01, w * 1.3));
        const b = span(t, 0.95, 1.45);
        pillar.visible = b > 0 && b < 1;
        pillar.scale.set(1 - b * 0.3, Math.max(0.01, easeOut(span(t, 0.95, 1.05))), 1 - b * 0.3);
        fade(pillar, b > 0 && b < 1 ? 1 - easeIn(b) : 0);
        ring.visible = b > 0 && b < 1; ring.position.set(P.x, 0.05, P.z); ring.scale.setScalar(1 + b * 1.2); fade(ring, b > 0 && b < 1 ? 1 - b : 0);
        const sm = span(t, 1.2, 2.4);
        smoke.position.y = sm * 1.2; smoke.scale.setScalar(0.8 + sm); fade(smoke, sm > 0 && sm < 1 ? 1 - sm : 0);
        es.forEach((e, i) => {
          hitAt(e, t, 0.97, 0.3); squish(e, t, 0.97, 0.35, 0.35);
          stun(stars[i], e, t, t > 0.97 && t < 2.6, i);
        });
        commit(all);
      },
    };
  },
};
NOTES.light_strike_array = {
  see: "Cinder raises a hand and an orange ring appears on the ground, filling from the centre for half a second. Then a column of fire slams down inside it, and everyone still in the circle flashes and is stunned under circling stars.",
  tell: "The filling circle is the warning; it is the whole half second an enemy has to step out. The stars last the 1.6-second stun.",
};

// E Kindled Step: each spell gives Cinder attack and move speed, up to three stacks.
ABILITIES.fiery_soul = {
  T: 4.2, still: 2.6, fps: 12,
  setup(st) {
    const h = hero(st, "hero_cinder", -1.4, 0.3, 70);
    const target = enemy(st, 2.4, 0, 250);
    const all = [h, target];
    const flick = halo(C.flame, 0.9, 0.9); st.scene.add(flick);
    const orbs = [0, 1, 2].map(() => { const o = glowBall(C.fire, 0.15); st.scene.add(o); return o; });
    const shots = [0, 1, 2, 3].map(() => { const o = glowBall(C.fire, 0.09); st.scene.add(o); return o; });
    const trail = ready(group());
    for (let i = 0; i < 5; i++) trail.add(halo(C.fire, 0.22 - i * 0.03, 0.7));
    st.scene.add(trail);
    const casts = [0.3, 0.9, 1.5];
    return {
      look: [0.3, 0.6, 0.1], dist: 9.5,
      update(t) {
        rest(t, all);
        let n = 0;
        casts.forEach((c) => { if (t > c + 0.15) n++; });
        const ci = casts.findIndex((c) => t > c - 0.05 && t < c + 0.35);
        if (ci >= 0) castPose(h.rig, window_(t, casts[ci], casts[ci] + 0.25, 0.06), "R", "up");
        if (t > 3.9) n = 0;
        flick.visible = casts.some((c) => t > c + 0.1 && t < c + 0.3);
        flick.position.copy(top(h)).add(V(0.2, 0.2, 0));
        orbs.forEach((o, i) => {
          o.visible = i < n;
          const a = t * 4 + (i / 3) * Math.PI * 2;
          o.position.copy(base(h)).add(V(Math.cos(a) * 0.55, 0.7 + Math.sin(t * 6 + i) * 0.08, Math.sin(a) * 0.55));
        });
        paint(h, C.flame, n * 0.08);
        shots.forEach((s, i) => {
          const t0 = 2.0 + i * 0.4;
          shotAt(s, hand(h), mid(target), t, t0, 0.3, 0.25);
          hitAt(target, t, t0 + 0.3, 0.15);
          if (t > t0 - 0.12 && t < t0 + 0.05) castPose(h.rig, 0.8, "R", "forward");
        });
        trail.children.forEach((c, i) => {
          const ph = (t * 2.4 + i * 0.1) % 1;
          c.position.copy(base(h)).add(V(-0.25 - i * 0.12, 0.15, Math.sin(t * 9 + i) * 0.1));
          c.visible = n === 3;
          c.material.opacity = 0.6 * (1 - i / 5) * (0.5 + ph * 0.5);
        });
        commit(all);
      },
    };
  },
};
NOTES.fiery_soul = {
  see: "Each time Cinder casts, a small flame jumps off her and joins a ring of flames circling her waist, up to three. With three she glows warmer, leaves embers at her feet and throws fireballs much faster.",
  tell: "The number of circling flames is the number of stacks, so an enemy can count them; they all go out together when the seven seconds run out (shortened here).",
};

// R Searing Lance: a short warning, then a lance of fire into one enemy.
ABILITIES.laguna_blade = {
  T: 3.2, still: 1.1, fps: 12,
  setup(st) {
    const h = hero(st, "hero_cinder", -2.8, 0.4, 75);
    const target = enemy(st, 2.2, 0.0, 240, "hero_hexwright");
    const all = [h, target];
    const mark = ready(own(group(groundRing(0.45, 0.55, soft(C.ember, 0.9), 0.03), groundRing(0.75, 0.8, soft(C.ember, 0.7), 0.03))));
    st.scene.add(mark);
    const charge = halo(C.flame, 1.3, 0.9); st.scene.add(charge);
    const lance = ready(group());
    for (let i = 0; i < 3; i++) lance.add(new THREE.Mesh(new THREE.CylinderGeometry(1, 1, 1, 16), soft(i ? C.fire : C.flame, i ? 0.6 : 0.95)));
    st.scene.add(lance);
    const blast = halo(C.flame, 2.6, 0.95); st.scene.add(blast);
    const ring = shock(C.ember, 0.18); st.scene.add(ring);
    const scorch = own(disc(0.7, 0, 0.016, 0, veil(0x4a1f12, 0.5))); st.scene.add(scorch);
    return {
      look: [-0.2, 0.6, 0.2], dist: 10.5,
      update(t) {
        rest(t, all);
        const up = window_(t, 0.1, 1.1, 0.12);
        castPose(h.rig, up, "R", "both");
        const w = t > 0.35 && t < 0.95;
        mark.position.copy(base(target)); fade(mark, w ? 1 : 0); mark.rotation.y = t * 3;
        mark.children[1].scale.setScalar(lerp(1.6, 1, span(t, 0.35, 0.95)));
        charge.position.copy(hand(h)).add(V(0.1, 0.3, 0)); charge.visible = w; charge.scale.setScalar(0.6 + span(t, 0.35, 0.95) * 1.2);
        const b = span(t, 0.95, 1.6);
        lance.visible = b > 0 && b < 1;
        const a = hand(h).add(V(0, 0.3, 0)), z = mid(target);
        lance.children.forEach((seg, i) => {
          const r = (i === 0 ? 0.08 : 0.18 + i * 0.08) * (1 - b * 0.8);
          seg.scale.set(r, a.distanceTo(z), r);
          seg.position.copy(a).lerp(z, 0.5);
          seg.quaternion.setFromUnitVectors(V(0, 1, 0), z.clone().sub(a).normalize());
        });
        blast.position.copy(mid(target)); blast.visible = b > 0 && b < 0.7; blast.material.opacity = 0.95 * (1 - b / 0.7);
        ring.visible = b > 0 && b < 1; ring.position.copy(base(target)).setY(0.05); ring.scale.setScalar(0.4 + b * 1.6); fade(ring, b > 0 && b < 1 ? 1 - b : 0);
        scorch.position.copy(base(target)).setY(0.016); fade(scorch, window_(t, 1.0, 2.8, 0.1));
        hitAt(target, t, 0.97, 0.45); squish(target, t, 0.97, 0.5, 0.45);
        paint(target, C.ember, window_(t, 1.45, 2.4, 0.2) * 0.35);
        commit(all);
      },
    };
  },
};
NOTES.laguna_blade = {
  see: "Cinder raises both hands as fire gathers in them and a red ring turns under her target. Then a thick lance of white-hot fire joins her hands to the target, which flashes, squashes and is left on a scorch mark glowing red.",
  tell: "The red ring under the target is the warning: it marks who is about to be hit (the delay is a quarter second; the ring is held longer here so it can be seen). The lance cannot miss once the ring shows.",
};

// ============================================================ Thistle
// Q Rimed Arrows: while on, each attack costs mana and slows.
ABILITIES.frost_arrows = {
  T: 3.6, still: 2.35, fps: 12,
  setup(st) {
    const h = hero(st, "hero_thistle", -2.8, 0.4, 70);
    const target = enemy(st, 2.4, 0.1, 250, "hero_kestrel");
    const all = [h, target];
    const bowGlow = halo(C.frost, 1.3, 0.9); st.scene.add(bowGlow);
    const onRing = ready(own(groundRing(0.42, 0.5, soft(C.frost, 0.9), 0.03))); st.scene.add(onRing);
    const arrows = [0, 1, 2].map(() => {
      const a = ready(own(group(limb([-0.3, 0, 0], [0.22, 0, 0], 0.03, C.wood, { ink: false }), gem(0.08, 0.13, 0.08, 0.3, 0, 0, glow(C.frost, 1.0), { ink: false }), halo(C.frost, 0.9, 0.9))));
      a.children[1].rotation.z = -Math.PI / 2;
      a.children[2].position.x = 0.3;
      st.scene.add(a);
      return a;
    });
    const ff = frostFeet(st);
    const puffs = [0, 1, 2].map(() => { const p = halo(C.ice, 1.0, 0.9); st.scene.add(p); return p; });
    return {
      look: [-0.2, 0.5, 0.2], dist: 10.5,
      update(t) {
        rest(t, all);
        castPose(h.rig, 0.55, "L", "forward");
        const tip = h.at.bow ? wpos(h.at.bow) : hand(h, "L");
        bowGlow.position.copy(tip); bowGlow.material.opacity = 0.6 + Math.sin(t * 6) * 0.2;
        onRing.position.copy(base(h));
        let slowK = 0;
        arrows.forEach((a, i) => {
          const t0 = 0.4 + i * 0.9;
          if (t > t0 - 0.25 && t < t0 + 0.05) castPose(h.rig, window_(t, t0 - 0.2, t0, 0.05) * 0.6, "R", "forward");
          shotAt(a, tip, mid(target), t, t0, 0.3, 0.12);
          const d = mid(target).sub(tip);
          a.rotation.set(0, -Math.atan2(d.z, d.x), 0);
          hitAt(target, t, t0 + 0.3, 0.2);
          puffs[i].position.copy(mid(target)); puffs[i].visible = t > t0 + 0.3 && t < t0 + 0.55;
          puffs[i].material.opacity = 0.9 * (1 - (t - t0 - 0.3) / 0.25);
          if (t > t0 + 0.3) slowK = Math.max(slowK, 0.35 + i * 0.15);
        });
        slowK *= 1 - span(t, 3.2, 3.5);
        paint(target, C.frost, slowK);
        feet(ff, target, slowK > 0.01 ? slowK * 1.5 : 0);
        if (target.rig) walk(target.rig, t * (1.4 - slowK * 1.1), 1 - slowK * 0.6);
        commit(all);
      },
    };
  },
};
NOTES.frost_arrows = {
  see: "While Rimed Arrows is on, Thistle's bow glows blue and a blue ring sits under her. Each arrow has a glowing frost tip and bursts into a puff of ice on the hit; the target turns paler blue, gains ice at its feet and its walk slows.",
  tell: "The blue bow is the switched-on state, visible before she shoots. The tint and the ice at the feet are the slow, which lasts 1.5 seconds after each hit (7 on creeps).",
};

// W Hush: silences enemies in an area at a point.
ABILITIES.silence = {
  T: 3.4, still: 1.25, fps: 12,
  setup(st) {
    const h = hero(st, "hero_thistle", -3.2, 0.6, 70);
    const P = V(1.5, 0, 0);
    const es = [enemy(st, 1.1, 0.4, 220, "hero_hexwright"), enemy(st, 2.1, -0.3, 240, "hero_tempest")];
    const out = enemy(st, 4.5, 0.6, 250, "hero_rime");
    const all = [h, ...es, out];
    const dome = ready(own(group(ball(1.7, 1.1, 1.7, 0, 0, 0, soft(C.arcane, 0.25), { ink: false }), groundRing(1.6, 1.72, soft(C.arcane, 0.9), 0.03))));
    dome.position.copy(P); st.scene.add(dome);
    const spark = glowBall(C.arcane, 0.13); st.scene.add(spark);
    const bubbles = es.map(() => muteBubble(st));
    const zone = areaDecal(1.7, C.arcane, 0.2); zone.position.copy(P); st.scene.add(zone);
    return {
      look: [0, 0.4, 0.3], dist: 12,
      update(t) {
        rest(t, all);
        castPose(h.rig, window_(t, 0.1, 0.55, 0.1), "R", "up");
        shotAt(spark, hand(h).add(V(0, 0.2, 0)), P.clone().add(V(0, 0.4, 0)), t, 0.4, 0.4, 1.2);
        const b = span(t, 0.8, 1.3);
        dome.scale.set(Math.max(0.01, easeOut(b)), Math.max(0.01, easeOut(b) * (1 - b * 0.5)), Math.max(0.01, easeOut(b)));
        fade(dome, b > 0 && b < 1 ? 1 - b * b : 0);
        fade(zone, window_(t, 0.8, 1.6, 0.1));
        es.forEach((e, i) => {
          hitAt(e, t, 0.85, 0.2);
          const k = window_(t, 0.95, 3.1, 0.12);
          const bb = bubbles[i];
          bb.position.copy(top(e)).add(V(0.12, 0.25 + Math.sin(t * 4 + i) * 0.03, 0));
          fade(bb, k);
          bb.scale.setScalar(Math.max(0.01, easeOut(span(t, 0.95, 1.15))));
          paint(e, C.arcane, k * 0.18);
          if (e.rig && k > 0) { e.rig.head.rotation.x += k * 12 * D; e.rig.armL.rotation.x += k * 10 * D; e.rig.armR.rotation.x += k * 10 * D; }
        });
        commit(all);
      },
    };
  },
};
NOTES.silence = {
  see: "Thistle points and a violet spark falls on a spot, opening a violet dome over the area. Each enemy inside gets a white bubble over the head with a violet 'no' sign in it, and droops a little.",
  tell: "The bubble is the silence: while it shows, that enemy cannot cast, for three to six seconds (shortened here). The hero on the right was outside the dome and has none.",
};

// E Hunter's Eye: ranged allies nearby gain attack damage.
ABILITIES.trueshot_aura = {
  T: 3.0, still: 1.0, fps: 12,
  setup(st) {
    const h = hero(st, "hero_thistle", -0.4, 0.5, 40);
    const a1 = ally(st, "hero_longwick", 1.3, -0.6, 30), a2 = ally(st, "creep_ranged_sun", -2.0, -0.7, 40);
    const a3 = ally(st, "hero_ironvow", 1.8, 1.2, 20);
    const target = enemy(st, 3.8, -2.6, 230);
    const all = [h, a1, a2, a3, target];
    const aura = ready(own(group(groundRing(3.3, 3.42, soft(0x9be36a, 0.7), 0.025), groundRing(0.01, 3.3, soft(0x9be36a, 0.07), 0.02))));
    st.scene.add(aura);
    const marks = [h, a1, a2].map(() => {
      const c = cone(0.13, 0.24, 0, 0, 0, glow(0x7fd34a, 0.8), { rot: [180, 0, 0] });
      const g = ready(group(c));
      st.scene.add(g);
      return g;
    });
    const rings = [h, a1, a2].map((m, i) => buffRing(st, 0x9be36a, 0.45, 4, 50 + i));
    const shots = [h, a1, a2].map(() => { const s = glowBall(0x9be36a, 0.09); st.scene.add(s); return s; });
    return {
      look: [0.3, 0.3, -0.2], dist: 13,
      update(t) {
        rest(t, all);
        aura.position.copy(base(h)); aura.rotation.y = t * 0.4;
        [h, a1, a2].forEach((m, i) => {
          ringOn(rings[i], m, t, 1);
          marks[i].position.copy(top(m)).add(V(0, 0.38 + Math.abs(Math.sin(t * 3 + i)) * 0.08, 0));
          const t0 = 0.4 + i * 0.7;
          shotAt(shots[i], mid(m), mid(target), t, t0, 0.45, 0.4);
          hitAt(target, t, t0 + 0.45, 0.15);
        });
        commit(all);
      },
    };
  },
};
NOTES.trueshot_aura = {
  see: "A wide green circle surrounds Thistle. She and each ranged ally inside it have a green ring and rising motes at their feet and a small green arrowhead bobbing over the head, and their shots glow green.",
  tell: "Only ranged allies show the arrowhead: Ironvow, who fights in melee, stands inside the circle without one. The circle is drawn at about a third of the game's 900 radius.",
};

// R Steady Hand: a lasting gain in agility.
ABILITIES.marksmanship = {
  T: 3.0, still: 1.12, fps: 12,
  setup(st) {
    const h = hero(st, "hero_thistle", -1.6, 0.4, 70);
    const target = enemy(st, 2.0, 0.1, 250);
    const all = [h, target];
    const eye = halo(0x7fd34a, 0.8, 0.9); st.scene.add(eye);
    const leaves = ready(group());
    for (let i = 0; i < 7; i++) leaves.add(ball(0.12, 0.035, 0.07, 0, 0, 0, glow(0x7fd34a, 0.6)));
    st.scene.add(leaves);
    const shots = [0, 1, 2, 3].map(() => { const s = glowBall(0x9be36a, 0.12); st.scene.add(s); return s; });
    const glint = halo(0xffffff, 0.8, 0.9); st.scene.add(glint);
    return {
      look: [0, 0.6, 0.1], dist: 8.5,
      update(t) {
        rest(t, all);
        castPose(h.rig, 0.55, "L", "forward");
        eye.position.copy(top(h)).add(V(0.1, -0.35, 0.25));
        eye.material.opacity = 0.6 + Math.sin(t * 5) * 0.3;
        leaves.children.forEach((l, i) => {
          const a = t * (2 * Math.PI / 3) + (i / 7) * Math.PI * 2;
          l.position.copy(base(h)).add(V(Math.cos(a) * 0.6, 0.25 + ((t / 3 + i / 7) % 1) * 1.2, Math.sin(a) * 0.6));
          l.rotation.set(a, a * 2, 0);
        });
        const tip = h.at.bow ? wpos(h.at.bow) : hand(h, "L");
        shots.forEach((s, i) => {
          const t0 = 0.3 + i * 0.65;
          if (t > t0 - 0.2 && t < t0 + 0.05) castPose(h.rig, window_(t, t0 - 0.15, t0, 0.05) * 0.6, "R", "forward");
          shotAt(s, tip, mid(target), t, t0, 0.25, 0.08);
          hitAt(target, t, t0 + 0.25, 0.15);
        });
        glint.position.copy(tip); glint.visible = shots.some((s, i) => t > 0.3 + i * 0.65 - 0.06 && t < 0.3 + i * 0.65 + 0.06);
        commit(all);
      },
    };
  },
};
NOTES.marksmanship = {
  see: "Thistle has a green glint under her hood and green leaves drifting up around her; she draws and looses quickly, each shot sparkling at the bow.",
  tell: "Steady Hand is a lasting passive with no moment of its own: the green glint and leaves just say she has it learned. Its effect, more agility, shows as quicker shots and more damage, not as an event.",
};

// ============================================================ Gristle
// Q Gutting Hook: a hook on a chain that drags the first unit back.
ABILITIES.meat_hook = {
  T: 3.6, still: 1.05, fps: 12,
  setup(st) {
    const h = hero(st, "hero_gristle", -3.2, 0.3, 75);
    const target = enemy(st, 2.4, 0.2, 250, "hero_tempest");
    const all = [h, target];
    const home = target.holder.position.clone();
    const N = 22;
    const links = ready(group());
    for (let i = 0; i < N; i++) {
      links.add(ball(0.05, 0.05, 0.05, 0, 0, 0, i % 2 ? C.iron : C.steelDark, { lo: true }));
    }
    st.scene.add(links);
    const hook = ready(group(hoop(0.2, 0.045, 0, 0, 0, glow(C.steel, 0.3), { arc: 230 }), cone(0.06, 0.16, 0.1, 0.12, 0, glow(C.steel, 0.3))));
    st.scene.add(hook);
    const clang = halo(0xffffff, 1.4, 0.95); st.scene.add(clang);
    const blood = ready(group());
    for (let i = 0; i < 6; i++) blood.add(orb(0.05, 0, 0, 0, glow(C.blood, 0.6), { ink: false }));
    st.scene.add(blood);
    return {
      look: [-0.4, 0.4, 0.2], dist: 11.5,
      update(t) {
        rest(t, all);
        castPose(h.rig, window_(t, 0.1, 0.6, 0.1), "R", "forward");
        const out = span(t, 0.4, 0.95), back = span(t, 0.95, 1.9);
        const front = base(h).add(V(0.95, 0, 0));
        // reeled in, held at his belly, then (for the loop) back where it stood
        target.holder.position.copy(home).lerp(front, t < 3.3 ? easeIn(back) : 0);
        const start = hand(h);
        const tip = t > 0.95 ? mid(target) : start.clone().lerp(mid(target), out);
        const showChain = t > 0.4 && back < 1;
        links.visible = showChain; hook.visible = showChain;
        links.children.forEach((l, i) => { l.position.copy(start).lerp(tip, i / (N - 1)); l.position.y += Math.sin((i / (N - 1)) * Math.PI) * 0.1; });
        hook.position.copy(tip); hook.rotation.set(0, 0, -t * 2);
        clang.position.copy(tip); clang.visible = t > 0.95 && t < 1.2; clang.material.opacity = 0.95 * (1 - (t - 0.95) / 0.25);
        hitAt(target, t, 0.95, 0.3); squish(target, t, 0.95, 0.35, 0.3);
        const u = span(t, 1.9, 2.4);
        blood.visible = u > 0 && u < 1;
        blood.children.forEach((b, i) => {
          const a = (i / 6) * Math.PI * 2;
          b.position.copy(mid(target)).add(V(Math.cos(a) * u * 0.5, u * 0.5 - u * u * 0.9, Math.sin(a) * u * 0.5));
        });
        hitAt(target, t, 1.9, 0.3);
        if (t > 0.95 && t < 1.9 && target.rig) { target.rig.armL.rotation.x = -60 * D + Math.sin(t * 20) * 0.3; target.rig.armR.rotation.x = -60 * D - Math.sin(t * 20) * 0.3; }
        commit(all);
      },
    };
  },
};
NOTES.meat_hook = {
  see: "Gristle flings his hook and its chain runs out in a straight line. It clangs into the first unit in its path, which flails as it is reeled all the way back to his belly and lands with a red splash.",
  tell: "The chain is the line: anyone it reaches first is caught, ally or enemy, so the throw can be read and dodged. Only an enemy takes the damage (the splash).",
};

// W Carrion Cloud: a toggle; a cloud that hurts and slows round Gristle, and him too.
ABILITIES.rot = {
  T: 3.0, still: 1.5, fps: 12,
  setup(st) {
    const h = hero(st, "hero_gristle", -0.6, 0.2, 60);
    const es = [enemy(st, 1.3, 0.6, 230), enemy(st, 0.8, -1.2, 200)];
    const out = enemy(st, 3.6, -0.2, 250);
    const all = [h, ...es, out];
    const cloud = ready(group());
    const R = rng(23);
    for (let i = 0; i < 20; i++) {
      const p = puff(i % 2 ? C.rot : C.poison, 0.17, 3, 60 + i, 0.3);
      p.userData.r = [R() * 6.28, 0.9 + R() * 0.9, R()];
      cloud.add(p);
    }
    st.scene.add(cloud);
    const ground = ready(group(disc(1.95, 0, 0.015, 0, veil(0x6f9e3a, 0.28)), groundRing(1.85, 1.95, soft(C.poison, 0.7), 0.025)));
    st.scene.add(ground);
    const bubbles = ready(group());
    for (let i = 0; i < 10; i++) bubbles.add(orb(0.05, 0, 0, 0, veil(C.poison, 0.8), { ink: false }));
    st.scene.add(bubbles);
    const fl = es.map(() => frostFeet(st, C.poison));
    return {
      look: [0.4, 0.2, -0.1], dist: 10,
      update(t) {
        rest(t, all);
        ground.position.copy(base(h));
        cloud.position.copy(base(h));
        cloud.children.forEach((p) => {
          const [a, d, s] = p.userData.r;
          const ang = a + t * (0.6 + s * 0.6) * (2 * Math.PI / 3) / 1.2;
          p.position.set(Math.cos(ang) * d, 0.25 + s * 0.6 + Math.sin(t * 2 * Math.PI / 1.5 + a) * 0.08, Math.sin(ang) * d);
          p.scale.setScalar(0.9 + Math.sin(t * 2 * Math.PI / 1.5 + a) * 0.15);
        });
        bubbles.children.forEach((b, i) => {
          const u = (t * (2 / 3) + i / 10) % 1;
          const a = i * 2.4;
          b.position.copy(base(h)).add(V(Math.cos(a) * 1.3, 0.1 + u * 1.2, Math.sin(a) * 1.3));
          b.scale.setScalar(0.05 * (Math.sin(u * Math.PI) + 0.01));
        });
        for (const at of [0.5, 1.5, 2.5]) { es.forEach((e) => hitAt(e, t, at, 0.2)); hitAt(h, t, at, 0.15); }
        es.forEach((e, i) => { if (!e.paint) paint(e, C.poison, 0.3); feet(fl[i], e, 1); });
        commit(all);
      },
    };
  },
};
NOTES.rot = {
  see: "While Carrion Cloud is on, a swirl of green fumes and bubbles circles Gristle over a green stain on the ground. Enemies inside turn sickly green with green at their feet, and everyone in it, Gristle included, flashes once a second.",
  tell: "The green stain marks the 250 radius (drawn at about 1.9 m) and stays as long as the toggle is on. Gristle's own flash is the cost: he takes the same damage.",
};

// E Grave Bulk: magic resistance, and strength from each enemy hero who dies nearby.
ABILITIES.flesh_heap = {
  T: 3.6, still: 2.05, fps: 12,
  setup(st) {
    const h = hero(st, "hero_gristle", -1.4, 0.2, 50);
    const victim = enemy(st, 1.4, 0.3, 240, "hero_thistle");
    const all = [h, victim];
    const soul = glowBall(C.blood, 0.15, 0xffd0d8); st.scene.add(soul);
    const ring = ready(own(groundRing(2.1, 2.25, soft(C.blood, 0.8), 0.025))); st.scene.add(ring);
    const pop = shock(C.blood, 0.15); st.scene.add(pop);
    const heart = ready(own(group(orb(0.09, -0.06, 0, 0, glow(0xff6f8a, 1.0), { ink: false }), orb(0.09, 0.06, 0, 0, glow(0xff6f8a, 1.0), { ink: false }), cone(0.13, 0.17, 0, -0.2, 0, glow(0xff6f8a, 1.0), { ink: false, rot: [180, 0, 0] }))));
    st.scene.add(heart);
    const vRing = victim.holder.children[1];
    return {
      look: [0, 0.4, 0], dist: 10,
      update(t) {
        rest(t, all);
        ring.position.copy(base(h));
        // the enemy hero topples, greys and fades (it stands again for the loop)
        const fall = t < 3.3 ? span(t, 0.3, 0.8) : 0;
        victim.g.rotation.z = victim._r.z - easeIn(fall) * 80 * D;
        paint(victim, 0x8a8494, fall * 0.6);
        if (vRing) vRing.visible = fall < 0.5;
        shotAt(soul, mid(victim).add(V(0, 0.2, 0)), mid(h), t, 0.9, 0.7, 1.0);
        const s = span(t, 1.6, 2.1);
        const swell = Math.sin(s * Math.PI) * 0.18 + s * 0.08 * (1 - span(t, 3.2, 3.5));
        h.g.scale.copy(h._s).multiplyScalar(1 + swell);
        hitAt(h, t, 1.6, 0.3);
        if (!h.paint) paint(h, 0xffa0b0, window_(t, 1.6, 2.2, 0.1) * 0.3);
        pop.visible = s > 0 && s < 1; pop.position.copy(base(h)).setY(0.05); pop.scale.setScalar(0.5 + s * 1.4); fade(pop, s > 0 && s < 1 ? 1 - s : 0);
        const hu = span(t, 1.7, 2.8);
        heart.position.copy(top(h)).add(V(0, 0.2 + hu * 0.5, 0));
        heart.scale.setScalar(Math.sin(hu * Math.PI) * 1.2 + 0.01);
        fade(heart, hu > 0 && hu < 1 ? 1 - hu * hu : 0);
        commit(all);
      },
    };
  },
};
NOTES.flesh_heap = {
  see: "An enemy hero falls inside the faint red circle round Gristle. A red soul drifts from the body into his belly, he swells with a pink flash and a little heart pops up, and he stays a touch bigger.",
  tell: "The soul flying to him marks each strength gain; it comes only from enemy heroes dying within the circle (450, drawn at about 2.2 m). The magic resistance part has no cue of its own.",
};

// R Butcher's Feast: a channel on one enemy; it is stunned and hurt each second.
ABILITIES.dismember = {
  T: 3.8, still: 1.4, fps: 12,
  setup(st) {
    const h = hero(st, "hero_gristle", -0.9, 0.2, 75);
    const target = enemy(st, 0.75, 0.1, 250, "hero_cinder");
    const all = [h, target];
    const stars = starsOver(st);
    const beads = ready(group());
    for (let i = 0; i < 6; i++) beads.add(orb(0.05, 0, 0, 0, glow(C.blood, 0.8), { ink: false }));
    st.scene.add(beads);
    const zone = ready(own(group(groundRing(0.55, 0.65, soft(C.blood, 0.8), 0.03))));
    st.scene.add(zone);
    const pulses = [0, 1, 2].map(() => { const p = halo(C.blood, 1.2, 0.8); st.scene.add(p); return p; });
    return {
      look: [0, 0.6, 0.1], dist: 8,
      update(t) {
        const held = t > 0.55 && t < 3.3;
        rest(t, all, held ? [target] : []);
        const ch = window_(t, 0.3, 3.3, 0.12);
        castPose(h.rig, ch * 0.8, "R", "forward");
        castPose(h.rig, ch * 0.8, "L", "forward");
        h.rig.body.rotation.x = ch * (12 + Math.sin(t * 9) * 6) * D;
        target.g.position.y = target._p.y + (held ? 0.3 + Math.sin(t * 9) * 0.05 : 0);
        stun(stars, target, t, held);
        zone.position.copy(base(target)); fade(zone, ch);
        [1.0, 2.0, 3.0].forEach((at, i) => {
          hitAt(target, t, at, 0.25); squish(target, t, at, 0.3, 0.25);
          const p = pulses[i];
          p.position.copy(mid(target)); p.visible = t > at && t < at + 0.3; p.material.opacity = 0.8 * (1 - (t - at) / 0.3);
        });
        beads.visible = held;
        beads.children.forEach((b, i) => {
          const u = (t * 1.3 + i / 6) % 1;
          b.position.copy(mid(target)).lerp(mid(h), u);
          b.position.y += Math.sin(u * Math.PI) * 0.25;
        });
        if (!target.paint && held) paint(target, C.blood, 0.15);
        commit(all);
      },
    };
  },
};
NOTES.dismember = {
  see: "Gristle wraps both arms round the enemy in front of him and leans in, chomping. The enemy is lifted off the ground under stun stars, a red ring marks it, red beads stream from it to him, and it flashes red once a second.",
  tell: "The held, lifted enemy and the stars mean the channel is running, up to three seconds on heroes; if Gristle is stunned the hold breaks at once. Each red flash is one tick of damage.",
};
