//! A unit's numbers right now: its kind's base, a hero's attributes, its
//! items and its modifiers, combined the way DotA combines them.
//!
//! Strength gives health and health regeneration, agility gives armor and
//! attack speed, intelligence gives mana and mana regeneration, and the
//! primary attribute adds to damage one for one. Percent movement bonuses
//! from items do not stack with each other (boots of one kind count once);
//! slows add up. Attack speed and movement speed are held to the table's
//! limits.

use crate::rules::{Attr, Kind, TPS};
use crate::world::{Ctx, Uid, World, M};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stats {
    pub max_hp: i32,
    pub max_mana: i32,
    /// Hundredths a second.
    pub hp_regen: i32,
    pub mana_regen: i32,
    /// Hundredths.
    pub armor: i32,
    /// Base damage range including the primary attribute.
    pub damage: [i32; 2],
    /// Bonus damage from items and modifiers (the green number).
    pub bonus: i32,
    /// Percent attack speed above base (may be negative).
    pub attack_speed: i32,
    pub move_speed: i32,
    pub range: i32,
    pub magic_resist: i32,
    pub evasion: i32,
    pub lifesteal: i32,
    /// Whole attributes.
    pub attrs: [i32; 3],
    /// Base attack time in hundredths of a second.
    pub bat: i32,
}

impl Stats {
    /// Ticks between attacks.
    pub fn attack_ticks(&self) -> i32 {
        let speed = 100 + self.attack_speed;
        (self.bat * TPS * 100 / (100 * speed)).max(1)
    }

    /// Sixteenths a tick.
    pub fn step(&self) -> i32 {
        self.move_speed * crate::rules::SUB / TPS
    }
}

/// Attributes in hundredths (base + gain a level + attribute bonus + items).
pub fn attrs_x100(ctx: &Ctx, w: &World, id: Uid) -> [i32; 3] {
    let Some(u) = w.get(id) else { return [0; 3] };
    let d = &ctx.rules.units[u.def as usize];
    let Some(a) = &d.attrs else { return [0; 3] };
    let level = u.level();
    let bonus = u.hero.as_ref().map(|h| h.bonus).unwrap_or(0) * ctx.rules.t.stat_bonus * 100;
    let mut out = [0; 3];
    for (k, v) in out.iter_mut().enumerate() {
        *v = a.base[k] * 100 + a.gain[k] * (level - 1) + bonus;
    }
    for s in &u.items {
        if s.item == 0 {
            continue;
        }
        let b = &ctx.rules.items[s.item as usize - 1].bonus;
        out[0] += b.str * 100;
        out[1] += b.agi * 100;
        out[2] += b.int * 100;
    }
    for m in &u.mods {
        if m.kind == M::Tag && m.b >= 1000 && m.b < 1003 {
            // a timed attribute bonus: b = 1000 + attribute index
            out[(m.b - 1000) as usize] += m.a * 100;
        }
    }
    // a share of an attribute taken away (Whirling Death), of the whole
    for m in &u.mods {
        let c = crate::ability::TAG_ATTR_PCT;
        if m.kind == M::Tag && m.b >= c && m.b < c + 3 {
            let v = &mut out[(m.b - c) as usize];
            *v -= *v * m.a / 100;
        }
    }
    out
}

/// The speed of a unit's attack missile now, or 0 for a melee hit:
/// Berserker Rage's melee form swings its axes instead of throwing them.
pub fn missile(ctx: &Ctx, w: &World, id: Uid) -> i32 {
    match w.get(id) {
        Some(u) if !crate::ability::berserk(u) => ctx.rules.units[u.def as usize].missile,
        _ => 0,
    }
}

pub fn of(ctx: &Ctx, w: &World, id: Uid) -> Stats {
    let Some(u) = w.get(id) else {
        return Stats::default();
    };
    let t = &ctx.rules.t;
    let d = &ctx.rules.units[u.def as usize];
    let mut s = Stats {
        max_hp: d.hp,
        max_mana: d.mana,
        hp_regen: d.hp_regen,
        mana_regen: d.mana_regen,
        armor: d.armor,
        damage: d.damage,
        bonus: 0,
        attack_speed: 0,
        move_speed: d.speed,
        range: d.range,
        magic_resist: d.magic_resist,
        evasion: 0,
        lifesteal: 0,
        attrs: [0; 3],
        bat: d.bat,
    };
    if d.kind == Kind::Hero {
        // heroes resist spells innately (DotA's Spells against Hero armor)
        s.magic_resist = 100 - (100 - s.magic_resist) * (100 - t.hero_magic_resist) / 100;
        let a = attrs_x100(ctx, w, id);
        let whole = [a[0] / 100, a[1] / 100, a[2] / 100];
        s.attrs = whole;
        s.max_hp = t.hero_base_hp + whole[0] * t.hp_per_str;
        s.max_mana = whole[2] * t.mana_per_int;
        s.hp_regen += whole[0] * t.hp_regen_per_str;
        s.mana_regen += whole[2] * t.mana_regen_per_int;
        s.armor += a[1] * t.armor_per_agi / 100;
        s.attack_speed += whole[1] * t.as_per_agi;
        let primary = match d.attrs.as_ref().map(|x| x.primary) {
            Some(Attr::Str) => whole[0],
            Some(Attr::Agi) => whole[1],
            Some(Attr::Int) => whole[2],
            None => 0,
        };
        s.damage = [d.damage[0] + primary, d.damage[1] + primary];
    }
    if d.kind == Kind::Creep {
        s.max_hp += d.upgrade_hp * u.upgrade;
        s.damage = [
            d.damage[0] + d.upgrade_damage * u.upgrade,
            d.damage[1] + d.upgrade_damage * u.upgrade,
        ];
    }
    let mut move_pct_item = 0;
    let mut mana_regen_pct = 0;
    let mut evasion_miss = 100;
    for slot in &u.items {
        if slot.item == 0 {
            continue;
        }
        let b = &ctx.rules.items[slot.item as usize - 1].bonus;
        s.bonus += b.damage;
        s.armor += b.armor;
        s.attack_speed += b.attack_speed;
        s.move_speed += b.move_speed;
        move_pct_item = move_pct_item.max(b.move_pct);
        s.max_hp += b.hp;
        s.max_mana += b.mana;
        s.hp_regen += b.hp_regen;
        s.mana_regen += b.mana_regen;
        mana_regen_pct += b.mana_regen_pct;
        if b.evasion > 0 {
            evasion_miss = evasion_miss * (100 - b.evasion) / 100;
        }
        s.lifesteal = s.lifesteal.max(b.lifesteal);
        if b.magic_resist > 0 {
            // magic resistances multiply
            s.magic_resist = 100 - (100 - s.magic_resist) * (100 - b.magic_resist) / 100;
        }
    }
    // only base mana regeneration from intelligence grows by percent
    if d.kind == Kind::Hero {
        s.mana_regen += (s.attrs[2] * t.mana_regen_per_int + d.mana_regen) * mana_regen_pct / 100;
    }
    let mut move_pct = move_pct_item;
    let mut damage_pct = 0;
    let mut haste = false;
    for m in &u.mods {
        match m.kind {
            M::MovePct => move_pct += m.a,
            M::AttackPct => s.attack_speed += m.a,
            M::Armor => s.armor += m.a,
            M::Damage => s.bonus += m.a,
            M::DamagePct => damage_pct += m.a,
            M::Regen => {
                if m.a > 0 {
                    s.hp_regen += m.a
                }
            }
            M::ManaRegen => s.mana_regen += m.a,
            M::Haste => haste = true,
            M::MagicResist => {
                s.magic_resist = 100 - (100 - s.magic_resist) * (100 - m.a) / 100;
            }
            M::Evasion => evasion_miss = evasion_miss * (100 - m.a) / 100,
            // Take Aim: extra attack range
            M::Tag if m.b == 2002 => s.range += m.a,
            // Berserker Rage's melee form: its range, attack time and health
            // from the ability, flat move speed in `a`
            M::Tag if m.b == crate::ability::TAG_BERSERK => {
                if let Some(a) = (m.tag as usize)
                    .checked_sub(1)
                    .and_then(|k| ctx.rules.abilities.get(k))
                {
                    s.range = a.v("range", 1);
                    s.bat = a.v("bat", 1);
                    s.max_hp += a.v("hp", 1);
                }
                s.move_speed += m.a;
            }
            _ => {}
        }
    }
    s.evasion = 100 - evasion_miss;
    let base_avg = (s.damage[0] + s.damage[1]) / 2;
    s.bonus += base_avg * damage_pct / 100;
    s.attack_speed = s.attack_speed.clamp(t.min_attack_speed, t.max_attack_speed);
    s.move_speed = (s.move_speed + s.move_speed * move_pct / 100).clamp(t.min_move_speed, t.max_move_speed);
    if haste {
        s.move_speed = t.max_move_speed;
    }
    if u.has(M::Hex) {
        s.move_speed = s.move_speed.min(t.min_move_speed + 40);
    }
    if u.has(M::Root) || d.speed == 0 {
        s.move_speed = 0;
    }
    s
}

/// Regeneration, fountain healing, damage over time, and cooldown upkeep,
/// once a tick for every living unit.
pub fn regen(w: &mut World, ctx: &Ctx) {
    let fountains: Vec<(u8, crate::rules::P)> = ctx
        .rules
        .map
        .sides
        .iter()
        .enumerate()
        .map(|(k, s)| (k as u8, s.fountain))
        .collect();
    // a unit at no health is dead this tick, whatever it would regenerate
    let ids: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| u.alive && u.hp > 0)
        .map(|u| u.id)
        .collect();
    let mut dots: Vec<(Uid, Uid, i32)> = Vec::new();
    for id in ids {
        let s = of(ctx, w, id);
        let tick = w.tick;
        let u = w.get_mut(id).expect("listed alive");
        let mut hp_regen = s.hp_regen;
        let mut mana_regen = s.mana_regen;
        for (team, at) in &fountains {
            if u.team == *team && crate::world::within(u.pos(), *at, ctx.rules.t.fountain_range) {
                hp_regen += s.max_hp * ctx.rules.t.fountain_regen;
                mana_regen += s.max_mana * ctx.rules.t.fountain_regen + ctx.rules.t.fountain_mana;
            }
        }
        // hundredths a second → thousandths a tick: ×10 / TPS
        u.hp = (u.hp + hp_regen * 10 / TPS).min(s.max_hp * 1000);
        u.mana = (u.mana + mana_regen * 10 / TPS).clamp(0, s.max_mana * 1000);
        for m in &u.mods {
            if m.kind == M::Regen && m.a < 0 {
                dots.push((m.src, id, -m.a));
            }
        }
        let _ = tick;
    }
    for (src, dst, per_sec) in dots {
        // damage over time is magical, in hundredths a second
        crate::combat::damage_milli(
            w,
            ctx,
            src,
            dst,
            per_sec * 10 / TPS,
            crate::combat::Dmg::Magic,
            false,
        );
    }
}
