mod common;

use pots_core::rules::TPS;
use pots_core::world::PLAYERS;

#[test]
#[ignore]
fn diag_ten_bots_in_the_arena() {
    let f = common::arena_fixture();
    let ctx = f.ctx();
    let mut w = f.arena_world(5);
    for p in 0..PLAYERS {
        w.players[p].bot = Some(2);
        w.players[p].present = true;
    }
    let ticks = 30 * 60 * TPS as u32;
    for _ in 0..ticks {
        w.step(&ctx, &[]);
        if w.tick.is_multiple_of(60 * TPS as u32) {
            let kills: i32 = w.players.iter().map(|p| p.kills).sum();
            let creeps = w
                .units
                .iter()
                .filter(|u| u.alive && ctx.rules.units[u.def as usize].kind == pots_core::rules::Kind::Creep)
                .count();
            let heroes: Vec<[i32; 2]> = w
                .players
                .iter()
                .filter_map(|p| w.get(p.hero))
                .map(|u| u.pos())
                .collect();
            println!(
                "min {} tickets {:?} kills {kills} creeps {creeps} heroes {:?}",
                w.tick / (60 * TPS as u32),
                w.tickets,
                &heroes[..3]
            );
            let rules: Vec<_> = w.players.iter().map(|p| p.brain.rule).collect();
            let gold: Vec<i32> = w.players.iter().map(|p| p.gold).collect();
            println!("  rules {rules:?} gold {gold:?}");
        }
        if w.winner.is_some() {
            break;
        }
    }
    println!(
        "end tick {} ({} s) winner {:?} tickets {:?}",
        w.tick,
        w.tick / TPS as u32,
        w.winner,
        w.tickets
    );
}
