// Round-4 mockups (T13, T14): an All Random All Mid mode. The models are
// the base cannon that throws a hero back to the middle of the lane, and a
// health relic that waits in the lane; the animations show the cannon
// fired, its aim, a relic taken, and the random draw of a hero. Not the
// game: the game draws with its own renderer from data/models.json.
// Distances are drawn shorter than they are: the cannon's real throw is
// several thousand game units (the journal tab gives the numbers).
import { THREE, C, glow, shiny, light, veil, ball, orb, limb, cyl, cone, hoop, group, joint, face, rig, halo, unit, wpos,
  shock, areaDecal, groundRing, disc, puff, fade, own, tint, resetRig, idle, walk, swing, hop, castPose, squash,
  span, ease, easeOut, easeIn, window_, lerp, rng, lanePatch } from "./kit.js";

const D = Math.PI / 180;
const V = (x = 0, y = 0, z = 0) => new THREE.Vector3(x, y, z);
function soft(color, opacity = 0.8) {
  return new THREE.MeshBasicMaterial({ color, transparent: true, opacity, blending: THREE.NormalBlending, depthWrite: false, toneMapped: false, side: THREE.DoubleSide });
}
function ready(o) { o.traverse((c) => { c.castShadow = false; }); return o; }
function anchors(g, pts) {
  const at = {};
  for (const [k, p] of Object.entries(pts)) { const o = new THREE.Object3D(); o.position.set(...p); g.add(o); at[k] = o; }
  return at;
}

// ============================================================ models
/** The base cannon: a fat bronze mortar on a turning stone drum, banded in
 * the side's colour, with sleepy eyes on the drum and a glowing fuse. */
function cannon(side) {
  const sun = side === "sun";
  const stone = sun ? C.sand : 0x8a93ae, dark = sun ? 0xd9b77a : 0x6a7594, team = sun ? C.sunDeep : C.moon, bright = sun ? C.gold : C.lilac;
  const metal = sun ? C.gold : C.steel;
  const g = new THREE.Group();
  // the drum it turns on, with four round feet
  g.add(cyl([0.95, 1.05], 0.4, 0, 0.2, 0, dark));
  for (let i = 0; i < 4; i++) {
    const a = (i / 4) * Math.PI * 2 + Math.PI / 4;
    g.add(ball(0.22, 0.16, 0.22, Math.cos(a) * 0.95, 0.1, Math.sin(a) * 0.95, stone));
  }
  const turn = joint(0, 0.4, 0);
  turn.add(cyl([0.72, 0.82], 0.5, 0, 0.25, 0, stone));
  turn.add(hoop(0.8, 0.06, 0, 0.42, 0, shiny(team), { rot: [90, 0, 0] }));
  for (const s of [-1, 1]) turn.add(ball(0.08, 0.12, 0.04, s * 0.2, 0.3, 0.76, C.eye, { ink: false }));
  // the barrel: a tilted, tapering bronze drum with a fat lip, banded
  const tilt = joint(0, 0.7, 0);
  const barrel = group(
    cyl([0.42, 0.55], 1.25, 0, 0.55, 0, shiny(metal)),
    hoop(0.47, 0.09, 0, 1.18, 0, shiny(team), { rot: [90, 0, 0] }),
    hoop(0.52, 0.07, 0, 0.5, 0, shiny(team), { rot: [90, 0, 0] }),
    cyl(0.33, 0.04, 0, 1.2, 0, mat_(0x2b2140), { ink: false }),
    orb(0.5, 0, 0, 0, shiny(metal)),
  );
  barrel.rotation.x = 38 * D;
  tilt.add(barrel);
  // the emblem on the side: a gold sun disc or a violet moon crescent
  const em = joint(0.6, 0.15, 0);
  em.add(cyl(0.2, 0.05, 0, 0, 0, glow(bright, 0.6), { rot: [0, 0, 90] }));
  if (!sun) em.add(cyl(0.15, 0.06, 0.02, 0.05, 0.04, shiny(dark), { rot: [0, 0, 90] }));
  tilt.add(em);
  turn.add(tilt);
  // the fuse: a short cord at the back with a spark
  const spark = orb(0.07, 0, 0.9, -0.62, glow(C.flame, 2.0), { ink: false });
  turn.add(limb([0, 0.55, -0.55], [0, 0.88, -0.62], 0.025, C.woodDark), spark);
  g.add(turn);
  const at = anchors(g, { top: [0, 2.0, 0], headTop: [0, 2.0, 0], mouth: [0, 1.95, 0.75], chest: [0, 1.0, 0.9] });
  const out = { g, at, rig: rig(g), turn, tilt, barrel };
  const s0 = spark.scale.x;
  out.update = (t) => { spark.scale.setScalar(s0 * (0.8 + Math.sin(t * 17) * 0.25)); };
  return out;
}
const mats_ = new Map();
function mat_(c) { if (!mats_.has(c)) mats_.set(c, new THREE.MeshBasicMaterial({ color: c })); return mats_.get(c); }

/** A health relic: a round glass flask with a green heart of light and a
 * gold cap, floating over a small ring of stones. */
function relic() {
  const g = new THREE.Group();
  g.add(cyl([0.45, 0.5], 0.12, 0, 0.06, 0, C.stone));
  for (let i = 0; i < 6; i++) {
    const a = (i / 6) * Math.PI * 2;
    g.add(ball(0.09, 0.08, 0.09, Math.cos(a) * 0.42, 0.14, Math.sin(a) * 0.42, C.stoneDark));
  }
  const f = joint(0, 0.85, 0);
  f.add(orb(0.26, 0, 0, 0, veil(0xe8fff0, 0.3, { roughness: 0.1 }), { ink: true }));
  f.add(orb(0.17, 0, -0.02, 0, glow(0x3ee06a, 2.2), { ink: false }));
  f.add(cyl([0.08, 0.1], 0.14, 0, 0.3, 0, veil(0xe8fff0, 0.6)));
  f.add(cyl(0.11, 0.08, 0, 0.4, 0, shiny(C.gold)));
  // a plus of light on the heart
  f.add(limb([-0.08, 0, 0.16], [0.08, 0, 0.16], 0.025, glow(0xffffff, 1.2), { ink: false }));
  f.add(limb([0, -0.08, 0.16], [0, 0.08, 0.16], 0.025, glow(0xffffff, 1.2), { ink: false }));
  f.add(halo(0x7cf09a, 1.2, 0.55));
  g.add(f);
  const at = anchors(g, { top: [0, 1.3, 0], headTop: [0, 1.3, 0], chest: [0, 0.85, 0.3] });
  return { g, at, rig: rig(g), flask: f, update: (t) => { f.position.y = 0.85 + Math.sin(t * 2.2) * 0.07; f.rotation.y = t * 0.9; } };
}

export const MODELS = {
  cannon_sun: () => cannon("sun"),
  cannon_moon: () => cannon("moon"),
  relic: () => relic(),
};
export const MODEL_NOTES = {
  cannon_sun: "Sunward cannon: a fat gold mortar tilted up on a turning sandstone drum, banded in sun orange, a sun disc on its side, two sleepy eyes and a sparking fuse.",
  cannon_moon: "Moonward cannon: the same mortar in silver, banded in violet on a slate drum, with a crescent on its side.",
  relic: "Health relic: a round glass flask with a green heart of light and a white plus, floating and turning over a ring of stones.",
};

// ============================================================ helpers
function prep(m) {
  m._p = m.g.position.clone(); m._s = m.g.scale.clone(); m._r = m.g.rotation.clone();
  return m;
}
function restAll(t, ms) {
  for (const m of ms) {
    m.g.position.copy(m._p); m.g.scale.copy(m._s); m.g.rotation.copy(m._r);
    if (m.rig) { resetRig(m.rig); idle(m.rig, t); }
    if (m.update) m.update(t);
  }
}
/** A dotted arc of glows from a to b, `h` high at its middle. */
function dottedArc(color, n = 18) {
  const g = new THREE.Group();
  for (let i = 0; i < n; i++) g.add(own(group(orb(0.07, 0, 0, 0, soft(color, 0.9), { ink: false }))));
  ready(g);
  g.userData.place = (a, b, h, k, phase = 0) => {
    g.children.forEach((c, i) => {
      const u = (i + phase) / n;
      c.position.copy(a).lerp(b, u);
      c.position.y += Math.sin(u * Math.PI) * h;
      fade(c, k * (0.35 + 0.65 * Math.sin(u * Math.PI)));
    });
  };
  return g;
}
/** The landing mark: a ring that pulses in the side's colour, with a disc. */
function landing(color, r) {
  const g = group(groundRing(r * 0.86, r, soft(color, 0.9), 0.03), disc(r * 0.86, 0, 0.025, 0, soft(color, 0.22)), groundRing(r * 0.3, r * 0.38, soft(color, 0.9), 0.03));
  return ready(g);
}

// ============================================================ animations
export const ABILITIES = {};
export const NOTES = {};

// The cannon thrown: a hero steps up, hops in, the cannon turns and fires,
// the hero flies curled up along a high arc and lands where the ring shows.
ABILITIES.cannon_launch = {
  T: 4.6, still: 2.35, fps: 12,
  setup(st) {
    const can = prep(unit(st, "cannon_sun", "sun", -6.2, 0.4, 90, { ringR: 1.1 }));
    const h = prep(unit(st, "hero_rime", "sun", -8.0, 1.6, 90));
    const tower = prep(unit(st, "tower_sun", "sun", 3.2, -2.4, 270, { ringR: 1.4 }));
    const all = [can, h, tower];
    const target = V(6.2, 0, 0.6);
    const mark = landing(C.sun, 0.9); st.scene.add(mark);
    const arc = dottedArc(C.sun); st.scene.add(arc);
    const smoke = ready(own(puff(0xf4efe6, 0.5, 6, 11, 0.85))); st.scene.add(smoke);
    const flashB = halo(C.flame, 2.4, 0.9); st.scene.add(flashB);
    const dust = shock(C.sand, 0.16); st.scene.add(dust);
    const trail = [];
    for (let i = 0; i < 7; i++) { const s = halo(C.sun, 0.35 - i * 0.03, 0.8); st.scene.add(s); trail.push(s); }
    return {
      look: [0, 0.8, 0.4], view: [0, 0.55, 1], dist: 17,
      update(t) {
        restAll(t, all);
        const mouth = wpos(can.at.mouth);
        // 0.0-0.8: walk up; 0.8-1.1: hop into the barrel
        const w = span(t, 0, 0.8), inn = span(t, 0.8, 1.1);
        const start = V(-8.0, 0, 1.6), foot = V(-6.9, 0, 0.9);
        const hp = start.clone().lerp(foot, ease(w));
        h.holder.position.copy(hp);
        h.holder.rotation.y = Math.atan2(foot.x - start.x, foot.z - start.z);
        if (w < 1) walk(h.rig, t * 1.3);
        if (inn > 0) {
          h.holder.position.lerpVectors(foot, mouth.clone().add(V(0, -0.6, 0)), easeIn(inn));
          h.holder.position.y += Math.sin(inn * Math.PI) * 0.9;
          h.g.scale.setScalar(m1(1 - inn * 0.7));
        }
        // 1.0-1.9: the cannon turns to the mark and leans back; the mark and the arc show
        const aimU = span(t, 1.0, 1.7);
        can.turn.rotation.y = lerp(0, 0.18, ease(aimU));
        const lean = span(t, 1.55, 1.9), kick = span(t, 1.9, 2.25);
        can.tilt.rotation.x = -lerp(0, 0.18, ease(lean)) + easeOut(kick) * 0.18 - Math.sin(kick * Math.PI) * 0.25;
        squash(can.rig, 1 - Math.sin(lean * Math.PI) * 0.08 + Math.sin(kick * Math.PI) * 0.1);
        const markOn = window_(t, 1.0, 4.1, 0.15);
        mark.position.set(target.x, 0, target.z);
        mark.scale.setScalar(0.85 + Math.sin(t * 7) * 0.06);
        fade(mark, markOn);
        arc.userData.place(mouth, target.clone().add(V(0, 0.4, 0)), 3.2, window_(t, 1.1, 2.0, 0.12), (t * 6) % 1);
        // 1.9: the shot, with a flash and a puff of smoke
        flashB.position.copy(mouth); fade(flashB, t > 1.9 && t < 2.15 ? 1 - span(t, 1.9, 2.15) : 0);
        const sm = span(t, 1.9, 2.9);
        smoke.position.copy(mouth).add(V(0.3 * sm, 0.4 * sm, 0)); smoke.scale.setScalar(0.4 + sm * 1.2); fade(smoke, t > 1.9 ? 1 - sm : 0);
        // 1.9-3.3: the flight, curled up and turning, on a high arc
        const fl = span(t, 1.9, 3.3);
        const air = t >= 1.9 && t < 3.3;
        if (t >= 1.1 && t < 1.9) h.g.visible = false;
        else h.g.visible = true;
        if (air) {
          const p = mouth.clone().lerp(target, fl); p.y += Math.sin(fl * Math.PI) * 3.4 + (1 - fl) * 0.2;
          h.holder.position.copy(p);
          h.holder.rotation.set(0, 90 * D, 0);
          h.g.rotation.x = fl * Math.PI * 4;
          h.g.scale.setScalar(m1(0.7 + 0.2 * Math.sin(fl * Math.PI)));
          castPose(h.rig, 1, "R", "up");
          trail.forEach((s, i) => {
            const u = Math.max(0, fl - (i + 1) * 0.035);
            const q = mouth.clone().lerp(target, u); q.y += Math.sin(u * Math.PI) * 3.4 + (1 - u) * 0.2 + 0.5;
            s.position.copy(q); fade(s, 0.9 - i * 0.12);
          });
        } else trail.forEach((s) => fade(s, 0));
        // 3.3: the landing, a squash and a ring of dust; then up again
        const land = span(t, 3.3, 3.75);
        if (t >= 3.3) {
          h.holder.position.copy(target); h.holder.rotation.set(0, 90 * D, 0);
          const k = Math.sin(land * Math.PI) * 0.3;
          h.g.scale.set(1 + k * 0.6, 1 - k, 1 + k * 0.6);
        }
        dust.position.set(target.x, 0.04, target.z); dust.scale.setScalar(0.4 + easeOut(land) * 1.6); fade(dust, t > 3.3 && land < 1 ? 1 - land : 0);
      },
    };
  },
};
function m1(x) { return Math.max(0.05, x); }
NOTES.cannon_launch = {
  see: "A hero walks up to the cannon by the fountain and hops into the barrel. The cannon turns toward a gold ring on the lane, leans back and fires with a flash and a puff of smoke; the hero flies curled up along a high arc with a trail of sparks, and lands in the ring with a squash and a ring of dust.",
  tell: "The gold ring shows where the hero will land from the moment the cannon is aimed until the landing, to both sides, as the teleport's landing glow does.",
};

// The cannon aimed: what the player sees while choosing where to land.
ABILITIES.cannon_aim = {
  T: 2.4, still: 1.2, fps: 12,
  setup(st) {
    const can = prep(unit(st, "cannon_sun", "sun", -6.4, 0.2, 90, { ringR: 1.1 }));
    const tower = prep(unit(st, "tower_sun", "sun", 4.6, -2.4, 270, { ringR: 1.4 }));
    const h = prep(unit(st, "hero_cinder", "sun", -7.6, 1.5, 90));
    const all = [can, tower, h];
    // the reach: a band down the lane from the base to the outer tower
    const band = ready(own(group(new THREE.Mesh(new THREE.PlaneGeometry(9.6, 2.6), soft(C.sun, 0.16)))));
    band.children[0].rotation.x = -Math.PI / 2;
    band.position.set(0.6, 0.03, 0.3);
    const edge = ready(own(group(
      new THREE.Mesh(new THREE.PlaneGeometry(9.6, 0.06), soft(C.sun, 0.9)),
      new THREE.Mesh(new THREE.PlaneGeometry(9.6, 0.06), soft(C.sun, 0.9)),
      new THREE.Mesh(new THREE.PlaneGeometry(0.08, 2.6), soft(C.sun, 0.9)),
    )));
    edge.children.forEach((c) => { c.rotation.x = -Math.PI / 2; });
    edge.children[0].position.set(0, 0.035, -1.3); edge.children[1].position.set(0, 0.035, 1.3); edge.children[2].position.set(4.8, 0.035, 0);
    edge.position.set(0.6, 0, 0.3);
    st.scene.add(band, edge);
    const mark = landing(C.sun, 0.9); st.scene.add(mark);
    const arc = dottedArc(C.sun, 22); st.scene.add(arc);
    return {
      look: [-0.6, 1.2, 0.2], view: [0, 0.75, 1], dist: 17.5,
      update(t) {
        restAll(t, all);
        const u = 0.5 - 0.5 * Math.cos((t / 2.4) * Math.PI * 2);
        const target = V(lerp(-1.6, 4.4, u), 0, lerp(0.9, -0.2, u));
        mark.position.copy(target); mark.scale.setScalar(0.9 + Math.sin(t * 8) * 0.05);
        const mouth = wpos(can.at.mouth);
        can.turn.rotation.y = Math.atan2(target.z - 0.2, target.x + 6.4) * -1 * 0.3;
        arc.userData.place(mouth, target.clone().add(V(0, 0.3, 0)), 2.4 + u * 1.2, 1, (t * 5) % 1);
      },
    };
  },
};
NOTES.cannon_aim = {
  see: "With the cannon chosen, a pale gold band shows the ground it can reach: the lane from the base out to the side's outer tower. A gold ring follows the pointer inside the band, with a dotted arc from the barrel to it.",
  tell: "The band ends at the outer tower, so a thrown hero can rejoin the fight but cannot be thrown past it into the enemy.",
};

// A health relic taken: the flask bursts into green motes over the hero,
// a heal ring opens, and the empty stones show a timer filling.
ABILITIES.relic_pickup = {
  T: 3.6, still: 1.45, fps: 12,
  setup(st) {
    const rel = prep(unit(st, "relic", "neutral", 0.6, 0, 0, { ring: false }));
    const h = prep(unit(st, "hero_kestrel", "sun", -2.6, 0.2, 90));
    const all = [rel, h];
    const R = rng(7);
    const motes = [];
    for (let i = 0; i < 12; i++) { const m = halo(0x7cf09a, 0.22, 0.9); m.userData.r = [R(), R(), R()]; st.scene.add(m); motes.push(m); }
    const ring = ready(own(group(groundRing(0.5, 0.62, soft(0x7cf09a, 0.9), 0.03)))); st.scene.add(ring);
    const timer = ready(own(group(new THREE.Mesh(new THREE.RingGeometry(0.5, 0.58, 40, 1, 0, Math.PI * 2), soft(0x7cf09a, 0.8)))));
    timer.children[0].rotation.x = -Math.PI / 2; timer.position.set(0.6, 0.05, 0); st.scene.add(timer);
    const plus = [];
    for (let i = 0; i < 4; i++) { const p = own(group(limb([-0.07, 0, 0], [0.07, 0, 0], 0.025, glow(0x7cf09a, 1.3), { ink: false }), limb([0, -0.07, 0], [0, 0.07, 0], 0.025, glow(0x7cf09a, 1.3), { ink: false }))); ready(p); st.scene.add(p); plus.push(p); }
    return {
      look: [-0.4, 0.6, 0.2], view: [0, 0.6, 1], dist: 8.5,
      update(t) {
        restAll(t, all);
        const w = span(t, 0, 1.2);
        h.holder.position.set(lerp(-2.6, 0.6, ease(w)), 0, 0.2 - 0.2 * ease(w));
        if (w < 1) walk(h.rig, t * 1.4);
        const take = 1.15;
        rel.flask.visible = t < take;
        const b = span(t, take, take + 0.8);
        motes.forEach((m) => {
          const [a, s, k] = m.userData.r;
          const out = easeOut(b) * (0.4 + s * 0.5);
          m.position.set(0.6 + Math.cos(a * 6.3) * out, 0.85 + b * (0.4 + k) , Math.sin(a * 6.3) * out);
          fade(m, t > take && b < 1 ? 1 - b : 0);
        });
        const hr = span(t, take + 0.1, take + 1.0);
        ring.position.copy(h.holder.position); ring.scale.setScalar(0.6 + easeOut(hr) * 0.8); fade(ring, t > take && hr < 1 ? 1 - hr : 0);
        plus.forEach((p, i) => {
          const u = span(t, take + 0.15 + i * 0.15, take + 1.1 + i * 0.15);
          p.position.copy(h.holder.position).add(V(Math.cos(i * 1.7) * 0.35, 1.0 + u * 0.9, Math.sin(i * 1.7) * 0.35));
          fade(p, u > 0 && u < 1 ? Math.sin(u * Math.PI) : 0);
        });
        if (t > take && t < take + 0.25) tint(h.g, 0x7cf09a, 0.5 * (1 - (t - take) / 0.25)); else tint(h.g, 0xffffff, 0);
        // the timer: the ring fills as the relic's return draws near (drawn quick)
        const tm = span(t, take + 0.3, 3.5);
        timer.children[0].geometry.dispose();
        timer.children[0].geometry = new THREE.RingGeometry(0.5, 0.58, 40, 1, Math.PI / 2, Math.max(0.001, tm * Math.PI * 2));
        fade(timer, t > take + 0.3 ? 0.9 : 0);
      },
    };
  },
};
NOTES.relic_pickup = {
  see: "A hero walks through a floating green flask. It bursts into green motes, a green ring opens under the hero and small plus signs float up as health returns. The empty stones then show a green ring filling until the flask comes back.",
  tell: "Only the stones remain while the relic is away, and the filling ring says how long until it is back.",
};

// The random draw: ten heroes turn on a ring, slow, and stop on one, who
// hops forward into a gold light. Drawn for the pick screen.
ABILITIES.random_reveal = {
  T: 4.4, still: 3.6, fps: 12,
  setup(st) {
    const ids = ["hero_ironvow", "hero_tectus", "hero_gristle", "hero_kestrel", "hero_thistle", "hero_longwick", "hero_rime", "hero_cinder", "hero_hexwright", "hero_tempest"];
    const ms = ids.map((id) => prep(unit(st, id, "sun", 0, 0, 0, { ringR: 0.38 })));
    const n = ms.length, R0 = 3.2, pick = 7;
    const spot = ready(own(group(groundRing(0.55, 0.75, soft(C.gold, 0.9), 0.03), disc(0.55, 0, 0.025, 0, soft(C.gold, 0.25)))));
    st.scene.add(spot);
    const beam = ready(own(group(new THREE.Mesh(new THREE.CylinderGeometry(0.75, 0.75, 6, 28, 1, true), soft(C.gold, 0.18)))));
    st.scene.add(beam);
    const sparks = [];
    const R = rng(3);
    for (let i = 0; i < 10; i++) { const s = halo(C.gold, 0.2, 0.9); s.userData.r = [R(), R()]; st.scene.add(s); sparks.push(s); }
    return {
      look: [0, 0.7, 0.8], view: [0, 0.42, 1], dist: 11.5,
      update(t) {
        restAll(t, ms);
        // the ring's turn: fast, then easing to a stop with the pick in front
        const stop = 2.6;
        const u = Math.min(t, stop) / stop;
        const turns = 2.25;
        // what is left of the turn: the chosen hero ends in front (+z)
        const left = (1 - u) * (1 - u) * (1 - u) * turns * Math.PI * 2;
        ms.forEach((m, i) => {
          const th = Math.PI / 2 + ((i - pick) / n) * Math.PI * 2 + left;
          m.holder.position.set(Math.cos(th) * R0, 0, Math.sin(th) * R0 * 0.55);
          m.holder.rotation.y = 0;
          const near = (Math.sin(th) + 1) / 2;
          m.holder.scale.setScalar(0.75 + near * 0.35);
          tint(m.g, 0x2b2140, t > stop && i !== pick ? 0.45 * span(t, stop, stop + 0.4) : 0);
        });
        const chosen = ms[pick];
        const rise = span(t, stop + 0.1, stop + 0.55);
        if (rise > 0) {
          const p = chosen.holder.position.clone();
          chosen.holder.position.set(p.x, Math.sin(rise * Math.PI) * 0.5, p.z + easeOut(rise) * 1.2);
          chosen.holder.scale.setScalar(1.1 + easeOut(rise) * 0.25);
          if (rise < 1) hop(chosen.rig, rise, 0.3);
          else castPose(chosen.rig, Math.min(1, span(t, stop + 0.6, stop + 1.0)), "R", "up");
        }
        const lit = span(t, stop + 0.3, stop + 0.7);
        spot.position.set(chosen.holder.position.x, 0, chosen.holder.position.z); fade(spot, lit);
        beam.position.set(chosen.holder.position.x, 3, chosen.holder.position.z); fade(beam, lit * (0.8 + Math.sin(t * 6) * 0.2));
        sparks.forEach((s) => {
          const [a, b] = s.userData.r;
          const v = (t * 0.7 + b) % 1;
          s.position.set(chosen.holder.position.x + Math.cos(a * 6.3) * 0.6, v * 2.6, chosen.holder.position.z + Math.sin(a * 6.3) * 0.6);
          fade(s, lit * Math.sin(v * Math.PI));
        });
      },
    };
  },
};
NOTES.random_reveal = {
  see: "On the pick screen the ten heroes turn on a ring, quickly at first, then slow and stop with one in front. The others dim; the chosen hero hops forward into a column of gold light and raises a hand.",
  tell: "Everyone sees the draw at the same moment; the hero in the light is yours.",
};

// All mid: both sides' five heroes, the waves and two relics, from the
// game's own camera pitch.
ABILITIES.aram_mid = {
  T: 2, still: 0.7, fps: 12,
  setup(st) {
    const put = (id, team, x, z, ry, o) => { try { return prep(unit(st, id, team, x, z, ry, o)); } catch (e) { return null; } };
    const R = rng(13);
    for (let i = 0; i < 20; i++) {
      const x = -17 + i * 1.8 + (R() - 0.5) * 0.6;
      for (const z of [-5.2 - R() * 1.5, 6.0 + R() * 1.5]) put(x < 0 ? "tree_sun" : "tree_moon", "neutral", x, z, R() * 360, { ring: false });
    }
    put("tower_sun", "sun", -12.5, -2.8, 90, { ringR: 1.6 });
    put("tower_moon", "moon", 13, -2.8, 270, { ringR: 1.6 });
    put("relic", "neutral", -6.5, 3.6, 0, { ring: false });
    put("relic", "neutral", 7.0, 3.6, 0, { ring: false });
    const fighters = [];
    for (const [team, x0, dir] of [["sun", -0.8, 1], ["moon", 0.8, -1]]) {
      for (let i = 0; i < 3; i++) fighters.push(put(`creep_melee_${team}`, team, x0 + dir * (i % 2) * 0.3, -1.6 + i * 1.1, dir > 0 ? 90 : 270, { ringR: 0.32 }));
      for (let i = 0; i < 2; i++) fighters.push(put(`creep_ranged_${team}`, team, x0 - dir * 1.4, -1.0 + i * 1.2, dir > 0 ? 90 : 270, { ringR: 0.32 }));
    }
    const sunH = ["hero_ironvow", "hero_rime", "hero_thistle", "hero_gristle", "hero_tempest"];
    const moonH = ["hero_tectus", "hero_cinder", "hero_kestrel", "hero_hexwright", "hero_longwick"];
    const heroes = [];
    sunH.forEach((id, i) => heroes.push(put(id, "sun", -3.2 - (i % 2) * 1.6, -2.4 + i * 1.2, 75)));
    moonH.forEach((id, i) => heroes.push(put(id, "moon", 3.4 + (i % 2) * 1.6, -2.2 + i * 1.2, 255)));
    const all = [...fighters, ...heroes].filter(Boolean);
    const others = [];
    st.scene.traverse((o) => { if (o.userData && o.userData.update) others.push(o); });
    return {
      look: [0, 0, 0.6], view: [0, Math.tan((56 * Math.PI) / 180), 1], dist: 17,
      update(t) {
        restAll(t, all);
        fighters.filter(Boolean).forEach((m, i) => swing(m.rig, (t * 0.9 + i * 0.29) % 1));
      },
    };
  },
};
NOTES.aram_mid = {
  see: "The one lane at full strength: five heroes a side behind their creep waves, the towers at either end, and a health relic on each side of the middle.",
  tell: "",
};
