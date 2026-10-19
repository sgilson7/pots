//! Round 3 (T6, T8–T11): what the player's own hand shows. Each shooter's
//! shot has a look of its own; a right-click on a shop item buys it; an
//! aimed ability shows the ground it will cover; an order click is marked
//! where it landed; a teleport glows at both ends while it channels. None
//! of the marks the hand shows reach the world.

use pots_core::game::{Files, Game};
use pots_core::marks::shot_for;
use pots_core::orders::Cmd;
use pots_core::overlay::{ability_area, Area};
use pots_core::rules::SUB;
use pots_core::ui::{view, Node};
use pots_core::world::{ItemSlot, NO_UNIT};

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

/// An arcade match as `hero`, one frame in.
fn match_as(hero: &str) -> Game {
    let mut g = Game::new(&files(), 3).unwrap();
    g.act("screen", "arcade").unwrap();
    g.act("mission", "0").unwrap();
    let r = g
        .rules
        .heroes
        .iter()
        .position(|&u| g.rules.units[u].id == hero)
        .unwrap();
    g.act("pick", &r.to_string()).unwrap();
    g.act("start", "").unwrap();
    g.frame(0);
    g
}

/// Runs display frames of `us` microseconds each.
fn run(g: &mut Game, frames: i64, us: i64, from: &mut i64) {
    for _ in 0..frames {
        *from += us;
        g.frame(*from);
    }
}

fn instances(g: &Game) -> u32 {
    g.scene().1.iter().map(|r| r.3).sum()
}

#[test]
fn every_shooter_has_a_shot_of_its_own() {
    let g = Game::new(&files(), 3).unwrap();
    let known = [
        "orb", "bolt", "arrow", "streak", "shard", "flame", "boulder", "spark", "hammer", "hook",
    ];
    let mut looks = Vec::new();
    for d in g.rules.units.iter().filter(|d| d.missile > 0) {
        let s = shot_for(&g.look.shots, &d.id).unwrap_or_else(|| panic!("{} has no shot", d.id));
        assert!(
            known.contains(&s.shape.as_str()),
            "{}: unknown shape {}",
            d.id,
            s.shape
        );
        looks.push((d.id.clone(), d.kind, s.shape.clone(), s.color.clone(), s.size));
    }
    // a tower's bolt, a ranged creep's and each ranged hero's read apart
    let mut seen: Vec<(String, String, i32)> = Vec::new();
    for (id, kind, shape, color, size) in &looks {
        let one_of_a_kind = matches!(kind, pots_core::rules::Kind::Hero)
            || id == "tower1_sun"
            || id == "creep_ranged_sun"
            || id == "creep_siege_sun";
        if !one_of_a_kind {
            continue;
        }
        let key = (shape.clone(), color.clone(), *size);
        assert!(!seen.contains(&key), "{id} looks like another shooter: {key:?}");
        seen.push(key);
    }
    assert!(seen.len() >= 9, "only {} shooters compared", seen.len());
    // the longest matching key wins: a mega creep shoots as a ranged creep
    assert_eq!(
        shot_for(&g.look.shots, "creep_ranged_mega_moon").map(|s| s.size),
        g.look.shots.get("creep_ranged").map(|s| s.size)
    );
}

fn find<'a>(n: &'a Node, class: &str, out: &mut Vec<&'a Node>) {
    if n.c.split(' ').any(|c| c == class) {
        out.push(n);
    }
    for k in &n.n {
        find(k, class, out);
    }
}

#[test]
fn a_right_click_on_an_item_in_the_shop_buys_it() {
    let mut g = match_as("ironvow");
    g.act("shop.toggle", "").unwrap();
    let v = view(&g);
    let mut items = Vec::new();
    find(&v.root, "shop-item", &mut items);
    assert!(!items.is_empty(), "the shop lists items");
    for n in &items {
        assert_eq!(n.alt, "buy", "a right-click on {} buys it", n.v);
    }
    let cheap = items
        .iter()
        .find(|n| !n.c.contains("dim"))
        .expect("something is affordable");
    let (alt, value) = (cheap.alt.clone(), cheap.v.clone());
    let me = g.my_hero();
    let gold = |g: &Game| g.m.as_ref().unwrap().world.players[0].gold;
    let before = gold(&g);
    g.act(&alt, &value).unwrap();
    let mut t = 0;
    run(&mut g, 4, 33_334, &mut t);
    let w = &g.m.as_ref().unwrap().world;
    let k: u16 = value.parse::<u16>().unwrap() + 1;
    assert!(gold(&g) < before, "the right-click spent gold");
    assert!(
        w.get(me).unwrap().items.iter().any(|s| s.item == k),
        "the item is in the inventory"
    );
}

#[test]
fn a_right_click_on_a_carried_item_near_a_shop_sells_it_for_half() {
    // round 7. A Magic Wand is a Magic Stick (200), three Branches (53
    // each) and a 150 recipe: 509 in all, so it sells for 254.
    let mut g = match_as("ironvow");
    let me = g.my_hero();
    let wand = g.rules.item("magic_wand").unwrap() as u16 + 1;
    let gold = |g: &Game| g.m.as_ref().unwrap().world.players[0].gold;
    {
        let w = &mut g.m.as_mut().unwrap().world;
        w.get_mut(me).unwrap().items[0] = ItemSlot {
            item: wand,
            charges: 0,
            ready_at: 0,
        };
    }
    let slot = |g: &Game| {
        let v = view(g);
        let mut items = Vec::new();
        find(&v.root, "item", &mut items);
        items
            .into_iter()
            .find(|n| n.v == "0")
            .cloned()
            .expect("the first slot")
    };
    // at the fountain, by the base shop
    let n = slot(&g);
    assert_eq!(n.alt, "sell", "a right-click on a carried item sells it");
    let price = |n: &Node| n.n.iter().find(|k| k.c == "sell-price").map(|k| k.x.clone());
    assert_eq!(price(&n), None, "no price tag with the shop closed");
    g.act("shop.toggle", "").unwrap();
    assert_eq!(
        price(&slot(&g)).as_deref(),
        Some("254"),
        "with the shop open the slot shows its price"
    );
    let before = gold(&g);
    g.act(&n.alt, &n.v).unwrap();
    let mut t = 0;
    run(&mut g, 4, 33_334, &mut t);
    assert_eq!(gold(&g) - before, 254, "half the wand's 509, rounded down");
    let w = &g.m.as_ref().unwrap().world;
    assert_eq!(w.get(me).unwrap().items[0].item, 0, "the slot is empty");
    // out in the middle of the map no shop is in reach: no sale
    {
        let w = &mut g.m.as_mut().unwrap().world;
        let u = w.get_mut(me).unwrap();
        u.items[0] = ItemSlot {
            item: wand,
            charges: 0,
            ready_at: 0,
        };
        u.x = 8192 * SUB;
        u.y = 8192 * SUB;
    }
    let n = slot(&g);
    assert_eq!(n.alt, "", "away from a shop a right-click sells nothing");
}

#[test]
fn each_aimed_ability_outlines_the_ground_its_own_numbers_cover() {
    let g = Game::new(&files(), 3).unwrap();
    let area = |id: &str| {
        let a = g.rules.abilities.iter().find(|a| a.id == id).unwrap();
        ability_area(a, a.max_level())
    };
    // worked from data/abilities.json at the top level
    assert_eq!(
        area("meat_hook"),
        Area::Line {
            len: 1200,
            width: 100,
            end: 100
        }
    );
    assert_eq!(
        area("dragon_slave"),
        Area::Line {
            len: 1075,
            width: 275,
            end: 150
        }
    );
    assert_eq!(
        area("fissure"),
        Area::Line {
            len: 1200,
            width: 225,
            end: 225
        }
    );
    assert_eq!(area("light_strike_array"), Area::Circle(225));
    assert_eq!(area("crystal_nova"), Area::Circle(400));
    assert_eq!(area("storm_bolt"), Area::Unit);
    assert_eq!(area("laguna_blade"), Area::Unit);
}

#[test]
fn pressing_an_aimed_ability_shows_its_outline_until_the_order_is_given() {
    let mut g = match_as("cinder");
    g.pointer_move(900, 300, true);
    let plain = instances(&g);
    g.key("KeyQ", true);
    let o = g.overlay();
    let aim = o.aim.expect("the key aims the ability");
    assert_eq!(
        aim.area,
        Area::Line {
            len: 1075,
            width: 275,
            end: 150
        },
        "the level-one line"
    );
    assert_eq!(
        aim.at,
        g.ground_point(900, 300),
        "the outline follows the pointer"
    );
    assert!(instances(&g) > plain + 10, "the outline is drawn");
    g.click(2, 900, 300);
    assert!(g.overlay().aim.is_none(), "a right-click puts it away");
}

#[test]
fn aiming_and_order_marks_change_nothing_in_the_world() {
    // the outline and the marker are the local player's alone, so online
    // play cannot see them: two matches, one with the hand busy, agree
    let mut plain = match_as("rime");
    let mut busy = match_as("rime");
    let (mut a, mut b) = (0, 0);
    for i in 0..30 {
        busy.pointer_move(400 + i * 10, 300, true);
        busy.key("KeyQ", true);
        busy.key("KeyQ", false);
        let _ = busy.overlay();
        let _ = busy.scene();
        run(&mut plain, 1, 33_334, &mut a);
        run(&mut busy, 1, 33_334, &mut b);
    }
    let (wa, wb) = (&plain.m.as_ref().unwrap().world, &busy.m.as_ref().unwrap().world);
    assert!(wa.tick > 20, "the matches ran");
    assert!(wa == wb, "the world is the same with or without aiming");
}

#[test]
fn a_right_click_on_the_ground_is_marked_where_it_landed_for_seven_tenths_of_a_second() {
    let mut g = match_as("ironvow");
    let mut t = 0;
    run(&mut g, 2, 33_334, &mut t);
    let plain = instances(&g);
    g.pointer_move(700, 420, true);
    g.click(2, 700, 420);
    let p = g.overlay().ping.expect("the click is marked");
    assert!(!p.attack && p.unit == NO_UNIT, "a move, not an attack");
    assert_eq!(p.at, g.ground_point(700, 420));
    assert!(instances(&g) > plain + 4, "the marker is drawn");
    // data/presentation.json "ping": 70 hundredths, so still there at 0.6 s
    // and gone by 0.75 s
    run(&mut g, 6, 100_000, &mut t);
    assert!(g.overlay().ping.is_some(), "still shown at 0.6 s");
    run(&mut g, 3, 50_000, &mut t);
    assert!(g.overlay().ping.is_none(), "gone by 0.75 s");
}

#[test]
fn a_teleport_glows_at_both_ends_while_it_channels_then_lands_where_it_glowed() {
    let start = || {
        let mut g = match_as("ironvow");
        let tp = g.rules.item("tp").unwrap();
        let me = g.my_hero();
        let w = &mut g.m.as_mut().unwrap().world;
        let u = w.get_mut(me).unwrap();
        u.items[0].item = tp as u16 + 1;
        u.items[0].charges = 1;
        u.mana = 500_000;
        g
    };
    let mut plain = start();
    let mut tp = start();
    let goal = [tp.rules.map.size / 2, tp.rules.map.size / 2];
    tp.command(Cmd::Item(0, NO_UNIT, goal));
    let (mut a, mut b) = (0, 0);
    run(&mut plain, 30, 33_334, &mut a);
    run(&mut tp, 30, 33_334, &mut b);
    let me = tp.my_hero();
    let (dest, team) = {
        let m = tp.m.as_ref().unwrap();
        let ctx = tp.ctx();
        let team = m.world.get(me).unwrap().team;
        (pots_core::item::tp_destination(&m.world, &ctx, team, goal), team)
    };
    let _ = team;
    assert!(
        matches!(
            tp.m.as_ref().unwrap().world.get(me).unwrap().act,
            pots_core::world::Act::Cast { .. }
        ),
        "the teleport is channelling a second in"
    );
    assert!(
        instances(&tp) > instances(&plain) + 12,
        "the channel is drawn: {} against {}",
        instances(&tp),
        instances(&plain)
    );
    run(&mut tp, 75, 33_334, &mut b);
    let at = tp.m.as_ref().unwrap().world.get(me).unwrap().pos();
    let land = [dest[0] - 150, dest[1] - 150];
    assert!(
        pots_core::world::dist2(at, land) < 60 * 60,
        "the hero landed at {at:?}, beside the building the glow showed ({land:?})"
    );
}

#[test]
fn with_the_shop_open_a_carried_item_can_be_picked_and_sold_with_a_button() {
    // round 7: click an item you carry, then Sell. A Magic Wand (509 in
    // all) sells for 254.
    let mut g = match_as("ironvow");
    let me = g.my_hero();
    let wand = g.rules.item("magic_wand").unwrap() as u16 + 1;
    let put = |g: &mut Game, x: i32, y: i32| {
        let w = &mut g.m.as_mut().unwrap().world;
        let u = w.get_mut(me).unwrap();
        u.items[2] = ItemSlot {
            item: wand,
            charges: 0,
            ready_at: 0,
        };
        if x > 0 {
            u.x = x * SUB;
            u.y = y * SUB;
        }
    };
    put(&mut g, 0, 0);
    let find_one = |g: &Game, class: &str| {
        let v = view(g);
        let mut out = Vec::new();
        find(&v.root, class, &mut out);
        out.into_iter().cloned().collect::<Vec<Node>>()
    };
    let slot2 = |g: &Game| find_one(g, "item").into_iter().find(|n| n.v == "2").unwrap();
    assert_eq!(slot2(&g).a, "item", "with the shop closed a click uses the item");
    g.act("shop.toggle", "").unwrap();
    let n = slot2(&g);
    assert_eq!(n.a, "shop.pick_slot", "with the shop open a click picks it");
    g.act(&n.a, &n.v).unwrap();
    let sell = find_one(&g, "sell-button");
    assert_eq!(sell.len(), 1, "the shop shows the picked item's Sell button");
    assert!(!sell[0].off, "on at the fountain's shop");
    assert!(sell[0].x.contains("254"), "it names the price: {}", sell[0].x);
    let gold = |g: &Game| g.m.as_ref().unwrap().world.players[0].gold;
    let before = gold(&g);
    g.act(&sell[0].a, &sell[0].v).unwrap();
    let mut t = 0;
    run(&mut g, 4, 33_334, &mut t);
    assert_eq!(gold(&g) - before, 254);
    assert!(
        find_one(&g, "sell-button").is_empty(),
        "the slot is empty: no Sell button"
    );
    // away from every shop the button is off
    put(&mut g, 8192, 8192);
    g.act("shop.pick_slot", "2").unwrap();
    let sell = find_one(&g, "sell-button");
    assert!(sell[0].off, "off away from a shop");
}
