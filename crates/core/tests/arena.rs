//! The arena (round 8, after SMITE 2's Arena; notes/LITERATURE.md section
//! 10). Each side starts with 500 tickets and loses 5 for each hero that
//! dies, 1 for each of its creeps a hero kills (none once it is down to its
//! last 10), 1 for each enemy creep that walks into its portal and 15 for a
//! brute; at none it loses. Waves of 3 melee, 3 ranged and 1 siege creep
//! leave every 20 s by one lane, then the other; each side sends a brute
//! every 3 minutes. A cleared camp gives its clearer a buff for 2 minutes
//! and returns 3 minutes later. The numbers are worked from those rules.

mod common;

use pots_core::combat::{self, Dmg};
use pots_core::game::{Files, Game};
use pots_core::rules::{Kind, P, SUB, TPS};
use pots_core::world::{Mod, Uid, World, M, MOON, NO_UNIT, SUN};

fn files() -> Files<'static> {
    Files {
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

fn put(w: &mut World, id: Uid, at: P) {
    let u = w.get_mut(id).unwrap();
    u.x = at[0] * SUB;
    u.y = at[1] * SUB;
    u.path.clear();
}

fn spawn(f: &common::Fixture, w: &mut World, name: &str, team: u8, at: P) -> Uid {
    let def = f.rules.unit(name).unwrap();
    let id = w.spawn(&f.ctx(), def, team, at);
    w.add_mod(
        id,
        Mod {
            kind: M::Root,
            src: NO_UNIT,
            until: 0,
            a: 0,
            b: 0,
            tag: 0,
        },
    );
    id
}

fn kill(f: &common::Fixture, w: &mut World, by: Uid, v: Uid) {
    combat::damage(w, &f.ctx(), by, v, 100_000, Dmg::Pure);
    w.step(&f.ctx(), &[]);
}

/// Sunward lane creeps and computer players kept out of the way: the
/// rules under test act alone.
fn quiet(f: &common::Fixture, seed: u64) -> World {
    let mut w = f.arena_world(seed);
    // no horn, so no waves and no periodic gold; no camps
    w.horn_at = 100_000_000;
    w.camps_back.iter_mut().for_each(|t| *t = 0);
    w
}

#[test]
fn an_arena_match_starts_with_500_tickets_no_towers_and_portals_nothing_can_hurt() {
    let f = common::arena_fixture();
    let mut w = quiet(&f, 1);
    assert_eq!(w.tickets, [500, 500]);
    w.step(&f.ctx(), &[]);
    w.step(&f.ctx(), &[]);
    let kinds: Vec<Kind> = w
        .units
        .iter()
        .filter(|u| u.alive)
        .map(|u| f.rules.units[u.def as usize].kind)
        .collect();
    assert!(
        !kinds.contains(&Kind::Tower) && !kinds.contains(&Kind::Barracks),
        "no towers, no barracks"
    );
    let spheres: Vec<&pots_core::world::Unit> = w
        .units
        .iter()
        .filter(|u| u.alive && f.rules.units[u.def as usize].kind == Kind::Sphere)
        .collect();
    assert_eq!(spheres.len(), 2);
    assert!(
        spheres.iter().all(|s| s.has(M::Invulnerable)),
        "a portal cannot be hurt"
    );
}

#[test]
fn a_hero_s_death_costs_its_side_five_tickets() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = quiet(&f, 2);
    let mine = w.add_hero(&ctx, 0, 0);
    let foe = w.add_hero(&ctx, 5, 1);
    put(&mut w, mine, [4000, 4000]);
    put(&mut w, foe, [4300, 4000]);
    kill(&f, &mut w, foe, mine);
    assert_eq!(w.tickets, [495, 500]);
}

#[test]
fn a_creep_a_hero_kills_costs_one_ticket_and_none_once_down_to_ten() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = quiet(&f, 3);
    let foe = w.add_hero(&ctx, 5, 1);
    put(&mut w, foe, [4300, 4000]);
    let c = spawn(&f, &mut w, "creep_melee_sun", SUN, [4000, 4000]);
    kill(&f, &mut w, foe, c);
    assert_eq!(w.tickets[SUN as usize], 499, "a hero's last hit");
    let killer = spawn(&f, &mut w, "creep_melee_moon", MOON, [4300, 4200]);
    let c = spawn(&f, &mut w, "creep_melee_sun", SUN, [4000, 4200]);
    kill(&f, &mut w, killer, c);
    assert_eq!(w.tickets[SUN as usize], 499, "a creep's last hit costs nothing");
    w.tickets[SUN as usize] = 10;
    let c = spawn(&f, &mut w, "creep_melee_sun", SUN, [4000, 4400]);
    kill(&f, &mut w, foe, c);
    assert_eq!(w.tickets[SUN as usize], 10, "the last ten");
}

#[test]
fn a_creep_that_walks_into_the_portal_costs_one_and_a_brute_fifteen() {
    let f = common::arena_fixture();
    let mut w = quiet(&f, 4);
    let portal = pots_core::arena::sphere_at(&f.ctx(), SUN).unwrap();
    let c = spawn(
        &f,
        &mut w,
        "creep_melee_moon",
        MOON,
        [portal[0] + 200, portal[1] + 200],
    );
    w.step(&f.ctx(), &[]);
    assert!(
        !w.get(c).map(|u| u.alive).unwrap_or(false),
        "gone into the portal"
    );
    assert_eq!(w.tickets, [499, 500]);
    let b = spawn(&f, &mut w, "brute_moon", MOON, [portal[0] + 100, portal[1]]);
    w.step(&f.ctx(), &[]);
    assert!(!w.get(b).map(|u| u.alive).unwrap_or(false));
    assert_eq!(w.tickets, [484, 500]);
    // one that stops short of 450 is still in the field
    let far = spawn(
        &f,
        &mut w,
        "creep_melee_moon",
        MOON,
        [portal[0] + 400, portal[1] + 300],
    );
    w.step(&f.ctx(), &[]);
    assert!(w.get(far).unwrap().alive);
}

#[test]
fn a_side_with_no_tickets_left_loses() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = quiet(&f, 5);
    let mine = w.add_hero(&ctx, 0, 0);
    let foe = w.add_hero(&ctx, 5, 1);
    put(&mut w, mine, [4000, 4000]);
    put(&mut w, foe, [4300, 4000]);
    w.tickets[SUN as usize] = 4;
    kill(&f, &mut w, foe, mine);
    assert_eq!(w.tickets[SUN as usize], 0);
    assert_eq!(w.winner, Some(MOON));
}

#[test]
fn waves_of_seven_leave_every_20_s_by_one_lane_then_the_other_and_a_brute_every_3_minutes() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = f.arena_world(6);
    w.horn_at = 10;
    let creeps = |w: &World, lane: u8, team: u8| {
        w.units
            .iter()
            .filter(|u| {
                u.alive
                    && u.team == team
                    && f.rules.units[u.def as usize].kind == Kind::Creep
                    && u.lane == lane
            })
            .count()
    };
    for _ in 0..11 {
        w.step(&ctx, &[]);
    }
    assert_eq!(
        (creeps(&w, 0, SUN), creeps(&w, 2, SUN), creeps(&w, 0, MOON)),
        (7, 0, 7),
        "the first by the top lane"
    );
    for _ in 0..20 * TPS {
        w.step(&ctx, &[]);
    }
    assert_eq!(creeps(&w, 2, SUN), 7, "the next by the bottom lane");
    let brutes = |w: &World| {
        w.units
            .iter()
            .filter(|u| u.alive && f.rules.units[u.def as usize].id.starts_with("brute_"))
            .count()
    };
    assert_eq!(brutes(&w), 0);
    while w.tick <= 10 + 180 * TPS as u32 + 1 {
        w.step(&ctx, &[]);
    }
    assert_eq!(brutes(&w), 2, "one for each side at 3 minutes");
}

#[test]
fn clearing_a_camp_gives_its_buff_for_two_minutes_and_the_camp_returns_after_three() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = quiet(&f, 7);
    w.camps_back = vec![1; f.rules.map.camps.len()];
    w.step(&ctx, &[]);
    w.step(&ctx, &[]);
    let k = f.rules.map.camps.iter().position(|c| c.buff == "red").unwrap();
    let tag = k as u8 + 1;
    let hero = w.add_hero(&ctx, 0, 0);
    let camp = f.rules.map.camps[k].at;
    put(&mut w, hero, [camp[0] + 150, camp[1]]);
    let members: Vec<Uid> = w
        .units
        .iter()
        .filter(|u| u.alive && u.camp == tag)
        .map(|u| u.id)
        .collect();
    assert!(!members.is_empty(), "the camp is up");
    for v in members {
        kill(&f, &mut w, hero, v);
    }
    let cleared = w.tick;
    let red: Vec<&Mod> = w
        .get(hero)
        .unwrap()
        .mods
        .iter()
        .filter(|m| m.kind == M::DamagePct)
        .collect();
    assert_eq!(red.len(), 1, "the red buff");
    assert_eq!(red[0].a, 20);
    assert_eq!(red[0].until, cleared - 1 + 120 * TPS as u32);
    assert_eq!(w.camps_back[k], cleared - 1 + 180 * TPS as u32);
    put(&mut w, hero, [500, 500]);
    while w.tick <= w.camps_back[k] {
        w.step(&ctx, &[]);
    }
    assert!(
        w.units.iter().any(|u| u.alive && u.camp == tag),
        "back after 3 minutes"
    );
}

#[test]
fn the_custom_match_switches_to_the_arena_and_its_map_and_back() {
    let mut g = Game::new(&files(), 9).unwrap();
    let normal = g.rules.map.size;
    g.act("screen", "arcade").unwrap();
    g.act("setup.mode", "").unwrap();
    assert!(g.setup.aram);
    g.act("setup.mode", "").unwrap();
    assert!(g.setup.arena && !g.setup.aram);
    g.act("custom", "").unwrap();
    g.act("pick", "13").unwrap();
    g.act("start", "").unwrap();
    g.frame(0);
    let m = g.m.as_ref().unwrap();
    assert!(m.world.arena && g.on_arena);
    assert_eq!(g.rules.map.size, 8192);
    assert_eq!(m.world.tickets, [500, 500]);
    let me = m.world.players[0].hero;
    assert_eq!(
        m.world.get(me).map(|u| u.def as usize),
        Some(g.rules.heroes[13]),
        "any hero"
    );
    // the top bar shows each side's tickets where the kills would stand
    fn scores(n: &pots_core::ui::Node, out: &mut Vec<(String, String)>) {
        if n.c.starts_with("score ") {
            out.push((n.c.clone(), n.x.clone()));
        }
        for k in &n.n {
            scores(k, out);
        }
    }
    let mut seen = Vec::new();
    scores(&pots_core::ui::view(&g).root, &mut seen);
    assert_eq!(
        seen,
        vec![
            ("score sun".to_string(), "500".to_string()),
            ("score moon".to_string(), "500".to_string())
        ]
    );
    g.act("quit", "").unwrap();
    assert!(!g.on_arena);
    assert_eq!(g.rules.map.size, normal, "the normal map is back");
    g.act("setup.mode", "").unwrap();
    assert!(!g.setup.arena && !g.setup.aram, "and round to Normal");
}

#[test]
fn heroes_appear_within_reach_of_their_shop_and_every_computer_player_goes_out_to_fight() {
    // the main map's shop is 424 from where heroes appear; the reach is 600
    let f = common::arena_fixture();
    let ctx = f.ctx();
    for team in [SUN, MOON] {
        let side = &f.rules.map.sides[team as usize];
        let shop = f
            .rules
            .map
            .shops
            .iter()
            .find(|(k, _)| *k == format!("home{team}"))
            .unwrap()
            .1;
        assert!(
            pots_core::world::within(side.spawn, shop, f.rules.t.shop_range),
            "side {team}"
        );
    }
    // the arena has no middle lane: a seat meant for it takes another
    let mut w = f.arena_world(11);
    for p in 0..pots_core::world::PLAYERS {
        w.players[p].bot = Some(2);
        w.players[p].present = true;
    }
    let mut out = [false; pots_core::world::PLAYERS];
    for _ in 0..(w.horn_at as i32 + 60 * TPS) {
        w.step(&ctx, &[]);
        for (k, p) in w.players.iter().enumerate() {
            let team = p.team as usize;
            if let Some(u) = w.get(p.hero) {
                out[k] |= !pots_core::world::within(u.pos(), f.rules.map.sides[team].fountain, 2000);
            }
        }
    }
    assert_eq!(out, [true; pots_core::world::PLAYERS], "every hero left its base");
}
