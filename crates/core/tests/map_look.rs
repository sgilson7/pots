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
fn every_model_is_faceted_without_an_outline_and_glows_stay_smooth() {
    // round 7: tab 10's look for every model, characters included
    let g = Game::new(&files(), 1).unwrap();
    let raw = Models::load(files().models).unwrap();
    assert!(g.look.map_look.keep_smooth.is_empty(), "no model is kept smooth");
    for (m, r) in g.models.list.iter().zip(&raw.list) {
        let name = &m.name;
        assert!(m.faceted, "{name} is in the new look");
        for (p, q) in m.parts.iter().zip(&r.parts) {
            if p.style == GLOW {
                // glowing parts stay smooth, so their light fades evenly
                assert_eq!(p.mesh, q.mesh, "{name}: a glow was faceted");
            } else {
                assert!(p.plain, "{name}: a part keeps its outline");
                assert!(
                    !ROUND.contains(&p.mesh),
                    "{name}: a part is still round ({:?})",
                    p.mesh
                );
            }
        }
    }
    // the heroes were round and outlined in models.json; now they are not
    for name in ["hero_ironvow", "hero_rime", "creep_melee_sun", "golem", "courier"] {
        let r = &raw.list[raw.find(name).unwrap()];
        assert!(
            r.parts.iter().any(|p| ROUND.contains(&p.mesh) && !p.plain),
            "{name} was round"
        );
    }
}

#[test]
fn a_model_named_to_keep_smooth_keeps_its_round_outlined_look() {
    let mut f = files();
    let pres = include_str!("../../../data/presentation.json")
        .replace("\"keep_smooth\": []", "\"keep_smooth\": [\"hero_ironvow\"]");
    assert!(pres.contains("hero_ironvow\"]"), "the edit matched");
    f.presentation = Box::leak(pres.into_boxed_str());
    let g = Game::new(&f, 1).unwrap();
    let m = &g.models.list[g.models.find("hero_ironvow").unwrap()];
    assert!(!m.faceted);
    assert!(m.parts.iter().any(|p| !p.plain && ROUND.contains(&p.mesh)));
}

#[test]
fn the_match_draws_nothing_outlined_and_soft_shadows_under_units_and_the_map() {
    let g = match_game();
    let (_, runs) = g.scene();
    let inked: u32 = runs.iter().filter(|r| r.0 == LAYER_INKED).map(|r| r.3).sum();
    assert_eq!(inked, 0, "no shape is outlined");
    assert!(count(&g, LAYER_PLAIN, &FACETED) > 0, "faceted shapes are drawn");
    assert_eq!(
        count(&g, LAYER_INKED, &ROUND),
        0,
        "no round outlined hero or creep"
    );
    // every shadow is two see-through faceted discs, under the heroes too:
    // ten heroes stand at the fountains, so at least twenty discs
    let shadows = count(&g, LAYER_TINT, &[Mesh::Facet]);
    assert!(
        shadows >= 20 && shadows.is_multiple_of(2),
        "soft shadows, two discs each: {shadows}"
    );
    // the heroes stand on shadows of their own: with the heroes taken off
    // the field (not alive, and not falling) the frame draws two discs
    // fewer for each one in view
    let mut g = g;
    let near = count(&g, LAYER_TINT, &[Mesh::Facet]);
    let heroes: Vec<_> =
        g.m.as_ref()
            .unwrap()
            .world
            .players
            .iter()
            .map(|p| p.hero)
            .collect();
    for &h in &heroes {
        if let Some(u) = g.m.as_mut().unwrap().world.get_mut(h) {
            u.alive = false;
        }
    }
    let far = count(&g, LAYER_TINT, &[Mesh::Facet]);
    assert!(
        near >= far + 2,
        "a hero's shadow: {near} with the heroes, {far} without"
    );
    // shots and spells in flight: a volley of every shooter, a hammer, a hook, a rock wall
    for stage in [
        "volley:20",
        "cast:storm_bolt,12",
        "cast:meat_hook,10",
        "cast:fissure,30",
    ] {
        let mut g = Game::new(&files(), 4).unwrap();
        g.set_stage(stage).unwrap();
        g.frame(0);
        let (_, runs) = g.scene();
        let inked: u32 = runs.iter().filter(|r| r.0 == LAYER_INKED).map(|r| r.3).sum();
        assert_eq!(inked, 0, "{stage}: no shape is outlined");
    }
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

#[test]
fn the_lanes_are_paved_and_their_dirt_varies() {
    // round 7: the lane was one flat colour; now its dirt changes shade
    // from patch to patch, its edge blends into the grass, grass creeps in
    // at the edge and flagstones lie down the middle
    let mut g = match_game();
    // the middle lane between the Sunward's outer and second towers, where
    // it is wide enough to have a middle
    g.view.target = [5600, 5600];
    assert_eq!(g.scene(), g.scene(), "the same road every frame");
    let both = count(&g, LAYER_PLAIN, &FACETED);
    g.look.map_look.lane.creep_pct = 0;
    let paved = count(&g, LAYER_PLAIN, &FACETED);
    g.look.map_look.lane.stone_pct = 0;
    let bare = count(&g, LAYER_PLAIN, &FACETED);
    assert!(
        paved > bare + 10,
        "flagstones on the lane near the camera: {paved} against {bare}"
    );
    assert!(
        both > paved + 10,
        "grass creeping onto the lane's edge: {both} against {paved}"
    );
    // with the edge blend off on both sides, the patches alone change the colours
    g.look.map_look.lane.edge_pct = 0;
    let patched = g.scene();
    g.look.map_look.lane.dark_pct = 0;
    g.look.map_look.lane.light_pct = 0;
    let flat = g.scene();
    assert_eq!(patched.1.len(), flat.1.len());
    assert_ne!(patched, flat, "the lane's dirt is not one colour");
    // and the edge blend changes them too
    g.look.map_look.lane.edge_pct = 45;
    assert_ne!(g.scene(), flat, "the lane's edge blends into the grass");
}

#[test]
fn the_ground_is_one_faceted_surface_with_rocks_along_its_cliffs() {
    // round 7: the Sunward base's east face, where high ground meets the forest
    let mut g = match_game();
    g.view.target = [4500, 2200];
    let tris = count(&g, LAYER_PLAIN, &[Mesh::Tri]);
    assert!(tris > 2000, "the ground is drawn as triangles: {tris}");
    assert!(count(&g, LAYER_PLAIN, &[Mesh::Cube]) < 100, "no block per cell");
    let rocks = count(&g, LAYER_PLAIN, &[Mesh::Facet]);
    g.look.map_look.ground.cliff_rocks = 0;
    let bare = count(&g, LAYER_PLAIN, &[Mesh::Facet]);
    // four boulders on each cliff side of a cell, and dozens of sides in view
    assert!(
        rocks > bare + 40,
        "boulders line the cliff: {rocks} against {bare}"
    );
}

#[test]
fn lane_creeps_are_drawn_smaller_than_heroes_and_neutrals() {
    // round 7: creeps at 85% of their own size, so they read apart from heroes
    let g = Game::new(&files(), 1).unwrap();
    let full = |id: &str| {
        let d = g.rules.units.iter().find(|u| u.id == id).unwrap();
        let undiminished = d.scale as i64 * g.look.unit_scale as i64 * pots_core::fx::ONE as i64 / 10_000;
        (pots_core::scene::drawn_scale(&g.look, d) as i64 * 100 + undiminished / 2) / undiminished
    };
    for id in ["creep_melee_sun", "creep_ranged_moon", "creep_siege_sun"] {
        assert_eq!(full(id), 85, "{id} is drawn at 85%");
    }
    for id in ["ironvow", "golem", "warden"] {
        assert_eq!(full(id), 100, "{id} keeps its size");
    }
}
