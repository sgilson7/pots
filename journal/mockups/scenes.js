// Whole scenes for the journal: a lane at the clash of two creep waves,
// seen from the mockups' angle (?scene=lane) and from the game's own pitch
// of 56° (?scene=lane_game), so the style can be judged where it is played.
import { THREE, makeScene, lanePatch, aim, unit, resetRig, idle, walk, swing, rng, APE, tint } from "./kit.js";

export function scene(name, { renderer, camera, build }) {
  // in the new look the far trees go into the haze, framing the fight
  const { scene } = makeScene({ shadowBox: 16, sunAt: [6, 14, 8], fog: [20, 52] });
  scene.add(lanePatch(48, 34, 5));
  const st = { scene, camera, build };
  const R = rng(9);
  const put = (id, team, x, z, ry, o) => { try { return unit(st, id, team, x, z, ry, o); } catch (e) { return null; } };
  // trees along both edges, Sunward pines to the west, Moonward frost trees east
  for (let i = 0; i < 26; i++) {
    const x = -22 + i * 1.75 + (R() - 0.5) * 0.8;
    for (const z of [-4.6 - R() * 2.5, 6.4 + R() * 2.5]) {
      const t = put(x < 0 ? "tree_sun" : "tree_moon", "neutral", x, z, R() * 360, { ring: false });
      if (t) t.holder.scale.setScalar(0.85 + R() * 0.3);
      // the new look: no two trees the same shade (Ape Race repeats every piece with a different shade, size and tilt)
      if (t && APE) { tint(t.g, R() < 0.5 ? 0x2f5f2a : 0xc9d98a, 0.08 + R() * 0.14); t.holder.rotation.z = (R() - 0.5) * 0.08; }
    }
  }
  put("tower_sun", "sun", -11, -2.3, 90, { ringR: 1.6 });
  put("tower_moon", "moon", 12, -2.6, 270, { ringR: 1.6 });
  const fighters = [];
  const wave = (team, x0, dir) => {
    for (let i = 0; i < 3; i++) fighters.push(put(team === "sun" ? "creep_melee_sun" : "creep_melee_moon", team, x0 + dir * (i % 2) * 0.3, -1.2 + i * 1.2, dir > 0 ? 90 : 270, { ringR: 0.32 }));
    for (let i = 0; i < 2; i++) fighters.push(put(team === "sun" ? "creep_ranged_sun" : "creep_ranged_moon", team, x0 - dir * 1.4, -0.6 + i * 1.3, dir > 0 ? 90 : 270, { ringR: 0.32 }));
  };
  wave("sun", -0.7, 1);
  wave("moon", 0.7, -1);
  const heroes = [
    put("hero_ironvow", "sun", -2.6, 1.9, 70), put("hero_rime", "sun", -4.4, -1.6, 80),
    put("hero_hexwright", "moon", 3.6, 1.8, 250), put("hero_kestrel", "moon", 2.9, -1.7, 290),
  ];
  put("creep_siege_sun", "sun", -5.6, 0.4, 90, { ringR: 0.6 });
  const all = [...fighters, ...heroes].filter(Boolean);
  if (name === "lane_game") aim(camera, [0, 0, 0.2], [0, Math.tan((56 * Math.PI) / 180), 1], 21);
  else aim(camera, [0, 0.3, 0.4], [0, 0.9, 1], 22);
  return (t) => {
    all.forEach((m, i) => {
      if (!m.rig) return;
      resetRig(m.rig);
      idle(m.rig, t + i * 0.37);
      if (fighters.includes(m)) swing(m.rig, ((t * 0.9 + i * 0.29) % 1));
    });
    renderer.render(scene, camera);
  };
}
