//! Diagnostic, ignored by default: which rule each bot holds over a match.
mod common;
use pots_core::rules::TPS;
#[test]
#[ignore]
fn rule_histogram() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    for p in 0..10 {
        w.players[p].bot = Some(2);
        w.players[p].present = true;
    }
    let mut hist = vec![[0u32; 20]; 10];
    let mut far = [0u32; 10];
    for _ in 0..(12 * 60 * TPS) {
        w.step(&ctx, &[]);
        if w.tick.is_multiple_of(30) {
            for p in 0..10 {
                let r = w.players[p].brain.rule.clamp(0, 19) as usize;
                hist[p][r] += 1;
                if let Some(u) = w.get(w.players[p].hero) {
                    let home = ctx.rules.map.sides[u.team as usize].fountain;
                    if pots_core::world::dist(u.pos(), home) < 3000 {
                        far[p] += 1;
                    }
                }
            }
        }
    }
    for p in 0..10 {
        println!(
            "seat {p}: rules {:?} near-home {} lh {}",
            &hist[p][..16],
            far[p],
            w.players[p].last_hits
        );
    }
}

#[test]
#[ignore]
fn lone_bot_damage_sources() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    w.players[0].bot = Some(2);
    w.players[0].present = true;
    let mut by_kind = std::collections::BTreeMap::new();
    let mut last_hp = 0;
    let mut homes = 0;
    let mut was_home = false;
    for _ in 0..(10 * 60 * TPS) {
        w.step(&ctx, &[]);
        let h = w.players[0].hero;
        if let Some(u) = w.get(h) {
            if u.hp < last_hp {
                let k = w
                    .get(u.last_attacker)
                    .map(|a| format!("{:?}", ctx.rules.units[a.def as usize].kind))
                    .unwrap_or("?".into());
                *by_kind.entry(k).or_insert(0) += (last_hp - u.hp) / 1000;
            }
            last_hp = u.hp;
            let home = w.players[0].brain.rule == 2;
            if home && !was_home {
                homes += 1;
            }
            was_home = home;
        }
    }
    println!(
        "damage by {by_kind:?} trips home {homes} lh {} dn {} level {} farm {:?} hero {}",
        w.players[0].last_hits,
        w.players[0].denies,
        w.get(w.players[0].hero).unwrap().level(),
        w.players[0].brain.farm,
        ctx.rules.units[w.get(w.players[0].hero).unwrap().def as usize].id
    );
}

#[test]
#[ignore]
fn one_attack_on_a_low_creep() {
    use pots_core::orders::Cmd;
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(7);
    let h = w.add_hero(&ctx, 0, 0);
    let def = f.rules.unit("creep_melee_moon").unwrap();
    let at = w.get(h).unwrap().pos();
    let c = w.spawn(&ctx, def, pots_core::world::MOON, [at[0] + 300, at[1]]);
    w.get_mut(c).unwrap().hp = 40_000;
    w.step(&ctx, &[(0, Cmd::Attack(c))]);
    for t in 0..90 {
        let u = w.get(h).unwrap();
        let cu = w.get(c).unwrap();
        if t % 5 == 0 || !cu.alive {
            println!(
                "t{t} hero {:?} act {:?} order {:?} target {} anim {} creep alive {} hp {} lh {}",
                u.pos(),
                u.act,
                u.order,
                u.target,
                u.anim,
                cu.alive,
                cu.hp,
                w.players[0].last_hits
            );
        }
        if !cu.alive {
            break;
        }
        w.step(&ctx, &[]);
    }
}

#[test]
#[ignore]
fn how_long_a_bot_match_lasts() {
    let f = common::fixture();
    let ctx = f.ctx();
    for seed in [3u64, 11] {
        let mut w = f.world(seed);
        for p in 0..10 {
            w.players[p].bot = Some(2);
            w.players[p].present = true;
        }
        let mut towers_at = Vec::new();
        while w.winner.is_none() && w.tick < 60 * 60 * TPS as u32 {
            w.step(&ctx, &[]);
            if w.tick.is_multiple_of(5 * 60 * TPS as u32) {
                let n = w
                    .units
                    .iter()
                    .filter(|u| {
                        u.alive && ctx.rules.units[u.def as usize].kind == pots_core::rules::Kind::Tower
                    })
                    .count();
                towers_at.push((w.tick / (60 * TPS as u32), n));
            }
        }
        println!(
            "seed {seed}: winner {:?} at minute {} towers {:?}",
            w.winner,
            w.tick / (60 * TPS as u32),
            towers_at
        );
    }
}
