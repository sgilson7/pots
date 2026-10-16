//! Orders: what a player tells their hero, and how every unit carries out
//! the order it holds, tick by tick.
//!
//! A player's command arrives as a `Cmd`, the same few integers whether it
//! came from this browser's mouse, a computer player, or another browser
//! over the network. Commands only change a unit's order; the order is then
//! carried out by the same rule for every unit: turn at the unit's turn
//! rate, walk the path around trees, swing when in range and ready, cast
//! when in range and facing.

use crate::combat;
use crate::fx;
use crate::path;
use crate::rules::{Kind, Target, P, SUB};
use crate::stats;
use crate::world::{dist2, within, Act, Ctx, Order, Uid, World, M, NO_UNIT};

/// One command from a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cmd {
    /// Choose a hero from the roster (before the hero is on the map).
    Pick(u8),
    Move(P),
    AttackMove(P),
    Attack(Uid),
    Stop,
    Hold,
    Cast(u8, Uid, P),
    Item(u8, Uid, P),
    /// Spend a point on ability `slot`; slot 4 is the attribute bonus.
    Learn(u8),
    Buy(u16),
    Sell(u8),
    Swap(u8, u8),
    Buyback,
    Glyph,
    /// The seat's computer player takes over at a difficulty, or hands back (255).
    Autopilot(u8),
}

pub const CMD_VALUES: usize = 5;

impl Cmd {
    /// The command as five integers, for the network and for replays.
    pub fn encode(self) -> [i32; CMD_VALUES] {
        match self {
            Cmd::Pick(k) => [1, k as i32, 0, 0, 0],
            Cmd::Move(p) => [2, p[0], p[1], 0, 0],
            Cmd::AttackMove(p) => [3, p[0], p[1], 0, 0],
            Cmd::Attack(t) => [4, t as i32, 0, 0, 0],
            Cmd::Stop => [5, 0, 0, 0, 0],
            Cmd::Hold => [6, 0, 0, 0, 0],
            Cmd::Cast(s, t, p) => [7, s as i32, t as i32, p[0], p[1]],
            Cmd::Item(s, t, p) => [8, s as i32, t as i32, p[0], p[1]],
            Cmd::Learn(s) => [9, s as i32, 0, 0, 0],
            Cmd::Buy(k) => [10, k as i32, 0, 0, 0],
            Cmd::Sell(s) => [11, s as i32, 0, 0, 0],
            Cmd::Swap(a, b) => [12, a as i32, b as i32, 0, 0],
            Cmd::Buyback => [13, 0, 0, 0, 0],
            Cmd::Glyph => [14, 0, 0, 0, 0],
            Cmd::Autopilot(d) => [15, d as i32, 0, 0, 0],
        }
    }

    pub fn decode(v: [i32; CMD_VALUES]) -> Option<Cmd> {
        let small = |x: i32| u8::try_from(x).ok();
        Some(match v[0] {
            1 => Cmd::Pick(small(v[1])?),
            2 => Cmd::Move([v[1], v[2]]),
            3 => Cmd::AttackMove([v[1], v[2]]),
            4 => Cmd::Attack(v[1] as u32),
            5 => Cmd::Stop,
            6 => Cmd::Hold,
            7 => Cmd::Cast(small(v[1])?, v[2] as u32, [v[3], v[4]]),
            8 => Cmd::Item(small(v[1])?, v[2] as u32, [v[3], v[4]]),
            9 => Cmd::Learn(small(v[1])?),
            10 => Cmd::Buy(u16::try_from(v[1]).ok()?),
            11 => Cmd::Sell(small(v[1])?),
            12 => Cmd::Swap(small(v[1])?, small(v[2])?),
            13 => Cmd::Buyback,
            14 => Cmd::Glyph,
            15 => Cmd::Autopilot(small(v[1])?),
            _ => return None,
        })
    }
}

fn clamp_point(ctx: &Ctx, p: P) -> P {
    let s = ctx.rules.map.size - 1;
    [p[0].clamp(0, s), p[1].clamp(0, s)]
}

/// Applies one player's command. A command that does not fit the moment
/// (a cast without mana, a purchase away from a shop) does nothing.
pub fn apply(w: &mut World, ctx: &Ctx, player: usize, cmd: Cmd) {
    if player >= w.players.len() {
        return;
    }
    let hero = w.players[player].hero;
    match cmd {
        Cmd::Pick(k) => {
            if hero == NO_UNIT && (k as usize) < ctx.rules.heroes.len() {
                w.add_hero(ctx, player, k as usize);
            }
        }
        Cmd::Autopilot(d) => {
            w.players[player].bot = if d == 255 { None } else { Some(d) };
            w.players[player].present = true;
        }
        Cmd::Buy(k) => crate::item::buy(w, ctx, player, k as usize),
        Cmd::Sell(s) => crate::item::sell(w, ctx, player, s as usize),
        Cmd::Swap(a, b) => {
            if let Some(u) = w.get_mut(hero) {
                if (a as usize) < 6 && (b as usize) < 6 {
                    u.items.swap(a as usize, b as usize);
                }
            }
        }
        Cmd::Learn(s) => crate::ability::learn(w, ctx, hero, s as usize),
        Cmd::Buyback => crate::lanes::buyback(w, ctx, player),
        Cmd::Glyph => crate::lanes::glyph(w, ctx, w.players[player].team),
        _ => {
            let Some(u) = w.live(hero) else { return };
            if u.has(M::Stun) && !matches!(cmd, Cmd::Stop | Cmd::Hold) {
                // a stunned hero keeps the order for when the stun ends
            }
            let order = match cmd {
                Cmd::Move(p) => Order::Move(clamp_point(ctx, p)),
                Cmd::AttackMove(p) => Order::AttackMove(clamp_point(ctx, p)),
                Cmd::Attack(t) => {
                    let Some(v) = w.live(t) else { return };
                    // an attack order on an enemy hero draws the aggro of
                    // enemy creeps and towers near the attacker; spells do not
                    if v.hero.is_some() && v.team != u.team {
                        crate::lanes::draw_aggro(w, ctx, hero);
                    }
                    Order::Attack(t)
                }
                Cmd::Stop => Order::Idle,
                Cmd::Hold => Order::Hold,
                Cmd::Cast(s, t, p) => {
                    if !crate::ability::can_start(w, ctx, hero, s as usize, t, p) {
                        return;
                    }
                    let target = ability_target(ctx, w, hero, s as usize);
                    if target == Some(Target::None) || target == Some(Target::Toggle) {
                        // an ability without a target goes off at once
                        crate::ability::begin(w, ctx, hero, s as usize, NO_UNIT, p);
                        return;
                    }
                    Order::Cast(s, t, clamp_point(ctx, p))
                }
                Cmd::Item(s, t, p) => {
                    if !crate::item::can_use(w, ctx, hero, s as usize, t, p) {
                        return;
                    }
                    if crate::item::instant(w, ctx, hero, s as usize) {
                        crate::item::use_item(w, ctx, hero, s as usize, t, p);
                        return;
                    }
                    Order::Item(s, t, clamp_point(ctx, p))
                }
                _ => return,
            };
            let u = w.get_mut(hero).expect("checked live");
            // a new order cancels a swing that has not landed and a channel
            if !matches!(u.act, Act::Cast { .. }) {
                u.act = Act::None;
            }
            u.order = order;
            u.path.clear();
            u.target = NO_UNIT;
        }
    }
}

fn ability_target(ctx: &Ctx, w: &World, id: Uid, slot: usize) -> Option<Target> {
    let u = w.get(id)?;
    let k = *ctx.rules.unit_abilities[u.def as usize].get(slot)?;
    Some(ctx.rules.abilities[k].target)
}

/// Turn units per tick for a Warcraft III turn rate (thousandths of a
/// radian per 0.03 s): rate/1000 · (1/30)/0.03 radians · 65 536/2π.
pub fn turn_per_tick(turn: i32) -> i32 {
    turn * 11_589 / 1000
}

/// Turns toward `angle`; returns whether the unit now faces within a
/// sixteenth of a turn.
pub fn turn_to(ctx: &Ctx, w: &mut World, id: Uid, angle: i32) -> bool {
    let Some(u) = w.get(id) else { return false };
    let rate = turn_per_tick(ctx.rules.units[u.def as usize].turn).max(1);
    let u = w.get_mut(id).expect("checked");
    let diff = fx::wrap(angle - u.facing);
    if diff.abs() <= rate {
        u.facing = angle & (fx::TURN - 1);
    } else {
        u.facing = (u.facing + rate * diff.signum()) & (fx::TURN - 1);
    }
    fx::wrap(angle - u.facing).abs() <= fx::TURN / 16
}

fn angle_to(a: P, b: P) -> i32 {
    fx::atan2(b[1] - a[1], b[0] - a[0])
}

/// One step toward `goal`, finding a path around trees when the straight
/// line is blocked. Returns true once within `stop` map units.
pub fn walk(w: &mut World, ctx: &Ctx, id: Uid, goal: P, stop: i32) -> bool {
    let s = stats::of(ctx, w, id);
    let Some(u) = w.get(id) else { return true };
    let here = u.pos();
    if within(here, goal, stop) {
        let u = w.get_mut(id).expect("checked");
        u.path.clear();
        if u.anim == crate::world::ANIM_WALK {
            u.anim = crate::world::ANIM_STAND;
        }
        return true;
    }
    if s.move_speed == 0 {
        return false;
    }
    let stale = u.path.is_empty() || dist2(u.path_goal, goal) > 160 * 160;
    if stale {
        let route = path::find(&w.walk(ctx.terrain), here, goal);
        let u = w.get_mut(id).expect("checked");
        u.path = route.into_iter().rev().collect();
        u.path_goal = goal;
        if u.path.is_empty() {
            u.path.push(goal);
        }
    }
    let next = {
        let u = w.get_mut(id).expect("checked");
        while u.path.len() > 1 && within(here, *u.path.last().expect("not empty"), 24) {
            u.path.pop();
        }
        *u.path.last().expect("not empty")
    };
    let angle = angle_to(here, next);
    let facing_ok = {
        let rate = turn_per_tick(ctx.rules.units[w.get(id).expect("checked").def as usize].turn).max(1);
        let u = w.get_mut(id).expect("checked");
        let diff = fx::wrap(angle - u.facing);
        if diff.abs() <= rate {
            u.facing = angle & (fx::TURN - 1);
        } else {
            u.facing = (u.facing + rate * diff.signum()) & (fx::TURN - 1);
        }
        fx::wrap(angle - u.facing).abs() < fx::QUARTER
    };
    if !facing_ok {
        return false;
    }
    let step = s.step();
    let tick = w.tick;
    let u = w.get_mut(id).expect("checked");
    let (dx, dy) = ((next[0] * SUB - u.x) as i64, (next[1] * SUB - u.y) as i64);
    let d = fx::isqrt((dx * dx + dy * dy) as u64) as i64;
    if d <= step as i64 {
        u.x = next[0] * SUB;
        u.y = next[1] * SUB;
        if u.path.len() > 1 {
            u.path.pop();
        }
    } else {
        u.x += (dx * step as i64 / d) as i32;
        u.y += (dy * step as i64 / d) as i32;
    }
    if u.anim != crate::world::ANIM_WALK {
        u.anim = crate::world::ANIM_WALK;
        u.anim_at = tick;
    }
    false
}

/// The nearest enemy this unit can see within `r`, preferring heroes when
/// `heroes_first`.
pub fn nearest_enemy(w: &World, ctx: &Ctx, id: Uid, r: i32) -> Option<Uid> {
    let u = w.get(id)?;
    let here = u.pos();
    let team = u.team;
    let mut best: Option<(i64, Uid)> = None;
    for v in &w.units {
        if !v.alive || v.team == team || v.has(M::Invulnerable) {
            continue;
        }
        let kind = ctx.rules.units[v.def as usize].kind;
        if matches!(kind, Kind::Shop | Kind::Fountain) {
            continue;
        }
        // neutrals are not attacked unprovoked by lane units
        if v.team == crate::world::NEUTRAL && u.team != crate::world::NEUTRAL && u.hero.is_none() {
            continue;
        }
        if u.hero.is_some() && v.team == crate::world::NEUTRAL {
            continue;
        }
        if !w.sees(ctx, team, v) {
            continue;
        }
        let d = dist2(here, v.pos());
        if d <= (r as i64) * (r as i64) && best.map(|b| d < b.0).unwrap_or(true) {
            best = Some((d, v.id));
        }
    }
    best.map(|b| b.1)
}

/// Whether `t` is within attack range of `id`.
pub fn in_range(w: &World, ctx: &Ctx, id: Uid, t: Uid, range: i32) -> bool {
    match (w.get(id), w.get(t)) {
        (Some(a), Some(b)) => combat::gap(ctx, a, b) <= range,
        _ => false,
    }
}

/// Pursue and attack `t`. Returns false when the target is gone.
pub fn engage(w: &mut World, ctx: &Ctx, id: Uid, t: Uid, may_move: bool) -> bool {
    let Some(target) = w.live(t) else { return false };
    let tp = target.pos();
    let team = w.get(id).map(|u| u.team).unwrap_or(0);
    if !w.sees(ctx, team, target) {
        return false;
    }
    // a deny is allowed only below the table's share of health
    if target.team == team {
        let max = stats::of(ctx, w, t).max_hp * 1000;
        if target.hp * 100 > max * ctx.rules.t.deny_below {
            return false;
        }
    }
    let s = stats::of(ctx, w, id);
    if s.range <= 0
        || w.get(id)
            .map(|u| u.has(M::Hex) || u.has(M::Disarm))
            .unwrap_or(true)
    {
        return false;
    }
    if in_range(w, ctx, id, t, s.range) {
        let here = w.get(id).expect("checked").pos();
        let facing = turn_to(ctx, w, id, angle_to(here, tp));
        let ready = w.get(id).map(|u| u.attack_ready <= w.tick).unwrap_or(false);
        let u = w.get_mut(id).expect("checked");
        u.target = t;
        u.path.clear();
        if facing && ready {
            combat::swing(w, ctx, id, t);
        } else if u.anim == crate::world::ANIM_WALK {
            u.anim = crate::world::ANIM_STAND;
        }
        true
    } else if may_move {
        if let Some(u) = w.get_mut(id) {
            u.target = t;
        }
        walk(w, ctx, id, tp, 0);
        true
    } else {
        false
    }
}

/// Carries out every unit's order for this tick.
pub fn act_all(w: &mut World, ctx: &Ctx) {
    let ids: Vec<Uid> = w.units.iter().filter(|u| u.alive).map(|u| u.id).collect();
    for id in ids {
        act_one(w, ctx, id);
    }
    separate(w, ctx);
}

fn act_one(w: &mut World, ctx: &Ctx, id: Uid) {
    let Some(u) = w.live(id) else { return };
    let kind = ctx.rules.units[u.def as usize].kind;
    if u.expires != 0 && u.expires <= w.tick {
        let u = w.get_mut(id).expect("checked");
        u.hp = 0;
        return;
    }
    if u.disabled() {
        let u = w.get_mut(id).expect("checked");
        if matches!(
            u.act,
            Act::Channel { .. } | Act::Swing { .. } | Act::Cast { .. } | Act::Backswing { .. }
        ) {
            let was = u.act;
            u.act = Act::None;
            if let Act::Channel { slot, .. } = was {
                crate::ability::channel_broken(w, ctx, id, slot as usize);
            }
        }
        return;
    }
    match u.act {
        Act::Swing { t, hit_at } => {
            if w.tick >= hit_at {
                w.get_mut(id).expect("checked").act = Act::None;
                combat::release(w, ctx, id, t);
            }
            return;
        }
        Act::Cast {
            slot,
            t,
            at,
            fire_at,
            item,
        } => {
            if w.tick >= fire_at {
                w.get_mut(id).expect("checked").act = Act::None;
                if item {
                    crate::item::use_item(w, ctx, id, slot as usize, t, at);
                } else {
                    crate::ability::fire(w, ctx, id, slot as usize, t, at);
                }
            }
            return;
        }
        Act::Backswing { until } => {
            let now = w.tick;
            if now >= until {
                w.get_mut(id).expect("checked").act = Act::None;
            } else {
                return;
            }
        }
        Act::Channel { slot, t, at, until } => {
            if w.tick >= until {
                w.get_mut(id).expect("checked").act = Act::None;
                crate::ability::channel_done(w, ctx, id, slot as usize);
            } else {
                crate::ability::channel_tick(w, ctx, id, slot as usize, t, at);
            }
            return;
        }
        Act::None => {}
    }
    let Some(u) = w.live(id) else { return };
    if kind.is_building() || kind == Kind::Creep || kind == Kind::Neutral || kind == Kind::Boss {
        // these follow their own behaviour (lanes.rs) unless a player owns them
        if u.owner < 0 {
            return;
        }
    }
    // a taunt overrides every order
    if let Some(m) = u.mods.iter().find(|m| m.kind == M::Taunt).copied() {
        engage(w, ctx, id, m.src, true);
        return;
    }
    let order = u.order;
    let acquire = ctx.rules.units[u.def as usize].acquire;
    match order {
        Order::Idle => {
            let t = u.target;
            if t != NO_UNIT && w.live(t).is_some() && engage(w, ctx, id, t, true) {
                // keep fighting what we were fighting, if it stays near home
                return;
            }
            if let Some(e) = nearest_enemy(w, ctx, id, acquire) {
                engage(w, ctx, id, e, true);
            } else if let Some(u) = w.get_mut(id) {
                u.target = NO_UNIT;
                if u.anim == crate::world::ANIM_WALK {
                    u.anim = crate::world::ANIM_STAND;
                }
            }
        }
        Order::Hold => {
            let s = stats::of(ctx, w, id);
            if let Some(e) = nearest_enemy(w, ctx, id, s.range + 100) {
                engage(w, ctx, id, e, false);
            }
        }
        Order::Move(p) => {
            if walk(w, ctx, id, p, 8) {
                if let Some(u) = w.get_mut(id) {
                    u.order = Order::Idle;
                    u.target = NO_UNIT;
                }
            }
        }
        Order::Follow(t) => match w.live(t).map(|v| v.pos()) {
            Some(p) => {
                walk(w, ctx, id, p, 200);
            }
            None => w.get_mut(id).expect("checked").order = Order::Idle,
        },
        Order::Attack(t) => {
            if !engage(w, ctx, id, t, true) {
                if let Some(u) = w.get_mut(id) {
                    u.order = Order::Idle;
                    u.target = NO_UNIT;
                }
            }
        }
        Order::AttackMove(p) => {
            let t = u.target;
            if t != NO_UNIT && engage(w, ctx, id, t, true) {
                return;
            }
            if let Some(e) = nearest_enemy(w, ctx, id, acquire) {
                engage(w, ctx, id, e, true);
            } else if walk(w, ctx, id, p, 8) {
                w.get_mut(id).expect("checked").order = Order::Idle;
            }
        }
        Order::Cast(slot, t, at) => {
            crate::ability::approach(w, ctx, id, slot as usize, t, at);
        }
        Order::Item(slot, t, at) => {
            crate::item::approach(w, ctx, id, slot as usize, t, at);
        }
    }
}

/// Pushes overlapping walking units apart so a crowd spreads rather than
/// stacking on one point. Buildings and heroes in a channel do not move.
fn separate(w: &mut World, ctx: &Ctx) {
    let n = w.units.len();
    let mut push: Vec<(usize, i32, i32)> = Vec::new();
    for i in 0..n {
        let a = &w.units[i];
        if !a.alive {
            continue;
        }
        let da = &ctx.rules.units[a.def as usize];
        if da.kind.is_building() || da.speed == 0 || matches!(a.act, Act::Channel { .. }) {
            continue;
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            let b = &w.units[j];
            if !b.alive {
                continue;
            }
            let db = &ctx.rules.units[b.def as usize];
            if db.kind == Kind::Ward {
                continue;
            }
            let r = da.radius + db.radius + db.footprint;
            let (dx, dy) = (a.x - b.x, a.y - b.y);
            if dx.abs() >= r * SUB || dy.abs() >= r * SUB {
                continue;
            }
            let d2 = (dx as i64) * (dx as i64) + (dy as i64) * (dy as i64);
            let rr = (r * SUB) as i64;
            if d2 >= rr * rr {
                continue;
            }
            let d = fx::isqrt(d2 as u64) as i64;
            let overlap = rr - d;
            // a building pushes fully; two walkers share the push
            let share = if db.kind.is_building() || db.speed == 0 {
                1
            } else {
                2
            };
            let (ux, uy) = if d == 0 {
                // exactly on top: separate by slot order
                (if i < j { SUB as i64 } else { -(SUB as i64) }, 0)
            } else {
                (dx as i64 * SUB as i64 / d, dy as i64 * SUB as i64 / d)
            };
            let amount = (overlap / share).min(4 * SUB as i64);
            push.push((
                i,
                (ux * amount / SUB as i64) as i32,
                (uy * amount / SUB as i64) as i32,
            ));
        }
    }
    let walk = crate::world::Walk {
        terrain: ctx.terrain,
        trees: &w.trees,
    };
    let mut moves = Vec::new();
    for (i, dx, dy) in push {
        let u = &w.units[i];
        let (nx, ny) = (u.x + dx, u.y + dy);
        let cell = ctx.terrain.path_cell;
        let (cx, cy) = (nx / SUB / cell, ny / SUB / cell);
        if !path::Grid::blocked(&walk, cx, cy) {
            moves.push((i, nx, ny));
        }
    }
    for (i, nx, ny) in moves {
        w.units[i].x = nx;
        w.units[i].y = ny;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_survives_its_encoding() {
        for c in [
            Cmd::Pick(3),
            Cmd::Move([100, 200]),
            Cmd::AttackMove([5, 6]),
            Cmd::Attack(77),
            Cmd::Stop,
            Cmd::Hold,
            Cmd::Cast(2, 9, [1, 2]),
            Cmd::Item(5, 0, [3, 4]),
            Cmd::Learn(4),
            Cmd::Buy(12),
            Cmd::Sell(1),
            Cmd::Swap(0, 5),
            Cmd::Buyback,
            Cmd::Glyph,
            Cmd::Autopilot(2),
        ] {
            assert_eq!(Cmd::decode(c.encode()), Some(c));
        }
        assert_eq!(Cmd::decode([99, 0, 0, 0, 0]), None);
    }

    #[test]
    fn a_turn_rate_of_half_a_radian_turns_about_a_tenth_of_a_turn_a_tick() {
        // 0.5 rad per 0.03 s = 0.5556 rad per tick = 0.0884 turn = 5794 units
        assert_eq!(turn_per_tick(500), 5794);
    }
}
