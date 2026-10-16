//! The match's clock and the units no player controls: creep waves down the
//! three lanes, towers, neutral camps, runes, the boss, periodic gold and
//! respawns.
//!
//! Creeps walk their lane and fight the nearest enemy they see. A hero who
//! attacks an enemy hero near enemy creeps or towers draws their aggro for a
//! moment, as in DotA, which is what makes harassing under a tower costly.
//! Towers keep a target while it stays in range. Neutrals stay at their camp
//! and fight back when struck, then walk home when led too far.

use crate::rules::{Kind, P, SUB, TPS};
use crate::stats;
use crate::world::{
    dist2, lane_code, other, within, Ctx, Mod, Uid, World, EV_BOSS, EV_HORN, EV_RESPAWN, EV_RUNE, M, MOON,
    NEUTRAL, NO_UNIT, SUN,
};

/// Rune kinds.
pub const RUNE_HASTE: u8 = 0;
pub const RUNE_DOUBLE: u8 = 1;
pub const RUNE_ILLUSION: u8 = 2;
pub const RUNE_INVIS: u8 = 3;
pub const RUNE_REGEN: u8 = 4;
pub const RUNE_KINDS: u32 = 5;

const LANES: [&str; 3] = ["top", "mid", "bot"];

/// Neutral camps are this far across; a camp is blocked while any unit stands in it.
pub const CAMP_RADIUS: i32 = 400;
/// A neutral led farther than this from home turns back.
pub const LEASH: i32 = 900;

pub fn clock(w: &mut World, ctx: &Ctx) {
    let t = &ctx.rules.t;
    let tick = w.tick;
    if tick == 0 {
        spawn_boss(w, ctx);
    }
    if tick == w.horn_at {
        w.event(EV_HORN, 0, 0, [0, 0]);
    }
    // creep waves
    if tick >= w.horn_at && (tick - w.horn_at).is_multiple_of((t.wave_every * TPS) as u32) {
        w.waves += 1;
        spawn_waves(w, ctx);
    }
    // periodic gold, from the horn
    let every = (t.periodic_every * TPS / 100).max(1) as u32;
    if tick > w.horn_at && tick.is_multiple_of(every) {
        for p in w.players.iter_mut() {
            if p.present && p.hero != NO_UNIT {
                p.gold += t.periodic_gold;
            }
        }
    }
    // neutral camps: first half a minute after the horn, then on every minute mark
    let first = w.horn_at + (t.neutral_first * TPS) as u32;
    let minute = (t.neutral_every * TPS) as u32;
    if tick == first || (tick > first && (tick - w.horn_at).is_multiple_of(minute)) {
        spawn_camps(w, ctx);
    }
    // runes: every even minute from the horn, unless one still waits
    let rune_every = (t.rune_every * TPS) as u32;
    if tick >= w.horn_at
        && (tick - w.horn_at).is_multiple_of(rune_every)
        && !ctx.rules.map.runes.is_empty()
        && w.rune.is_none()
    {
        let spot = w.rng.below(ctx.rules.map.runes.len() as u32) as usize;
        let kind = w.rng.below(RUNE_KINDS) as u8;
        w.rune = Some((spot, kind));
    }
    pick_runes(w, ctx);
    if tick.is_multiple_of(15) {
        protect(w, ctx);
    }
    crate::courier::run(w, ctx);
    // the boss
    if w.boss_at != 0 && tick >= w.boss_at {
        w.boss_at = 0;
        spawn_boss(w, ctx);
    }
    respawn(w, ctx);
}

fn spawn_boss(w: &mut World, ctx: &Ctx) {
    let t = &ctx.rules.t;
    if let Some(def) = ctx.rules.units.iter().position(|d| d.kind == Kind::Boss) {
        // the boss grows with the match's clock, not with its deaths
        let since = w.tick.saturating_sub(w.horn_at) as i32 / TPS;
        let steps = since / t.boss_growth_every.max(1);
        let id = w.spawn(ctx, def, NEUTRAL, ctx.rules.map.boss);
        if let Some(u) = w.get_mut(id) {
            u.camp = 255;
            u.facing = crate::fx::deg(270);
            u.upgrade = steps;
            u.hp += steps * t.boss_growth_hp * 1000;
            if steps > 0 {
                u.mods.push(Mod {
                    kind: M::Damage,
                    src: id,
                    until: 0,
                    a: steps * t.boss_growth_damage,
                    b: 0,
                    tag: 9002,
                });
            }
        }
    }
}

pub fn boss_died(w: &mut World, ctx: &Ctx, killer: Uid, at: P) {
    let [lo, hi] = ctx.rules.t.boss_respawn;
    let secs = w.rng.between(lo, hi);
    w.boss_at = w.tick + (secs * TPS) as u32;
    let team = w.get(killer).map(|k| k.team).unwrap_or(NEUTRAL);
    // the shield drops where it fell
    crate::item::drop_aegis(w, ctx, killer, at);
    w.event(EV_BOSS, team as i32, 0, at);
}

/// Waves for both sides in all three lanes.
fn spawn_waves(w: &mut World, ctx: &Ctx) {
    let t = &ctx.rules.t;
    let elapsed = (w.tick - w.horn_at) as i32 / TPS;
    let upgrade = (elapsed / t.creep_upgrade_every.max(1)).min(t.creep_upgrade_max);
    let siege = t.siege_every > 0 && w.waves % t.siege_every == 0;
    for team in [SUN, MOON] {
        let side = &ctx.rules.map.sides[team as usize];
        for (li, lane) in LANES.iter().enumerate() {
            let Some(spawn) = side.creep_spawn.get(*lane) else {
                continue;
            };
            // mega creeps where the enemy barracks of this lane have fallen:
            // the melee barracks for melee, the ranged for ranged and siege
            let rax_up = |melee: bool| {
                w.units.iter().any(|u| {
                    u.alive
                        && u.team == other(team)
                        && ctx.rules.units[u.def as usize].kind == Kind::Barracks
                        && u.lane_name == lane_code(lane)
                        && ctx.rules.units[u.def as usize].id.contains("melee") == melee
                })
            };
            let all_down = !w.units.iter().any(|u| {
                u.alive && u.team == other(team) && ctx.rules.units[u.def as usize].kind == Kind::Barracks
            });
            let mega = [!rax_up(true), !rax_up(false), !rax_up(false)];
            let level = if all_down { t.creep_upgrade_max } else { upgrade };
            let extra_melee = t.extra_melee_at.iter().filter(|&&s| elapsed >= s).count();
            let extra_ranged = t.extra_ranged_at.iter().filter(|&&s| elapsed >= s).count();
            let mut list = vec![0; t.creeps_melee as usize + extra_melee];
            list.extend(std::iter::repeat_n(1, t.creeps_ranged as usize + extra_ranged));
            if siege {
                list.push(2);
            }
            list.extend(std::iter::repeat_n(2, extra_ranged));
            for (k, which) in list.into_iter().enumerate() {
                let base = &side.creeps[which];
                let name = if mega[which] {
                    base.replace("_sun", "_mega_sun").replace("_moon", "_mega_moon")
                } else {
                    base.clone()
                };
                let Some(def) = ctx.rules.unit(&name).or_else(|| ctx.rules.unit(base)) else {
                    continue;
                };
                // a column, two abreast, so the wave does not start stacked
                let off = [(k as i32 % 2) * 60 - 30, -(k as i32 / 2) * 60];
                let at = [spawn[0] + off[0], spawn[1] + off[1]];
                let id = w.spawn(ctx, def, team, at);
                let max = {
                    let u = w.get_mut(id).expect("just spawned");
                    u.lane = li as u8;
                    u.lane_name = li as u8;
                    u.waypoint = 0;
                    u.upgrade = level;
                    0
                };
                let _ = max;
                let s = stats::of(ctx, w, id);
                w.get_mut(id).expect("just spawned").hp = s.max_hp * 1000;
            }
        }
    }
}

fn spawn_camps(w: &mut World, ctx: &Ctx) {
    for (k, camp) in ctx.rules.map.camps.iter().enumerate() {
        let tag = k as u8 + 1;
        let occupied = w
            .units
            .iter()
            .any(|u| u.alive && (u.camp == tag || within(u.pos(), camp.at, CAMP_RADIUS)));
        if occupied || camp.groups.is_empty() {
            continue;
        }
        let g = w.rng.below(camp.groups.len() as u32) as usize;
        for (j, name) in camp.groups[g].iter().enumerate() {
            let Some(def) = ctx.rules.unit(name) else { continue };
            let a = crate::fx::TURN * j as i32 / camp.groups[g].len().max(1) as i32;
            let at = [
                camp.at[0] + crate::fx::cos(a) * 90 / crate::fx::ONE,
                camp.at[1] + crate::fx::sin(a) * 90 / crate::fx::ONE,
            ];
            let id = w.spawn(ctx, def, NEUTRAL, at);
            let facing = w.rng.below(crate::fx::TURN as u32) as i32;
            let u = w.get_mut(id).expect("just spawned");
            u.camp = tag;
            u.home = at;
            u.facing = facing;
        }
    }
}

fn pick_runes(w: &mut World, ctx: &Ctx) {
    let Some((spot, kind)) = w.rune else { return };
    let at = ctx.rules.map.runes[spot];
    let taker = w
        .units
        .iter()
        .find(|u| u.alive && u.hero.is_some() && within(u.pos(), at, 150))
        .map(|u| u.id);
    let Some(h) = taker else { return };
    w.rune = None;
    let tick = w.tick;
    let secs = |s: i32| tick + (s * TPS) as u32;
    let m = |kind: M, until: u32, a: i32| Mod {
        kind,
        src: h,
        until,
        a,
        b: 0,
        tag: 9000,
    };
    match kind {
        RUNE_HASTE => w.add_mod(h, m(M::Haste, secs(30), 0)),
        RUNE_DOUBLE => w.add_mod(h, m(M::DoubleDamage, secs(45), 0)),
        RUNE_INVIS => w.add_mod(h, m(M::Invisible, secs(45), 0)),
        RUNE_REGEN => {
            let s = stats::of(ctx, w, h);
            // DotA's regeneration rune: full health and mana over 30 s, ended by damage
            w.add_mod(h, m(M::Regen, secs(30), s.max_hp * 100 / 30));
            w.add_mod(h, m(M::ManaRegen, secs(30), s.max_mana * 100 / 30));
        }
        _ => crate::ability::illusions(w, ctx, h, 2, 75, 35, 300),
    }
    let team = w.get(h).map(|u| u.team as i32).unwrap_or(0);
    w.event(EV_RUNE, kind as i32, team, at);
}

fn respawn(w: &mut World, ctx: &Ctx) {
    let tick = w.tick;
    for p in 0..w.players.len() {
        let id = w.players[p].hero;
        let Some(u) = w.get(id) else { continue };
        if u.alive {
            continue;
        }
        let due = u.hero.as_ref().map(|h| h.respawn_at <= tick).unwrap_or(false);
        if due {
            revive(w, ctx, id);
        }
    }
}

pub fn revive(w: &mut World, ctx: &Ctx, id: Uid) {
    let Some(u) = w.get(id) else { return };
    let team = u.team;
    let spawn = ctx.rules.map.sides[team as usize].spawn;
    let tick = w.tick;
    {
        let u = w.get_mut(id).expect("checked");
        u.alive = true;
        u.x = spawn[0] * SUB;
        u.y = spawn[1] * SUB;
        u.order = crate::world::Order::Idle;
        u.act = crate::world::Act::None;
        u.target = NO_UNIT;
        u.path.clear();
        u.anim = crate::world::ANIM_STAND;
        u.anim_at = tick;
    }
    let s = stats::of(ctx, w, id);
    let u = w.get_mut(id).expect("checked");
    u.hp = s.max_hp * 1000;
    u.mana = s.max_mana * 1000;
    w.event(EV_RESPAWN, 0, 0, spawn);
}

pub fn buyback_cost(w: &World, ctx: &Ctx, player: usize) -> i32 {
    let t = &ctx.rules.t;
    let level = w.get(w.players[player].hero).map(|u| u.level()).unwrap_or(1);
    let minutes = w.tick.saturating_sub(w.horn_at) as i32 / (60 * TPS);
    let cost = t.buyback_base + level * level * t.buyback_per_level_sq / 100 + minutes * t.buyback_per_minute;
    cost / t.buyback_round.max(1) * t.buyback_round.max(1)
}

pub fn buyback(w: &mut World, ctx: &Ctx, player: usize) {
    let id = w.players[player].hero;
    let dead = w.get(id).map(|u| !u.alive).unwrap_or(false);
    if !dead || w.players[player].buyback_ready > w.tick {
        return;
    }
    let cost = buyback_cost(w, ctx, player);
    if w.players[player].gold < cost {
        return;
    }
    let p = &mut w.players[player];
    p.gold -= cost;
    p.reliable = p.reliable.min(p.gold);
    p.buyback_ready = w.tick + (ctx.rules.t.buyback_cooldown * TPS) as u32;
    revive(w, ctx, id);
}

pub fn glyph(w: &mut World, ctx: &Ctx, team: u8) {
    let k = team.min(1) as usize;
    if w.glyph_ready[k] > w.tick {
        return;
    }
    w.glyph_ready[k] = w.tick + (ctx.rules.t.glyph_cooldown * TPS) as u32;
    let until = w.tick + (ctx.rules.t.glyph_time * TPS) as u32;
    let ids: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| u.alive && u.team == team && ctx.rules.units[u.def as usize].kind == Kind::Tower)
        .map(|u| u.id)
        .collect();
    for id in ids {
        w.add_mod(
            id,
            Mod {
                kind: M::Glyph,
                src: id,
                until,
                a: 0,
                b: 0,
                tag: 9001,
            },
        );
    }
}

/// When a hero attacks an enemy hero, enemy creeps and towers near the
/// attacker turn on the attacker for a moment.
pub fn draw_aggro(w: &mut World, ctx: &Ctx, attacker: Uid) {
    let Some(a) = w.get(attacker) else { return };
    let (team, at) = (a.team, a.pos());
    let r = ctx.rules.t.creep_aggro_range;
    let until = w.tick + (ctx.rules.t.creep_aggro_time * TPS) as u32;
    let ids = w.near(at, r.max(800), |u| {
        u.team == other(team)
            && u.team != NEUTRAL
            && matches!(ctx.rules.units[u.def as usize].kind, Kind::Creep | Kind::Tower)
    });
    for id in ids {
        let s = stats::of(ctx, w, id);
        let u = w.get_mut(id).expect("listed");
        let kind = ctx.rules.units[u.def as usize].kind;
        // a tower turns only if the attacker is in its range; creeps within the aggro range
        let reach = if kind == Kind::Tower { s.range + 100 } else { r };
        if dist2(u.pos(), at) <= (reach as i64) * (reach as i64) {
            u.target = attacker;
            u.aggro_until = until;
        }
    }
}

/// Creeps, towers, neutrals and the boss choose and carry out what to do.
pub fn behave(w: &mut World, ctx: &Ctx) {
    let ids: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| u.alive && u.owner < 0 && !matches!(u.act, crate::world::Act::Swing { .. }))
        .filter(|u| !matches!(u.act, crate::world::Act::Backswing { until } if until > w.tick))
        .map(|u| u.id)
        .collect();
    for id in ids {
        let Some(u) = w.live(id) else { continue };
        if u.disabled() {
            continue;
        }
        let kind = ctx.rules.units[u.def as usize].kind;
        match kind {
            Kind::Creep => creep(w, ctx, id),
            Kind::Tower | Kind::Fountain => tower(w, ctx, id),
            Kind::Neutral | Kind::Boss => neutral(w, ctx, id),
            _ => {}
        }
    }
}

/// The best enemy for a lane unit within `r`: units first, heroes after,
/// nearest within each.
fn lane_target(w: &World, ctx: &Ctx, id: Uid, r: i32, heroes_last: bool) -> Option<Uid> {
    let u = w.get(id)?;
    let (team, here) = (u.team, u.pos());
    let mut best: Option<(i64, Uid)> = None;
    for v in &w.units {
        if !v.alive || v.team == team || v.team == NEUTRAL || v.has(M::Invulnerable) {
            continue;
        }
        let k = ctx.rules.units[v.def as usize].kind;
        if matches!(k, Kind::Shop | Kind::Ward) {
            continue;
        }
        if !w.sees(ctx, team, v) {
            continue;
        }
        let d = dist2(here, v.pos());
        let reach = r + ctx.rules.units[v.def as usize].footprint;
        if d > (reach as i64) * (reach as i64) {
            continue;
        }
        // heroes count as farther away, so creeps are preferred
        let key = if heroes_last && v.hero.is_some() {
            d + (1i64 << 40)
        } else {
            d
        };
        if best.map(|b| key < b.0).unwrap_or(true) {
            best = Some((key, v.id));
        }
    }
    best.map(|b| b.1)
}

fn creep(w: &mut World, ctx: &Ctx, id: Uid) {
    let u = w.get(id).expect("listed");
    let (team, lane, wp) = (u.team, u.lane as usize, u.waypoint as usize);
    let acquire = ctx.rules.units[u.def as usize].acquire;
    let mut current = u.target;
    // a hero is kept as the target only while the aggro it drew lasts, or
    // while no enemy creep is near
    let on_hero = w.live(current).map(|t| t.hero.is_some()).unwrap_or(false);
    if on_hero && u.aggro_until <= w.tick {
        if let Some(c) = lane_target(w, ctx, id, acquire, true) {
            if w.get(c).map(|t| t.hero.is_none()).unwrap_or(false) {
                current = c;
            }
        }
    }
    let keep = u.aggro_until > w.tick || current != NO_UNIT;
    if keep && current != NO_UNIT && w.live(current).is_some() {
        let near = w
            .get(current)
            .map(|t| within(t.pos(), w.get(id).expect("listed").pos(), acquire + 300))
            .unwrap_or(false);
        if near && crate::orders::engage(w, ctx, id, current, true) {
            return;
        }
    }
    if let Some(u) = w.get_mut(id) {
        u.target = NO_UNIT;
    }
    // lane creeps fight enemy creeps first and heroes only when no creep is
    // near (the review, §2.12, after the Dota 2 port)
    if let Some(t) = lane_target(w, ctx, id, acquire, true) {
        crate::orders::engage(w, ctx, id, t, true);
        return;
    }
    let side = &ctx.rules.map.sides[team as usize];
    let Some(path) = side.lanes.get(LANES[lane.min(2)]) else {
        return;
    };
    if wp >= path.len() {
        return;
    }
    let goal = path[wp];
    if crate::orders::walk(w, ctx, id, goal, 80) {
        let u = w.get_mut(id).expect("listed");
        u.waypoint = (wp + 1).min(path.len()) as u8;
        u.path.clear();
    }
}

fn tower(w: &mut World, ctx: &Ctx, id: Uid) {
    let s = stats::of(ctx, w, id);
    if s.range <= 0 {
        return;
    }
    let u = w.get(id).expect("listed");
    let current = u.target;
    if current != NO_UNIT
        && crate::orders::in_range(w, ctx, id, current, s.range)
        && w.live(current).is_some()
    {
        let ready = u.attack_ready <= w.tick;
        if ready {
            crate::combat::swing(w, ctx, id, current);
        }
        return;
    }
    let pick = lane_target(w, ctx, id, s.range + 30, true);
    let u = w.get_mut(id).expect("listed");
    u.target = pick.unwrap_or(NO_UNIT);
    if let Some(t) = pick {
        if u.attack_ready <= w.tick {
            crate::combat::swing(w, ctx, id, t);
        }
    }
}

fn neutral(w: &mut World, ctx: &Ctx, id: Uid) {
    let u = w.get(id).expect("listed");
    let home = u.home;
    let here = u.pos();
    let far = !within(here, home, LEASH);
    let attacker = u.last_attacker;
    let recently = u.hit_at + (3 * TPS) as u32 > w.tick;
    if far {
        // led too far: walk home, ignoring everything
        if let Some(u) = w.get_mut(id) {
            u.target = NO_UNIT;
        }
        crate::orders::walk(w, ctx, id, home, 40);
        return;
    }
    let t = w.get(id).expect("listed").target;
    if t != NO_UNIT && w.live(t).is_some() && crate::orders::engage(w, ctx, id, t, true) {
        return;
    }
    if recently && w.live(attacker).map(|a| a.team != NEUTRAL).unwrap_or(false) {
        // the whole camp answers an attack on any of them
        let camp = w.get(id).expect("listed").camp;
        let mates: Vec<Uid> = w
            .units
            .iter()
            .filter(|v| v.alive && v.camp == camp && v.team == NEUTRAL)
            .map(|v| v.id)
            .collect();
        for m in mates {
            if let Some(v) = w.get_mut(m) {
                if v.target == NO_UNIT {
                    v.target = attacker;
                }
            }
        }
        crate::orders::engage(w, ctx, id, attacker, true);
        return;
    }
    if !within(here, home, 40) {
        crate::orders::walk(w, ctx, id, home, 40);
    } else if let Some(u) = w.get_mut(id) {
        u.target = NO_UNIT;
        if u.anim == crate::world::ANIM_WALK {
            u.anim = crate::world::ANIM_STAND;
        }
    }
}

/// Which buildings can be hurt: a tower only once the tower before it in
/// its lane has fallen; the two by the Sphere once any third-tier tower
/// has; barracks once their lane's third tower has; the Sphere once both
/// towers beside it have. Backdoor protection: a guarded building with no
/// enemy creeps near it for a while regenerates quickly.
fn protect(w: &mut World, ctx: &Ctx) {
    let t = &ctx.rules.t;
    let standing = |w: &World, team: u8, tier: i32, lane: Option<u8>| {
        w.units.iter().any(|u| {
            u.alive
                && u.team == team
                && ctx.rules.units[u.def as usize].kind == Kind::Tower
                && u.tier == tier
                && lane.map(|l| u.lane_name == l).unwrap_or(true)
        })
    };
    let ids: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| {
            u.alive
                && matches!(
                    ctx.rules.units[u.def as usize].kind,
                    Kind::Tower | Kind::Barracks | Kind::Sphere
                )
        })
        .map(|u| u.id)
        .collect();
    let calm = (t.backdoor_calm * TPS) as u32;
    for id in ids {
        let u = w.get(id).expect("listed");
        let (team, tier, lane, kind) = (u.team, u.tier, u.lane_name, ctx.rules.units[u.def as usize].kind);
        let shielded = match kind {
            Kind::Tower if tier == 4 => (0..3u8).all(|l| standing(w, team, 3, Some(l))),
            Kind::Tower if tier > 1 => standing(w, team, tier - 1, Some(lane)),
            Kind::Barracks => standing(w, team, 3, Some(lane)),
            Kind::Sphere => standing(w, team, 4, None),
            _ => false,
        };
        let creeps_near = !w
            .near(u.pos(), t.backdoor_radius, |v| {
                v.team == other(team)
                    && v.team != NEUTRAL
                    && ctx.rules.units[v.def as usize].kind == Kind::Creep
            })
            .is_empty();
        let tick = w.tick;
        let guarded = kind != Kind::Tower || tier >= 2;
        let u = w.get_mut(id).expect("listed");
        u.mods.retain(|m| m.tag != 9100 && m.tag != 9101);
        if shielded {
            u.mods.push(Mod {
                kind: M::Invulnerable,
                src: id,
                until: 0,
                a: 0,
                b: 0,
                tag: 9100,
            });
        }
        if creeps_near {
            u.quiet = tick;
        }
        if guarded && tick >= u.quiet + calm {
            u.mods.push(Mod {
                kind: M::Regen,
                src: id,
                until: 0,
                a: t.backdoor_regen,
                b: 0,
                tag: 9101,
            });
        }
    }
}
