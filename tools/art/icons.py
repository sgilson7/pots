"""Shared setup for the game's icons (art_abilities.py, art_items_*.py,
art_heroes.py): which rim colour each icon gets, and a badge already drawn.

The rim says what kind of thing the icon is:

- an ability: its hero's element (ELEMENT below, hero -> rim role; the
  hero that owns an ability is read from data/units.json);
- an item: its shop tab, read from data/items.json (TAB_RIM below);
- a hero portrait: the hero's element.

Every name here is a role in the style pack (styles/pots/palette.json).
"""
import json
from pathlib import Path

import style
from artlib import Fig

ROOT = Path(__file__).resolve().parents[2]
M = style.load().motifs()

# hero -> element rim role
ELEMENT = {
    "ironvow": "steelrim",
    "rime": "frost",
    "cinder": "fire",
    "thistle": "frost",
    "gristle": "rot",
    "tectus": "earth",
    "kestrel": "blood",
    "hexwright": "arcane",
    "longwick": "steelrim",
    "tempest": "storm",
    "hackle": "blood",
    "chipper": "earth",
    "warden": "earth",
}

# shop tab -> rim role ("none" is the aegis, which no shop sells)
TAB_RIM = {
    "consumables": "consumables", "basics": "basics", "attributes": "attributes",
    "armaments": "armaments", "arcane": "arcanetab", "secret": "secret",
    "common": "common", "support": "support", "protect": "protect",
    "weapons": "weapons", "caster": "caster", "none": "relictab",
}


def _load(name, key):
    return json.loads((ROOT / "data" / name).read_text())[key]


def owners():
    """ability id -> unit id that has it."""
    out = {}
    units = _load("units.json", "units")
    rows = units.items() if isinstance(units, dict) else ((r["id"], r) for r in units)
    for unit, row in rows:
        for a in row.get("abilities", []):
            out.setdefault(a, unit)
    # an ability that takes another's slot (round 7) belongs to that hero
    for row in _load("abilities.json", "abilities"):
        if row.get("alt") and row["id"] in out:
            out.setdefault(row["alt"], out[row["id"]])
    return out


def tabs():
    """item id -> shop tab."""
    return {row["id"]: row.get("tab", "none") for row in _load("items.json", "items")}


def ability_rim(aid):
    return ELEMENT[owners()[aid]]


def item_rim(iid):
    return TAB_RIM[tabs()[iid]]


def ability_fig(aid):
    """A round icon with its hero's element badge drawn."""
    fig = Fig("icon")
    M.badge(fig, ability_rim(aid), "round")
    return fig


def item_fig(iid):
    """A rounded-square icon with its shop tab's badge drawn."""
    fig = Fig("icon")
    M.badge(fig, item_rim(iid), "square")
    return fig


def hero_fig(hid):
    fig = Fig("icon")
    M.badge(fig, ELEMENT[hid], "round")
    return fig


def ability_ids():
    return [row["id"] for row in _load("abilities.json", "abilities")]


def item_ids():
    return [row["id"] for row in _load("items.json", "items")]
