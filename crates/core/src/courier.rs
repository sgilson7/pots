//! The team courier: a slow animal that carries items from the base shop
//! to heroes in the field.
//!
//! In DotA a hero away from the shop buys through the courier standing at
//! the base; the courier then walks the items out. Here each side has one
//! courier from the start (DotA sold it for 200 gold; see the notebook). A
//! purchase made away from the shop while the courier waits at the base is
//! paid at once and held by the courier, which walks to the buyer and hands
//! the items over when it reaches them.

use crate::rules::P;
use crate::world::{within, Ctx, ItemSlot, Mod, Order, Uid, World, M, NO_UNIT};

/// Within this distance the courier hands items over.
pub const REACH: i32 = 250;

/// The side's courier, if it stands.
pub fn of(w: &World, team: u8) -> Option<Uid> {
    let id = w.couriers[team.min(1) as usize];
    w.live(id).map(|u| u.id)
}

/// Whether the side's courier waits at the base shop now.
pub fn at_base(w: &World, ctx: &Ctx, team: u8) -> bool {
    let Some(c) = of(w, team).and_then(|id| w.get(id)) else {
        return false;
    };
    let shop = home_shop(ctx, team);
    within(c.pos(), shop, ctx.rules.t.shop_range) && w.carrying[team.min(1) as usize] == NO_UNIT as i32
}

fn home_shop(ctx: &Ctx, team: u8) -> P {
    let key = format!("home{team}");
    ctx.rules
        .map
        .shops
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, p)| *p)
        .unwrap_or(ctx.rules.map.sides[team as usize].spawn)
}

pub fn spawn(w: &mut World, ctx: &Ctx) {
    let Some(def) = ctx.rules.unit("courier") else {
        return;
    };
    for team in 0..2u8 {
        let at = home_shop(ctx, team);
        let id = w.spawn(ctx, def, team, at);
        if let Some(u) = w.get_mut(id) {
            u.mods.push(Mod {
                kind: M::Invulnerable,
                src: id,
                until: 0,
                a: 0,
                b: 0,
                tag: 9200,
            });
        }
        w.couriers[team as usize] = id;
    }
}

/// Walks each courier to the next player with items waiting, hands them
/// over, and walks it home.
pub fn run(w: &mut World, ctx: &Ctx) {
    if w.tick == 1 {
        spawn(w, ctx);
    }
    for team in 0..2u8 {
        let Some(c) = of(w, team) else { continue };
        let carrying = w.carrying[team as usize];
        let target_player = if carrying >= 0 {
            Some(carrying as usize)
        } else {
            (0..w.players.len()).find(|&p| w.players[p].team == team && !w.players[p].pending.is_empty())
        };
        let home = home_shop(ctx, team);
        match target_player {
            Some(p) => {
                w.carrying[team as usize] = p as i32;
                let hero = w.players[p].hero;
                let Some(h) = w.live(hero).map(|u| u.pos()) else {
                    // the buyer is dead: wait at home until they return
                    walk_to(w, c, home);
                    continue;
                };
                walk_to(w, c, h);
                let here = w.get(c).map(|u| u.pos()).unwrap_or(home);
                if within(here, h, REACH) {
                    hand_over(w, p, hero);
                    w.carrying[team as usize] = -1;
                }
            }
            None => walk_to(w, c, home),
        }
    }
}

fn walk_to(w: &mut World, c: Uid, to: P) {
    if let Some(u) = w.get_mut(c) {
        if !matches!(u.order, Order::Move(p) if p == to) {
            u.order = Order::Move(to);
            u.path.clear();
        }
    }
}

/// Moves waiting items into the hero's free slots; what does not fit waits.
fn hand_over(w: &mut World, p: usize, hero: Uid) {
    let mut waiting = std::mem::take(&mut w.players[p].pending);
    if let Some(u) = w.get_mut(hero) {
        waiting.retain(|&(item, charges)| {
            if let Some(s) = u.items.iter_mut().find(|s| s.item == 0) {
                *s = ItemSlot {
                    item,
                    charges,
                    ready_at: 0,
                };
                false
            } else {
                true
            }
        });
    }
    w.players[p].pending = waiting;
}
