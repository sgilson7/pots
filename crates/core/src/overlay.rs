//! What a player's own hand shows on the ground (round 3): the outline of
//! an ability while it is aimed (T9), a marker where a right-click sent the
//! hero (T10), and a teleport's glow at both ends while it channels (T11).
//!
//! The aim and the marker are the local player's alone: `Game` builds an
//! `Overlay` each frame from its aim, pointer and last order click, and
//! nothing here reaches the world, so online play never sees them. The
//! teleport is drawn from the world (the caster's cast and the rule's own
//! destination), so every player who can see it sees it.

use crate::fx::{self, ONE};
use crate::geom::{v3, Affine, V3};
use crate::marks::{big, disc, glow, ground, heights, ring, rod, scene_len};
use crate::mesh::Mesh;
use crate::render::{Batch, Rgb};
use crate::rules::{AbilityDef, Target, P, SUB};
use crate::scene::{drawn_at, shown, to_scene, Frame};
use crate::world::{Act, Uid};

/// The shape an aimed ability will cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    /// A spot: a small ring where it goes.
    Point,
    /// A blast of this radius at the pointer.
    Circle(i32),
    /// A line out from the hero toward the pointer: its length, and its
    /// width where it starts and where it ends.
    Line { len: i32, width: i32, end: i32 },
    /// A unit: the one under the pointer is ringed when it may be chosen.
    Unit,
    /// A teleport: the building it would land beside.
    Teleport,
}

/// An ability or item being aimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aiming {
    pub hero: Uid,
    /// Cast range, map units; none drawn at 0 or past the map.
    pub range: i32,
    /// The ground under the pointer.
    pub at: P,
    /// The unit under the pointer, when it is one the ability may target.
    pub under: Uid,
    pub area: Area,
    pub color: Rgb,
}

/// The last order click: where, on whom, and how far through its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ping {
    pub at: P,
    pub unit: Uid,
    pub attack: bool,
    /// Q16 share of its life gone.
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Overlay {
    pub aim: Option<Aiming>,
    pub ping: Option<Ping>,
}

/// The area an ability covers at `level`, from its own numbers: a line
/// when it has a width, a circle when it has a radius.
pub fn ability_area(a: &AbilityDef, level: i32) -> Area {
    let level = level.max(1);
    match a.target {
        Target::Unit => Area::Unit,
        Target::Point | Target::Either => {
            let width = a.v("width", level);
            let radius = a.v("radius", level);
            if width > 0 {
                let len = a.v("length", level);
                let end = a.v("end_width", level);
                Area::Line {
                    len: if len > 0 { len } else { a.range_at(level) },
                    width,
                    end: if end > 0 { end } else { width },
                }
            } else if radius > 0 {
                Area::Circle(radius)
            } else {
                Area::Point
            }
        }
        _ => Area::Point,
    }
}

/// The share of a cycle `period` ticks long this frame is through, Q16.
fn cycle(f: &Frame, period: i32) -> i32 {
    let t = (f.w.tick as i64 * ONE as i64 + f.alpha as i64) % (period as i64 * ONE as i64);
    (t / period as i64) as i32
}

/// A flat strip on the ground from `a` along `dir` (a level unit vector).
fn strip(b: &mut Batch, a: V3, dir: V3, len: i32, w: i32, col: Rgb, alpha: i32) {
    if len <= 0 || w <= 0 {
        return;
    }
    let across = v3(dir.z, 0, -dir.x);
    let m = Affine {
        c0: across.scale(w),
        c1: v3(0, ONE / 40, 0),
        c2: dir.scale(len),
        t: a.add(dir.scale(len / 2)).add(v3(0, ONE / 25, 0)),
    };
    b.tint(Mesh::Cube, &m, col, alpha);
}

/// A cone pointing down at `at`, for the move marker's chevrons.
fn dart(b: &mut Batch, at: V3, r: i32, col: Rgb) {
    let a = Affine::translate(at)
        .then(&Affine::rot_x(fx::HALF))
        .then(&Affine::scale(v3(r, r * 2, r)));
    b.push(
        crate::render::LAYER_PLAIN,
        Mesh::Cone,
        &a,
        col,
        crate::scene::STYLE_GLOW,
    );
}

pub fn draw(f: &Frame, b: &mut Batch, o: &Overlay) {
    if let Some(a) = &o.aim {
        aim(f, b, a);
    }
    if let Some(p) = &o.ping {
        ping(f, b, p);
    }
}

fn floor_at(f: &Frame, p: P) -> V3 {
    to_scene(p, ground(f, p))
}

fn aim(f: &Frame, b: &mut Batch, a: &Aiming) {
    let Some(u) = f.w.get(a.hero) else { return };
    let (x, y, face) = drawn_at(f, u);
    let here = [x / SUB, y / SUB];
    let feet = floor_at(f, here);
    let white = f.models.named("select");
    let beat = fx::sin(fx::mul(cycle(f, 30), fx::TURN)) / 10;
    if a.range > 0 && a.range < f.ctx.rules.map.size {
        ring(b, feet, scene_len(a.range), white, ONE / 3);
    }
    let cur = floor_at(f, a.at);
    match a.area {
        Area::Point => {
            ring(b, cur, scene_len(70), a.color, ONE * 3 / 4 + beat);
            disc(b, cur, scene_len(70), a.color, ONE / 4);
        }
        Area::Circle(r) => {
            ring(b, cur, scene_len(r), a.color, ONE * 3 / 4 + beat);
            disc(b, cur, scene_len(r), a.color, ONE / 5);
            disc(b, cur, scene_len(24), a.color, ONE / 2);
        }
        Area::Line { len, width, end } => {
            let (dx, dy) = ((a.at[0] - here[0]) as i64, (a.at[1] - here[1]) as i64);
            let d = fx::isqrt((dx * dx + dy * dy) as u64) as i64;
            let dir = if d > 0 {
                v3((dx * ONE as i64 / d) as i32, 0, -((dy * ONE as i64 / d) as i32))
            } else {
                v3(fx::cos(face), 0, -fx::sin(face))
            };
            let n = 6;
            let seg = scene_len(len) / n;
            for i in 0..n {
                let w = width + (end - width) * (2 * i + 1) / (2 * n);
                strip(
                    b,
                    feet.add(dir.scale(seg * i)),
                    dir,
                    seg,
                    scene_len(w),
                    a.color,
                    ONE / 4,
                );
            }
            // bright edges, and a cap where it stops
            for side in [-1, 1] {
                for i in 0..n {
                    let w = width + (end - width) * (2 * i + 1) / (2 * n);
                    let across = v3(dir.z, 0, -dir.x).scale(scene_len(w) / 2 * side);
                    strip(
                        b,
                        feet.add(dir.scale(seg * i)).add(across),
                        dir,
                        seg,
                        big(f, ONE / 20),
                        a.color,
                        ONE * 3 / 4,
                    );
                }
            }
            let tip = feet.add(dir.scale(seg * n));
            strip(
                b,
                tip.sub(dir.scale(big(f, ONE / 20))),
                v3(dir.z, 0, -dir.x),
                scene_len(end),
                big(f, ONE / 10),
                a.color,
                ONE,
            );
            strip(
                b,
                tip.sub(dir.scale(big(f, ONE / 20)))
                    .add(v3(-dir.z, 0, dir.x).scale(scene_len(end) / 2)),
                v3(dir.z, 0, -dir.x),
                scene_len(end),
                big(f, ONE / 10),
                a.color,
                ONE,
            );
        }
        Area::Unit => {
            if let Some(t) = f.w.live(a.under) {
                let (tx, ty, _) = drawn_at(f, t);
                let d = &f.ctx.rules.units[t.def as usize];
                let r = scene_len(d.radius + 50);
                let p = floor_at(f, [tx / SUB, ty / SUB]);
                ring(b, p, r, a.color, ONE);
                ring(b, p, fx::mul(r, ONE * 13 / 10 + beat), a.color, ONE / 2);
            }
        }
        Area::Teleport => {
            let dest = crate::item::tp_destination(f.w, f.ctx, u.team, a.at);
            let p = floor_at(f, dest);
            ring(b, p, scene_len(260), a.color, ONE * 3 / 4 + beat);
            disc(b, p, scene_len(260), a.color, ONE / 5);
            ring(b, cur, scene_len(50), a.color, ONE / 2);
        }
    }
}

fn ping(f: &Frame, b: &mut Batch, p: &Ping) {
    let fade = ONE - p.age;
    if fade <= 0 {
        return;
    }
    if p.attack {
        // a red ring closing on the unit attacked
        let (spot, r) = match f.w.live(p.unit) {
            Some(t) => {
                let (tx, ty, _) = drawn_at(f, t);
                let d = &f.ctx.rules.units[t.def as usize];
                ([tx / SUB, ty / SUB], d.radius + d.footprint + 50)
            }
            None => (p.at, 60),
        };
        let at = floor_at(f, spot);
        let close = ONE * 2 - fx::mul(p.age, ONE).min(ONE);
        let col = f.models.named("bar_enemy");
        ring(b, at, fx::mul(scene_len(r), close), col, fade);
        ring(b, at, scene_len(r), col, fade / 2);
        return;
    }
    // a ring closing on the spot, and four darts dropping onto it
    let at = floor_at(f, p.at);
    let col = f.models.named("heal");
    let open = ONE - fx::mul(fx::mul(fade, fade), ONE);
    ring(b, at, scene_len(130) - fx::mul(scene_len(90), open), col, fade);
    ring(b, at, scene_len(40), col, fade);
    disc(b, at, scene_len(40), col, fade / 2);
    let drop = fx::mul(fade, fade);
    for i in 0..4 {
        let ang = fx::TURN * i / 4 + fx::TURN / 8;
        let out = fx::mul(scene_len(100), drop) + scene_len(24);
        let spot = at.add(v3(
            fx::mul(fx::cos(ang), out),
            big(f, ONE / 5) + fx::mul(drop, big(f, ONE)),
            fx::mul(fx::sin(ang), out),
        ));
        dart(b, spot, big(f, ONE / 6), col);
    }
}

/// Every teleport being channelled: a column of light rising round the
/// caster, and a ring and beam where it will land, both growing as the
/// channel runs (T11).
pub fn teleports(f: &Frame, b: &mut Batch) {
    for u in &f.w.units {
        let Act::Cast {
            slot,
            at,
            fire_at,
            item: true,
            ..
        } = u.act
        else {
            continue;
        };
        if !u.alive {
            continue;
        }
        let Some(it) = u
            .items
            .get(slot as usize)
            .filter(|s| s.item > 0)
            .map(|s| &f.ctx.rules.items[s.item as usize - 1])
        else {
            continue;
        };
        if it.logic != "tp" {
            continue;
        }
        let total = (fire_at.saturating_sub(u.anim_at) as i32).max(1) * ONE;
        let gone = crate::marks::age(f, u.anim_at).min(total);
        let grow = fx::div(gone, total);
        let col = f.models.team_named(u.team, "team_glow");
        let swirl = cycle(f, 45);
        if shown(f, u) {
            let (x, y, _) = drawn_at(f, u);
            let feet = floor_at(f, [x / SUB, y / SUB]);
            let (_, top) = heights(f, u);
            let r = big(f, ONE * 4 / 5);
            ring(b, feet, r + fx::mul(r, grow) / 2, col, ONE);
            ring(
                b,
                feet,
                fx::mul(r, ONE / 2 + fx::mul(swirl, ONE)),
                col,
                ONE - swirl,
            );
            disc(b, feet, r, col, ONE / 4 + grow / 4);
            let high = top / 2 + fx::mul(top * 2, grow);
            let column =
                Affine::translate(feet.add(v3(0, high / 2, 0))).then(&Affine::scale(v3(r * 2, high, r * 2)));
            b.tint(Mesh::Cyl, &column, col, ONE / 4 + grow / 4);
            let core = Affine::translate(feet.add(v3(0, high / 2, 0))).then(&Affine::scale(v3(r, high, r)));
            b.tint(Mesh::Cyl, &core, col, ONE / 3 + grow / 3);
            for i in 0..6 {
                let ang = fx::mul(swirl, fx::TURN) + fx::TURN * i / 6;
                let rise = (swirl + ONE * i / 6) % ONE;
                let p = feet.add(v3(
                    fx::mul(fx::cos(ang), r),
                    fx::mul(rise, top + top / 2),
                    fx::mul(fx::sin(ang), r),
                ));
                glow(b, p, big(f, ONE / 7), col, ONE - rise);
            }
        }
        let dest = crate::item::tp_destination(f.w, f.ctx, u.team, at);
        let land = [dest[0] - 150, dest[1] - 150];
        // shown to every side, as DotA shows where a teleport lands
        let p = floor_at(f, land);
        let r = scene_len(110);
        ring(b, p, r, col, ONE * 3 / 4);
        ring(b, p, fx::mul(r, ONE - fx::mul(swirl, ONE / 2)), col, ONE / 2);
        disc(b, p, r, col, ONE / 6 + grow / 4);
        rod(
            b,
            p,
            p.add(v3(0, fx::mul(big(f, ONE * 6), grow) + ONE / 4, 0)),
            big(f, ONE / 8),
            col,
            ONE,
        );
    }
}
