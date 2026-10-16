// Exports every mockup model's rest pose as parts for the game:
// tools/modelport.py turns this into data/models.json. Each part is a
// three.js shape (with its own parameters), its matrix relative to the
// model's root, the game joint it moves with, its colour and whether it is
// outlined, glowing or see-through. Run by `shoot.py export`.
import { THREE, resetRig } from "./kit.js";

const JOINTS = [["head", "head"], ["armL", "arm_l"], ["armR", "arm_r"], ["legL", "leg_l"], ["legR", "leg_r"], ["body", "body"]];

function exportOne(build, id, team) {
  const m = build(id, team);
  const root = m.g;
  if (m.rig) resetRig(m.rig);
  root.position.set(0, 0, 0);
  root.updateMatrixWorld(true);
  const inv = new THREE.Matrix4().copy(root.matrixWorld).invert();
  const jointOf = new Map();
  const pivots = {};
  if (m.rig) {
    for (const [k, name] of JOINTS) {
      const j = m.rig[k];
      if (!j || !j.parent) continue;
      jointOf.set(j, name);
      const p = new THREE.Vector3().setFromMatrixPosition(j.matrixWorld).applyMatrix4(inv);
      pivots[name] = [p.x, p.y, p.z];
    }
  }
  const parts = [];
  root.traverse((c) => {
    if (!c.isMesh || c.userData.ink) return;
    let joint = "body";
    for (let p = c; p && p !== root; p = p.parent) if (jointOf.has(p) && jointOf.get(p) !== "body") { joint = jointOf.get(p); break; }
    const mat = c.material;
    const g = c.geometry;
    const rel = new THREE.Matrix4().multiplyMatrices(inv, c.matrixWorld);
    const emissive = mat.emissive && mat.emissiveIntensity > 0 && mat.emissive.getHex() !== 0;
    parts.push({
      geo: g.type, params: g.parameters || {}, m: rel.elements,
      color: "#" + mat.color.getHexString(),
      ink: c.children.some((k) => k.userData.ink),
      glow: !!emissive || mat.blending === THREE.AdditiveBlending,
      veil: !!mat.transparent && mat.opacity < 0.95,
      joint,
    });
  });
  return { parts, pivots };
}

export function exportAll(build, ids) {
  const out = {};
  for (const id of ids) out[id] = { sun: exportOne(build, id, "sun"), moon: exportOne(build, id, "moon") };
  return out;
}
