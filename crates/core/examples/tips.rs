//! Prints every ability's tip, as the game shows it, as JSON: the journal's
//! ability cards read it, so their words and numbers come from the core.
//!
//!     cargo run -q -p pots_core --example tips > journal/art2/tips.json

use pots_core::game::{Files, Game};
use pots_core::ui::ability_tip;

fn main() {
    let files = Files {
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
    };
    let g = Game::new(&files, 1).expect("the game's data loads");
    let tips: Vec<serde_json::Value> = (0..g.rules.abilities.len())
        .map(|k| {
            let a = &g.rules.abilities[k];
            serde_json::json!({ "id": a.id, "key": a.key, "tip": ability_tip(&g, k, 0) })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&tips).expect("tips serialise"));
}
