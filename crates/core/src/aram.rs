//! All Random All Mid (round 4, T13 and T14): every player gets a random
//! hero, only the middle lane is built, a cannon at each base throws a hero
//! back out to the lane, and health relics wait in the lane. The numbers
//! are `data/rules.json`'s `aram`; this module holds the rules that read
//! them. A normal match never reaches any of it (`World::aram` is false).
//!
//! Points in the data are the Sunward side's; the Moonward's are their
//! mirrors through the map's centre, as the map itself is.

use crate::fx;
use crate::rules::{AramDef, Kind, P, SUB, TPS};
use crate::world::{dist2, within, Act, Ctx, Mod, Uid, World, M, MOON, PLAYERS};

pub fn def<'a>(w: &World, ctx: &'a Ctx) -> Option<&'a AramDef> {
    if w.aram {
        ctx.rules.aram.as_ref()
    } else {
        None
    }
}

/// A Sunward point as it stands on `team`'s side.
pub fn side_point(ctx: &Ctx, team: u8, p: P) -> P {
    if team == MOON {
        let s = ctx.rules.map.size;
        [s - p[0], s - p[1]]
    } else {
        p
    }
}

/// Whether the mode builds a lane's towers, barracks and creeps. Base
/// buildings (no lane) always stand.
pub fn builds_lane(w: &World, ctx: &Ctx, lane: &str) -> bool {
    match def(w, ctx) {
        Some(a) if matches!(lane, "top" | "mid" | "bot") => a.lanes.iter().any(|l| l == lane),
        _ => true,
    }
}

/// The lane codes (`world::lane_code`) the match builds.
pub fn built_lanes(w: &World, ctx: &Ctx) -> Vec<u8> {
    ["top", "mid", "bot"]
        .iter()
        .filter(|l| builds_lane(w, ctx, l))
        .map(|l| crate::world::lane_code(l))
        .collect()
}

/// Where an item is sold in this match: in this mode every shop's goods
/// are at the base, and teleports are not sold (R4.6).
pub fn shop_of<'a>(w: &World, ctx: &'a Ctx, k: usize) -> &'a str {
    let it = &ctx.rules.items[k];
    if def(w, ctx).is_some() {
        if it.logic == "tp" {
            return "none";
        }
        if it.shop == "side" || it.shop == "secret" {
            return "home";
        }
    }
    &it.shop
}

// ------------------------------------------------------------ the draw

/// Each seat's hero in this mode: each side's heroes are dealt from its
/// own shuffle of the roster, so no hero appears twice on one side; a seat
/// that has rerolled takes the next hero from the rest of its side's deck,
/// in seat order. A pure function of the seed and who rerolled, so every
/// browser in a room deals the same.
pub fn deal(seed: u64, heroes: usize, rerolled: &[bool]) -> [u8; PLAYERS] {
    let mut out = [0u8; PLAYERS];
    let n = heroes.max(1);
    for side in 0..2usize {
        let mut rng = crate::rng::Rng::new(seed ^ (0xa2a3_0000 + side as u64));
        let mut deck: Vec<u8> = (0..n as u8).collect();
        for i in (1..n).rev() {
            let j = rng.below(i as u32 + 1) as usize;
            deck.swap(i, j);
        }
        let mut next = 5;
        for j in 0..5 {
            let p = side * 5 + j;
            let mut pick = deck[j % n];
            if rerolled.get(p).copied().unwrap_or(false) && next < n {
                pick = deck[next];
                next += 1;
            }
            out[p] = pick;
        }
    }
    out
}

// ------------------------------------------------------------ the cannon

pub fn cannon_at(ctx: &Ctx, team: u8) -> Option<P> {
    ctx.rules.aram.as_ref().map(|a| side_point(ctx, team, a.cannon))
}

/// The ground a throw can land on: the middle lane from `team`'s base-level
/// tower out to `reach_past` beyond its outer tower, the lane's width across.
/// (start, end, half width).
pub fn band(ctx: &Ctx, team: u8) -> Option<(P, P, i32)> {
    let a = ctx.rules.aram.as_ref()?;
    let side = ctx.rules.map.sides.get(team as usize)?;
    let tower = |tier: i32| {
        side.buildings
            .iter()
            .find(|b| b.lane == "mid" && b.tier == tier)
            .map(|b| b.at)
    };
    let (inner, outer) = (tower(3)?, tower(1)?);
    let (dx, dy) = ((outer[0] - inner[0]) as i64, (outer[1] - inner[1]) as i64);
    let len = fx::isqrt((dx * dx + dy * dy) as u64).max(1) as i64;
    let end = [
        outer[0] + (dx * a.reach_past as i64 / len) as i32,
        outer[1] + (dy * a.reach_past as i64 / len) as i32,
    ];
    Some((inner, end, ctx.rules.map.lane_width / 2))
}

/// The nearest point to `at` inside `team`'s band.
pub fn clamp_landing(ctx: &Ctx, team: u8, at: P) -> P {
    let Some((a, b, half)) = band(ctx, team) else {
        return at;
    };
    let (vx, vy) = ((b[0] - a[0]) as i64, (b[1] - a[1]) as i64);
    let (px, py) = ((at[0] - a[0]) as i64, (at[1] - a[1]) as i64);
    let vv = (vx * vx + vy * vy).max(1);
    let along = px * vx + py * vy;
    // the side-to-side part, measured from the lane's line itself
    let (ox, oy) = (px - vx * along / vv, py - vy * along / vv);
    let t = along.clamp(0, vv);
    let foot = [a[0] as i64 + vx * t / vv, a[1] as i64 + vy * t / vv];
    let off = fx::isqrt((ox * ox + oy * oy) as u64) as i64;
    let k = off.min(half as i64);
    let (ox, oy) = if off > 0 {
        (ox * k / off, oy * k / off)
    } else {
        (0, 0)
    };
    [(foot[0] + ox) as i32, (foot[1] + oy) as i32]
}

/// At the side's base: within the fountain's range of its fountain or of
/// its spawn (a hero spawned at the edge of the ring is 825.4 from the
/// fountain, just outside its 825).
pub fn at_base(ctx: &Ctx, team: u8, p: P) -> bool {
    let Some(side) = ctx.rules.map.sides.get(team as usize) else {
        return false;
    };
    let r = ctx.rules.t.fountain_range;
    within(p, side.fountain, r) || within(p, side.spawn, r)
}

/// Whether `hero` may be thrown now: the mode is on, the hero is alive,
/// free to act, at its base, not already in the air, and its cannon loaded.
pub fn may_throw(w: &World, ctx: &Ctx, hero: Uid) -> bool {
    if def(w, ctx).is_none() {
        return false;
    }
    let Some(u) = w.live(hero) else { return false };
    !u.disabled()
        && !matches!(u.act, Act::Flight { .. })
        && u.hero.as_ref().map(|h| h.cannon).unwrap_or(false)
        && at_base(ctx, u.team, u.pos())
}

/// Puts `hero` in its side's cannon, aimed at `at` (held inside the band).
pub fn throw(w: &mut World, ctx: &Ctx, hero: Uid, at: P) {
    if !may_throw(w, ctx, hero) {
        return;
    }
    let a = def(w, ctx).expect("checked");
    let (climb, flight) = (
        (a.climb * TPS / 100).max(1) as u32,
        (a.flight * TPS / 100).max(1) as u32,
    );
    let team = w.get(hero).expect("checked").team;
    let from = cannon_at(ctx, team).expect("the mode has a cannon");
    let to = clamp_landing(ctx, team, at);
    let tick = w.tick;
    let land = tick + climb + flight;
    let u = w.get_mut(hero).expect("checked");
    u.act = Act::Flight {
        from,
        to,
        start: tick,
        up: tick + climb,
        land,
    };
    u.order = crate::world::Order::Idle;
    u.path.clear();
    u.target = crate::world::NO_UNIT;
    u.x = from[0] * SUB;
    u.y = from[1] * SUB;
    u.anim = crate::world::ANIM_CAST;
    u.anim_at = tick;
    if let Some(h) = u.hero.as_mut() {
        h.cannon = false;
    }
    // in the barrel and in the air the hero cannot be hit or chosen
    w.add_mod(
        hero,
        Mod {
            kind: M::Invulnerable,
            src: hero,
            until: land,
            a: 0,
            b: 0,
            tag: TAG_FLIGHT,
        },
    );
}

/// The modifier tag a throw's invulnerability carries.
pub const TAG_FLIGHT: u16 = 9401;

/// One tick of a hero in the air; returns true while it is still flying,
/// so the caller does nothing else with it.
pub fn fly(w: &mut World, id: Uid) -> bool {
    let now = w.tick;
    let Some(u) = w.get_mut(id) else { return false };
    let Act::Flight {
        from, to, up, land, ..
    } = u.act
    else {
        return false;
    };
    if now >= land {
        u.x = to[0] * SUB;
        u.y = to[1] * SUB;
        u.act = Act::None;
        u.anim = crate::world::ANIM_STAND;
        u.anim_at = now;
        return false;
    }
    if now >= up {
        let span = (land - up).max(1) as i64;
        let k = (now - up) as i64;
        u.x = ((from[0] as i64 * SUB as i64) + ((to[0] - from[0]) as i64 * SUB as i64) * k / span) as i32;
        u.y = ((from[1] as i64 * SUB as i64) + ((to[1] - from[1]) as i64 * SUB as i64) * k / span) as i32;
    }
    true
}

// ------------------------------------------------------------ the relics

/// Every relic's spot: the Sunward ones, then their Moonward mirrors.
pub fn relic_spots(ctx: &Ctx) -> Vec<P> {
    let Some(a) = ctx.rules.aram.as_ref() else {
        return Vec::new();
    };
    let mut v: Vec<P> = a.relics.clone();
    v.extend(a.relics.iter().map(|&p| side_point(ctx, MOON, p)));
    v
}

/// Once a tick in this mode: cannons load at the base, and a hero who walks
/// over a waiting relic takes it.
pub fn clock(w: &mut World, ctx: &Ctx) {
    let Some(a) = def(w, ctx) else { return };
    let now = w.tick;
    for u in w.units.iter_mut() {
        if !u.alive || u.hero.is_none() || u.team > MOON || matches!(u.act, Act::Flight { .. }) {
            continue;
        }
        let home = at_base(ctx, u.team, u.pos());
        if let Some(h) = u.hero.as_mut() {
            if home {
                h.cannon = true;
            }
        }
    }
    let spots = relic_spots(ctx);
    for (i, spot) in spots.iter().enumerate() {
        if w.relics[i] != 0 {
            if now >= w.relics[i] {
                w.relics[i] = 0;
            }
            continue;
        }
        let taker = w
            .units
            .iter()
            .filter(|u| {
                u.alive
                    && u.hero.is_some()
                    && !u.has(M::Illusion)
                    && !matches!(u.act, Act::Flight { .. })
                    && ctx.rules.units[u.def as usize].kind == Kind::Hero
                    && within(u.pos(), *spot, a.relic_radius)
            })
            .min_by_key(|u| (dist2(u.pos(), *spot), u.id))
            .map(|u| u.id);
        let Some(id) = taker else { continue };
        let s = crate::stats::of(ctx, w, id);
        let (hp, mana) = (s.max_hp * 1000, s.max_mana * 1000);
        if let Some(u) = w.get_mut(id) {
            u.hp = (u.hp + hp * a.relic_pct / 100).min(hp);
            u.mana = (u.mana + mana * a.relic_pct / 100).min(mana);
        }
        w.relics[i] = now + (a.relic_back * TPS) as u32;
    }
}
