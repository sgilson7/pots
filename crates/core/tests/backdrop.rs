//! The title screen's backdrop (round 7): a match between computer players
//! behind the menu, with a camera that follows the busiest fight. Values
//! worked out by hand from `data/presentation.json` `backdrop`: it is drawn
//! from 92 seconds in (tick 2760), catching up 160 ticks a frame.

mod common;

use pots_core::backdrop::busiest;
use pots_core::game::{Files, Game};
use pots_core::world::{MOON, SUN};

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

fn shapes(g: &Game) -> u32 {
    g.scene().1.iter().map(|r| r.3).sum()
}

#[test]
fn a_match_plays_behind_the_title_once_it_reaches_the_first_clash() {
    let mut g = Game::new(&files(), 5).unwrap();
    assert_eq!(g.screen, "title");
    let mut t = 0;
    // 17 frames make 2720 ticks: not there yet, nothing drawn
    for _ in 0..17 {
        t += 16_667;
        g.frame(t);
    }
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2720);
    assert_eq!(shapes(&g), 0, "nothing is drawn while it catches up");
    // the 18th reaches tick 2760 and stops there
    t += 16_667;
    g.frame(t);
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2760);
    assert!(shapes(&g) > 1000, "the match is drawn");
    // the camera stands 700 west of the hero it follows, so the fight
    // shows right of the menu
    let b = g.backdrop.as_ref().unwrap();
    let h = b.world.get(b.follow).expect("a hero to follow");
    assert_eq!(g.view.target, [h.x / 16 - 700, h.y / 16]);
    // then it runs at the match's own 30 ticks a second
    for _ in 0..30 {
        t += 33_334;
        g.frame(t);
    }
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2790);
    assert!(g.m.is_none(), "the backdrop is not a match anyone plays");
    let heroes = g
        .backdrop
        .as_ref()
        .unwrap()
        .world
        .players
        .iter()
        .filter(|p| p.bot.is_some())
        .count();
    assert_eq!(heroes, 10, "every seat is a computer player");
}

#[test]
fn the_backdrop_waits_while_another_screen_shows() {
    let mut g = Game::new(&files(), 5).unwrap();
    let mut t = 0;
    for _ in 0..20 {
        t += 16_667;
        g.frame(t);
    }
    let tick = g.backdrop.as_ref().unwrap().world.tick;
    g.act("screen", "arcade").unwrap();
    for _ in 0..30 {
        t += 33_334;
        g.frame(t);
    }
    assert_eq!(
        g.backdrop.as_ref().unwrap().world.tick,
        tick,
        "it does not run behind the arcade"
    );
    g.act("screen", "title").unwrap();
    t += 33_334;
    g.frame(t);
    assert_eq!(
        g.backdrop.as_ref().unwrap().world.tick,
        tick + 1,
        "and goes on when the title is back"
    );
}

#[test]
fn the_camera_follows_the_hero_with_the_most_enemies_near() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(2);
    let a = w.add_hero(&ctx, 0, 0);
    let b = w.add_hero(&ctx, 1, 1);
    let c = w.add_hero(&ctx, 5, 2);
    // move the Sunward heroes apart; three Moonward creeps by the second
    let creep = f
        .rules
        .units
        .iter()
        .position(|u| u.id == "creep_melee_moon")
        .unwrap();
    for (id, at) in [(a, [3000, 3000]), (b, [8000, 6000]), (c, [13000, 13000])] {
        let u = w.get_mut(id).unwrap();
        u.x = at[0] * 16;
        u.y = at[1] * 16;
    }
    for k in 0..3 {
        w.spawn(&ctx, creep, MOON, [8100 + k * 50, 6000]);
    }
    assert_eq!(w.get(b).unwrap().team, SUN);
    for turn in 0..3 {
        assert_eq!(busiest(&w, 900, turn), b, "the fight wins whatever the turn");
    }
}

#[test]
fn with_reduced_motion_the_backdrop_is_a_still() {
    let mut g = Game::new(&files(), 5).unwrap();
    g.act("motion", "reduce").unwrap();
    let mut t = 0;
    for _ in 0..18 {
        t += 16_667;
        g.frame(t);
    }
    // it still catches up to tick 2760 so there is a picture to show
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2760);
    let at = g.view.target;
    for _ in 0..30 {
        t += 33_334;
        g.frame(t);
    }
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2760, "nothing moves");
    assert_eq!(g.view.target, at, "the camera stays put");
    g.act("motion", "full").unwrap();
    t += 33_334;
    g.frame(t);
    assert_eq!(g.backdrop.as_ref().unwrap().world.tick, 2761);
}

#[test]
fn a_machine_too_slow_to_draw_it_keeps_a_plain_title() {
    let mut g = Game::new(&files(), 5).unwrap();
    let mut t = 0;
    for _ in 0..18 {
        t += 16_667;
        g.frame(t);
    }
    assert!(shapes(&g) > 0, "drawn once caught up");
    // four frames of a fifth of a second: still drawn
    for _ in 0..4 {
        t += 200_000;
        g.frame(t);
    }
    assert!(shapes(&g) > 0, "four slow frames are not enough");
    // a fifth in a row stops it for good, even once frames are quick again
    t += 200_000;
    g.frame(t);
    assert_eq!(shapes(&g), 0, "five slow frames in a row: the plain title");
    let tick = g.backdrop.as_ref().unwrap().world.tick;
    for _ in 0..30 {
        t += 16_667;
        g.frame(t);
    }
    assert_eq!(shapes(&g), 0);
    assert_eq!(
        g.backdrop.as_ref().unwrap().world.tick,
        tick,
        "and it no longer runs"
    );
}
