#!/usr/bin/env python3
"""Writes data/abilities.json. Numbers per level, from notes/LITERATURE.md
(DotA Allstars 6.6x); time in hundredths of a second, distances in map units.
Run: python3 tools/abilitygen.py
"""
import json, os
A = []
KEYS = ["KeyQ", "KeyW", "KeyE", "KeyR"]
def ab(id, logic, target, after, slot, color="", on="enemy", ultimate=False, mana=(), cooldown=(), range=(), cast_point=30, **values):
    d = {"id": id, "logic": logic, "target": target, "on": on, "ultimate": ultimate, "mana": list(mana),
         "cooldown": list(cooldown), "range": list(range), "cast_point": cast_point,
         "values": {k: list(v) if isinstance(v, (list, tuple)) else [v] for k, v in values.items()},
         "icon": f"abilities/{id}", "color": color, "key": KEYS[slot] if slot < 4 else "", "after": after}
    A.append(d)

# Ironvow (Sven)
ab("storm_bolt", "bolt_stun", "unit", "Storm Bolt", 0, "fx_steel", mana=[140], cooldown=[1300], range=[600], cast_point=30,
   speed=1000, radius=255, damage=[100, 175, 250, 325], stun=[125, 150, 175, 200])
ab("great_cleave", "cleave", "passive", "Great Cleave", 1, pct=[20, 30, 40, 50], radius=225)
ab("warcry", "warcry", "none", "Warcry", 2, "fx_holy", on="self", mana=[25], cooldown=[3200, 2600, 2000, 1400], cast_point=0,
   radius=700, armor=[3, 6, 9, 12], move_pct=[12], duration=[1500])
ab("gods_strength", "gods_strength", "none", "God's Strength", 3, "fx_holy", on="self", ultimate=True, mana=[100, 150, 200],
   cooldown=[8000], cast_point=0, damage_pct=[100, 150, 200], duration=[2500])
# Rime (Crystal Maiden)
ab("crystal_nova", "crystal_nova", "point", "Crystal Nova", 0, "fx_frost", mana=[100, 120, 140, 160], cooldown=[1500],
   range=[700], cast_point=45, radius=425, damage=[100, 150, 200, 250], slow=[30], attack_slow=[20], duration=[450])
ab("frostbite", "frostbite", "unit", "Frostbite", 1, "fx_frost", mana=[115, 125, 140, 150], cooldown=[1000, 900, 800, 700],
   range=[500], cast_point=45, duration=[150, 200, 250, 300], damage=[150, 200, 250, 300])
ab("brilliance_aura", "brilliance", "passive", "Brilliance Aura", 2, radius=[99999], regen=[60, 120, 180, 240])
ab("freezing_field", "freezing_field", "none", "Freezing Field", 3, "fx_frost", on="self", ultimate=True, mana=[200, 400, 600],
   cooldown=[15000], cast_point=0, duration=[1000], interval=[10], inner=[195], outer=[835], blast=[250],
   damage=[105, 170, 250], slow=[30], attack_slow=[20], armor=[20])
# Cinder (Lina)
ab("dragon_slave", "dragon_slave", "point", "Dragon Slave", 0, "fx_fire", mana=[100, 125, 140, 160], cooldown=[800],
   range=[1075], cast_point=45, length=1075, width=240, speed=1200, damage=[110, 180, 250, 320])
ab("light_strike_array", "light_strike", "point", "Light Strike Array", 1, "fx_fire", mana=[90, 100, 110, 125], cooldown=[700],
   range=[625], cast_point=45, radius=225, delay=50, damage=[90, 160, 230, 300], stun=[160, 180, 200, 220])
ab("fiery_soul", "fiery_soul", "passive", "Fiery Soul", 2, attack_speed=[10, 15, 20, 25], move_pct=[5, 6, 7, 8], stacks=3, duration=700)
ab("laguna_blade", "laguna", "unit", "Laguna Blade", 3, "fx_fire", ultimate=True, mana=[280, 420, 680], cooldown=[7000, 6000, 5000],
   range=[600], cast_point=45, delay=25, damage=[450, 675, 950])
# Thistle (Drow Ranger)
ab("frost_arrows", "frost_arrows", "toggle", "Frost Arrows", 0, "fx_frost", mana=[12], slow=[10, 20, 30, 40], duration=[150])
ab("silence", "silence_area", "point", "Silence", 1, "fx_arcane", mana=[70], cooldown=[2500, 2000, 1500, 1000], range=[900],
   cast_point=50, radius=275, duration=[300, 400, 500, 600])
ab("trueshot_aura", "trueshot", "passive", "Trueshot Aura", 2, radius=[900], pct=[7, 14, 21, 28])
ab("marksmanship", "marksmanship", "passive", "Marksmanship", 3, ultimate=True, agi=[15, 30, 45])
# Gristle (Pudge)
ab("meat_hook", "hook", "point", "Meat Hook", 0, "fx_steel", mana=[110, 120, 130, 140], cooldown=[1400], range=[700, 900, 1100, 1300],
   cast_point=30, speed=1600, width=100, damage=[90, 180, 270, 360])
ab("rot", "rot", "toggle", "Rot", 1, "fx_rot", radius=250, dps=[35, 60, 85, 110], slow=[20])
ab("flesh_heap", "flesh_heap", "passive", "Flesh Heap", 2, resist=[6, 8, 10, 12], str=[90, 120, 150, 180])
ab("dismember", "dismember", "unit", "Dismember", 3, "fx_blood", ultimate=True, mana=[100, 130, 170], cooldown=[3000],
   range=[160], cast_point=30, dps=[75, 125, 175], duration=[300])
# Tectus (Earthshaker)
ab("fissure", "fissure", "point", "Fissure", 0, "fx_earth", mana=[125, 140, 155, 170], cooldown=[1500], range=[1400],
   cast_point=69, length=1400, width=225, damage=[110, 160, 210, 260], stun=[100, 125, 150, 175], duration=[800])
ab("enchant_totem", "totem", "none", "Enchant Totem", 1, "fx_earth", on="self", mana=[50], cooldown=[500], cast_point=50,
   damage_pct=[100, 200, 300, 400], duration=[1400])
ab("aftershock", "aftershock", "passive", "Aftershock", 2, radius=280, damage=[50, 75, 100, 125], stun=[60, 80, 100, 120])
ab("echo_slam", "echo_slam", "none", "Echo Slam", 3, "fx_earth", on="self", ultimate=True, mana=[145, 205, 265],
   cooldown=[15000, 13000, 11000], cast_point=0, radius=575, damage=[160, 210, 270], echo=[40, 55, 70])
# Kestrel (Juggernaut)
ab("blade_fury", "blade_fury", "none", "Blade Fury", 0, "fx_steel", on="self", mana=[110], cooldown=[3000], cast_point=0,
   duration=[500], radius=250, dps=[80, 100, 120, 140])
ab("healing_ward", "healing_ward", "point", "Healing Ward", 1, "fx_heal", on="self", mana=[140], cooldown=[6000], range=[350],
   cast_point=30, radius=500, heal_pct=[1, 2, 3, 4], duration=[2500])
ab("blade_dance", "crit", "passive", "Blade Dance", 2, chance=[10, 15, 20, 25], multiplier=[200])
ab("omnislash", "omnislash", "unit", "Omnislash", 3, "fx_steel", ultimate=True, mana=[200, 275, 350], cooldown=[13000, 12000, 11000],
   range=[350], cast_point=30, jumps=[3, 5, 8], interval=[40], radius=425, min=[150], max=[250], pierces=[1])
# Hexwright (Lion)
ab("earth_spike", "earth_spike", "point", "Earth Spike (Impale)", 0, "fx_earth", mana=[100, 120, 140, 160], cooldown=[1200],
   range=[850], cast_point=30, length=825, width=125, damage=[80, 140, 200, 260], stun=[125, 175, 225, 275])
ab("hex", "hex", "unit", "Voodoo (Hex)", 1, "fx_arcane", mana=[125, 150, 175, 200], cooldown=[3000, 2400, 1800, 1200],
   range=[500], cast_point=30, duration=[125, 200, 275, 350])
ab("mana_drain", "mana_drain", "unit", "Mana Drain", 2, "fx_arcane", mana=[10], cooldown=[2500, 2000, 1500, 1000], range=[850],
   cast_point=30, drain=[20, 35, 50, 75], duration=[450], **{"break": [1100]})
ab("finger_of_death", "finger", "unit", "Finger of Death", 3, "fx_arcane", ultimate=True, mana=[200, 420, 650],
   cooldown=[16000, 10000, 4000], range=[900], cast_point=30, delay=25, damage=[600, 725, 850])
# Longwick (Sniper)
ab("shrapnel", "shrapnel", "point", "Shrapnel", 0, "fx_fire", mana=[120], cooldown=[1500], range=[1800], cast_point=30,
   radius=450, delay=[120], duration=[1100], dps=[15, 35, 55, 75], slow=[15, 20, 25, 30])
ab("headshot", "headshot", "passive", "Headshot", 1, chance=[40], damage=[20, 40, 60, 80], stun=[10])
ab("take_aim", "take_aim", "passive", "Take Aim", 2, range=[75, 150, 225, 300])
ab("assassinate", "assassinate", "unit", "Assassinate", 3, "fx_fire", ultimate=True, mana=[175, 225, 275], cooldown=[2000, 1500, 1000],
   range=[1500, 2000, 2500], cast_point=170, speed=2500, damage=[355, 505, 655], stun=[10])
# Grudge (Axe)
ab("berserkers_call", "berserkers_call", "none", "Berserker's Call", 0, "fx_blood", on="self", mana=[80, 90, 100, 110],
   cooldown=[1600, 1400, 1200, 1000], cast_point=40, radius=300, duration=[200, 250, 300, 350], armor=[40])
ab("battle_hunger", "battle_hunger", "unit", "Battle Hunger", 1, "fx_blood", mana=[75], cooldown=[1500], range=[750], cast_point=30,
   dps=[10, 15, 20, 25], slow=[12], speed=[12], duration=[2000])
ab("counter_helix", "counter_helix", "passive", "Counter Helix", 2, chance=[17], radius=275, damage=[100, 125, 150, 175])
ab("culling_blade", "culling", "unit", "Culling Blade", 3, "fx_blood", ultimate=True, mana=[60, 120, 180], cooldown=[7500, 6500, 5500],
   range=[150], cast_point=30, threshold=[300, 450, 625], damage=[150, 250, 300])
# the boss
ab("warden_bash", "bash", "passive", "Roshan's Bash", 9, chance=[15], damage=[50], stun=[165])

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
with open(os.path.join(root, "data", "abilities.json"), "w") as f:
    json.dump({"abilities": A}, f, indent=1)
print("wrote data/abilities.json with", len(A), "abilities")
