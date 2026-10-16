//! What the 3D view shows: the ground, the trees, every unit the local side
//! can see, missiles, spell areas, health bars and the fog.
//!
//! One scene unit is `GAME_UNITS_PER_SCENE` map units, so the whole map
//! fits in Q16 with room to spare. Scene x is map x, scene −z is map y
//! (north is away from the camera), y is up. The camera looks north and
//! down, like Warcraft III's default camera. Only what lies near the camera
//! is listed each frame, so a forest of thousands of trees costs only the
//! few hundred in view.

use crate::fx::{self, ONE};
use crate::geom::{v3, Affine, V3};
use crate::map::{self, Terrain, Tree};
use crate::mesh::Mesh;
use crate::models::{pose_parts, Models, Pose};
use crate::render::{Batch, Camera, Rgb, LAYER_INKED, LAYER_PLAIN};
use crate::rules::{Kind, P, SUB, TPS};
use crate::stats;
use crate::world::{
    Act, Ctx, Uid, World, ANIM_ATTACK, ANIM_CAST, ANIM_CHANNEL, ANIM_DEATH, ANIM_WALK, M, NEUTRAL, NO_UNIT,
};

pub const GAME_UNITS_PER_SCENE: i32 = 32;

/// The shader style drawn unlit (glows, the Spheres' cores, runes).
pub const STYLE_GLOW: i32 = 4;

/// Height of each ground level in Q16 scene units.
pub fn level_height(level: u8) -> i32 {
    match level {
        map::RIVER => -ONE * 3 / 4,
        map::HIGH => ONE * 2,
        _ => 0,
    }
}

/// A map point (map units) at a height (Q16 scene units) as a scene point.
pub fn to_scene(p: P, h: i32) -> V3 {
    v3(
        p[0] * ONE / GAME_UNITS_PER_SCENE,
        h,
        -p[1] * ONE / GAME_UNITS_PER_SCENE,
    )
}

/// Sixteenths of a map unit to a scene point at ground height.
pub fn sub_to_scene(x: i32, y: i32, h: i32) -> V3 {
    v3(
        (x as i64 * ONE as i64 / (GAME_UNITS_PER_SCENE * SUB) as i64) as i32,
        h,
        -(y as i64 * ONE as i64 / (GAME_UNITS_PER_SCENE * SUB) as i64) as i32,
    )
}

/// The height of the ground under a map point, smoothed on ramps.
pub fn ground_at(t: &Terrain, p: P) -> i32 {
    let lv = t.level_at(p[0], p[1]);
    if t.mark_at(p[0], p[1]) == map::T_RAMP {
        return level_height(map::HIGH) / 2;
    }
    level_height(lv)
}

/// The model index carried by an attack missile: its source's kind.
pub fn missile_model(_ctx: &Ctx, src: Uid, w: &World) -> u16 {
    w.get(src).map(|u| u.def).unwrap_or(0)
}

/// The model index carried by an ability missile: 1000 + the ability.
pub fn ability_missile(_ctx: &Ctx, k: usize) -> u16 {
    1000 + k as u16
}

/// The MOBA camera: a point on the map it looks at, how far back it stands,
/// and how steeply it looks down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct View {
    /// Map units.
    pub target: P,
    /// Distance from eye to target, Q16 scene units.
    pub distance: i32,
    /// Angle below the horizon, angle units.
    pub pitch: i32,
    pub fov: i32,
    /// Turn about the vertical axis (0 looks north), for model sheets.
    pub yaw: i32,
}

impl View {
    pub fn camera(&self, ground: i32) -> Camera {
        let t = to_scene(self.target, ground);
        // the eye stands back (south, +z) and up from the target
        let back = fx::mul(fx::cos(self.pitch), self.distance);
        let up = fx::mul(fx::sin(self.pitch), self.distance);
        let (s, c) = (fx::sin(self.yaw), fx::cos(self.yaw));
        let eye = v3(t.x + fx::mul(s, back), t.y + up, t.z + fx::mul(c, back));
        let fwd = t.sub(eye).norm();
        Camera {
            eye,
            fwd,
            fov: self.fov,
            far: self.distance * 6,
            up: V3::UP,
        }
    }

    /// The map point under a screen point given in percent (Q16) from the
    /// left and top, by casting a ray from the eye to the ground plane at
    /// `ground` height.
    pub fn pick(&self, ground: i32, aspect: i32, sx: i32, sy: i32) -> P {
        let cam = self.camera(ground);
        let fwd = cam.fwd;
        let right = fwd.cross(V3::UP).norm();
        let up = right.cross(fwd);
        let half = fx::div(fx::sin(self.fov / 2), fx::cos(self.fov / 2));
        // screen percent to -1..1
        let nx = (sx / 50) - ONE;
        let ny = ONE - (sy / 50);
        let dir = fwd
            .add(right.scale(fx::mul(fx::mul(nx, half), aspect)))
            .add(up.scale(fx::mul(ny, half)));
        if dir.y >= 0 {
            return self.target;
        }
        let t = fx::div(ground - cam.eye.y, dir.y);
        let hit = cam.eye.add(dir.scale(t));
        [
            hit.x * GAME_UNITS_PER_SCENE / ONE,
            -hit.z * GAME_UNITS_PER_SCENE / ONE,
        ]
    }
}

fn mulc(c: Rgb, k: i32) -> Rgb {
    [fx::mul(c[0], k), fx::mul(c[1], k), fx::mul(c[2], k)]
}

/// Numbers the scene reads from `data/presentation.json`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Look {
    /// How far around the camera target to list ground and trees, map units.
    pub reach_x: i32,
    pub reach_south: i32,
    pub reach_north: i32,
    /// Percent brightness of ground and trees under fog.
    pub fog_pct: i32,
    /// Health bar sizes in hundredths of a scene unit.
    pub bar_hero: [i32; 2],
    pub bar_unit: [i32; 2],
    pub bar_building: [i32; 2],
    /// Missile size, hundredths of a scene unit.
    pub missile: i32,
    /// Percent of scene: model scale.
    pub unit_scale: i32,
    /// The camera: distance in hundredths of a scene unit, pitch and field of view in degrees.
    pub cam_distance: i32,
    pub cam_pitch: i32,
    pub cam_fov: i32,
    /// Map units a second the camera pans, and the edge band (percent of the screen) that pans it.
    pub pan_speed: i32,
    pub edge: i32,
    pub zoom_min: i32,
    pub zoom_max: i32,
    pub zoom_step: i32,
    /// Lighting: direction in hundredths, colours 0–255.
    pub light_dir: [i32; 3],
    pub sky: [i32; 3],
    pub fog: [i32; 3],
    /// Fog start and end, map units at the default camera distance.
    pub fog_range: [i32; 2],
    pub light: [i32; 3],
    pub ink: [i32; 3],
    /// Outline width: least, growth per scene unit, most; hundredths.
    pub ink_size: [i32; 3],
    /// Lit bands: thresholds and brightness, hundredths.
    pub bands: [i32; 4],
    pub ambient: [i32; 3],
    pub max_ticks_per_frame: i32,
}

pub struct Frame<'a> {
    pub ctx: &'a Ctx<'a>,
    pub w: &'a World,
    pub models: &'a Models,
    pub look: &'a Look,
    /// The side whose vision the view shows; NEUTRAL shows everything.
    pub team: u8,
    /// Q16 share of the way to the next tick, for smooth motion.
    pub alpha: i32,
    /// Positions one tick back, by unit slot (x, y, facing), for smoothing.
    pub prev: &'a [(Uid, i32, i32, i32)],
    pub hover: Uid,
    pub selected: Uid,
    pub view: View,
}

fn visible_cell(f: &Frame, i: usize) -> bool {
    if f.team == NEUTRAL {
        return true;
    }
    f.w.vision.get(i).map(|v| v & (1 << f.team) != 0).unwrap_or(false)
}

/// 0 for the Sunward bank of the river, 1 for the Moonward: the sign of
/// the cross product with the river's first-to-last line.
pub fn river_side(river: &[P], p: P) -> usize {
    let (a, b) = match (river.first(), river.last()) {
        (Some(a), Some(b)) => (*a, *b),
        _ => return 0,
    };
    let cross = (b[0] - a[0]) as i64 * (p[1] - a[1]) as i64 - (b[1] - a[1]) as i64 * (p[0] - a[0]) as i64;
    if cross < 0 {
        0
    } else {
        1
    }
}

/// The ground and trees near the camera.
pub fn ground(f: &Frame, b: &mut Batch) {
    let t = f.ctx.terrain;
    let tc = t.tree_cell;
    let lk = f.look;
    let (cx, cy) = (f.view.target[0], f.view.target[1]);
    let x0 = ((cx - lk.reach_x) / tc).max(0);
    let x1 = ((cx + lk.reach_x) / tc).min(t.tw - 1);
    let y0 = ((cy - lk.reach_south) / tc).max(0);
    let y1 = ((cy + lk.reach_north) / tc).min(t.tw - 1);
    let fog = lk.fog_pct * ONE / 100;
    let m = f.models;
    let grass = [m.named("grass_sun"), m.named("grass_moon")];
    let lane = m.named("lane");
    let water = m.named("water");
    let stone = [m.named("stone_sun"), m.named("stone_moon")];
    let path = m.named("path");
    let tree_models = [m.find("tree_sun"), m.find("tree_moon")];
    for y in y0..=y1 {
        for x in x0..=x1 {
            let i = (y * t.tw + x) as usize;
            let c = t.tree_centre(i);
            // which bank of the river the cell is on, by the river's own line
            let side = river_side(&f.ctx.rules.map.river, c);
            let lv = t.level[i];
            let top = if t.mark[i] == map::T_RAMP {
                level_height(map::HIGH) / 2
            } else {
                level_height(lv)
            };
            // a little seeded variation per cell keeps the grass from looking tiled
            let vary = ONE - ((i as i32 * 7919) & 7) * ONE / 120;
            let mut col = match t.mark[i] {
                map::T_LANE => lane,
                map::T_RIVER => water,
                map::T_BASE | map::T_RAMP => stone[side],
                map::T_PATH => path,
                _ => grass[side],
            };
            col = mulc(col, vary);
            let seen = visible_cell(f, i);
            if !seen {
                col = mulc(col, fog);
            }
            let bottom = -ONE * 3;
            let h = top - bottom;
            let at = to_scene(c, bottom + h / 2);
            let w = tc * ONE / GAME_UNITS_PER_SCENE;
            let a = Affine::translate(at).then(&Affine::scale(v3(w, h, w)));
            b.push(
                LAYER_PLAIN,
                Mesh::Cube,
                &a,
                col,
                if t.mark[i] == map::T_RIVER { 9 } else { 6 },
            );
            if let Tree::Standing = f.w.trees[i] {
                if let Some(mi) = tree_models[side] {
                    let model = &m.list[mi];
                    // seeded size and turn per tree
                    let k = (i as i32).wrapping_mul(2_654_435_761u32 as i32);
                    let scale = ONE * 9 / 10 + ((k >> 8) & 15) * ONE / 80;
                    let turn = (k >> 4) & (fx::TURN - 1);
                    let place = Affine::translate(to_scene(c, top))
                        .then(&Affine::rot_y(turn))
                        .then(&Affine::scale(v3(scale, scale, scale)));
                    pose_parts(model, &Pose::STILL, &place, |p, a| {
                        let mut col = m.color(NEUTRAL, p.color);
                        if !seen {
                            col = mulc(col, fog);
                        }
                        b.push(
                            if p.plain { LAYER_PLAIN } else { LAYER_INKED },
                            p.mesh,
                            &a,
                            col,
                            p.style,
                        );
                    });
                }
            }
        }
    }
}

fn pose_of(f: &Frame, id: Uid) -> Pose {
    let Some(u) = f.w.get(id) else { return Pose::STILL };
    let tick = f.w.tick as i32;
    let since = (tick - u.anim_at as i32).max(0) * ONE + f.alpha;
    let d = &f.ctx.rules.units[u.def as usize];
    let mut p = Pose::STILL;
    match u.anim {
        ANIM_WALK => {
            // one stride for every 200 map units walked
            let per_sec = d.speed.max(1);
            p.walk =
                ((since as i64 * per_sec as i64 * fx::TURN as i64) / (200 * TPS as i64 * ONE as i64)) as i32;
        }
        ANIM_ATTACK => {
            let point = (d.attack_point * TPS / 100).max(1) * ONE;
            p.swing = fx::div(since.min(point), point).clamp(0, ONE);
            if since > point * 2 {
                p.swing = 0;
            }
        }
        ANIM_CAST | ANIM_CHANNEL => {
            let rise = ONE * 6;
            p.cast = fx::div(since.min(rise), rise);
            if u.anim == ANIM_CAST && since > ONE * 20 {
                p.cast = 0;
            }
        }
        ANIM_DEATH => {
            let fall = ONE * 18;
            p.death = fx::div(since.min(fall), fall);
        }
        _ => {}
    }
    if u.mods.iter().any(|m| m.kind == M::SpellImmune && m.tag != 0)
        && matches!(u.act, Act::None)
        && u.has(M::Disarm)
    {
        // a spinning hero (the Blade Fury of DotA)
        p.whirl = (tick * ONE + f.alpha) / ONE * (fx::TURN / 10);
    }
    p.spin = (tick * fx::TURN / (4 * TPS)) & (fx::TURN - 1);
    p
}

/// Where a unit is drawn this frame: between its last and current tick.
fn drawn_at(f: &Frame, u: &crate::world::Unit) -> (i32, i32, i32) {
    let slot = World::slot(u.id);
    match f.prev.get(slot) {
        Some(&(id, px, py, pf))
            if id == u.id && (px - u.x).abs() < 300 * SUB && (py - u.y).abs() < 300 * SUB =>
        {
            let x = px + fx::mul(u.x - px, f.alpha);
            let y = py + fx::mul(u.y - py, f.alpha);
            let face = fx::lerp_angle(pf, u.facing, f.alpha);
            (x, y, face)
        }
        _ => (u.x, u.y, u.facing),
    }
}

/// Whether the local side sees a unit (its own side always).
pub fn shown(f: &Frame, u: &crate::world::Unit) -> bool {
    if f.team == NEUTRAL || u.team == f.team {
        return true;
    }
    let kind = f.ctx.rules.units[u.def as usize].kind;
    // buildings stay drawn under fog once the map is known, as in DotA
    if kind.is_building() {
        return true;
    }
    f.w.sees(f.ctx, f.team, u)
}

pub fn units(f: &Frame, b: &mut Batch) {
    let m = f.models;
    let lk = f.look;
    let fog = lk.fog_pct * ONE / 100;
    for u in &f.w.units {
        let dying = !u.alive && u.anim == ANIM_DEATH && f.w.tick < u.anim_at + (3 * TPS) as u32;
        if !u.alive && !dying {
            continue;
        }
        if u.hero.is_some() && !u.alive && !dying {
            continue;
        }
        if !shown(f, u) {
            continue;
        }
        let d = &f.ctx.rules.units[u.def as usize];
        let Some(mi) = m.find(&d.model) else { continue };
        let model = &m.list[mi];
        let (x, y, face) = drawn_at(f, u);
        let p = [x / SUB, y / SUB];
        let h = ground_at(f.ctx.terrain, p);
        let scale = d.scale * lk.unit_scale * ONE / 10_000;
        let place = Affine::translate(sub_to_scene(x, y, h))
            .then(&Affine::rot_y(face))
            .then(&Affine::scale(v3(scale, scale, scale)));
        let pose = pose_of(f, u.id);
        let invisible = u.has(M::Invisible);
        let illusion = u.has(M::Illusion);
        let hexed = u.has(M::Hex);
        let cell_seen = f
            .ctx
            .terrain
            .tree_index(p[0], p[1])
            .map(|i| visible_cell(f, i))
            .unwrap_or(true);
        let dim = d.kind.is_building() && !cell_seen && u.team != f.team && f.team != NEUTRAL;
        if hexed {
            // a hexed unit is drawn as the hex's small critter
            if let Some(ci) = m.find("critter") {
                let critter = &m.list[ci];
                let place = Affine::translate(sub_to_scene(x, y, h)).then(&Affine::rot_y(face));
                pose_parts(critter, &pose, &place, |pt, a| {
                    b.push(LAYER_INKED, pt.mesh, &a, m.color(u.team, pt.color), pt.style);
                });
                continue;
            }
        }
        pose_parts(model, &pose, &place, |pt, a| {
            let mut col = m.color(u.team, pt.color);
            if dim {
                col = mulc(col, fog);
            }
            if invisible || illusion {
                // see-through: an invisible ally, or an illusion's blue cast
                let tint = if illusion { m.named("illusion") } else { col };
                b.tint(pt.mesh, &a, tint, ONE / 2);
            } else {
                b.push(
                    if pt.plain { LAYER_PLAIN } else { LAYER_INKED },
                    pt.mesh,
                    &a,
                    col,
                    pt.style,
                );
            }
        });
        // team ring under every unit that is not a building
        if !d.kind.is_building() && d.kind != Kind::Ward {
            let r = (d.radius + 16) * 2 * ONE / GAME_UNITS_PER_SCENE;
            let ring_col = m.team_named(u.team, "ring");
            let a =
                Affine::translate(sub_to_scene(x, y, h + ONE / 30)).then(&Affine::scale(v3(r, ONE / 40, r)));
            b.tint(Mesh::Ring, &a, ring_col, ONE * 3 / 4);
        }
        if u.id == f.hover || u.id == f.selected {
            let r = (d.radius + d.footprint + 40) * 2 * ONE / GAME_UNITS_PER_SCENE;
            let a =
                Affine::translate(sub_to_scene(x, y, h + ONE / 20)).then(&Affine::scale(v3(r, ONE / 20, r)));
            b.tint(Mesh::Ring, &a, m.named("select"), ONE);
        }
        if u.alive {
            bar(f, b, u, x, y, h + fx::mul(model.height, scale));
        }
    }
}

fn bar(f: &Frame, b: &mut Batch, u: &crate::world::Unit, x: i32, y: i32, top: i32) {
    let d = &f.ctx.rules.units[u.def as usize];
    if matches!(d.kind, Kind::Shop | Kind::Ward | Kind::Fountain) {
        return;
    }
    let s = stats::of(f.ctx, f.w, u.id);
    if s.max_hp <= 0 {
        return;
    }
    let size = match d.kind {
        Kind::Hero => f.look.bar_hero,
        k if k.is_building() => f.look.bar_building,
        _ => f.look.bar_unit,
    };
    let (bw, bh) = (size[0] * ONE / 100, size[1] * ONE / 100);
    let share = (u.hp as i64 * ONE as i64 / (s.max_hp as i64 * 1000)).clamp(0, ONE as i64) as i32;
    let m = f.models;
    let facing = Affine::rot_x(-f.view.pitch);
    let at = sub_to_scene(x, y, top + ONE / 3);
    let back =
        Affine::translate(at)
            .then(&facing)
            .then(&Affine::scale(v3(bw + ONE / 16, bh + ONE / 16, ONE)));
    b.push(LAYER_PLAIN, Mesh::Card, &back, m.named("bar_back"), 0);
    let fill_w = fx::mul(bw, share);
    let offset = (fill_w - bw) / 2;
    let col = if u.team == NEUTRAL {
        m.named("bar_neutral")
    } else if u.team == f.team || f.team == NEUTRAL && u.team == 0 {
        m.named("bar_ally")
    } else {
        m.named("bar_enemy")
    };
    let fill = Affine::translate(at)
        .then(&facing)
        .then(&Affine::translate(v3(offset, 0, ONE / 50)))
        .then(&Affine::scale(v3(fill_w.max(1), bh, ONE)));
    b.push(LAYER_PLAIN, Mesh::Card, &fill, col, 4);
    if d.kind == Kind::Hero && s.max_mana > 0 {
        let share = (u.mana as i64 * ONE as i64 / (s.max_mana as i64 * 1000)).clamp(0, ONE as i64) as i32;
        let mw = fx::mul(bw, share);
        let at2 = sub_to_scene(x, y, top + ONE / 3 - bh - ONE / 20);
        let back = Affine::translate(at2).then(&facing).then(&Affine::scale(v3(
            bw + ONE / 16,
            bh / 2 + ONE / 16,
            ONE,
        )));
        b.push(LAYER_PLAIN, Mesh::Card, &back, m.named("bar_back"), 0);
        let fill = Affine::translate(at2)
            .then(&facing)
            .then(&Affine::translate(v3((mw - bw) / 2, 0, ONE / 50)))
            .then(&Affine::scale(v3(mw.max(1), bh / 2, ONE)));
        b.push(LAYER_PLAIN, Mesh::Card, &fill, m.named("bar_mana"), 4);
    }
}

/// Missiles, spell areas, the rune.
pub fn effects(f: &Frame, b: &mut Batch) {
    let m = f.models;
    let size = f.look.missile * ONE / 100;
    for ms in &f.w.missiles {
        let p = [ms.x / SUB, ms.y / SUB];
        let h = ground_at(f.ctx.terrain, p) + ONE * 3 / 2;
        let col = if ms.model >= 1000 {
            let k = (ms.model - 1000) as usize;
            ability_color(f, k)
        } else {
            m.named("shot")
        };
        if f.team != NEUTRAL && ms.team != f.team {
            let seen = f
                .ctx
                .terrain
                .tree_index(p[0], p[1])
                .map(|i| visible_cell(f, i))
                .unwrap_or(false);
            if !seen {
                continue;
            }
        }
        let big = if ms.model >= 1000 { size * 2 } else { size };
        let a = Affine::translate(sub_to_scene(ms.x, ms.y, h)).then(&Affine::scale(v3(big, big, big)));
        b.push(LAYER_INKED, Mesh::Octa, &a, col, 4);
    }
    for z in &f.w.zones {
        let k = z.ability as usize;
        if f.team != NEUTRAL && z.team != f.team {
            let seen = f
                .ctx
                .terrain
                .tree_index(z.at[0], z.at[1])
                .map(|i| visible_cell(f, i))
                .unwrap_or(false);
            if !seen {
                continue;
            }
        }
        let col = ability_color(f, k);
        let r = z.radius.max(60) * 2 * ONE / GAME_UNITS_PER_SCENE;
        let h = ground_at(f.ctx.terrain, z.at) + ONE / 12;
        // a delayed blast shows its circle until it lands
        let a = Affine::translate(to_scene(z.at, h)).then(&Affine::scale(v3(r, ONE / 6, r)));
        b.tint(Mesh::Cyl, &a, col, ONE * 2 / 5);
    }
    if let Some((spot, kind)) = f.w.rune {
        let at = f.ctx.rules.map.runes[spot];
        let seen = f.team == NEUTRAL
            || f.ctx
                .terrain
                .tree_index(at[0], at[1])
                .map(|i| visible_cell(f, i))
                .unwrap_or(false);
        if seen {
            let name = [
                "rune_haste",
                "rune_double",
                "rune_illusion",
                "rune_invis",
                "rune_regen",
            ][kind as usize % 5];
            if let Some(mi) = m.find(name) {
                let h = ground_at(f.ctx.terrain, at);
                let pose = Pose {
                    spin: (f.w.tick as i32 * fx::TURN / (3 * TPS)) & (fx::TURN - 1),
                    ..Pose::STILL
                };
                let place = Affine::translate(to_scene(at, h));
                pose_parts(&m.list[mi], &pose, &place, |pt, a| {
                    b.push(LAYER_INKED, pt.mesh, &a, m.color(NEUTRAL, pt.color), pt.style);
                });
            }
        }
    }
}

fn ability_color(f: &Frame, k: usize) -> Rgb {
    let a = &f.ctx.rules.abilities[k];
    f.models.named(if a.color.is_empty() { "shot" } else { &a.color })
}

/// The camera's ground height: the level under its target.
pub fn view_ground(t: &Terrain, v: &View) -> i32 {
    ground_at(t, v.target)
}

pub fn unit_under(f: &Frame, aspect: i32, sx: i32, sy: i32) -> Uid {
    // the unit whose drawn centre lands nearest the pointer, within a few percent
    let cam = f.view.camera(view_ground(f.ctx.terrain, &f.view));
    let mut best: Option<(i64, Uid)> = None;
    for u in &f.w.units {
        if !u.alive || !shown(f, u) {
            continue;
        }
        let d = &f.ctx.rules.units[u.def as usize];
        if matches!(d.kind, Kind::Shop) {
            continue;
        }
        let (x, y, _) = drawn_at(f, u);
        let p = [x / SUB, y / SUB];
        let h = ground_at(f.ctx.terrain, p) + ONE;
        let Some((ux, uy)) = cam.screen(aspect, sub_to_scene(x, y, h)) else {
            continue;
        };
        let reach = ((d.radius + d.footprint).max(40) * ONE / 16).max(ONE * 2);
        let (dx, dy) = ((ux - sx) as i64, (uy - sy) as i64);
        let d2 = dx * dx + dy * dy;
        if d2 <= (reach as i64) * (reach as i64) && best.map(|b| d2 < b.0).unwrap_or(true) {
            best = Some((d2, u.id));
        }
    }
    best.map(|b| b.1).unwrap_or(NO_UNIT)
}
