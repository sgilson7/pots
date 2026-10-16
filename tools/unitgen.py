#!/usr/bin/env python3
"""Writes data/units.json: heroes, creeps, buildings, neutrals, the boss.

Run: python3 tools/unitgen.py

Numbers follow notes/LITERATURE.md, pinned to DotA Allstars 6.72/6.73:
heroes A (Sven, Earthshaker, Pudge, Juggernaut, Drow Ranger) use the 6.72
column, heroes B (Sniper, Crystal Maiden, Lina, Lion, Zeus) the 6.73 one;
buildings, creeps, neutrals and Roshan come from section 2 (6.72/6.73).
Every hero keeps its DotA hero's numbers under a new name. Values the
review marks UNVERIFIED are listed in notes/DATA-TRANSCRIPTION.md.
Units: hundredths for regeneration, armor, gains, base attack time, attack
point and backswing; thousandths of a radian per 0.03 s for turn rate; map
units for ranges, speeds and radii; percent for scale.
"""
import json, os

U = []

def unit(**kw):
    if "after" in kw:
        kw["_after"] = kw.pop("after")
    U.append(kw)

def hero(id, after, primary, base, gain, dmg, armor, speed, rng, bat, point, backswing, missile, abilities,
         turn=500, model=None, day_vision=1800, night_vision=800, hp_regen=25):
    """dmg: base damage without the primary attribute (the core adds it).
    armor: base armor in hundredths, without agility (the core adds agi*14/100)."""
    unit(id=id, kind="hero", model=model or f"hero_{id}", hp=0, hp_regen=hp_regen, mana=0, mana_regen=1,
         armor=armor, armor_type="hero", attack_type="hero", damage=dmg, range=rng, bat=bat,
         attack_point=point, backswing=backswing, missile=missile, speed=speed, turn=turn, radius=24, acquire=800,
         day_vision=day_vision, night_vision=night_vision, bounty=[0, 0], xp=0,
         attrs={"primary": primary, "base": base, "gain": gain}, abilities=abilities, _after=after)

# Heroes A, 6.72. Damage = level-1 damage minus the level-1 primary attribute.
# Armor = level-1 armor minus agi*0.14 (Sven 1.94-2.94, ES 2.68-1.68, Pudge 1.96-1.96,
# Juggernaut 3.8-2.8, Drow 2.08-3.08).
hero("ironvow", "Sven", "str", [23, 21, 14], [270, 200, 130], [31, 33], -100, 295, 128, 170, 40, 30, 0,
     ["storm_bolt", "great_cleave", "warcry", "gods_strength"])
hero("tectus", "Earthshaker", "str", [22, 12, 16], [250, 140, 180], [24, 34], 100, 300, 128, 170, 47, 86, 0,
     ["fissure", "enchant_totem", "aftershock", "echo_slam"], turn=400)
hero("gristle", "Pudge", "str", [25, 14, 14], [320, 150, 150], [27, 33], 0, 285, 128, 170, 50, 117, 0,
     ["meat_hook", "rot", "flesh_heap", "dismember"])
hero("kestrel", "Juggernaut", "agi", [20, 20, 14], [190, 285, 140], [24, 28], 100, 305, 128, 170, 33, 84, 0,
     ["blade_fury", "healing_ward", "blade_dance", "omnislash"], hp_regen=75)
hero("thistle", "Drow Ranger", "agi", [17, 22, 15], [190, 190, 140], [18, 29], -100, 300, 625, 170, 70, 30, 1250,
     ["frost_arrows", "silence", "trueshot_aura", "marksmanship"], night_vision=1700)
# Heroes B, 6.73. Base armor is -1 for all but Zeus, whose base is 0 (review).
hero("longwick", "Sniper", "agi", [16, 21, 15], [170, 290, 260], [15, 21], -100, 290, 550, 170, 17, 70, 3000,
     ["shrapnel", "headshot", "take_aim", "assassinate"], night_vision=1000)
hero("rime", "Crystal Maiden", "int", [16, 16, 21], [170, 160, 290], [17, 23], -100, 280, 600, 170, 55, 0, 900,
     ["crystal_nova", "frostbite", "brilliance_aura", "freezing_field"])
hero("cinder", "Lina", "int", [18, 16, 27], [150, 150, 320], [13, 31], -100, 295, 625, 170, 75, 78, 900,
     ["dragon_slave", "light_strike_array", "fiery_soul", "laguna_blade"])
hero("hexwright", "Lion", "int", [16, 15, 22], [170, 150, 300], [20, 26], -100, 290, 600, 170, 43, 74, 900,
     ["earth_spike", "hex", "mana_drain", "finger_of_death"])
hero("tempest", "Zeus", "int", [19, 11, 20], [180, 120, 270], [21, 29], 0, 295, 350, 170, 63, 37, 1100,
     ["arc_lightning", "lightning_bolt", "static_field", "thundergods_wrath"])

# Lane creeps (6.72/6.73, playdota unit stats). Sunward = Treant / Druid of the
# Talon / Glaive Thrower, Moonward = Ghoul / Necromancer / Meat Wagon: same
# numbers, different swing timings. Armor type: playdota prints "Unarmored";
# the Dota 2 port used "Medium" (the review flags this as UNVERIFIED).
SWING = {"sun": {"melee": (47, 53), "ranged": (50, 67), "siege": (10, 190)},
         "moon": {"melee": (39, 44), "ranged": (53, 47), "siege": (70, 130)}}
for side in ("sun", "moon"):
    sw = SWING[side]
    for mega in (False, True):
        sfx = "_mega" if mega else ""
        scale = 115 if mega else 100
        tag = "mega " if mega else ""
        unit(id=f"creep_melee{sfx}_{side}", kind="creep", model=f"creep_melee_{side}", scale=scale,
             hp=700 if mega else 550, hp_regen=50, armor=300 if mega else 200, armor_type="unarmored",
             attack_type="normal", damage=[36, 44] if mega else [19, 23], range=100, bat=100,
             attack_point=sw["melee"][0], backswing=sw["melee"][1], speed=325, radius=24, acquire=500,
             day_vision=850, night_vision=800, bounty=[18, 26] if mega else [38, 48], xp=25 if mega else 62,
             upgrade_hp=19 if mega else 10, upgrade_damage=2 if mega else 1,
             after=f"{tag}melee lane creep (playdota: Unarmored; Dota 2 port: Medium)")
        unit(id=f"creep_ranged{sfx}_{side}", kind="creep", model=f"creep_ranged_{side}", scale=scale,
             hp=475 if mega else 300, hp_regen=200, mana=300, mana_regen=75, armor=100 if mega else 0,
             armor_type="unarmored", attack_type="pierce", damage=[41, 46] if mega else [21, 26], range=500,
             bat=100, attack_point=sw["ranged"][0], backswing=sw["ranged"][1], missile=900, speed=325, radius=24,
             acquire=600, day_vision=850, night_vision=800, bounty=[18, 26] if mega else [43, 53],
             xp=25 if mega else 41, upgrade_hp=18 if mega else 10, upgrade_damage=3 if mega else 2,
             after=f"{tag}ranged lane creep")
        unit(id=f"creep_siege{sfx}_{side}", kind="creep", model=f"creep_siege_{side}", scale=scale, hp=500,
             hp_regen=0, armor=0, armor_type="fortified", attack_type="siege",
             damage=[51, 62] if mega else [35, 46], range=690, bat=270, attack_point=sw["siege"][0],
             backswing=sw["siege"][1], missile=1100, speed=325, radius=40, acquire=800, day_vision=850,
             night_vision=850, bounty=[66, 80], xp=88, upgrade_hp=0, upgrade_damage=0,
             after=f"{tag}siege creep (catapult)")
    # towers: hp, armor, damage, BAT, last-hit bounty
    for tier, hp, armor, dmg, bat, bounty in ((1, 1300, 1800, [100, 120], 100, [452, 470]),
                                              (2, 1600, 1800, [120, 140], 95, [502, 520]),
                                              (3, 1600, 2500, [122, 182], 95, [552, 570]),
                                              (4, 1600, 3000, [122, 182], 95, [602, 620])):
        unit(id=f"tower{tier}_{side}", kind="tower", model=f"tower_{side}", hp=hp, armor=armor, armor_type="fortified",
             attack_type="siege", damage=dmg, range=700, bat=bat, attack_point=40, backswing=60, missile=750,
             speed=0, radius=0, footprint=100, acquire=700, day_vision=1900, night_vision=800, bounty=bounty, xp=0,
             after=f"tier {tier} tower")
    unit(id=f"barracks_melee_{side}", kind="barracks", model=f"barracks_melee_{side}", hp=1500, hp_regen=250,
         armor=1500, armor_type="fortified", speed=0, radius=0, footprint=160, day_vision=900, night_vision=600,
         bounty=[352, 370], after="melee barracks")
    unit(id=f"barracks_ranged_{side}", kind="barracks", model=f"barracks_ranged_{side}", hp=1500, hp_regen=250,
         armor=500, armor_type="fortified", speed=0, radius=0, footprint=150, day_vision=900, night_vision=600,
         bounty=[352, 370], after="ranged barracks")
    unit(id=f"sphere_{side}", kind="sphere", model=f"sphere_{side}", hp=4250, hp_regen=300, armor=1500,
         armor_type="fortified", speed=0, radius=0, footprint=250, day_vision=1800, night_vision=1800,
         after="the Ancient (World Tree / Frozen Throne)")
    # fountain regen is not in the review; DotA's fountain cannot be killed in practice
    unit(id=f"fountain_{side}", kind="fountain", model=f"fountain_{side}", hp=50000, hp_regen=99900, armor=0,
         armor_type="fortified", attack_type="chaos", damage=[190, 199], range=800, bat=15, attack_point=0,
         missile=1450, speed=0, radius=0, footprint=150, day_vision=1300, night_vision=1300,
         after="the fountain")

def neutral(id, hp, armor, dmg, rng, bounty, xp, speed=270, missile=0, bat=135, scale=100, attack="normal",
            armor_type="heavy", model=None, **kw):
    unit(id=id, kind="neutral", model=model or id, hp=hp, hp_regen=50, armor=armor, armor_type=armor_type,
         attack_type=attack, damage=dmg, range=rng, bat=bat, attack_point=50, missile=missile, speed=speed,
         radius=24, acquire=400, day_vision=800, night_vision=800, bounty=bounty, xp=xp, scale=scale, **kw)

# xp by unit level: L1 25, L2 41, L3 62, L4 88, L5 119, L6 155
neutral("imp", 240, 0, [10, 11], 100, [7, 9], 25, after="Kobold")
neutral("imp_chief", 400, 0, [24, 27], 100, [23, 29], 41, speed=330, model="imp", scale=130, after="Kobold Taskmaster")
neutral("hound", 500, 100, [21, 24], 100, [22, 26], 62, speed=350, bat=145, after="Giant Wolf")
neutral("hound_alpha", 600, 300, [30, 33], 100, [37, 45], 88, speed=350, model="hound", scale=125, after="Alpha Wolf")
neutral("mossback", 800, 400, [29, 33], 100, [54, 62], 119, after="Mud Golem")
neutral("golem_small", 550, 200, [18, 21], 100, [20, 24], 41, speed=350, after="Centaur Outrunner")
neutral("golem", 1100, 400, [49, 55], 100, [66, 78], 119, speed=320, bat=165, attack="chaos", after="Centaur Khan")
neutral("hornet", 500, 0, [24, 27], 500, [26, 33], 62, missile=900, attack="pierce", after="Dark Troll")
neutral("shaman", 1100, 100, [49, 54], 500, [54, 62], 155, speed=320, missile=900, attack="pierce",
        after="Dark Troll Warlord")
neutral("drake_young", 950, 200, [34, 45], 300, [44, 56], 62, speed=350, bat=180, missile=900, attack="pierce",
        after="Black Drake")
neutral("drake", 2000, 600, [48, 81], 300, [164, 234], 155, speed=300, bat=150, missile=900, attack="chaos",
        after="Black Dragon")
unit(id="warden", kind="boss", model="warden", hp=7500, hp_regen=2000, armor=300, armor_type="heavy",
     attack_type="chaos", damage=[65, 65], range=128, bat=100, attack_point=30, backswing=30, speed=270, radius=60,
     acquire=250, day_vision=1400, night_vision=1400, bounty=[105, 600], xp=1789, magic_resist=75,
     abilities=["warden_bash"], after="Roshan")
unit(id="healing_ward", kind="ward", model="healing_ward", hp=1, armor=0, armor_type="normal", speed=300, radius=16,
     day_vision=500, night_vision=500, bounty=[0, 0], after="Healing Ward")
unit(id="observer_ward", kind="ward", model="observer_ward", hp=200, armor=0, armor_type="normal", speed=0, radius=16,
     day_vision=1600, night_vision=1600, after="Observer Ward")
unit(id="sentry_ward", kind="ward", model="sentry_ward", hp=200, armor=0, armor_type="normal", speed=0, radius=16,
     day_vision=70, night_vision=70, after="Sentry Ward")
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
