//! Whole matches between computer players, on the game's own data.

mod common;

use pots_core::rules::TPS;
use pots_core::world::PLAYERS;

#[test]
fn ten_bots_play_twenty_minutes_without_a_fault() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    for p in 0..PLAYERS {
        w.players[p].bot = Some(2);
        w.players[p].present = true;
    }
    let start = std::time::Instant::now();
    let ticks = 20 * 60 * TPS as u32;
    for _ in 0..ticks {
        w.step(&ctx, &[]);
        if w.winner.is_some() {
            break;
        }
    }
    let per_tick = start.elapsed().as_micros() as f64 / w.tick as f64;
    let kills: i32 = w.players.iter().map(|p| p.kills).sum();
    let lh: i32 = w.players.iter().map(|p| p.last_hits).sum();
    let levels: Vec<i32> = w
        .players
        .iter()
        .map(|p| w.get(p.hero).map(|u| u.level()).unwrap_or(0))
        .collect();
    let towers = w
        .units
        .iter()
        .filter(|u| u.alive && ctx.rules.units[u.def as usize].kind == pots_core::rules::Kind::Tower)
        .count();
    println!(
        "tick {} ({} s), {:.0} us a tick, kills {kills}, last hits {lh}, levels {levels:?}, towers standing {towers}, units {}",
        w.tick,
        w.tick / TPS as u32,
        per_tick,
        w.units.iter().filter(|u| u.alive).count()
    );
    for (k, p) in w.players.iter().enumerate() {
        let rule = w.players[k].brain.rule;
        println!(
            "seat {k}: lh {} dn {} k/d/a {}/{}/{} gold {} items {:?} rule {rule}",
            p.last_hits,
            p.denies,
            p.kills,
            p.deaths,
            p.assists,
            p.gold,
            w.get(p.hero)
                .map(|u| u.items.iter().filter(|s| s.item > 0).count())
                .unwrap_or(0)
        );
    }
    assert!(w.players.iter().all(|p| p.hero != 0), "every bot picked a hero");
    assert!(lh > 0, "bots last-hit creeps");
}
