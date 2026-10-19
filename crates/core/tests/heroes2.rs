//! Two added heroes (round 7, preview: not yet on the roster): Hackle plays
//! as DotA 6.73's Troll Warlord and Chipper as Goblin Shredder (Dota 2's
//! Timbersaw), per notes/LITERATURE.md section 7. Expected values are
//! worked out by hand from that section and data/rules.json's tables: a
//! hero has 150 health and 19 more for each point of strength; heroes
//! resist a quarter of spell damage; a lane creep has none.

mod common;

use pots_core::ability::{self, slot_info, TAG_BLIND};
use pots_core::map::Tree;
use pots_core::orders::Cmd;
use pots_core::rules::{P, SUB, TPS};
use pots_core::stats;
use pots_core::world::{Ctx, Uid, World, M, MOON, NO_UNIT};

const HACKLE: usize = 0;
const CHIPPER: usize = 1;

fn roster(f: &common::Fixture, which: usize) -> usize {
    f.rules.roster + which
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
    // standing still, whatever comes near
    w.get_mut(c).unwrap().lane = 0;
    c
}

fn learn(w: &mut World, id: Uid, levels: [i32; 4]) {
    let u = w.get_mut(id).unwrap();
    for (s, l) in levels.iter().enumerate() {
        u.abil[s].level = *l;
    }
    u.mana = 100_000 * 1000;
}

/// Health lost, in thousandths, is `expected` whole points, less at most
/// two points a creep may grow back over a test's few seconds (0.5 a second).
fn lost(full: i32, now: i32, expected: i32, what: &str) {
    let l = full - now;
    assert!(
        (expected * 1000 - 2000..=expected * 1000).contains(&l),
        "{what}: lost {l}, not {expected} thousand"
    );
}

/// Casts from `slot`, lets the cast point pass, then has the hero stand
/// still, so its own attacks add nothing to what the test measures.
fn cast(w: &mut World, ctx: &Ctx, slot: u8, at: P) {
    w.step(ctx, &[(0, Cmd::Cast(slot, NO_UNIT, at))]);
    for _ in 0..12 {
        w.step(ctx, &[]);
    }
    w.step(ctx, &[(0, Cmd::Stand)]);
}

/// `at`, with every tree within `r` of it cut, so a test stands on open ground.
fn open(f: &common::Fixture, w: &mut World, at: P, r: i32) -> P {
    w.cut_trees_in(&f.ctx(), at, r);
    at
}

// --------------------------------------------------------------- Hackle

#[test]
fn red_tusk_turns_hackle_to_melee_with_its_bonuses_and_back() {
    // level 1, 17 strength: 150 + 17 × 19 = 473 health; in melee form 573
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(3);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    learn(&mut w, h, [1, 0, 0, 0]);
    let s = stats::of(&ctx, &w, h);
    assert_eq!((s.max_hp, s.range, s.bat, s.move_speed), (473, 500, 170, 300));
    assert_eq!(stats::missile(&ctx, &w, h), 1200, "ranged: throws its hatchets");
    let armor = s.armor;
    w.get_mut(h).unwrap().hp = 473 * 1000;
    w.step(&ctx, &[(0, Cmd::Cast(0, NO_UNIT, [0, 0]))]);
    let m = stats::of(&ctx, &w, h);
    assert_eq!((m.max_hp, m.range, m.bat, m.move_speed), (573, 128, 155, 320));
    assert_eq!(m.armor - armor, 300, "3 more armor, in hundredths");
    assert_eq!(m.bonus - s.bonus, 15, "15 more damage");
    assert_eq!(stats::missile(&ctx, &w, h), 0, "melee: swings them");
    assert_eq!(w.get(h).unwrap().hp / 1000, 573, "full health stays full");
    w.step(&ctx, &[(0, Cmd::Cast(0, NO_UNIT, [0, 0]))]);
    let back = stats::of(&ctx, &w, h);
    assert_eq!((back.max_hp, back.range, back.bat), (473, 500, 170));
}

#[test]
fn hatchet_storm_is_the_ranged_cone_from_range_and_the_whirl_up_close_each_with_its_own_cooldown() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(4);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    learn(&mut w, h, [1, 1, 0, 0]);
    let id = |w: &World| f.rules.abilities[slot_info(&ctx, w, h, 1).unwrap().0].id.clone();
    assert_eq!(id(&w), "whirling_axes");
    let here = w.get(h).unwrap().pos();
    w.step(&ctx, &[(0, Cmd::Cast(1, NO_UNIT, [here[0] + 400, here[1]]))]);
    for _ in 0..10 {
        w.step(&ctx, &[]);
    }
    let ranged_ready = w.get(h).unwrap().abil[1].ready_at;
    assert!(ranged_ready > w.tick, "the ranged throw is cooling down (20 s)");
    // into melee: the slot holds the whirl, ready at once
    w.step(&ctx, &[(0, Cmd::Cast(0, NO_UNIT, [0, 0]))]);
    assert_eq!(id(&w), "whirling_axes_melee");
    assert!(
        ability::can_start(&w, &ctx, h, 1, NO_UNIT, here),
        "the melee whirl has its own cooldown"
    );
    // and back: the ranged throw still has its time to wait
    w.step(&ctx, &[(0, Cmd::Cast(0, NO_UNIT, [0, 0]))]);
    assert_eq!(id(&w), "whirling_axes");
    assert_eq!(w.get(h).unwrap().abil[1].ready_at, ranged_ready);
}

#[test]
fn thrown_hatchets_hit_each_enemy_once_for_75_and_slow_it() {
    // five axes in a cone; a creep in the middle of it takes 75 magic
    // damage once, however many axes pass, and a 30% slow for 3 s
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    learn(&mut w, h, [0, 1, 0, 0]);
    let spot = open(&f, &mut w, [8192, 8192], 900);
    put(&mut w, h, spot);
    let c = creep(&f, &ctx, &mut w, [spot[0] + 300, spot[1]]);
    let full = w.get(c).unwrap().hp;
    cast(&mut w, &ctx, 1, [spot[0] + 600, spot[1]]);
    for _ in 0..30 {
        w.step(&ctx, &[]);
    }
    let v = w.get(c).unwrap();
    lost(full, v.hp, 75, "once, for 75");
    let slow = v
        .mods
        .iter()
        .find(|m| m.kind == M::MovePct && m.a < 0)
        .expect("slowed");
    assert_eq!(slow.a, -30);
}

#[test]
fn the_whirl_up_close_hits_for_125_and_makes_enemies_miss() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(6);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    learn(&mut w, h, [1, 1, 0, 0]);
    let spot = open(&f, &mut w, [8192, 8192], 600);
    put(&mut w, h, spot);
    let c = creep(&f, &ctx, &mut w, [spot[0] + 250, spot[1]]);
    let full = w.get(c).unwrap().hp;
    w.step(&ctx, &[(0, Cmd::Cast(0, NO_UNIT, [0, 0]))]);
    cast(&mut w, &ctx, 1, spot);
    for _ in 0..(3 * TPS) {
        w.step(&ctx, &[]);
    }
    let v = w.get(c).unwrap();
    lost(full, v.hp, 125, "once, for 125");
    let blind = v
        .mods
        .iter()
        .find(|m| m.kind == M::Tag && m.b == TAG_BLIND)
        .expect("made to miss");
    assert_eq!(blind.a, 60);
}

#[test]
fn grudge_adds_attack_speed_on_the_same_target_and_starts_again_on_a_new_one() {
    // level 1: +20 a hit, at most 2 stacks
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    learn(&mut w, h, [0, 0, 1, 0]);
    let spot = open(&f, &mut w, [8192, 8192], 400);
    put(&mut w, h, spot);
    let a = creep(&f, &ctx, &mut w, [spot[0] + 200, spot[1]]);
    let b = creep(&f, &ctx, &mut w, [spot[0] - 200, spot[1]]);
    let base = stats::of(&ctx, &w, h).attack_speed;
    let bonus = |w: &World| stats::of(&ctx, w, h).attack_speed - base;
    ability::after_attack(&mut w, &ctx, h, a, 1);
    assert_eq!(bonus(&w), 20);
    ability::after_attack(&mut w, &ctx, h, a, 1);
    ability::after_attack(&mut w, &ctx, h, a, 1);
    assert_eq!(bonus(&w), 40, "two stacks at the most");
    ability::after_attack(&mut w, &ctx, h, b, 1);
    assert_eq!(bonus(&w), 20, "a new target starts again");
}

#[test]
fn war_drums_speeds_every_allied_hero_and_hackle_twice_as_long() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(8);
    let h = w.add_hero(&ctx, 0, roster(&f, HACKLE));
    let ally = w.add_hero(&ctx, 1, 0);
    learn(&mut w, h, [0, 0, 0, 1]);
    let before = stats::of(&ctx, &w, ally).attack_speed;
    w.step(&ctx, &[(0, Cmd::Cast(3, NO_UNIT, [0, 0]))]);
    for _ in 0..10 {
        w.step(&ctx, &[]);
    }
    assert_eq!(stats::of(&ctx, &w, ally).attack_speed - before, 60);
    let until = |id: Uid, w: &World| {
        w.get(id)
            .unwrap()
            .mods
            .iter()
            .find(|m| m.kind == M::AttackPct)
            .unwrap()
            .until
    };
    // 5 s and 10 s: 150 and 300 ticks
    assert_eq!(until(h, &w) - until(ally, &w), 150);
}

// -------------------------------------------------------------- Chipper

#[test]
fn splinter_spin_deals_magic_damage_without_trees_and_cuts_the_main_attribute() {
    // level 1: 100 damage, a quarter resisted by a hero: 75; Kestrel's
    // agility 20 loses 15%: 17
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(9);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    let foe = w.add_hero(&ctx, 5, 6);
    learn(&mut w, c, [1, 0, 0, 0]);
    let spot = open(&f, &mut w, [8192, 8192], 500);
    put(&mut w, c, spot);
    put(&mut w, foe, [spot[0] + 150, spot[1]]);
    assert_eq!(stats::of(&ctx, &w, foe).attrs[1], 20);
    let full = w.get(foe).unwrap().hp;
    let max = stats::of(&ctx, &w, foe).max_hp;
    cast(&mut w, &ctx, 0, spot);
    assert_eq!(stats::of(&ctx, &w, foe).attrs[1], 17);
    // compared at the new maximum (the cut keeps the share of health): 75
    let after_max = stats::of(&ctx, &w, foe).max_hp;
    let scaled = (full as i64 * after_max as i64 / max as i64) as i32;
    lost(scaled, w.get(foe).unwrap().hp, 75, "75 magic damage");
    // 7 s later the agility is back
    for _ in 0..(7 * TPS) {
        w.step(&ctx, &[]);
    }
    assert_eq!(stats::of(&ctx, &w, foe).attrs[1], 20);
}

#[test]
fn splinter_spin_is_pure_damage_when_it_cuts_a_tree() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(10);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    learn(&mut w, c, [1, 0, 0, 0]);
    // beside a standing tree, inside the forest's edge
    let t = &f.terrain;
    let tree = (0..(t.tw * t.tw) as usize)
        .find(|&i| {
            let (x, y) = (i as i32 % t.tw, i as i32 / t.tw);
            x > 20 && y > 20 && matches!(w.trees[i], Tree::Standing)
        })
        .unwrap();
    let at = t.tree_centre(tree);
    put(&mut w, c, at);
    // a hero resists a quarter of magic damage (75 of 100) and none of pure
    let k = w.add_hero(&ctx, 5, 6);
    put(&mut w, k, [at[0] + 100, at[1]]);
    let full = w.get(k).unwrap().hp;
    let max = stats::of(&ctx, &w, k).max_hp;
    cast(&mut w, &ctx, 0, at);
    assert!(!matches!(w.trees[tree], Tree::Standing), "the tree is cut");
    // the agility cut keeps the share of health: compare at the new maximum
    let after_max = stats::of(&ctx, &w, k).max_hp;
    let scaled = (full as i64 * after_max as i64 / max as i64) as i32;
    lost(scaled, w.get(k).unwrap().hp, 100, "pure: all 100");
}

#[test]
fn grapple_claw_pulls_chipper_to_the_first_tree_and_cuts_enemies_on_the_way() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(11);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    learn(&mut w, c, [0, 1, 0, 0]);
    // 600 east of a tree with open ground between
    let t = &f.terrain;
    let (tree, from) = (0..(t.tw * t.tw) as usize)
        .filter(|&i| matches!(w.trees[i], Tree::Standing) && (i as i32 % t.tw) > 20 && (i as i32 / t.tw) > 20)
        .find_map(|i| {
            let tc = t.tree_centre(i);
            let from = [tc[0] + 600, tc[1]];
            let clear = (1..=18).all(|k| {
                let p = [tc[0] + 32 * k + 100, tc[1]];
                (0..(t.tw * t.tw) as usize)
                    .filter(|&j| matches!(w.trees[j], Tree::Standing))
                    .all(|j| {
                        let q = t.tree_centre(j);
                        let (dx, dy) = ((q[0] - p[0]) as i64, (q[1] - p[1]) as i64);
                        dx * dx + dy * dy > 120 * 120
                    })
            });
            clear.then_some((i, from))
        })
        .expect("a tree with open ground east of it");
    put(&mut w, c, from);
    let tc = t.tree_centre(tree);
    let k = creep(&f, &ctx, &mut w, [tc[0] + 300, tc[1] + 60]);
    let full = w.get(k).unwrap().hp;
    cast(&mut w, &ctx, 1, tc);
    // once landed he is free to fight, so look at how near the pull took him
    let mut nearest = i32::MAX;
    for _ in 0..(2 * TPS) {
        w.step(&ctx, &[]);
        let at = w.get(c).unwrap().pos();
        nearest = nearest.min((at[0] - tc[0]).abs().max((at[1] - tc[1]).abs()));
    }
    assert!(nearest <= 40, "pulled to the tree: within {nearest}");
    assert!(!matches!(w.trees[tree], Tree::Standing), "the tree falls");
    lost(full, w.get(k).unwrap().hp, 80, "80 pure damage on the way");
}

#[test]
fn thick_bark_stacks_on_each_attack_begun_up_to_its_limit() {
    // level 1: +1 armor and +1 health a second a stack, 4 at the most
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(12);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    learn(&mut w, c, [0, 0, 1, 0]);
    let armor = stats::of(&ctx, &w, c).armor;
    for _ in 0..6 {
        ability::attacked(&mut w, &ctx, c, NO_UNIT);
    }
    assert_eq!(
        stats::of(&ctx, &w, c).armor - armor,
        400,
        "four stacks, in hundredths"
    );
    for _ in 0..(16 * TPS + 2) {
        w.step(&ctx, &[]);
    }
    assert_eq!(stats::of(&ctx, &w, c).armor, armor, "gone after 16 s");
}

#[test]
fn buzz_blade_flies_out_spins_drains_mana_and_comes_back_on_recall() {
    // level 1: 100 pure damage passing, 20 mana a second spinning
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(13);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    learn(&mut w, c, [0, 0, 0, 1]);
    let spot = open(&f, &mut w, [8192, 8192], 1000);
    put(&mut w, c, spot);
    let k = creep(&f, &ctx, &mut w, [spot[0] + 300, spot[1]]);
    let full = w.get(k).unwrap().hp;
    let target = [spot[0] + 800, spot[1]];
    cast(&mut w, &ctx, 3, target);
    for _ in 0..TPS {
        w.step(&ctx, &[]);
    }
    lost(full, w.get(k).unwrap().hp, 100, "passed once: 100 pure");
    assert!(w.get(c).unwrap().has(M::Disarm), "no attacking while it is out");
    assert_eq!(
        f.rules.abilities[slot_info(&ctx, &w, c, 3).unwrap().0].id,
        "return_chakram"
    );
    // spinning: 20 mana a second
    let m0 = w.get(c).unwrap().mana;
    for _ in 0..(2 * TPS) {
        w.step(&ctx, &[]);
    }
    let spent = m0 - w.get(c).unwrap().mana;
    assert!(
        (38_000..=42_000).contains(&spent),
        "about 40 mana in 2 s: {spent}"
    );
    w.step(&ctx, &[(0, Cmd::Cast(3, NO_UNIT, target))]);
    for _ in 0..(2 * TPS) {
        w.step(&ctx, &[]);
    }
    assert!(!w.get(c).unwrap().has(M::Disarm), "back: Chipper can attack");
    assert_eq!(
        f.rules.abilities[slot_info(&ctx, &w, c, 3).unwrap().0].id,
        "chakram"
    );
    assert!(w.zones.is_empty(), "the blade is home");
}

#[test]
fn preview_heroes_cannot_be_chosen() {
    let f = common::fixture();
    assert_eq!(f.rules.roster, 10, "the roster is the first ten");
    assert_eq!(f.rules.heroes.len(), 12, "two preview heroes after it");
}

#[test]
fn a_stunned_chipper_is_not_pulled_when_the_claw_holds() {
    // DotA 6.73: "You will not be pulled in if you are stunned when the
    // chain latches onto a tree"
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(14);
    let c = w.add_hero(&ctx, 0, roster(&f, CHIPPER));
    learn(&mut w, c, [0, 1, 0, 0]);
    let t = &f.terrain;
    let tree = (0..(t.tw * t.tw) as usize)
        .find(|&i| matches!(w.trees[i], Tree::Standing) && (i as i32 % t.tw) > 20 && (i as i32 / t.tw) > 20)
        .unwrap();
    let tc = t.tree_centre(tree);
    let from = open(&f, &mut w, [tc[0] + 600, tc[1]], 450);
    w.cut_trees_in(&ctx, [tc[0] + 330, tc[1]], 260);
    put(&mut w, c, from);
    w.step(&ctx, &[(0, Cmd::Cast(1, NO_UNIT, tc))]);
    // the cast point passes and the claw is in the air: a long stun now
    for _ in 0..40 {
        if !w.missiles.is_empty() {
            break;
        }
        w.step(&ctx, &[]);
    }
    assert_eq!(w.missiles.len(), 1, "the claw is flying");
    let until = w.tick + 3 * TPS as u32;
    w.get_mut(c).unwrap().mods.push(pots_core::world::Mod {
        kind: M::Stun,
        src: c,
        until,
        a: 0,
        b: 0,
        tag: 0,
    });
    for _ in 0..TPS {
        w.step(&ctx, &[]);
    }
    let at = w.get(c).unwrap().pos();
    assert!((at[0] - from[0]).abs() < 100, "not pulled: {at:?} from {from:?}");
    assert!(matches!(w.trees[tree], Tree::Standing), "the tree still stands");
}
