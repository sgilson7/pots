//! Rules of the match on the game's own data, each against a value worked
//! out by hand from the literature review (notes/LITERATURE.md, 6.72/6.73).

mod common;

use pots_core::combat;
use pots_core::orders::{self, Cmd};
use pots_core::rules::{Kind, SUB, TPS};
use pots_core::world::{Uid, World, M, MOON, NO_UNIT, SUN};

fn first(w: &World, f: &common::Fixture, kind: Kind, team: u8, tier: i32, lane: u8) -> Uid {
    w.units
        .iter()
        .find(|u| {
            u.alive
                && u.team == team
                && f.rules.units[u.def as usize].kind == kind
                && (tier == 0 || u.tier == tier)
                && (lane == 9 || u.lane_name == lane)
        })
        .map(|u| u.id)
        .unwrap_or(NO_UNIT)
}

#[test]
fn a_hero_starts_with_six_hundred_and_three_gold() {
    // DotA 6.72: a team pool of 4265 gives 853 each, less 250 for the hero
    let f = common::fixture();
    let w = f.world(1);
    assert_eq!(w.players[0].gold, 603);
}

#[test]
fn experience_raises_levels_at_the_table_s_thresholds() {
    // f(2) = 200, f(3) = 200 + 300 = 500
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    let h = w.add_hero(&ctx, 0, 0);
    combat::give_xp(&mut w, &ctx, h, 199);
    assert_eq!(w.get(h).unwrap().level(), 1);
    combat::give_xp(&mut w, &ctx, h, 1);
    assert_eq!(w.get(h).unwrap().level(), 2);
    combat::give_xp(&mut w, &ctx, h, 300);
    assert_eq!(w.get(h).unwrap().level(), 3);
}

#[test]
fn a_hero_waits_four_seconds_a_level_to_respawn() {
    // level 3: 12 seconds, 360 ticks
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    let h = w.add_hero(&ctx, 0, 0);
    combat::give_xp(&mut w, &ctx, h, 500);
    w.get_mut(h).unwrap().hp = 0;
    let at = w.tick;
    w.step(&ctx, &[]);
    let u = w.get(h).unwrap();
    assert!(!u.alive);
    assert_eq!(u.hero.as_ref().unwrap().respawn_at, at + 12 * TPS as u32);
}

#[test]
fn a_creep_can_be_denied_only_below_half_health() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    let h = w.add_hero(&ctx, 0, 0);
    let def = f.rules.unit("creep_melee_sun").unwrap();
    let at = w.get(h).unwrap().pos();
    let c = w.spawn(&ctx, def, SUN, [at[0] + 80, at[1]]);
    let max = f.rules.units[def].hp * 1000;
    // 300 of 550: above half, refused
    w.get_mut(c).unwrap().hp = 300 * 1000;
    assert!(
        !orders::engage(&mut w, &ctx, h, c, true),
        "an ally above half health cannot be attacked"
    );
    // 270 of 550: below half, allowed
    w.get_mut(c).unwrap().hp = 270 * 1000;
    assert!(
        orders::engage(&mut w, &ctx, h, c, true),
        "an ally below half health can be denied"
    );
    let _ = max;
}

#[test]
fn a_last_hit_pays_the_killer_alone_within_the_bounty() {
    // a melee creep is worth 38 to 48 gold to whoever lands the killing blow
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    let h = w.add_hero(&ctx, 0, 0);
    let def = f.rules.unit("creep_melee_moon").unwrap();
    let at = w.get(h).unwrap().pos();
    let c = w.spawn(&ctx, def, MOON, [at[0] + 80, at[1]]);
    let before: Vec<i32> = w.players.iter().map(|p| p.gold).collect();
    combat::damage(&mut w, &ctx, h, c, 10_000, combat::Dmg::Pure);
    w.step(&ctx, &[]);
    let gained = w.players[0].gold - before[0];
    assert!((38..=48).contains(&gained), "the killer gained {gained}");
    for (p, gold) in before.iter().enumerate().skip(1) {
        assert_eq!(w.players[p].gold, *gold, "player {p} gained nothing");
    }
    assert_eq!(w.players[0].last_hits, 1);
}

#[test]
fn a_tower_can_be_hurt_only_after_the_one_before_it_falls() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    for _ in 0..20 {
        w.step(&ctx, &[]);
    }
    let t1 = first(&w, &f, Kind::Tower, MOON, 1, 1);
    let t2 = first(&w, &f, Kind::Tower, MOON, 2, 1);
    assert!(
        t1 != NO_UNIT && t2 != NO_UNIT,
        "the middle lane has a first and second tower"
    );
    assert!(
        !w.get(t1).unwrap().has(M::Invulnerable),
        "the first tower can be hurt"
    );
    assert!(
        w.get(t2).unwrap().has(M::Invulnerable),
        "the second tower cannot while the first stands"
    );
    w.get_mut(t1).unwrap().hp = 0;
    for _ in 0..20 {
        w.step(&ctx, &[]);
    }
    assert!(
        !w.get(t2).unwrap().has(M::Invulnerable),
        "the second tower can be hurt once the first falls"
    );
}

#[test]
fn a_hero_s_attack_does_half_damage_to_a_building() {
    // hero attack against fortified armor: 50 %; then armor 18 keeps
    // 1 / (1 + 1.08) = 48.1 %: a 100 hit lands as 24
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    for _ in 0..20 {
        w.step(&ctx, &[]);
    }
    let h = w.add_hero(&ctx, 0, 0);
    let t1 = first(&w, &f, Kind::Tower, MOON, 1, 1);
    let before = w.get(t1).unwrap().hp;
    combat::attack_lands(&mut w, &ctx, h, t1, 100, false);
    let lost = (before - w.get(t1).unwrap().hp) / 1000;
    assert_eq!(
        lost, 24,
        "a 100-damage hero attack takes {lost} from a first tower"
    );
}

#[test]
fn the_first_mission_is_won_at_fifteen_last_hits() {
    let f = common::fixture();
    let _ = f;
    let mut g = pots_core::game::Game::new(&files(), 3).unwrap();
    g.act("mission", "0").unwrap();
    g.act("pick", "0").unwrap();
    g.act("start", "").unwrap();
    {
        let m = g.m.as_mut().unwrap();
        m.world.players[0].last_hits = 14;
    }
    g.frame(0);
    g.frame(40_000);
    assert!(
        g.m.as_ref().unwrap().over.is_none(),
        "fourteen last hits do not win"
    );
    g.m.as_mut().unwrap().world.players[0].last_hits = 15;
    g.frame(80_000);
    assert_eq!(g.m.as_ref().unwrap().over, Some(pots_core::game::Outcome::Won));
    let _ = (Cmd::Stop, SUB);
}

fn files() -> pots_core::game::Files<'static> {
    pots_core::game::Files {
        strings: include_str!("../../../data/strings.json"),
        glossary: include_str!("../../../GLOSSARY.md"),
        rules: include_str!("../../../data/rules.json"),
        units: include_str!("../../../data/units.json"),
        abilities: include_str!("../../../data/abilities.json"),
        items: include_str!("../../../data/items.json"),
        map: include_str!("../../../data/map.json"),
        ai: include_str!("../../../data/ai.json"),
        models: include_str!("../../../data/models.json"),
        presentation: include_str!("../../../data/presentation.json"),
        keys: include_str!("../../../data/keys.json"),
        arcade: include_str!("../../../data/arcade.json"),
    }
}

#[test]
fn a_hero_keeps_its_gold_when_it_dies() {
    // round 7, the person's ask: no gold is lost on death (DotA took 30 a
    // level of unreliable gold)
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(9);
    let h = w.add_hero(&ctx, 0, 0);
    w.players[0].gold = 1000;
    w.players[0].reliable = 0;
    w.get_mut(h).unwrap().hp = 0;
    w.step(&ctx, &[]);
    assert!(!w.get(h).unwrap().alive, "the hero died");
    assert_eq!(w.players[0].deaths, 1);
    assert_eq!(w.players[0].gold, 1000, "all 1000 gold kept");
}

/// The Moonward base with only its two towers by the Sphere and the Sphere
/// left: every lower tower and barracks gone.
fn bare_moon_base(w: &mut World, f: &common::Fixture) {
    for u in w.units.iter_mut() {
        let k = f.rules.units[u.def as usize].kind;
        if u.team == MOON && ((k == Kind::Tower && u.tier < 4) || k == Kind::Barracks) {
            u.alive = false;
        }
    }
}

/// The person's report: middle-lane creeps took one tower by the Sphere,
/// then stood behind the Sphere and never attacked the other, so the match
/// could not end. The lane's last waypoint is the Sphere's centre, inside
/// its footprint; with the Sphere shielded, creeps pressed against it about
/// 1,000 map units from the far tower, out of their 500 to 800 reach. With
/// every other creep taken away each tick, the Sunward middle lane's creeps
/// alone must take both towers and the Sphere. A wave reaches the base
/// about 48 s after the horn (15,600 map units from (3000, 3000) to the
/// Sphere at 325 a second); the test allows ten waves, 300 s.
#[test]
fn middle_lane_creeps_alone_take_both_base_towers_and_the_sphere() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    bare_moon_base(&mut w, &f);
    let end = w.horn_at + 300 * TPS as u32;
    while w.tick < end && w.winner.is_none() {
        for u in w.units.iter_mut() {
            if f.rules.units[u.def as usize].kind == Kind::Creep && !(u.team == SUN && u.lane == 1) {
                u.alive = false;
            }
        }
        w.step(&ctx, &[]);
    }
    let towers = w
        .units
        .iter()
        .filter(|u| u.alive && u.team == MOON && f.rules.units[u.def as usize].kind == Kind::Tower)
        .count();
    assert_eq!(towers, 0, "{towers} Moonward towers still stand");
    assert_eq!(w.winner, Some(SUN), "the Sunward creeps took the Sphere");
}

/// The cause itself: a creep on its lane's last leg, east of the Sphere
/// where the first tower fell, with the other tower (at 13434, 14334, about
/// 1,010 map units away) out of its reach, walks to that tower and hits it
/// within 10 s (about 3 s of walking at 325 a second, then its attacks).
#[test]
fn a_creep_at_the_lane_s_end_walks_to_the_tower_it_may_still_hurt() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    while w.tick < w.horn_at {
        w.step(&ctx, &[]);
    }
    bare_moon_base(&mut w, &f);
    let base_tower = |w: &World, at: [i32; 2]| {
        w.units
            .iter()
            .find(|u| u.alive && u.team == MOON && u.tier == 4 && u.pos() == at)
            .expect("a tower by the Sphere")
            .id
    };
    let east = base_tower(&w, [14334, 13434]);
    let west = base_tower(&w, [13434, 14334]);
    w.get_mut(east).unwrap().alive = false;
    let melee = f.rules.unit("creep_melee_sun").unwrap();
    let c = w.spawn(&ctx, melee, SUN, [14420, 14100]);
    let last = f.rules.map.sides[SUN as usize].lanes["mid"].len() - 1;
    let whole = {
        let u = w.get_mut(c).unwrap();
        u.lane = 1;
        u.lane_name = 1;
        u.waypoint = last as u8;
        u.hp
    };
    let before = w.get(west).unwrap().hp;
    for _ in 0..(10 * TPS) {
        for u in w.units.iter_mut() {
            if f.rules.units[u.def as usize].kind == Kind::Creep && u.id != c {
                u.alive = false;
            }
        }
        // the tower's shots are not the question: the creep is kept whole
        w.get_mut(c).unwrap().hp = whole;
        w.step(&ctx, &[]);
    }
    assert!(
        w.get(west).unwrap().hp < before,
        "the creep hit the other tower: {} of {} health left",
        w.get(west).unwrap().hp / 1000,
        before / 1000
    );
}
