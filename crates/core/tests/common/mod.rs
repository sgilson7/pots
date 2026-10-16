//! Loads the game's real data for tests that play matches.
#![allow(dead_code)]

use std::path::PathBuf;

use pots_core::ai::AiDef;
use pots_core::map::{Terrain, Tree};
use pots_core::rules::Rules;
use pots_core::world::{self, Ctx, World};

pub fn data(name: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data").join(name);
    std::fs::read_to_string(&root).unwrap_or_else(|e| panic!("read {}: {e}", root.display()))
}

pub fn rules() -> Rules {
    let mut r = Rules::load(
        &data("rules.json"),
        &data("units.json"),
        &data("abilities.json"),
        &data("items.json"),
        &data("map.json"),
    )
    .expect("rules load");
    r.ai = Some(AiDef::load(&data("ai.json")).expect("ai loads"));
    r
}

pub struct Fixture {
    pub rules: Rules,
    pub terrain: Terrain,
    pub trees: Vec<Tree>,
}

pub fn fixture() -> Fixture {
    let rules = rules();
    let (terrain, trees) = world::setup(&rules);
    Fixture { rules, terrain, trees }
}

impl Fixture {
    pub fn ctx(&self) -> Ctx<'_> {
        Ctx {
            rules: &self.rules,
            terrain: &self.terrain,
        }
    }

    pub fn world(&self, seed: u64) -> World {
        World::new(&self.ctx(), self.trees.clone(), seed)
    }
}
