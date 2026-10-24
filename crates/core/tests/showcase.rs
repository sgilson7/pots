//! The heroes screen's inspector (round 7, T34): every hero opens on a
//! click, the camera turns with a drag and zooms with the wheel, and the
//! attack and each ability play on loop. Values worked out by hand from
//! `data/presentation.json` `showcase`: a degree is 65536 / 360 angle
//! units, rounded down (40° is 7281, 20° is 3640, 80° is 14563); a scene
//! unit is 65536.

use pots_core::game::{Files, Game};
use pots_core::showcase::Demo;
use pots_core::ui::{self, Node};

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

fn actions(n: &Node, a: &str, out: &mut Vec<String>) {
    if n.a == a {
        out.push(n.v.clone());
    }
    for k in &n.n {
        actions(k, a, out);
    }
}

fn on_screen(g: &Game, a: &str) -> Vec<String> {
    let mut out = Vec::new();
    actions(&ui::view(g).root, a, &mut out);
    out
}

fn heroes_screen() -> Game {
    let mut g = Game::new(&files(), 3).unwrap();
    g.act("screen", "heroes").unwrap();
    g
}

/// Runs `secs` seconds of display frames, 30 a second.
fn play(g: &mut Game, t: &mut i64, secs: i32) {
    for _ in 0..secs * 30 {
        *t += 33_334;
        g.frame(*t);
    }
}

#[test]
fn every_hero_on_the_heroes_screen_opens_its_inspector() {
    let mut g = heroes_screen();
    let cards = on_screen(&g, "heroes.view");
    let want: Vec<String> = (0..15).map(|k| k.to_string()).collect();
    assert_eq!(cards, want, "fifteen heroes, each clickable");
    g.act("heroes.view", "10").unwrap();
    let s = g.showcase.as_ref().expect("open");
    assert_eq!((s.hero, s.demo), (10, Demo::Stand));
    // the hero stands alone: nothing else within 1000 of it
    let h = s.world.get(s.hero_unit().unwrap()).unwrap().pos();
    assert_eq!(s.world.near(h, 1000, |u| u.alive).len(), 1);
    // its attack, its four abilities and the close form of Whirling Axes
    let demos = on_screen(&g, "heroes.demo");
    assert_eq!(
        demos,
        [
            "attack",
            "berserker_rage",
            "whirling_axes",
            "whirling_axes_melee",
            "fervor",
            "battle_trance"
        ]
    );
    g.act("heroes.close", "").unwrap();
    assert!(g.showcase.is_none());
    assert_eq!(on_screen(&g, "heroes.view").len(), 15, "back to every hero");
}

#[test]
fn a_drag_turns_the_camera_and_the_wheel_zooms_within_its_limits() {
    let mut g = heroes_screen();
    g.act("heroes.view", "0").unwrap();
    let s = g.showcase.as_ref().unwrap();
    // starting at 0° turn, 20° down, 15 scene units away
    assert_eq!((s.yaw, s.pitch, s.distance), (0, 3640, 15 * 65536));
    // 40° for each 100 pixels: 72 angle units a pixel
    g.click(0, 600, 300);
    g.pointer_move(700, 350, true);
    let s = g.showcase.as_ref().unwrap();
    assert_eq!((s.yaw, s.pitch), (-7200, 3640 + 3600));
    // a long drag down stops at 80°
    g.pointer_move(700, 2000, true);
    assert_eq!(g.showcase.as_ref().unwrap().pitch, 14563);
    // let go: moving the pointer turns nothing
    g.release();
    g.pointer_move(100, 100, true);
    assert_eq!(g.showcase.as_ref().unwrap().yaw, -7200);
    // a notch in is 2.5 units nearer; never nearer than 4.5 or farther than 42
    g.wheel(-100);
    assert_eq!(g.showcase.as_ref().unwrap().distance, 15 * 65536 - 163_840);
    for _ in 0..20 {
        g.wheel(-100);
    }
    assert_eq!(g.showcase.as_ref().unwrap().distance, 294_912);
    for _ in 0..40 {
        g.wheel(100);
    }
    assert_eq!(g.showcase.as_ref().unwrap().distance, 42 * 65536);
}

#[test]
fn an_ability_plays_on_loop_and_choosing_it_again_stops_it() {
    let mut g = heroes_screen();
    g.act("heroes.view", "11").unwrap();
    g.act("heroes.demo", "chakram").unwrap();
    let mut t = 0;
    g.frame(t);
    play(&mut g, &mut t, 2);
    {
        let s = g.showcase.as_ref().unwrap();
        assert_eq!(
            s.demo,
            Demo::Ability(g.rules.abilities.iter().position(|a| a.id == "chakram").unwrap())
        );
        assert!(!s.world.zones.is_empty(), "the blade is out and spinning");
        assert!(
            s.world.units.iter().filter(|u| u.alive).count() > 1,
            "with targets to cut"
        );
    }
    // Chakram plays 6 s, then starts again from its first tick
    let first = g.showcase.as_ref().unwrap().begun;
    play(&mut g, &mut t, 5);
    let s = g.showcase.as_ref().unwrap();
    assert!(
        s.world.tick < first + 2 * 30,
        "started again: tick {} from {first}",
        s.world.tick
    );
    g.act("heroes.demo", "chakram").unwrap();
    assert_eq!(g.showcase.as_ref().unwrap().demo, Demo::Stand);
}

#[test]
fn the_attack_plays_against_a_creep() {
    let mut g = heroes_screen();
    g.act("heroes.view", "8").unwrap();
    g.act("heroes.demo", "attack").unwrap();
    let s = g.showcase.as_ref().unwrap();
    let h = s.world.get(s.hero_unit().unwrap()).unwrap().pos();
    let near = s.world.near(h, 1000, |u| u.alive && u.hero.is_none());
    assert_eq!(near.len(), 1, "one creep to attack");
    let creep = near[0];
    let full = g.showcase.as_ref().unwrap().world.get(creep).unwrap().hp;
    let mut t = 0;
    g.frame(t);
    play(&mut g, &mut t, 3);
    let s = g.showcase.as_ref().unwrap();
    assert!(
        s.world.get(creep).map(|u| u.hp < full).unwrap_or(true),
        "the creep was hit"
    );
}

#[test]
fn the_numbers_shown_are_level_1_whatever_the_demonstration_plays_at() {
    // Hackle at level 1: 17 strength, 150 + 17 × 19 = 473 health
    let mut g = heroes_screen();
    g.act("heroes.view", "10").unwrap();
    g.act("heroes.demo", "battle_trance").unwrap();
    assert_eq!(g.showcase.as_ref().unwrap().level1.max_hp, 473);
    let mut texts = Vec::new();
    fn all(n: &Node, out: &mut Vec<String>) {
        out.push(n.x.clone());
        for k in &n.n {
            all(k, out);
        }
    }
    all(&ui::view(&g).root, &mut texts);
    assert!(texts.iter().any(|x| x.contains("473 health")), "{texts:?}");
}

#[test]
fn leaving_the_heroes_screen_closes_the_inspector() {
    let mut g = heroes_screen();
    g.act("heroes.view", "2").unwrap();
    g.act("screen", "title").unwrap();
    assert!(g.showcase.is_none());
    assert!(g.act("heroes.demo", "attack").is_err(), "no hero chosen");
}
