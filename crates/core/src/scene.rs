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
/// The style the water's shapes carry.
pub const STYLE_WATER: i32 = 9;
/// Surface corners to a tree cell's side (round 7): every half cell.
pub const SURFACE_SUB: i32 = 2;

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

/// The height of the drawn ground under a map point: the surface's corners
/// round it, blended by where the point lies between them, so a unit stands
/// on the faceted ground as drawn.
pub fn ground_at(t: &Terrain, p: P) -> i32 {
    let step = (t.tree_cell / SURFACE_SUB).max(1);
    let (k, j) = (p[0].div_euclid(step), p[1].div_euclid(step));
    let (u, v) = (
        p[0].rem_euclid(step) * ONE / step,
        p[1].rem_euclid(step) * ONE / step,
    );
    let row = |j: i32| {
        let (a, b) = (t.surface_corner(k, j), t.surface_corner(k + 1, j));
        a + fx::mul(b - a, u)
    };
    let (a, b) = (row(j), row(j + 1));
    a + fx::mul(b - a, v)
}

/// A seeded unevenness in -16..16 for a surface corner at map point (x, y),
/// the same in every browser.
pub fn surface_jitter(x: i32, y: i32) -> i32 {
    let h = ((x as u32).wrapping_mul(73_856_093) ^ (y as u32).wrapping_mul(19_349_663))
        .wrapping_mul(2_654_435_761);
    ((h >> 20) & 31) as i32 - 16
}

/// The drawn ground's corner heights for a map (round 7), in Q16 scene
/// units, `SURFACE_SUB` corners to a tree cell's side. A corner where high
/// ground meets low ground (neither a ramp) is high, so a plateau keeps its
/// edge and drops to the low ground within half a cell: a cliff. Elsewhere a corner
/// blends the cells within three quarters of a cell of it, so ramps, river
/// banks and the riverbed slope. Every corner carries a small seeded
/// unevenness, so the facets catch the light.
pub fn surface(level: &[u8], mark: &[u8], tw: i32, tc: i32) -> Vec<i32> {
    let n = tw * SURFACE_SUB + 1;
    let step = tc / SURFACE_SUB;
    let cell = |x: i32, y: i32| (y.clamp(0, tw - 1) * tw + x.clamp(0, tw - 1)) as usize;
    let top = |i: usize| match mark.get(i).copied().unwrap_or(map::T_GRASS) {
        map::T_RAMP => level_height(map::HIGH) / 2,
        map::T_RIVER => level_height(map::RIVER) - ONE / 2,
        _ => level_height(level.get(i).copied().unwrap_or(map::LOW)),
    };
    let ramp = |i: usize| mark.get(i) == Some(&map::T_RAMP);
    let high = |i: usize| level.get(i) == Some(&map::HIGH) && !ramp(i);
    let low = |i: usize| level.get(i) != Some(&map::HIGH) && !ramp(i);
    let mut out = Vec::with_capacity((n * n) as usize);
    for j in 0..n {
        for k in 0..n {
            let (px, py) = (k * step, j * step);
            // the cells whose square holds this corner, edges included
            let (gx, gy) = (px / tc, py / tc);
            let xs = if px % tc == 0 { [gx - 1, gx] } else { [gx, gx] };
            let ys = if py % tc == 0 { [gy - 1, gy] } else { [gy, gy] };
            // a cliff's top edge: high ground on one side, low on the other
            let touches = |f: &dyn Fn(usize) -> bool| xs.iter().any(|&x| ys.iter().any(|&y| f(cell(x, y))));
            let on_cliff = touches(&high) && touches(&low);
            let h = if on_cliff {
                level_height(map::HIGH)
            } else {
                let (mut wsum, mut acc) = (0i64, 0i64);
                for yy in gy - 1..=gy + 1 {
                    for xx in gx - 1..=gx + 1 {
                        let (dx, dy) = ((px - xx * tc - tc / 2) as i64, (py - yy * tc - tc / 2) as i64);
                        let d = fx::isqrt((dx * dx + dy * dy) as u64) as i32;
                        let w = (tc * 3 / 4 - d).max(0) as i64;
                        wsum += w;
                        acc += top(cell(xx, yy)) as i64 * w;
                    }
                }
                (acc / wsum.max(1)) as i32
            };
            out.push(h + surface_jitter(px, py) * ONE / 160);
        }
    }
    out
}

/// A unit's drawn scale, Q16: its own scale, the scene's, and for a lane
/// creep the creeps' share.
pub fn drawn_scale(lk: &Look, d: &crate::rules::UnitDef) -> i32 {
    let kind = if d.kind == Kind::Creep {
        lk.creep_scale_pct
    } else {
        100
    };
    (d.scale as i64 * lk.unit_scale as i64 * kind as i64 * ONE as i64 / 1_000_000) as i32
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

/// `a` moved the Q16 share `k` of the way to `b`.
fn mix(a: Rgb, b: Rgb, k: i32) -> Rgb {
    [
        a[0] + fx::mul(b[0] - a[0], k),
        a[1] + fx::mul(b[1] - a[1], k),
        a[2] + fx::mul(b[2] - a[2], k),
    ]
}

/// A seeded number below 100 for a patch corner: the same in every browser.
fn corner_roll(px: i32, py: i32) -> i32 {
    let h = (px as u32).wrapping_mul(73_856_093) ^ (py as u32).wrapping_mul(19_349_663);
    (h.wrapping_mul(2_654_435_761) >> 16) as i32 % 100
}

/// The lane's dirt at cell (x, y), Q16 from dark (0) to light (ONE): the
/// rolls at the four corners of its patch, blended across the patch so the
/// shade changes smoothly from cell to cell.
fn dirt_at(x: i32, y: i32, size: i32) -> i32 {
    let n = size.max(1);
    let (px, py) = (x.div_euclid(n), y.div_euclid(n));
    let (u, v) = (x.rem_euclid(n) * ONE / n, y.rem_euclid(n) * ONE / n);
    let row = |py: i32| corner_roll(px, py) * (ONE - u) + corner_roll(px + 1, py) * u;
    let both = (row(py) as i64 * (ONE - v) as i64 + row(py + 1) as i64 * v as i64) >> 16;
    (both / 100) as i32
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
    /// Percent of their size lane creeps are drawn at, so they read apart
    /// from heroes (round 7). Drawing only: collision is the rules' radius.
    pub creep_scale_pct: i32,
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
    /// Soft light (round 2), hundredths: how far light wraps past the
    /// terminator, where the gentle ramp starts and ends, and how much of
    /// that ramp is mixed in (0 smooth, 100 two-toned).
    pub bands: [i32; 4],
    /// The fill from the sky above, hundredths of each colour.
    pub ambient: [i32; 3],
    /// The bounce from the ground below, hundredths of each colour.
    pub ground: [i32; 3],
    /// How long the order marker shows, hundredths of a second (T10).
    pub ping: i32,
    /// How each attack's missile looks (T6), by the shooter's unit id.
    #[serde(default)]
    pub shots: crate::marks::Shots,
    /// How each ability looks while it happens (`marks`), by ability id.
    #[serde(default)]
    pub fx: crate::marks::Book,
    pub max_ticks_per_frame: i32,
    /// The map's look (T19).
    pub map_look: MapLook,
    /// The title screen's backdrop match (round 7).
    pub backdrop: crate::backdrop::BackdropLook,
    /// The heroes screen's inspector (round 7, T34).
    pub showcase: crate::showcase::ShowcaseLook,
}

/// The look of Mage Tower Battle's newer version (journal tab 10): faceted
/// models with no outline on soft shadows, and the ground's dressing. T19
/// gave it to the map; round 7 to every model, characters included.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapLook {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// Models kept smooth and outlined; every other model is drawn
    /// faceted, with no outline and a soft shadow.
    pub keep_smooth: Vec<String>,
    /// The shadow disc: percent of a building's footprint across, and how
    /// dark (percent); a tree's disc across, hundredths of a scene unit.
    pub shadow_pct: i32,
    pub shadow_alpha: i32,
    pub tree_shadow: i32,
    /// Percent of open grass cells that carry dressing, and the most pieces
    /// on one cell.
    pub dress_pct: i32,
    pub dress_most: i32,
    /// Percent of brightness one grass cell may differ from the next.
    pub vary_pct: i32,
    /// The lanes' ground (round 7).
    pub lane: LaneLook,
    /// The faceted ground (round 7).
    pub ground: GroundLook,
}

/// The faceted ground (round 7, journal tab 11): numbers the surface, the
/// water and the cliffs are drawn with.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroundLook {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// Percent of the way the riverbed's dirt goes to the water's colour.
    pub bed_pct: i32,
    /// Percent darker the forest floor is under a standing tree.
    pub floor_pct: i32,
    /// How much a corner's seeded unevenness shades it (sixteenths of this
    /// many percent at the most).
    pub shade_pct: i32,
    /// Percent of the way a steep face turns to rock.
    pub steep_pct: i32,
    /// Hundredths of a scene unit the water stands above the river's level.
    pub water_lift: i32,
    /// Boulders along each cliff side of a cell, and their least size in
    /// hundredths of a scene unit.
    pub cliff_rocks: i32,
    pub rock_size: i32,
}

/// The lanes' ground (round 7): patches of darker and lighter dirt a few
/// cells across, edges that blend into the grass on both sides, grass
/// creeping in at the edges, and worn flagstones down the middle, so a
/// lane reads as a trodden road rather than one flat strip.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneLook {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// Cells across one patch of dirt.
    pub patch: i32,
    /// Percent of the way the darkest dirt goes to `lane_dark`, and the
    /// lightest to `lane_light`.
    pub dark_pct: i32,
    pub light_pct: i32,
    /// Percent of the way a cell at the lane's edge blends toward the
    /// ground beside it, with two or more sides touching.
    pub edge_pct: i32,
    /// Percent of the lane's inner cells that carry flagstones, and the
    /// most on one cell.
    pub stone_pct: i32,
    pub stone_most: i32,
    /// Percent of edge cells where grass creeps onto the lane.
    pub creep_pct: i32,
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

/// The ground and trees near the camera (round 7: one faceted surface).
/// Each triangle of the surface is the core's flat triangle mesh stretched
/// onto three corners of `Terrain::surface`; its colour blends the cells
/// around its corners, so the edges between grass, lane, river and base
/// are gradients and the fog's edge is soft. Steep faces turn to rock,
/// faceted boulders line the foot of every cliff, and the river is one
/// flat sheet the sloping banks run down into.
pub fn ground(f: &Frame, b: &mut Batch) {
    let t = f.ctx.terrain;
    let tc = t.tree_cell;
    let lk = f.look;
    let (cx, cy) = (f.view.target[0], f.view.target[1]);
    let x0 = ((cx - lk.reach_x) / tc).max(0);
    let x1 = ((cx + lk.reach_x) / tc).min(t.tw - 1);
    let y0 = ((cy - lk.reach_south) / tc).max(0);
    let y1 = ((cy + lk.reach_north) / tc).min(t.tw - 1);
    if x1 < x0 || y1 < y0 {
        return;
    }
    let fog = lk.fog_pct * ONE / 100;
    let m = f.models;
    let grass = [m.named("grass_sun"), m.named("grass_moon")];
    let lane = m.named("lane");
    let water = m.named("water");
    let stone = [m.named("stone_sun"), m.named("stone_moon")];
    let path = m.named("path");
    let ll = &lk.map_look.lane;
    let gl = &lk.map_look.ground;
    let lane_dark = m.named("lane_dark");
    let lane_light = m.named("lane_light");
    let bed = mix(lane_dark, water, gl.bed_pct * ONE / 100);
    let tree_models = [m.find("tree_sun"), m.find("tree_moon")];
    // one margin cell round the window, so corners on its edge blend fully
    let (mx0, my0) = ((x0 - 1).max(0), (y0 - 1).max(0));
    let (mx1, my1) = ((x1 + 1).min(t.tw - 1), (y1 + 1).min(t.tw - 1));
    let cw = mx1 - mx0 + 1;
    // each cell's colour before fog, and how lit it is (fog or not)
    let mut cells: Vec<(Rgb, i32)> = Vec::with_capacity((cw * (my1 - my0 + 1)) as usize);
    for y in my0..=my1 {
        for x in mx0..=mx1 {
            let i = (y * t.tw + x) as usize;
            let c = t.tree_centre(i);
            let side = river_side(&f.ctx.rules.map.river, c);
            let vary = ONE - ((i as i32 * 7919) & 15) * lk.map_look.vary_pct * ONE / 1500;
            let col = match t.mark[i] {
                map::T_LANE => {
                    // patches of darker and lighter dirt, blended cell to cell
                    let d = dirt_at(x, y, ll.patch);
                    let c0 = if d < ONE / 2 {
                        mix(lane, lane_dark, (ONE - d * 2) * ll.dark_pct / 100)
                    } else {
                        mix(lane, lane_light, (d * 2 - ONE) * ll.light_pct / 100)
                    };
                    edge_blend(c0, grass[side], 4 - lane_sides(t, x, y), ll.edge_pct)
                }
                map::T_RIVER => bed,
                map::T_BASE | map::T_RAMP => stone[side],
                kind => {
                    let mut c0 = if kind == map::T_PATH { path } else { grass[side] };
                    // the forest floor is darker under standing trees
                    if kind == map::T_GRASS && matches!(f.w.trees[i], Tree::Standing) {
                        c0 = mulc(c0, ONE - gl.floor_pct * ONE / 100);
                    }
                    edge_blend(c0, lane, lane_sides(t, x, y), ll.edge_pct / 2)
                }
            };
            let lit = if visible_cell(f, i) { ONE } else { fog };
            cells.push((mulc(col, vary), lit));
        }
    }
    let cell = |x: i32, y: i32| cells[((y.clamp(my0, my1) - my0) * cw + x.clamp(mx0, mx1) - mx0) as usize];
    // the corners of the window's surface: position, colour and light, the
    // colour blended from the cells within a cell and a quarter of it
    let sub = SURFACE_SUB;
    let step = tc / sub;
    let (nx, ny) = ((x1 - x0 + 1) * sub + 1, (y1 - y0 + 1) * sub + 1);
    let mut verts: Vec<(V3, Rgb, i32)> = Vec::with_capacity((nx * ny) as usize);
    for j in 0..ny {
        for k in 0..nx {
            let (px, py) = (x0 * tc + k * step, y0 * tc + j * step);
            let (gx, gy) = (px.div_euclid(tc), py.div_euclid(tc));
            let (mut wsum, mut acc, mut lit) = (0i64, [0i64; 3], 0i64);
            for yy in gy - 1..=gy + 1 {
                for xx in gx - 1..=gx + 1 {
                    let (dx, dy) = ((px - xx * tc - tc / 2) as i64, (py - yy * tc - tc / 2) as i64);
                    let d = fx::isqrt((dx * dx + dy * dy) as u64) as i32;
                    let wgt = (tc * 5 / 4 - d).max(0) as i64;
                    if wgt == 0 {
                        continue;
                    }
                    let (c, l) = cell(xx, yy);
                    wsum += wgt;
                    for (a, v) in acc.iter_mut().zip(c) {
                        *a += v as i64 * wgt;
                    }
                    lit += l as i64 * wgt;
                }
            }
            let ws = wsum.max(1);
            let mut col = [(acc[0] / ws) as i32, (acc[1] / ws) as i32, (acc[2] / ws) as i32];
            // the same seeded unevenness the heights carry, in the shade
            col = mulc(col, ONE + surface_jitter(px, py) * gl.shade_pct * ONE / 1600);
            let h = t.surface_corner(x0 * sub + k, y0 * sub + j);
            verts.push((to_scene([px, py], h), col, (lit / ws) as i32));
        }
    }
    let rock = m.named("cliff_rock");
    for j in 0..ny - 1 {
        for k in 0..nx - 1 {
            let v = |a: i32, bb: i32| verts[((j + bb) * nx + k + a) as usize];
            let q = [v(0, 0), v(1, 0), v(1, 1), v(0, 1)];
            // split along alternating diagonals, so the facets do not all lean one way
            let tris = if (j + k) % 2 == 0 {
                [[q[0], q[1], q[2]], [q[0], q[2], q[3]]]
            } else {
                [[q[0], q[1], q[3]], [q[1], q[2], q[3]]]
            };
            for tri in tris {
                terrain_tri(b, tri, rock, gl.steep_pct, 6);
            }
        }
    }
    // the water: one flat sheet over every cell within a cell of the river,
    // lit at the same corners as the ground, so its fog has no steps
    let sheet = level_height(map::RIVER) + ONE * gl.water_lift / 100;
    let wet = |x: i32, y: i32| {
        (-1..=1).any(|dy| {
            (-1..=1).any(|dx| {
                let (nx, ny) = (x + dx, y + dy);
                nx >= 0
                    && ny >= 0
                    && nx < t.tw
                    && ny < t.tw
                    && t.mark[(ny * t.tw + nx) as usize] == map::T_RIVER
            })
        })
    };
    for j in 0..ny - 1 {
        for k in 0..nx - 1 {
            if !wet(x0 + k / sub, y0 + j / sub) {
                continue;
            }
            let v = |a: i32, bb: i32| {
                let (p, _, lit) = verts[((j + bb) * nx + k + a) as usize];
                (v3(p.x, sheet, p.z), water, lit)
            };
            let q = [v(0, 0), v(1, 0), v(1, 1), v(0, 1)];
            terrain_tri(b, [q[0], q[1], q[2]], water, 0, STYLE_WATER);
            terrain_tri(b, [q[0], q[2], q[3]], water, 0, STYLE_WATER);
        }
    }
    for y in y0..=y1 {
        for x in x0..=x1 {
            let i = (y * t.tw + x) as usize;
            let c = t.tree_centre(i);
            let side = river_side(&f.ctx.rules.map.river, c);
            let seen = visible_cell(f, i);
            let dim = if seen { ONE } else { fog };
            let top = ground_at(t, c);
            cliff_rocks(f, b, x, y, dim);
            if matches!(f.w.trees[i], Tree::None) && t.mark[i] != map::T_RIVER {
                // open grass takes every kind; the lanes, paths and bases
                // only stones and pebbles, and fewer of them
                let open = t.mark[i] == map::T_GRASS;
                dress(f, b, i, c, top, dim, open);
                if t.mark[i] == map::T_LANE {
                    road(f, b, i, c, top, dim, lane_sides(t, x, y) < 4, grass[side]);
                }
            }
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
                    if model.faceted {
                        let d = fx::mul(lk.map_look.tree_shadow * ONE / 100, scale);
                        shadow(f, b, to_scene(c, top), d);
                    }
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

/// Faceted boulders along the foot of a cliff (round 7): on a low cell,
/// along each side it shares with high ground that is not a ramp. Seeded by
/// the cell, so every browser draws the same rocks.
fn cliff_rocks(f: &Frame, b: &mut Batch, x: i32, y: i32, dim: i32) {
    let t = f.ctx.terrain;
    let i = (y * t.tw + x) as usize;
    if t.level[i] == map::HIGH || t.mark[i] == map::T_RAMP {
        return;
    }
    let gl = &f.look.map_look.ground;
    let tc = t.tree_cell;
    let c = t.tree_centre(i);
    for (s, (dx, dy)) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().enumerate() {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= t.tw || ny >= t.tw {
            continue;
        }
        let k = (ny * t.tw + nx) as usize;
        if t.level[k] != map::HIGH || t.mark[k] == map::T_RAMP {
            continue;
        }
        let mut h = (i as u32 * 4 + s as u32).wrapping_mul(2_654_435_761) ^ 0x2545_f491;
        let mut next = |n: u32| {
            h ^= h << 13;
            h ^= h >> 17;
            h ^= h << 5;
            (h % n.max(1)) as i32
        };
        for r in 0..gl.cliff_rocks {
            // spread along the shared side, just in front of the face
            let along = -tc / 2 + (r * 2 + 1) * tc / (gl.cliff_rocks * 2).max(1) + next(17) - 8;
            let out = tc / 2 - tc / 5 - next(tc as u32 / 6);
            let at = [
                c[0] + dx * out + dy.abs() * along,
                c[1] + dy * out + dx.abs() * along,
            ];
            let size = ONE * gl.rock_size / 100 + next(40) * ONE / 100;
            let shade = ONE - next(18) * ONE / 100;
            let a = Affine::translate(to_scene(at, ground_at(t, at) + size / 4))
                .then(&Affine::rot_y(next(fx::TURN as u32)))
                .then(&Affine::rot_z(next(30) * fx::TURN / 360 - fx::TURN / 24))
                .then(&Affine::scale(v3(size, size * 6 / 5, size * 4 / 5)));
            let col = if r % 2 == 0 {
                "cliff_rock"
            } else {
                "cliff_rock_dark"
            };
            b.push(
                LAYER_PLAIN,
                Mesh::Facet,
                &a,
                mulc(mulc(f.models.named(col), shade), dim),
                0,
            );
        }
    }
}

/// One triangle of the faceted ground: the core's flat triangle mesh
/// stretched onto three points, lit by its own slope, its colour the mean
/// of its corners', turning `steep_pct` percent of the way to rock where it
/// is steeper than about 35°.
fn terrain_tri(b: &mut Batch, tri: [(V3, Rgb, i32); 3], rock: Rgb, steep_pct: i32, style: i32) {
    let (p0, mut p1, mut p2) = (tri[0].0, tri[1].0, tri[2].0);
    let mid = |p1: V3, p2: V3| v3((p1.x + p2.x) / 2, (p1.y + p2.y) / 2, (p1.z + p2.z) / 2);
    let mut c0 = p1.sub(p2);
    let mut c2 = p0.sub(mid(p1, p2));
    // keep the reference triangle's turn, so its face is not culled
    if (c0.x as i64 * c2.z as i64 - c0.z as i64 * c2.x as i64) < 0 {
        core::mem::swap(&mut p1, &mut p2);
        c0 = p1.sub(p2);
        c2 = p0.sub(mid(p1, p2));
    }
    let mut n = p1.sub(p0).cross(p2.sub(p0)).norm();
    if n.y < 0 {
        n = n.neg();
    }
    let avg = |k: usize| (tri[0].1[k] + tri[1].1[k] + tri[2].1[k]) / 3;
    let mut col = [avg(0), avg(1), avg(2)];
    let steep = (ONE * 4 / 5 - n.y).clamp(0, ONE / 5) * 5;
    col = mix(col, rock, steep * steep_pct / 100);
    col = mulc(col, (tri[0].2 + tri[1].2 + tri[2].2) / 3);
    let a = Affine { c0, c1: n, c2, t: p0 };
    b.push(LAYER_PLAIN, Mesh::Tri, &a, col, style);
}

/// The map's soft shadow (T19): a dark see-through disc `d` across, on the
/// ground at `at`. The renderer casts no shadows, as Mage Tower's does not.
pub fn shadow(f: &Frame, b: &mut Batch, at: V3, d: i32) {
    // two discs, the inner one two thirds across: darkest in the middle,
    // so the edge reads soft (Mage Tower's shadows are soft too)
    let alpha = f.look.map_look.shadow_alpha * ONE / 200;
    for (k, lift) in [(ONE, ONE / 50), (ONE * 2 / 3, ONE / 40)] {
        let w = fx::mul(d, k);
        let a = Affine::translate(at.add(v3(0, lift, 0))).then(&Affine::scale(v3(w, ONE / 100, w)));
        b.tint(Mesh::Facet, &a, f.models.named("dress_shadow"), alpha);
    }
}

/// Seeded dressing on one open grass cell (T19): stones, tufts and
/// flowers at three sizes, each with its own place, size and turn, so the
/// ground does not look tiled. Fixed by the cell's index, so every browser
/// draws the same and nothing moves between frames.
fn dress(f: &Frame, b: &mut Batch, i: usize, c: P, top: i32, dim: i32, open: bool) {
    let lk = &f.look.map_look;
    let m = f.models;
    let mut h = (i as u32).wrapping_mul(2_654_435_761) ^ 0x9e37_79b9;
    let mut next = |n: u32| {
        h ^= h << 13;
        h ^= h >> 17;
        h ^= h << 5;
        (h % n.max(1)) as i32
    };
    if next(100) >= if open { lk.dress_pct } else { lk.dress_pct / 3 } {
        return;
    }
    let tc = f.ctx.terrain.tree_cell;
    let pieces = 1 + next(lk.dress_most.max(1) as u32);
    for _ in 0..pieces {
        let at = [c[0] - tc / 2 + next(tc as u32), c[1] - tc / 2 + next(tc as u32)];
        let turn = next(fx::TURN as u32);
        let base = to_scene(at, top);
        let shade = ONE - next(16) * ONE / 100;
        // off the grass, only a stone (the first 12) or a pebble (78 and up)
        let kind = match next(100) {
            k if open => k,
            k if k < 30 => k * 12 / 30,
            k => 78 + k % 22,
        };
        let put = |b: &mut Batch, mesh: Mesh, off: V3, size: V3, col: Rgb, tilt: i32| {
            let a = Affine::translate(base.add(off))
                .then(&Affine::rot_y(turn))
                .then(&Affine::rot_z(tilt))
                .then(&Affine::scale(size));
            b.push(
                crate::render::LAYER_PLAIN,
                mesh,
                &a,
                mulc(mulc(col, shade), dim),
                0,
            );
        };
        if kind < 12 {
            // a boulder, the large size
            let s = ONE * 2 / 5 + next(40) * ONE / 100;
            put(
                b,
                Mesh::Facet,
                v3(0, s / 5, 0),
                v3(s * 4 / 3, s * 3 / 4, s),
                m.named("dress_stone"),
                next(20) * fx::TURN / 360,
            );
        } else if kind < 60 {
            // a tuft: three blades leaning out
            for k in 0..3 {
                let lean = (k - 1) * fx::TURN / 24;
                let tall = ONE / 4 + next(20) * ONE / 100;
                put(
                    b,
                    Mesh::FacetCone,
                    v3((k - 1) * ONE / 14, tall / 2, 0),
                    v3(ONE / 12, tall, ONE / 12),
                    m.named("dress_tuft"),
                    lean,
                );
            }
        } else if kind < 78 {
            // a flower: a stalk and a head
            let col = ["dress_flower_a", "dress_flower_b", "dress_flower_c"][next(3) as usize];
            put(
                b,
                Mesh::FacetCyl,
                v3(0, ONE / 6, 0),
                v3(ONE / 40, ONE / 3, ONE / 40),
                m.named("dress_tuft"),
                0,
            );
            put(
                b,
                Mesh::Facet,
                v3(0, ONE / 3, 0),
                v3(ONE / 8, ONE / 10, ONE / 8),
                m.named(col),
                0,
            );
        } else {
            // a pebble, the small size
            let s = ONE / 5 + next(16) * ONE / 100;
            put(
                b,
                Mesh::Facet,
                v3(0, s / 6, 0),
                v3(s, s / 2, s * 4 / 5),
                m.named("dress_stone"),
                0,
            );
        }
    }
}

/// A cell at the lane's edge moved toward the ground across the edge:
/// `pct` percent of the way with two or more of its sides touching it,
/// half that with one.
fn edge_blend(col: Rgb, toward: Rgb, sides: i32, pct: i32) -> Rgb {
    mix(col, toward, pct * sides.clamp(0, 2) * ONE / 200)
}

/// How many of a cell's four neighbours are lane.
fn lane_sides(t: &map::Terrain, x: i32, y: i32) -> i32 {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            nx >= 0 && ny >= 0 && nx < t.tw && ny < t.tw && t.mark[(ny * t.tw + nx) as usize] == map::T_LANE
        })
        .count() as i32
}

/// A lane cell's road (round 7): worn flagstones set flat in the middle of
/// the lane, and grass creeping in at its edges. Seeded by the cell's
/// index, so it is the same in every browser and every frame.
#[allow(clippy::too_many_arguments)]
fn road(f: &Frame, b: &mut Batch, i: usize, c: P, top: i32, dim: i32, edge: bool, grass: Rgb) {
    let ll = &f.look.map_look.lane;
    let m = f.models;
    let mut h = (i as u32).wrapping_mul(2_246_822_519) ^ 0x85eb_ca6b;
    let mut next = |n: u32| {
        h ^= h << 13;
        h ^= h >> 17;
        h ^= h << 5;
        (h % n.max(1)) as i32
    };
    let tc = f.ctx.terrain.tree_cell;
    let put = |b: &mut Batch, mesh: Mesh, at: P, lift: i32, turn: i32, tilt: i32, size: V3, col: Rgb| {
        let a = Affine::translate(to_scene(at, top).add(v3(0, lift, 0)))
            .then(&Affine::rot_y(turn))
            .then(&Affine::rot_z(tilt))
            .then(&Affine::scale(size));
        b.push(crate::render::LAYER_PLAIN, mesh, &a, mulc(col, dim), 0);
    };
    if edge {
        if next(100) >= ll.creep_pct {
            return;
        }
        // a clump of grass blades, a shade of the bank it grows from
        let at = [
            c[0] - tc / 3 + next(tc as u32 * 2 / 3),
            c[1] - tc / 3 + next(tc as u32 * 2 / 3),
        ];
        let col = mix(grass, m.named("dress_tuft"), ONE / 2);
        for k in 0..2 + next(3) {
            let tall = ONE / 6 + next(16) * ONE / 100;
            let off = [at[0] + (k - 1) * 6, at[1] + next(9) - 4];
            put(
                b,
                Mesh::FacetCone,
                off,
                tall / 2,
                next(fx::TURN as u32),
                (k - 1) * fx::TURN / 20,
                v3(ONE / 11, tall, ONE / 11),
                col,
            );
        }
        return;
    }
    if next(100) >= ll.stone_pct {
        return;
    }
    // flagstones: flat, a little sunk, each its own size, turn and shade
    for _ in 0..1 + next(ll.stone_most.max(1) as u32) {
        let at = [
            c[0] - tc / 3 + next(tc as u32 * 2 / 3),
            c[1] - tc / 3 + next(tc as u32 * 2 / 3),
        ];
        let s = ONE * 9 / 10 + next(50) * ONE / 100;
        let shade = ONE - next(14) * ONE / 100;
        put(
            b,
            Mesh::Facet,
            at,
            ONE / 40,
            next(fx::TURN as u32),
            0,
            v3(s, ONE / 10, s * 3 / 4),
            mulc(m.named("lane_stone"), shade),
        );
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
            // the wind-up to the hit, then the follow-through, both timed
            // as the rules time them at this unit's attack speed (T7)
            let s = crate::stats::of(f.ctx, f.w, id);
            let speed = 100 + s.attack_speed;
            let point = (d.attack_point * TPS * 100 / (100 * speed)).max(1) * ONE;
            let back = (d.backswing * TPS * 100 / (100 * speed)).max(TPS / 4) * ONE;
            if since < point {
                p.swing = fx::div(since, point);
            } else if since < point + back {
                p.recover = fx::div(since - point, back).max(1);
            }
        }
        ANIM_CAST | ANIM_CHANNEL => {
            let rise = ONE * 6;
            p.cast = fx::div(since.min(rise), rise);
            // the arms stay up while a long cast (a teleport) runs
            if u.anim == ANIM_CAST && since > ONE * 20 && !matches!(u.act, Act::Cast { .. }) {
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
pub(crate) fn drawn_at(f: &Frame, u: &crate::world::Unit) -> (i32, i32, i32) {
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
        let scale = drawn_scale(lk, d);
        // a hero thrown by the cannon: hidden in the barrel, then high on
        // its arc and tumbling (All Random All Mid)
        let (lift, tumble, flying) = match u.act {
            Act::Flight { up, .. } if f.w.tick < up => continue,
            Act::Flight { up, land, .. } => {
                let (k, lift) = crate::overlay::flight_arc(f, up, land);
                (lift, fx::mul(k, fx::TURN * 2), true)
            }
            _ => (0, 0, false),
        };
        let half = fx::mul(model.height, scale) / 2;
        let place = Affine::translate(sub_to_scene(x, y, h + lift + if flying { half } else { 0 }))
            .then(&Affine::rot_y(face))
            .then(&Affine::rot_z(-tumble))
            .then(&Affine::translate(v3(0, if flying { -half } else { 0 }, 0)))
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
                    let layer = if pt.plain { LAYER_PLAIN } else { LAYER_INKED };
                    b.push(layer, pt.mesh, &a, m.color(u.team, pt.color), pt.style);
                });
                continue;
            }
        }
        if model.faceted && !flying && !invisible && !illusion {
            let d = (d.radius + d.footprint) * 2 * lk.map_look.shadow_pct / 100 * ONE / GAME_UNITS_PER_SCENE;
            shadow(f, b, sub_to_scene(x, y, h), d.max(ONE));
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
        if !d.kind.is_building() && d.kind != Kind::Ward && !flying {
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
            crate::marks::draw_status(f, b, u, sub_to_scene(x, y, h));
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
    for ms in &f.w.missiles {
        let p = [ms.x / SUB, ms.y / SUB];
        let h = ground_at(f.ctx.terrain, p) + ONE * 3 / 2;
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
        crate::marks::draw_missile(f, b, sub_to_scene(ms.x, ms.y, h), ms);
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
        if crate::marks::draw_zone(f, b, z) {
            continue;
        }
        let col = ability_color(f, k);
        let r = z.radius.max(60) * 2 * ONE / GAME_UNITS_PER_SCENE;
        let h = ground_at(f.ctx.terrain, z.at) + ONE / 12;
        // a delayed blast shows its circle until it lands
        let a = Affine::translate(to_scene(z.at, h)).then(&Affine::scale(v3(r, ONE / 6, r)));
        b.tint(Mesh::Cyl, &a, col, ONE * 2 / 5);
    }
    crate::marks::draw_marks(f, b);
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
                    let layer = if pt.plain { LAYER_PLAIN } else { LAYER_INKED };
                    b.push(layer, pt.mesh, &a, m.color(NEUTRAL, pt.color), pt.style);
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
