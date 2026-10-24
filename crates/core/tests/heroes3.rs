//! Two heroes added in round 8 (the roster's fourteenth and fifteenth):
//! Tallow plays as DotA 6.72/6.73's Vengeful Spirit and Skipjack as its
//! Priestess of the Moon (Mirana), per notes/LITERATURE.md section 9.
//! Expected values are worked out by hand from that section and
//! data/rules.json's tables: a hero has 150 health and 19 more for each
//! point of strength, 13 mana for each point of intelligence, and resists a
//! quarter of spell damage; a lane creep resists none and grows back half a
//! point a second.

mod common;

use pots_core::orders::Cmd;
use pots_core::rules::{P, SUB, TPS};
use pots_core::stats;
use pots_core::world::{Ctx, Mod, Uid, World, M, MOON, NO_UNIT, SUN};

fn roster(f: &common::Fixture, id: &str) -> usize {
    f.rules
        .heroes
        .iter()
        .position(|&u| f.rules.units[u].id == id)
        .unwrap()
}

fn put(w: &mut World, id: Uid, at: P) {
    let u = w.get_mut(id).unwrap();
    u.x = at[0] * SUB;
    u.y = at[1] * SUB;
    u.path.clear();
}

fn rooted(w: &mut World, id: Uid) {
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
}

/// A lane creep of `team`, held where it stands (it would walk off to its lane).
fn creep(f: &common::Fixture, ctx: &Ctx, w: &mut World, team: u8, at: P) -> Uid {
    let name = if team == MOON {
        "creep_melee_moon"
    } else {
        "creep_melee_sun"
    };
    let def = f.rules.units.iter().position(|u| u.id == name).unwrap();
    let c = w.spawn(ctx, def, team, at);
    rooted(w, c);
    c
}

fn learn(w: &mut World, id: Uid, levels: [i32; 4]) {
    let u = w.get_mut(id).unwrap();
    for (s, l) in levels.iter().enumerate() {
        u.abil[s].level = *l;
    }
    u.mana = 100_000 * 1000;
}

fn run(w: &mut World, ctx: &Ctx, ticks: i32) {
    for _ in 0..ticks {
        w.step(ctx, &[]);
    }
}

/// The hero `id` for seat 0 on flat open jungle ground (the low river at
/// the middle would hide higher ground from its side), with its levels.
fn hero(f: &common::Fixture, w: &mut World, id: &str, levels: [i32; 4]) -> (Uid, P) {
    let ctx = f.ctx();
    let h = w.add_hero(&ctx, 0, roster(f, id));
    w.players[0].present = true;
    learn(w, h, levels);
    let spot = [9000, 5000];
    w.cut_trees_in(&ctx, spot, 1800);
    put(w, h, spot);
    if let Some(u) = w.get_mut(h) {
        u.facing = 0;
    }
    (h, spot)
}

fn order(w: &mut World, ctx: &Ctx, cmd: Cmd) {
    // one tick first, so the side sees what stands near it
    w.step(ctx, &[]);
    w.step(ctx, &[(0, cmd)]);
    // through the cast point, then standing, so its own attacks add nothing
    let me = w.players[0].hero;
    for _ in 0..TPS {
        let busy = w.get(me).map(|u| {
            matches!(u.act, pots_core::world::Act::Cast { .. })
                || matches!(u.order, pots_core::world::Order::Cast(..))
        });
        if busy != Some(true) {
            break;
        }
        w.step(ctx, &[]);
    }
    w.step(ctx, &[(0, Cmd::Stand)]);
}

fn stunned_for(w: &World, id: Uid) -> i32 {
    w.get(id)
        .unwrap()
        .mods
        .iter()
        .filter(|m| m.kind == M::Stun)
        .map(|m| m.until as i32 - w.tick as i32)
        .max()
        .unwrap_or(0)
}

// ------------------------------------------------------------ Tallow

#[test]
fn tallow_and_skipjack_can_be_chosen_with_their_level_one_numbers() {
    // Tallow: 16 strength, 150 + 16 × 19 = 454; 15 intelligence, 195 mana;
    // damage 12–26 plus 27 agility. Skipjack: 17 strength, 473; 17
    // intelligence, 221; damage 18–29 plus 20 agility.
    let f = common::fixture();
    assert_eq!(f.rules.units[f.rules.heroes[13]].id, "tallow");
    assert_eq!(f.rules.units[f.rules.heroes[14]].id, "skipjack");
    let ctx = f.ctx();
    let mut w = f.world(1);
    let t = w.add_hero(&ctx, 0, 13);
    let s = w.add_hero(&ctx, 1, 14);
    let a = stats::of(&ctx, &w, t);
    assert_eq!((a.max_hp, a.max_mana, a.damage), (454, 195, [39, 53]));
    assert_eq!((a.range, a.move_speed), (400, 295));
    let b = stats::of(&ctx, &w, s);
    assert_eq!((b.max_hp, b.max_mana, b.damage), (473, 221, [38, 49]));
    assert_eq!((b.range, b.move_speed), (600, 300));
}

#[test]
fn ember_dart_damages_and_stuns_one_enemy() {
    // level 1: 100 magic damage, a stun of 1.45 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(2);
    let (_, spot) = hero(&f, &mut w, "tallow", [1, 0, 0, 0]);
    let c = creep(&f, &ctx, &mut w, MOON, [spot[0] + 300, spot[1]]);
    let full = w.get(c).unwrap().hp;
    order(&mut w, &ctx, Cmd::Cast(0, c, [0, 0]));
    // the cast point (0.3 s) and 300 at 1250 a second
    run(&mut w, &ctx, 12);
    let lost = full - w.get(c).unwrap().hp;
    assert!((99_000..=100_000).contains(&lost), "lost {lost}");
    let held = stunned_for(&w, c);
    assert!(
        held > TPS * 120 / 100 && held <= TPS * 145 / 100,
        "stunned {held} ticks"
    );
}

#[test]
fn dusk_wave_takes_health_and_armor_from_each_enemy_it_passes() {
    // level 4: 75 health (not damage: magic resistance does not cut it), 5 armor for 20 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(3);
    let (_, spot) = hero(&f, &mut w, "tallow", [0, 4, 0, 0]);
    let near = creep(&f, &ctx, &mut w, MOON, [spot[0] + 400, spot[1]]);
    let far = creep(&f, &ctx, &mut w, MOON, [spot[0] + 1300, spot[1] + 100]);
    let past = creep(&f, &ctx, &mut w, MOON, [spot[0] + 1700, spot[1]]);
    let hp: Vec<i32> = [near, far, past].iter().map(|&c| w.get(c).unwrap().hp).collect();
    order(&mut w, &ctx, Cmd::Cast(1, NO_UNIT, [spot[0] + 1000, spot[1]]));
    run(&mut w, &ctx, TPS);
    for (i, &c) in [near, far].iter().enumerate() {
        let lost = hp[i] - w.get(c).unwrap().hp;
        assert!((74_000..=75_000).contains(&lost), "creep {i} lost {lost}");
        let armor: i32 = w
            .get(c)
            .unwrap()
            .mods
            .iter()
            .filter(|m| m.kind == M::Armor)
            .map(|m| m.a)
            .sum();
        assert_eq!(armor, -500, "creep {i}");
    }
    assert_eq!(w.get(past).unwrap().hp, hp[2], "the wave ends at 1400");
}

#[test]
fn lamplight_raises_every_ally_s_damage_by_a_share_of_its_base() {
    // level 4: 36% of a Sunward melee creep's 19–23 (21 on average) is 7
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(4);
    let (_, spot) = hero(&f, &mut w, "tallow", [0, 0, 4, 0]);
    let ally = creep(&f, &ctx, &mut w, SUN, [spot[0] + 300, spot[1]]);
    let foe = creep(&f, &ctx, &mut w, MOON, [spot[0] - 300, spot[1]]);
    let out = creep(&f, &ctx, &mut w, SUN, [spot[0], spot[1] + 1100]);
    run(&mut w, &ctx, 3);
    let bonus = |w: &World, id: Uid| -> i32 {
        w.get(id)
            .unwrap()
            .mods
            .iter()
            .filter(|m| m.kind == M::Damage)
            .map(|m| m.a)
            .sum()
    };
    assert_eq!(bonus(&w, ally), 7);
    assert_eq!(bonus(&w, foe), 0, "not enemies");
    assert_eq!(bonus(&w, out), 0, "not past 900");
}

#[test]
fn moth_switch_trades_places_with_a_hero_even_one_immune_to_spells() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    let (t, spot) = hero(&f, &mut w, "tallow", [0, 0, 0, 1]);
    let foe = w.add_hero(&ctx, 5, 0);
    w.players[5].present = true;
    let there = [spot[0] + 500, spot[1]];
    put(&mut w, foe, there);
    rooted(&mut w, foe);
    w.add_mod(
        foe,
        Mod {
            kind: M::SpellImmune,
            src: NO_UNIT,
            until: 0,
            a: 0,
            b: 0,
            tag: 0,
        },
    );
    // a creep is not a hero: no swap
    let c = creep(&f, &ctx, &mut w, MOON, [spot[0] - 300, spot[1]]);
    order(&mut w, &ctx, Cmd::Cast(3, c, [0, 0]));
    run(&mut w, &ctx, TPS);
    assert_eq!(w.get(t).unwrap().pos(), spot, "a creep cannot be swapped");
    order(&mut w, &ctx, Cmd::Cast(3, foe, [0, 0]));
    run(&mut w, &ctx, TPS);
    assert_eq!(w.get(t).unwrap().pos(), there);
    assert_eq!(w.get(foe).unwrap().pos(), spot);
}

// ------------------------------------------------------------ Skipjack

#[test]
fn star_shower_strikes_every_enemy_near_and_one_close_by_again_for_half() {
    // level 1: 75 to each enemy within 625; one within 175 takes 37 more
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(6);
    let (_, spot) = hero(&f, &mut w, "skipjack", [1, 0, 0, 0]);
    let close = creep(&f, &ctx, &mut w, MOON, [spot[0] + 120, spot[1]]);
    let wide = creep(&f, &ctx, &mut w, MOON, [spot[0] - 500, spot[1]]);
    let out = creep(&f, &ctx, &mut w, MOON, [spot[0], spot[1] + 800]);
    let hp: Vec<i32> = [close, wide, out].iter().map(|&c| w.get(c).unwrap().hp).collect();
    order(&mut w, &ctx, Cmd::Cast(0, NO_UNIT, [0, 0]));
    run(&mut w, &ctx, TPS);
    let lost: Vec<i32> = [close, wide, out]
        .iter()
        .enumerate()
        .map(|(i, &c)| hp[i] - w.get(c).unwrap().hp)
        .collect();
    assert!((111_000..=112_000).contains(&lost[0]), "close lost {}", lost[0]);
    assert!((74_000..=75_000).contains(&lost[1]), "wide lost {}", lost[1]);
    assert_eq!(lost[2], 0, "past 625");
}

#[test]
fn comet_arrow_stops_at_the_first_enemy_and_stuns_longer_the_farther_it_flew() {
    // 0.5 s for each 150 flown, from 0.5 s to 5 s; level 1 deals 90
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    let (_, spot) = hero(&f, &mut w, "skipjack", [0, 1, 0, 0]);
    let near = creep(&f, &ctx, &mut w, MOON, [spot[0] + 250, spot[1]]);
    let behind = creep(&f, &ctx, &mut w, MOON, [spot[0] + 500, spot[1]]);
    let hp = w.get(behind).unwrap().hp;
    order(&mut w, &ctx, Cmd::Cast(1, NO_UNIT, [spot[0] + 2000, spot[1]]));
    for _ in 0..TPS {
        if stunned_for(&w, near) > 0 {
            break;
        }
        w.step(&ctx, &[]);
    }
    let held = stunned_for(&w, near);
    assert!(held > 0 && held <= TPS * 70 / 100, "near: {held} ticks");
    // long enough for an arrow that flew on to reach the next creep
    run(&mut w, &ctx, TPS);
    assert_eq!(w.get(behind).unwrap().hp, hp, "the arrow stopped at the first");
    // a long shot: about 1500 flown is the full 5 s
    let mut w = f.world(8);
    let (_, spot) = hero(&f, &mut w, "skipjack", [0, 1, 0, 0]);
    w.cut_trees_in(&ctx, [spot[0] + 1600, spot[1]], 400);
    let far = creep(&f, &ctx, &mut w, MOON, [spot[0] + 1700, spot[1]]);
    order(&mut w, &ctx, Cmd::Cast(1, NO_UNIT, [spot[0] + 2000, spot[1]]));
    for _ in 0..3 * TPS {
        w.step(&ctx, &[]);
        if stunned_for(&w, far) > 0 {
            break;
        }
    }
    let held = stunned_for(&w, far);
    assert!(held > TPS * 45 / 10 && held <= TPS * 5, "far: {held} ticks");
}

#[test]
fn lily_leap_bounds_forward_and_speeds_allied_heroes_where_it_lands() {
    // level 4: 870 the way she faces; +16% attack and movement speed within 775 for 10 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(9);
    let (s, spot) = hero(&f, &mut w, "skipjack", [0, 0, 4, 0]);
    let ally = w.add_hero(&ctx, 1, 0);
    w.players[1].present = true;
    put(&mut w, ally, [spot[0] + 1200, spot[1] + 200]);
    rooted(&mut w, ally);
    let far = w.add_hero(&ctx, 2, 1);
    w.players[2].present = true;
    put(&mut w, far, [spot[0] - 300, spot[1] + 1000]);
    rooted(&mut w, far);
    order(&mut w, &ctx, Cmd::Cast(2, NO_UNIT, spot));
    run(&mut w, &ctx, TPS);
    let at = w.get(s).unwrap().pos();
    assert!(
        (at[0] - spot[0] - 870).abs() <= 64 && (at[1] - spot[1]).abs() <= 64,
        "landed at {at:?}"
    );
    let speed = |w: &World, id: Uid, k: M| -> i32 {
        w.get(id)
            .unwrap()
            .mods
            .iter()
            .filter(|m| m.kind == k)
            .map(|m| m.a)
            .sum()
    };
    assert_eq!(
        (speed(&w, ally, M::AttackPct), speed(&w, ally, M::MovePct)),
        (16, 16)
    );
    assert_eq!(
        (speed(&w, s, M::AttackPct), speed(&w, s, M::MovePct)),
        (16, 16),
        "herself too"
    );
    assert_eq!(speed(&w, far, M::MovePct), 0, "not past 775 of where she lands");
}

#[test]
fn pond_mist_hides_allied_heroes_after_the_fade_and_an_attack_shows_one_again() {
    // level 1: invisible after 2.5 s, for 11 s; after an attack, hidden 2.5 s later
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(10);
    let (s, spot) = hero(&f, &mut w, "skipjack", [0, 0, 0, 1]);
    let ally = w.add_hero(&ctx, 1, 0);
    w.players[1].present = true;
    put(&mut w, ally, [spot[0] + 3000, spot[1]]);
    w.cut_trees_in(&ctx, [spot[0] + 3000, spot[1]], 600);
    let c = creep(&f, &ctx, &mut w, MOON, [spot[0] + 3150, spot[1]]);
    // the ally stands still: an attack during the fade puts the fade back
    w.step(&ctx, &[(1, Cmd::Stand)]);
    order(&mut w, &ctx, Cmd::Cast(3, NO_UNIT, [0, 0]));
    let hidden = |w: &World, id: Uid| w.get(id).unwrap().has(M::Invisible);
    run(&mut w, &ctx, TPS * 2);
    assert!(!hidden(&w, ally), "not before the fade");
    run(&mut w, &ctx, TPS);
    assert!(
        hidden(&w, ally) && hidden(&w, s),
        "both hidden after 2.5 s, anywhere on the map"
    );
    // the ally attacks: shown, then hidden again once 2.5 s pass without another
    w.step(&ctx, &[(1, Cmd::Attack(c))]);
    let mut shown = false;
    for _ in 0..TPS {
        w.step(&ctx, &[]);
        shown |= !hidden(&w, ally);
    }
    assert!(shown, "the attack showed it");
    w.step(&ctx, &[(1, Cmd::Stand)]);
    run(&mut w, &ctx, TPS * 3);
    assert!(hidden(&w, ally), "hidden again after the fade");
    run(&mut w, &ctx, TPS * 8);
    assert!(!hidden(&w, s), "11 s and gone");
}

#[test]
fn an_invisible_hero_attacks_only_when_told_to() {
    // as in Warcraft III: a hidden hero does not strike what comes near on its own
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(11);
    let (s, spot) = hero(&f, &mut w, "skipjack", [0, 0, 0, 0]);
    w.add_mod(
        s,
        Mod {
            kind: M::Invisible,
            src: NO_UNIT,
            until: 0,
            a: 0,
            b: 0,
            tag: 0,
        },
    );
    let c = creep(&f, &ctx, &mut w, MOON, [spot[0] + 300, spot[1]]);
    // the creep is told to stand, so it does not strike first
    if let Some(u) = w.get_mut(c) {
        u.mods.push(Mod {
            kind: M::Disarm,
            src: NO_UNIT,
            until: 0,
            a: 0,
            b: 0,
            tag: 0,
        });
    }
    let full = w.get(c).unwrap().hp;
    run(&mut w, &ctx, 3 * TPS);
    assert!(w.get(c).unwrap().hp >= full, "left alone");
    w.step(&ctx, &[(0, Cmd::Attack(c))]);
    run(&mut w, &ctx, 3 * TPS);
    assert!(w.get(c).unwrap().hp < full, "struck when told");
}
