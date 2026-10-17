//! All Random All Mid through the screens (round 4): the custom match's
//! mode switch, the dealt hero and its one reroll, and the cannon's key.

use pots_core::game::{Files, Game};
use pots_core::overlay::Area;
use pots_core::ui::{view, Node};

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

fn find<'a>(n: &'a Node, a: &str, out: &mut Vec<&'a Node>) {
    if n.a == a {
        out.push(n);
    }
    for k in &n.n {
        find(k, a, out);
    }
}

#[test]
fn the_custom_match_deals_a_hero_allows_one_reroll_and_starts_in_the_mode() {
    let mut g = Game::new(&files(), 11).unwrap();
    g.act("screen", "arcade").unwrap();
    g.act("setup.mode", "").unwrap();
    g.act("custom", "").unwrap();
    let dealt = g.arcade_deal();
    assert_eq!(
        g.setup.hero, dealt[0] as i32,
        "the pick screen shows the dealt hero"
    );
    assert!(g.act("pick", "3").is_err(), "no choosing in this mode");
    let mut picks = Vec::new();
    let v = view(&g);
    find(&v.root, "pick", &mut picks);
    assert!(picks.is_empty(), "the hero cards are not buttons here");
    g.act("reroll", "").unwrap();
    let again = g.setup.hero;
    assert_ne!(again, dealt[0] as i32, "the reroll dealt another hero");
    assert!(g.act("reroll", "").is_err(), "one reroll only");
    let seed = g.arcade_seed();
    g.act("start", "").unwrap();
    g.frame(0);
    let m = g.m.as_ref().unwrap();
    assert!(m.world.aram, "the match is in the mode");
    let mut rr = [false; pots_core::world::PLAYERS];
    rr[0] = true;
    let deal = pots_core::aram::deal(seed, g.rules.heroes.len(), &rr);
    assert_eq!(deal[0] as i32, again);
    for (p, pl) in m.world.players.iter().enumerate() {
        if !pl.present {
            continue;
        }
        let def = m.world.get(pl.hero).unwrap().def as usize;
        assert_eq!(
            def, g.rules.heroes[deal[p] as usize],
            "seat {p} plays its dealt hero"
        );
    }
    // the HUD's cannon button, and F aims the cannon from the fountain
    let v = view(&g);
    let mut buttons = Vec::new();
    find(&v.root, "cannon", &mut buttons);
    assert_eq!(buttons.len(), 1, "a cannon button on the HUD");
    assert!(!buttons[0].off, "and it may fire from the fountain");
    g.key("KeyF", true);
    assert_eq!(
        g.overlay().aim.map(|a| a.area),
        Some(Area::Cannon),
        "F aims the cannon"
    );
}

#[test]
fn a_normal_custom_match_still_lets_you_choose_and_has_no_cannon() {
    let mut g = Game::new(&files(), 12).unwrap();
    g.act("screen", "arcade").unwrap();
    g.act("custom", "").unwrap();
    assert_eq!(g.setup.hero, -1, "nothing dealt");
    g.act("pick", "3").unwrap();
    assert!(g.act("reroll", "").is_err(), "no reroll outside the mode");
    g.act("start", "").unwrap();
    g.frame(0);
    assert!(!g.m.as_ref().unwrap().world.aram);
    let v = view(&g);
    let mut buttons = Vec::new();
    find(&v.root, "cannon", &mut buttons);
    assert!(buttons.is_empty(), "no cannon button");
    assert!(g.act("cannon", "").is_err(), "and no cannon to fire");
}
