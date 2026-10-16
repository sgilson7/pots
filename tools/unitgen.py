#!/usr/bin/env python3
"""Writes data/units.json: heroes, creeps, buildings, neutrals, the boss.

Run: python3 tools/unitgen.py

Numbers follow notes/LITERATURE.md (DotA Allstars 6.6x). The Sunward and
Moonward copies of each creep and building share their numbers; only the
model differs. Every hero keeps its DotA hero's numbers under a new name.
Units: hundredths for regeneration, armor, gains, base attack time and
attack point; map units for ranges, speeds and radii; percent for scale.
"""
import json, os

U = []

def unit(**kw):
    U.append(kw)

def hero(id, after, primary, base, gain, dmg, armor, speed, rng, bat, point, missile, abilities, turn=500, model=None):
    unit(id=id, kind="hero", model=model or f"hero_{id}", hp=0, hp_regen=25, mana=0, mana_regen=1,
         armor=armor, armor_type="hero", attack_type="hero", damage=dmg, range=rng, bat=bat,
         attack_point=point, missile=missile, speed=speed, turn=turn, radius=24, acquire=800,
         day_vision=1800, night_vision=800, bounty=[0, 0], xp=0,
         attrs={"primary": primary, "base": base, "gain": gain}, abilities=abilities, after=after)

# base damage below excludes the primary attribute, which the core adds
hero("ironvow", "Sven", "str", [22, 21, 16], [270, 200, 130], [19, 21], 100, 290, 128, 180, 40, 0,
     ["storm_bolt", "great_cleave", "warcry", "gods_strength"])
hero("rime", "Crystal Maiden", "int", [16, 16, 21], [170, 160, 290], [17, 23], 50, 280, 600, 170, 45, 900,
     ["crystal_nova", "frostbite", "brilliance_aura", "freezing_field"])
hero("cinder", "Lina", "int", [18, 15, 22], [150, 150, 320], [16, 30], 60, 295, 650, 170, 75, 1000,
     ["dragon_slave", "light_strike_array", "fiery_soul", "laguna_blade"])
hero("thistle", "Drow Ranger", "agi", [17, 20, 15], [190, 190, 140], [7, 18], -100, 300, 625, 170, 70, 1250,
     ["frost_arrows", "silence", "trueshot_aura", "marksmanship"])
hero("gristle", "Pudge", "str", [25, 14, 14], [320, 150, 150], [27, 33], -100, 285, 128, 170, 50, 0,
     ["meat_hook", "rot", "flesh_heap", "dismember"])
hero("tectus", "Earthshaker", "str", [22, 12, 16], [250, 140, 180], [25, 35], 100, 300, 128, 170, 47, 0,
     ["fissure", "enchant_totem", "aftershock", "echo_slam"])
hero("kestrel", "Juggernaut", "agi", [20, 20, 14], [220, 275, 140], [18, 22], 100, 300, 128, 160, 33, 0,
     ["blade_fury", "healing_ward", "blade_dance", "omnislash"])
hero("hexwright", "Lion", "int", [15, 15, 18], [170, 150, 300], [11, 17], 100, 290, 600, 170, 43, 1000,
     ["earth_spike", "hex", "mana_drain", "finger_of_death"])
hero("longwick", "Sniper", "agi", [16, 21, 15], [170, 290, 260], [15, 21], 0, 290, 550, 170, 17, 3000,
     ["shrapnel", "headshot", "take_aim", "assassinate"])
hero("grudge", "Axe", "str", [25, 20, 14], [280, 220, 160], [24, 28], 100, 290, 128, 170, 50, 0,
     ["berserkers_call", "battle_hunger", "counter_helix", "culling_blade"])

for side in ("sun", "moon"):
    unit(id=f"creep_melee_{side}", kind="creep", model=f"creep_melee_{side}", hp=550, hp_regen=50, armor=200,
         armor_type="normal", attack_type="normal", damage=[19, 23], range=100, bat=100, attack_point=47, speed=325,
         radius=24, acquire=500, day_vision=750, night_vision=750, bounty=[38, 48], xp=62,
         upgrade_hp=10, upgrade_damage=1, after="melee lane creep")
    unit(id=f"creep_ranged_{side}", kind="creep", model=f"creep_ranged_{side}", hp=300, hp_regen=50, mana=500,
         mana_regen=75, armor=0, armor_type="normal", attack_type="pierce", damage=[21, 26], range=500, bat=100,
         attack_point=50, missile=900, speed=325, radius=24, acquire=500, day_vision=750, night_vision=750,
         bounty=[43, 53], xp=41, upgrade_hp=10, upgrade_damage=2, after="ranged lane creep")
    unit(id=f"creep_siege_{side}", kind="creep", model=f"creep_siege_{side}", hp=550, hp_regen=0, armor=0,
         armor_type="fortified", attack_type="siege", damage=[35, 46], range=690, bat=270, attack_point=70,
         missile=1100, speed=325, radius=40, acquire=800, day_vision=750, night_vision=750, bounty=[66, 80], xp=88,
         upgrade_hp=0, upgrade_damage=0, after="siege creep (catapult)")
    for tier, hp, dmg in ((1, 1300, [100, 120]), (2, 1600, [120, 140]), (3, 1600, [120, 140]), (4, 1600, [120, 140])):
        unit(id=f"tower{tier}_{side}", kind="tower", model=f"tower_{side}", hp=hp, armor=2000, armor_type="fortified",
             attack_type="siege", damage=dmg, range=700, bat=100, attack_point=0, missile=750, speed=0, radius=0,
             footprint=100, acquire=700, day_vision=1900, night_vision=900, bounty=[264, 264], xp=0,
             after=f"tier {tier} tower")
    unit(id=f"barracks_melee_{side}", kind="barracks", model=f"barracks_melee_{side}", hp=1500, armor=1500,
         armor_type="fortified", speed=0, radius=0, footprint=160, day_vision=900, night_vision=600, bounty=[150, 200],
         after="melee barracks")
    unit(id=f"barracks_ranged_{side}", kind="barracks", model=f"barracks_ranged_{side}", hp=1200, armor=1000,
         armor_type="fortified", speed=0, radius=0, footprint=150, day_vision=900, night_vision=600, bounty=[150, 200],
         after="ranged barracks")
    unit(id=f"sphere_{side}", kind="sphere", model=f"sphere_{side}", hp=4250, hp_regen=300, armor=1300,
         armor_type="fortified", speed=0, radius=0, footprint=250, day_vision=1200, night_vision=900,
         after="the Ancient (World Tree / Frozen Throne)")
    unit(id=f"fountain_{side}", kind="fountain", model=f"fountain_{side}", hp=99999, hp_regen=99900, armor=10000,
         armor_type="fortified", attack_type="chaos", damage=[180, 200], range=1200, bat=15, attack_point=0,
         missile=2000, speed=0, radius=0, footprint=150, day_vision=1800, night_vision=1800,
         after="the fountain")

def neutral(id, hp, armor, dmg, rng, bounty, xp, speed=270, missile=0, bat=135, scale=100, attack="normal", armor_type="normal", **kw):
    unit(id=id, kind="neutral", model=id, hp=hp, hp_regen=50, armor=armor, armor_type=armor_type, attack_type=attack,
         damage=dmg, range=rng, bat=bat, attack_point=50, missile=missile, speed=speed, radius=24, acquire=400,
         day_vision=800, night_vision=800, bounty=bounty, xp=xp, scale=scale, **kw)

neutral("imp", 240, 100, [10, 12], 100, [11, 16], 20)
neutral("hound", 400, 100, [18, 22], 100, [18, 24], 36, speed=350)
neutral("mossback", 600, 400, [22, 26], 100, [24, 30], 48, speed=250)
neutral("golem_small", 400, 300, [16, 20], 100, [16, 22], 36)
neutral("golem", 800, 400, [30, 36], 100, [36, 46], 78, speed=250)
neutral("hornet", 300, 100, [16, 20], 100, [15, 20], 30, speed=350)
neutral("shaman", 700, 100, [30, 36], 500, [30, 38], 62, missile=900)
neutral("drake_young", 1100, 400, [36, 42], 300, [42, 50], 88, missile=900, attack="chaos", armor_type="heavy")
neutral("drake", 2000, 600, [64, 72], 300, [80, 96], 130, missile=900, attack="chaos", armor_type="heavy")
unit(id="warden", kind="boss", model="warden", hp=7500, hp_regen=2000, armor=900, armor_type="heavy",
     attack_type="hero", damage=[75, 75], range=128, bat=100, attack_point=60, speed=270, radius=60, acquire=250,
     day_vision=1400, night_vision=1400, bounty=[200, 200], xp=400, magic_resist=75, abilities=["warden_bash"],
     after="Roshan")
unit(id="healing_ward", kind="ward", model="healing_ward", hp=1, armor=0, armor_type="normal", speed=420, radius=16,
     day_vision=500, night_vision=500, bounty=[0, 0], after="Healing Ward")
unit(id="observer_ward", kind="ward", model="observer_ward", hp=100, armor=0, armor_type="normal", speed=0, radius=16,
     day_vision=1600, night_vision=1600, after="Observer Ward")
unit(id="sentry_ward", kind="ward", model="sentry_ward", hp=100, armor=0, armor_type="normal", speed=0, radius=16,
     day_vision=200, night_vision=200, after="Sentry Ward")
unit(id="fissure_rock", kind="building", model="fissure_rock", hp=1, armor=0, armor_type="fortified", speed=0,
     radius=60, day_vision=0, night_vision=0, after="Fissure's wall")
unit(id="courier", kind="summon", model="courier", hp=75, armor=0, armor_type="normal", speed=350, radius=16,
     day_vision=500, night_vision=500, bounty=[0, 0], after="Animal Courier")
unit(id="shop", kind="shop", model="shop", hp=1, armor=0, armor_type="fortified", speed=0, radius=0, footprint=90,
     day_vision=0, night_vision=0, after="shops")

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
with open(os.path.join(root, "data", "units.json"), "w") as f:
    json.dump({"units": U}, f, indent=1)
print("wrote data/units.json with", len(U), "units")
