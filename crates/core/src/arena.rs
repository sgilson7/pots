//! The arena (round 8, after SMITE 2's Arena): a round map with a base at
//! each end, no towers and no barracks, two lanes round the middle that
//! the waves take turn about, and a small jungle on each side whose camps
//! give buffs. Each side starts with tickets and loses them when its heroes
//! die, when a hero kills one of its creeps, and when an enemy creep walks
//! into its portal (its Sphere, which nothing can hurt here). A side with
//! no tickets left loses. The numbers are `data/rules.json`'s `arena`, the
//! map is `data/map.json`'s `arena`; a normal match never reaches any of it
//! (`World::arena` is false).

use crate::rules::{ArenaDef, Kind, TPS};
use crate::world::{other, within, Ctx, Mod, Uid, World, EV_WIN, M, MOON, NEUTRAL, SUN};

/// Tag of the arena's buffs, so a buff taken again refreshes.
const TAG_BUFF: u16 = 9200;

pub fn def<'a>(w: &World, ctx: &'a Ctx) -> Option<&'a ArenaDef> {
    if w.arena {
        ctx.rules.arena.as_ref()
    } else {
        None
    }
}

/// A new arena match: full tickets and every camp due at the horn.
pub fn start(w: &mut World, ctx: &Ctx) {
    let Some(a) = ctx.rules.arena.as_ref() else {
        return;
    };
    w.arena = true;
    w.tickets = [a.tickets, a.tickets];
    w.players.iter_mut().for_each(|p| p.gold = a.starting_gold);
    w.camps_back = vec![w.horn_at.max(1); ctx.rules.map.camps.len()];
}

/// Takes `n` tickets from `team`, and ends the match when it has none left.
pub fn spend(w: &mut World, team: u8, n: i32) {
    if team > MOON || n <= 0 || w.winner.is_some() {
        return;
    }
    let t = &mut w.tickets[team as usize];
    *t = (*t - n).max(0);
    if *t == 0 {
        w.winner = Some(other(team));
        w.event(EV_WIN, other(team) as i32, 0, [0, 0]);
    }
}

/// The match's clock: waves, brutes, camps and the portals.
pub fn clock(w: &mut World, ctx: &Ctx) {
    let Some(a) = def(w, ctx) else { return };
    let tick = w.tick;
    if tick >= w.horn_at {
        let since = tick - w.horn_at;
        let every = (a.wave_every * TPS).max(1) as u32;
        if since.is_multiple_of(every) {
            w.waves += 1;
            // the first wave leaves by the top lane, the next by the bottom, and so on
            let lane = if w.waves % 2 == 1 { 0 } else { 2 };
            spawn_wave(w, ctx, a, lane);
        }
        let brute = (a.brute_every * TPS).max(1) as u32;
        if since > 0 && since.is_multiple_of(brute) {
            for team in [SUN, MOON] {
                let lane = if (since / brute) % 2 == 1 { 0 } else { 2 };
                spawn_one(w, ctx, &a.brutes[team as usize], team, lane, [0, 0]);
            }
        }
    }
    // a cleared camp returns once its time is up
    for k in 0..w.camps_back.len() {
        let due = w.camps_back[k];
        if due != 0 && tick >= due {
            w.camps_back[k] = 0;
            crate::lanes::spawn_camp(w, ctx, k);
        }
    }
    portals(w, ctx, a);
}

fn spawn_wave(w: &mut World, ctx: &Ctx, a: &ArenaDef, lane: usize) {
    let t = &ctx.rules.t;
    let elapsed = (w.tick - w.horn_at) as i32 / TPS;
    let level = (elapsed / t.creep_upgrade_every.max(1)).min(t.creep_upgrade_max);
    for team in [SUN, MOON] {
        let side = &ctx.rules.map.sides[team as usize];
        let mut k = 0;
        for (which, &n) in a.wave.iter().enumerate() {
            for _ in 0..n {
                let off = [(k % 2) * 60 - 30, -(k / 2) * 60];
                let name = side.creeps[which].clone();
                if let Some(id) = spawn_one(w, ctx, &name, team, lane, off) {
                    if let Some(u) = w.get_mut(id) {
                        u.upgrade = level;
                    }
                }
                k += 1;
            }
        }
    }
}

fn spawn_one(w: &mut World, ctx: &Ctx, name: &str, team: u8, lane: usize, off: [i32; 2]) -> Option<Uid> {
    let def = ctx.rules.unit(name)?;
    let side = &ctx.rules.map.sides[team as usize];
    let key = if lane == 0 { "top" } else { "bot" };
    let spawn = *side.creep_spawn.get(key)?;
    let at = [spawn[0] + off[0], spawn[1] + off[1]];
    Some(crate::lanes::spawn_creep(w, ctx, def, team, lane, at, 0))
}

/// A creep that reaches the enemy's Sphere walks into its portal: it is
/// gone, and the enemy loses tickets for it.
fn portals(w: &mut World, ctx: &Ctx, a: &ArenaDef) {
    let mut entered: Vec<(Uid, u8, i32)> = Vec::new();
    for u in &w.units {
        if !u.alive || u.team > MOON || ctx.rules.units[u.def as usize].kind != Kind::Creep {
            continue;
        }
        let enemy = other(u.team);
        let Some(sphere) = sphere_at(ctx, enemy) else {
            continue;
        };
        if within(u.pos(), sphere, a.portal_radius) {
            let brute = a.brutes.iter().any(|b| *b == ctx.rules.units[u.def as usize].id);
            entered.push((u.id, enemy, if brute { a.brute_cost } else { a.portal_cost }));
        }
    }
    for (id, enemy, cost) in entered {
        let at = w.get(id).map(|u| u.pos()).unwrap_or([0, 0]);
        if let Some(u) = w.get_mut(id) {
            u.alive = false;
            u.mods.clear();
        }
        w.event(crate::world::EV_PORTAL, enemy as i32, cost, at);
        spend(w, enemy, cost);
    }
}

/// Where `team`'s Sphere (its portal) stands.
pub fn sphere_at(ctx: &Ctx, team: u8) -> Option<crate::rules::P> {
    let side = ctx.rules.map.sides.get(team as usize)?;
    side.buildings
        .iter()
        .find(|b| {
            ctx.rules
                .unit(&b.unit)
                .map(|d| ctx.rules.units[d].kind == Kind::Sphere)
                .unwrap_or(false)
        })
        .map(|b| b.at)
}

/// A unit has died (not an illusion): what it costs its side, and the
/// buff a cleared camp gives the hero who cleared it.
pub fn died(w: &mut World, ctx: &Ctx, kind: Kind, team: u8, camp: u8, killer: Uid) {
    let Some(a) = def(w, ctx) else { return };
    let by_hero = w.get(killer).map(|k| k.owner >= 0).unwrap_or(false);
    match kind {
        Kind::Hero => spend(w, team, a.hero_cost),
        Kind::Creep if by_hero && team <= MOON => {
            if w.tickets[team as usize] > a.last_tickets {
                spend(w, team, a.creep_cost);
            }
        }
        Kind::Neutral if team == NEUTRAL && camp > 0 => {
            let k = camp as usize - 1;
            let left = w.units.iter().any(|u| u.alive && u.camp == camp);
            if left || k >= w.camps_back.len() {
                return;
            }
            w.camps_back[k] = w.tick + (a.camp_back * TPS) as u32;
            let buff = &ctx.rules.map.camps[k].buff;
            let owner = w.get(killer).map(|k| k.owner).unwrap_or(-1);
            if owner >= 0 && !buff.is_empty() {
                let hero = w.players[owner as usize].hero;
                give_buff(w, a, hero, buff);
            }
        }
        _ => {}
    }
}

/// The buff named `name` on `hero` for the buff's time.
pub fn give_buff(w: &mut World, a: &ArenaDef, hero: Uid, name: &str) {
    let until = w.tick + (a.buff_secs * TPS) as u32;
    let Some(k) = a.buffs.iter().position(|b| b.0 == name) else {
        return;
    };
    let (effect, amount) = (a.buffs[k].1.as_str(), a.buffs[k].2);
    let kind = match effect {
        "damage_pct" => M::DamagePct,
        "attack_pct" => M::AttackPct,
        "mana_regen" => M::ManaRegen,
        "regen" => M::Regen,
        "move_pct" => M::MovePct,
        _ => return,
    };
    let tag = TAG_BUFF + k as u16;
    if let Some(u) = w.get_mut(hero) {
        if !u.alive {
            return;
        }
        u.mods.retain(|m| m.tag != tag);
        u.mods.push(Mod {
            kind,
            src: hero,
            until,
            a: amount,
            b: 0,
            tag,
        });
    }
}

/// The buff a unit carries, as the index into `buffs`, if any.
pub fn buff_of(w: &World, ctx: &Ctx, id: Uid) -> Option<usize> {
    let a = def(w, ctx)?;
    let u = w.get(id)?;
    u.mods
        .iter()
        .find(|m| m.tag >= TAG_BUFF && ((m.tag - TAG_BUFF) as usize) < a.buffs.len())
        .map(|m| (m.tag - TAG_BUFF) as usize)
}

/// The arena's Sphere is a portal: nothing can hurt it.
pub fn portal_guarded(w: &World, kind: Kind) -> bool {
    w.arena && kind == Kind::Sphere
}

/// Seconds a dead hero waits here, from the usual.
pub fn respawn_secs(w: &World, ctx: &Ctx, secs: i32) -> i32 {
    match def(w, ctx) {
        Some(a) => secs * a.respawn_pct / 100,
        None => secs,
    }
}
