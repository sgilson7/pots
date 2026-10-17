// Routes one mockup per address and exposes frame(t) so the shooter can
// step time itself: ?model=<id>, ?lineup=<id,id,...>, ?ability=<id>,
// ?scene=lane. Every picture is a pure function of t, so frames repeat.
import { THREE, makeRenderer, makeScene, plinth, aim, SHEET_VIEW, GAME_VIEW, resetRig, idle, lanePatch, APE, blob } from "./kit.js";
import { HEROES } from "./heroes.js";

const q = new URLSearchParams(location.search);
const canvas = document.getElementById("gl");
let MODELS = { ...HEROES };
let ABILITIES = {};
async function load() {
  for (const f of ["./world.js", "./aram.js"]) {
    try { const m = await import(f); MODELS = { ...MODELS, ...m.MODELS }; } catch (e) { if (!String(e).includes("Failed to fetch")) throw e; }
  }
  for (const f of ["./fx_a.js", "./fx_b.js", "./aram.js"]) {
    try { const m = await import(f); ABILITIES = { ...ABILITIES, ...m.ABILITIES }; } catch (e) { if (!String(e).includes("Failed to fetch")) throw e; }
  }
}
/** Builds a model by id; returns { g, rig?, at?, ... }. */
export function build(id, team = "sun") {
  const f = MODELS[id];
  if (!f) throw new Error("no model " + id);
  const m = f(team);
  return m.isObject3D ? { g: m } : m;
}
window.build = build;

function boundsOf(o) {
  const b = new THREE.Box3();
  o.updateMatrixWorld(true);
  o.traverse((c) => { if (c.isMesh && !c.userData.ink && c.geometry) { c.geometry.computeBoundingBox(); const bb = c.geometry.boundingBox.clone().applyMatrix4(c.matrixWorld); b.union(bb); } });
  return b;
}

async function main() {
  await load();
  window.MODEL_IDS = Object.keys(MODELS);
  window.ABILITY_IDS = Object.keys(ABILITIES);
  const W = +(q.get("w") || 1000), H = +(q.get("h") || 750);
  const renderer = makeRenderer(canvas, W, H);
  const camera = new THREE.PerspectiveCamera(+(q.get("fov") || 30), W / H, 0.05, 200);
  let frame = () => {};
  if (q.get("model")) {
    const id = q.get("model");
    const m = build(id, q.get("team") || "sun");
    const b0 = boundsOf(m.g);
    const size = b0.getSize(new THREE.Vector3());
    const pr = Math.max(0.6, Math.max(size.x, size.z) * 0.75 + 0.25);
    const big = Math.max(size.x, size.y * 0.9, size.z, pr * 1.45);
    // a model sheet keeps the haze behind the model, on the sky
    const { scene } = makeScene({ shadowBox: Math.max(3, big * 1.4), fog: [big * 8, big * 20] });
    const base = plinth(pr);
    scene.add(base, m.g);
    if (APE) scene.add(blob(Math.max(size.x, size.z) * 0.45 + 0.15, 0.34));
    const ctr = b0.getCenter(new THREE.Vector3());
    const at = [ctr.x * 0.5, size.y * 0.45, ctr.z * 0.5];
    const dist = (big / (2 * Math.tan((camera.fov * Math.PI) / 360))) * 1.55 + 0.45;
    aim(camera, at, SHEET_VIEW, dist);
    frame = (t) => {
      if (m.rig) { resetRig(m.rig); idle(m.rig, t); }
      if (m.update) m.update(t);
      renderer.render(scene, camera);
    };
  } else if (q.get("lineup")) {
    const ids = q.get("lineup").split(",");
    const { scene } = makeScene({ shadowBox: ids.length * 1.2 + 2, sunAt: [3, 9, 7] });
    scene.add(lanePatch(ids.length * 2.2 + 6, 7));
    const ms = ids.map((id, i) => { const m = build(id, q.get("team") || "sun"); m.g.position.x = (i - (ids.length - 1) / 2) * 1.9; scene.add(m.g); return m; });
    aim(camera, [0, 0.75, 0], [0, 0.35, 1], ids.length * 2.25 + 2);
    frame = (t) => { ms.forEach((m, i) => { if (m.rig) { resetRig(m.rig); idle(m.rig, t + i * 0.7); } if (m.update) m.update(t); }); renderer.render(scene, camera); };
  } else if (q.get("ability")) {
    const id = q.get("ability");
    const A = ABILITIES[id];
    if (!A) throw new Error("no ability " + id);
    const { scene, sun } = makeScene({ shadowBox: 8, sunAt: [4, 10, 6] });
    scene.add(lanePatch(34, 26));
    const st = { scene, camera, build, sun, renderer };
    const fx = A.setup(st);
    aim(camera, fx.look ?? [0, 0.6, 0], fx.view ?? GAME_VIEW, fx.dist ?? 13);
    window.ABILITY = { T: A.T, still: A.still, fps: A.fps ?? 12 };
    frame = (t) => { fx.update(t); renderer.render(scene, camera); };
  } else if (q.get("scene")) {
    const m = await import("./scenes.js");
    frame = m.scene(q.get("scene"), { renderer, camera, build });
  }
  window.frame = (t) => { frame(t); return true; };
  window.frame(+(q.get("t") || 0.5));
  document.body.dataset.drawn = "1";
}
main().catch((e) => { console.error(e.stack || String(e)); document.body.dataset.error = String(e); });
