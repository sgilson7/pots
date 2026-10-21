//! Shiver (round 7, the roster's thirteenth hero) plays as DotA 6.72/6.73's
//! Ancient Apparition (Kaldr), per notes/LITERATURE.md section 8. Expected
//! values are worked out by hand from that section and data/rules.json's
//! tables: a hero has 150 health and 19 more for each point of strength,
//! 13 mana for each point of intelligence, and resists a quarter of spell
//! damage; a lane creep resists none and grows back half a point a second.

mod common;

use pots_core::ability::{self, slot_info, TAG_CHILL, TAG_TRACER};
use pots_core::combat;
use pots_core::orders::Cmd;
use pots_core::rules::{P, SUB, TPS};
use pots_core::stats;
use pots_core::world::{Ctx, Mod, Uid, World, M, MOON, NO_UNIT};

fn roster(f: &common::Fixture) -> usize {
    f.rules
        .heroes
        .iter()
        .position(|&u| f.rules.units[u].id == "shiver")
        .unwrap()
}

fn put(w: &mut World, id: Uid, at: P) {
    let u = w.get_mut(id).unwrap();
    u.x = at[0] * SUB;
    u.y = at[1] * SUB;
    u.path.clear();
}

fn creep(f: &common::Fixture, ctx: &Ctx, w: &mut World, at: P) -> Uid {
    let def = f
        .rules
        .units
        .iter()
        .position(|u| u.id == "creep_melee_moon")
        .unwrap();
    let c = w.spawn(ctx, def, MOON, at);
    // held where it stands (a lane creep would walk off to its lane)
    w.add_mod(
        c,
        Mod {
            kind: M::Root,
            src: NO_UNIT,
            until: 0,
            a: 0,
            b: 0,
            tag: 0,
        },
    );
    c
}

fn learn(w: &mut World, id: Uid, levels: [i32; 4]) {
    let u = w.get_mut(id).unwrap();
    for (s, l) in levels.iter().enumerate() {
        u.abil[s].level = *l;
    }
    u.mana = 100_000 * 1000;
}

fn open(f: &common::Fixture, w: &mut World, at: P, r: i32) -> P {
    w.cut_trees_in(&f.ctx(), at, r);
    at
}

fn run(w: &mut World, ctx: &Ctx, ticks: i32) {
    for _ in 0..ticks {
        w.step(ctx, &[]);
    }
}

/// Shiver at the middle of open ground with its levels, standing still
/// after each order so its own attacks add nothing.
fn shiver(f: &common::Fixture, w: &mut World, levels: [i32; 4]) -> (Uid, P) {
    let ctx = f.ctx();
    let s = w.add_hero(&ctx, 0, roster(f));
    // a seat someone holds, so its side keeps its sight
    w.players[0].present = true;
    learn(w, s, levels);
    // flat high ground in the jungle, away from every building: the low
    // river at the middle would hide higher ground from Shiver's side
    let spot = open(f, w, [9000, 5000], 1600);
    put(w, s, spot);
    (s, spot)
}

fn order(w: &mut World, ctx: &Ctx, cmd: Cmd) {
    // one tick first, so Shiver's side sees what stands near it
    w.step(ctx, &[]);
    w.step(ctx, &[(0, cmd)]);
    for _ in 0..3 {
        w.step(ctx, &[]);
    }
    w.step(ctx, &[(0, Cmd::Stand)]);
}

#[test]
fn shiver_can_be_chosen_with_its_level_one_numbers() {
    // 18 strength: 150 + 18 × 19 = 492 health; 25 intelligence: 25 × 13 =
    // 325 mana; damage 19–29 plus 25 for intelligence, its main attribute
    let f = common::fixture();
    assert_eq!(f.rules.roster, 13, "ten heroes, Hackle, Chipper and Shiver");
    assert_eq!(f.rules.units[f.rules.heroes[12]].id, "shiver");
    let ctx = f.ctx();
    let mut w = f.world(1);
    let s = w.add_hero(&ctx, 0, 12);
    let st = stats::of(&ctx, &w, s);
    assert_eq!((st.max_hp, st.max_mana), (492, 325));
    assert_eq!(st.damage, [44, 54]);
    assert_eq!((st.range, st.bat, st.move_speed), (600, 170, 295));
    assert_eq!(stats::missile(&ctx, &w, s), 1250, "a ranged attacker");
}

#[test]
fn frozen_ground_hits_four_times_then_stuns_an_enemy_that_stayed() {
    // level 1: four hits of 37.5 by 3.4 s, 150 in all; a stun of 1.25 s at 4 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(2);
    let (s, spot) = shiver(&f, &mut w, [1, 0, 0, 0]);
    let c = creep(&f, &ctx, &mut w, [spot[0] + 500, spot[1]]);
    let full = w.get(c).unwrap().hp;
    order(&mut w, &ctx, Cmd::Cast(0, c, [0, 0]));
    run(&mut w, &ctx, 3 * TPS);
    assert!(!w.get(c).unwrap().has(M::Stun), "not yet at 3 s");
    run(&mut w, &ctx, TPS + 5);
    let v = w.get(c).unwrap();
    assert!(v.has(M::Stun), "frozen at 4 s");
    // 150 lost, less what it grew back over about 4 s (0.5 a second)
    let lost = full - v.hp;
    assert!((148_000..=150_000).contains(&lost), "lost {lost}");
    let _ = s;
}

#[test]
fn frozen_ground_breaks_once_the_enemy_is_740_from_where_it_stood() {
    // one hit (37.5 at 0.85 s), then it is carried 800 away: no stun, no more hits
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(3);
    let (_, spot) = shiver(&f, &mut w, [1, 0, 0, 0]);
    let at = [spot[0] + 500, spot[1]];
    let c = creep(&f, &ctx, &mut w, at);
    let full = w.get(c).unwrap().hp;
    order(&mut w, &ctx, Cmd::Cast(0, c, [0, 0]));
    run(&mut w, &ctx, TPS);
    put(&mut w, c, [at[0], at[1] + 800]);
    run(&mut w, &ctx, 4 * TPS);
    let v = w.get(c).unwrap();
    assert!(!v.has(M::Stun), "broken free");
    let lost = full - v.hp;
    assert!((35_000..=37_500).contains(&lost), "one hit only: lost {lost}");
    assert!(w.zones.is_empty());
}

#[test]
fn snow_whirl_slows_and_lowers_magic_resistance_inside_it_only() {
    // level 1: 18% slower; a hero's 25% magic resistance less 10%: 100 −
    // 75 × 110 / 100 = 18 (rounded down from 17.5 the integer way)
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(4);
    let (_, spot) = shiver(&f, &mut w, [0, 1, 0, 0]);
    let foe = w.add_hero(&ctx, 5, 1);
    let at = [spot[0] + 900, spot[1]];
    put(&mut w, foe, at);
    let before = stats::of(&ctx, &w, foe);
    assert_eq!(before.magic_resist, 25);
    order(&mut w, &ctx, Cmd::Cast(1, NO_UNIT, at));
    w.step(&ctx, &[(5, Cmd::Hold)]);
    run(&mut w, &ctx, 3);
    let inside = stats::of(&ctx, &w, foe);
    assert_eq!(inside.magic_resist, 18);
    // 18% off, the integer way: speed − speed × 18 / 100
    assert_eq!(
        inside.move_speed,
        before.move_speed - before.move_speed * 18 / 100
    );
    // out of the whirl, both wear off within a fifth of a second
    put(&mut w, foe, [at[0] + 600, at[1]]);
    run(&mut w, &ctx, TPS / 2);
    let out = stats::of(&ctx, &w, foe);
    assert_eq!((out.magic_resist, out.move_speed), (25, before.move_speed));
}

#[test]
fn frost_talons_add_magic_damage_to_an_allys_next_attacks_on_heroes_only() {
    // level 1: two charges of 40 magic damage, 30 after a hero's quarter;
    // the ally attacks 15% slower while a charge is left
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    let (_, spot) = shiver(&f, &mut w, [0, 0, 1, 0]);
    let ally = w.add_hero(&ctx, 1, 1);
    put(&mut w, ally, [spot[0] + 300, spot[1]]);
    let speed = stats::of(&ctx, &w, ally).attack_speed;
    order(&mut w, &ctx, Cmd::Cast(2, NO_UNIT, [spot[0] + 300, spot[1]]));
    let charges = |w: &World, id: Uid| {
        w.get(id)
            .unwrap()
            .mods
            .iter()
            .find(|m| m.kind == M::Tag && m.b == TAG_CHILL)
            .map(|m| m.a)
    };
    assert_eq!(charges(&w, ally), Some(2));
    assert_eq!(stats::of(&ctx, &w, ally).attack_speed, speed - 15);
    let foe = w.add_hero(&ctx, 5, 2);
    let c = creep(&f, &ctx, &mut w, [spot[0] + 400, spot[1] + 400]);
    // a hit on a creep spends nothing and adds nothing
    let creep_full = w.get(c).unwrap().hp;
    ability::after_attack(&mut w, &ctx, ally, c, 0);
    assert_eq!((charges(&w, ally), w.get(c).unwrap().hp), (Some(2), creep_full));
    let full = w.get(foe).unwrap().hp;
    ability::after_attack(&mut w, &ctx, ally, foe, 0);
    assert_eq!(full - w.get(foe).unwrap().hp, 30_000);
    assert_eq!(charges(&w, ally), Some(1));
    ability::after_attack(&mut w, &ctx, ally, foe, 0);
    assert_eq!(full - w.get(foe).unwrap().hp, 60_000);
    assert_eq!(charges(&w, ally), None, "both charges spent");
    assert_eq!(
        stats::of(&ctx, &w, ally).attack_speed,
        speed,
        "and the slowness gone"
    );
}

/// Casts Hailstar east from Shiver and lets the tracer fly `ticks` ticks
/// from the tick it set off; returns where the tracer is.
fn tracer_flies(w: &mut World, ctx: &Ctx, s: Uid, spot: P, ticks: u32) -> P {
    w.step(ctx, &[(0, Cmd::Cast(3, NO_UNIT, [spot[0] + 5000, spot[1]]))]);
    while w.zones.is_empty() {
        w.step(ctx, &[]);
    }
    w.step(ctx, &[(0, Cmd::Stand)]);
    let start = w.zones[0].start;
    while w.tick < start + ticks {
        w.step(ctx, &[]);
    }
    assert!(w.get(s).unwrap().mods.iter().any(|m| m.b == TAG_TRACER));
    w.zones[0].at
}

#[test]
fn hailstar_flies_at_1500_and_takes_the_slot_for_release() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(6);
    let (s, spot) = shiver(&f, &mut w, [0, 0, 0, 1]);
    // one second: 1500 east, 50 a tick
    let at = tracer_flies(&mut w, &ctx, s, spot, TPS as u32);
    assert_eq!(at, [spot[0] + 1500, spot[1]]);
    assert_eq!(
        f.rules.abilities[slot_info(&ctx, &w, s, 3).unwrap().0].id,
        "ice_blast_release"
    );
    // it waits at the map's edge for Release
    run(&mut w, &ctx, 10 * TPS);
    // to within a map unit: the edge is found in millionths of the way
    let end = w.zones[0].at;
    let size = f.rules.map.size;
    assert!(
        (size - 1..=size).contains(&end[0]) && end[1] == spot[1],
        "{end:?}"
    );
    assert_eq!(w.zones[0].a, 0, "still the tracer");
}

#[test]
fn released_hailstar_bursts_wider_the_longer_it_flew_and_frostbites_the_path() {
    // two seconds of flight: 225 + 50 × 2 = 325 across (give or take one
    // tick for the order); 3000 away, so the hail takes its longest, 2 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    let (s, spot) = shiver(&f, &mut w, [0, 0, 0, 1]);
    let at = tracer_flies(&mut w, &ctx, s, spot, 2 * TPS as u32 - 1);
    let hit = creep(&f, &ctx, &mut w, [at[0] + 200, at[1] + 100]);
    let path = creep(&f, &ctx, &mut w, [spot[0] + 1500, spot[1] + 250]);
    let clear = creep(&f, &ctx, &mut w, [spot[0] + 1500, spot[1] + 600]);
    let full = w.get(hit).unwrap().hp;
    w.step(&ctx, &[(0, Cmd::Cast(3, NO_UNIT, [0, 0]))]);
    w.step(&ctx, &[]);
    let z = &w.zones[0];
    assert_eq!(z.a, 1, "released: the hail is falling");
    assert!((323..=327).contains(&z.radius), "radius {}", z.radius);
    let lands = z.next;
    assert!((2 * TPS as u32 - 1..=2 * TPS as u32 + 1).contains(&(lands - z.start)));
    assert!(!w.get(s).unwrap().mods.iter().any(|m| m.b == TAG_TRACER));
    while w.tick <= lands {
        w.step(&ctx, &[]);
    }
    // 250 magic damage at the burst; frostbite from the burst and the path
    let v = w.get(hit).unwrap();
    assert!(ability::no_heal(v));
    assert!(full - v.hp >= 250_000, "lost {}", full - v.hp);
    assert!(ability::no_heal(w.get(path).unwrap()), "275 from the hail's path");
    assert_eq!(w.get(path).unwrap().hp, full, "frostbite only: no burst damage");
    assert!(
        !ability::no_heal(w.get(clear).unwrap()),
        "600 from the path: untouched"
    );
}

#[test]
fn frostbite_stops_healing_hurts_each_second_and_shatters_below_ten_percent() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(8);
    let (s, spot) = shiver(&f, &mut w, [0, 0, 0, 1]);
    // 1500 out: past the reach of Shiver's own attacks
    let at = tracer_flies(&mut w, &ctx, s, spot, TPS as u32);
    let c = creep(&f, &ctx, &mut w, at);
    let other = creep(&f, &ctx, &mut w, [at[0], at[1] + 1500]);
    w.step(&ctx, &[(0, Cmd::Cast(3, NO_UNIT, [0, 0]))]);
    while !ability::no_heal(w.get(c).unwrap()) {
        w.step(&ctx, &[]);
    }
    // one second: 12.5 lost and nothing grown back (12 480 by the tick)
    let before = w.get(c).unwrap().hp;
    run(&mut w, &ctx, TPS);
    let lost = before - w.get(c).unwrap().hp;
    assert!((12_400..=12_600).contains(&lost), "lost {lost}");
    let now = w.get(c).unwrap().hp;
    combat::heal(&mut w, &ctx, c, 100);
    assert_eq!(w.get(c).unwrap().hp, now, "no heal takes hold");
    // 550 health: under 10% (55) it shatters; another creep at 50 lives
    w.get_mut(c).unwrap().hp = 54_000;
    w.get_mut(other).unwrap().hp = 50_000;
    run(&mut w, &ctx, 2);
    assert!(!w.get(c).unwrap().alive, "shattered");
    assert!(w.get(other).unwrap().alive, "no frostbite, no shatter");
}

#[test]
fn a_frostbitten_unit_at_its_own_fountain_loses_the_frostbite() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(9);
    let (s, spot) = shiver(&f, &mut w, [0, 0, 0, 1]);
    // 1500 out: past the reach of Shiver's own attacks
    let at = tracer_flies(&mut w, &ctx, s, spot, TPS as u32);
    let c = creep(&f, &ctx, &mut w, at);
    w.step(&ctx, &[(0, Cmd::Cast(3, NO_UNIT, [0, 0]))]);
    while !ability::no_heal(w.get(c).unwrap()) {
        w.step(&ctx, &[]);
    }
    put(&mut w, c, f.rules.map.sides[MOON as usize].fountain);
    run(&mut w, &ctx, 2);
    assert!(!ability::no_heal(w.get(c).unwrap()));
}

// ------------------------------------------------------ computer players

#[test]
fn a_computer_shiver_releases_its_hailstar_over_an_enemy_hero() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(10);
    let (s, spot) = shiver(&f, &mut w, [0, 0, 0, 1]);
    let foe = w.add_hero(&ctx, 5, 6);
    // within the 900 at which every computer hero casts at an enemy hero
    put(&mut w, foe, [spot[0] + 800, spot[1]]);
    w.get_mut(foe).unwrap().hp = 1_000_000_000;
    w.players[0].bot = Some(2);
    w.players[0].present = true;
    w.get_mut(s).unwrap().hero.as_mut().unwrap().points = 0;
    w.step(&ctx, &[(5, Cmd::Hold)]);
    for _ in 0..4 * TPS {
        w.step(&ctx, &[]);
        if ability::no_heal(w.get(foe).unwrap()) {
            break;
        }
    }
    assert!(
        ability::no_heal(w.get(foe).unwrap()),
        "cast, released over the foe, and the foe frostbitten"
    );
}

// ------------------------------------------------------------- its look

#[test]
fn shiver_floats_its_body_rising_and_falling_14_hundredths() {
    // data/models.json: hover 14 hundredths of a scene unit; the body is
    // lowest at a quarter turn back (sin = −1) and highest a half turn on
    use pots_core::fx::{self, ONE};
    use pots_core::geom::Affine;
    use pots_core::models::{pose_parts, Models, Pose};
    let m = Models::load(include_str!("../../../data/models.json")).unwrap();
    let shiver = &m.list[m.find("hero_shiver").unwrap()];
    let ironvow = &m.list[m.find("hero_ironvow").unwrap()];
    let lift = |model, spin: i32| {
        let mut y = None;
        let pose = Pose { spin, ..Pose::STILL };
        pose_parts(model, &pose, &Affine::IDENTITY, |_, a| {
            y.get_or_insert(a.t.y);
        });
        y.unwrap()
    };
    // spin × 2 is the bob's angle: −90° at spin −45°, +90° at spin 45°
    let low = lift(shiver, -fx::TURN / 8);
    let high = lift(shiver, fx::TURN / 8);
    let rise = high - low;
    assert!((rise - 14 * ONE / 100).abs() <= 2, "rose {rise}");
    assert_eq!(
        lift(ironvow, -fx::TURN / 8),
        lift(ironvow, fx::TURN / 8),
        "a standing hero stays put"
    );
}
