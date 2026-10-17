//! Units are solid (T12): the creeps of a wave walk apart rather than
//! piling onto one point, and a hero standing in their way is walked round,
//! not through, which is what lets a player block creeps as in DotA.

mod common;

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
