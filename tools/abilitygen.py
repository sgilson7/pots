#!/usr/bin/env python3
"""Writes data/abilities.json. Numbers per level, from notes/LITERATURE.md
(DotA Allstars 6.72/6.73); time in hundredths of a second, distances in map units.
Run: python3 tools/abilitygen.py
"""
import json, os
A = []
KEYS = ["KeyQ", "KeyW", "KeyE", "KeyR"]
def ab(id, logic, target, after, slot, color="", on="enemy", ultimate=False, mana=(), cooldown=(), range=(), cast_point=30, **values):
    d = {"id": id, "logic": logic, "target": target, "on": on, "ultimate": ultimate, "mana": list(mana),
         "cooldown": list(cooldown), "range": list(range), "cast_point": cast_point,
         "values": {k: list(v) if isinstance(v, (list, tuple)) else [v] for k, v in values.items()},
         "icon": f"abilities/{id}", "color": color, "key": KEYS[slot] if slot < 4 else "", "_after": after}
    A.append(d)

# Cast points are each hero's playdota "Casting Animation" point (the review
# lists one per hero, not per ability). Heroes A are 6.72, heroes B 6.73.
# Ironvow (Sven), 6.72, cast point 0.3
ab("storm_bolt", "bolt_stun", "unit", "Storm Bolt", 0, "fx_steel", mana=[140], cooldown=[1500], range=[600], cast_point=30,
   speed=1000, radius=255, damage=[100, 175, 250, 325], stun=[200])
ab("great_cleave", "cleave", "passive", "Great Cleave", 1, pct=[20, 30, 40, 50], radius=200)
ab("warcry", "warcry", "none", "Warcry", 2, "fx_holy", on="self", mana=[25], cooldown=[3600, 3000, 2400, 1800], cast_point=30,
   radius=700, armor=[3, 6, 9, 12], move_pct=[12], duration=[600])
ab("gods_strength", "gods_strength", "none", "God's Strength", 3, "fx_holy", on="self", ultimate=True, mana=[100, 150, 200],
   cooldown=[8000], cast_point=30, damage_pct=[100, 125, 150], duration=[2500])
# Rime (Crystal Maiden), 6.73, cast point 0.3
ab("crystal_nova", "crystal_nova", "point", "Crystal Nova", 0, "fx_frost", mana=[100, 120, 140, 160], cooldown=[1500],
   range=[700], cast_point=30, radius=400, damage=[100, 150, 200, 250], slow=[30], attack_slow=[20],
   duration=[350, 400, 450, 500])
# damage is the total over the duration: 70 a second
ab("frostbite", "frostbite", "unit", "Frostbite", 1, "fx_frost", mana=[115, 125, 140, 150], cooldown=[1000],
   range=[500], cast_point=30, duration=[150, 200, 250, 300], damage=[105, 140, 175, 210], dps=[70], creep_duration=[1000])
ab("brilliance_aura", "brilliance", "passive", "Brilliance Aura", 2, radius=[99999], regen=[50, 100, 150, 200])
ab("freezing_field", "freezing_field", "none", "Freezing Field", 3, "fx_frost", on="self", ultimate=True, mana=[200, 400, 600],
   cooldown=[15000, 12000, 9000], cast_point=30, duration=[400], interval=[10], inner=[195], outer=[630], blast=[190],
   damage=[105, 170, 250], scepter_damage=[170, 250, 310], slow=[30], attack_slow=[20], armor=[0])
# Cinder (Lina), 6.73, cast point 0.45
ab("dragon_slave", "dragon_slave", "point", "Dragon Slave", 0, "fx_fire", mana=[90, 105, 125, 140], cooldown=[850],
   range=[800], cast_point=45, length=1075, width=275, end_width=150, speed=1200, damage=[100, 170, 230, 280])
ab("light_strike_array", "light_strike", "point", "Light Strike Array", 1, "fx_fire", mana=[90, 100, 110, 125], cooldown=[700],
   range=[600], cast_point=45, radius=225, delay=50, damage=[90, 150, 210, 280], stun=[160])
ab("fiery_soul", "fiery_soul", "passive", "Fiery Soul", 2, attack_speed=[30, 40, 50, 60], move_pct=[3, 4, 5, 6], stacks=3, duration=700)
ab("laguna_blade", "laguna", "unit", "Laguna Blade", 3, "fx_fire", ultimate=True, mana=[280, 420, 680], cooldown=[12000, 9000, 5500],
   range=[600], cast_point=45, delay=25, damage=[450, 675, 950], scepter_damage=[600, 925, 1250], scepter_range=[900])
# Thistle (Drow Ranger), 6.72, cast point 0.4
ab("frost_arrows", "frost_arrows", "toggle", "Frost Arrows", 0, "fx_frost", mana=[12], range=[625], slow=[10, 20, 30, 40],
   attack_slow=[5, 10, 15, 20], duration=[150], creep_duration=[700])
ab("silence", "silence_area", "point", "Silence", 1, "fx_arcane", mana=[90], cooldown=[1500], range=[900],
   cast_point=40, radius=275, duration=[300, 400, 500, 600])
ab("trueshot_aura", "trueshot", "passive", "Trueshot Aura", 2, radius=[900], pct=[7, 14, 21, 28])
ab("marksmanship", "marksmanship", "passive", "Marksmanship", 3, ultimate=True, agi=[15, 30, 45])
# Gristle (Pudge), 6.72, cast point 0.3
ab("meat_hook", "hook", "point", "Meat Hook", 0, "fx_steel", mana=[110, 120, 130, 140], cooldown=[1400, 1300, 1200, 1100],
   range=[600, 800, 1000, 1200], cast_point=30, speed=1600, width=100, damage=[90, 180, 270, 360])
ab("rot", "rot", "toggle", "Rot", 1, "fx_rot", radius=250, dps=[35, 60, 85, 110], slow=[20])
ab("flesh_heap", "flesh_heap", "passive", "Flesh Heap", 2, resist=[4, 8, 12, 16], str=[90, 120, 150, 180], radius=[450])
ab("dismember", "dismember", "unit", "Dismember", 3, "fx_blood", ultimate=True, mana=[100, 130, 170], cooldown=[3000],
   range=[160], cast_point=30, dps=[75, 125, 175], duration=[300], creep_duration=[600], scepter_str_pct=[75])
# Tectus (Earthshaker), 6.72, cast point 0.69
ab("fissure", "fissure", "point", "Fissure", 0, "fx_earth", mana=[125, 140, 155, 170], cooldown=[1500], range=[1200],
   cast_point=69, length=1200, width=225, damage=[125, 175, 225, 275], stun=[100, 125, 150, 175], duration=[800])
ab("enchant_totem", "totem", "none", "Enchant Totem", 1, "fx_earth", on="self", mana=[50], cooldown=[700], cast_point=69,
   damage_pct=[75, 150, 225, 300], duration=[1400])
ab("aftershock", "aftershock", "passive", "Aftershock", 2, radius=300, damage=[25, 45, 75, 115], stun=[30, 70, 120, 150])
ab("echo_slam", "echo_slam", "none", "Echo Slam", 3, "fx_earth", on="self", ultimate=True, mana=[145, 205, 265],
   cooldown=[15000, 13000, 11000], cast_point=69, radius=500, echo_radius=550, damage=[160, 210, 270], echo=[40, 55, 70])
# Kestrel (Juggernaut), 6.72, cast point 0.3
ab("blade_fury", "blade_fury", "none", "Blade Fury", 0, "fx_steel", on="self", mana=[110], cooldown=[3000, 2800, 2600, 2400],
   cast_point=30, duration=[500], radius=250, dps=[80, 100, 120, 140])
ab("healing_ward", "healing_ward", "point", "Healing Ward", 1, "fx_heal", on="self", mana=[80, 100, 120, 140], cooldown=[6000],
   range=[350], cast_point=30, radius=500, heal_pct=[2, 3, 4, 5], duration=[2500])
ab("blade_dance", "crit", "passive", "Blade Dance", 2, chance=[15, 20, 25, 35], multiplier=[200])
ab("omnislash", "omnislash", "unit", "Omnislash", 3, "fx_steel", ultimate=True, mana=[200, 275, 350], cooldown=[13000, 12000, 11000],
   range=[450], cast_point=30, jumps=[3, 5, 8], interval=[40], radius=425, min=[150], max=[250], pierces=[1],
   scepter_jumps=[5, 7, 10], scepter_cooldown=[11000, 10000, 9000])
# Hexwright (Lion), 6.73, cast point 0.3
ab("earth_spike", "earth_spike", "point", "Earth Spike (Impale)", 0, "fx_earth", mana=[100, 120, 145, 170], cooldown=[1200],
   range=[500], cast_point=30, length=700, width=125, speed=1600, airborne=52, damage=[60, 130, 200, 260],
   stun=[50, 100, 150, 200])
ab("hex", "hex", "unit", "Voodoo (Hex)", 1, "fx_arcane", mana=[110, 140, 170, 200], cooldown=[1300],
   range=[500], cast_point=30, duration=[125, 200, 275, 350])
ab("mana_drain", "mana_drain", "unit", "Mana Drain", 2, "fx_arcane", mana=[10], cooldown=[2500, 2000, 1500, 1000],
   range=[600, 650, 700, 750], cast_point=30, drain=[20, 40, 60, 120], duration=[400], tick=[25], **{"break": [800]})
ab("finger_of_death", "finger", "unit", "Finger of Death", 3, "fx_arcane", ultimate=True, mana=[200, 420, 650],
   cooldown=[15000, 9000, 5500], range=[700], cast_point=30, delay=25, damage=[500, 650, 850],
   scepter_damage=[600, 800, 1025], scepter_cooldown=[15000, 9000, 5000])
# Longwick (Sniper), 6.73, cast point 0.3; Assassinate takes 1.7 s
ab("shrapnel", "shrapnel", "point", "Shrapnel", 0, "fx_fire", mana=[120], cooldown=[1500], range=[1200], cast_point=30,
   radius=350, delay=[80], duration=[800], dps=[12, 24, 36, 48], slow=[15, 20, 25, 30], hits=[9], building_pct=[33])
ab("headshot", "headshot", "passive", "Headshot", 1, chance=[25, 30, 35, 40], damage=[30, 30, 40, 50], stun=[1, 10, 20, 20])
ab("take_aim", "take_aim", "passive", "Take Aim", 2, range=[65, 130, 195, 260])
ab("assassinate", "assassinate", "unit", "Assassinate", 3, "fx_fire", ultimate=True, mana=[175, 275, 375], cooldown=[2000, 1500, 1000],
   range=[1500, 2000, 2500], cast_point=170, speed=2500, damage=[355, 505, 655], stun=[10])
# Tempest (Zeus), 6.73, cast point 0.4
ab("arc_lightning", "arc_lightning", "unit", "Arc Lightning", 0, "fx_holy", mana=[65, 70, 75, 80], cooldown=[200],
   range=[700], cast_point=40, damage=[85, 100, 115, 145], bounces=[5, 7, 9, 15], radius=500)
ab("lightning_bolt", "lightning_bolt", "unit", "Lightning Bolt", 1, "fx_holy", mana=[75, 95, 115, 135], cooldown=[600],
   range=[700], cast_point=40, damage=[100, 175, 275, 350], stun=[10], sight=[900], vision=[1000], sight_duration=[300])
ab("static_field", "static_field", "passive", "Static Field", 2, pct=[5, 7, 9, 11], radius=1000)
ab("thundergods_wrath", "thundergods_wrath", "none", "Thundergod's Wrath", 3, "fx_holy", on="self", ultimate=True,
   mana=[225, 325, 450], cooldown=[12000], cast_point=40, damage=[210, 335, 460], scepter_damage=[400, 500, 600],
   sight=[900], vision=[1000], sight_duration=[300])
# the boss
ab("warden_bash", "bash", "passive", "Roshan's Bash", 9, chance=[15], damage=[50], stun=[165])

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
with open(os.path.join(root, "data", "abilities.json"), "w") as f:
    json.dump({"abilities": A}, f, indent=1)
print("wrote data/abilities.json with", len(A), "abilities")
