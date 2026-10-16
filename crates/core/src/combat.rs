//! Hitting and dying: attack damage through the attack-type table and
//! armor, spell damage through magic resistance, missiles in flight, and
//! what a death pays out.
//!
//! Armor reduces physical damage by `f·a / (1 + f·a)` with DotA's factor
//! f = 0.06; negative armor amplifies it by `1 − 0.94^|a|`. Both are worked
//! in per-mille integers. A last hit pays its gold to the hitter's player;
//! the same unit killed by its own side is a deny and pays nothing.
//! Experience goes to enemy heroes in range, split evenly.

use crate::rules::{Kind, SUB, TPS};
use crate::stats;
use crate::world::{
    self, dist2, other, Act, Ctx, Hit, Uid, World, EV_CRIT, EV_DENY, EV_FIRST_BLOOD, EV_GOLD, EV_KILL, EV_MISS,
    EV_TOWER, EV_WIN, M, NEUTRAL, NO_UNIT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dmg {
    Physical,
    Magic,
    Pure,
}

/// Damage kept, in per-mille, after armor in hundredths (factor in
/// ten-thousandths: 0.06 is 600).
pub fn armor_keep(armor_x100: i32, factor: i32) -> i32 {
    if armor_x100 >= 0 {
        // keep = 1 / (1 + f·a)
        let fa = factor as i64 * armor_x100 as i64; // ten-thousandths × hundredths
        let denom = 1_000_000 + fa;
        ((1_000_000i64 * 1000 + denom / 2) / denom) as i32
    } else {
        // keep = 2 − 0.94^|a|, with whole armor points (DotA's table)
        let n = (-armor_x100 + 50) / 100;
        let mut p: i64 = 1000;
        for _ in 0..n {
            p = p * 940 / 1000;
        }
        (2000 - p) as i32
    }
}

/// Pseudo-random chance, as Warcraft III rolls bashes and crits: the chance
/// starts low and grows by `c` after each failure, so long droughts and
/// streaks are rare and the average holds. `key` picks the unit's counter.
pub fn prd(w: &mut World, id: Uid, key: usize, pct: i32) -> bool {
    // c in ten-thousandths for the nominal chance (Warcraft III's table)
    let c = match pct {
        p if p <= 5 => 38,
        p if p <= 10 => 147,
        p if p <= 15 => 322,
        p if p <= 17 => 402,
        p if p <= 20 => 557,
        p if p <= 25 => 847,
        p if p <= 30 => 1189,
        p if p <= 35 => 1579,
        p if p <= 40 => 2015,
        p if p <= 50 => 3021,
        _ => 10_000,
    };
    let Some(u) = w.get(id) else { return false };
    let n = u.prd[key % 6] as i32 + 1;
    let hit = (w.rng.below(10_000) as i32) < (c * n).min(10_000);
    if let Some(u) = w.get_mut(id) {
        u.prd[key % 6] = if hit { 0 } else { (n as u8).min(250) };
    }
    hit
}

/// Applies damage in thousandths of a hit point. Returns the damage dealt.
pub fn damage_milli(w: &mut World, ctx: &Ctx, src: Uid, dst: Uid, amount: i32, kind: Dmg, attack: bool) -> i32 {
    let Some(v) = w.live(dst) else { return 0 };
    if v.has(M::Invulnerable) {
        return 0;
    }
    if kind == Dmg::Magic && v.has(M::SpellImmune) {
        return 0;
    }
    let mut amount = amount;
    if kind == Dmg::Magic {
        let mr = stats::of(ctx, w, dst).magic_resist;
        amount = amount * (100 - mr) / 100;
    }
    if amount <= 0 {
        return 0;
    }
    let src_hero = w.live(src).filter(|s| s.hero.is_some()).map(|s| (s.id, s.team));
    let tick = w.tick;
    let v = w.get_mut(dst).expect("checked live");
    v.hp -= amount;
    // the regeneration rune ends on damage
    if src != dst {
        v.mods.retain(|m| !(m.tag == 9000 && matches!(m.kind, M::Regen | M::ManaRegen)));
    }
    v.hit_at = tick;
    v.last_attacker = src;
    if let (Some((sid, steam)), Some(h)) = (src_hero, v.hero.as_mut()) {
        if steam != v.team {
            h.damaged_by.retain(|(id, _)| *id != sid);
            h.damaged_by.push((sid, tick));
        }
    }
    let _ = attack;
    if v.hp <= 0 {
        v.hp = 0;
    }
    amount
}

/// Whole hit points of damage of a kind.
pub fn damage(w: &mut World, ctx: &Ctx, src: Uid, dst: Uid, amount: i32, kind: Dmg) -> i32 {
    damage_milli(w, ctx, src, dst, amount * 1000, kind, false) / 1000
}

/// Heals whole hit points, up to the maximum.
pub fn heal(w: &mut World, ctx: &Ctx, id: Uid, amount: i32) {
    let max = stats::of(ctx, w, id).max_hp * 1000;
    if let Some(u) = w.get_mut(id) {
        if u.alive {
            u.hp = (u.hp + amount * 1000).min(max);
        }
    }
}

/// Rolls an attack's damage: base within its range plus bonus.
pub fn roll(w: &mut World, ctx: &Ctx, src: Uid) -> i32 {
    let s = stats::of(ctx, w, src);
    // two dice rather than one: the middle of the range comes up more often,
    // as DotA's XdY damage does
    let span = (s.damage[1] - s.damage[0]).max(0);
    let base = s.damage[0] + w.rng.between(0, span / 2) + w.rng.between(0, span - span / 2);
    let mut dmg = base + s.bonus;
    if w.live(src).map(|u| u.has(M::DoubleDamage)).unwrap_or(false) {
        dmg += base;
    }
    dmg
}

/// An attack's hit lands: evasion, uphill miss, the type table, armor,
/// lifesteal and on-hit effects.
pub fn attack_lands(w: &mut World, ctx: &Ctx, src: Uid, dst: Uid, raw: i32, crit: bool) {
    let (Some(a), Some(v)) = (w.get(src), w.live(dst)) else { return };
    let (atk_def, def_def) = (a.def as usize, v.def as usize);
    let (a_pos, v_pos) = (a.pos(), v.pos());
    let v_glyph = v.has(M::Glyph);
    let ranged = ctx.rules.units[atk_def].missile > 0;
    let evasion = stats::of(ctx, w, dst).evasion;
    if v_glyph {
        return;
    }
    if evasion > 0 && w.rng.below(100) < evasion as u32 {
        w.event(EV_MISS, 0, 0, v_pos);
        return;
    }
    if ranged
        && ctx.terrain.level_at(a_pos[0], a_pos[1]) < ctx.terrain.level_at(v_pos[0], v_pos[1])
        && w.rng.below(100) < ctx.rules.t.uphill_miss as u32
    {
        w.event(EV_MISS, 0, 0, v_pos);
        return;
    }
    let mut pct = ctx.rules.type_percent(ctx.rules.unit_attack[atk_def], ctx.rules.unit_armor[def_def]);
    // an illusion deals and takes its share of damage
    if let Some(m) = w.get(src).and_then(|u| u.mods.iter().find(|m| m.kind == M::Illusion)) {
        pct = pct * m.a / 100;
    }
    if let Some(m) = w.get(dst).and_then(|u| u.mods.iter().find(|m| m.kind == M::Illusion)) {
        pct = pct * m.b / 100;
    }
    let armor = stats::of(ctx, w, dst).armor;
    let keep = armor_keep(armor, ctx.rules.t.armor_factor);
    let after = raw as i64 * 1000 * pct as i64 / 100 * keep as i64 / 1000;
    let dealt = damage_milli(w, ctx, src, dst, after as i32, Dmg::Physical, true);
    if crit {
        w.event(EV_CRIT, raw, 0, v_pos);
    }
    let ls = stats::of(ctx, w, src).lifesteal;
    if ls > 0 && !ctx.rules.units[def_def].kind.is_building() {
        let gain = dealt * ls / 100;
        let max = stats::of(ctx, w, src).max_hp * 1000;
        if let Some(a) = w.get_mut(src) {
            a.hp = (a.hp + gain).min(max);
        }
    }
    crate::ability::after_attack(w, ctx, src, dst, raw);
}

/// Starts an attack swing at `t` if the attacker is ready.
pub fn swing(w: &mut World, ctx: &Ctx, src: Uid, t: Uid) {
    let s = stats::of(ctx, w, src);
    let Some(u) = w.get(src) else { return };
    let d = &ctx.rules.units[u.def as usize];
    let speed = 100 + s.attack_speed;
    let point = (d.attack_point * TPS * 100 / (100 * speed)).max(1) as u32;
    let tick = w.tick;
    let u = w.get_mut(src).expect("checked");
    u.act = Act::Swing { t, hit_at: tick + point };
    u.attack_ready = tick + s.attack_ticks() as u32;
    u.anim = world::ANIM_ATTACK;
    u.anim_at = tick;
}

/// The swing reaches its hit: melee damage now, or a missile.
pub fn release(w: &mut World, ctx: &Ctx, src: Uid, t: Uid) {
    let Some(u) = w.live(src) else { return };
    let d = &ctx.rules.units[u.def as usize];
    let s = stats::of(ctx, w, src);
    let speed = 100 + s.attack_speed;
    let back = (d.backswing * TPS * 100 / (100 * speed)).max(0) as u32;
    let until = w.tick + back;
    if let Some(u) = w.get_mut(src) {
        u.act = Act::Backswing { until };
    }
    let Some(u) = w.live(src) else { return };
    let d = &ctx.rules.units[u.def as usize];
    let (team, x, y) = (u.team, u.x, u.y);
    let missile = d.missile;
    let (mut raw, mut crit) = (roll(w, ctx, src), false);
    if let Some(bonus) = crate::ability::before_attack(w, ctx, src, t, raw) {
        raw = bonus;
        crit = true;
    }
    if missile > 0 {
        let model = crate::scene::missile_model(ctx, src, w);
        w.missiles.push(world::Missile {
            src,
            team,
            t,
            x,
            y,
            to: [0, 0],
            speed: missile * SUB / TPS,
            what: Hit::Attack { damage: raw, crit },
            model,
            age: 0,
            done: Vec::new(),
        });
    } else {
        attack_lands(w, ctx, src, t, raw, crit);
    }
}

/// Moves every missile; a homing one hits when it reaches its target.
pub fn missiles(w: &mut World, ctx: &Ctx) {
    let mut keep = Vec::new();
    let list = std::mem::take(&mut w.missiles);
    let mut arrived = Vec::new();
    for mut m in list {
        m.age += 1;
        let goal = if m.t != NO_UNIT {
            match w.get(m.t) {
                Some(t) if t.alive => [t.x, t.y],
                _ => {
                    // the target died or vanished: the missile is lost
                    continue;
                }
            }
        } else {
            m.to
        };
        let (dx, dy) = ((goal[0] - m.x) as i64, (goal[1] - m.y) as i64);
        let d = crate::fx::isqrt((dx * dx + dy * dy) as u64) as i64;
        if d <= m.speed as i64 {
            m.x = goal[0];
            m.y = goal[1];
            arrived.push(m);
        } else {
            m.x += (dx * m.speed as i64 / d) as i32;
            m.y += (dy * m.speed as i64 / d) as i32;
            if let Hit::Ability { .. } = m.what {
                // a line missile hits what it passes through
                crate::ability::missile_passes(w, ctx, &mut m);
            }
            keep.push(m);
        }
    }
    w.missiles = keep;
    for m in arrived {
        match m.what {
            Hit::Attack { damage, crit } => attack_lands(w, ctx, m.src, m.t, damage, crit),
            Hit::Ability { .. } => crate::ability::missile_arrives(w, ctx, &m),
            Hit::Item { .. } => crate::item::missile_arrives(w, ctx, &m),
        }
    }
}

/// Bounty and experience for every unit that died this tick.
pub fn deaths(w: &mut World, ctx: &Ctx) {
    let dead: Vec<Uid> = w.units.iter().filter(|u| u.alive && u.hp <= 0).map(|u| u.id).collect();
    for id in dead {
        if crate::ability::cheats_death(w, ctx, id) {
            continue;
        }
        kill(w, ctx, id);
    }
}

fn give_gold(w: &mut World, player: usize, gold: i32, reliable: bool, at: crate::rules::P) {
    w.players[player].gold += gold;
    if reliable {
        w.players[player].reliable += gold;
    }
    w.event(EV_GOLD, gold, player as i32, at);
}

/// Heroes of `team` alive within the experience radius of `at`.
fn heroes_near(w: &World, ctx: &Ctx, team: u8, at: crate::rules::P) -> Vec<Uid> {
    let r = ctx.rules.t.xp_radius;
    w.near(at, r, |u| u.team == team && u.hero.is_some())
}

pub fn give_xp(w: &mut World, ctx: &Ctx, id: Uid, xp: i32) {
    let levels = &ctx.rules.t.xp_levels;
    let max = ctx.rules.t.max_level;
    let mut gained = 0;
    if let Some(u) = w.get_mut(id) {
        if let Some(h) = u.hero.as_mut() {
            h.xp += xp;
            while h.level < max && h.xp >= levels[(h.level - 1) as usize] {
                h.level += 1;
                h.points += 1;
                gained += 1;
            }
        }
    }
    if gained > 0 {
        // levelling keeps the share of health and mana
        let s = stats::of(ctx, w, id);
        let at = w.get(id).map(|u| u.pos()).unwrap_or([0, 0]);
        w.event(world::EV_LEVEL, gained, 0, at);
        let _ = s;
    }
}

fn kill(w: &mut World, ctx: &Ctx, id: Uid) {
    let Some(v) = w.get(id) else { return };
    let killer = v.last_attacker;
    let (team, kind, def, at) = (v.team, ctx.rules.units[v.def as usize].kind, v.def as usize, v.pos());
    let level = v.level();
    let tier = v.tier;
    let illusion = v.has(M::Illusion);
    let tick = w.tick;
    let killer_info = w.get(killer).map(|k| (k.team, k.owner));
    {
        let v = w.get_mut(id).expect("checked");
        v.alive = false;
        v.anim = world::ANIM_DEATH;
        v.anim_at = tick;
        v.act = Act::None;
        v.path.clear();
        v.mods.clear();
    }
    if illusion {
        return;
    }
    let t = &ctx.rules.t;
    let d = &ctx.rules.units[def];
    let (k_team, k_owner) = killer_info.unwrap_or((NEUTRAL, -1));
    let denied = k_team == team;
    if kind == Kind::Sphere {
        w.winner = Some(other(team));
        w.event(EV_WIN, other(team) as i32, 0, at);
        return;
    }
    if kind == Kind::Hero {
        kill_hero(w, ctx, id, killer, level, at, denied);
        return;
    }
    // gold: the last hitter's player alone, and none for a deny
    if denied {
        if k_owner >= 0 {
            w.players[k_owner as usize].denies += 1;
            w.event(EV_DENY, 0, k_owner as i32, at);
        }
    } else if k_owner >= 0 {
        let gold = w.rng.between(d.bounty[0], d.bounty[1].max(d.bounty[0]));
        let p = k_owner as usize;
        if matches!(kind, Kind::Creep | Kind::Neutral | Kind::Boss) {
            w.players[p].last_hits += 1;
        }
        give_gold(w, p, gold, false, at);
    }
    if kind == Kind::Tower {
        // the side that takes a tower is paid by its tier; a deny pays the
        // other side half
        let gold = t.tower_team_gold.get(tier.max(0) as usize).copied().unwrap_or(0);
        let gold = if denied { gold / 2 } else { gold };
        let takers = other(team);
        for p in 0..world::PLAYERS {
            if w.players[p].team == takers && w.players[p].present {
                give_gold(w, p, gold, false, at);
            }
        }
        w.event(EV_TOWER, team as i32, 0, at);
    }
    if kind == Kind::Boss {
        crate::lanes::boss_died(w, ctx, killer, at);
    }
    let summon_share = if kind == Kind::Summon { 50 } else { 100 };
    if denied {
        // a deny leaves the enemies near it a small flat share: melee heroes
        // more than ranged (DotA before 6.79)
        let near = heroes_near(w, ctx, other(team), at);
        if !near.is_empty() && kind == Kind::Creep {
            let n = near.len() as i32;
            for h in near {
                let ranged = w.get(h).map(|u| ctx.rules.units[u.def as usize].missile > 0).unwrap_or(false);
                let xp = if ranged { t.deny_xp_ranged } else { t.deny_xp_melee };
                give_xp(w, ctx, h, xp / n);
            }
        }
        return;
    }
    let takers = if team == NEUTRAL { k_team } else { other(team) };
    if takers == NEUTRAL {
        return;
    }
    let near = heroes_near(w, ctx, takers, at);
    if !near.is_empty() && d.xp > 0 {
        let share = d.xp * summon_share / 100 / near.len() as i32;
        for h in near {
            give_xp(w, ctx, h, share);
        }
    }
}

fn kill_hero(w: &mut World, ctx: &Ctx, id: Uid, killer: Uid, level: i32, at: crate::rules::P, denied: bool) {
    let t = &ctx.rules.t;
    let v = w.get(id).expect("checked");
    let owner = v.owner;
    let team = v.team;
    let streak = v.hero.as_ref().map(|h| h.streak).unwrap_or(0);
    let respawn = w.tick + (level * t.respawn_per_level * TPS) as u32;
    crate::ability::hero_died_near(w, ctx, at, team);
    if let Some(h) = w.get_mut(id).and_then(|u| u.hero.as_mut()) {
        h.streak = 0;
        h.respawn_at = respawn;
        h.damaged_by.clear();
    }
    if owner >= 0 {
        let p = owner as usize;
        w.players[p].deaths += 1;
        // unreliable gold is lost on death
        let unreliable = (w.players[p].gold - w.players[p].reliable).max(0);
        let loss = (level * t.death_gold_per_level).min(unreliable);
        if !denied && loss > 0 {
            w.players[p].gold -= loss;
        }
    }
    if denied {
        return;
    }
    let (k_owner, k_hero) = w.get(killer).map(|k| (k.owner, k.hero.is_some() && !k.has(M::Illusion))).unwrap_or((-1, false));
    // the hero whose player made the kill (a summon or illusion counts for its owner)
    let killer_hero = if k_owner >= 0 { Some(w.players[k_owner as usize].hero) } else { None };
    let _ = k_hero;
    let mut gold = t.kill_gold_base + level * t.kill_gold_per_level;
    if streak >= 3 {
        let k = ((streak - 3) as usize).min(t.streak_gold.len().saturating_sub(1));
        gold += t.streak_gold.get(k).copied().unwrap_or(0);
    }
    if !w.first_blood && killer_hero.is_some() {
        w.first_blood = true;
        gold += t.first_blood_gold;
        w.event(EV_FIRST_BLOOD, k_owner as i32, owner as i32, at);
    }
    // the other enemy heroes near the death share the assist gold
    let near: Vec<Uid> = w
        .near(at, t.assist_radius, |u| u.team == other(team) && u.hero.is_some() && !u.has(M::Illusion))
        .into_iter()
        .filter(|h| Some(*h) != killer_hero)
        .collect();
    match killer_hero {
        Some(kh) => {
            let kp = k_owner as usize;
            w.players[kp].kills += 1;
            give_gold(w, kp, gold, true, at);
            if let Some(h) = w.get_mut(kh).and_then(|u| u.hero.as_mut()) {
                h.streak += 1;
            }
        }
        None => {
            // killed by a creep, a tower or a neutral: the heroes near share the bounty
            if !near.is_empty() {
                let each = gold / near.len() as i32;
                for h in &near {
                    if let Some(p) = w.get(*h).map(|u| u.owner).filter(|o| *o >= 0) {
                        give_gold(w, p as usize, each, true, at);
                    }
                }
            }
        }
    }
    w.event(EV_KILL, k_owner as i32, owner as i32, at);
    if !near.is_empty() && killer_hero.is_some() {
        let n = near.len().min(t.assist_gold.len());
        let [base, per] = t.assist_gold[n - 1];
        for h in &near {
            if let Some(p) = w.get(*h).map(|u| u.owner).filter(|o| *o >= 0) {
                w.players[p as usize].assists += 1;
                give_gold(w, p as usize, base + per * level, true, at);
            }
        }
    }
    // experience: the victim's level's worth shared among enemy heroes in
    // range, plus the bonus for how many share it; the killer is always
    // counted, even out of range
    let mut sharers = heroes_near(w, ctx, other(team), at);
    if let Some(kh) = killer_hero {
        if !sharers.contains(&kh) && w.live(kh).is_some() {
            sharers.push(kh);
        }
    }
    if !sharers.is_empty() {
        let n = sharers.len();
        let base = t.hero_kill_xp.get((level - 1).max(0) as usize).copied().unwrap_or(0);
        let [b0, b1] = t.hero_kill_bonus[n.min(t.hero_kill_bonus.len()) - 1];
        let each = base / n as i32 + b0 + b1 * level;
        for h in sharers {
            give_xp(w, ctx, h, each);
        }
    }
}

/// Distance between two units' edges (centre distance less both radii).
pub fn gap(ctx: &Ctx, a: &world::Unit, b: &world::Unit) -> i32 {
    let ra = ctx.rules.units[a.def as usize].radius;
    let rb = ctx.rules.units[b.def as usize].radius + ctx.rules.units[b.def as usize].footprint;
    let d = crate::fx::isqrt(dist2(a.pos(), b.pos()) as u64) as i32;
    (d - ra - rb).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_reduction_matches_hand_worked_values() {
        // 10 armor: 0.6 / 1.6 = 37.5 % reduced, 62.5 % kept
        assert_eq!(armor_keep(1000, 600), 625);
        // 0 armor keeps everything
        assert_eq!(armor_keep(0, 600), 1000);
        // 20 armor (a tower): 1.2 / 2.2 = 54.5 % reduced, 45.5 % kept
        assert_eq!(armor_keep(2000, 600), 455);
        // -5 armor: 2 − 0.94^5 = 2 − 0.7339 = 1.266 → 1267 with per-mille steps
        let k = armor_keep(-500, 600);
        assert!((1265..=1268).contains(&k), "{k}");
    }
}
