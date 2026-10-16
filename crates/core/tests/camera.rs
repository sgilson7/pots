//! The camera pans at every edge of the window (T4): at the bottom and the
//! left the HUD and the minimap lie over the 3D view, and the pointer there
//! must still pan, as it does at the top and the right.

use pots_core::game::{Files, Game};

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

/// Where the camera looks after a second with the pointer at (x, y) of a
/// 1600 × 900 window, over the 3D view or not.
fn pan(x: i32, y: i32, over_view: bool) -> ([i32; 2], [i32; 2]) {
    let mut g = Game::new(&files(), 3).unwrap();
    g.resize(1600, 900, 100);
    g.act("mission", "0").unwrap();
    g.act("pick", "0").unwrap();
    g.act("start", "").unwrap();
    g.view.target = [8000, 8000];
    g.frame(0);
    let before = g.view.target;
    g.pointer_move(x, y, over_view);
    g.frame(1_000_000);
    (before, g.view.target)
}

#[test]
fn the_bottom_left_corner_pans_down_and_left_over_the_hud() {
    let (a, b) = pan(3, 897, false);
    assert!(b[0] < a[0], "left: {a:?} to {b:?}");
    assert!(b[1] < a[1], "down: {a:?} to {b:?}");
}

#[test]
fn the_top_right_corner_pans_up_and_right() {
    let (a, b) = pan(1597, 3, true);
    assert!(b[0] > a[0] && b[1] > a[1], "{a:?} to {b:?}");
}

#[test]
fn the_middle_of_the_window_does_not_pan() {
    let (a, b) = pan(800, 450, true);
    assert_eq!(a, b);
}

#[test]
fn the_locked_cursor_stops_at_the_window_s_edges() {
    let mut g = Game::new(&files(), 3).unwrap();
    g.resize(1600, 900, 100);
    g.pointer_move(100, 800, true);
    // 100 - 300 stops at 0; 800 + 300 stops at 899
    assert_eq!(g.nudge(-300, 300), [0, 899]);
    assert_eq!(g.nudge(40, -50), [40, 849]);
}
