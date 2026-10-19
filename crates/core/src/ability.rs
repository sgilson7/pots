//! Hero abilities: learning them, casting them, and what each one does.
//!
//! Every ability shares one path: a player orders a cast, the hero walks
//! into range and turns to face, the cast point passes, then mana is spent,
//! the cooldown starts and the ability's rule runs. The numbers (damage,
//! durations, radii) are in `data/abilities.json` per level; the rule here
//! is chosen by the ability's `logic` name. Passives hook attacks
//! (`before_attack`, `after_attack`), being attacked, and casting.
//!
//! Each rule follows the DotA ability named in its data entry's `after`.
//! Where the game departs from DotA the comment says so.

use crate::combat::{self, Dmg};
use crate::fx;
use crate::rules::{Kind, Target, P, SUB, TPS};
use crate::stats;
use crate::world::{
    dist, dist2, other, within, Act, Ctx, Hit, Missile, Mod, Order, Uid, Unit, World, Zone, EV_CAST, M,
    MK_CAST, MK_FIRE, MK_LAND, MK_PROC, NEUTRAL, NO_UNIT,
};

/// `Tag` modifier codes the stats and combat read (round 7, two added
/// heroes): Berserker Rage's melee form (`a` = flat move speed), a miss
/// chance on the unit's own attacks (`a` = percent), Chakram's blade out,
/// and a primary-attribute cut (`b` = `TAG_ATTR_PCT` + attribute index,
/// `a` = percent).
pub const TAG_BERSERK: i32 = 3001;
pub const TAG_BLIND: i32 = 3002;
pub const TAG_BLADE_OUT: i32 = 3011;
pub const TAG_ATTR_PCT: i32 = 1100;

fn tagged(u: &Unit, code: i32) -> bool {
    u.mods.iter().any(|m| m.kind == M::Tag && m.b == code)
}

/// Whether Berserker Rage's melee form is on.
pub fn berserk(u: &Unit) -> bool {
    tagged(u, TAG_BERSERK)
}

/// Whether an ability's `alt` holds its slot now.
fn alt_holds(u: &Unit, when: &str) -> bool {
    match when {
        "berserk" => tagged(u, TAG_BERSERK),
        "blade_out" => tagged(u, TAG_BLADE_OUT),
        _ => false,
    }
}

fn tag(ability: usize) -> u16 {
    ability as u16 + 1
}

fn secs_x100(w: &World, hundredths: i32) -> u32 {
    w.tick + ((hundredths * TPS + 50) / 100).max(1) as u32
}

/// The ability index and level in a unit's slot.
/// The ability index and level in a unit's slot: the ability's `alt` while
/// its condition holds (Whirling Axes up close, Return Chakram).
pub fn slot_info(ctx: &Ctx, w: &World, id: Uid, slot: usize) -> Option<(usize, i32)> {
    let u = w.get(id)?;
    let mut k = *ctx.rules.unit_abilities[u.def as usize].get(slot)?;
    if let Some(alt) = ctx.rules.ability_alt[k] {
        if alt_holds(u, &ctx.rules.abilities[k].alt_when) {
            k = alt;
        }
    }
    Some((k, u.abil.get(slot)?.level))
}

/// Finds a slot of a unit holding ability logic `logic`, with its level.
fn has_logic(ctx: &Ctx, w: &World, id: Uid, logic: &str) -> Option<(usize, usize, i32)> {
    let u = w.get(id)?;
    for (slot, &k) in ctx.rules.unit_abilities[u.def as usize].iter().enumerate() {
        if ctx.rules.abilities[k].logic == logic {
            let lv = u.abil.get(slot).map(|a| a.level).unwrap_or(0);
            if lv > 0 {
                return Some((slot, k, lv));
            }
        }
    }
    None
}

/// Spends an ability point. Ultimates open at levels 6, 11 and 16; other
/// abilities may not pass half the hero's level rounded up; slot 4 is the
/// attribute bonus.
pub fn learn(w: &mut World, ctx: &Ctx, id: Uid, slot: usize) {
    let Some(u) = w.get(id) else { return };
    let Some(h) = u.hero.as_ref() else { return };
    if h.points <= 0 {
        return;
    }
    let level = h.level;
    if slot == 4 {
        if h.bonus >= ctx.rules.t.stat_bonus_max {
            return;
        }
        let before = stats::of(ctx, w, id);
        let u = w.get_mut(id).expect("checked");
        let h = u.hero.as_mut().expect("checked");
        h.bonus += 1;
        h.points -= 1;
        keep_share(w, ctx, id, before);
        return;
    }
    let Some(&k) = ctx.rules.unit_abilities[u.def as usize].get(slot) else {
        return;
    };
    let a = &ctx.rules.abilities[k];
    let have = u.abil[slot].level;
    if have >= a.max_level() {
        return;
    }
    let allowed = if a.ultimate {
        // levels 6, 11, 16
        if level >= 16 {
            3
        } else if level >= 11 {
            2
        } else if level >= 6 {
            1
        } else {
            0
        }
    } else {
        (level + 1) / 2
    };
    if have >= allowed {
        return;
    }
    let before = stats::of(ctx, w, id);
    let u = w.get_mut(id).expect("checked");
    u.abil[slot].level += 1;
    u.hero.as_mut().expect("checked").points -= 1;
    keep_share(w, ctx, id, before);
}

/// After a maximum changes, keeps the same share of health and mana.
fn keep_share(w: &mut World, ctx: &Ctx, id: Uid, before: stats::Stats) {
    let after = stats::of(ctx, w, id);
    if let Some(u) = w.get_mut(id) {
        if before.max_hp > 0 {
            u.hp = (u.hp as i64 * after.max_hp as i64 / before.max_hp as i64) as i32;
        }
        if before.max_mana > 0 {
            u.mana = (u.mana as i64 * after.max_mana as i64 / before.max_mana as i64) as i32;
        }
    }
}

/// Whether a cast could begin now (level, cooldown, mana, not silenced, a fit target).
pub fn can_start(w: &World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, _p: P) -> bool {
    let Some((k, level)) = slot_info(ctx, w, id, slot) else {
        return false;
    };
    let a = &ctx.rules.abilities[k];
    let Some(u) = w.live(id) else { return false };
    if level <= 0 || a.target == Target::Passive {
        return false;
    }
    if a.target == Target::Toggle {
        return !u.has(M::Silence) && !u.has(M::Hex);
    }
    // an ability with no cooldown (Return Chakram) ignores its slot's timer
    if (a.cooldown_at(level) > 0 && u.abil[slot].ready_at > w.tick) || u.mana < a.mana_at(level) * 1000 {
        return false;
    }
    if u.has(M::Silence) || u.has(M::Hex) || u.has(M::Stun) {
        return false;
    }
    if a.target == Target::Unit {
        let Some(v) = w.live(t) else { return false };
        let ok = match a.on.as_str() {
            "ally" => v.team == u.team,
            "any" => true,
            _ => v.team != u.team,
        };
        if !ok || !w.sees(ctx, u.team, v) {
            return false;
        }
        if v.team != u.team && v.has(M::SpellImmune) && !a.values.contains_key("pierces") {
            return false;
        }
        if ctx.rules.units[v.def as usize].kind.is_building() {
            return false;
        }
    }
    true
}

/// Walks into range, faces, then starts the cast.
pub fn approach(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, at: P) {
    let Some((k, level)) = slot_info(ctx, w, id, slot) else {
        idle(w, id);
        return;
    };
    if !can_start(w, ctx, id, slot, t, at) {
        idle(w, id);
        return;
    }
    let a = &ctx.rules.abilities[k];
    let here = w.get(id).expect("checked").pos();
    let goal = if a.target == Target::Unit {
        match w.live(t) {
            Some(v) => v.pos(),
            None => {
                idle(w, id);
                return;
            }
        }
    } else {
        at
    };
    let range = a.range_at(level);
    if !within(here, goal, range + 30) {
        crate::orders::walk(w, ctx, id, goal, range);
        return;
    }
    let facing = crate::orders::turn_to(ctx, w, id, fx::atan2(goal[1] - here[1], goal[0] - here[0]));
    if !facing && goal != here {
        return;
    }
    begin(w, ctx, id, slot, t, at);
}

fn idle(w: &mut World, id: Uid) {
    if let Some(u) = w.get_mut(id) {
        u.order = Order::Idle;
        u.path.clear();
    }
}

/// Starts the cast point; a toggle flips at once.
pub fn begin(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, at: P) {
    let Some((k, level)) = slot_info(ctx, w, id, slot) else {
        return;
    };
    let a = &ctx.rules.abilities[k];
    if a.target == Target::Toggle {
        let u = w.get_mut(id).expect("checked");
        u.abil[slot].on = !u.abil[slot].on;
        let on = u.abil[slot].on;
        toggled(w, ctx, id, k, level, on);
        if on {
            w.mark(MK_FIRE, k, id, NO_UNIT, [0, 0]);
        }
        return;
    }
    let tick = w.tick;
    let fire_at = secs_x100(w, a.cast_point);
    let u = w.get_mut(id).expect("checked");
    u.act = Act::Cast {
        slot: slot as u8,
        t,
        at,
        fire_at,
        item: false,
    };
    u.order = Order::Idle;
    u.path.clear();
    u.anim = crate::world::ANIM_CAST;
    u.anim_at = tick;
    w.mark(MK_CAST, k, id, t, at);
}

/// The cast point has passed: spend, start the cooldown, and act.
pub fn fire(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, at: P) {
    let Some((k, level)) = slot_info(ctx, w, id, slot) else {
        return;
    };
    if !can_start(w, ctx, id, slot, t, at) {
        return;
    }
    let a = &ctx.rules.abilities[k];
    let ready = secs_x100(w, a.cooldown_at(level));
    let cost = a.mana_at(level) * 1000;
    {
        let u = w.get_mut(id).expect("checked");
        u.mana -= cost;
        if a.cooldown_at(level) > 0 {
            u.abil[slot].ready_at = ready;
        }
    }
    let here = w.get(id).expect("checked").pos();
    w.event(EV_CAST, k as i32, 0, here);
    w.mark(MK_FIRE, k, id, t, at);
    effect(w, ctx, id, slot, k, level, t, at);
    on_cast(w, ctx, id);
    crate::item::charge_wands(w, ctx, id);
}

/// Passives that answer the caster's own casts (Fiery Soul, Aftershock).
fn on_cast(w: &mut World, ctx: &Ctx, id: Uid) {
    if let Some((_, k, lv)) = has_logic(ctx, w, id, "fiery_soul") {
        let a = &ctx.rules.abilities[k];
        let until = secs_x100(w, a.v("duration", lv));
        let stacks = w
            .get(id)
            .map(|u| {
                u.mods
                    .iter()
                    .filter(|m| m.tag == tag(k) && m.kind == M::AttackPct)
                    .count() as i32
            })
            .unwrap_or(0);
        let n = (stacks + 1).min(a.v("stacks", lv));
        w.mark(MK_PROC, k, id, id, [0, 0]);
        w.remove_mods(id, tag(k));
        for _ in 0..n {
            add(
                w,
                id,
                M::AttackPct,
                id,
                until,
                a.v("attack_speed", lv),
                tag(k),
                false,
            );
            add(w, id, M::MovePct, id, until, a.v("move_pct", lv), tag(k), false);
        }
    }
    if let Some((_, k, lv)) = has_logic(ctx, w, id, "static_field") {
        // Static Field: every cast takes a share of each nearby enemy's current health
        let a = &ctx.rules.abilities[k];
        let here = w.get(id).expect("checked").pos();
        w.mark(MK_PROC, k, id, id, here);
        let team = w.get(id).expect("checked").team;
        for v in enemies_near(w, ctx, team, here, a.v("radius", lv), false) {
            let hp = w.get(v).map(|u| u.hp).unwrap_or(0);
            combat::damage_milli(w, ctx, id, v, hp * a.v("pct", lv) / 100, Dmg::Magic, false);
        }
    }
    if let Some((_, k, lv)) = has_logic(ctx, w, id, "aftershock") {
        let a = &ctx.rules.abilities[k];
        let here = w.get(id).expect("checked").pos();
        w.mark(MK_PROC, k, id, id, here);
        let team = w.get(id).expect("checked").team;
        for v in enemies_near(w, ctx, team, here, a.v("radius", lv), false) {
            stun(w, ctx, id, v, a.v("stun", lv), tag(k));
            combat::damage(w, ctx, id, v, a.v("damage", lv), Dmg::Magic);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn add(w: &mut World, id: Uid, kind: M, src: Uid, until: u32, a: i32, t: u16, refresh: bool) {
    let m = Mod {
        kind,
        src,
        until,
        a,
        b: 0,
        tag: if refresh { t } else { 0 },
    };
    if refresh {
        w.add_mod(id, m);
    } else if let Some(u) = w.get_mut(id) {
        u.mods.push(Mod { tag: t, ..m });
    }
}

/// Stuns for hundredths of a second (the longer stun holds).
pub fn stun(w: &mut World, _ctx: &Ctx, src: Uid, id: Uid, hundredths: i32, t: u16) {
    let until = secs_x100(w, hundredths);
    let Some(u) = w.live(id) else { return };
    if u.has(M::SpellImmune) && u.team != w.get(src).map(|s| s.team).unwrap_or(NEUTRAL) {
        return;
    }
    let longer = u.mods.iter().any(|m| m.kind == M::Stun && m.until >= until);
    if !longer {
        if let Some(u) = w.get_mut(id) {
            u.mods.retain(|m| m.kind != M::Stun);
            u.mods.push(Mod {
                kind: M::Stun,
                src,
                until,
                a: 0,
                b: 0,
                tag: t,
            });
        }
    }
}

fn slow(w: &mut World, src: Uid, id: Uid, move_pct: i32, attack_pct: i32, hundredths: i32, t: u16) {
    let until = secs_x100(w, hundredths);
    if move_pct != 0 {
        w.add_mod(
            id,
            Mod {
                kind: M::MovePct,
                src,
                until,
                a: -move_pct,
                b: 0,
                tag: t,
            },
        );
    }
    if attack_pct != 0 {
        w.add_mod(
            id,
            Mod {
                kind: M::AttackPct,
                src,
                until,
                a: -attack_pct,
                b: 0,
                tag: t,
            },
        );
    }
}

/// Enemy units of `team` near a point, skipping buildings; spell-immune
/// units left out unless `pierce`.
pub fn enemies_near(w: &World, ctx: &Ctx, team: u8, at: P, r: i32, pierce: bool) -> Vec<Uid> {
    w.near(at, r, |u| {
        u.team != team
            && !ctx.rules.units[u.def as usize].kind.is_building()
            && ctx.rules.units[u.def as usize].kind != Kind::Ward
            && !u.has(M::Invulnerable)
            && (pierce || !u.has(M::SpellImmune))
    })
}

pub(crate) fn toward(from: P, to: P, len: i32) -> P {
    let (dx, dy) = ((to[0] - from[0]) as i64, (to[1] - from[1]) as i64);
    let d = fx::isqrt((dx * dx + dy * dy) as u64) as i64;
    if d == 0 {
        return [from[0] + len, from[1]];
    }
    [
        from[0] + (dx * len as i64 / d) as i32,
        from[1] + (dy * len as i64 / d) as i32,
    ]
}

/// Units whose centre is within `half_width` of the segment a–b.
fn in_line(w: &World, ctx: &Ctx, team: u8, a: P, b: P, half_width: i32, pierce: bool) -> Vec<Uid> {
    let r2 = (half_width as i64) * (half_width as i64);
    w.units
        .iter()
        .filter(|u| {
            u.alive
                && u.team != team
                && !ctx.rules.units[u.def as usize].kind.is_building()
                && !u.has(M::Invulnerable)
                && (pierce || !u.has(M::SpellImmune))
                && crate::map::dist2_to_segment(u.pos(), a, b) <= r2
        })
        .map(|u| u.id)
        .collect()
}

fn point_missile(w: &mut World, src: Uid, from: P, to: P, speed: i32, k: usize, level: i32, model: u16) {
    let team = w.get(src).map(|u| u.team).unwrap_or(NEUTRAL);
    w.missiles.push(Missile {
        src,
        team,
        t: NO_UNIT,
        x: from[0] * SUB,
        y: from[1] * SUB,
        to: [to[0] * SUB, to[1] * SUB],
        speed: speed * SUB / TPS,
        what: Hit::Ability {
            ability: k as u16,
            level,
        },
        model,
        age: 0,
        done: Vec::new(),
    });
}

fn unit_missile(w: &mut World, src: Uid, t: Uid, speed: i32, k: usize, level: i32, model: u16) {
    let Some(u) = w.get(src) else { return };
    let (team, x, y) = (u.team, u.x, u.y);
    w.missiles.push(Missile {
        src,
        team,
        t,
        x,
        y,
        to: [0, 0],
        speed: speed * SUB / TPS,
        what: Hit::Ability {
            ability: k as u16,
            level,
        },
        model,
        age: 0,
        done: Vec::new(),
    });
}

fn zone(w: &mut World, src: Uid, at: P, radius: i32, k: usize, level: i32, delay: u32, until: u32, a: i32) {
    let team = w.get(src).map(|u| u.team).unwrap_or(NEUTRAL);
    let start = w.tick;
    w.zones.push(Zone {
        src,
        team,
        at,
        radius,
        ability: k as u16,
        level,
        start,
        until,
        next: start + delay,
        a,
        to: at,
        done: Vec::new(),
    });
}

/// The model index for an ability's missile, by convention its icon name.
fn missile_of(ctx: &Ctx, k: usize) -> u16 {
    crate::scene::ability_missile(ctx, k)
}

#[allow(clippy::too_many_arguments)]
fn effect(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, k: usize, lv: i32, t: Uid, at: P) {
    let a = &ctx.rules.abilities[k];
    let here = w.get(id).expect("checked").pos();
    let team = w.get(id).expect("checked").team;
    let tg = tag(k);
    match a.logic.as_str() {
        // Storm Bolt: a slow missile that stuns and damages around its target.
        "bolt_stun" | "assassinate" | "laguna_missile" => {
            unit_missile(w, id, t, a.v("speed", lv), k, lv, missile_of(ctx, k));
        }
        "warcry" => {
            let until = secs_x100(w, a.v("duration", lv));
            for ally in w.near(here, a.v("radius", lv), |u| u.team == team && u.hero.is_some()) {
                add(w, ally, M::Armor, id, until, a.v("armor", lv) * 100, tg, true);
                add(w, ally, M::MovePct, id, until, a.v("move_pct", lv), tg, true);
            }
        }
        "gods_strength" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, id, M::DamagePct, id, until, a.v("damage_pct", lv), tg, true);
        }
        "crystal_nova" => {
            for v in enemies_near(w, ctx, team, at, a.v("radius", lv), false) {
                combat::damage(w, ctx, id, v, a.v("damage", lv), Dmg::Magic);
                slow(
                    w,
                    id,
                    v,
                    a.v("slow", lv),
                    a.v("attack_slow", lv),
                    a.v("duration", lv),
                    tg,
                );
            }
            zone(w, id, at, a.v("radius", lv), k, lv, 0, w.tick + 15, 1);
        }
        "frostbite" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, t, M::Root, id, until, 0, tg, true);
            add(w, t, M::Disarm, id, until, 0, tg, true);
            // the damage spread over the duration, in hundredths a second
            let per = a.v("damage", lv) * 10_000 / a.v("duration", lv).max(1);
            add(w, t, M::Regen, id, until, -per, tg, true);
        }
        "freezing_field" => {
            let until = secs_x100(w, a.v("duration", lv));
            let u = w.get_mut(id).expect("checked");
            u.act = Act::Channel {
                slot: slot as u8,
                t,
                at: here,
                until,
            };
            u.anim = crate::world::ANIM_CHANNEL;
            add(w, id, M::Armor, id, until, a.v("armor", lv) * 100, tg, true);
        }
        "dragon_slave" => {
            let end = toward(here, at, a.v("length", lv));
            point_missile(w, id, here, end, a.v("speed", lv), k, lv, missile_of(ctx, k));
        }
        "light_strike" => {
            let delay = secs_x100(w, a.v("delay", lv)) - w.tick;
            zone(w, id, at, a.v("radius", lv), k, lv, delay, w.tick + delay + 1, 0);
        }
        "laguna" | "finger" => {
            // a short delay before the strike lands, as in DotA
            let delay = secs_x100(w, a.v("delay", lv)) - w.tick;
            let p = w.live(t).map(|v| v.pos()).unwrap_or(at);
            zone(w, id, p, 0, k, lv, delay, w.tick + delay + 1, t as i32);
        }
        "silence_area" => {
            let until = secs_x100(w, a.v("duration", lv));
            for v in enemies_near(w, ctx, team, at, a.v("radius", lv), false) {
                add(w, v, M::Silence, id, until, 0, tg, true);
            }
        }
        "hook" => {
            let end = toward(here, at, a.range_at(lv));
            point_missile(w, id, here, end, a.v("speed", lv), k, lv, missile_of(ctx, k));
        }
        "dismember" | "mana_drain" => {
            let until = secs_x100(w, a.v("duration", lv));
            let u = w.get_mut(id).expect("checked");
            u.act = Act::Channel {
                slot: slot as u8,
                t,
                at,
                until,
            };
            u.anim = crate::world::ANIM_CHANNEL;
            if a.logic == "dismember" {
                stun(w, ctx, id, t, a.v("duration", lv), tg);
            }
        }
        "fissure" => {
            let end = toward(here, at, a.v("length", lv));
            for v in in_line(w, ctx, team, here, end, a.v("width", lv), false) {
                combat::damage(w, ctx, id, v, a.v("damage", lv), Dmg::Magic);
                stun(w, ctx, id, v, a.v("stun", lv), tg);
            }
            // the rocks: still, untargetable blocks along the line for a while
            if let Some(rock) = ctx.rules.unit("fissure_rock") {
                let n = a.v("length", lv) / 100;
                let until = secs_x100(w, a.v("duration", lv));
                for i in 1..=n {
                    let p = toward(here, end, i * 100);
                    let r = w.spawn(ctx, rock, NEUTRAL, p);
                    if let Some(u) = w.get_mut(r) {
                        u.expires = until;
                        u.mods.push(Mod {
                            kind: M::Invulnerable,
                            src: id,
                            until: 0,
                            a: 0,
                            b: 0,
                            tag: tg,
                        });
                    }
                }
            }
            zone(w, id, end, 0, k, lv, 0, w.tick + 20, 2);
        }
        "totem" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, id, M::Tag, id, until, a.v("damage_pct", lv), tg, true);
        }
        "echo_slam" => {
            let near = enemies_near(w, ctx, team, here, a.v("radius", lv), false);
            let n = near.len() as i32;
            for v in &near {
                combat::damage(w, ctx, id, *v, a.v("damage", lv), Dmg::Magic);
            }
            // each unit echoes to the others: echo damage times the others in range
            for v in &near {
                let others = n - 1;
                if others > 0 {
                    combat::damage(w, ctx, id, *v, a.v("echo", lv) * others, Dmg::Magic);
                }
            }
            zone(w, id, here, a.v("radius", lv), k, lv, 0, w.tick + 20, 1);
        }
        "blade_fury" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, id, M::SpellImmune, id, until, 0, tg, true);
            add(w, id, M::Disarm, id, until, 0, tg, true);
            zone(w, id, here, a.v("radius", lv), k, lv, 0, until, id as i32);
        }
        "healing_ward" => {
            if let Some(def) = ctx.rules.unit("healing_ward") {
                let ward = w.spawn(ctx, def, team, at);
                let owner = w.get(id).map(|u| u.owner).unwrap_or(-1);
                let until = secs_x100(w, a.v("duration", lv));
                if let Some(u) = w.get_mut(ward) {
                    u.expires = until;
                    u.owner = owner;
                    u.order = Order::Follow(id);
                }
                zone(w, id, at, a.v("radius", lv), k, lv, 0, until, ward as i32);
            }
        }
        "omnislash" => {
            let until = w.tick + (a.v("jumps", lv) * a.v("interval", lv) * TPS / 100) as u32 + 2;
            add(w, id, M::Invulnerable, id, until, 0, tg, true);
            add(w, id, M::Disarm, id, until, 0, tg, true);
            zone(w, id, here, a.v("radius", lv), k, lv, 0, until, t as i32);
        }
        "earth_spike" => {
            let end = toward(
                here,
                if t != NO_UNIT {
                    w.live(t).map(|v| v.pos()).unwrap_or(at)
                } else {
                    at
                },
                a.v("length", lv),
            );
            for v in in_line(w, ctx, team, here, end, a.v("width", lv), false) {
                combat::damage(w, ctx, id, v, a.v("damage", lv), Dmg::Magic);
                stun(w, ctx, id, v, a.v("stun", lv), tg);
            }
            zone(w, id, end, 0, k, lv, 0, w.tick + 20, 2);
        }
        "hex" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, t, M::Hex, id, until, 0, tg, true);
            if let Some(u) = w.get_mut(t) {
                u.act = Act::None;
            }
        }
        "shrapnel" => {
            let until = secs_x100(w, a.v("duration", lv));
            zone(
                w,
                id,
                at,
                a.v("radius", lv),
                k,
                lv,
                secs_x100(w, a.v("delay", lv)) - w.tick,
                until,
                0,
            );
        }
        "berserkers_call" => {
            let until = secs_x100(w, a.v("duration", lv));
            for v in enemies_near(w, ctx, team, here, a.v("radius", lv), true) {
                add(w, v, M::Taunt, id, until, 0, tg, true);
                if let Some(u) = w.get_mut(v) {
                    u.act = Act::None;
                    u.target = id;
                    u.aggro_until = until;
                }
            }
            add(w, id, M::Armor, id, until, a.v("armor", lv) * 100, tg, true);
        }
        "battle_hunger" => {
            let until = secs_x100(w, a.v("duration", lv));
            add(w, t, M::Regen, id, until, -a.v("dps", lv) * 100, tg, true);
            add(w, t, M::MovePct, id, until, -a.v("slow", lv), tg, true);
            add(w, id, M::MovePct, id, until, a.v("speed", lv), tg, true);
        }
        // Arc Lightning: a strike that leaps to the nearest enemy not yet struck
        "arc_lightning" => {
            let mut hit = vec![t];
            let mut at_p = w.live(t).map(|v| v.pos()).unwrap_or(at);
            w.mark(MK_LAND, k, id, t, at_p);
            combat::damage(w, ctx, id, t, a.v("damage", lv), Dmg::Magic);
            for _ in 1..a.v("bounces", lv).max(1) {
                let next = enemies_near(w, ctx, team, at_p, a.v("radius", lv), false)
                    .into_iter()
                    .filter(|v| !hit.contains(v))
                    .min_by_key(|v| w.get(*v).map(|u| dist2(u.pos(), at_p)).unwrap_or(i64::MAX));
                let Some(n) = next else { break };
                w.mark(MK_LAND, k, *hit.last().expect("one struck"), n, at_p);
                combat::damage(w, ctx, id, n, a.v("damage", lv), Dmg::Magic);
                at_p = w.get(n).map(|u| u.pos()).unwrap_or(at_p);
                hit.push(n);
            }
            zone(w, id, at_p, 80, k, lv, 0, w.tick + 8, 1);
        }
        // Lightning Bolt: a strike from above with a brief stun
        "lightning_bolt" => {
            let p = w.live(t).map(|v| v.pos()).unwrap_or(at);
            w.mark(MK_LAND, k, id, t, p);
            combat::damage(w, ctx, id, t, a.v("damage", lv), Dmg::Magic);
            stun(w, ctx, id, t, a.v("stun", lv), tg);
            zone(w, id, p, 120, k, lv, 0, w.tick + 10, 1);
        }
        // Thundergod's Wrath: every living enemy hero on the map is struck
        "thundergods_wrath" => {
            let foes: Vec<Uid> = w
                .units
                .iter()
                .filter(|u| u.alive && u.team == other(team) && u.hero.is_some() && !u.has(M::SpellImmune))
                .map(|u| u.id)
                .collect();
            for f in foes {
                w.mark(MK_LAND, k, id, f, here);
                combat::damage(w, ctx, id, f, a.v("damage", lv), Dmg::Magic);
                let p = w.get(f).map(|u| u.pos()).unwrap_or(here);
                zone(w, id, p, 120, k, lv, 0, w.tick + 10, 1);
            }
        }
        "culling" => {
            let Some(v) = w.live(t) else { return };
            let hp = v.hp / 1000;
            let threshold = a.v("threshold", lv);
            if hp <= threshold {
                // DotA's Culling Blade: kills outright below the threshold
                combat::damage_milli(w, ctx, id, t, v.hp + 1000, Dmg::Pure, false);
            } else {
                combat::damage(w, ctx, id, t, a.v("damage", lv), Dmg::Magic);
            }
        }
        // Whirling Axes, ranged: five axes fan out in a cone from a tip `tip`
        // wide to a base `base` wide, `range` ahead (DotA 6.72c; the
        // guide's measured triangle, notes/LITERATURE.md 7.1)
        "whirling_axes" => {
            let range = a.range_at(lv);
            let end = toward(here, at, range);
            let (dx, dy) = ((end[0] - here[0]) as i64, (end[1] - here[1]) as i64);
            let len = (fx::isqrt((dx * dx + dy * dy) as u64) as i64).max(1);
            let n = a.v("axes", lv).max(1);
            for i in 0..n {
                // -1..1 across the fan, in hundredths
                let f = if n == 1 {
                    0
                } else {
                    (i * 200 / (n - 1) - 100) as i64
                };
                let side = |w: i32| {
                    let off = w as i64 / 2 * f / 100;
                    [(-dy * off / len) as i32, (dx * off / len) as i32]
                };
                let (s0, s1) = (side(a.v("tip", lv)), side(a.v("base", lv)));
                let from = [here[0] + s0[0], here[1] + s0[1]];
                let to = [end[0] + s1[0], end[1] + s1[1]];
                point_missile(w, id, from, to, a.v("speed", lv), k, lv, missile_of(ctx, k));
            }
        }
        // Whirling Axes, melee: two axes spin out round the caster and back
        "whirling_axes_melee" => {
            let out = secs_x100(w, a.v("out", lv)) - w.tick;
            zone(w, id, here, a.v("radius", lv), k, lv, 0, w.tick + out * 2, 0);
        }
        // Battle Trance: every allied hero on the map, twice as long on the caster
        "battle_trance" => {
            let heroes: Vec<Uid> = w
                .units
                .iter()
                .filter(|u| u.alive && u.team == team && u.hero.is_some() && !u.has(M::Illusion))
                .map(|u| u.id)
                .collect();
            for h in heroes {
                let d = if h == id {
                    a.v("self_duration", lv)
                } else {
                    a.v("duration", lv)
                };
                let until = secs_x100(w, d);
                add(w, h, M::AttackPct, id, until, a.v("attack_speed", lv), tg, true);
            }
        }
        // Whirling Death: cuts trees; pure damage if any fell, magic if not;
        // enemy heroes lose a share of their primary attribute (the debuff
        // pierces spell immunity, the damage does not)
        "whirling_death" => {
            let r = a.v("radius", lv);
            let before = w.tree_version;
            w.cut_trees_in(ctx, here, r);
            let kind = if w.tree_version != before {
                Dmg::Pure
            } else {
                Dmg::Magic
            };
            let until = secs_x100(w, a.v("duration", lv));
            for v in enemies_near(w, ctx, team, here, r, true) {
                let Some(u) = w.get(v) else { continue };
                if !u.has(M::SpellImmune) {
                    combat::damage(w, ctx, id, v, a.v("damage", lv), kind);
                }
                let Some(u) = w.live(v) else { continue };
                let d = &ctx.rules.units[u.def as usize];
                let Some(attrs) = d.attrs.as_ref().filter(|_| u.hero.is_some()) else {
                    continue;
                };
                let idx = match attrs.primary {
                    crate::rules::Attr::Str => 0,
                    crate::rules::Attr::Agi => 1,
                    crate::rules::Attr::Int => 2,
                };
                let was = stats::of(ctx, w, v);
                w.add_mod(
                    v,
                    Mod {
                        kind: M::Tag,
                        src: id,
                        until,
                        a: a.v("attr_pct", lv),
                        b: TAG_ATTR_PCT + idx,
                        tag: tg,
                    },
                );
                keep_share(w, ctx, v, was);
            }
        }
        // Timber Chain: the chain flies toward the point and holds the first
        // tree within `latch` of its path; with no tree it flies its full
        // range and nothing follows
        "timber_chain" => {
            let end = toward(here, at, a.range_at(lv));
            let to = match first_tree(w, ctx, here, end, a.v("latch", lv)) {
                Some(i) => ctx.terrain.tree_centre(i),
                None => end,
            };
            point_missile(w, id, here, to, a.v("speed", lv), k, lv, missile_of(ctx, k));
        }
        // Chakram: the blade flies out, spins, and comes back (`pulse`); the
        // caster cannot attack while it is out, and Return Chakram holds the slot
        "chakram" => {
            let to = toward(here, at, dist(here, at).min(a.range_at(lv)));
            zone(w, id, here, a.v("radius", lv), k, lv, 0, u32::MAX, 0);
            if let Some(z) = w.zones.last_mut() {
                z.to = to;
            }
            add(w, id, M::Disarm, id, 0, 0, tg, true);
            w.add_mod(
                id,
                Mod {
                    kind: M::Tag,
                    src: id,
                    until: 0,
                    a: 0,
                    b: TAG_BLADE_OUT,
                    tag: tg,
                },
            );
        }
        // Return Chakram: the blade turns for home
        "return_chakram" => {
            for z in w.zones.iter_mut() {
                if z.src == id && ctx.rules.abilities[z.ability as usize].logic == "chakram" && z.a < 2 {
                    z.a = 2;
                    z.done.clear();
                }
            }
        }
        _ => {}
    }
}

/// The first standing tree (not the map's rim) within `r` of the segment
/// from `a` to `b`, nearest `a` first.
pub(crate) fn first_tree(w: &World, ctx: &Ctx, a: P, b: P, r: i32) -> Option<usize> {
    let t = ctx.terrain;
    let len = dist(a, b).max(1);
    let step = 16;
    let mut d = 0;
    while d <= len {
        let p = [a[0] + (b[0] - a[0]) * d / len, a[1] + (b[1] - a[1]) * d / len];
        let tc = t.tree_cell;
        let (x0, y0) = ((p[0] - r) / tc, (p[1] - r) / tc);
        let (x1, y1) = ((p[0] + r) / tc, (p[1] + r) / tc);
        let mut best: Option<(i64, usize)> = None;
        for cy in y0.max(2)..=y1.min(t.tw - 3) {
            for cx in x0.max(2)..=x1.min(t.tw - 3) {
                let i = (cy * t.tw + cx) as usize;
                if matches!(w.trees[i], crate::map::Tree::Standing) {
                    let d2 = dist2(t.tree_centre(i), p);
                    if d2 <= r as i64 * r as i64 && best.map(|(b, _)| d2 < b).unwrap_or(true) {
                        best = Some((d2, i));
                    }
                }
            }
        }
        if let Some((_, i)) = best {
            return Some(i);
        }
        d += step;
    }
    None
}

fn toggled(w: &mut World, ctx: &Ctx, id: Uid, k: usize, lv: i32, on: bool) {
    let a = &ctx.rules.abilities[k];
    if a.logic == "berserker_rage" {
        // Berserker Rage: melee form on or off, keeping the share of health;
        // Whirling Axes keeps a cooldown for each form, so the two timers
        // swap (the other form's in the slot's `count`)
        let before = stats::of(ctx, w, id);
        if on {
            add(w, id, M::Damage, id, 0, a.v("damage", lv), tag(k), true);
            add(w, id, M::Armor, id, 0, a.v("armor", lv), tag(k), true);
            w.add_mod(
                id,
                Mod {
                    kind: M::Tag,
                    src: id,
                    until: 0,
                    a: a.v("move", lv),
                    b: TAG_BERSERK,
                    tag: tag(k),
                },
            );
        } else {
            w.remove_mods(id, tag(k));
        }
        if let Some((slot, _, _)) = has_logic(ctx, w, id, "whirling_axes") {
            if let Some(u) = w.get_mut(id) {
                let s = &mut u.abil[slot];
                let other = s.count.max(0) as u32;
                s.count = s.ready_at as i32;
                s.ready_at = other;
            }
        }
        keep_share(w, ctx, id, before);
        return;
    }
    if a.logic == "rot" {
        if on {
            let here = w.get(id).expect("checked").pos();
            zone(w, id, here, a.v("radius", lv), k, lv, 0, u32::MAX, id as i32);
        } else {
            w.zones.retain(|z| !(z.src == id && z.ability as usize == k));
        }
    }
}

/// Zones act: delayed blasts land, fields pulse, spins and slashes go on.
pub fn zones(w: &mut World, ctx: &Ctx) {
    let list = std::mem::take(&mut w.zones);
    let mut keep = Vec::new();
    let tick = w.tick;
    for mut z in list {
        if z.until != u32::MAX && tick >= z.until {
            continue;
        }
        if tick >= z.next {
            pulse(w, ctx, &mut z);
        }
        if z.until == u32::MAX || tick + 1 < z.until {
            keep.push(z);
        }
    }
    // zones made during this tick's pulses join the list
    keep.append(&mut w.zones);
    w.zones = keep;
    auras(w, ctx);
    crate::item::auras(w, ctx);
}

fn pulse(w: &mut World, ctx: &Ctx, z: &mut Zone) {
    let k = z.ability as usize;
    let a = &ctx.rules.abilities[k];
    let lv = z.level;
    let tg = tag(k);
    let src_alive = w.live(z.src).is_some();
    match a.logic.as_str() {
        // two axes on a circle round the caster, out to `radius` (less an
        // axe's own reach) and back, a turn a second; each enemy hit once
        "whirling_axes_melee" => {
            let Some(u) = w.live(z.src) else {
                z.until = 0;
                return;
            };
            z.at = u.pos();
            let out = (z.until - z.start).max(2) / 2;
            let t = w.tick - z.start;
            let orbit = z.radius - a.v("axe_radius", lv);
            let r = if t < out {
                orbit * t as i32 / out as i32
            } else {
                orbit * (2 * out - t.min(2 * out)) as i32 / out as i32
            };
            let turn = (t as i32 * fx::TURN / TPS) & (fx::TURN - 1);
            let until = secs_x100(w, a.v("duration", lv));
            for half in [0, fx::TURN / 2] {
                let ang = turn + half;
                let axe = [
                    z.at[0] + fx::mul(fx::cos(ang), fx::int(r)) / fx::ONE,
                    z.at[1] + fx::mul(fx::sin(ang), fx::int(r)) / fx::ONE,
                ];
                for v in enemies_near(w, ctx, z.team, axe, a.v("axe_radius", lv), false) {
                    if z.done.contains(&v) {
                        continue;
                    }
                    z.done.push(v);
                    combat::damage(w, ctx, z.src, v, a.v("damage", lv), Dmg::Magic);
                    w.add_mod(
                        v,
                        Mod {
                            kind: M::Tag,
                            src: z.src,
                            until,
                            a: a.v("miss", lv),
                            b: TAG_BLIND,
                            tag: tg,
                        },
                    );
                }
            }
            z.next = w.tick + 1;
        }
        // Timber Chain's pull: the caster flies to the tree, cutting enemies
        // within `radius` of his path once each; the tree falls when he lands
        "timber_chain" => {
            let Some(u) = w.live(z.src) else {
                z.until = 0;
                return;
            };
            let here = u.pos();
            let step = (z.a / TPS).max(1);
            let left = dist(here, z.to);
            let next = if left <= step + 32 {
                z.to
            } else {
                toward(here, z.to, step)
            };
            for v in in_line(w, ctx, z.team, here, next, z.radius, false) {
                if !z.done.contains(&v) {
                    z.done.push(v);
                    combat::damage(w, ctx, z.src, v, a.v("damage", lv), Dmg::Pure);
                }
            }
            if let Some(u) = w.get_mut(z.src) {
                u.x = next[0] * SUB;
                u.y = next[1] * SUB;
                u.path.clear();
            }
            if next == z.to {
                if let Some(i) = ctx.terrain.tree_index(z.to[0], z.to[1]) {
                    w.cut_tree(ctx, i);
                }
                w.remove_mods(z.src, tg);
                z.until = 0;
                return;
            }
            z.next = w.tick + 1;
        }
        // Chakram: out to its point (phase 0), spinning there (1), back to
        // the caster (2). Pure damage to enemies it passes, once each way;
        // trees on its path and in its circle fall; spinning, every `tick`
        // it deals its damage and takes its mana, and slows enemies in it
        // by 5% for each 5% of health they are missing. It turns home when
        // the caster is out of mana or more than `leash` away.
        "chakram" => {
            let Some(u) = w.live(z.src) else {
                z.until = 0;
                return;
            };
            let caster = u.pos();
            let step = (a.v("speed", lv) / TPS).max(1);
            if z.a == 2 {
                z.to = caster;
            }
            if z.a != 1 {
                let left = dist(z.at, z.to);
                let next = if left <= step {
                    z.to
                } else {
                    toward(z.at, z.to, step)
                };
                for v in in_line(w, ctx, z.team, z.at, next, z.radius, false) {
                    if !z.done.contains(&v) {
                        z.done.push(v);
                        combat::damage(w, ctx, z.src, v, a.v("pass_damage", lv), Dmg::Pure);
                    }
                }
                z.at = next;
                w.cut_trees_in(ctx, z.at, z.radius);
                if next == z.to {
                    if z.a == 2 {
                        w.remove_mods(z.src, tg);
                        z.until = 0;
                        return;
                    }
                    z.a = 1;
                    z.done.clear();
                    // the first spin tick comes one tick-length after it stops
                    z.start = secs_x100(w, a.v("tick", lv));
                }
            } else {
                let every = secs_x100(w, a.v("tick", lv)) - w.tick;
                for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                    let s = stats::of(ctx, w, v);
                    let hp = w.get(v).map(|u| u.hp).unwrap_or(0);
                    let missing = 100 - (hp as i64 * 100 / (s.max_hp.max(1) as i64 * 1000)) as i32;
                    let pct = ((missing / 5) * 5 + 5).min(100);
                    slow(w, z.src, v, pct, 0, a.v("tick", lv), tg);
                }
                if w.tick >= z.start {
                    let cost = a.v("drain", lv) * 1000 * a.v("tick", lv) / 100;
                    let mana = w.get(z.src).map(|u| u.mana).unwrap_or(0);
                    if mana < cost || dist(caster, z.at) > a.v("leash", lv) {
                        z.a = 2;
                        z.done.clear();
                    } else {
                        if let Some(u) = w.get_mut(z.src) {
                            u.mana -= cost;
                        }
                        let dmg = a.v("dps", lv) * 1000 * a.v("tick", lv) / 100;
                        for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                            combat::damage_milli(w, ctx, z.src, v, dmg, Dmg::Pure, false);
                        }
                        z.start = w.tick + every;
                    }
                }
                if dist(caster, z.at) > a.v("leash", lv) {
                    z.a = 2;
                    z.done.clear();
                }
            }
            z.next = w.tick + 1;
        }
        "light_strike" => {
            w.mark(MK_LAND, k, z.src, NO_UNIT, z.at);
            for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                combat::damage(w, ctx, z.src, v, a.v("damage", lv), Dmg::Magic);
                stun(w, ctx, z.src, v, a.v("stun", lv), tg);
            }
            z.next = u32::MAX;
        }
        "laguna" | "finger" => {
            let t = z.a as u32;
            if w.live(t).is_some() {
                w.mark(MK_LAND, k, z.src, t, z.at);
                let kind = Dmg::Magic;
                combat::damage(w, ctx, z.src, t, a.v("damage", lv), kind);
            }
            z.next = u32::MAX;
        }
        "rot" => {
            let Some(u) = w.live(z.src) else {
                z.until = 0;
                return;
            };
            z.at = u.pos();
            let per_tick = a.v("dps", lv) * 1000 / TPS;
            for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                combat::damage_milli(w, ctx, z.src, v, per_tick, Dmg::Magic, false);
                slow(w, z.src, v, a.v("slow", lv), 0, 20, tg);
            }
            // Rot hurts Pudge too, as in DotA
            combat::damage_milli(w, ctx, z.src, z.src, per_tick, Dmg::Magic, false);
            z.next = w.tick + 1;
        }
        "blade_fury" => {
            let Some(u) = w.live(z.src) else {
                z.until = 0;
                return;
            };
            z.at = u.pos();
            let per_tick = a.v("dps", lv) * 1000 / TPS;
            for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                combat::damage_milli(w, ctx, z.src, v, per_tick, Dmg::Magic, false);
            }
            z.next = w.tick + 1;
        }
        "healing_ward" => {
            let ward = z.a as u32;
            let Some(wd) = w.live(ward) else {
                z.until = 0;
                return;
            };
            z.at = wd.pos();
            let allies = w.near(z.at, z.radius, |u| {
                u.team == z.team && !ctx.rules.units[u.def as usize].kind.is_building()
            });
            for id in allies {
                let max = stats::of(ctx, w, id).max_hp;
                // percent of maximum health a second
                let per_tick = max * a.v("heal_pct", lv) * 10 / TPS;
                if let Some(u) = w.get_mut(id) {
                    u.hp = (u.hp + per_tick).min(max * 1000);
                }
            }
            z.next = w.tick + 1;
        }
        "omnislash" => {
            if !src_alive {
                z.until = 0;
                return;
            }
            let interval = (a.v("interval", lv) * TPS / 100).max(1) as u32;
            let first = z.a as u32;
            let here = w.get(z.src).expect("alive").pos();
            let target = if w.tick == z.start && w.live(first).is_some() {
                Some(first)
            } else {
                let list = enemies_near(w, ctx, z.team, here, z.radius, true);
                if list.is_empty() {
                    None
                } else {
                    let i = w.rng.below(list.len() as u32) as usize;
                    Some(list[i])
                }
            };
            match target {
                Some(t) => {
                    let p = w.get(t).expect("listed").pos();
                    w.mark(MK_LAND, k, z.src, t, p);
                    let now = w.tick;
                    if let Some(u) = w.get_mut(z.src) {
                        u.x = p[0] * SUB;
                        u.y = p[1] * SUB + 40 * SUB;
                        u.anim = crate::world::ANIM_ATTACK;
                        u.anim_at = now;
                    }
                    let dmg = w.rng.between(a.v("min", lv), a.v("max", lv));
                    combat::damage(w, ctx, z.src, t, dmg, Dmg::Physical);
                    z.next = w.tick + interval;
                }
                None => {
                    // nothing left to strike: the slash ends early
                    z.until = 0;
                    if let Some(u) = w.get_mut(z.src) {
                        u.mods.retain(|m| m.tag != tg);
                    }
                }
            }
        }
        "shrapnel" => {
            let per_tick = a.v("dps", lv) * 1000 / TPS;
            for v in enemies_near(w, ctx, z.team, z.at, z.radius, false) {
                combat::damage_milli(w, ctx, z.src, v, per_tick, Dmg::Magic, false);
                slow(w, z.src, v, a.v("slow", lv), 0, 20, tg);
            }
            z.next = w.tick + 1;
        }
        // shown only: the blast already happened when it was cast
        _ => z.next = u32::MAX,
    }
}

/// Channels: what goes on each tick while the hero stands still.
pub fn channel_tick(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, _at: P) {
    let Some((k, lv)) = slot_info(ctx, w, id, slot) else {
        return;
    };
    let a = &ctx.rules.abilities[k];
    let team = w.get(id).expect("checked").team;
    let here = w.get(id).expect("checked").pos();
    match a.logic.as_str() {
        "dismember" => {
            if w.live(t).is_none() || !within(here, w.get(t).expect("checked").pos(), a.range_at(lv) + 300) {
                end_channel(w, id);
                return;
            }
            // strength adds to the damage, as in DotA's later Dismember; here the base only
            let per_tick = a.v("dps", lv) * 1000 / TPS;
            combat::damage_milli(w, ctx, id, t, per_tick, Dmg::Magic, false);
            combat::heal(w, ctx, id, 0);
        }
        "mana_drain" => {
            let Some(v) = w.live(t) else {
                end_channel(w, id);
                return;
            };
            if !within(here, v.pos(), a.v("break", lv)) || v.mana <= 0 {
                end_channel(w, id);
                return;
            }
            let per_tick = (a.v("drain", lv) * 1000 / TPS).min(v.mana);
            let max = stats::of(ctx, w, id).max_mana * 1000;
            if let Some(v) = w.get_mut(t) {
                v.mana -= per_tick;
            }
            if let Some(u) = w.get_mut(id) {
                u.mana = (u.mana + per_tick).min(max);
            }
        }
        "freezing_field" => {
            let every = (a.v("interval", lv) * TPS / 100).max(1) as u32;
            if w.tick.is_multiple_of(every) {
                // an explosion at a seeded point in the ring around the hero
                let ang = w.rng.below(fx::TURN as u32) as i32;
                let lo = a.v("inner", lv);
                let hi = a.v("outer", lv);
                let r = w.rng.between(lo, hi);
                let p = [
                    here[0] + fx::cos(ang) * r / fx::ONE,
                    here[1] + fx::sin(ang) * r / fx::ONE,
                ];
                w.mark(MK_LAND, k, id, NO_UNIT, p);
                for v in enemies_near(w, ctx, team, p, a.v("blast", lv), false) {
                    combat::damage(w, ctx, id, v, a.v("damage", lv), Dmg::Magic);
                }
                zone(w, id, p, a.v("blast", lv), k, lv, 0, w.tick + 6, 1);
            }
            for v in enemies_near(w, ctx, team, here, a.v("outer", lv), false) {
                slow(w, id, v, a.v("slow", lv), a.v("attack_slow", lv), 20, tag(k));
            }
        }
        _ => {}
    }
}

fn end_channel(w: &mut World, id: Uid) {
    if let Some(u) = w.get_mut(id) {
        u.act = Act::None;
        u.anim = crate::world::ANIM_STAND;
    }
}

pub fn channel_done(w: &mut World, _ctx: &Ctx, id: Uid, _slot: usize) {
    end_channel(w, id);
}

pub fn channel_broken(w: &mut World, ctx: &Ctx, id: Uid, slot: usize) {
    if let Some((k, _)) = slot_info(ctx, w, id, slot) {
        if ctx.rules.abilities[k].logic == "dismember" {
            // the victim's stun ends with the channel
            let tg = tag(k);
            for u in w.units.iter_mut() {
                u.mods
                    .retain(|m| !(m.tag == tg && m.src == id && m.kind == M::Stun));
            }
        }
    }
    end_channel(w, id);
}

/// A line missile passing through units (Dragon Slave, the hook).
pub fn missile_passes(w: &mut World, ctx: &Ctx, m: &mut Missile) {
    let Hit::Ability { ability, level } = m.what else {
        return;
    };
    let k = ability as usize;
    let a = &ctx.rules.abilities[k];
    let at = [m.x / SUB, m.y / SUB];
    match a.logic.as_str() {
        // Whirling Axes, ranged: each enemy is hit once a cast, by the first
        // axe to reach it (its slow from this cast marks it)
        "whirling_axes" => {
            for v in enemies_near(w, ctx, m.team, at, a.v("radius", level), false) {
                let hit = w
                    .get(v)
                    .map(|u| u.mods.iter().any(|x| x.tag == tag(k) && x.src == m.src))
                    .unwrap_or(true);
                if !hit && !m.done.contains(&v) {
                    m.done.push(v);
                    combat::damage(w, ctx, m.src, v, a.v("damage", level), Dmg::Magic);
                    slow(w, m.src, v, a.v("slow", level), 0, a.v("duration", level), tag(k));
                }
            }
        }
        "dragon_slave" => {
            let r = a.v("width", level);
            for v in enemies_near(w, ctx, m.team, at, r, false) {
                if !m.done.contains(&v) {
                    m.done.push(v);
                    combat::damage(w, ctx, m.src, v, a.v("damage", level), Dmg::Magic);
                }
            }
        }
        "hook" => {
            let r = a.v("width", level);
            let hit = w
                .near(at, r, |u| {
                    u.id != m.src
                        && !ctx.rules.units[u.def as usize].kind.is_building()
                        && !u.has(M::Invulnerable)
                })
                .into_iter()
                .next();
            if let Some(v) = hit {
                hooked(w, ctx, m.src, v, k, level);
                // the hook stops at the first unit
                m.to = [m.x, m.y];
                m.speed = i32::MAX / 4;
                m.done.push(v);
            }
        }
        _ => {}
    }
}

fn hooked(w: &mut World, ctx: &Ctx, src: Uid, v: Uid, k: usize, lv: i32) {
    let a = &ctx.rules.abilities[k];
    let Some(s) = w.get(src) else { return };
    let (team, sp) = (s.team, s.pos());
    let enemy = w.get(v).map(|u| u.team != team).unwrap_or(false);
    w.mark(MK_LAND, k, src, v, sp);
    if enemy {
        combat::damage(w, ctx, src, v, a.v("damage", lv), Dmg::Pure);
    }
    // pulled to stand in front of the hooker
    let dest = toward(sp, w.get(v).map(|u| u.pos()).unwrap_or(sp), 120);
    stun(w, ctx, src, v, 30, tag(k));
    if let Some(u) = w.get_mut(v) {
        if u.alive {
            u.x = dest[0] * SUB;
            u.y = dest[1] * SUB;
            u.path.clear();
            u.act = Act::None;
        }
    }
}

pub fn missile_arrives(w: &mut World, ctx: &Ctx, m: &Missile) {
    let Hit::Ability { ability, level } = m.what else {
        return;
    };
    let k = ability as usize;
    let a = &ctx.rules.abilities[k];
    w.mark(MK_LAND, k, m.src, m.t, [m.x / SUB, m.y / SUB]);
    match a.logic.as_str() {
        "bolt_stun" => {
            let at = w.get(m.t).map(|u| u.pos()).unwrap_or([m.x / SUB, m.y / SUB]);
            for v in enemies_near(w, ctx, m.team, at, a.v("radius", level), false) {
                combat::damage(w, ctx, m.src, v, a.v("damage", level), Dmg::Magic);
                stun(w, ctx, m.src, v, a.v("stun", level), tag(k));
            }
        }
        "assassinate" | "laguna_missile" => {
            combat::damage(w, ctx, m.src, m.t, a.v("damage", level), Dmg::Magic);
            stun(w, ctx, m.src, m.t, a.v("stun", level), tag(k));
        }
        // Timber Chain: holding a tree, the chain pulls the caster to it,
        // unless he is stunned as it holds (DotA 6.73's rule)
        "timber_chain" => {
            let to = [m.to[0] / SUB, m.to[1] / SUB];
            let held = ctx.terrain.tree_index(to[0], to[1]).filter(|&i| {
                matches!(w.trees[i], crate::map::Tree::Standing) && ctx.terrain.tree_centre(i) == to
            });
            let Some(caster) = w.live(m.src) else { return };
            if held.is_none() || caster.has(M::Stun) {
                return;
            }
            let at = caster.pos();
            zone(
                w,
                m.src,
                at,
                a.v("radius", level),
                k,
                level,
                0,
                u32::MAX,
                a.v("speed", level),
            );
            if let Some(z) = w.zones.last_mut() {
                z.to = to;
            }
            // pulled: no walking or attacking until he lands
            add(w, m.src, M::Root, m.src, 0, 0, tag(k), true);
            add(w, m.src, M::Disarm, m.src, 0, 0, tag(k), true);
            if let Some(u) = w.get_mut(m.src) {
                u.path.clear();
                u.act = Act::None;
            }
        }
        _ => {}
    }
}

/// Attack modifiers before the roll lands: Blade Dance's crit, Enchant
/// Totem. Returns the new damage when it changed (shown as a crit).
pub fn before_attack(w: &mut World, ctx: &Ctx, src: Uid, _t: Uid, raw: i32) -> Option<i32> {
    let mut out = crate::item::before_attack(w, ctx, src, raw);
    if let Some((_, k, lv)) = has_logic(ctx, w, src, "crit") {
        let a = &ctx.rules.abilities[k];
        if combat::prd(w, src, 0, a.v("chance", lv)) {
            w.mark(MK_PROC, k, src, NO_UNIT, [0, 0]);
            let v = raw * a.v("multiplier", lv) / 100;
            out = Some(out.map(|o: i32| o.max(v)).unwrap_or(v));
        }
    }
    let totem = w.get(src).and_then(|u| {
        u.mods
            .iter()
            .find(|m| m.kind == M::Tag && has_tag_logic(ctx, m.tag, "totem"))
            .copied()
    });
    if let Some(m) = totem {
        let base = out.unwrap_or(raw);
        out = Some(base * (100 + m.a) / 100);
        w.mark(MK_PROC, m.tag as usize - 1, src, NO_UNIT, [0, 0]);
        if let Some(u) = w.get_mut(src) {
            u.mods.retain(|x| !(x.kind == M::Tag && x.tag == m.tag));
        }
    }
    out
}

fn has_tag_logic(ctx: &Ctx, t: u16, logic: &str) -> bool {
    t >= 1
        && (t as usize - 1) < ctx.rules.abilities.len()
        && ctx.rules.abilities[t as usize - 1].logic == logic
}

/// On-hit effects after an attack lands: cleave, frost arrows, headshot,
/// counter helix, battle hunger ending.
pub fn after_attack(w: &mut World, ctx: &Ctx, src: Uid, dst: Uid, raw: i32) {
    crate::item::after_attack(w, ctx, src, dst, raw);
    let Some(s) = w.get(src) else { return };
    let team = s.team;
    let dst_pos = w.get(dst).map(|u| u.pos()).unwrap_or([0, 0]);
    if let Some((_, k, lv)) = has_logic(ctx, w, src, "cleave") {
        let a = &ctx.rules.abilities[k];
        let pct = a.v("pct", lv);
        w.mark(MK_PROC, k, src, dst, dst_pos);
        for v in enemies_near(w, ctx, team, dst_pos, a.v("radius", lv), true) {
            if v != dst {
                combat::damage(w, ctx, src, v, raw * pct / 100, Dmg::Physical);
            }
        }
    }
    if let Some((slot, k, lv)) = has_logic(ctx, w, src, "frost_arrows") {
        let a = &ctx.rules.abilities[k];
        let on = w.get(src).map(|u| u.abil[slot].on).unwrap_or(false);
        let cost = a.mana_at(lv) * 1000;
        let enough = w.get(src).map(|u| u.mana >= cost).unwrap_or(false);
        if on && enough && w.get(dst).map(|v| v.team != team).unwrap_or(false) {
            if let Some(u) = w.get_mut(src) {
                u.mana -= cost;
            }
            w.mark(MK_PROC, k, src, dst, dst_pos);
            slow(w, src, dst, a.v("slow", lv), 0, a.v("duration", lv), tag(k));
        }
    }
    if let Some((_, k, lv)) = has_logic(ctx, w, src, "headshot") {
        let a = &ctx.rules.abilities[k];
        if combat::prd(w, src, 1, a.v("chance", lv)) {
            w.mark(MK_PROC, k, src, dst, dst_pos);
            combat::damage(w, ctx, src, dst, a.v("damage", lv), Dmg::Physical);
            stun(w, ctx, src, dst, a.v("stun", lv), tag(k));
        }
    }
    // Fervor: each attack on the same target adds to the attack speed, up to
    // `stacks`; a new target starts the count again at one (the slot's
    // `count` holds the stacks, its unused `ready_at` the last target)
    if let Some((slot, k, lv)) = has_logic(ctx, w, src, "fervor") {
        let a = &ctx.rules.abilities[k];
        if w.get(dst).map(|v| v.team != team).unwrap_or(false) {
            let stacks = if let Some(u) = w.get_mut(src) {
                let s = &mut u.abil[slot];
                s.count = if s.ready_at == dst {
                    (s.count + 1).min(a.v("stacks", lv))
                } else {
                    1
                };
                s.ready_at = dst;
                s.count
            } else {
                0
            };
            add(
                w,
                src,
                M::AttackPct,
                src,
                0,
                stacks * a.v("attack_speed", lv),
                tag(k),
                true,
            );
        }
    }
    // Berserker Rage's bash, in melee form only: plain random as in DotA
    // (pseudo-random came with the Dota 2 port), and its stun pierces
    // spell immunity
    if let Some((_, k, lv)) = has_logic(ctx, w, src, "berserker_rage") {
        let a = &ctx.rules.abilities[k];
        let melee = w.get(src).map(berserk).unwrap_or(false);
        let foe = w
            .get(dst)
            .map(|v| v.team != team && !ctx.rules.units[v.def as usize].kind.is_building())
            .unwrap_or(false);
        if melee && foe && w.rng.below(100) < a.v("bash_chance", lv) as u32 {
            w.mark(MK_PROC, k, src, dst, dst_pos);
            combat::damage(w, ctx, src, dst, a.v("bash_damage", lv), Dmg::Magic);
            let until = secs_x100(w, a.v("bash_stun", lv));
            if let Some(v) = w.get_mut(dst) {
                if v.alive && !v.mods.iter().any(|m| m.kind == M::Stun && m.until >= until) {
                    v.mods.retain(|m| m.kind != M::Stun);
                    v.mods.push(Mod {
                        kind: M::Stun,
                        src,
                        until,
                        a: 0,
                        b: 0,
                        tag: tag(k),
                    });
                }
            }
        }
    }
    if let Some((_, k, lv)) = has_logic(ctx, w, src, "bash") {
        let a = &ctx.rules.abilities[k];
        if combat::prd(w, src, 1, a.v("chance", lv)) {
            w.mark(MK_PROC, k, src, dst, dst_pos);
            combat::damage(w, ctx, src, dst, a.v("damage", lv), Dmg::Magic);
            stun(w, ctx, src, dst, a.v("stun", lv), tag(k));
        }
    }
    // Counter Helix answers being attacked
    if let Some((_, k, lv)) = has_logic(ctx, w, dst, "counter_helix") {
        let a = &ctx.rules.abilities[k];
        if combat::prd(w, dst, 2, a.v("chance", lv)) {
            let dteam = w.get(dst).map(|u| u.team).unwrap_or(NEUTRAL);
            for v in enemies_near(w, ctx, dteam, dst_pos, a.v("radius", lv), true) {
                combat::damage(w, ctx, dst, v, a.v("damage", lv), Dmg::Pure);
            }
            if let Some(u) = w.get_mut(dst) {
                u.anim = crate::world::ANIM_CAST;
            }
        }
    }
}

/// An attack on `dst` begins (Warcraft III's "unit is attacked"): Reactive
/// Armor gains a stack, each with its own timer, the oldest giving way at
/// the most.
pub fn attacked(w: &mut World, ctx: &Ctx, dst: Uid, _src: Uid) {
    let Some((_, k, lv)) = has_logic(ctx, w, dst, "reactive_armor") else {
        return;
    };
    let a = &ctx.rules.abilities[k];
    let tg = tag(k);
    let Some(u) = w.get_mut(dst) else { return };
    let held: Vec<u32> = u
        .mods
        .iter()
        .filter(|m| m.tag == tg && m.kind == M::Armor)
        .map(|m| m.until)
        .collect();
    if held.len() as i32 >= a.v("stacks", lv) {
        if let Some(&oldest) = held.iter().min() {
            for kind in [M::Armor, M::Regen] {
                if let Some(i) = u
                    .mods
                    .iter()
                    .position(|m| m.tag == tg && m.kind == kind && m.until == oldest)
                {
                    u.mods.remove(i);
                }
            }
        }
    }
    let until = secs_x100(w, a.v("duration", lv));
    add(w, dst, M::Armor, dst, until, a.v("armor", lv), tg, false);
    add(w, dst, M::Regen, dst, until, a.v("regen", lv), tg, false);
}

/// Auras, renewed every tick for the units in reach: Brilliance Aura,
/// Trueshot Aura, Marksmanship, Flesh Heap, Take Aim.
fn auras(w: &mut World, ctx: &Ctx) {
    let until = w.tick + 2;
    let heroes: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| u.alive && u.hero.is_some())
        .map(|u| u.id)
        .collect();
    for id in heroes {
        let team = w.get(id).expect("listed").team;
        let here = w.get(id).expect("listed").pos();
        let def = w.get(id).expect("listed").def as usize;
        let illusion = w.get(id).map(|u| u.has(M::Illusion)).unwrap_or(false);
        for (slot, &k) in ctx.rules.unit_abilities[def].iter().enumerate() {
            let lv = w.get(id).expect("listed").abil[slot].level;
            if lv <= 0 {
                continue;
            }
            let a = &ctx.rules.abilities[k];
            let tg = tag(k);
            let reach = a.v("radius", lv);
            match a.logic.as_str() {
                "brilliance" if !illusion => {
                    for ally in w.near(here, reach, |u| u.team == team && u.hero.is_some()) {
                        w.add_mod(
                            ally,
                            Mod {
                                kind: M::ManaRegen,
                                src: id,
                                until,
                                a: a.v("regen", lv),
                                b: 0,
                                tag: tg,
                            },
                        );
                    }
                }
                "trueshot" if !illusion => {
                    // 6.72: each ranged ally gains a share of its own base damage
                    let allies = w.near(here, reach, |u| {
                        u.team == team
                            && (ctx.rules.units[u.def as usize].missile > 0 && !berserk(u))
                            && !ctx.rules.units[u.def as usize].kind.is_building()
                    });
                    for ally in allies {
                        let base = stats::of(ctx, w, ally).damage;
                        let bonus = (base[0] + base[1]) / 2 * a.v("pct", lv) / 100;
                        w.add_mod(
                            ally,
                            Mod {
                                kind: M::Damage,
                                src: id,
                                until,
                                a: bonus,
                                b: 0,
                                tag: tg,
                            },
                        );
                    }
                }
                "marksmanship" => {
                    w.add_mod(
                        id,
                        Mod {
                            kind: M::Tag,
                            src: id,
                            until,
                            a: a.v("agi", lv),
                            b: 1001,
                            tag: tg,
                        },
                    );
                }
                "take_aim" => {
                    w.add_mod(
                        id,
                        Mod {
                            kind: M::Tag,
                            src: id,
                            until,
                            a: a.v("range", lv),
                            b: 2002,
                            tag: tg,
                        },
                    );
                }
                "flesh_heap" => {
                    w.add_mod(
                        id,
                        Mod {
                            kind: M::MagicResist,
                            src: id,
                            until,
                            a: a.v("resist", lv),
                            b: 0,
                            tag: tg,
                        },
                    );
                    let stacks = w.get(id).expect("listed").abil[slot].count;
                    if stacks > 0 {
                        w.add_mod(
                            id,
                            Mod {
                                kind: M::Tag,
                                src: id,
                                until,
                                a: stacks * a.v("str", lv) / 100,
                                b: 1000,
                                tag: tg + 4000,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
    }
    let _ = other(0);
}

/// Flesh Heap: a stack for each enemy hero that dies near the hero.
pub fn hero_died_near(w: &mut World, ctx: &Ctx, at: P, team: u8) {
    let heroes: Vec<Uid> = w.near(at, 450, |u| u.team != team && u.hero.is_some());
    for id in heroes {
        if let Some((slot, _, _)) = has_logic(ctx, w, id, "flesh_heap") {
            if let Some(u) = w.get_mut(id) {
                u.abil[slot].count += 1;
            }
        }
    }
}

/// Whether a dying unit is saved (the shield the boss drops).
pub fn cheats_death(w: &mut World, ctx: &Ctx, id: Uid) -> bool {
    crate::item::aegis_saves(w, ctx, id)
}

/// Makes `n` illusions of a hero for `secs`, dealing `dealt` percent and
/// taking `taken` percent of damage.
pub fn illusions(w: &mut World, ctx: &Ctx, hero: Uid, n: i32, secs: i32, dealt: i32, taken: i32) {
    let Some(h) = w.get(hero) else { return };
    let (def, team, owner, here) = (h.def as usize, h.team, h.owner, h.pos());
    let abil = h.abil.clone();
    let items = h.items;
    let hero_state = h.hero.clone();
    let hp = h.hp;
    let mana = h.mana;
    let until = w.tick + (secs * TPS) as u32;
    for i in 0..n {
        let at = [here[0] + 80 * (2 * i - 1), here[1]];
        let id = w.spawn(ctx, def, team, at);
        if let Some(u) = w.get_mut(id) {
            u.owner = owner;
            u.abil = abil.clone();
            u.items = items;
            u.hero = hero_state.clone();
            u.hp = hp;
            u.mana = mana;
            u.expires = until;
            u.mods.push(Mod {
                kind: M::Illusion,
                src: hero,
                until: 0,
                a: dealt,
                b: taken,
                tag: 0,
            });
            u.order = Order::AttackMove(here);
        }
    }
    let _ = dist2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_toward_another_lies_at_the_given_length() {
        assert_eq!(toward([0, 0], [300, 400], 50), [30, 40]);
        assert_eq!(toward([10, 10], [10, 10], 5), [15, 10]);
    }
}
