#!/usr/bin/env python3
"""Writes data/items.json: the shop, after DotA Allstars 6.6x prices and
recipes (notes/LITERATURE.md). Each item has an original name in
data/strings.json; the id here is a working name.
Run: python3 tools/itemgen.py
"""
import json, os
I = []
def it(id, cost, after, tab, parts=(), recipe=False, shop="home", logic="", target=None, cooldown=0, mana=0, range=0,
       charges=0, stacks=False, values=None, **bonus):
    d = {"id": id, "cost": cost, "parts": list(parts), "recipe": recipe, "shop": shop, "tab": tab, "bonus": bonus,
         "logic": logic, "values": values or {}, "cooldown": cooldown, "mana": mana, "range": range,
         "charges": charges, "stacks": stacks, "icon": f"items/{id}", "after": after}
    if target:
        d["target"] = target
    I.append(d)

# consumables
it("tango", 90, "Tango", "consumables", logic="tango", target="point", range=165, charges=4, stacks=True, values={"heal": 115, "duration": 1600})
it("salve", 100, "Healing Salve", "consumables", logic="salve", target="unit", range=250, charges=1, stacks=True, values={"heal": 400, "duration": 1000})
it("clarity", 50, "Clarity Potion", "consumables", logic="clarity", target="unit", range=250, charges=1, stacks=True, values={"mana": 100, "duration": 3000})
it("tp", 135, "Scroll of Town Portal", "consumables", logic="tp", target="point", cooldown=6500, mana=75, range=99999, charges=1, stacks=True)
it("observer", 200, "Observer Wards", "consumables", logic="ward_observer", target="point", range=500, charges=2, values={"duration": 36000})
it("sentry", 200, "Sentry Wards", "consumables", logic="ward_sentry", target="point", range=500, charges=2, values={"duration": 18000, "true_sight": 850})
it("bottle", 600, "Bottle", "consumables", logic="bottle", target="none", cooldown=50, charges=3, values={"heal": 135, "mana": 70, "duration": 300})
# attributes
it("branch", 53, "Ironwood Branch", "basics", str=1, agi=1, int=1)
it("gauntlets", 150, "Gauntlets of Strength", "basics", str=3)
it("slippers", 150, "Slippers of Agility", "basics", agi=3)
it("mantle", 150, "Mantle of Intelligence", "basics", int=3)
it("circlet", 185, "Circlet of Nobility", "basics", str=2, agi=2, int=2)
it("belt", 450, "Belt of Giant Strength", "attributes", str=6)
it("elvenskin", 450, "Band of Elvenskin", "attributes", agi=6)
it("robe", 450, "Robe of the Magi", "attributes", int=6)
it("ogre_club", 1000, "Ogre Axe", "attributes", str=10)
it("alacrity", 1000, "Blade of Alacrity", "attributes", agi=10)
it("wizardry", 1000, "Staff of Wizardry", "attributes", int=10)
it("ultimate_orb", 2100, "Ultimate Orb", "attributes", str=10, agi=10, int=10)
# armaments
it("ring_protection", 175, "Ring of Protection", "basics", armor=200)
it("blades", 450, "Blades of Attack", "armaments", damage=9)
it("quarterstaff", 900, "Quarterstaff", "armaments", damage=10, attack_speed=10)
it("chainmail", 550, "Chainmail", "armaments", armor=500)
it("broadsword", 1200, "Broadsword", "armaments", damage=18)
it("claymore", 1400, "Claymore", "armaments", damage=21)
it("platemail", 1400, "Platemail", "armaments", armor=1000)
it("javelin", 1500, "Javelin", "armaments", damage=21)
it("mithril", 1610, "Mithril Hammer", "armaments", damage=24)
it("gloves", 500, "Gloves of Haste", "basics", attack_speed=15)
# arcane
it("boots", 500, "Boots of Speed", "basics", move_speed=50)
it("ring_regen", 350, "Ring of Regeneration", "basics", hp_regen=200)
it("sobi", 325, "Sage's Mask", "basics", mana_regen_pct=50)
it("void_stone", 900, "Void Stone", "arcane", mana_regen_pct=100)
it("ring_health", 875, "Ring of Health", "arcane", hp_regen=500)
it("vitality", 1100, "Vitality Booster", "arcane", hp=250)
it("energy", 1000, "Energy Booster", "arcane", mana=250)
it("point_booster", 1200, "Point Booster", "arcane", hp=200, mana=150)
it("magic_stick", 200, "Magic Stick", "basics", logic="wand", target="none", cooldown=1300, charges=0, values={"per_charge": 15, "max": 10})
it("blink", 2150, "Blink Dagger", "arcane", logic="blink", target="point", cooldown=1400, mana=75, range=1200)
# secret shop
it("hyperstone", 2100, "Hyperstone", "secret", shop="secret", attack_speed=55)
it("demon_edge", 2400, "Demon Edge", "secret", shop="secret", damage=46)
it("eaglesong", 3300, "Eaglesong", "secret", shop="secret", agi=25)
it("reaver", 3200, "Reaver", "secret", shop="secret", str=25)
it("mystic", 2700, "Mystic Staff", "secret", shop="secret", int=25)
it("relic", 3800, "Sacred Relic", "secret", shop="secret", damage=60)
it("talisman", 1800, "Talisman of Evasion", "secret", shop="secret", evasion=25)
it("mask_death", 900, "Mask of Death", "armaments", lifesteal=15)
# recipes
it("magic_wand", 150, "Magic Wand", "common", parts=["magic_stick", "branch", "branch"], recipe=True, logic="wand", target="none",
   cooldown=1300, str=3, agi=3, int=3, values={"per_charge": 15, "max": 15})
it("bracer", 175, "Bracer", "common", parts=["circlet", "gauntlets"], recipe=True, str=6, agi=3, int=3)
it("wraith_band", 150, "Wraith Band", "common", parts=["circlet", "slippers"], recipe=True, str=3, agi=6, int=3)
it("null_talisman", 135, "Null Talisman", "common", parts=["circlet", "mantle"], recipe=True, str=3, agi=3, int=6)
it("power_treads", 0, "Power Treads", "common", parts=["boots", "gloves", "belt"], move_speed=60, attack_speed=30, str=8)
it("phase_boots", 0, "Phase Boots", "common", parts=["boots", "blades", "blades"], move_speed=60, damage=24)
it("travel_boots", 2200, "Boots of Travel", "common", parts=["boots"], recipe=True, move_speed=100, logic="tp", target="point", cooldown=6000, mana=75, range=99999)
it("perseverance", 0, "Perseverance", "common", parts=["ring_health", "void_stone"], hp_regen=500, mana_regen_pct=125, damage=10)
it("midas", 1400, "Hand of Midas", "common", parts=["gloves"], recipe=True, attack_speed=30, logic="midas", target="unit",
   cooldown=10000, range=600, values={"gold": 190, "xp_pct": 250})
it("buckler", 200, "Buckler", "support", parts=["chainmail", "branch"], recipe=True, armor=500, str=1, agi=1, int=1)
it("headdress", 200, "Headdress", "support", parts=["ring_regen", "branch"], recipe=True, hp_regen=300, str=1, agi=1, int=1)
it("mekansm", 900, "Mekansm", "support", parts=["buckler", "headdress"], recipe=True, armor=500, str=5, agi=5, int=5,
   logic="mekansm", target="none", cooldown=4500, mana=150, values={"heal": 250, "armor": 2, "radius": 750, "duration": 2500})
it("vanguard", 0, "Vanguard", "protect", parts=["ring_health", "vitality"], hp=250, hp_regen=600)
it("bkb", 1300, "Black King Bar", "protect", parts=["ogre_club", "mithril"], recipe=True, str=10, damage=24, logic="bkb",
   target="none", cooldown=8000, values={"duration": 1000})
it("heart", 1200, "Heart of Tarrasque", "protect", parts=["reaver", "vitality"], recipe=True, str=40, hp=300, hp_regen=1500)
it("shivas", 600, "Shiva's Guard", "protect", parts=["platemail", "mystic"], recipe=True, int=30, armor=1500, logic="shivas",
   target="none", cooldown=3000, mana=100, values={"radius": 720, "damage": 200, "slow": 40, "duration": 400})
it("assault", 1300, "Assault Cuirass", "protect", parts=["platemail", "hyperstone", "chainmail"], recipe=True, armor=1000,
   attack_speed=35, logic="assault", values={"radius": 900, "armor": 5, "attack_speed": 15})
it("linkens", 1325, "Linken's Sphere", "protect", parts=["ultimate_orb", "perseverance"], recipe=True, str=15, agi=15, int=15,
   hp_regen=600, mana_regen_pct=150, damage=10)
it("battlefury", 0, "Battle Fury", "weapons", parts=["claymore", "broadsword", "perseverance"], damage=65, hp_regen=600,
   mana_regen_pct=150, logic="cleave", values={"pct": 35, "radius": 225})
it("daedalus", 1250, "Buriza-do Kyanon", "weapons", parts=["demon_edge", "claymore"], recipe=True, damage=81, logic="crit",
   values={"chance": 25, "multiplier": 240})
it("mkb", 0, "Monkey King Bar", "weapons", parts=["demon_edge", "javelin", "javelin"], damage=88, attack_speed=15, logic="bash",
   values={"chance": 35, "ranged_chance": 35, "damage": 100, "stun": 1})
it("basher", 1000, "Cranium Basher", "weapons", parts=["javelin", "belt"], recipe=True, damage=40, str=6, logic="bash",
   values={"chance": 25, "ranged_chance": 10, "damage": 0, "stun": 140})
it("butterfly", 0, "Butterfly", "weapons", parts=["eaglesong", "talisman", "quarterstaff"], agi=30, damage=30, attack_speed=30, evasion=35)
it("desolator", 900, "Stygian Desolator", "weapons", parts=["mithril", "mithril"], recipe=True, damage=60, logic="corrupt",
   values={"armor": 7, "duration": 700})
it("maelstrom", 600, "Maelstrom", "weapons", parts=["mithril", "gloves"], recipe=True, damage=24, logic="lightning",
   values={"chance": 25, "damage": 120, "bounces": 4})
it("mjollnir", 900, "Mjollnir", "weapons", parts=["maelstrom", "hyperstone"], recipe=True, damage=24, attack_speed=80, logic="lightning",
   values={"chance": 25, "damage": 160, "bounces": 8})
it("radiance", 1350, "Radiance", "weapons", parts=["relic"], recipe=True, damage=60, logic="radiance", values={"radius": 650, "dps": 50})
it("rapier", 0, "Divine Rapier", "weapons", parts=["relic", "demon_edge"], damage=300)
it("satanic", 1100, "Satanic", "weapons", parts=["reaver", "mask_death", "claymore"], recipe=True, str=25, damage=21, armor=500,
   lifesteal=25, logic="satanic", target="none", cooldown=3500, values={"lifesteal": 75, "duration": 350})
it("sange", 1000, "Sange", "weapons", parts=["belt", "ogre_club"], recipe=True, str=16, damage=10)
it("yasha", 1000, "Yasha", "weapons", parts=["elvenskin", "alacrity"], recipe=True, agi=16, attack_speed=15, move_pct=10)
it("sange_yasha", 0, "Sange and Yasha", "weapons", parts=["sange", "yasha"], str=16, agi=16, damage=12, attack_speed=15, move_pct=12)
it("manta", 800, "Manta Style", "weapons", parts=["yasha", "ultimate_orb"], recipe=True, str=10, agi=26, int=10, attack_speed=15,
   move_pct=10, logic="manta", target="none", cooldown=5000, mana=165, values={"duration": 2000, "dealt": 33, "taken": 350})
it("dagon", 1300, "Dagon", "caster", parts=["wizardry", "null_talisman"], recipe=True, int=13, str=3, agi=3, damage=9, logic="dagon",
   target="unit", cooldown=3500, mana=180, range=600, values={"damage": 400})
it("euls", 0, "Eul's Scepter of Divinity", "caster", parts=["wizardry", "void_stone", "sobi"], int=10, mana_regen_pct=150, move_speed=30,
   logic="euls", target="unit", cooldown=3000, mana=75, range=700, values={"duration": 250})
it("force_staff", 1000, "Force Staff", "caster", parts=["wizardry", "ring_regen"], recipe=True, int=10, hp_regen=300, logic="force",
   target="unit", cooldown=2000, mana=25, range=800, values={"distance": 600})
it("orchid", 775, "Orchid Malevolence", "caster", parts=["quarterstaff", "robe", "sobi", "quarterstaff", "robe", "sobi"], recipe=True,
   int=25, attack_speed=30, damage=30, mana_regen_pct=225, logic="orchid", target="unit", cooldown=1800, mana=100, range=900,
   values={"duration": 500})
it("sheep", 0, "Guinsoo's Scythe of Vyse", "caster", parts=["mystic", "ultimate_orb", "void_stone"], int=35, str=10, agi=10,
   mana_regen_pct=150, logic="sheep", target="unit", cooldown=3500, mana=100, range=800, values={"duration": 350})
it("aghanims", 0, "Aghanim's Scepter", "caster", parts=["point_booster", "ogre_club", "alacrity", "wizardry"], str=10, agi=10, int=10,
   hp=200, mana=150)
# dropped by the boss, never sold
it("aegis", 0, "Aegis of the Immortal", "none", shop="none", values={"delay": 500})

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ids = {i["id"] for i in I}
for i in I:
    for p in i["parts"]:
        assert p in ids, (i["id"], p)
with open(os.path.join(root, "data", "items.json"), "w") as f:
    json.dump({"items": I}, f, indent=1)
print("wrote data/items.json with", len(I), "items")
