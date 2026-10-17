//! Computer players: behavior trees, after Vagrancy's pilots.
//!
//! A tree is a priority list of rules, read from `data/ai.json`. Each rule
//! has conditions and an action; the first rule whose conditions all hold
//! wins. An action, once chosen, holds until its rule stops holding or a
//! rule marked `interrupt` above it fires. Difficulty is a separate table of
//! numbers passed to the same trees: how many ticks between decisions (the
//! reaction time), the chance to time a last hit, to deny, to use an
//! ability when it would help, and the health share at which to retreat. So
//! an ally's skill and an enemy's skill are set apart, as the brief asks.
//!
//! Bots run inside the match: their choices are part of the world, so every
//! browser in an online match computes the same choices from the same state.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::orders::{self, Cmd};
use crate::rules::{Kind, Target, P, TPS};
use crate::stats;
use crate::world::{dist2, other, within, Act, Ctx, Uid, World, M, NEUTRAL, NO_UNIT};

/// One condition of a rule. Numbers are percents or map units.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Cond {
    Dead,
    HpBelow(i32),
    HpAbove(i32),
    ManaAbove(i32),
    /// An enemy hero visible within this range.
    EnemyHeroNear(i32),
    /// An enemy hero visible within range [1] below health share [0].
    EnemyHeroWeak([i32; 2]),
    /// At least [0] enemy heroes within range [1].
    EnemiesNear([i32; 2]),
    /// At least [0] allied heroes within range [1].
    AlliesNear([i32; 2]),
    /// More enemy heroes than allied heroes within range.
    Outnumbered(i32),
    UnderEnemyTower,
    /// An enemy tower is attacking this hero.
    TowerTargetsMe,
    EnemyCreepsNear(i32),
    HasPoints,
    CanBuy,
    AtFountain,
    /// Health below this share and away from the fountain.
    NeedsFountain(i32),
    LevelAtLeast(i32),
    /// Any damaging or disabling ability ready with mana for it.
    AbilityReady,
    /// The match is past this minute.
    After(i32),
    /// A seeded chance, percent.
    Chance(i32),
    /// The difficulty table's own flag (e.g. "denies").
    Skill(String),
    /// Carries a ready item that heals (a salve, a tango, the bottle), or one that restores mana.
    CanHeal,
    CanMana,
    Not(Box<Cond>),
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    #[serde(rename = "if", default)]
    pub when: Vec<Cond>,
    #[serde(rename = "do")]
    pub action: String,
    #[serde(default)]
    pub interrupt: bool,
    /// Shown in the tree view.
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Tree {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Difficulty {
    pub id: String,
    /// Ticks between decisions.
    pub reaction: i32,
    /// Percent chance a decision times a last hit well.
    pub last_hit: i32,
    pub deny: i32,
    /// Percent chance to use a useful ability when one is ready.
    pub cast: i32,
    /// Percent health at which to retreat.
    pub retreat: i32,
    /// Percent chance to commit to a fight it could win.
    pub aggression: i32,
    /// Whether it buys its build's items.
    pub shops: bool,
    /// Flags for `Skill` conditions.
    #[serde(default)]
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Build {
    /// Ability slots in the order to learn them (4 is the attribute bonus).
    pub skills: Vec<u8>,
    pub items: Vec<String>,
    /// Which tree to run.
    #[serde(default = "hero_tree")]
    pub tree: String,
}

fn hero_tree() -> String {
    "hero".into()
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AiDef {
    pub trees: BTreeMap<String, Tree>,
    pub difficulty: Vec<Difficulty>,
    pub builds: BTreeMap<String, Build>,
    /// Lane for each seat in a side (seat 0 to 4).
    pub lanes: Vec<String>,
}

impl AiDef {
    pub fn load(json: &str) -> Result<AiDef, String> {
        let d: AiDef = serde_json::from_str(json).map_err(|e| format!("ai.json could not be read: {e}"))?;
        if d.difficulty.is_empty() {
            return Err("ai.json lists no difficulty.".into());
        }
        if !d.trees.contains_key("hero") {
            return Err("ai.json has no \"hero\" tree.".into());
        }
        Ok(d)
    }
}

/// A bot's memory: kept in the world, so rollback replays it exactly.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Brain {
    /// The rule now held, and since when.
    pub rule: i32,
    pub since: u32,
    /// Next item in the build to buy.
    pub next_item: u16,
    pub next_skill: u16,
    /// The lane it farms: 0 top, 1 mid, 2 bot.
    pub lane: u8,
    /// The trace of rules tried at the last decision, for the tree view:
    /// one entry per rule: 0 not reached, 1 failed, 2 chosen.
    pub trace: Vec<u8>,
    /// Counts for measuring the farm routine: decisions, creeps in reach,
    /// last-hit tries, deny tries.
    pub farm: [u32; 4],
}

/// Every bot whose turn it is decides and issues a command.
pub fn think(w: &mut World, ctx: &Ctx) {
    let Some(ai) = ctx.rules.ai.as_ref() else { return };
    for p in 0..w.players.len() {
        let Some(diff) = w.players[p].bot else { continue };
        let d = &ai.difficulty[(diff as usize).min(ai.difficulty.len() - 1)];
        let reaction = d.reaction.max(1) as u32;
        // seats decide on staggered ticks so ten bots do not share one tick
        if !(w.tick + p as u32 * 7).is_multiple_of(reaction) {
            continue;
        }
        if w.players[p].hero == NO_UNIT {
            // pick the hero this seat's index points to, skipping taken ones
            let taken: Vec<i32> = w.players.iter().map(|x| x.pick).collect();
            let n = ctx.rules.heroes.len();
            let mut k = (p * 3 + w.tick as usize) % n;
            for _ in 0..n {
                if !taken.contains(&(k as i32)) {
                    break;
                }
                k = (k + 1) % n;
            }
            orders::apply(w, ctx, p, Cmd::Pick(k as u8));
            let seat = p % 5;
            w.players[p].brain.lane = lane_index(ai.lanes.get(seat).map(|s| s.as_str()).unwrap_or("mid"));
            continue;
        }
        if w.aram && aram(w, ctx, p) {
            continue;
        }
        decide(w, ctx, ai, d, p);
    }
}

/// All Random All Mid: a computer player keeps to the middle lane, rides
/// the cannon out when healed at the base, and fetches a relic near it when
/// below half health. Returns true when it gave an order (or is in the air).
fn aram(w: &mut World, ctx: &Ctx, p: usize) -> bool {
    w.players[p].brain.lane = lane_index("mid");
    let hero = w.players[p].hero;
    let Some(u) = w.live(hero) else { return false };
    if matches!(u.act, crate::world::Act::Flight { .. }) {
        return true;
    }
    let (team, here, hp) = (u.team, u.pos(), u.hp);
    let max = crate::stats::of(ctx, w, hero).max_hp * 1000;
    if crate::aram::may_throw(w, ctx, hero) && hp * 10 >= max * 9 {
        if let Some((_, end, _)) = crate::aram::band(ctx, team) {
            orders::apply(w, ctx, p, Cmd::Cannon(end));
            return true;
        }
    }
    if hp * 2 < max {
        let spots = crate::aram::relic_spots(ctx);
        let near = spots
            .iter()
            .enumerate()
            .filter(|&(i, s)| w.relics.get(i) == Some(&0) && crate::world::within(here, *s, 1800))
            .min_by_key(|&(_, s)| crate::world::dist2(here, *s))
            .map(|(_, s)| *s);
        if let Some(s) = near {
            orders::apply(w, ctx, p, Cmd::Move(s));
            return true;
        }
    }
    false
}

fn lane_index(name: &str) -> u8 {
    match name {
        "top" => 0,
        "bot" => 2,
        _ => 1,
    }
}

struct Look {
    hero: Uid,
    team: u8,
    here: P,
    hp_pct: i32,
    mana_pct: i32,
    level: i32,
}

fn look(w: &World, ctx: &Ctx, p: usize) -> Option<Look> {
    let hero = w.players[p].hero;
    let u = w.get(hero)?;
    let s = stats::of(ctx, w, hero);
    Some(Look {
        hero,
        team: u.team,
        here: u.pos(),
        hp_pct: if s.max_hp > 0 { u.hp / 10 / s.max_hp } else { 0 },
        mana_pct: if s.max_mana > 0 {
            u.mana / 10 / s.max_mana
        } else {
            0
        },
        level: u.level(),
    })
}

fn visible_enemy_heroes(w: &World, ctx: &Ctx, l: &Look, r: i32) -> Vec<Uid> {
    w.near(l.here, r, |u| {
        u.team == other(l.team) && u.hero.is_some() && !u.has(M::Illusion) && w.sees(ctx, l.team, u)
    })
}

fn enemy_towers_near(w: &World, ctx: &Ctx, team: u8, at: P, pad: i32) -> Vec<Uid> {
    w.units
        .iter()
        .filter(|u| {
            u.alive
                && u.team == other(team)
                && u.team != NEUTRAL
                && matches!(ctx.rules.units[u.def as usize].kind, Kind::Tower | Kind::Fountain)
                && within(u.pos(), at, ctx.rules.units[u.def as usize].range + pad)
        })
        .map(|u| u.id)
        .collect()
}

fn holds(w: &mut World, ctx: &Ctx, ai: &AiDef, d: &Difficulty, p: usize, l: &Look, c: &Cond) -> bool {
    let alive = w.live(l.hero).is_some();
    match c {
        Cond::Dead => !alive,
        _ if !alive => false,
        Cond::HpBelow(x) => l.hp_pct < *x,
        Cond::HpAbove(x) => l.hp_pct > *x,
        Cond::ManaAbove(x) => l.mana_pct > *x,
        Cond::EnemyHeroNear(r) => !visible_enemy_heroes(w, ctx, l, *r).is_empty(),
        Cond::EnemyHeroWeak([pct, r]) => visible_enemy_heroes(w, ctx, l, *r).iter().any(|&e| {
            let s = stats::of(ctx, w, e);
            w.get(e)
                .map(|u| s.max_hp > 0 && u.hp / 10 / s.max_hp < *pct)
                .unwrap_or(false)
        }),
        Cond::EnemiesNear([n, r]) => visible_enemy_heroes(w, ctx, l, *r).len() as i32 >= *n,
        Cond::AlliesNear([n, r]) => {
            w.near(l.here, *r, |u| {
                u.team == l.team && u.hero.is_some() && u.id != l.hero
            })
            .len() as i32
                >= *n
        }
        Cond::Outnumbered(r) => {
            let foes = visible_enemy_heroes(w, ctx, l, *r).len();
            let friends = w.near(l.here, *r, |u| u.team == l.team && u.hero.is_some()).len();
            foes > friends
        }
        Cond::UnderEnemyTower => !enemy_towers_near(w, ctx, l.team, l.here, 100).is_empty(),
        Cond::TowerTargetsMe => w.units.iter().any(|u| {
            u.alive
                && u.team == other(l.team)
                && u.target == l.hero
                && ctx.rules.units[u.def as usize].kind == Kind::Tower
        }),
        Cond::EnemyCreepsNear(r) => !w
            .near(l.here, *r, |u| {
                u.team == other(l.team) && ctx.rules.units[u.def as usize].kind == Kind::Creep
            })
            .is_empty(),
        Cond::HasPoints => w
            .get(l.hero)
            .and_then(|u| u.hero.as_ref())
            .map(|h| h.points > 0)
            .unwrap_or(false),
        Cond::CanBuy => next_purchase(w, ctx, ai, p).is_some(),
        Cond::AtFountain => within(
            l.here,
            ctx.rules.map.sides[l.team as usize].fountain,
            ctx.rules.t.fountain_range,
        ),
        Cond::NeedsFountain(x) => {
            l.hp_pct < *x
                && !within(
                    l.here,
                    ctx.rules.map.sides[l.team as usize].fountain,
                    ctx.rules.t.fountain_range,
                )
        }
        Cond::LevelAtLeast(n) => l.level >= *n,
        Cond::AbilityReady => (0..4).any(|s| useful_cast(w, ctx, l, s).is_some()),
        Cond::After(m) => w.tick as i32 >= m * 60 * TPS,
        Cond::Chance(pct) => w.rng.below(100) < *pct as u32,
        Cond::Skill(flag) => d.flags.contains(flag),
        Cond::CanHeal | Cond::CanMana => {
            let want: &[&str] = if *c == Cond::CanHeal {
                &["salve", "tango", "bottle"]
            } else {
                &["clarity", "bottle"]
            };
            w.get(l.hero)
                .map(|u| {
                    u.items.iter().any(|s| {
                        s.item > 0
                            && s.ready_at <= w.tick
                            && (s.charges > 0 || ctx.rules.items[s.item as usize - 1].charges == 0)
                            && want.contains(&ctx.rules.items[s.item as usize - 1].logic.as_str())
                    })
                })
                .unwrap_or(false)
        }
        Cond::Not(c) => !holds(w, ctx, ai, d, p, l, c),
    }
}

fn decide(w: &mut World, ctx: &Ctx, ai: &AiDef, d: &Difficulty, p: usize) {
    let Some(l) = look(w, ctx, p) else { return };
    let tree_name = build_of(ctx, ai, w, p)
        .map(|b| b.tree.clone())
        .unwrap_or_else(hero_tree);
    let Some(tree) = ai.trees.get(&tree_name).or_else(|| ai.trees.get("hero")) else {
        return;
    };
    let held = w.players[p].brain.rule;
    let mut trace = vec![0u8; tree.rules.len()];
    let mut chosen = None;
    for (i, rule) in tree.rules.iter().enumerate() {
        // a held action keeps its place unless an interrupting rule above it fires
        if held >= 0 && (held as usize) < i && !rule.interrupt {
            // rules below the held one are still tried when the held one fails
        }
        let ok = rule.when.iter().all(|c| holds(w, ctx, ai, d, p, &l, c));
        if ok {
            trace[i] = 2;
            chosen = Some(i);
            break;
        }
        trace[i] = 1;
    }
    let Some(i) = chosen else { return };
    let brain = &mut w.players[p].brain;
    if brain.rule != i as i32 {
        brain.rule = i as i32;
        brain.since = w.tick;
    }
    brain.trace = trace;
    let action = tree.rules[i].action.clone();
    act(w, ctx, ai, d, p, &l, &action);
}

fn build_of<'a>(ctx: &Ctx, ai: &'a AiDef, w: &World, p: usize) -> Option<&'a Build> {
    let u = w.get(w.players[p].hero)?;
    ai.builds.get(&ctx.rules.units[u.def as usize].id)
}

fn next_purchase(w: &World, ctx: &Ctx, ai: &AiDef, p: usize) -> Option<usize> {
    let b = build_of(ctx, ai, w, p)?;
    let i = w.players[p].brain.next_item as usize;
    let name = b.items.get(i)?;
    let k = ctx.rules.item(name)?;
    let cost = crate::item::price(w, ctx, p, k);
    if w.players[p].gold >= cost && crate::item::near_shop(w, ctx, p, crate::aram::shop_of(w, ctx, k)) {
        Some(k)
    } else {
        None
    }
}

/// A slot whose ability would do something useful now, and its target.
fn useful_cast(w: &World, ctx: &Ctx, l: &Look, slot: usize) -> Option<(Uid, P)> {
    let (k, level) = crate::ability::slot_info(ctx, w, l.hero, slot)?;
    if level <= 0 {
        return None;
    }
    let a = &ctx.rules.abilities[k];
    if matches!(a.target, Target::Passive | Target::Toggle) {
        return None;
    }
    let range = a.range_at(level).max(a.v("radius", level)) + 200;
    let foes = visible_enemy_heroes(w, ctx, l, range);
    let foe = *foes.first()?;
    let at = w.get(foe)?.pos();
    let t = if a.on == "ally" { l.hero } else { foe };
    if !crate::ability::can_start(w, ctx, l.hero, slot, t, at) {
        return None;
    }
    // a no-target ability needs the foe close
    if a.target == Target::None && !within(l.here, at, a.v("radius", level).max(300)) && a.on != "self" {
        return None;
    }
    Some((t, at))
}

/// The point a laning hero holds: a little behind its own creeps' front.
fn lane_spot(w: &World, ctx: &Ctx, l: &Look, lane: u8, range: i32) -> P {
    let names = ["top", "mid", "bot"];
    let side = &ctx.rules.map.sides[l.team as usize];
    let path = side
        .lanes
        .get(names[lane as usize % 3])
        .cloned()
        .unwrap_or_default();
    // the allied creep of this lane farthest along its waypoints
    let front = w
        .units
        .iter()
        .filter(|u| {
            u.alive
                && u.team == l.team
                && ctx.rules.units[u.def as usize].kind == Kind::Creep
                && u.lane == lane
        })
        .max_by_key(|u| {
            (
                u.waypoint,
                -dist2(u.pos(), *path.get(u.waypoint as usize).unwrap_or(&u.pos())) / 1024,
            )
        })
        .map(|u| u.pos());
    match front {
        Some(f) => {
            // wait just inside reach of the enemy creeps nearest the front,
            // on the home side of them
            let foe = w
                .units
                .iter()
                .filter(|u| {
                    u.alive && u.team == other(l.team) && ctx.rules.units[u.def as usize].kind == Kind::Creep
                })
                .min_by_key(|u| crate::world::dist2(u.pos(), f))
                .map(|u| u.pos())
                .filter(|p| within(*p, f, 900))
                .unwrap_or(f);
            let home = side.fountain;
            let d = crate::world::dist(foe, home).max(1);
            let back = (range + 40).clamp(150, 500);
            [
                foe[0] + (home[0] - foe[0]) * back / d,
                foe[1] + (home[1] - foe[1]) * back / d,
            ]
        }
        // no creeps, in All Random All Mid: out in the lane where the cannon
        // lands, past the outer tower, not pressed against it
        None if w.aram => crate::aram::band(ctx, l.team)
            .map(|(_, end, _)| end)
            .unwrap_or(side.spawn),
        // no creeps: the first allied tower of the lane
        None => w
            .units
            .iter()
            .filter(|u| {
                u.alive
                    && u.team == l.team
                    && ctx.rules.units[u.def as usize].kind == Kind::Tower
                    && u.lane_name == lane
            })
            .min_by_key(|u| {
                crate::world::dist2(u.pos(), ctx.rules.map.sides[other(l.team) as usize].fountain)
            })
            .map(|u| u.pos())
            .unwrap_or(side.spawn),
    }
}

fn act(w: &mut World, ctx: &Ctx, ai: &AiDef, d: &Difficulty, p: usize, l: &Look, action: &str) {
    let hero = l.hero;
    let busy = w
        .get(hero)
        .map(|u| matches!(u.act, Act::Cast { .. } | Act::Channel { .. }))
        .unwrap_or(true);
    if busy && action != "wait" {
        return;
    }
    match action {
        "wait" => {
            if w.live(hero).is_none() && d.shops {
                if let Some(k) = next_purchase(w, ctx, ai, p) {
                    orders::apply(w, ctx, p, Cmd::Buy(k as u16));
                    w.players[p].brain.next_item += 1;
                }
            }
        }
        "learn" => {
            let Some(b) = build_of(ctx, ai, w, p) else { return };
            let i = w.players[p].brain.next_skill as usize;
            let slot = b.skills.get(i).copied().unwrap_or(4);
            let before = w
                .get(hero)
                .and_then(|u| u.hero.as_ref())
                .map(|h| h.points)
                .unwrap_or(0);
            orders::apply(w, ctx, p, Cmd::Learn(slot));
            let after = w
                .get(hero)
                .and_then(|u| u.hero.as_ref())
                .map(|h| h.points)
                .unwrap_or(0);
            if after < before {
                w.players[p].brain.next_skill += 1;
            } else {
                // the build's next skill is not allowed yet: take the first that is
                for s in 0..5u8 {
                    orders::apply(w, ctx, p, Cmd::Learn(s));
                    let now = w
                        .get(hero)
                        .and_then(|u| u.hero.as_ref())
                        .map(|h| h.points)
                        .unwrap_or(0);
                    if now < before {
                        break;
                    }
                }
            }
        }
        "buy" => {
            if let Some(k) = next_purchase(w, ctx, ai, p) {
                let before = w.players[p].gold;
                orders::apply(w, ctx, p, Cmd::Buy(k as u16));
                if w.players[p].gold == before {
                    // a full inventory: sell the cheapest item to make room, then try again
                    let cheapest = w.get(hero).and_then(|u| {
                        u.items
                            .iter()
                            .enumerate()
                            .filter(|(_, s)| s.item > 0)
                            .min_by_key(|(_, s)| ctx.rules.item_total(s.item as usize - 1))
                            .map(|(i, _)| i)
                    });
                    if let Some(slot) = cheapest {
                        orders::apply(w, ctx, p, Cmd::Sell(slot as u8));
                        orders::apply(w, ctx, p, Cmd::Buy(k as u16));
                    }
                }
                if w.players[p].gold < before {
                    w.players[p].brain.next_item += 1;
                }
            }
        }
        "potion" => use_consumable(w, ctx, p, l),
        "retreat" | "go_home" => {
            let to = if action == "go_home" {
                ctx.rules.map.sides[l.team as usize].fountain
            } else {
                // the nearest allied tower nearer home than this hero, else home
                let home = ctx.rules.map.sides[l.team as usize].fountain;
                let mine = crate::world::dist2(l.here, home);
                w.units
                    .iter()
                    .filter(|u| {
                        u.alive
                            && u.team == l.team
                            && ctx.rules.units[u.def as usize].kind == Kind::Tower
                            && crate::world::dist2(u.pos(), home) < mine - 600 * 600
                    })
                    .min_by_key(|u| crate::world::dist2(u.pos(), l.here))
                    .map(|u| {
                        [
                            u.pos()[0] + (home[0] - u.pos()[0]).signum() * 200,
                            u.pos()[1] + (home[1] - u.pos()[1]).signum() * 200,
                        ]
                    })
                    .unwrap_or(home)
            };
            orders::apply(w, ctx, p, Cmd::Move(to));
        }
        "stay" => {
            orders::apply(w, ctx, p, Cmd::Hold);
        }
        "cast" => {
            if w.rng.below(100) >= d.cast as u32 {
                return;
            }
            for slot in [3usize, 0, 1, 2] {
                if let Some((t, at)) = useful_cast(w, ctx, l, slot) {
                    orders::apply(w, ctx, p, Cmd::Cast(slot as u8, t, at));
                    return;
                }
            }
        }
        "fight" => {
            if w.rng.below(100) >= d.aggression as u32 {
                return;
            }
            let foes = visible_enemy_heroes(w, ctx, l, 1200);
            // the weakest enemy hero in reach
            let pick = foes
                .into_iter()
                .min_by_key(|&e| w.get(e).map(|u| u.hp).unwrap_or(i32::MAX));
            if let Some(e) = pick {
                orders::apply(w, ctx, p, Cmd::Attack(e));
            }
        }
        "harass" => {
            let foes = visible_enemy_heroes(w, ctx, l, stats::of(ctx, w, hero).range + 150);
            if let Some(&e) = foes.first() {
                let at = w.get(e).map(|u| u.pos()).unwrap_or(l.here);
                if enemy_towers_near(w, ctx, l.team, at, 0).is_empty() {
                    orders::apply(w, ctx, p, Cmd::Attack(e));
                }
            }
        }
        "farm" => farm(w, ctx, d, p, l),
        "siege" => {
            // an enemy building our creeps are already at, and that can be hurt
            let target = w
                .units
                .iter()
                .filter(|u| {
                    u.alive
                        && u.team == other(l.team)
                        && u.team != NEUTRAL
                        && matches!(
                            ctx.rules.units[u.def as usize].kind,
                            Kind::Tower | Kind::Barracks | Kind::Sphere
                        )
                        && !u.has(M::Invulnerable)
                        && within(u.pos(), l.here, 1400)
                })
                .filter(|b| {
                    w.units.iter().any(|c| {
                        c.alive
                            && c.team == l.team
                            && ctx.rules.units[c.def as usize].kind == Kind::Creep
                            && within(c.pos(), b.pos(), 700)
                    })
                })
                .min_by_key(|b| crate::world::dist2(b.pos(), l.here))
                .map(|b| b.id);
            match target {
                Some(t) => orders::apply(w, ctx, p, Cmd::Attack(t)),
                None => farm(w, ctx, d, p, l),
            }
        }
        "push" => {
            let lane = w.players[p].brain.lane;
            let names = ["top", "mid", "bot"];
            let path = &ctx.rules.map.sides[l.team as usize].lanes[names[lane as usize % 3]];
            let goal = *path.last().unwrap_or(&l.here);
            orders::apply(w, ctx, p, Cmd::AttackMove(goal));
        }
        "rune" => {
            if let Some((spot, _)) = w.rune {
                orders::apply(w, ctx, p, Cmd::Move(ctx.rules.map.runes[spot]));
            }
        }
        _ => {}
    }
}

/// Uses a healing item when hurt or a mana item when low: a salve or the
/// bottle on itself, a tango on the nearest tree, a clarity.
fn use_consumable(w: &mut World, ctx: &Ctx, p: usize, l: &Look) {
    let Some(u) = w.get(l.hero) else { return };
    let items = u.items;
    let healing = u.mods.iter().any(|m| m.kind == M::Regen && m.tag >= 5000);
    for (slot, s) in items.iter().enumerate() {
        if s.item == 0 || s.ready_at > w.tick {
            continue;
        }
        let it = &ctx.rules.items[s.item as usize - 1];
        let cmd = match it.logic.as_str() {
            "salve" | "bottle" if l.hp_pct < 60 && !healing => Some(Cmd::Item(slot as u8, l.hero, l.here)),
            "clarity" if l.mana_pct < 40 => Some(Cmd::Item(slot as u8, l.hero, l.here)),
            "tango" if l.hp_pct < 75 && !healing => {
                // the nearest standing tree
                let t = ctx.terrain;
                let (cx, cy) = (l.here[0] / t.tree_cell, l.here[1] / t.tree_cell);
                let mut best: Option<(i32, P)> = None;
                for dy in -4..=4 {
                    for dx in -4..=4 {
                        let (x, y) = (cx + dx, cy + dy);
                        if x < 0 || y < 0 || x >= t.tw || y >= t.tw {
                            continue;
                        }
                        let i = (y * t.tw + x) as usize;
                        if matches!(w.trees[i], crate::map::Tree::Standing) {
                            let d = dx * dx + dy * dy;
                            if best.map(|b| d < b.0).unwrap_or(true) {
                                best = Some((d, t.tree_centre(i)));
                            }
                        }
                    }
                }
                best.map(|(_, at)| Cmd::Item(slot as u8, NO_UNIT, at))
            }
            _ => None,
        };
        if let Some(c) = cmd {
            orders::apply(w, ctx, p, c);
            return;
        }
    }
}

/// Laning: last-hit enemy creeps, deny allied ones, otherwise hold a spot
/// behind the creep front. A miss-timed attack is how a weaker bot shows.
fn farm(w: &mut World, ctx: &Ctx, d: &Difficulty, p: usize, l: &Look) {
    let hero = l.hero;
    // a swing already under way at a creep is left to land
    let swinging = w
        .get(hero)
        .map(|u| matches!(u.act, Act::Swing { .. }))
        .unwrap_or(false);
    if swinging {
        return;
    }
    let s = stats::of(ctx, w, hero);
    let reach = s.range + 350;
    let avg = (s.damage[0] + s.damage[1]) / 2 + s.bonus;
    let creeps: Vec<(Uid, i32, u8)> = w
        .units
        .iter()
        .filter(|u| {
            u.alive
                && ctx.rules.units[u.def as usize].kind == Kind::Creep
                && within(u.pos(), l.here, reach)
                && w.sees(ctx, l.team, u)
        })
        .map(|u| (u.id, u.hp / 1000, u.team))
        .collect();
    let skilled = w.rng.below(100) < d.last_hit as u32;
    let mut best: Option<(i32, Uid)> = None;
    // the time from the order to the blow landing: the walk is ignored, the
    // swing and the missile's flight are counted, and the next decision is
    // a reaction away, so a creep's expected loss over that time is allowed for
    let hero_def = &ctx.rules.units[w.get(hero).map(|u| u.def as usize).unwrap_or(0)];
    for &(id, hp, team) in &creeps {
        let armor = stats::of(ctx, w, id).armor;
        let mut hit = avg * crate::combat::armor_keep(armor, ctx.rules.t.armor_factor) / 1000;
        let creep_at = w.get(id).map(|u| u.pos()).unwrap_or(l.here);
        let far = crate::world::dist(l.here, creep_at);
        let flight = if hero_def.missile > 0 {
            far * 100 / hero_def.missile
        } else {
            0
        };
        let walk = (far - s.range - 48).max(0) * 100 / s.move_speed.max(1);
        let lands_in = hero_def.attack_point + flight + walk + d.reaction * 100 / TPS / 2;
        let attackers = w
            .units
            .iter()
            .filter(|u| {
                u.alive
                    && u.team != team
                    && u.target == id
                    && ctx.rules.units[u.def as usize].kind == Kind::Creep
            })
            .count() as i32;
        // a lane creep deals about 22 damage a second
        hit += attackers * 22 * lands_in / 100;
        let enemy = team != l.team;
        let max = stats::of(ctx, w, id).max_hp;
        let deny_ok = !enemy && hp * 100 < max * ctx.rules.t.deny_below && w.rng.below(100) < d.deny as u32;
        // a skilled decision waits for the creep to drop within one hit;
        // an unskilled one swings at whatever is lowest
        let ready = if skilled { hp <= hit } else { true };
        // a creep under an enemy tower is left alone
        let under_tower = !enemy_towers_near(w, ctx, l.team, creep_at, 50).is_empty();
        if (enemy || deny_ok) && ready && !under_tower {
            let key = hp;
            if best.map(|b| key < b.0).unwrap_or(true) {
                best = Some((key, id));
            }
        }
    }
    w.players[p].brain.farm[0] += 1;
    if !creeps.is_empty() {
        w.players[p].brain.farm[1] += 1;
    }
    if let Some((_, t)) = best {
        let deny = w.get(t).map(|u| u.team == l.team).unwrap_or(false);
        w.players[p].brain.farm[if deny { 3 } else { 2 }] += 1;
        orders::apply(w, ctx, p, Cmd::Attack(t));
        return;
    }
    let mut spot = lane_spot(w, ctx, l, w.players[p].brain.lane, s.range);
    // never wait inside an enemy tower's reach: step back toward home out of it
    let home = ctx.rules.map.sides[l.team as usize].fountain;
    for _ in 0..8 {
        if enemy_towers_near(w, ctx, l.team, spot, 150).is_empty() {
            break;
        }
        let d = crate::world::dist(spot, home).max(1);
        spot = [
            spot[0] + (home[0] - spot[0]) * 200 / d,
            spot[1] + (home[1] - spot[1]) * 200 / d,
        ];
    }
    if !within(l.here, spot, 120) {
        orders::apply(w, ctx, p, Cmd::Move(spot));
    } else {
        // between last hits a player stops rather than pushing the lane
        orders::apply(w, ctx, p, Cmd::Stand);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tree_reads_from_json_with_its_conditions() {
        let t: Tree = serde_json::from_str(
            r#"{"rules":[{"if":["dead"],"do":"wait"},{"if":[{"hp_below":30},{"not":"at_fountain"}],"do":"go_home","interrupt":true},{"do":"farm"}]}"#,
        )
        .unwrap();
        assert_eq!(t.rules.len(), 3);
        assert_eq!(t.rules[1].when[0], Cond::HpBelow(30));
        assert_eq!(t.rules[1].when[1], Cond::Not(Box::new(Cond::AtFountain)));
        assert!(t.rules[2].when.is_empty());
    }
}
