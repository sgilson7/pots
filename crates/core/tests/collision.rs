//! Units are solid (T12): the creeps of a wave walk apart rather than
//! piling onto one point, and a hero standing in their way is walked round,
//! not through, which is what lets a player block creeps as in DotA.

mod common;

use pots_core::orders::Cmd;
use pots_core::rules::{Kind, SUB, TPS};
use pots_core::world::{dist2, Order, Uid, World, SUN};

fn creeps(w: &World, f: &common::Fixture) -> Vec<Uid> {
    w.units
        .iter()
        .filter(|u| u.alive && f.rules.units[u.def as usize].kind == Kind::Creep)
        .map(|u| u.id)
        .collect()
}

/// Pairs of creeps standing inside each other: centres closer than their
/// two radii (less a sixteenth). Before units were solid, up to 17 pairs at
/// once did in the first 70 seconds of this match; now none do.
fn piled(w: &World, f: &common::Fixture) -> usize {
    let ids = creeps(w, f);
    let mut n = 0;
    for (i, &a) in ids.iter().enumerate() {
        for &b in &ids[i + 1..] {
            let (ua, ub) = (w.get(a).unwrap(), w.get(b).unwrap());
            let r =
                (f.rules.units[ua.def as usize].radius + f.rules.units[ub.def as usize].radius) * SUB - SUB;
            let (dx, dy) = ((ua.x - ub.x) as i64, (ua.y - ub.y) as i64);
            if dx * dx + dy * dy < (r as i64) * (r as i64) {
                n += 1;
            }
        }
    }
    n
}

#[test]
fn the_creeps_of_a_wave_never_stand_inside_each_other() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(5);
    let horn = w.horn_at;
    let mut worst = 0;
    let mut looked = 0;
    // the first waves walk out, meet in the lanes and fight
    while w.tick < horn + 70 * TPS as u32 {
        w.step(&ctx, &[]);
        if w.tick > horn && w.tick.is_multiple_of(TPS as u32 / 3) {
            worst = worst.max(piled(&w, &f));
            looked += 1;
        }
    }
    assert!(
        looked > 150 && creeps(&w, &f).len() > 10,
        "the waves ran ({looked} looks)"
    );
    assert_eq!(
        worst, 0,
        "at worst {worst} pairs of creeps stood inside each other"
    );
}

#[test]
fn a_hero_standing_in_the_creeps_way_is_walked_round_not_through() {
    let f = common::fixture();
    let ctx = f.ctx();
    let run = |block: bool| {
        let mut w = f.world(5);
        while w.tick < w.horn_at + 12 {
            w.step(&ctx, &[]);
        }
        // the wave's lead creep, and where it is heading
        let wave: Vec<Uid> = creeps(&w, &f)
            .into_iter()
            .filter(|&c| w.get(c).unwrap().team == SUN && w.get(c).unwrap().lane == 1)
            .collect();
        let lead = wave[0];
        let p0 = w.get(lead).unwrap().pos();
        for _ in 0..6 {
            w.step(&ctx, &[]);
        }
        let p1 = w.get(lead).unwrap().pos();
        let (hx, hy) = (p1[0] + (p1[0] - p0[0]) * 8, p1[1] + (p1[1] - p0[1]) * 8);
        let mut hero = None;
        if block {
            let h = w.add_hero(&ctx, 0, 0);
            let u = w.get_mut(h).unwrap();
            u.x = hx * SUB;
            u.y = hy * SUB;
            u.path.clear();
            u.order = Order::Hold;
            hero = Some(h);
        }
        let mut closest = i64::MAX;
        for _ in 0..(3 * TPS) {
            w.step(&ctx, &[]);
            if let Some(h) = hero {
                let hp = w.get(h).unwrap();
                for &c in &wave {
                    let cu = w.get(c).unwrap();
                    let (dx, dy) = ((cu.x - hp.x) as i64, (cu.y - hp.y) as i64);
                    closest = closest.min(dx * dx + dy * dy);
                }
            }
        }
        let moved = hero.map(|h| w.get(h).unwrap().pos());
        (dist2(w.get(lead).unwrap().pos(), p1), closest, moved, [hx, hy])
    };
    let (free, _, _, _) = run(false);
    let (held, closest, stood, spot) = run(true);
    let touch = ((f.rules.units[f.rules.heroes[0]].radius + 24) * SUB) as i64;
    assert!(
        closest >= (touch - SUB as i64) * (touch - SUB as i64),
        "a creep came {} of {} sixteenths from the hero's centre",
        pots_core::fx::isqrt(closest as u64),
        touch
    );
    assert_eq!(stood, Some(spot), "the creeps did not shove the hero aside");
    assert!(
        held < free,
        "the hero held the lead creep back: {held} against {free} (squared map units walked)"
    );
}

/// Blocking slows a wave; it does not stop one. A hero who holds still on
/// the middle lane's line, with no enemy near, is walked round by the whole
/// first wave: each creep gets past the hero within the time the walk takes
/// plus a few seconds, and arrives later than the free wave does.
#[test]
fn a_held_wave_gets_past_a_hero_who_stands_still() {
    let f = common::fixture();
    let ctx = f.ctx();
    // the hero stands on the middle lane, about 2,800 map units (8.7 s at
    // 325 a second) from where the Sunward wave starts at (3000, 3000)
    let spot = [5000, 5000];
    let run = |block: bool| {
        let mut w = f.world(5);
        while w.tick < w.horn_at + 1 {
            w.step(&ctx, &[]);
        }
        let wave: Vec<Uid> = creeps(&w, &f)
            .into_iter()
            .filter(|&c| w.get(c).unwrap().team == SUN && w.get(c).unwrap().lane == 1)
            .collect();
        if block {
            let h = w.add_hero(&ctx, 0, 0);
            let u = w.get_mut(h).unwrap();
            u.x = spot[0] * SUB;
            u.y = spot[1] * SUB;
            u.path.clear();
            u.order = Order::Hold;
        }
        // the tick each creep first stands past the hero, measured along the
        // lane's diagonal (x + y), 100 map units beyond the spot
        let mut past = vec![None; wave.len()];
        for _ in 0..(18 * TPS) {
            w.step(&ctx, &[]);
            for (k, &c) in wave.iter().enumerate() {
                let p = w.get(c).unwrap().pos();
                if past[k].is_none() && p[0] + p[1] > spot[0] + spot[1] + 200 {
                    past[k] = Some(w.tick - w.horn_at);
                }
            }
        }
        (wave.len(), past)
    };
    let (n, free) = run(false);
    let (_, held) = run(true);
    assert!(n >= 4, "a wave of {n} creeps");
    let free_last = free
        .iter()
        .map(|t| t.expect("every free creep passed the spot"))
        .max()
        .unwrap();
    let held_ticks: Vec<u32> = held
        .iter()
        .map(|t| t.expect("every held creep got past the hero within 18 s"))
        .collect();
    let held_last = *held_ticks.iter().max().unwrap();
    assert!(
        held_last >= free_last,
        "the hero did not speed the wave: the last creep passed at tick {held_last} against {free_last}"
    );
}

/// A hero who keeps stepping into a lane creep's way holds it back as in
/// DotA. One creep (325 a second) walks the middle lane from about
/// (3000, 3000) until x + y passes 12,000: 4,243 map units, about 13 s. The
/// first roster card's hero (295 a second, radius 24) keeps moving to a
/// point 51 map units ahead of it, inside the 48 at which the two touch
/// plus one step, so while it stays behind the creep goes no faster than
/// the hero: at least 14.4 s, 1.3 s later. The test asks for one second.
#[test]
fn a_hero_stepping_in_front_of_a_creep_delays_it_by_a_second() {
    let f = common::fixture();
    let ctx = f.ctx();
    let hero = &f.rules.units[f.rules.heroes[0]];
    assert_eq!(
        (hero.speed, hero.radius),
        (295, 24),
        "the hand-worked numbers' hero"
    );
    let run = |block: bool| {
        let mut w = f.world(5);
        while w.tick < w.horn_at + 1 {
            w.step(&ctx, &[]);
        }
        let wave: Vec<Uid> = creeps(&w, &f)
            .into_iter()
            .filter(|&c| w.get(c).unwrap().team == SUN && w.get(c).unwrap().lane == 1)
            .collect();
        // one creep of the wave, the rest taken away
        let (rest, one) = wave.split_at(wave.len() - 1);
        for &c in rest {
            w.get_mut(c).unwrap().alive = false;
        }
        let c = one[0];
        let h = w.add_hero(&ctx, 0, 0);
        let start = if block { 3250 } else { 1000 };
        let u = w.get_mut(h).unwrap();
        u.x = start * SUB;
        u.y = start * SUB;
        u.path.clear();
        u.order = Order::Idle;
        let mut closest = i64::MAX;
        for _ in 0..(25 * TPS) {
            let (cu, hu) = (w.get(c).unwrap(), w.get(h).unwrap());
            let (dx, dy) = ((cu.x - hu.x) as i64, (cu.y - hu.y) as i64);
            closest = closest.min(dx * dx + dy * dy);
            let p = cu.pos();
            if p[0] + p[1] > 12000 {
                return (w.tick - w.horn_at, closest);
            }
            let orders = if block {
                vec![(0u8, Cmd::Move([p[0] + 36, p[1] + 36]))]
            } else {
                vec![]
            };
            w.step(&ctx, &orders);
        }
        panic!("the creep never got past (block {block})");
    };
    let (free, _) = run(false);
    let (held, closest) = run(true);
    // the creep never walks into the hero: centres at least the two radii
    // apart, less the 4 map units a tick by which units are pushed apart
    // (a push from a building or another unit can press the creep that far
    // into the hero for a tick while it slides round)
    let touch = ((hero.radius + 24 - 4) * SUB) as i64;
    assert!(
        closest >= touch * touch,
        "the creep came {} sixteenths from the hero's centre, inside {touch}",
        pots_core::fx::isqrt(closest as u64)
    );
    assert!(
        held >= free + TPS as u32,
        "the hero held the creep back by at least a second: {held} ticks against {free}"
    );
}
