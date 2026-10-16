//! The shop's build trees (T2): each built item shows its parts, the parts
//! a hero carries are marked and counted, and picking an item to look at
//! changes nothing in the match. Values worked out by hand from
//! data/items.json: a Magic Wand is a Magic Stick (200), three Branches
//! (53 each) and a 150 recipe, 509 in all.

mod common;

use pots_core::item;
use pots_core::world::ItemSlot;

#[test]
fn a_built_item_s_tree_lists_its_parts_and_recipe() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    w.add_hero(&ctx, 0, 0);
    let wand = f.rules.item("magic_wand").unwrap();
    let t = item::tree(&w, &ctx, 0, wand);
    assert_eq!(t.item, wand);
    assert!(!t.owned);
    assert_eq!(t.recipe, 150, "the wand's recipe costs 150");
    let names: Vec<&str> = t
        .parts
        .iter()
        .map(|p| f.rules.items[p.item].id.as_str())
        .collect();
    assert_eq!(names, ["magic_stick", "branch", "branch", "branch"]);
    assert!(t.parts.iter().all(|p| !p.owned && p.parts.is_empty()));
}

#[test]
fn a_carried_part_is_marked_once_and_taken_off_the_price() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    let h = w.add_hero(&ctx, 0, 0);
    let wand = f.rules.item("magic_wand").unwrap();
    let branch = f.rules.item("branch").unwrap();
    w.get_mut(h).unwrap().items[0] = ItemSlot {
        item: branch as u16 + 1,
        charges: 0,
        ready_at: 0,
    };
    let t = item::tree(&w, &ctx, 0, wand);
    let owned: Vec<bool> = t.parts.iter().map(|p| p.owned).collect();
    assert_eq!(
        owned,
        [false, true, false, false],
        "one branch carried covers one branch"
    );
    // 509 less the carried branch's 53
    assert_eq!(item::price(&w, &ctx, 0, wand), 456);
}

#[test]
fn a_tree_goes_down_through_parts_that_are_built_themselves() {
    let f = common::fixture();
    let ctx = f.ctx();
    let mut w = f.world(1);
    w.add_hero(&ctx, 0, 0);
    let bf = f.rules.item("battlefury").unwrap();
    let t = item::tree(&w, &ctx, 0, bf);
    assert!(
        t.parts.iter().any(|p| !p.parts.is_empty()),
        "a Battle Fury part is itself built from parts"
    );
}

#[test]
fn a_part_lists_what_it_builds_into() {
    let f = common::fixture();
    let ctx = f.ctx();
    let branch = f.rules.item("branch").unwrap();
    let mut into: Vec<&str> = item::builds_into(&ctx, branch)
        .iter()
        .map(|&j| f.rules.items[j].id.as_str())
        .collect();
    into.sort();
    assert_eq!(into, ["buckler", "headdress", "magic_wand"]);
}

#[test]
fn picking_an_item_in_the_shop_changes_nothing_in_the_match() {
    // online play depends on it: a pick is the page's, never an order.
    // Two matches from the same seed, one with the shop open and an item
    // picked, are the same world a second later.
    let start = || {
        let mut g = pots_core::game::Game::new(&common_files(), 3).unwrap();
        g.act("mission", "0").unwrap();
        g.act("pick", "0").unwrap();
        g.act("start", "").unwrap();
        g.frame(0);
        g
    };
    let mut plain = start();
    let mut looked = start();
    let wand = looked.rules.item("magic_wand").unwrap();
    looked.act("shop.toggle", "").unwrap();
    looked.act("shop.pick", &wand.to_string()).unwrap();
    assert_eq!(looked.shop_pick, wand as i32);
    for t in 1..=30 {
        plain.frame(t * 33_334);
        looked.frame(t * 33_334);
    }
    let (a, b) = (
        &plain.m.as_ref().unwrap().world,
        &looked.m.as_ref().unwrap().world,
    );
    assert!(a.tick > 10, "the matches ran");
    assert!(a == b, "the world is the same with or without the pick");
    assert!(
        looked.act("shop.pick", "9999").is_err(),
        "an unknown item is refused"
    );
}

fn common_files() -> pots_core::game::Files<'static> {
    pots_core::game::Files {
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
