//! Every button the core draws names an action the core accepts. The copy
//! buttons in the lobby once named one it did not, and did nothing.

use pots_core::game::{Files, Game};
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

fn buttons(n: &Node, out: &mut Vec<(String, String)>) {
    if !n.a.is_empty() {
        out.push((n.a.clone(), n.v.clone()));
    }
    for k in &n.n {
        buttons(k, out);
    }
}

/// Each screen a player can reach, as the steps that reach it.
const SCREENS: &[&[(&str, &str)]] = &[
    &[],
    &[("screen", "arcade")],
    &[("screen", "heroes")],
    &[("screen", "help")],
    &[("screen", "arcade"), ("mission", "0")],
    &[("screen", "online")],
    &[("screen", "online"), ("online.host", "")],
    &[
        ("screen", "arcade"),
        ("mission", "0"),
        ("pick", "0"),
        ("start", ""),
    ],
    &[
        ("screen", "arcade"),
        ("mission", "0"),
        ("pick", "0"),
        ("start", ""),
        ("shop.toggle", ""),
    ],
];

fn reach(steps: &[(&str, &str)]) -> Game {
    let mut g = Game::new(&files(), 3).unwrap();
    for (a, v) in steps {
        g.act(a, v).unwrap();
    }
    g.frame(0);
    g
}

#[test]
fn every_button_on_every_screen_names_an_action_the_core_accepts() {
    let mut seen = 0;
    for steps in SCREENS {
        let mut found = Vec::new();
        buttons(&view(&reach(steps)).root, &mut found);
        assert!(!found.is_empty(), "the screen after {steps:?} has buttons");
        for (a, v) in found {
            // a fresh copy of the screen for each, so one button cannot hide another
            if let Err(e) = reach(steps).act(&a, &v) {
                assert!(
                    !e.starts_with("Unknown action"),
                    "after {steps:?}, the button \"{a}\" does nothing: {e}"
                );
            }
            seen += 1;
        }
    }
    assert!(seen > 40, "only {seen} buttons were tried");
}

#[test]
fn copying_the_room_link_says_it_was_copied() {
    let mut g = reach(&[("screen", "online"), ("online.host", "")]);
    let mut found = Vec::new();
    buttons(&view(&g).root, &mut found);
    let (a, v) = found
        .into_iter()
        .find(|(a, _)| a == "copy.link")
        .expect("the lobby has a copy link button");
    assert!(v.starts_with("room="), "the link names the room: {v}");
    g.act(&a, &v).unwrap();
    assert_eq!(view(&g).notice, g.strings.ui("online.copied"));
}
