//! The map's look (T19, after Mage Tower Battle's newer one; journal tab
//! 10): the person chose it for every map asset and kept the current look
//! for characters. Map models are faceted, drawn without an outline and
//! stand on a soft shadow; heroes, creeps and creatures keep round 2's
//! smooth, outlined shapes; open ground carries seeded dressing.

use pots_core::game::{Files, Game};
use pots_core::mesh::Mesh;
use pots_core::models::Models;
use pots_core::render::{LAYER_INKED, LAYER_PLAIN, LAYER_TINT};

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

const FACETED: [Mesh; 4] = [Mesh::Facet, Mesh::FacetCyl, Mesh::FacetCone, Mesh::FacetTaper];
const ROUND: [Mesh; 5] = [Mesh::Sphere, Mesh::Blob, Mesh::Cyl, Mesh::Cone, Mesh::Taper];
const GLOW: i32 = 4;

fn match_game() -> Game {
    let mut g = Game::new(&files(), 21).unwrap();
    g.act("screen", "arcade").unwrap();
    g.act("custom", "").unwrap();
    g.act("pick", "0").unwrap();
    g.act("start", "").unwrap();
    g.frame(0);
    g
}

/// Instances in the scene by layer and mesh.
fn count(g: &Game, layer: usize, meshes: &[Mesh]) -> u32 {
    let (_, runs) = g.scene();
    runs.iter()
        .filter(|r| r.0 == layer && meshes.iter().any(|m| *m as u32 == r.1))
        .map(|r| r.3)
        .sum()
}

#[test]
fn map_models_are_faceted_without_an_outline_and_characters_keep_their_look() {
    let g = Game::new(&files(), 1).unwrap();
    let raw = Models::load(files().models).unwrap();
    for name in [
        "tower_sun",
        "barracks_melee_moon",
        "sphere_sun",
        "shop",
        "tree_sun",
        "tree_moon",
        "relic",
    ] {
        let m = &g.models.list[g.models.find(name).unwrap()];
        assert!(m.faceted, "{name} is in the map's look");
        for p in m.parts.iter().filter(|p| p.style != GLOW) {
            assert!(p.plain, "{name}: a part keeps its outline");
            assert!(
                !ROUND.contains(&p.mesh),
                "{name}: a part is still round ({:?})",
                p.mesh
            );
        }
        // glowing parts stay smooth, so their light fades evenly
        for p in m.parts.iter().filter(|p| p.style == GLOW) {
            assert!(!FACETED.contains(&p.mesh), "{name}: a glow was faceted");
        }
    }
    for name in [
        "hero_ironvow",
        "hero_rime",
        "creep_melee_sun",
        "creep_ranged_moon",
        "golem",
        "drake",
        "courier",
    ] {
        let m = &g.models.list[g.models.find(name).unwrap()];
        let r = &raw.list[raw.find(name).unwrap()];
        assert!(!m.faceted, "{name} keeps the characters' look");
        let same = m
            .parts
            .iter()
            .zip(&r.parts)
            .all(|(a, b)| a.mesh == b.mesh && a.plain == b.plain);
        assert!(same, "{name}'s parts are as models.json has them");
        assert!(m.parts.iter().any(|p| !p.plain), "{name} still has its outline");
    }
}

#[test]
fn the_match_draws_no_faceted_shape_with_an_outline_and_soft_shadows_under_the_map() {
    let g = match_game();
    assert_eq!(
        count(&g, LAYER_INKED, &FACETED),
        0,
        "no faceted shape is outlined"
    );
    assert!(
        count(&g, LAYER_PLAIN, &FACETED) > 0,
        "faceted trees and buildings are drawn"
    );
    assert!(
        count(&g, LAYER_INKED, &ROUND) > 0,
        "the heroes and creeps are still round and outlined"
    );
    // every shadow is two see-through faceted discs
    let shadows = count(&g, LAYER_TINT, &[Mesh::Facet]);
    assert!(
        shadows > 0 && shadows.is_multiple_of(2),
        "soft shadows, two discs each: {shadows}"
    );
}

#[test]
fn the_ground_is_dressed_the_same_every_frame_and_only_when_asked() {
    let mut g = match_game();
    let dressed = count(&g, LAYER_PLAIN, &FACETED);
    assert_eq!(g.scene(), g.scene(), "the same dressing every frame");
    g.look.map_look.dress_pct = 0;
    let bare = count(&g, LAYER_PLAIN, &FACETED);
    // a tuft is three cones and a flower two shapes, so dressing a third
    // of the open cells adds well over a hundred shapes near the camera
    assert!(
        dressed > bare + 100,
        "dressing adds stones, tufts and flowers: {dressed} against {bare}"
    );
}
