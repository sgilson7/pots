"""The game's own browser interactions, run by check.py in every engine.

check.py has already loaded the page, waited for it to report ready, and let
it idle. This plays the start of an arcade match the way a person would and
asserts what the page shows.

Expected values are worked out from the brief and the literature review
(C4), never copied from the game's own output: a hero starts with 603 gold
(DotA 6.72: 853 less the hero's 250), level one, and one point to spend.
Waits are on selectors and data-* attributes, never fixed times.
"""

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
STRINGS = json.loads((ROOT / "data" / "strings.json").read_text())


def probe(page):
    return json.loads(page.get_attribute("body", "data-probe") or "{}")


def typing(page, check):
    """Round 7: a text box keeps what is typed in it, key by key. Each key
    used to rebuild the box empty and take the focus away. Only the online
    form is typed in, so no relay is contacted."""
    page.click("[data-a='screen'][data-v='online']")
    name = page.locator("input[data-a='online.name']")
    name.click()
    page.keyboard.type("Sam the brave", delay=30)
    code = page.locator("input[data-a='online.code']")
    code.click()
    page.keyboard.type("ab12cd", delay=30)
    check(name.input_value() == "Sam the brave", f"the name box keeps every key typed: {name.input_value()!r}")
    check(code.input_value() == "ab12cd", f"the room code box keeps every key typed: {code.input_value()!r}")
    page.click("[data-a='screen'][data-v='title']")
    page.wait_for_selector("body[data-screen='title']")


def run(page, check, engine):
    check(page.inner_text(".game-title").lower() == STRINGS["ui"]["title"].lower(), "the title from data/strings.json is drawn")
    check(page.get_attribute("body", "data-screen") == "title", "the core's title screen is shown")
    typing(page, check)
    page.click("[data-a='screen'][data-v='arcade']")
    page.wait_for_selector("body[data-screen='arcade']")
    first = STRINGS["missions"]["first_steps"]["name"]
    check(first.lower() in page.inner_text(".missions").lower(), "the first arcade mission is listed by its name")
    page.click("[data-a='mission'][data-v='0']")
    page.wait_for_selector("body[data-screen='pick']")
    page.click("[data-a='pick'][data-v='0']")
    page.click("[data-a='start']")
    page.wait_for_selector("body[data-screen='match']")
    page.wait_for_selector(".gold")
    check(page.inner_text(".gold").strip().startswith("603"), "a hero starts with 603 gold")
    page.wait_for_function("() => JSON.parse(document.body.dataset.probe || '{}').tick > 30")
    start = probe(page)
    check(start["alive"] and start["level"] == 1, "the hero stands on the map at level one")
    check(page.locator("button.learn").count() >= 4, "a learn button waits under each ability at level one")
    # the shop: open, buy the first item it lists, see the gold drop
    page.keyboard.press("KeyB")
    page.wait_for_selector(".shop")
    check(page.locator(".shop-item:not(.dim)").count() > 0, "the base shop sells to a hero at the fountain")
    gold = int(page.inner_text(".gold").strip().split()[0])
    # a click picks the item and shows its tree; the tree's button buys it
    page.locator(".shop-item:not(.dim)").first.click()
    page.wait_for_selector(".build .buy-button:not([disabled])")
    page.locator(".build .buy-button").click()
    page.wait_for_function(
        "(g) => { const p = JSON.parse(document.body.dataset.probe || '{}'); return p.gold < g; }", arg=gold, timeout=20000
    )
    check(probe(page)["gold"] < gold, "buying an item at the base shop spends gold")
    # a built item's tree shows its parts under it
    found = False
    for i in range(page.locator(".shop .tabs button").count()):
        page.locator(".shop .tabs button").nth(i).click()
        for j in range(page.locator(".shop-item").count()):
            page.locator(".shop-item").nth(j).click()
            page.wait_for_selector(".build")
            if page.locator(".build .tree-kids .tree-item").count() > 0:
                found = True
                break
        if found:
            break
    check(found, "the build tree of a built item shows the parts it is made from")
    check(page.locator(".inventory .slot:not(.empty)").count() >= 1, "the bought item is in the inventory")
    # a right-click on an item in the shop buys it at once (T8)
    page.click(".shop .tabs button >> nth=0")
    gold = int(page.inner_text(".gold").strip().split()[0])
    held = page.locator(".inventory .slot:not(.empty)").count()
    page.locator(".shop-item:not(.dim)").first.click(button="right")
    page.wait_for_function(
        "(g) => { const p = JSON.parse(document.body.dataset.probe || '{}'); return p.gold < g; }", arg=gold, timeout=20000
    )
    check(probe(page)["gold"] < gold, "a right-click on an item in the shop buys it")
    check(page.locator(".inventory .slot:not(.empty)").count() >= held, "the right-clicked item is carried")
    # learn the first ability with its button
    page.click("button.learn[data-v='0']")
    page.wait_for_selector(".abilities .pip.on")
    check(page.locator(".abilities .pip.on").count() == 1, "learning the first ability lights one pip")
    # walk: a right-click on the ground somewhere off the hero
    box = page.locator("#view").bounding_box()
    page.keyboard.press("Escape")
    page.mouse.click(box["x"] + box["width"] * 0.75, box["y"] + box["height"] * 0.3, button="right")
    page.wait_for_function(
        "([x, y]) => { const p = JSON.parse(document.body.dataset.probe || '{}'); return p.x !== x || p.y !== y; }",
        arg=[start["x"], start["y"]],
        timeout=20000,
    )
    moved = probe(page)
    check((moved["x"], moved["y"]) != (start["x"], start["y"]), "a right-click on the ground walks the hero")
    check(page.inner_text("#notice") == "", "no error message is shown")
    aram(page, check)


def aram(page, check):
    """All Random All Mid (round 4), as a player starts it: the mode on in
    the custom match, a dealt hero, one reroll, a full 5v5 in the middle
    lane, and the cannon at the fountain throwing the hero to the lane."""
    page.reload()
    page.wait_for_selector("body[data-ready='true']", state="attached", timeout=60000)
    page.click("[data-a='screen'][data-v='arcade']")
    page.click("[data-a='setup.mode']")
    page.wait_for_selector("button.mode.on")
    page.click("[data-a='custom']")
    page.wait_for_selector(".hero-card.dealt")
    check(page.locator("[data-a='pick']").count() == 0, "All Random All Mid deals the hero: no hero card can be chosen")
    first = page.inner_text(".hero-card.dealt strong")
    page.click("[data-a='reroll']")
    page.wait_for_function("(n) => { const c = document.querySelector('.hero-card.dealt strong'); return c && c.textContent !== n; }", arg=first)
    check(page.locator("[data-a='reroll'][disabled]").count() == 1, "the reroll deals another hero, once")
    page.click("[data-a='start']")
    page.wait_for_selector("body[data-screen='match']")
    page.wait_for_function("() => JSON.parse(document.body.dataset.probe || '{}').tick > 5")
    p = probe(page)
    check(p["aram"], "the match is All Random All Mid")
    check(p["heroes"] == 10, f"ten heroes are dealt into the match: {p['heroes']}")
    check(p["side_towers"] == 0, "the side lanes have no towers")
    check(p["gold"] == 853, f"each player starts with 853 gold: {p['gold']}")
    page.wait_for_selector("[data-a='cannon']:not([disabled])")
    box = page.locator("#view").bounding_box()
    page.keyboard.press("KeyF")
    page.mouse.click(box["x"] + box["width"] * 0.7, box["y"] + box["height"] * 0.3)
    page.wait_for_function("() => JSON.parse(document.body.dataset.probe || '{}').flying", timeout=20000)
    check(probe(page)["flying"], "the cannon throws the hero")
    page.wait_for_function("() => { const p = JSON.parse(document.body.dataset.probe || '{}'); return !p.flying && p.x > 4000; }", timeout=20000)
    p = probe(page)
    check(p["x"] > 4000 and p["y"] > 3500, f"the hero lands in the middle lane, far from the fountain: {p['x']}, {p['y']}")
    # T15: the computer players come out past their outer towers before the
    # first wave, to where the cannon lands, not pressed against the tower.
    # The Sunward outer tower stands at (6450, 6050), x + y = 12500; the
    # Moonward one at its mirror, x + y = 20268. A Sunward hero is one with
    # x + y below the centre's 16384.
    out = "() => { const b = JSON.parse(document.body.dataset.probe || '{}').bots || []; return b.length == 9 && b.every(([x, y]) => x + y < 16384 ? x + y > 12500 : x + y < 20268); }"
    try:
        page.wait_for_function(out, timeout=40000)
        came = True
    except Exception:
        came = False
    check(came, f"the computer players wait out in the lane, past their outer towers: {probe(page).get('bots')}")
    check(page.inner_text("#notice") == "", "no error message is shown in All Random All Mid")
