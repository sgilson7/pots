//! How an ability looks while it happens (round 2, T3): the cast glow in
//! the caster's hand, missiles, warnings on the ground before a delayed
//! blast, the blast, beams and waves, lingering clouds, and a mark on every
//! unit an ability holds (stars for a stun, a bubble for a silence, frost
//! for a slow, a ring for a buff), after the animations drawn in the
//! journal's round-2 mockups (`journal/mockups/fx_*.js`).
//!
//! Everything here is drawn from the world as it is: the marks the rules
//! leave (`World::marks`), missiles, zones, units' modifiers and channels.
//! Nothing here changes the world. Each ability's look is data
//! (`data/presentation.json`, `fx`): a colour from the palette and a short
//! list of shapes for each moment.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::fx::{self, ONE};
use crate::geom::{v3, Affine, V3};
use crate::mesh::Mesh;
use crate::render::{Batch, Rgb, LAYER_PLAIN};
use crate::rules::{Kind, P, SUB, TPS};
use crate::scene::{ground_at, to_scene, Frame, GAME_UNITS_PER_SCENE, STYLE_GLOW};
use crate::world::{Act, Hit, Mark, Unit, M, MK_CAST, MK_FIRE, MK_LAND, MK_PROC, NO_UNIT};

/// One shape of an ability's look.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shape {
    /// `ring` (a ring opening on the ground), `flash` (a glow bursting at
    /// the point), `sky` (a bolt from above), `beam` (from where it came to
    /// where it lands), `wave` (glows running from where it came to where it
    /// lands), `spikes` (cones thrown up along that line), `rise` (motes
    /// rising round the point), `spin` (a ring turning round the caster).
    pub kind: String,
    /// Size in map units, or the ability value that gives it (`key`).
    #[serde(default)]
    pub r: i32,
    #[serde(default)]
    pub key: String,
    /// How long it shows, hundredths of a second.
    #[serde(default = "short")]
    pub dur: i32,
}

fn short() -> i32 {
    50
}

/// An ability's look: its colour, the glow while it is cast, and the
/// shapes of each moment.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Look {
    pub color: String,
    /// A glow in the caster's hands while the cast point passes.
    #[serde(default)]
    pub cast: bool,
    #[serde(default)]
    pub fire: Vec<Shape>,
    #[serde(default)]
    pub land: Vec<Shape>,
    #[serde(default, rename = "proc")]
    pub procs: Vec<Shape>,
    /// How its missile looks (T6); a glowing ball in its colour when unset.
    #[serde(default)]
    pub missile: Option<Shot>,
    /// How its zones are drawn: `warn` (a pulsing ring until it lands),
    /// `cloud` (puffs round the caster), `rain` (falling shot), `spin`,
    /// `heal` (a soft ring), `flash` (a fading disc), or `none`.
    #[serde(default)]
    pub zone: String,
}

pub type Book = BTreeMap<String, Look>;

/// The decorative sizes below are for a model of scale one; models are
/// drawn at `unit_scale`, so the glows and rings grow with them.
pub(crate) fn big(f: &Frame, x: i32) -> i32 {
    fx::mul(x, f.look.unit_scale * ONE / 100)
}

pub(crate) fn scene_len(map_units: i32) -> i32 {
    map_units * ONE / GAME_UNITS_PER_SCENE
}

/// Q16 ticks since tick `t`, with the frame's share of the next one.
pub(crate) fn age(f: &Frame, t: u32) -> i32 {
    (f.w.tick.saturating_sub(t) as i32) * ONE + f.alpha
}

pub(crate) fn ticks(hundredths: i32) -> i32 {
    (hundredths * TPS / 100).max(1) * ONE
}

fn color(f: &Frame, l: &Look) -> Rgb {
    f.models.named(&l.color)
}

fn look<'a>(f: &'a Frame, k: usize) -> Option<&'a Look> {
    let a = f.ctx.rules.abilities.get(k)?;
    f.look.fx.get(&a.id)
}

fn value(f: &Frame, k: usize, s: &Shape) -> i32 {
    if s.key.is_empty() {
        return s.r;
    }
    let a = &f.ctx.rules.abilities[k];
    a.values
        .get(&s.key)
        .and_then(|v| v.last().copied())
        .unwrap_or(s.r)
}

/// The height of a unit's chest and top, in scene units, as drawn.
pub(crate) fn heights(f: &Frame, u: &Unit) -> (i32, i32) {
    let d = &f.ctx.rules.units[u.def as usize];
    let top = f
        .models
        .find(&d.model)
        .map(|i| f.models.list[i].height)
        .unwrap_or(ONE * 3);
    let scale = d.scale * f.look.unit_scale * ONE / 10_000;
    let top = fx::mul(top, scale);
    (top / 2, top)
}

pub(crate) fn ground(f: &Frame, p: P) -> i32 {
    ground_at(f.ctx.terrain, p)
}

/// A glowing ball: a lit-less core and a see-through halo round it.
pub(crate) fn glow(b: &mut Batch, at: V3, r: i32, col: Rgb, alpha: i32) {
    if r <= 0 || alpha <= 0 {
        return;
    }
    let core = Affine::translate(at).then(&Affine::scale(v3(r, r, r)));
    b.push(LAYER_PLAIN, Mesh::Sphere, &core, col, STYLE_GLOW);
    let halo = Affine::translate(at).then(&Affine::scale(v3(r * 2, r * 2, r * 2)));
    b.tint(Mesh::Sphere, &halo, col, alpha / 3);
}

/// A flat ring on the ground of radius `r`.
pub(crate) fn ring(b: &mut Batch, at: V3, r: i32, col: Rgb, alpha: i32) {
    if r <= 0 || alpha <= 0 {
        return;
    }
    let a = Affine::translate(at.add(v3(0, ONE / 20, 0))).then(&Affine::scale(v3(r * 2, ONE / 4, r * 2)));
    b.tint(Mesh::Ring, &a, col, alpha);
}

/// A flat disc on the ground of radius `r`.
pub(crate) fn disc(b: &mut Batch, at: V3, r: i32, col: Rgb, alpha: i32) {
    if r <= 0 || alpha <= 0 {
        return;
    }
    let a = Affine::translate(at.add(v3(0, ONE / 30, 0))).then(&Affine::scale(v3(r * 2, ONE / 40, r * 2)));
    b.tint(Mesh::Cyl, &a, col, alpha);
}

/// A rod from `a` to `c`, `w` across: beams, bolts, tethers, chains.
pub(crate) fn rod(b: &mut Batch, a: V3, c: V3, w: i32, col: Rgb, alpha: i32) {
    let d = c.sub(a);
    let len = d.len();
    if len <= 0 || alpha <= 0 {
        return;
    }
    let axis = d.norm();
    let helper = if axis.y.abs() > ONE - ONE / 16 {
        v3(ONE, 0, 0)
    } else {
        V3::UP
    };
    let x = helper.cross(axis).norm();
    let z = axis.cross(x);
    let m = Affine {
        c0: x.scale(w),
        c1: axis.scale(len),
        c2: z.scale(w),
        t: a.lerp(c, ONE / 2),
    };
    b.push(LAYER_PLAIN, Mesh::Cyl, &m, col, STYLE_GLOW);
    let halo = Affine {
        c0: x.scale(w * 3),
        c2: z.scale(w * 3),
        ..m
    };
    b.tint(Mesh::Cyl, &halo, col, alpha / 3);
}

/// Where a mark happens and where it came from, in the scene, at chest height.
fn ends(f: &Frame, m: &Mark) -> (V3, V3) {
    let lift = |id, p: P| {
        let h = ground(f, p);
        let chest = f.w.get(id).map(|u| heights(f, u).0).unwrap_or(ONE);
        to_scene(p, h + chest)
    };
    (lift(m.tgt, m.at), lift(m.src, m.from))
}

fn shape(f: &Frame, b: &mut Batch, k: usize, m: &Mark, s: &Shape, col: Rgb) {
    let dur = ticks(s.dur);
    let t = age(f, m.tick);
    if t >= dur {
        return;
    }
    let u = fx::div(t, dur);
    let fade = ONE - u;
    let size = scene_len(value(f, k, s).max(1));
    let (at, from) = ends(f, m);
    let floor = to_scene(m.at, ground(f, m.at));
    match s.kind.as_str() {
        "ring" => {
            let open = ONE - fx::mul(ONE - u, ONE - u);
            ring(
                b,
                floor,
                fx::mul(size, ONE / 5 + fx::mul(open, ONE * 4 / 5)),
                col,
                fade,
            );
            disc(b, floor, fx::mul(size, open), col, fade / 6);
        }
        "flash" => {
            let r = fx::mul(size, ONE / 3 + fx::mul(u, ONE * 2 / 3));
            glow(b, at, r, col, fade);
        }
        "sky" => {
            let top = at.add(v3(0, ONE * 14, 0));
            rod(b, top, floor, big(f, ONE / 5) + fx::mul(size, ONE / 8), col, fade);
            ring(b, floor, size, col, fade);
        }
        "beam" => {
            rod(b, from, at, big(f, ONE / 6) + fx::mul(size, ONE / 10), col, fade);
            glow(b, at, big(f, ONE / 2) + fx::mul(size, ONE / 6), col, fade);
        }
        "wave" => {
            // a crest of glows running out along the line
            let reach = fx::mul(ONE, u);
            for i in 0..4 {
                let p = from.lerp(at, (reach - i * ONE / 12).clamp(0, ONE));
                glow(b, p, size / 2 - i * size / 12, col, fade);
            }
        }
        "spikes" => {
            // cones thrown up along the line, each in turn
            let n = 7;
            let g0 = to_scene(m.from, ground(f, m.from));
            for i in 0..n {
                let at_i = fx::div(i + 1, n);
                let rise = fx::div((u - at_i / 2).clamp(0, ONE / 3), ONE / 3);
                if rise <= 0 {
                    continue;
                }
                let p = g0.lerp(floor, at_i);
                let h = fx::mul(size, rise);
                let a =
                    Affine::translate(p.add(v3(0, h / 2, 0))).then(&Affine::scale(v3(size / 2, h, size / 2)));
                b.push(crate::render::LAYER_INKED, Mesh::Cone, &a, col, 0);
            }
        }
        "rise" => {
            for i in 0..5 {
                let a = fx::TURN * i / 5 + fx::mul(u, fx::TURN / 3);
                let r = size / 2;
                let p = floor.add(v3(
                    fx::mul(fx::cos(a), r),
                    fx::mul(u, size * 2) + ONE / 3,
                    fx::mul(fx::sin(a), r),
                ));
                glow(b, p, big(f, ONE / 5), col, fade);
            }
        }
        "spin" => {
            let a = fx::mul(u, fx::TURN * 2);
            for i in 0..3 {
                let ang = a + fx::TURN * i / 3;
                let p = from.add(v3(fx::mul(fx::cos(ang), size), 0, fx::mul(fx::sin(ang), size)));
                glow(b, p, big(f, ONE / 4), col, fade);
            }
            ring(b, to_scene(m.from, ground(f, m.from)), size, col, fade);
        }
        _ => {}
    }
}

/// Every mark still showing.
pub fn draw_marks(f: &Frame, b: &mut Batch) {
    for m in &f.w.marks {
        let k = m.ability as usize;
        let Some(l) = look(f, k) else { continue };
        if !seen(f, m.at) && !seen(f, m.from) {
            continue;
        }
        let col = color(f, l);
        match m.kind {
            MK_CAST if l.cast => {
                // the cast glow grows in the hands until the spell goes
                let Some(u) = f.w.get(m.src) else { continue };
                if !matches!(u.act, Act::Cast { .. }) {
                    continue;
                }
                let t = age(f, m.tick).min(ONE * 12);
                let (chest, _) = heights(f, u);
                let ahead = v3(
                    fx::mul(fx::cos(u.facing), ONE),
                    0,
                    -fx::mul(fx::sin(u.facing), ONE),
                );
                let p = to_scene(u.pos(), ground(f, u.pos()) + chest + chest / 3).add(ahead);
                glow(b, p, big(f, ONE / 6 + t / 40), col, ONE);
            }
            MK_FIRE => l.fire.iter().for_each(|s| shape(f, b, k, m, s, col)),
            MK_LAND => l.land.iter().for_each(|s| shape(f, b, k, m, s, col)),
            MK_PROC => l.procs.iter().for_each(|s| shape(f, b, k, m, s, col)),
            _ => {}
        }
    }
}

fn seen(f: &Frame, p: P) -> bool {
    f.team == crate::world::NEUTRAL
        || f.ctx
            .terrain
            .tree_index(p[0], p[1])
            .map(|i| f.w.vision.get(i).map(|v| v & (1 << f.team) != 0).unwrap_or(true))
            .unwrap_or(true)
}

/// How a missile looks (T6): its shape, its palette colour and its size.
/// Attacks are looked up by the shooter's unit id (the longest key it
/// starts with), abilities by the `missile` of their look.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    /// `orb` (a glowing ball), `bolt` (a big ball with a streak, for
    /// towers), `arrow`, `streak` (a long thin bullet), `shard` (an ice
    /// crystal), `flame` (a fireball with a smoky tail), `boulder` (a stone
    /// lobbed in an arc), `spark` (crackling lightning), `hammer` (a
    /// spinning hammer), `hook` (a hook on its chain).
    pub shape: String,
    /// A palette role; `team_glow` and the other team roles take the
    /// shooter's side. Empty for an ability: its look's colour.
    #[serde(default)]
    pub color: String,
    /// The ball's radius, hundredths of a scene unit, for a model of scale one.
    pub size: i32,
}

pub type Shots = BTreeMap<String, Shot>;

/// The attack look for a shooter: the longest key its unit id starts with.
pub fn shot_for<'a>(shots: &'a Shots, unit_id: &str) -> Option<&'a Shot> {
    shots
        .iter()
        .filter(|(k, _)| unit_id.starts_with(k.as_str()))
        .max_by_key(|(k, _)| k.len())
        .map(|(_, s)| s)
}

/// A frame for a part lying along `axis` (a unit vector): `len` along it
/// and `w` across, centred on `at`.
pub(crate) fn along(at: V3, axis: V3, len: i32, w: i32) -> Affine {
    let helper = if axis.y.abs() > ONE - ONE / 16 {
        v3(ONE, 0, 0)
    } else {
        V3::UP
    };
    let x = helper.cross(axis).norm();
    let z = axis.cross(x);
    Affine {
        c0: x.scale(w),
        c1: axis.scale(len),
        c2: z.scale(w),
        t: at,
    }
}

/// Where a missile is going, in sixteenths, and how far it still has to go.
fn heading(f: &Frame, ms: &crate::world::Missile) -> (i64, i64, i64) {
    let goal = if ms.t != NO_UNIT {
        f.w.get(ms.t).map(|u| [u.x, u.y]).unwrap_or(ms.to)
    } else {
        ms.to
    };
    let (dx, dy) = ((goal[0] - ms.x) as i64, (goal[1] - ms.y) as i64);
    (dx, dy, fx::isqrt((dx * dx + dy * dy) as u64) as i64)
}

/// Missiles (T6). Each kind of shot has its own shape, colour and size, so
/// a tower's bolt, a ranged creep's spit, a hero's arrow and a spell read
/// apart at a glance. A missile moves on between ticks with the frame, and
/// leaves a short fading trail behind it.
pub fn draw_missile(f: &Frame, b: &mut Batch, at: V3, ms: &crate::world::Missile) {
    let (shot, col) = match ms.what {
        Hit::Ability { ability, .. } => {
            let k = ability as usize;
            let Some(l) = look(f, k) else { return };
            let fallback = Shot {
                shape: "orb".into(),
                color: String::new(),
                size: 50,
            };
            let s = l.missile.clone().unwrap_or(fallback);
            let col = if s.color.is_empty() {
                color(f, l)
            } else {
                f.models.named(&s.color)
            };
            (s, col)
        }
        _ => {
            let id =
                f.w.get(ms.src)
                    .map(|u| f.ctx.rules.units[u.def as usize].id.as_str());
            let s = id
                .and_then(|id| shot_for(&f.look.shots, id))
                .cloned()
                .unwrap_or(Shot {
                    shape: "orb".into(),
                    color: "shot".into(),
                    size: f.look.missile,
                });
            let col = if s.color.starts_with("team") {
                f.models.team_named(ms.team, &s.color)
            } else {
                f.models.named(&s.color)
            };
            (s, col)
        }
    };
    let (dx, dy, left) = heading(f, ms);
    // the way it flies, in the scene (map y is the scene's −z)
    let dir = if left > 0 {
        v3(
            (dx * ONE as i64 / left) as i32,
            0,
            -((dy * ONE as i64 / left) as i32),
        )
    } else {
        v3(ONE, 0, 0)
    };
    // on between ticks, never past where it lands
    let ahead = (fx::mul(ms.speed, f.alpha) as i64).min(left) as i32;
    let at = at.add(dir.scale(scene_len(ahead) / crate::rules::SUB));
    let gone = (ms.age as i32 * ms.speed + ahead) / crate::rules::SUB;
    let r = big(f, shot.size * ONE / 100);
    // how far back the trail may reach: no further than it has flown
    let tail = scene_len(gone).min(r * 8);
    let trail = |b: &mut Batch, from: V3, n: i32, gap: i32, col: Rgb| {
        for i in 1..=n {
            let back = gap * i;
            if back > tail {
                break;
            }
            let s = r - r * i / (n + 1);
            let p = from.sub(dir.scale(back));
            b.tint(
                Mesh::Sphere,
                &Affine::translate(p).then(&Affine::scale(v3(s * 2, s * 2, s * 2))),
                col,
                ONE / 2 - ONE * i / (2 * (n + 1)),
            );
        }
    };
    let spin = (f.w.tick as i32 * ONE + f.alpha) / ONE * (fx::TURN / 9);
    match shot.shape.as_str() {
        "bolt" => {
            glow(b, at, r, col, ONE);
            rod(b, at, at.sub(dir.scale((r * 5).min(tail))), r / 3, col, ONE);
            trail(b, at, 5, r, col);
        }
        "arrow" => {
            let shaft = along(at.sub(dir.scale(r * 2)), dir, r * 4, r / 4);
            b.push(
                crate::render::LAYER_INKED,
                Mesh::Cyl,
                &shaft,
                f.models.named("wood"),
                0,
            );
            let tip = along(at.add(dir.scale(r / 2)), dir, r, r * 2 / 3);
            b.push(
                crate::render::LAYER_INKED,
                Mesh::Cone,
                &tip,
                f.models.named("steel_bright"),
                0,
            );
            let fletch = along(at.sub(dir.scale(r * 4)), dir, r, r * 3 / 4);
            b.push(crate::render::LAYER_PLAIN, Mesh::Cone, &fletch, col, STYLE_GLOW);
            trail(b, at.sub(dir.scale(r * 4)), 4, r, col);
        }
        "streak" => {
            rod(b, at, at.sub(dir.scale((r * 10).min(tail))), r / 4, col, ONE);
            glow(b, at, r / 2, col, ONE);
        }
        "shard" => {
            let a = along(at, dir, r * 3, r).then(&Affine::rot_y(spin));
            b.push(LAYER_PLAIN, Mesh::Octa, &a, col, STYLE_GLOW);
            let halo = Affine::translate(at).then(&Affine::scale(v3(r * 3, r * 3, r * 3)));
            b.tint(Mesh::Sphere, &halo, col, ONE / 4);
            trail(b, at, 4, r, f.models.named("ice"));
        }
        "flame" => {
            glow(b, at, r, f.models.named("ember"), ONE);
            let flick = fx::sin(spin * 3) / 6;
            glow(b, at.sub(dir.scale(r)), r * 3 / 4 + fx::mul(r, flick), col, ONE);
            trail(b, at, 6, r * 2 / 3, col);
        }
        "boulder" => {
            // lobbed: highest halfway along its flight
            let total = (gone as i64 + left / crate::rules::SUB as i64).max(1);
            let u = ((gone as i64 * ONE as i64) / total) as i32;
            let lift = fx::mul(fx::mul(u, ONE - u), ONE * 10);
            let p = at.add(v3(0, lift, 0));
            let rock = Affine::translate(p)
                .then(&Affine::rot_x(spin))
                .then(&Affine::scale(v3(r * 2, r * 2, r * 2)));
            b.push(crate::render::LAYER_INKED, Mesh::Sphere, &rock, col, 0);
            trail(b, p, 3, r, f.models.named("cloud"));
        }
        "spark" => {
            glow(b, at, r, col, ONE);
            for i in 0..3 {
                let a = spin * 2 + fx::TURN * i / 3;
                let out = v3(
                    fx::mul(fx::cos(a), r * 2),
                    fx::mul(fx::sin(a * 2), r),
                    fx::mul(fx::sin(a), r * 2),
                );
                rod(b, at, at.add(out), r / 6, col, ONE);
            }
            trail(b, at, 4, r, col);
        }
        "hammer" => {
            let turn = Affine::translate(at)
                .then(&Affine::rot_y(fx::atan2(dir.x, dir.z)))
                .then(&Affine::rot_x(spin * 2));
            let head = turn.then(&Affine::scale(v3(r * 2, r * 2, r * 3)));
            b.push(
                crate::render::LAYER_INKED,
                Mesh::Cube,
                &head,
                f.models.named("steel_bright"),
                0,
            );
            let handle = turn
                .then(&Affine::translate(v3(0, -r * 2, 0)))
                .then(&Affine::scale(v3(r / 2, r * 3, r / 2)));
            b.push(
                crate::render::LAYER_INKED,
                Mesh::Cyl,
                &handle,
                f.models.named("wood"),
                0,
            );
            let halo = Affine::translate(at).then(&Affine::scale(v3(r * 4, r * 4, r * 4)));
            b.tint(Mesh::Sphere, &halo, col, ONE / 3);
            trail(b, at, 5, r, col);
        }
        "hook" => {
            let tip = along(at, dir, r * 2, r);
            b.push(
                crate::render::LAYER_INKED,
                Mesh::Cone,
                &tip,
                f.models.named("steel_bright"),
                0,
            );
            if let Some(u) = f.w.get(ms.src) {
                let (chest, _) = heights(f, u);
                let from = to_scene(u.pos(), ground(f, u.pos()) + chest);
                for i in 1..12 {
                    let p = from.lerp(at, fx::div(i, 12));
                    let c = r * 2 / 5;
                    let a = Affine::translate(p).then(&Affine::scale(v3(c, c, c)));
                    b.push(
                        crate::render::LAYER_INKED,
                        Mesh::Sphere,
                        &a,
                        f.models.named("iron"),
                        0,
                    );
                }
            }
            glow(b, at, r / 2, col, ONE);
        }
        _ => {
            glow(b, at, r, col, ONE);
            trail(b, at, 4, r, col);
        }
    }
}

/// A zone in its ability's way; returns false to leave it to the plain disc.
pub fn draw_zone(f: &Frame, b: &mut Batch, z: &crate::world::Zone) -> bool {
    let k = z.ability as usize;
    let Some(l) = look(f, k) else { return false };
    let col = color(f, l);
    let floor = to_scene(z.at, ground(f, z.at));
    let r = scene_len(z.radius.max(60));
    let t = age(f, z.start);
    let beat = fx::sin(fx::mul(t, fx::TURN / 12));
    match l.zone.as_str() {
        "warn" => {
            // a ring that pulses until the blast lands
            if f.w.tick < z.next || z.next == u32::MAX && f.w.tick < z.until {
                ring(b, floor, r, col, ONE * 3 / 5 + beat / 5);
                disc(b, floor, r, col, ONE / 6);
            }
        }
        "cloud" => {
            for i in 0..8 {
                let a = fx::TURN * i / 8 + fx::mul(t, fx::TURN / 160);
                let rr = fx::mul(r, ONE * 3 / 4);
                let p = floor.add(v3(
                    fx::mul(fx::cos(a), rr),
                    ONE / 2 + beat / 6,
                    fx::mul(fx::sin(a), rr),
                ));
                let s = big(f, ONE + ONE / 2);
                b.tint(
                    Mesh::Sphere,
                    &Affine::translate(p).then(&Affine::scale(v3(s, s, s))),
                    col,
                    ONE / 3,
                );
            }
            disc(b, floor, r, col, ONE / 6);
        }
        "rain" => {
            disc(b, floor, r, col, ONE / 5);
            ring(b, floor, r, col, ONE / 2);
            for i in 0..6 {
                let ph = (t / 4 + i * ONE * 7 / 3) % (ONE * 4);
                let fall = fx::div(ph, ONE * 4);
                let a = fx::TURN * i * 3 / 7;
                let rr = fx::mul(r, ONE * (i % 3 + 1) / 4);
                let p = floor.add(v3(
                    fx::mul(fx::cos(a), rr),
                    fx::mul(ONE - fall, ONE * 6),
                    fx::mul(fx::sin(a), rr),
                ));
                glow(b, p, big(f, ONE / 6), col, ONE);
            }
        }
        "spin" => {
            let a = fx::mul(t, fx::TURN / 10);
            for i in 0..4 {
                let ang = a + fx::TURN * i / 4;
                let p = floor.add(v3(
                    fx::mul(fx::cos(ang), r),
                    big(f, ONE),
                    fx::mul(fx::sin(ang), r),
                ));
                glow(b, p, big(f, ONE / 4), col, ONE);
            }
            ring(b, floor, r, col, ONE / 2);
        }
        "heal" => {
            ring(b, floor, r, col, ONE / 2 + beat / 6);
            disc(b, floor, r, col, ONE / 8);
        }
        "flash" => {
            let life = (z.until.saturating_sub(z.start)).max(1) as i32 * ONE;
            let left = ONE - fx::div(t.min(life), life);
            disc(b, floor, r, col, left / 2);
        }
        "none" => {}
        _ => return false,
    }
    true
}

/// The marks on one unit: what holds it, what helps it, and its channel.
pub fn draw_status(f: &Frame, b: &mut Batch, u: &Unit, foot: V3) {
    let d = &f.ctx.rules.units[u.def as usize];
    if d.kind.is_building() || d.kind == Kind::Ward {
        return;
    }
    let (chest, top) = heights(f, u);
    let now = (f.w.tick as i32) * ONE + f.alpha;
    let head = foot.add(v3(0, top + big(f, ONE / 3), 0));
    let one = big(f, ONE);
    if u.has(M::Stun) {
        // little stars circling the head
        for i in 0..3 {
            let a = fx::mul(now, fx::TURN / 18) + fx::TURN * i / 3;
            let p = head.add(v3(fx::mul(fx::cos(a), one), 0, fx::mul(fx::sin(a), one)));
            let s = one / 2;
            let star = Affine::translate(p)
                .then(&Affine::rot_y(a))
                .then(&Affine::scale(v3(s, s, s / 3)));
            b.push(
                LAYER_PLAIN,
                Mesh::Octa,
                &star,
                f.models.named("fx_holy"),
                STYLE_GLOW,
            );
        }
    }
    if u.has(M::Silence) {
        // a bubble over the head, crossed through
        glow(
            b,
            head.add(v3(0, one / 2, 0)),
            one / 2,
            f.models.named("fx_hex"),
            ONE / 2,
        );
        rod(
            b,
            head.add(v3(-one * 3 / 4, one, 0)),
            head.add(v3(one * 3 / 4, 0, 0)),
            one / 10,
            f.models.named("fx_shadow"),
            ONE,
        );
    }
    let slowed = u.mods.iter().any(|m| m.kind == M::MovePct && m.a < 0);
    if slowed && !u.has(M::Stun) {
        ring(b, foot, one * 3 / 2, f.models.named("fx_frost"), ONE * 3 / 4);
    }
    if u.has(M::Root) {
        // frozen in place: an ice block round the unit
        let a = Affine::translate(foot.add(v3(0, chest, 0))).then(&Affine::scale(v3(one * 3, top, one * 3)));
        b.tint(Mesh::Cyl, &a, f.models.named("fx_frost"), ONE * 2 / 5);
    }
    // a ring at the feet for each ability that helps this unit
    let mut drawn: Vec<u16> = Vec::new();
    for m in &u.mods {
        let helps = matches!(
            m.kind,
            M::Armor | M::DamagePct | M::Damage | M::AttackPct | M::MovePct | M::ManaRegen
        ) && m.a > 0
            || m.kind == M::Regen && m.a > 0;
        if m.tag == 0 || !helps || drawn.contains(&m.tag) || m.tag as usize > f.ctx.rules.abilities.len() {
            continue;
        }
        drawn.push(m.tag);
        let k = m.tag as usize - 1;
        let Some(l) = look(f, k) else { continue };
        let col = color(f, l);
        let beat = fx::sin(fx::mul(now, fx::TURN / 30));
        ring(b, foot, one * 2 + fx::mul(beat, one / 4), col, ONE / 2 + beat / 8);
        let rise = (now / 30) % ONE;
        glow(
            b,
            foot.add(v3(one, fx::mul(rise, top), 0)),
            one / 8,
            col,
            ONE - rise,
        );
    }
    // a damage over time from an ability: small puffs in its colour
    for m in &u.mods {
        if m.kind == M::Regen && m.a < 0 && m.tag != 0 && (m.tag as usize) <= f.ctx.rules.abilities.len() {
            if let Some(l) = look(f, m.tag as usize - 1) {
                let p = (now / 20) % ONE;
                glow(
                    b,
                    foot.add(v3(-one, fx::mul(p, chest * 2), one / 2)),
                    one / 6,
                    color(f, l),
                    ONE - p,
                );
            }
            break;
        }
    }
    // spinning blades, or a body nothing can touch
    if (u.has(M::SpellImmune) || u.has(M::Invulnerable)) && u.mods.iter().any(|m| m.tag != 0) {
        let tagged = u
            .mods
            .iter()
            .find(|m| matches!(m.kind, M::SpellImmune | M::Invulnerable) && m.tag != 0)
            .map(|m| m.tag);
        if let Some(t) = tagged.filter(|t| (*t as usize) <= f.ctx.rules.abilities.len()) {
            if let Some(l) = look(f, t as usize - 1) {
                let a = fx::mul(now, fx::TURN / 8);
                for i in 0..3 {
                    let ang = a + fx::TURN * i / 3;
                    let p = foot.add(v3(
                        fx::mul(fx::cos(ang), one * 3),
                        chest,
                        fx::mul(fx::sin(ang), one * 3),
                    ));
                    glow(b, p, one / 4, color(f, l), ONE);
                }
            }
        }
    }
    // a channel: a tether to its target, or a ring round the caster
    if let Act::Channel { slot, t, .. } = u.act {
        if let Some((k, _)) = crate::ability::slot_info(f.ctx, f.w, u.id, slot as usize) {
            if let Some(l) = look(f, k) {
                let col = color(f, l);
                match f.w.get(t) {
                    Some(v) if t != NO_UNIT && t != u.id => {
                        let vp = v.pos();
                        let vc = to_scene(vp, ground(f, vp) + heights(f, v).0);
                        rod(b, foot.add(v3(0, chest, 0)), vc, one / 6, col, ONE * 3 / 4);
                    }
                    _ => ring(b, foot, one * 4, col, ONE / 2),
                }
            }
        }
    }
}

/// A unit's feet in the scene as drawn (sixteenths in, smoothed).
pub fn foot(f: &Frame, x: i32, y: i32) -> V3 {
    let p = [x / SUB, y / SUB];
    crate::scene::sub_to_scene(x, y, ground(f, p))
}
