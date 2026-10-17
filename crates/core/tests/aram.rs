//! All Random All Mid (round 4, T13 and T14): one lane, dealt heroes, the
//! base cannon and the health relics. Expected values are worked out by hand
//! from data/rules.json and data/map.json, never read back from the code
//! under test.

mod common;

use pots_core::aram;
use pots_core::orders::{self, Cmd};
use pots_core::rules::{Kind, SUB, TPS};
use pots_core::world::{Act, World, M, MOON, PLAYERS, SUN};

fn aram_world(f: &common::Fixture, seed: u64) -> World {
    World::new_mode(&f.ctx(), f.trees.clone(), seed, true)
}

fn towers(w: &World, f: &common::Fixture, lane: u8) -> usize {
    w.units
        .iter()
        .filter(|u| u.alive && f.rules.units[u.def as usize].kind == Kind::Tower && u.lane_name == lane)
        .count()
}

#[test]
fn only_the_middle_lane_is_built_and_fought_over() {
    let f = common::fixture();
    let ctx = f.ctx();
    let normal = f.world(3);
    let mut w = aram_world(&f, 3);
    // three towers a lane a side in a normal match; none on the sides here
    assert_eq!((towers(&normal, &f, 0), towers(&normal, &f, 2)), (6, 6));
    assert_eq!(
        (towers(&w, &f, 0), towers(&w, &f, 1), towers(&w, &f, 2)),
        (0, 6, 0)
    );
    assert_eq!(towers(&w, &f, 3), 4, "the four base towers stand");
    // past the horn, the first camps (30 s later) and the first rune
    while w.tick < w.horn_at + 40 * TPS as u32 {
        w.step(&ctx, &[]);
    }
    let creeps: Vec<u8> = w
        .units
        .iter()
        .filter(|u| u.alive && f.rules.units[u.def as usize].kind == Kind::Creep)
        .map(|u| u.lane)
        .collect();
    assert!(!creeps.is_empty(), "the waves came");
    assert!(
        creeps.iter().all(|&l| l == 1),
        "every creep walks the middle lane: {creeps:?}"
    );
    let forest = w
        .units
        .iter()
        .filter(|u| u.alive && matches!(f.rules.units[u.def as usize].kind, Kind::Neutral | Kind::Boss))
        .count();
    assert_eq!(forest, 0, "no camps and no boss");
    assert!(w.rune.is_none(), "no runes");
}

#[test]
fn the_base_towers_open_once_the_middle_lane_falls() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = aram_world(&f, 4);
    let base = |w: &World| {
        w.units
            .iter()
            .filter(|u| {
                u.alive && u.team == SUN && u.tier == 4 && f.rules.units[u.def as usize].kind == Kind::Tower
            })
            .all(|u| u.has(M::Invulnerable))
    };
    for _ in 0..20 {
        w.step(&ctx, &[]);
    }
    assert!(
        base(&w),
        "the base towers cannot be hurt while the middle base-level tower stands"
    );
    let t3: Vec<_> = w
        .units
        .iter()
        .filter(|u| u.team == SUN && u.tier == 3 && f.rules.units[u.def as usize].kind == Kind::Tower)
        .map(|u| u.id)
        .collect();
    assert_eq!(t3.len(), 1, "one base-level tower: the middle lane's");
    w.get_mut(t3[0]).unwrap().hp = 0;
    for _ in 0..20 {
        w.step(&ctx, &[]);
    }
    assert!(!base(&w), "with it down, the base towers can be hurt");
}

#[test]
fn each_side_is_dealt_five_different_heroes_and_a_reroll_draws_a_sixth() {
    let none = [false; PLAYERS];
    let a = aram::deal(77, 10, &none);
    assert_eq!(a, aram::deal(77, 10, &none), "the same seed deals the same");
    assert_ne!(a, aram::deal(78, 10, &none), "another seed deals another");
    for side in [&a[..5], &a[5..]] {
        let mut v = side.to_vec();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), 5, "five different heroes a side: {side:?}");
    }
    let mut rr = none;
    rr[2] = true;
    let b = aram::deal(77, 10, &rr);
    assert_ne!(b[2], a[2], "the reroll changed seat 2's hero");
    assert!(!a[..5].contains(&b[2]), "and never to one its side was dealt");
    assert_eq!(
        (&b[..2], &b[3..]),
        (&a[..2], &a[3..]),
        "nobody else's hero changed"
    );
}

#[test]
fn players_start_with_853_gold() {
    let f = common::fixture();
    assert_eq!(aram_world(&f, 1).players[0].gold, 853);
    assert_eq!(f.world(1).players[0].gold, 603, "603 in a normal match");
}

#[test]
fn the_cannon_throws_a_hero_to_the_band_s_end_in_two_and_a_half_seconds() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = aram_world(&f, 5);
    let h = w.add_hero(&ctx, 0, 0);
    w.step(&ctx, &[]);
    // aimed far past the enemy base: held at the band's end. The middle
    // lane's base-level tower is at (4250, 3850), its outer tower at
    // (6450, 6050), 3,111 units on; 300 farther is (6662, 6262).
    let start = w.tick;
    // (on the lane's own line, x = y + 400, so held at its end, not beside it)
    w.step(&ctx, &[(0, Cmd::Cannon([15400, 15000]))]);
    assert!(
        matches!(w.get(h).unwrap().act, Act::Flight { .. }),
        "in the cannon"
    );
    let hp = w.get(h).unwrap().hp;
    pots_core::combat::damage(&mut w, &ctx, h, h, 500, pots_core::combat::Dmg::Pure);
    assert_eq!(w.get(h).unwrap().hp, hp, "no damage in the cannon");
    // 0.5 s climbing in and 2 s in the air: 75 ticks
    while w.tick < start + 75 {
        w.step(&ctx, &[]);
    }
    assert!(
        matches!(w.get(h).unwrap().act, Act::Flight { .. }),
        "still in the air at 75 ticks"
    );
    w.step(&ctx, &[]);
    let u = w.get(h).unwrap();
    assert!(!matches!(u.act, Act::Flight { .. }), "landed");
    assert_eq!(u.pos(), [6662, 6262]);
    assert!(!u.hero.as_ref().unwrap().cannon, "the throw spent the cannon");
    w.step(&ctx, &[(0, Cmd::Cannon([6000, 6000]))]);
    assert!(
        !matches!(w.get(h).unwrap().act, Act::Flight { .. }),
        "far from the fountain it is refused"
    );
    // back at the fountain it loads again
    let spawn = f.rules.map.sides[0].spawn;
    let u = w.get_mut(h).unwrap();
    u.x = spawn[0] * SUB;
    u.y = spawn[1] * SUB;
    w.step(&ctx, &[]);
    assert!(
        w.get(h).unwrap().hero.as_ref().unwrap().cannon,
        "loaded again at the base"
    );
}

#[test]
fn a_throw_lands_inside_the_lane_and_on_its_own_side() {
    let f = common::fixture();
    let ctx = f.ctx();
    let w = aram_world(&f, 6);
    // aimed off the lane to the side: pulled onto the lane. The band's line
    // runs from (4250, 3850) along x = y + 400, so a point's distance from
    // it is |x - y - 400| / 1.414
    let p = aram::clamp_landing(&ctx, SUN, [5500, 4500]);
    let off = ((p[0] - p[1] - 400).abs() as i64 * 1000) / 1414;
    assert!(
        off <= 211,
        "within half the lane's 420 of the line: {p:?} is {off} off it"
    );
    // the Moonward band is the Sunward one turned through the centre
    let (a, b, half) = aram::band(&ctx, SUN).unwrap();
    let (c, d, half2) = aram::band(&ctx, MOON).unwrap();
    assert_eq!(
        (c, d, half2),
        ([16384 - a[0], 16384 - a[1]], [16384 - b[0], 16384 - b[1]], half)
    );
    let _ = w;
}

#[test]
fn a_relic_heals_a_hero_once_and_returns_after_a_minute() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = aram_world(&f, 7);
    let h = w.add_hero(&ctx, 0, 0);
    let max = pots_core::stats::of(&ctx, &w, h).max_hp * 1000;
    {
        let u = w.get_mut(h).unwrap();
        u.hp = max / 4;
        u.x = 7060 * SUB;
        u.y = 6800 * SUB;
    }
    w.step(&ctx, &[]);
    // 25 % plus the relic's 15 %, plus at most a tick of regeneration
    let hp = w.get(h).unwrap().hp;
    assert!(
        hp >= max * 40 / 100 && hp < max * 41 / 100,
        "healed by 15 %: {hp} of {max}"
    );
    let back = w.relics[0];
    assert!(back > w.tick, "the relic is away");
    w.step(&ctx, &[]);
    assert!(
        w.get(h).unwrap().hp < max * 41 / 100,
        "taken once, not every tick"
    );
    while w.tick <= back {
        w.step(&ctx, &[]);
    }
    w.get_mut(h).unwrap().x = 2000 * SUB;
    w.step(&ctx, &[]);
    assert_eq!(w.relics[0], 0, "back after 60 s");
}

#[test]
fn teleports_are_not_sold_and_the_far_shops_goods_are_sold_at_the_base() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = aram_world(&f, 8);
    w.add_hero(&ctx, 0, 0);
    w.players[0].gold = 20_000;
    let tp = f.rules.item("tp").unwrap();
    let hyper = f.rules.item("hyperstone").unwrap();
    let gold = w.players[0].gold;
    orders::apply(&mut w, &ctx, 0, Cmd::Buy(tp as u16));
    assert_eq!(w.players[0].gold, gold, "no scroll for sale");
    orders::apply(&mut w, &ctx, 0, Cmd::Buy(hyper as u16));
    assert!(w.players[0].gold < gold, "a secret-shop item at the base");
}

#[test]
fn computer_players_ride_the_cannon_out_and_keep_to_the_middle_lane() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = aram_world(&f, 9);
    let deal = aram::deal(9, f.rules.heroes.len(), &[false; PLAYERS]);
    for (p, &hero) in deal.iter().enumerate() {
        w.players[p].present = true;
        w.players[p].bot = Some(2);
        orders::apply(&mut w, &ctx, p, Cmd::Pick(hero));
    }
    let mut flew = [false; PLAYERS];
    for _ in 0..(20 * TPS) {
        w.step(&ctx, &[]);
        for (p, flown) in flew.iter_mut().enumerate() {
            if let Some(u) = w.get(w.players[p].hero) {
                *flown |= matches!(u.act, Act::Flight { .. });
            }
        }
    }
    assert!(
        flew.iter().all(|&x| x),
        "every computer player rode the cannon: {flew:?}"
    );
    for p in 0..PLAYERS {
        assert_eq!(w.players[p].brain.lane, 1, "seat {p} keeps to the middle lane");
    }
}
