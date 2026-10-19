//! Items: buying near a shop, recipes that combine, selling, actives and
//! passives, and the shield the boss drops.
//!
//! A purchase needs the hero (alive or dead in the fountain) near a shop
//! that sells the item: the base shop for most, a side shop or the secret
//! shop for some, as in DotA. Buying the last part of a recipe combines the
//! parts in the inventory at once. Selling returns half the price.

use crate::combat::{self, Dmg};
use crate::fx;
use crate::rules::{Kind, Target, P, SUB, TPS};
use crate::stats;
use crate::world::{
    within, Act, Ctx, Hit, ItemSlot, Missile, Mod, Order, Uid, World, EV_BUY, M, NEUTRAL, NO_UNIT,
};

fn tag(item: usize) -> u16 {
    5000 + item as u16
}

fn secs_x100(w: &World, hundredths: i32) -> u32 {
    w.tick + ((hundredths * TPS + 50) / 100).max(1) as u32
}

/// Whether the hero stands near a shop of this kind ("home" also counts at
/// the fountain while dead).
pub fn near_shop(w: &World, ctx: &Ctx, player: usize, kind: &str) -> bool {
    let id = w.players[player].hero;
    let Some(u) = w.get(id) else { return false };
    let team = u.team;
    let here = if u.alive {
        u.pos()
    } else {
        ctx.rules.map.sides[team as usize].spawn
    };
    let r = ctx.rules.t.shop_range;
    for (k, at) in &ctx.rules.map.shops {
        let ok = match kind {
            "home" => k == &format!("home{team}"),
            "side" => k == "side",
            "secret" => k == "secret",
            _ => false,
        };
        if ok && within(here, *at, r) {
            return true;
        }
    }
    false
}

/// Where an item may be bought now: a basic item at its shop; a recipe
/// item wherever its recipe scroll is sold (the base shop).
fn sold_here(w: &World, ctx: &Ctx, player: usize, k: usize) -> bool {
    let it = &ctx.rules.items[k];
    let shop = crate::aram::shop_of(w, ctx, k);
    if shop == "none" {
        return false;
    }
    near_shop(w, ctx, player, shop) || (shop == "home" && near_shop(w, ctx, player, "side") && side_sells(it))
}

/// Side shops sell a short list of basic items, as in DotA.
fn side_sells(it: &crate::rules::ItemDef) -> bool {
    it.tab == "basics"
}

/// Buys item `k`: pays for the missing parts and the recipe, combining
/// parts already carried.
pub fn buy(w: &mut World, ctx: &Ctx, player: usize, k: usize) {
    if k >= ctx.rules.items.len() {
        return;
    }
    let id = w.players[player].hero;
    if w.get(id).is_none() {
        return;
    }
    if !sold_here(w, ctx, player, k) {
        // away from the shop: buy through the courier waiting at the base,
        // at the full price (parts carried are not combined from afar)
        let team = w.players[player].team;
        let it = &ctx.rules.items[k];
        let cost = ctx.rules.item_total(k);
        let alive = w.live(id).is_some();
        if crate::aram::shop_of(w, ctx, k) == "home"
            && alive
            && crate::courier::at_base(w, ctx, team)
            && w.players[player].gold >= cost
        {
            pay(w, player, cost);
            w.players[player].pending.push((k as u16 + 1, it.charges));
            w.event(EV_BUY, k as i32, player as i32, [0, 0]);
        }
        return;
    }
    // which carried items the purchase uses as parts
    let mut used = [false; 6];
    let cost = price_with(w, ctx, id, k, &mut used, true);
    if w.players[player].gold < cost {
        return;
    }
    let it = &ctx.rules.items[k];
    // stacking charges into a held item of the same kind
    if it.stacks {
        if let Some(u) = w.get_mut(id) {
            if let Some(s) = u.items.iter_mut().find(|s| s.item as usize == k + 1) {
                s.charges += it.charges;
                pay(w, player, cost);
                w.event(EV_BUY, k as i32, player as i32, [0, 0]);
                return;
            }
        }
    }
    let u = w.get(id).expect("checked");
    let free = (0..6).filter(|&i| used[i] || u.items[i].item == 0).count();
    if free == 0 {
        return;
    }
    let u = w.get_mut(id).expect("checked");
    for (i, slot) in u.items.iter_mut().enumerate() {
        if used[i] {
            *slot = ItemSlot::default();
        }
    }
    let slot = u.items.iter().position(|s| s.item == 0).expect("counted");
    u.items[slot] = ItemSlot {
        item: k as u16 + 1,
        charges: it.charges,
        ready_at: 0,
    };
    pay(w, player, cost);
    w.event(EV_BUY, k as i32, player as i32, [0, 0]);
}

fn pay(w: &mut World, player: usize, cost: i32) {
    let p = &mut w.players[player];
    p.gold -= cost;
    // unreliable gold is spent first
    let unreliable = (p.gold + cost - p.reliable).max(0);
    if cost > unreliable {
        p.reliable -= cost - unreliable;
    }
    p.reliable = p.reliable.clamp(0, p.gold.max(0));
}

/// The gold still to pay for item `k`, marking carried parts it would use.
fn price_with(w: &World, ctx: &Ctx, id: Uid, k: usize, used: &mut [bool; 6], top: bool) -> i32 {
    let it = &ctx.rules.items[k];
    if !top {
        // a carried copy of this part covers it
        if let Some(u) = w.get(id) {
            for (i, slot) in u.items.iter().enumerate() {
                if !used[i] && slot.item as usize == k + 1 {
                    used[i] = true;
                    return 0;
                }
            }
        }
    }
    if it.parts.is_empty() {
        return it.cost;
    }
    let mut total = if it.recipe { it.cost } else { 0 };
    for &p in &ctx.rules.item_parts[k] {
        total += price_with(w, ctx, id, p, used, false);
    }
    total
}

/// One piece of an item's build tree: the item, whether a carried copy
/// covers it, its recipe's own cost (0 for none), and its parts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub item: usize,
    pub owned: bool,
    pub recipe: i32,
    pub parts: Vec<Piece>,
}

/// Item `k`'s build tree for this player, with the carried parts that
/// buying it would use marked owned. It walks the parts in the same order
/// as `price`, so the tree and the price agree.
pub fn tree(w: &World, ctx: &Ctx, player: usize, k: usize) -> Piece {
    let mut used = [false; 6];
    tree_with(w, ctx, w.players[player].hero, k, &mut used, true)
}

fn tree_with(w: &World, ctx: &Ctx, id: Uid, k: usize, used: &mut [bool; 6], top: bool) -> Piece {
    let it = &ctx.rules.items[k];
    if !top {
        if let Some(u) = w.get(id) {
            for (i, slot) in u.items.iter().enumerate() {
                if !used[i] && slot.item as usize == k + 1 {
                    used[i] = true;
                    return Piece {
                        item: k,
                        owned: true,
                        recipe: 0,
                        parts: Vec::new(),
                    };
                }
            }
        }
    }
    let recipe = if !it.parts.is_empty() && it.recipe {
        it.cost
    } else {
        0
    };
    let parts = ctx.rules.item_parts[k]
        .iter()
        .map(|&p| tree_with(w, ctx, id, p, used, false))
        .collect();
    Piece {
        item: k,
        owned: false,
        recipe,
        parts,
    }
}

/// The items item `k` is a part of.
pub fn builds_into(ctx: &Ctx, k: usize) -> Vec<usize> {
    (0..ctx.rules.items.len())
        .filter(|&j| ctx.rules.item_parts[j].contains(&k) && ctx.rules.items[j].shop != "none")
        .collect()
}

/// What item `k` costs this player now, with carried parts counted.
pub fn price(w: &World, ctx: &Ctx, player: usize, k: usize) -> i32 {
    let mut used = [false; 6];
    price_with(w, ctx, w.players[player].hero, k, &mut used, true)
}

/// The gold the item in `slot` would sell for now: half its whole price,
/// at any shop, or nothing when no shop is in reach, the slot is empty or
/// the item is one no shop sells.
pub fn sell_value(w: &World, ctx: &Ctx, player: usize, slot: usize) -> Option<i32> {
    if slot >= 6
        || !near_shop(w, ctx, player, "home")
            && !near_shop(w, ctx, player, "side")
            && !near_shop(w, ctx, player, "secret")
    {
        return None;
    }
    let s = w.get(w.players[player].hero)?.items[slot];
    if s.item == 0 {
        return None;
    }
    let k = s.item as usize - 1;
    if ctx.rules.items[k].shop == "none" {
        return None;
    }
    Some(ctx.rules.item_total(k) / 2)
}

pub fn sell(w: &mut World, ctx: &Ctx, player: usize, slot: usize) {
    let Some(back) = sell_value(w, ctx, player, slot) else {
        return;
    };
    let id = w.players[player].hero;
    let Some(u) = w.get_mut(id) else { return };
    u.items[slot] = ItemSlot::default();
    w.players[player].gold += back;
}

fn item_of(w: &World, id: Uid, slot: usize) -> Option<(usize, ItemSlot)> {
    let u = w.get(id)?;
    let s = *u.items.get(slot)?;
    if s.item == 0 {
        return None;
    }
    Some((s.item as usize - 1, s))
}

pub fn can_use(w: &World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, _p: P) -> bool {
    let Some((k, s)) = item_of(w, id, slot) else {
        return false;
    };
    let it = &ctx.rules.items[k];
    let Some(u) = w.live(id) else { return false };
    if it.logic.is_empty() || it.target == Some(Target::Passive) || it.target.is_none() {
        return false;
    }
    if s.ready_at > w.tick || u.mana < it.mana * 1000 || u.has(M::Stun) || u.has(M::Hex) {
        return false;
    }
    if it.charges > 0 && s.charges <= 0 && it.logic != "bottle" && it.logic != "wand" {
        return false;
    }
    if it.target == Some(Target::Unit) {
        let Some(v) = w.live(t) else { return false };
        if ctx.rules.units[v.def as usize].kind.is_building() {
            return false;
        }
    }
    true
}

/// An item that acts at once, with no target to walk to.
pub fn instant(_w: &World, ctx: &Ctx, _id: Uid, _slot: usize) -> bool {
    let Some((k, _)) = item_of(_w, _id, _slot) else {
        return true;
    };
    ctx.rules.items[k].target == Some(Target::None)
}

pub fn approach(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, at: P) {
    let Some((k, _)) = item_of(w, id, slot) else {
        idle(w, id);
        return;
    };
    if !can_use(w, ctx, id, slot, t, at) {
        idle(w, id);
        return;
    }
    let it = &ctx.rules.items[k];
    let here = w.get(id).expect("checked").pos();
    let goal = if it.target == Some(Target::Unit) {
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
    // blink goes as far as it can toward a far point instead of walking
    let range = it.range.max(100);
    if it.logic != "blink" && it.logic != "tp" && !within(here, goal, range + 30) {
        crate::orders::walk(w, ctx, id, goal, range);
        return;
    }
    let tick = w.tick;
    let u = w.get_mut(id).expect("checked");
    u.act = Act::Cast {
        slot: slot as u8,
        t,
        at,
        fire_at: tick + if it.logic == "tp" { (3 * TPS) as u32 } else { 1 },
        item: true,
    };
    u.order = Order::Idle;
    u.path.clear();
    u.anim = crate::world::ANIM_CAST;
    u.anim_at = tick;
}

/// Where a teleport toward `at` lands: the side's building nearest the
/// point, or its spawn when none stands. The scene draws the same place.
pub fn tp_destination(w: &World, ctx: &Ctx, team: u8, at: P) -> P {
    w.units
        .iter()
        .filter(|u| {
            u.alive
                && u.team == team
                && matches!(
                    ctx.rules.units[u.def as usize].kind,
                    Kind::Tower | Kind::Fountain | Kind::Barracks | Kind::Sphere
                )
        })
        .min_by_key(|u| crate::world::dist2(u.pos(), at))
        .map(|u| u.pos())
        .unwrap_or(ctx.rules.map.sides[team as usize].spawn)
}

fn idle(w: &mut World, id: Uid) {
    if let Some(u) = w.get_mut(id) {
        u.order = Order::Idle;
        u.path.clear();
    }
}

fn add(w: &mut World, id: Uid, kind: M, src: Uid, until: u32, a: i32, t: u16) {
    w.add_mod(
        id,
        Mod {
            kind,
            src,
            until,
            a,
            b: 0,
            tag: t,
        },
    );
}

/// The item's effect, its cooldown and its charge.
pub fn use_item(w: &mut World, ctx: &Ctx, id: Uid, slot: usize, t: Uid, at: P) {
    let Some((k, _)) = item_of(w, id, slot) else {
        return;
    };
    if !can_use(w, ctx, id, slot, t, at) {
        return;
    }
    let it = &ctx.rules.items[k];
    let tg = tag(k);
    let here = w.get(id).expect("checked").pos();
    let team = w.get(id).expect("checked").team;
    let mut spend_charge = it.charges > 0;
    match it.logic.as_str() {
        "tango" => {
            // eat the tree nearest the point
            let tc = ctx.terrain.tree_cell;
            let i = ctx.terrain.tree_index(at[0], at[1]);
            let Some(i) = i else { return };
            if !matches!(w.trees[i], crate::map::Tree::Standing) || !within(here, at, it.range + tc) {
                return;
            }
            w.cut_tree(ctx, i);
            let until = secs_x100(w, it.val("duration"));
            add(
                w,
                id,
                M::Regen,
                id,
                until,
                it.val("heal") * 10_000 / it.val("duration").max(1),
                tg,
            );
        }
        "salve" => {
            let target = if t == NO_UNIT { id } else { t };
            let until = secs_x100(w, it.val("duration"));
            add(
                w,
                target,
                M::Regen,
                id,
                until,
                it.val("heal") * 10_000 / it.val("duration").max(1),
                tg,
            );
        }
        "clarity" => {
            let target = if t == NO_UNIT { id } else { t };
            let until = secs_x100(w, it.val("duration"));
            add(
                w,
                target,
                M::ManaRegen,
                id,
                until,
                it.val("mana") * 10_000 / it.val("duration").max(1),
                tg,
            );
        }
        "tp" => {
            let dest = tp_destination(w, ctx, team, at);
            if let Some(u) = w.get_mut(id) {
                u.x = (dest[0] - 150) * SUB;
                u.y = (dest[1] - 150) * SUB;
                u.path.clear();
            }
        }
        "ward_observer" | "ward_sentry" => {
            let name = if it.logic == "ward_observer" {
                "observer_ward"
            } else {
                "sentry_ward"
            };
            if let Some(def) = ctx.rules.unit(name) {
                let wid = w.spawn(ctx, def, team, at);
                let until = secs_x100(w, it.val("duration"));
                let truesight = it.val("true_sight");
                if let Some(u) = w.get_mut(wid) {
                    u.expires = until;
                    u.mods.push(Mod {
                        kind: M::Invisible,
                        src: id,
                        until: 0,
                        a: 0,
                        b: 0,
                        tag: tg,
                    });
                    if truesight > 0 {
                        u.mods.push(Mod {
                            kind: M::TrueSight,
                            src: id,
                            until: 0,
                            a: truesight,
                            b: 0,
                            tag: tg,
                        });
                    }
                }
            }
        }
        "blink" => {
            // a hero hurt by another hero in the last 3 s cannot blink, as in DotA
            let hurt = w
                .get(id)
                .and_then(|u| u.hero.as_ref())
                .map(|h| h.damaged_by.iter().any(|(_, at)| w.tick < at + (3 * TPS) as u32))
                .unwrap_or(false);
            if hurt {
                return;
            }
            let far = crate::world::dist(here, at);
            let range = it.range;
            let dest = if far <= range {
                at
            } else {
                [
                    here[0] + (at[0] - here[0]) * range / far.max(1),
                    here[1] + (at[1] - here[1]) * range / far.max(1),
                ]
            };
            let walk = w.walk(ctx.terrain);
            let pc = ctx.terrain.path_cell;
            let open = crate::path::nearest_open(&walk, dest[0] / pc, dest[1] / pc, 6);
            if let Some((cx, cy)) = open {
                let u = w.get_mut(id).expect("checked");
                u.x = (cx * pc + pc / 2) * SUB;
                u.y = (cy * pc + pc / 2) * SUB;
                u.path.clear();
            }
        }
        "bkb" => {
            let until = secs_x100(w, it.val("duration"));
            add(w, id, M::SpellImmune, id, until, 0, tg);
        }
        "wand" => {
            let charges = w.get(id).expect("checked").items[slot].charges;
            if charges <= 0 {
                return;
            }
            combat::heal(w, ctx, id, charges * it.val("per_charge"));
            let max = stats::of(ctx, w, id).max_mana * 1000;
            let u = w.get_mut(id).expect("checked");
            u.mana = (u.mana + charges * it.val("per_charge") * 1000).min(max);
            u.items[slot].charges = 0;
            spend_charge = false;
        }
        "bottle" => {
            let charges = w.get(id).expect("checked").items[slot].charges;
            if charges <= 0 {
                return;
            }
            let until = secs_x100(w, it.val("duration"));
            add(
                w,
                id,
                M::Regen,
                id,
                until,
                it.val("heal") * 10_000 / it.val("duration").max(1),
                tg,
            );
            add(
                w,
                id,
                M::ManaRegen,
                id,
                until,
                it.val("mana") * 10_000 / it.val("duration").max(1),
                tg,
            );
        }
        "midas" => {
            let Some(v) = w.live(t) else { return };
            let kind = ctx.rules.units[v.def as usize].kind;
            if !matches!(kind, Kind::Creep | Kind::Neutral) || v.team == team {
                return;
            }
            let xp = ctx.rules.units[v.def as usize].xp * it.val("xp_pct") / 100;
            let owner = w.get(id).expect("checked").owner;
            if let Some(v) = w.get_mut(t) {
                v.hp = 0;
                v.last_attacker = NO_UNIT;
            }
            if owner >= 0 {
                w.players[owner as usize].gold += it.val("gold");
            }
            combat::give_xp(w, ctx, id, xp);
        }
        "force" => {
            let target = if t == NO_UNIT { id } else { t };
            if let Some(u) = w.get_mut(target) {
                let d = it.val("distance");
                let nx = u.x / SUB + fx::cos(u.facing) * d / fx::ONE;
                let ny = u.y / SUB + fx::sin(u.facing) * d / fx::ONE;
                let s = ctx.rules.map.size - 200;
                u.x = nx.clamp(200, s) * SUB;
                u.y = ny.clamp(200, s) * SUB;
                u.path.clear();
            }
        }
        "euls" => {
            let target = if t == NO_UNIT { id } else { t };
            let until = secs_x100(w, it.val("duration"));
            add(w, target, M::Stun, id, until, 0, tg);
            add(w, target, M::Invulnerable, id, until, 0, tg);
        }
        "dagon" => {
            combat::damage(w, ctx, id, t, it.val("damage"), Dmg::Magic);
        }
        "mekansm" => {
            let until = secs_x100(w, it.val("duration"));
            for ally in w.near(here, it.val("radius"), |u| {
                u.team == team && !ctx.rules.units[u.def as usize].kind.is_building()
            }) {
                combat::heal(w, ctx, ally, it.val("heal"));
                add(w, ally, M::Armor, id, until, it.val("armor") * 100, tg);
            }
        }
        "manta" => {
            crate::ability::illusions(
                w,
                ctx,
                id,
                2,
                it.val("duration") / 100,
                it.val("dealt"),
                it.val("taken"),
            );
        }
        "sheep" => {
            let until = secs_x100(w, it.val("duration"));
            add(w, t, M::Hex, id, until, 0, tg);
        }
        "orchid" => {
            let until = secs_x100(w, it.val("duration"));
            add(w, t, M::Silence, id, until, 0, tg);
        }
        "shivas" => {
            for v in crate::ability::enemies_near(w, ctx, team, here, it.val("radius"), false) {
                combat::damage(w, ctx, id, v, it.val("damage"), Dmg::Magic);
                let until = secs_x100(w, it.val("duration"));
                add(w, v, M::MovePct, id, until, -it.val("slow"), tg);
            }
        }
        "satanic" => {
            let until = secs_x100(w, it.val("duration"));
            add(w, id, M::Tag, id, until, it.val("lifesteal"), tg);
        }
        _ => {}
    }
    let cd = secs_x100(w, it.cooldown);
    let mana = it.mana * 1000;
    if let Some(u) = w.get_mut(id) {
        u.items[slot].ready_at = cd;
        u.mana -= mana;
        if spend_charge {
            u.items[slot].charges -= 1;
            let k1 = u.items[slot].item as usize - 1;
            if u.items[slot].charges <= 0
                && ctx.rules.items[k1].logic != "bottle"
                && ctx.rules.items[k1].logic != "wand"
            {
                u.items[slot] = ItemSlot::default();
            }
        }
    }
}

pub fn missile_arrives(_w: &mut World, _ctx: &Ctx, _m: &Missile) {}

/// Item passives on an attack, before the roll lands: critical strikes.
pub fn before_attack(w: &mut World, ctx: &Ctx, src: Uid, raw: i32) -> Option<i32> {
    let u = w.get(src)?;
    let mut best: Option<i32> = None;
    let items: Vec<usize> = u
        .items
        .iter()
        .filter(|s| s.item > 0)
        .map(|s| s.item as usize - 1)
        .collect();
    for k in items {
        let it = &ctx.rules.items[k];
        if it.logic == "crit" && combat::prd(w, src, 3, it.val("chance")) {
            let v = raw * it.val("multiplier") / 100;
            best = Some(best.map(|b| b.max(v)).unwrap_or(v));
        }
    }
    best
}

/// Item passives after an attack lands: bash, chain lightning, armor
/// reduction, cleave, lifesteal from Satanic's active.
pub fn after_attack(w: &mut World, ctx: &Ctx, src: Uid, dst: Uid, raw: i32) {
    let Some(u) = w.get(src) else { return };
    let team = u.team;
    let melee = stats::missile(ctx, w, src) == 0;
    let items: Vec<usize> = u
        .items
        .iter()
        .filter(|s| s.item > 0)
        .map(|s| s.item as usize - 1)
        .collect();
    let dst_pos = w.get(dst).map(|v| v.pos()).unwrap_or([0, 0]);
    let mut bashed = false;
    for k in items {
        let it = &ctx.rules.items[k];
        let tg = tag(k);
        match it.logic.as_str() {
            "bash" if !bashed => {
                let chance = if melee {
                    it.val("chance")
                } else {
                    it.val("ranged_chance")
                };
                if combat::prd(w, src, 4, chance) {
                    bashed = true;
                    combat::damage(w, ctx, src, dst, it.val("damage"), Dmg::Physical);
                    crate::ability::stun(w, ctx, src, dst, it.val("stun"), tg);
                }
            }
            "lightning" => {
                if combat::prd(w, src, 5, it.val("chance")) {
                    let mut hit = vec![dst];
                    let mut at = dst_pos;
                    combat::damage(w, ctx, src, dst, it.val("damage"), Dmg::Magic);
                    for _ in 1..it.val("bounces") {
                        let next = crate::ability::enemies_near(w, ctx, team, at, 500, false)
                            .into_iter()
                            .find(|v| !hit.contains(v));
                        let Some(n) = next else { break };
                        combat::damage(w, ctx, src, n, it.val("damage"), Dmg::Magic);
                        at = w.get(n).map(|v| v.pos()).unwrap_or(at);
                        hit.push(n);
                    }
                }
            }
            "corrupt" => {
                let until = secs_x100(w, it.val("duration"));
                add(w, dst, M::Armor, src, until, -it.val("armor") * 100, tg);
            }
            "cleave" if melee => {
                for v in crate::ability::enemies_near(w, ctx, team, dst_pos, it.val("radius"), true) {
                    if v != dst {
                        combat::damage(w, ctx, src, v, raw * it.val("pct") / 100, Dmg::Physical);
                    }
                }
            }
            _ => {}
        }
    }
    // Satanic's active: extra lifesteal while it lasts
    let rage = w
        .get(src)
        .map(|u| {
            u.mods
                .iter()
                .filter(|m| {
                    m.kind == M::Tag
                        && m.tag >= 5000
                        && ctx
                            .rules
                            .items
                            .get(m.tag as usize - 5000)
                            .map(|i| i.logic == "satanic")
                            .unwrap_or(false)
                })
                .map(|m| m.a)
                .sum::<i32>()
        })
        .unwrap_or(0);
    if rage > 0 {
        combat::heal(w, ctx, src, raw * rage / 100);
    }
}

/// Radiance's burn and other item auras, every tick.
pub fn auras(w: &mut World, ctx: &Ctx) {
    let carriers: Vec<(Uid, u8, P, usize)> = w
        .units
        .iter()
        .filter(|u| u.alive && u.hero.is_some() && !u.has(M::Illusion))
        .flat_map(|u| {
            u.items
                .iter()
                .filter(|s| s.item > 0)
                .map(move |s| (u.id, u.team, u.pos(), s.item as usize - 1))
        })
        .collect();
    for (id, team, at, k) in carriers {
        let it = &ctx.rules.items[k];
        match it.logic.as_str() {
            "radiance" => {
                let per = it.val("dps") * 1000 / TPS;
                for v in crate::ability::enemies_near(w, ctx, team, at, it.val("radius"), false) {
                    combat::damage_milli(w, ctx, id, v, per, Dmg::Magic, false);
                }
            }
            "assault" => {
                let until = w.tick + 2;
                for ally in w.near(at, it.val("radius"), |u| u.team == team) {
                    add(w, ally, M::Armor, id, until, it.val("armor") * 100, tag(k));
                    add(w, ally, M::AttackPct, id, until, it.val("attack_speed"), tag(k));
                }
            }
            "heart" => {
                // two percent of maximum health a second
                let max = stats::of(ctx, w, id).max_hp;
                let until = w.tick + 2;
                add(w, id, M::Regen, id, until, max * it.val("regen_pct"), tag(k));
            }
            "bottle" => {
                // a bottle refills at the fountain
                let fountain = ctx.rules.map.sides[team as usize].fountain;
                if within(at, fountain, ctx.rules.t.fountain_range) {
                    if let Some(u) = w.get_mut(id) {
                        for s in u.items.iter_mut() {
                            if s.item as usize == k + 1 {
                                s.charges = it.charges;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// A hero casting a spell charges enemy heroes' wands nearby.
pub fn charge_wands(w: &mut World, ctx: &Ctx, caster: Uid) {
    let Some(c) = w.get(caster) else { return };
    let (team, at) = (c.team, c.pos());
    let near = w.near(at, 1200, |u| u.team != team && u.hero.is_some());
    for id in near {
        if let Some(u) = w.get_mut(id) {
            for s in u.items.iter_mut() {
                if s.item == 0 {
                    continue;
                }
                let it = &ctx.rules.items[s.item as usize - 1];
                if it.logic == "wand" {
                    s.charges = (s.charges + 1).min(it.val("max"));
                }
            }
        }
    }
}

/// The boss's shield appears in the slayer's inventory, or at their feet
/// when full (here: given to the slayer's hero if there is a free slot).
pub fn drop_aegis(w: &mut World, ctx: &Ctx, killer: Uid, _at: P) {
    let Some(k) = ctx.rules.item("aegis") else { return };
    let owner = w.get(killer).map(|u| u.owner).unwrap_or(-1);
    if owner < 0 {
        return;
    }
    let hero = w.players[owner as usize].hero;
    if let Some(u) = w.get_mut(hero) {
        if let Some(s) = u.items.iter_mut().find(|s| s.item == 0) {
            *s = ItemSlot {
                item: k as u16 + 1,
                charges: 1,
                ready_at: 0,
            };
        }
    }
}

/// A hero holding the shield is saved from death: it is spent and the hero
/// rises in place at full health after a short pause.
pub fn aegis_saves(w: &mut World, ctx: &Ctx, id: Uid) -> bool {
    let Some(k) = ctx.rules.item("aegis") else {
        return false;
    };
    let Some(u) = w.get(id) else { return false };
    let Some(slot) = u.items.iter().position(|s| s.item as usize == k + 1) else {
        return false;
    };
    let s = stats::of(ctx, w, id);
    let until = w.tick + (ctx.rules.items[k].val("delay") * TPS / 100) as u32;
    let u = w.get_mut(id).expect("checked");
    u.items[slot] = ItemSlot::default();
    u.hp = s.max_hp * 1000;
    u.mana = s.max_mana * 1000;
    u.mods.retain(|m| m.kind == M::Illusion);
    u.mods.push(Mod {
        kind: M::Stun,
        src: id,
        until,
        a: 0,
        b: 0,
        tag: tag(k),
    });
    u.mods.push(Mod {
        kind: M::Invulnerable,
        src: id,
        until,
        a: 0,
        b: 0,
        tag: tag(k),
    });
    true
}

/// For the HUD: whether a ward or a bought unit stands for the side.
pub fn neutral() -> u8 {
    NEUTRAL
}

#[allow(dead_code)]
fn unused(_: Hit) {}
