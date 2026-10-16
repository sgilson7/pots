//! Every ability can be seen (T3): cast by its hero at full levels against
//! enemy creeps, each active ability leaves the marks its look draws, and
//! the marks never touch the match itself.

mod common;

use pots_core::game::{Files, Game};
use pots_core::world::{MK_CAST, MK_FIRE, MK_LAND, MK_PROC};

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

/// The kinds of mark ability `id` leaves in its first three seconds.
fn marks_of(g: &Game, id: &str) -> Vec<u8> {
    let k = g.rules.abilities.iter().position(|a| a.id == id).unwrap();
    let mut w = g.cast_world(id, 1).unwrap();
    let ctx = g.ctx();
    let mut seen = Vec::new();
    for _ in 0..90 {
        for m in &w.marks {
            if m.ability as usize == k && !seen.contains(&m.kind) {
                seen.push(m.kind);
            }
        }
        w.step(&ctx, &[]);
    }
    seen
}

#[test]
fn every_active_ability_is_cast_and_let_go_where_it_can_be_seen() {
    let g = Game::new(&files(), 1).unwrap();
    let mut missing = Vec::new();
    for a in &g.rules.abilities {
        if a.id == "warden_bash" || matches!(a.target, pots_core::rules::Target::Passive) {
            continue;
        }
        let seen = marks_of(&g, &a.id);
        if !seen.contains(&MK_FIRE) {
            missing.push(format!("{} (saw {seen:?})", a.id));
        }
    }
    assert!(missing.is_empty(), "no fire mark: {missing:?}");
}

#[test]
fn a_thrown_hammer_is_cast_then_lands() {
    let g = Game::new(&files(), 1).unwrap();
    let seen = marks_of(&g, "storm_bolt");
    for kind in [MK_CAST, MK_FIRE, MK_LAND] {
        assert!(seen.contains(&kind), "Vow Hammer left {seen:?}");
    }
}

#[test]
fn an_attack_passive_shows_when_it_fires() {
    let g = Game::new(&files(), 1).unwrap();
    assert!(
        marks_of(&g, "great_cleave").contains(&MK_PROC),
        "Wide Arc shows on a hit"
    );
}

#[test]
fn every_ability_has_a_look_in_known_colours_and_shapes() {
    let g = Game::new(&files(), 1).unwrap();
    let shapes = ["ring", "flash", "sky", "beam", "wave", "spikes", "rise", "spin"];
    let zones = ["", "warn", "cloud", "rain", "spin", "heal", "flash", "none"];
    for a in &g.rules.abilities {
        let l = g
            .look
            .fx
            .get(&a.id)
            .unwrap_or_else(|| panic!("{} has no look", a.id));
        assert!(
            g.models.palette_names.contains(&l.color),
            "{}: no colour {}",
            a.id,
            l.color
        );
        assert!(zones.contains(&l.zone.as_str()), "{}: zone {}", a.id, l.zone);
        for s in l.fire.iter().chain(&l.land).chain(&l.procs) {
            assert!(shapes.contains(&s.kind.as_str()), "{}: shape {}", a.id, s.kind);
            assert!(
                s.key.is_empty() || a.values.contains_key(&s.key),
                "{}: no value {}",
                a.id,
                s.key
            );
        }
    }
}

#[test]
fn marks_leave_the_online_checksum_alone() {
    // two copies of one cast: one with its marks cleared every tick
    let g = Game::new(&files(), 1).unwrap();
    let ctx = g.ctx();
    let mut a = g.cast_world("light_strike_array", 1).unwrap();
    let mut b = a.clone();
    for _ in 0..60 {
        a.step(&ctx, &[]);
        b.step(&ctx, &[]);
        b.marks.clear();
        assert_eq!(a.checksum(), b.checksum());
    }
    assert!(
        a.units.iter().zip(&b.units).all(|(x, y)| x == y),
        "the units are the same"
    );
}
